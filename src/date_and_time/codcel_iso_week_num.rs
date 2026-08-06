// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, Utc};
use std::error::Error;

/// Excel-compatible `ISOWEEKNUM` that returns the ISO 8601 week number for a date.
/// - `date`: a UTC datetime value.
///   Returns the ISO week number (1–53), where weeks start on Monday.
pub fn codcel_iso_week_num(date: DateTime<Utc>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let iso_week = date.iso_week();
    Ok(iso_week.week() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn create_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_iso_week_num_start_of_year() {
        // =ISOWEEKNUM("2023-01-01") in US format
        // =ISOWEEKNUM("2023-01-01") in German format
        let date = create_date(2023, 1, 1);
        let result = codcel_iso_week_num(date).unwrap();
        println!("{result}");
        assert_eq!(result, 52); // January 1, 2023 is in week 52 of 2022 in ISO
    }

    #[test]
    fn test_iso_week_num_first_week() {
        // =ISOWEEKNUM("2023-01-02") in US format
        // =ISOWEEKNUM("2023-01-02") in German format
        let date = create_date(2023, 1, 2);
        let result = codcel_iso_week_num(date).unwrap();
        println!("{result}");
        assert_eq!(result, 1); // January 2, 2023 is in week 1 of 2023 in ISO
    }

    #[test]
    fn test_iso_week_num_middle_of_year() {
        // =ISOWEEKNUM("2023-06-15") in US format
        // =ISOWEEKNUM("2023-06-15") in German format
        let date = create_date(2023, 6, 15);
        let result = codcel_iso_week_num(date).unwrap();
        println!("{result}");
        assert_eq!(result, 24); // June 15, 2023 is in week 24 of 2023 in ISO
    }

    #[test]
    fn test_iso_week_num_end_of_year() {
        // =ISOWEEKNUM("2023-12-31") in US format
        // =ISOWEEKNUM("2023-12-31") in German format
        let date = create_date(2023, 12, 31);
        let result = codcel_iso_week_num(date).unwrap();
        println!("{result}");
        assert_eq!(result, 52); // December 31, 2023 is in week 52 of 2023 in ISO
    }

    #[test]
    fn test_iso_week_num_leap_year() {
        // =ISOWEEKNUM("2020-02-29") in US format
        // =ISOWEEKNUM("2020-02-29") in German format
        let date = create_date(2020, 2, 29);
        let result = codcel_iso_week_num(date).unwrap();
        println!("{result}");
        assert_eq!(result, 9); // February 29, 2020 is in week 9 of 2020 in ISO
    }

    #[test]
    fn test_iso_week_num_week_53() {
        // =ISOWEEKNUM("2020-12-31") in US format
        // =ISOWEEKNUM("2020-12-31") in German format
        let date = create_date(2020, 12, 31);
        let result = codcel_iso_week_num(date).unwrap();
        println!("{result}");
        assert_eq!(result, 53); // December 31, 2020 is in week 53 of 2020 in ISO (leap year)
    }
}
