// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, TimeZone, Utc};
use std::error::Error;

/// Days in `month` of `year`, using Excel's leap-year rules. `month` is 1-12.
fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
        2 => 28,
        _ => 31,
    }
}

/// Shifts `date` by `months`, preserving Excel's coupon-schedule conventions: the day is
/// clamped to the target month's length, and a date that was the last day of its own month
/// lands on the last day of the target month.
///
/// Returns an error rather than panicking when the shifted year falls outside the calendar
/// chrono can represent, which is why this is a `Result` — the previous per-function copies
/// built the date through `LocalResult::unwrap()`, a panic clippy's `unwrap_used` cannot see.
pub fn add_months(
    date: DateTime<Utc>,
    months: i32,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    let mut year = date.year();
    let mut month = date.month() as i32 + months;

    while month > 12 {
        month -= 12;
        year += 1;
    }
    while month < 1 {
        month += 12;
        year -= 1;
    }
    let month = month as u32;

    let original_day = date.day();
    let max_day_in_target_month = days_in_month(year, month);

    // For end-of-month dates, preserve end-of-month behavior.
    let day = if original_day >= days_in_month(date.year(), date.month()) {
        max_day_in_target_month
    } else {
        original_day.min(max_day_in_target_month)
    };

    let shifted = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| format!("Date {year}-{month:02}-{day:02} is outside the usable range"))?;
    Ok(Utc.from_utc_datetime(&shifted.and_time(NaiveTime::MIN)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.from_utc_datetime(
            &NaiveDate::from_ymd_opt(year, month, day)
                .expect("test date")
                .and_time(NaiveTime::MIN),
        )
    }

    #[test]
    fn clamps_the_day_to_the_target_month() {
        assert_eq!(add_months(date(2024, 1, 31), 1).unwrap(), date(2024, 2, 29));
        assert_eq!(add_months(date(2023, 1, 31), 1).unwrap(), date(2023, 2, 28));
    }

    #[test]
    fn preserves_end_of_month() {
        // Aug 31 back six months lands on Feb 29 in a leap year.
        assert_eq!(
            add_months(date(2024, 8, 31), -6).unwrap(),
            date(2024, 2, 29)
        );
    }

    #[test]
    fn rolls_the_year_in_both_directions() {
        assert_eq!(
            add_months(date(2024, 11, 15), 3).unwrap(),
            date(2025, 2, 15)
        );
        assert_eq!(
            add_months(date(2024, 2, 15), -3).unwrap(),
            date(2023, 11, 15)
        );
    }

    #[test]
    fn reports_an_error_instead_of_panicking_out_of_range() {
        // Previously `LocalResult::unwrap()`, which aborted the process.
        assert!(add_months(date(2024, 1, 15), i32::MAX / 2).is_err());
    }
}
