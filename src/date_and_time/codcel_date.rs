// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::date_and_time::days_in_month::days_in_month;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use std::error::Error;

/// Excel-compatible `DATE` that constructs a date from year, month, and day components.
/// - `year`: the year component (must be between 1 and 9999).
/// - `month`: the month component (values outside 1–12 roll over into adjacent years).
/// - `day`: the day component (values outside valid range roll over into adjacent months).
///   Returns a UTC `DateTime` at midnight or an error if the adjusted year is outside the allowed range.
pub fn codcel_date(
    year: i32,
    month: i32,
    day: i32,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    let mut year = year;
    let mut month = month;
    let mut day = day;

    // Validate year range (Excel's limitation)
    if !(1..=9999).contains(&year) {
        return Err("Year must be between 1 and 9999".into());
    }

    // Adjust month if out of standard range
    while month > 12 {
        year += 1;
        month -= 12;
    }
    while month < 1 {
        year -= 1;
        month += 12;
    }

    // Handle day calculation, including negative and zero days
    while day < 1 {
        month -= 1;

        // Adjust year if month becomes 0
        if month < 1 {
            year -= 1;
            month = 12;
        }

        // Get days in the previous month
        let prev_month_days = days_in_month("DATE", year, month as u32)?;

        // Add remaining days from previous month
        day += prev_month_days as i32;
    }

    // Handle day overflow
    while day > days_in_month("DATE", year, month as u32)? as i32 {
        let max_days = days_in_month("DATE", year, month as u32)?;
        day -= max_days as i32;
        month += 1;

        // Adjust year if month exceeds 12
        if month > 12 {
            year += 1;
            month = 1;
        }
    }

    // Create final date
    match NaiveDate::from_ymd_opt(year, month as u32, day as u32) {
        Some(final_date) => Ok(Utc.from_utc_datetime(&final_date.and_hms_opt(0, 0, 0).unwrap())),
        None => Err("Invalid date calculation".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_basic() {
        // =DATE(2023, 5, 15) in US format
        // =DATE(2023; 5; 15) in German format
        let result = codcel_date(2023, 5, 15).unwrap();
        println!("{result}");
        // Expected: 2023-05-15T00:00:00Z
        assert_eq!(result.to_string(), "2023-05-15 00:00:00 UTC");
    }

    #[test]
    fn test_date_month_overflow() {
        // =DATE(2023, 14, 15) in US format
        // =DATE(2023; 14; 15) in German format
        let result = codcel_date(2023, 14, 15).unwrap();
        println!("{result}");
        // Expected: 2024-02-15T00:00:00Z
        assert_eq!(result.to_string(), "2024-02-15 00:00:00 UTC");
    }

    #[test]
    fn test_date_month_underflow() {
        // =DATE(2023, 0, 15) in US format
        // =DATE(2023; 0; 15) in German format
        let result = codcel_date(2023, 0, 15).unwrap();
        println!("{result}");
        // Expected: 2022-12-15T00:00:00Z
        assert_eq!(result.to_string(), "2022-12-15 00:00:00 UTC");
    }

    #[test]
    fn test_date_day_overflow() {
        // =DATE(2023, 2, 30) in US format
        // =DATE(2023; 2; 30) in German format
        let result = codcel_date(2023, 2, 30).unwrap();
        println!("{result}");
        // Expected: 2023-03-02T00:00:00Z
        assert_eq!(result.to_string(), "2023-03-02 00:00:00 UTC");
    }

    #[test]
    fn test_date_day_underflow() {
        // =DATE(2023, 3, 0) in US format
        // =DATE(2023; 3; 0) in German format
        let result = codcel_date(2023, 3, 0).unwrap();
        println!("{result}");
        // Expected: 2023-02-28T00:00:00Z
        assert_eq!(result.to_string(), "2023-02-28 00:00:00 UTC");
    }

    #[test]
    fn test_date_leap_year() {
        // =DATE(2024, 2, 29) in US format
        // =DATE(2024; 2; 29) in German format
        let result = codcel_date(2024, 2, 29).unwrap();
        println!("{result}");
        // Expected: 2024-02-29T00:00:00Z
        assert_eq!(result.to_string(), "2024-02-29 00:00:00 UTC");
    }

    #[test]
    fn test_date_invalid_year() {
        // =DATE(10000, 5, 15) in US format
        // =DATE(10000; 5; 15) in German format
        let result = codcel_date(10000, 5, 15);
        assert!(result.is_err());
    }
}
