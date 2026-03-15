// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use chrono::{DateTime, Datelike, Utc};
use std::error::Error;

/// Excel-compatible `DATEDIF` that calculates the difference between two dates.
/// - `start_date`: the starting date (must be before or equal to end_date).
/// - `end_date`: the ending date.
/// - `unit`: the unit of time to return (`"Y"` years, `"M"` months, `"D"` days, `"MD"` days ignoring months/years, `"YM"` months ignoring years, `"YD"` days ignoring years).
///   Returns the difference as an integer or an error for invalid unit or reversed dates.
pub fn codcel_date_dif<S: AsRef<str>>(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    unit: S,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if end_date < start_date {
        return Err("end_date must be greater than or equal to start_date".into());
    }

    let start = start_date.date_naive();
    let end = end_date.date_naive();

    let unit = unit.as_ref().to_uppercase();

    match unit.as_str() {
        "Y" => {
            // Complete years
            let years = end.year() - start.year();
            let adjust = if end.ordinal() < start.ordinal() {
                -1
            } else {
                0
            };
            Ok(years + adjust)
        }
        "M" => {
            // Complete months
            let years = end.year() - start.year();
            let months = end.month() as i32 - start.month() as i32;
            let total_months = years * 12 + months;
            let adjust = if end.day() < start.day() { -1 } else { 0 };
            Ok(total_months + adjust)
        }
        "D" => {
            // Total days
            Ok((end - start).num_days() as i32)
        }
        "MD" => {
            // Days ignoring months and years
            let mut end_day = end.day() as i64;
            let start_day = start.day() as i64;

            if end_day < start_day {
                // Adjust for month rollover
                let prev_month = if end.month() == 1 {
                    12
                } else {
                    end.month() - 1
                };

                let days_in_prev_month = chrono::NaiveDate::from_ymd_opt(
                    if prev_month == 12 {
                        end.year() - 1
                    } else {
                        end.year()
                    },
                    prev_month,
                    1,
                )
                .unwrap()
                .with_day(
                    chrono::NaiveDate::from_ymd_opt(end.year(), prev_month + 1, 1)
                        .unwrap()
                        .day()
                        - 1,
                )
                .unwrap()
                .day();

                end_day += days_in_prev_month as i64;
            }

            Ok((end_day - start_day) as i32)
        }
        "YM" => {
            // Months ignoring years
            let start_month = start.month() as i32;
            let end_month = end.month() as i32;
            let mut months_diff = end_month - start_month;

            if end.day() < start.day() {
                // Adjust if end day is earlier than start day in the month
                months_diff -= 1;
            }

            if months_diff < 0 {
                // Adjust if months_diff is negative due to year difference
                months_diff += 12;
            }

            Ok(months_diff)
        }
        "YD" => {
            // Days ignoring years
            // For YD, Excel calculates the difference as if the dates were in the same year
            // If end month/day is earlier in the year than start month/day, Excel adds 1 day

            // Create dates with the same year to compare month and day
            let same_year_start =
                chrono::NaiveDate::from_ymd_opt(2000, start.month(), start.day()).unwrap();
            let same_year_end =
                chrono::NaiveDate::from_ymd_opt(2000, end.month(), end.day()).unwrap();

            // Calculate the difference in days
            let days_diff = if same_year_end >= same_year_start {
                (same_year_end - same_year_start).num_days()
            } else {
                // If end date is earlier in the year, add days in a leap year (366)
                366 - (same_year_start - same_year_end).num_days()
            };

            Ok(days_diff as i32)
        }
        _ => Err("Invalid unit. Supported units are 'Y', 'M', 'D', 'MD', 'YM', 'YD'.".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn create_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_date_dif_years() {
        // =DATEDIF("2020-01-15", "2023-05-20", "Y") in US format
        // =DATEDIF("2020-01-15"; "2023-05-20"; "Y") in German format
        let start_date = create_date(2020, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_date_dif(start_date, end_date, "Y").unwrap();
        println!("{result}");
        // Expected: 3
        assert_eq!(result, 3);
    }

    #[test]
    fn test_date_dif_months() {
        // =DATEDIF("2020-01-15", "2023-05-20", "M") in US format
        // =DATEDIF("2020-01-15"; "2023-05-20"; "M") in German format
        let start_date = create_date(2020, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_date_dif(start_date, end_date, "M").unwrap();
        println!("{result}");
        // Expected: 40
        assert_eq!(result, 40);
    }

    #[test]
    fn test_date_dif_days() {
        // =DATEDIF("2020-01-15", "2023-05-20", "D") in US format
        // =DATEDIF("2020-01-15"; "2023-05-20"; "D") in German format
        let start_date = create_date(2020, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_date_dif(start_date, end_date, "D").unwrap();
        println!("{result}");
        // Expected: 1221
        assert_eq!(result, 1221);
    }

    #[test]
    fn test_date_dif_days_ignoring_months_years() {
        // =DATEDIF("2020-01-15", "2023-05-20", "MD") in US format
        // =DATEDIF("2020-01-15"; "2023-05-20"; "MD") in German format
        let start_date = create_date(2020, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_date_dif(start_date, end_date, "MD").unwrap();
        println!("{result}");
        // Expected: 5
        assert_eq!(result, 5);
    }

    #[test]
    fn test_date_dif_months_ignoring_years() {
        // =DATEDIF("2020-01-15", "2023-05-20", "YM") in US format
        // =DATEDIF("2020-01-15"; "2023-05-20"; "YM") in German format
        let start_date = create_date(2020, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_date_dif(start_date, end_date, "YM").unwrap();
        println!("{result}");
        // Expected: 4
        assert_eq!(result, 4);
    }

    #[test]
    fn test_date_dif_days_ignoring_years() {
        // =DATEDIF("2020-01-15", "2023-05-20", "YD") in US format
        // =DATEDIF("2020-01-15"; "2023-05-20"; "YD") in German format
        let start_date = create_date(2020, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_date_dif(start_date, end_date, "YD").unwrap();
        println!("{result}");
        // Expected: 126
        assert_eq!(result, 126);
    }

    #[test]
    fn test_date_dif_invalid_unit() {
        // =DATEDIF("2020-01-15", "2023-05-20", "X") in US format
        // =DATEDIF("2020-01-15"; "2023-05-20"; "X") in German format
        let start_date = create_date(2020, 1, 15);
        let end_date = create_date(2023, 5, 20);
        let result = codcel_date_dif(start_date, end_date, "X");
        assert!(result.is_err());
    }

    #[test]
    fn test_date_dif_invalid_dates() {
        // =DATEDIF("2023-05-20", "2020-01-15", "Y") in US format
        // =DATEDIF("2023-05-20"; "2020-01-15"; "Y") in German format
        let start_date = create_date(2023, 5, 20);
        let end_date = create_date(2020, 1, 15);
        let result = codcel_date_dif(start_date, end_date, "Y");
        assert!(result.is_err());
    }
}
