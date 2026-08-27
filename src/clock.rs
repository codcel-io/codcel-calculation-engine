// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! The clock behind `NOW` and `TODAY`.
//!
//! # Why these are not UTC
//!
//! Excel's `NOW()` and `TODAY()` read the machine's wall clock. A spreadsheet
//! opened in Auckland at nine in the morning shows that day's date, and so does
//! the same spreadsheet opened in Los Angeles at nine in the morning, thirteen
//! hours later. Excel has no timezone concept at all: a date serial is a
//! zoneless wall-clock reading, and there is nowhere in the file format to
//! record which zone produced it.
//!
//! Codcel returned `Utc::now()` for both, which is a different function. For a
//! caller at UTC+13 it hands back yesterday's date for thirteen hours of every
//! day, and for one at UTC-8 it rolls over to tomorrow eight hours early.
//!
//! # How the wall clock is carried
//!
//! [`Value::ChronoDateTime`](crate::value::Value::ChronoDateTime) holds a
//! `DateTime<Utc>`, and it stays that way. What these functions return is the
//! *local wall-clock reading* re-labelled as UTC — the same trick the file
//! format plays, and the one that makes the conversion to a serial come out
//! right. Reading the `Utc` in the type as a claim about the instant would be a
//! mistake; it marks a zoneless local reading.
//!
//! # Resolution order
//!
//! 1. `CODCEL_MOCK_NOW`, when the `mock-clock` feature is on. A fixed instant,
//!    for tests that would otherwise only pass on the day they were written.
//! 2. [`ValueFormat::timezone`], an IANA name such as `Europe/Berlin`, set from
//!    a transpiler flag, the `CODCEL_TIMEZONE` environment variable or a
//!    per-call format. Requires the `named-timezones` feature.
//! 3. The host's local timezone.
//! 4. UTC, if the host's zone cannot be determined.

use crate::value_format::ValueFormat;
use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Utc};
use std::error::Error;

/// Parses a `CODCEL_MOCK_NOW` value.
///
/// RFC 3339, so an offset is required: `2023-05-15T14:30:45Z` or
/// `2023-05-15T16:30:45+02:00`, but not a bare `2023-05-15T14:30:45`.
///
/// `None` for anything unparseable, so a stray environment variable cannot take
/// down a calculation.
///
/// Split out from [`mock_now`] so it can be tested without touching the
/// environment — see the note above the test module.
#[cfg(feature = "mock-clock")]
fn parse_mock_now(raw: &str) -> Option<NaiveDateTime> {
    DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|dt| dt.naive_utc())
}

/// A fixed instant from `CODCEL_MOCK_NOW`, if one is set.
#[cfg(feature = "mock-clock")]
fn mock_now() -> Option<NaiveDateTime> {
    parse_mock_now(&std::env::var("CODCEL_MOCK_NOW").ok()?)
}

#[cfg(not(feature = "mock-clock"))]
fn mock_now() -> Option<NaiveDateTime> {
    None
}

/// The wall-clock reading in an explicitly named IANA timezone, or `None` if
/// the name is not one.
///
/// `pub(crate)` rather than private so that timezone tests can target it
/// directly. [`now`] consults the mock clock *before* reaching here, so a test
/// written at this level cannot observe a mocked instant — which is what keeps
/// those tests independent of process-wide environment state.
#[cfg(feature = "named-timezones")]
pub(crate) fn wall_clock_in(zone: &str) -> Option<NaiveDateTime> {
    let tz: chrono_tz::Tz = zone.parse().ok()?;
    Some(Utc::now().with_timezone(&tz).naive_local())
}

#[cfg(not(feature = "named-timezones"))]
pub(crate) fn wall_clock_in(_zone: &str) -> Option<NaiveDateTime> {
    None
}

/// The current wall-clock reading for `value_format`'s timezone.
///
/// Total: every fallible step falls through to the next candidate and finally
/// to UTC, because the crate denies panics and there is no sensible error to
/// return from a clock.
///
/// See the module note on what the `Utc` in the return type does and does not
/// mean.
pub fn now(value_format: &ValueFormat) -> DateTime<Utc> {
    let naive = mock_now()
        .or_else(|| {
            let zone = value_format.timezone.trim();
            if zone.is_empty() {
                None
            } else {
                wall_clock_in(zone)
            }
        })
        .unwrap_or_else(|| chrono::Local::now().naive_local());

    Utc.from_utc_datetime(&naive)
}

