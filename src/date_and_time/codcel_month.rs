// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use chrono::{DateTime, Datelike, Utc};
use std::error::Error;

/// Excel-compatible `MONTH` that extracts the month from a date.
/// - `date`: a UTC datetime value.
///   Returns the month as an integer (1–12).
pub fn codcel_month(date: DateTime<Utc>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    Ok(date.month() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn create_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_month_january() {
        // =MONTH("2023-01-15") in US format
        // =MONTH("2023-01-15") in German format
        let date = create_date(2023, 1, 15);
        let result = codcel_month(date).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_month_february() {
        // =MONTH("2023-02-15") in US format
        // =MONTH("2023-02-15") in German format
        let date = create_date(2023, 2, 15);
        let result = codcel_month(date).unwrap();
        println!("{result}");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_month_june() {
        // =MONTH("2023-06-15") in US format
        // =MONTH("2023-06-15") in German format
        let date = create_date(2023, 6, 15);
        let result = codcel_month(date).unwrap();
        println!("{result}");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_month_december() {
        // =MONTH("2023-12-31") in US format
        // =MONTH("2023-12-31") in German format
        let date = create_date(2023, 12, 31);
        let result = codcel_month(date).unwrap();
        println!("{result}");
        assert_eq!(result, 12);
    }

    #[test]
    fn test_month_leap_year_february() {
        // =MONTH("2020-02-29") in US format
        // =MONTH("2020-02-29") in German format
        let date = create_date(2020, 2, 29);
        let result = codcel_month(date).unwrap();
        println!("{result}");
        assert_eq!(result, 2);
    }
}
