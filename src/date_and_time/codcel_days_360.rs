// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, Utc};
use std::error::Error;

/// Excel-compatible `DAYS360` that calculates days between two dates using a 30-day month basis.
/// - `start_date`: the starting date.
/// - `end_date`: the ending date.
/// - `use_european_method`: optional; if `true` uses European 30/360, otherwise US/NASD method (default).
///   Returns the number of days based on a 360-day year (12 months of 30 days each).
pub fn codcel_days_360(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    use_european_method: Option<bool>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let use_european_method = use_european_method.unwrap_or(false);

    let start_day = start_date.day();
    let start_month = start_date.month();
    let start_year = start_date.year();

    let end_day = end_date.day();
    let end_month = end_date.month();
    let end_year = end_date.year();

    let start_day_30 = if use_european_method {
        start_day.min(30)
    } else if start_day == 31 {
        30
    } else {
        start_day
    };

    let mut end_day_30 = if use_european_method {
        end_day.min(30)
    } else if end_day == 31 && start_day_30 == 30 {
        30
    } else {
        end_day
    };

    if use_european_method && start_day_30 == 30 && end_day == 31 {
        end_day_30 = 30;
    }

    let days360 = (end_year - start_year) * 360
        + (end_month as i32 - start_month as i32) * 30
        + (end_day_30 as i32 - start_day_30 as i32);

    Ok(days360)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn create_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_days_360_us_method_basic() {
        // =DAYS360("2023-01-15", "2023-05-20", FALSE) in US format
        // =DAYS360("2023-01-15"; "2023-05-20"; FALSE) in German format
        let start_date = create_date(2023, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_days_360(start_date, end_date, Some(false)).unwrap();
        println!("{result}");
        assert_eq!(result, 125);
    }

    #[test]
    fn test_days_360_european_method_basic() {
        // =DAYS360("2023-01-15", "2023-05-20", TRUE) in US format
        // =DAYS360("2023-01-15"; "2023-05-20"; TRUE) in German format
        let start_date = create_date(2023, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_days_360(start_date, end_date, Some(true)).unwrap();
        println!("{result}");
        assert_eq!(result, 125);
    }

    #[test]
    fn test_days_360_us_method_start_31st() {
        // =DAYS360("2023-01-31", "2023-05-20", FALSE) in US format
        // =DAYS360("2023-01-31"; "2023-05-20"; FALSE) in German format
        let start_date = create_date(2023, 1, 31);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_days_360(start_date, end_date, Some(false)).unwrap();
        println!("{result}");
        assert_eq!(result, 110); // 31st is treated as 30th in US method
    }

    #[test]
    fn test_days_360_european_method_start_31st() {
        // =DAYS360("2023-01-31", "2023-05-20", TRUE) in US format
        // =DAYS360("2023-01-31"; "2023-05-20"; TRUE) in German format
        let start_date = create_date(2023, 1, 31);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_days_360(start_date, end_date, Some(true)).unwrap();
        println!("{result}");
        assert_eq!(result, 110); // 31st is treated as 30th in European method
    }

    #[test]
    fn test_days_360_us_method_end_31st() {
        // =DAYS360("2023-01-15", "2023-05-31", FALSE) in US format
        // =DAYS360("2023-01-15"; "2023-05-31"; FALSE) in German format
        let start_date = create_date(2023, 1, 15);
        let end_date = create_date(2023, 5, 31);
        let result = codcel_days_360(start_date, end_date, Some(false)).unwrap();
        println!("{result}");
        assert_eq!(result, 136); // End 31st is treated as 31st in US method when start is not 30th
    }

    #[test]
    fn test_days_360_european_method_end_31st() {
        // =DAYS360("2023-01-15", "2023-05-31", TRUE) in US format
        // =DAYS360("2023-01-15"; "2023-05-31"; TRUE) in German format
        let start_date = create_date(2023, 1, 15);
        let end_date = create_date(2023, 5, 31);
        let result = codcel_days_360(start_date, end_date, Some(true)).unwrap();
        println!("{result}");
        assert_eq!(result, 135); // 31st is treated as 30th in European method
    }

    #[test]
    fn test_days_360_us_method_both_31st() {
        // =DAYS360("2023-01-31", "2023-05-31", FALSE) in US format
        // =DAYS360("2023-01-31"; "2023-05-31"; FALSE) in German format
        let start_date = create_date(2023, 1, 31);
        let end_date = create_date(2023, 5, 31);
        let result = codcel_days_360(start_date, end_date, Some(false)).unwrap();
        println!("{result}");
        assert_eq!(result, 120); // Start 31st is 30th, end 31st is 30th when start is 30th
    }

    #[test]
    fn test_days_360_european_method_both_31st() {
        // =DAYS360("2023-01-31", "2023-05-31", TRUE) in US format
        // =DAYS360("2023-01-31"; "2023-05-31"; TRUE) in German format
        let start_date = create_date(2023, 1, 31);
        let end_date = create_date(2023, 5, 31);
        let result = codcel_days_360(start_date, end_date, Some(true)).unwrap();
        println!("{result}");
        assert_eq!(result, 120); // Both 31st are treated as 30th in European method
    }

    #[test]
    fn test_days_360_default_method() {
        // =DAYS360("2023-01-15", "2023-05-20") in US format
        // =DAYS360("2023-01-15"; "2023-05-20") in German format
        let start_date = create_date(2023, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_days_360(start_date, end_date, None).unwrap();
        println!("{result}");
        assert_eq!(result, 125); // Default is US method
    }

    #[test]
    fn test_days_360_different_years() {
        // =DAYS360("2020-01-15", "2023-05-20", FALSE) in US format
        // =DAYS360("2020-01-15"; "2023-05-20"; FALSE) in German format
        let start_date = create_date(2020, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_days_360(start_date, end_date, Some(false)).unwrap();
        println!("{result}");
        assert_eq!(result, 1205); // 3 years, 4 months, 5 days = 3*360 + 4*30 + 5 = 1205
    }
}
