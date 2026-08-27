// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::clock;
use crate::value_format::ValueFormat;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Excel-compatible `NOW` that returns the current date and time.
///
/// Reads the wall clock of [`ValueFormat::timezone`], or of the host when that
/// is empty, which is what Excel does — `NOW()` is local time, not UTC. See
/// [`crate::clock`] for what the `Utc` in the return type means here.
pub fn codcel_now(
    value_format: &ValueFormat,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    Ok(clock::now(value_format))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn now_tracks_the_clock() {
        let vf = ValueFormat::default();
        let before = clock::now(&vf);
        let result = codcel_now(&vf).unwrap();
        let after = clock::now(&vf);

        assert!(result >= before);
        assert!(result <= after);
    }

    /// What this wrapper is responsible for is delegating to the shared clock.
    ///
    /// Asserted as delegation rather than against UTC: `NOW()` reads local time,
    /// so comparing it to `Utc::now()` only ever tested `chrono::Local`, and it
    /// broke outright whenever something else in the process had frozen the
    /// clock. Both sides here move together either way.
    #[test]
    fn now_delegates_to_the_shared_clock() {
        let vf = ValueFormat::default();
        let result = codcel_now(&vf).unwrap();
        let diff = (result - clock::now(&vf)).num_seconds().abs();
        assert!(diff <= 5, "{diff}s from clock::now");
    }

    /// Two named zones read different wall clocks.
    ///
    /// Asserted through `wall_clock_in`, which sits below the mock check in
    /// `clock::now`, so a frozen clock elsewhere in the process cannot collapse
    /// both zones onto the same instant and make this vacuously false.
    #[cfg(feature = "named-timezones")]
    #[test]
    fn a_named_timezone_reads_its_own_wall_clock() {
        let berlin = clock::wall_clock_in("Europe/Berlin").expect("a real IANA zone");
        let tokyo = clock::wall_clock_in("Asia/Tokyo").expect("a real IANA zone");

        // Tokyo is seven or eight hours ahead of Berlin depending on the season.
        let hours = (tokyo - berlin).num_minutes() as f64 / 60.0;
        assert!((6.9..=8.1).contains(&hours), "{hours} hours apart");
    }
}
