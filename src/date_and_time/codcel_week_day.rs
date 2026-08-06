// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, Utc};
use std::error::Error;

/// Excel-compatible `WEEKDAY` that returns the day of the week for a date.
/// - `date`: a UTC datetime value.
/// - `return_type`: optional; specifies the numbering system (1 = Sunday–Saturday as 1–7, 2 = Monday–Sunday as 1–7, 3 = Monday–Sunday as 0–6, 11–17 for other variants).
///   Returns the weekday as an integer according to the specified return type, or an error for invalid types.
pub fn codcel_week_day(
    date: DateTime<Utc>,
    return_type: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let weekday = date.weekday();
    let return_type = return_type.unwrap_or(1);

    let result =
        match return_type {
            1 => weekday.number_from_sunday() as i32, // Sunday = 1, ..., Saturday = 7
            2 => weekday.number_from_monday() as i32, // Monday = 1, ..., Sunday = 7
            3 => weekday.num_days_from_monday() as i32, // Monday = 0, ..., Sunday = 6
            11 => weekday.number_from_monday() as i32, // Monday = 1, ..., Sunday = 7
            12 => {
                (if weekday.num_days_from_monday() == 0 {
                    7
                } else {
                    weekday.num_days_from_monday()
                }) as i32
            } // Tuesday = 1, ..., Monday = 7
            13 => {
                (if weekday.num_days_from_monday() == 6 {
                    7
                } else {
                    (weekday.num_days_from_monday() + 1) % 7 + 1
                }) as i32
            } // Wednesday = 1, ..., Tuesday = 7
            14 => {
                (if weekday.num_days_from_monday() == 5 {
                    7
                } else if weekday.num_days_from_monday() == 6 {
                    1
                } else {
                    (weekday.num_days_from_monday() + 2) % 7 + 1
                }) as i32
            } // Thursday = 1, ..., Wednesday = 7
            15 => {
                (if weekday.num_days_from_monday() == 4 {
                    7
                } else if weekday.num_days_from_monday() >= 5 {
                    weekday.num_days_from_monday() - 4
                } else {
                    weekday.num_days_from_monday() + 3
                }) as i32
            } // Friday = 1, ..., Thursday = 7
            16 => {
                (if weekday.num_days_from_monday() == 3 {
                    7
                } else if weekday.num_days_from_monday() >= 4 {
                    weekday.num_days_from_monday() - 3
                } else {
                    weekday.num_days_from_monday() + 4
                }) as i32
            } // Saturday = 1, ..., Friday = 7
            17 => {
                (if weekday.num_days_from_monday() == 2 {
                    7
                } else if weekday.num_days_from_monday() >= 3 {
                    weekday.num_days_from_monday() - 2
                } else {
                    weekday.num_days_from_monday() + 5
                }) as i32
            } // Sunday = 1, ..., Saturday = 7
            _ => return Err(
                "Error: WEEKDAY Invalid return_type. Must be between 1 to 3 and between 11 to 17."
                    .into(),
            ),
        };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn create_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    // May 15, 2023 was a Monday
    #[test]
    fn test_week_day_monday_default() {
        // =WEEKDAY("2023-05-15") in US format
        // =WEEKDAY("2023-05-15") in German format
        let date = create_date(2023, 5, 15);
        let result = codcel_week_day(date, None).unwrap();
        println!("{result}");
        assert_eq!(result, 2); // Monday = 2 in default mode (1)
    }

    #[test]
    fn test_week_day_monday_type_1() {
        // =WEEKDAY("2023-05-15", 1) in US format
        // =WEEKDAY("2023-05-15"; 1) in German format
        let date = create_date(2023, 5, 15);
        let result = codcel_week_day(date, Some(1)).unwrap();
        println!("{result}");
        assert_eq!(result, 2); // Monday = 2 in type 1
    }

    #[test]
    fn test_week_day_monday_type_2() {
        // =WEEKDAY("2023-05-15", 2) in US format
        // =WEEKDAY("2023-05-15"; 2) in German format
        let date = create_date(2023, 5, 15);
        let result = codcel_week_day(date, Some(2)).unwrap();
        println!("{result}");
        assert_eq!(result, 1); // Monday = 1 in type 2
    }

    #[test]
    fn test_week_day_monday_type_3() {
        // =WEEKDAY("2023-05-15", 3) in US format
        // =WEEKDAY("2023-05-15"; 3) in German format
        let date = create_date(2023, 5, 15);
        let result = codcel_week_day(date, Some(3)).unwrap();
        println!("{result}");
        assert_eq!(result, 0); // Monday = 0 in type 3
    }

    // May 21, 2023 was a Sunday
    #[test]
    fn test_week_day_sunday_default() {
        // =WEEKDAY("2023-05-21") in US format
        // =WEEKDAY("2023-05-21") in German format
        let date = create_date(2023, 5, 21);
        let result = codcel_week_day(date, None).unwrap();
        println!("{result}");
        assert_eq!(result, 1); // Sunday = 1 in default mode (1)
    }

    #[test]
    fn test_week_day_sunday_type_2() {
        // =WEEKDAY("2023-05-21", 2) in US format
        // =WEEKDAY("2023-05-21"; 2) in German format
        let date = create_date(2023, 5, 21);
        let result = codcel_week_day(date, Some(2)).unwrap();
        println!("{result}");
        assert_eq!(result, 7); // Sunday = 7 in type 2
    }

    // Test type 11-17
    #[test]
    fn test_week_day_type_11() {
        // =WEEKDAY("2023-05-15", 11) in US format
        // =WEEKDAY("2023-05-15"; 11) in German format
        let date = create_date(2023, 5, 15);
        let result = codcel_week_day(date, Some(11)).unwrap();
        println!("{result}");
        assert_eq!(result, 1); // Monday = 1 in type 11 (same as type 2)
    }

    #[test]
    fn test_week_day_type_12() {
        // =WEEKDAY("2023-05-16", 12) in US format
        // =WEEKDAY("2023-05-16"; 12) in German format
        let date = create_date(2023, 5, 16); // Tuesday
        let result = codcel_week_day(date, Some(12)).unwrap();
        println!("{result}");
        assert_eq!(result, 1); // Tuesday = 1 in type 12
    }

    #[test]
    fn test_week_day_invalid_type() {
        // =WEEKDAY("2023-05-15", 4) in US format
        // =WEEKDAY("2023-05-15"; 4) in German format
        let date = create_date(2023, 5, 15);
        let result = codcel_week_day(date, Some(4));
        assert!(result.is_err());
    }
}
