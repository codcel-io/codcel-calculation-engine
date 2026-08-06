// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Utc};
use std::error::Error;

/// Excel-compatible `DAYS` that returns the number of days between two dates.
/// - `end_date`: the ending date.
/// - `start_date`: the starting date.
///   Returns the number of days (positive if end_date > start_date, negative otherwise).
pub fn codcel_days(
    end_date: DateTime<Utc>,
    start_date: DateTime<Utc>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    Ok((end_date.date_naive() - start_date.date_naive()).num_days() as i32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn create_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_days_same_year() {
        // =DAYS("2023-05-20", "2023-01-15") in US format
        // =DAYS("2023-05-20"; "2023-01-15") in German format
        let end_date = create_date(2023, 5, 20);
        let start_date = create_date(2023, 1, 15);
        let result = codcel_days(end_date, start_date).unwrap();
        println!("{result}");
        assert_eq!(result, 125);
    }

    #[test]
    fn test_days_different_years() {
        // =DAYS("2023-05-20", "2020-01-15") in US format
        // =DAYS("2023-05-20"; "2020-01-15") in German format
        let end_date = create_date(2023, 5, 20);
        let start_date = create_date(2020, 1, 15);
        let result = codcel_days(end_date, start_date).unwrap();
        println!("{result}");
        assert_eq!(result, 1221);
    }

    #[test]
    fn test_days_leap_year() {
        // =DAYS("2020-03-01", "2020-02-28") in US format
        // =DAYS("2020-03-01"; "2020-02-28") in German format
        let end_date = create_date(2020, 3, 1);
        let start_date = create_date(2020, 2, 28);
        let result = codcel_days(end_date, start_date).unwrap();
        println!("{result}");
        assert_eq!(result, 2); // 2 days because 2020 is a leap year
    }

    #[test]
    fn test_days_non_leap_year() {
        // =DAYS("2023-03-01", "2023-02-28") in US format
        // =DAYS("2023-03-01"; "2023-02-28") in German format
        let end_date = create_date(2023, 3, 1);
        let start_date = create_date(2023, 2, 28);
        let result = codcel_days(end_date, start_date).unwrap();
        println!("{result}");
        assert_eq!(result, 1); // 1 day because 2023 is not a leap year
    }

    #[test]
    fn test_days_negative() {
        // =DAYS("2020-01-15", "2023-05-20") in US format
        // =DAYS("2020-01-15"; "2023-05-20") in German format
        let end_date = create_date(2020, 1, 15);
        let start_date = create_date(2023, 5, 20);
        let result = codcel_days(end_date, start_date).unwrap();
        println!("{result}");
        assert_eq!(result, -1221); // Negative because end_date is before start_date
    }

    #[test]
    fn test_days_same_day() {
        // =DAYS("2023-05-20", "2023-05-20") in US format
        // =DAYS("2023-05-20"; "2023-05-20") in German format
        let end_date = create_date(2023, 5, 20);
        let start_date = create_date(2023, 5, 20);
        let result = codcel_days(end_date, start_date).unwrap();
        println!("{result}");
        assert_eq!(result, 0); // 0 days because it's the same day
    }
}
