// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::date_and_time::days_in_month::days_in_month;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use std::error::Error;

/// Excel-compatible `EOMONTH` that returns the last day of a month offset by a number of months.
/// - `start_date`: the starting date.
/// - `months`: number of months to add (positive) or subtract (negative) from the start date.
///   Returns the last day of the target month as a UTC datetime.
pub fn codcel_eo_month(
    start_date: DateTime<Utc>,
    months: i32,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    let start_year = start_date.year();
    let start_month = start_date.month() as i32;
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
    let last_day = days_in_month("EOMONTH", target_year, target_month as u32)?;

    // Create the naive date for the last day of the month
    let naive_date = NaiveDate::from_ymd_opt(target_year, target_month as u32, last_day)
        .ok_or("EOMONTH: Invalid date")?;

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
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_eo_month_same_month() {
        // =EOMONTH("2023-01-15", 0) in US format
        // =EOMONTH("2023-01-15"; 0) in German format
        let start_date = create_date(2023, 1, 15);
        let result = codcel_eo_month(start_date, 0).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2023);
        assert_eq!(result.month(), 1);
        assert_eq!(result.day(), 31); // January has 31 days
    }

    #[test]
    fn test_eo_month_add_months() {
        // =EOMONTH("2023-01-15", 4) in US format
        // =EOMONTH("2023-01-15"; 4) in German format
        let start_date = create_date(2023, 1, 15);
        let result = codcel_eo_month(start_date, 4).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2023);
        assert_eq!(result.month(), 5);
        assert_eq!(result.day(), 31); // May has 31 days
    }

    #[test]
    fn test_eo_month_subtract_months() {
        // =EOMONTH("2023-05-15", -4) in US format
        // =EOMONTH("2023-05-15"; -4) in German format
        let start_date = create_date(2023, 5, 15);
        let result = codcel_eo_month(start_date, -4).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2023);
        assert_eq!(result.month(), 1);
        assert_eq!(result.day(), 31); // January has 31 days
    }

    #[test]
    fn test_eo_month_cross_year_forward() {
        // =EOMONTH("2023-11-15", 3) in US format
        // =EOMONTH("2023-11-15"; 3) in German format
        let start_date = create_date(2023, 11, 15);
        let result = codcel_eo_month(start_date, 3).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2024);
        assert_eq!(result.month(), 2);
        assert_eq!(result.day(), 29); // February 2024 has 29 days (leap year)
    }

    #[test]
    fn test_eo_month_cross_year_backward() {
        // =EOMONTH("2023-02-15", -3) in US format
        // =EOMONTH("2023-02-15"; -3) in German format
        let start_date = create_date(2023, 2, 15);
        let result = codcel_eo_month(start_date, -3).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2022);
        assert_eq!(result.month(), 11);
        assert_eq!(result.day(), 30); // November has 30 days
    }

    #[test]
    fn test_eo_month_february_non_leap_year() {
        // =EOMONTH("2023-01-15", 1) in US format
        // =EOMONTH("2023-01-15"; 1) in German format
        let start_date = create_date(2023, 1, 15);
        let result = codcel_eo_month(start_date, 1).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2023);
        assert_eq!(result.month(), 2);
        assert_eq!(result.day(), 28); // February 2023 has 28 days
    }

    #[test]
    fn test_eo_month_february_leap_year() {
        // =EOMONTH("2020-01-15", 1) in US format
        // =EOMONTH("2020-01-15"; 1) in German format
        let start_date = create_date(2020, 1, 15);
        let result = codcel_eo_month(start_date, 1).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2020);
        assert_eq!(result.month(), 2);
        assert_eq!(result.day(), 29); // February 2020 has 29 days (leap year)
    }

    #[test]
    fn test_eo_month_30_day_month() {
        // =EOMONTH("2023-03-15", 1) in US format
        // =EOMONTH("2023-03-15"; 1) in German format
        let start_date = create_date(2023, 3, 15);
        let result = codcel_eo_month(start_date, 1).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2023);
        assert_eq!(result.month(), 4);
        assert_eq!(result.day(), 30); // April has 30 days
    }

    #[test]
    fn test_eo_month_multiple_years() {
        // =EOMONTH("2023-05-15", 24) in US format
        // =EOMONTH("2023-05-15"; 24) in German format
        let start_date = create_date(2023, 5, 15);
        let result = codcel_eo_month(start_date, 24).unwrap();
        println!("{result}");
        assert_eq!(result.year(), 2025);
        assert_eq!(result.month(), 5);
        assert_eq!(result.day(), 31); // May has 31 days
    }
}
