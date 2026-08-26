// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, Utc};
use std::error::Error;

/// Excel-compatible `YEAR` that extracts the year from a date.
/// - `date`: a UTC datetime value.
///   Returns the four-digit year as an integer.
pub fn codcel_year(date: DateTime<Utc>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    Ok(date.year())
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
    fn test_year_basic() {
        // =YEAR("2023-05-15") in US format
        // =YEAR("2023-05-15") in German format
        let date = create_date(2023, 5, 15);
        let result = codcel_year(date).unwrap();
        println!("{result}");
        assert_eq!(result, 2023);
    }

    #[test]
    fn test_year_leap_year() {
        // =YEAR("2024-02-29") in US format
        // =YEAR("2024-02-29") in German format
        let date = create_date(2024, 2, 29);
        let result = codcel_year(date).unwrap();
        println!("{result}");
        assert_eq!(result, 2024);
    }

    #[test]
    fn test_year_first_day() {
        // =YEAR("2023-01-01") in US format
        // =YEAR("2023-01-01") in German format
        let date = create_date(2023, 1, 1);
        let result = codcel_year(date).unwrap();
        println!("{result}");
        assert_eq!(result, 2023);
    }

    #[test]
    fn test_year_last_day() {
        // =YEAR("2023-12-31") in US format
        // =YEAR("2023-12-31") in German format
        let date = create_date(2023, 12, 31);
        let result = codcel_year(date).unwrap();
        println!("{result}");
        assert_eq!(result, 2023);
    }

    #[test]
    fn test_year_past_date() {
        // =YEAR("1900-01-01") in US format
        // =YEAR("1900-01-01") in German format
        let date = create_date(1900, 1, 1);
        let result = codcel_year(date).unwrap();
        println!("{result}");
        assert_eq!(result, 1900);
    }

    #[test]
    fn test_year_future_date() {
        // =YEAR("2100-12-31") in US format
        // =YEAR("2100-12-31") in German format
        let date = create_date(2100, 12, 31);
        let result = codcel_year(date).unwrap();
        println!("{result}");
        assert_eq!(result, 2100);
    }
}
