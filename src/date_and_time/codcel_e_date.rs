// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_and_time::days_in_month::days_in_month;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use std::error::Error;

/// Excel-compatible `EDATE` that returns a date offset by a specified number of months.
/// - `start_date`: the starting date.
/// - `months`: number of months to add (positive) or subtract (negative).
///   Returns the date that is the specified number of months before or after the start date, clamping to the last valid day if necessary.
pub fn codcel_e_date(
    start_date: DateTime<Utc>,
    months: i32,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    let start_year = start_date.year();
    let start_month = start_date.month() as i32;
    let start_day = start_date.day();
    let start_time = start_date.time();

    // Calculate the target year and month
    let total_months = start_month + months;
    let mut target_year = start_year + (total_months - 1) / 12;
    let mut target_month = ((total_months - 1) % 12) + 1;

    // Handle negative months properly
    if target_month <= 0 {
        target_month += 12;
        target_year -= 1;
    }

    // Get the last day of the target month
    let last_day_of_month = days_in_month("EDATE", target_year, target_month as u32)?;

    // Use the original day unless it exceeds the number of days in the target month
    let target_day = std::cmp::min(start_day, last_day_of_month);

    // Create the naive date and time
    let naive_date = NaiveDate::from_ymd_opt(target_year, target_month as u32, target_day)
        .ok_or("EDATE: Invalid date")?;

    // Preserve the original time
    let naive_datetime = naive_date.and_time(start_time);

    // Convert to DateTime<Utc>
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(
        naive_datetime,
        Utc,
    ))
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
    fn test_e_date_add_months() {
        // =EDATE("2023-01-15", 4) in US format
        // =EDATE("2023-01-15"; 4) in German format
        let start_date = create_date(2023, 1, 15);
        let result = codcel_e_date(start_date, 4).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2023);
        assert_eq!(result.month(), 5);
        assert_eq!(result.day(), 15);
    }

    #[test]
    fn test_e_date_subtract_months() {
        // =EDATE("2023-05-15", -4) in US format
        // =EDATE("2023-05-15"; -4) in German format
        let start_date = create_date(2023, 5, 15);
        let result = codcel_e_date(start_date, -4).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2023);
        assert_eq!(result.month(), 1);
        assert_eq!(result.day(), 15);
    }

    #[test]
    fn test_e_date_cross_year_forward() {
        // =EDATE("2023-11-15", 3) in US format
        // =EDATE("2023-11-15"; 3) in German format
        let start_date = create_date(2023, 11, 15);
        let result = codcel_e_date(start_date, 3).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2024);
        assert_eq!(result.month(), 2);
        assert_eq!(result.day(), 15);
    }

    #[test]
    fn test_e_date_cross_year_backward() {
        // =EDATE("2023-02-15", -3) in US format
        // =EDATE("2023-02-15"; -3) in German format
        let start_date = create_date(2023, 2, 15);
        let result = codcel_e_date(start_date, -3).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2022);
        assert_eq!(result.month(), 11);
        assert_eq!(result.day(), 15);
    }

    #[test]
    fn test_e_date_month_end_to_shorter_month() {
        // =EDATE("2023-01-31", 1) in US format
        // =EDATE("2023-01-31"; 1) in German format
        let start_date = create_date(2023, 1, 31);
        let result = codcel_e_date(start_date, 1).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2023);
        assert_eq!(result.month(), 2);
        assert_eq!(result.day(), 28); // February 2023 has 28 days
    }

    #[test]
    fn test_e_date_month_end_to_longer_month() {
        // =EDATE("2023-02-28", 1) in US format
        // =EDATE("2023-02-28"; 1) in German format
        let start_date = create_date(2023, 2, 28);
        let result = codcel_e_date(start_date, 1).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2023);
        assert_eq!(result.month(), 3);
        assert_eq!(result.day(), 28);
    }

    #[test]
    fn test_e_date_leap_year() {
        // =EDATE("2020-01-31", 1) in US format
        // =EDATE("2020-01-31"; 1) in German format
        let start_date = create_date(2020, 1, 31);
        let result = codcel_e_date(start_date, 1).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2020);
        assert_eq!(result.month(), 2);
        assert_eq!(result.day(), 29); // February 2020 has 29 days (leap year)
    }

    #[test]
    fn test_e_date_multiple_years() {
        // =EDATE("2023-05-15", 24) in US format
        // =EDATE("2023-05-15"; 24) in German format
        let start_date = create_date(2023, 5, 15);
        let result = codcel_e_date(start_date, 24).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2025);
        assert_eq!(result.month(), 5);
        assert_eq!(result.day(), 15);
    }
}
