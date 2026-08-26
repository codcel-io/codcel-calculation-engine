// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, Utc};
use std::error::Error;

/// Excel-compatible `DAY` that extracts the day of the month from a date.
/// - `date`: a UTC datetime value.
///   Returns the day of the month as an integer (1–31).
pub fn codcel_day(date: DateTime<Utc>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    Ok(date.day() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn create_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0)
            .single()
            .expect("valid test date")
    }

    #[test]
    fn test_day_start_of_month() {
        // =DAY("2023-05-01") in US format
        // =DAY("2023-05-01") in German format
        let date = create_date(2023, 5, 1);
        let result = codcel_day(date).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_day_middle_of_month() {
        // =DAY("2023-05-15") in US format
        // =DAY("2023-05-15") in German format
        let date = create_date(2023, 5, 15);
        let result = codcel_day(date).unwrap();
        println!("{result}");
        assert_eq!(result, 15);
    }

    #[test]
    fn test_day_end_of_month() {
        // =DAY("2023-05-31") in US format
        // =DAY("2023-05-31") in German format
        let date = create_date(2023, 5, 31);
        let result = codcel_day(date).unwrap();
        println!("{result}");
        assert_eq!(result, 31);
    }

    #[test]
    fn test_day_february_leap_year() {
        // =DAY("2020-02-29") in US format
        // =DAY("2020-02-29") in German format
        let date = create_date(2020, 2, 29);
        let result = codcel_day(date).unwrap();
        println!("{result}");
        assert_eq!(result, 29);
    }

    #[test]
    fn test_day_february_non_leap_year() {
        // =DAY("2023-02-28") in US format
        // =DAY("2023-02-28") in German format
        let date = create_date(2023, 2, 28);
        let result = codcel_day(date).unwrap();
        println!("{result}");
        assert_eq!(result, 28);
    }
}