/// The current wall-clock reading truncated to midnight.
pub fn today(value_format: &ValueFormat) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    let now = now(value_format);
    let midnight = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .ok_or("Failed to construct TODAY at midnight")?;
    debug_assert_eq!(midnight.day(), now.day());
    Ok(Utc.from_utc_datetime(&midnight))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Nothing in this module reads or writes `CODCEL_MOCK_NOW`, and that is
    // deliberate rather than incidental.
    //
    // Rust runs unit tests as threads of one process, so a `set_var` in any one
    // of them is visible to every other for as long as it is set. `now` consults
    // the mock clock first, which makes every clock-reading test downstream of
    // whatever another test happens to have set — and the failure only shows on
    // some interleavings, so it reads as a heisenbug rather than as a shared
    // global.
    //
    // Two things keep that from happening here. The parse is tested as a pure
    // function, and timezone behaviour is tested through `wall_clock_in`, which
    // sits *below* the mock check in `now` and therefore cannot see one. The
    // environment plumbing is tested in `tests/mock_clock.rs`, which is its own
    // binary and so its own process.
    //
    // Please keep it that way: a test here that sets the variable would make the
    // rest of this module flaky again.

    #[cfg(feature = "mock-clock")]
    #[test]
    fn parse_mock_now_accepts_rfc_3339_and_normalises_to_utc() {
        assert_eq!(
            parse_mock_now("2023-05-15T14:30:45Z").map(|d| d.to_string()),
            Some("2023-05-15 14:30:45".to_string())
        );
        // An explicit offset is honoured and folded into UTC.
        assert_eq!(
            parse_mock_now("2023-05-15T16:30:45+02:00"),
            parse_mock_now("2023-05-15T14:30:45Z")
        );
    }

    /// An offset is not optional. Worth pinning: a value like
    /// `2023-05-15T14:30:45` looks correct, parses as nothing, and so silently
    /// leaves the real clock in place rather than failing loudly.
    #[cfg(feature = "mock-clock")]
    #[test]
    fn parse_mock_now_rejects_a_timestamp_with_no_offset() {
        assert_eq!(parse_mock_now("2023-05-15T14:30:45"), None);
    }

    /// A stray environment variable must not take a calculation down.
    #[cfg(feature = "mock-clock")]
    #[test]
    fn parse_mock_now_ignores_anything_it_cannot_read() {
        for raw in ["", "not a timestamp", "2023-05-15", "1684161045"] {
            assert_eq!(parse_mock_now(raw), None, "{raw:?}");
        }
    }

    /// The point of the timezone model: a caller east of the date line and one
    /// west of it disagree about what day it is, exactly as two copies of Excel
    /// would.
    ///
    /// Asserted through `wall_clock_in` rather than `now`, so a mocked clock
    /// elsewhere in the process cannot freeze one side and not the other.
    #[cfg(feature = "named-timezones")]
    #[test]
    fn the_date_differs_across_the_date_line() {
        let east = wall_clock_in("Pacific/Auckland").expect("a real IANA zone");
        let west = wall_clock_in("Pacific/Honolulu").expect("a real IANA zone");

        assert!(
            east.date() >= west.date(),
            "Auckland is ahead of Honolulu: {east} vs {west}"
        );
        // Twenty-three hours apart, so never more than one calendar day.
        assert!((east.date() - west.date()).num_days() <= 1);
    }

    /// An unknown zone name is not a zone, and must degrade rather than fail.
    #[test]
    fn an_unknown_timezone_is_not_resolved() {
        assert_eq!(wall_clock_in("Mars/Olympus_Mons"), None);
    }

    /// ...so `now` ignores it and answers as though no zone had been named,
    /// rather than erroring.
    ///
    /// Asserted as equivalence to the no-zone case rather than against the host
    /// clock directly: both sides resolve the same way whether or not something
    /// has frozen the clock, so this does not depend on process-wide state.
    #[test]
    fn an_unknown_timezone_is_ignored_rather_than_failing() {
        let unknown = ValueFormat {
            timezone: "Mars/Olympus_Mons".to_string(),
            ..Default::default()
        };
        let none = ValueFormat::default();
        let diff = (now(&unknown) - now(&none)).num_seconds().abs();
        assert!(diff <= 5, "{diff}s apart from the no-zone reading");
    }

    #[test]
    fn today_is_midnight() {
        use chrono::Timelike;
        let t = today(&ValueFormat::default()).unwrap();
        assert_eq!(
            (t.hour(), t.minute(), t.second(), t.nanosecond()),
            (0, 0, 0, 0)
        );
    }

    /// `today` is `now` with the time removed, whatever `now` happens to be.
    #[test]
    fn today_is_the_same_day_as_now() {
        let vf = ValueFormat::default();
        assert_eq!(today(&vf).unwrap().date_naive(), now(&vf).date_naive());
    }
}
