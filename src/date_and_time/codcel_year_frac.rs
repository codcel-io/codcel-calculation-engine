// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_and_time::is_leap_year::is_leap_year;
use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, Utc};
use std::error::Error;

/// Returns the last day of February for the given year (28 or 29).
fn last_day_of_feb(year: i32) -> i32 {
    if is_leap_year(year) {
        29
    } else {
        28
    }
}

/// Excel-compatible `YEARFRAC` that calculates the fraction of a year between two dates.
/// - `start_date`: the starting date.
/// - `end_date`: the ending date.
/// - `basis`: optional day-count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///   Returns the year fraction as a decimal or an error for invalid basis values.
pub fn codcel_year_frac(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let basis = basis.unwrap_or(0);

    // Handle reversed dates - Excel returns the same result regardless of date order
    let (actual_start, actual_end) = if start_date > end_date {
        (end_date, start_date)
    } else {
        (start_date, end_date)
    };

    match basis {
        0 => {
            // US (NASD) 30/360 with February end-of-month adjustments
            // This differs from DAYS360: YEARFRAC has special rules for last day of February
            let sy = actual_start.year();
            let sm = actual_start.month();
            let mut sd = actual_start.day() as i32;
            let ey = actual_end.year();
            let em = actual_end.month();
            let mut ed = actual_end.day() as i32;

            let start_is_last_feb = sm == 2 && sd == last_day_of_feb(sy);
            let end_is_last_feb = em == 2 && ed == last_day_of_feb(ey);

            // Apply adjustment rules in priority order (matching Excel):
            if sd == 31 && ed == 31 {
                sd = 30;
                ed = 30;
            } else if sd == 31 {
                sd = 30;
            } else if sd == 30 && ed == 31 {
                ed = 30;
            } else if start_is_last_feb && end_is_last_feb {
                sd = 30;
                ed = 30;
            } else if start_is_last_feb {
                sd = 30;
            }

            let days = (ey - sy) * 360 + (em as i32 - sm as i32) * 30 + (ed - sd);
            Ok(days as f64 / 360.0)
        }
        1 => {
            // Actual/Actual - Excel-compatible algorithm
            // Uses two sub-algorithms depending on whether the period is ≤ 1 year or > 1 year.
            let sy = actual_start.year();
            let sm = actual_start.month();
            let sd = actual_start.day();
            let ey = actual_end.year();
            let em = actual_end.month();
            let ed = actual_end.day();
            let total_days = (actual_end - actual_start).num_days();

            // Determine if the period "appears to be ≤ 1 year":
            // Same year, or adjacent years where (start_month, start_day) >= (end_month, end_day)
            let short_period = sy == ey || ((sy + 1) == ey && (sm > em || (sm == em && sd >= ed)));

            if short_period {
                // Short period: use 365 or 366 based on Feb 29 presence
                let mut ylength = 365;

                if sy == ey && is_leap_year(sy) {
                    ylength = 366;
                } else {
                    // Check if Feb 29 falls between the dates
                    let feb29_between = |d1: &DateTime<Utc>, d2: &DateTime<Utc>| -> bool {
                        let y1 = d1.year();
                        let y2 = d2.year();
                        if is_leap_year(y1) {
                            if let Some(mar1) = NaiveDate::from_ymd_opt(y1, 3, 1) {
                                let mar1_dt = DateTime::<Utc>::from_naive_utc_and_offset(
                                    mar1.and_time(NaiveTime::MIN),
                                    Utc,
                                );
                                if *d1 < mar1_dt && *d2 >= mar1_dt {
                                    return true;
                                }
                            }
                        }
                        if is_leap_year(y2) {
                            if let Some(mar1) = NaiveDate::from_ymd_opt(y2, 3, 1) {
                                let mar1_dt = DateTime::<Utc>::from_naive_utc_and_offset(
                                    mar1.and_time(NaiveTime::MIN),
                                    Utc,
                                );
                                if *d1 < mar1_dt && *d2 >= mar1_dt {
                                    return true;
                                }
                            }
                        }
                        false
                    };

                    if feb29_between(&actual_start, &actual_end) || (em == 2 && ed == 29) {
                        ylength = 366;
                    }
                }

                Ok(total_days as f64 / ylength as f64)
            } else {
                // Long period (> 1 year): use average year length
                let num_years = (ey - sy) + 1;
                let jan1_start = NaiveDate::from_ymd_opt(sy, 1, 1)
                    .ok_or("YEARFRAC: Invalid date")?
                    .and_hms_opt(0, 0, 0)
                    .ok_or("YEARFRAC: Invalid time")?;
                let jan1_end_plus1 = NaiveDate::from_ymd_opt(ey + 1, 1, 1)
                    .ok_or("YEARFRAC: Invalid date")?
                    .and_hms_opt(0, 0, 0)
                    .ok_or("YEARFRAC: Invalid time")?;
                let jan1_start_dt = DateTime::<Utc>::from_naive_utc_and_offset(jan1_start, Utc);
                let jan1_end_plus1_dt =
                    DateTime::<Utc>::from_naive_utc_and_offset(jan1_end_plus1, Utc);
                let total_year_days = (jan1_end_plus1_dt - jan1_start_dt).num_days();
                let average = total_year_days as f64 / num_years as f64;
                Ok(total_days as f64 / average)
            }
        }
        2 => {
            // Actual/360
            let total_days = (actual_end - actual_start).num_days();
            Ok(total_days as f64 / 360.0)
        }
        3 => {
            // Actual/365
            let total_days = (actual_end - actual_start).num_days();
            Ok(total_days as f64 / 365.0)
        }
        4 => {
            // European 30/360
            let days360 =
                super::codcel_days_360::codcel_days_360(actual_start, actual_end, Some(true))?;
            Ok(days360 as f64 / 360.0)
        }
        _ => Err("YEARFRAC: Invalid basis provided".into()),
    }
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
    fn test_year_frac_basis_0() {
        // =YEARFRAC("2023-01-01", "2023-12-31", 0) in US format
        // =YEARFRAC("2023-01-01"; "2023-12-31"; 0) in German format
        let start_date = create_date(2023, 1, 1);
        let end_date = create_date(2023, 12, 31);
        let result = codcel_year_frac(start_date, end_date, Some(0)).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_year_frac_basis_1_same_year() {
        // =YEARFRAC("2023-01-01", "2023-12-31", 1) in US format
        // =YEARFRAC("2023-01-01"; "2023-12-31"; 1) in German format
        let start_date = create_date(2023, 1, 1);
        let end_date = create_date(2023, 12, 31);
        let result = codcel_year_frac(start_date, end_date, Some(1)).unwrap();
        println!("{result}");
        assert!((result - 0.9972602739726028).abs() < 0.0001);
    }

    #[test]
    fn test_year_frac_basis_1_leap_year() {
        // =YEARFRAC("2024-01-01", "2024-12-31", 1) in US format
        // =YEARFRAC("2024-01-01"; "2024-12-31"; 1) in German format
        let start_date = create_date(2024, 1, 1);
        let end_date = create_date(2024, 12, 31);
        let result = codcel_year_frac(start_date, end_date, Some(1)).unwrap();
        println!("{result}");
        assert!((result - 0.9972677595628415).abs() < 0.0001);
    }

    #[test]
    fn test_year_frac_basis_1_multiple_years() {
        // =YEARFRAC("2023-01-01", "2025-12-31", 1) in US format
        // =YEARFRAC("2023-01-01"; "2025-12-31"; 1) in German format
        // Uses average year length: 1095 days / (1096/3) = 2.997262773722628
        let start_date = create_date(2023, 1, 1);
        let end_date = create_date(2025, 12, 31);
        let result = codcel_year_frac(start_date, end_date, Some(1)).unwrap();
        println!("{result}");
        assert!((result - 2.997262773722628).abs() < 0.000001);
    }

    #[test]
    fn test_year_frac_basis_2() {
        // =YEARFRAC("2023-01-01", "2023-12-31", 2) in US format
        // =YEARFRAC("2023-01-01"; "2023-12-31"; 2) in German format
        let start_date = create_date(2023, 1, 1);
        let end_date = create_date(2023, 12, 31);
        let result = codcel_year_frac(start_date, end_date, Some(2)).unwrap();
        println!("{result}");
        assert!((result - 1.01111111111111).abs() < 0.0001);
    }

    #[test]
    fn test_year_frac_basis_3() {
        // =YEARFRAC("2023-01-01", "2023-12-31", 3) in US format
        // =YEARFRAC("2023-01-01"; "2023-12-31"; 3) in German format
        let start_date = create_date(2023, 1, 1);
        let end_date = create_date(2023, 12, 31);
        let result = codcel_year_frac(start_date, end_date, Some(3)).unwrap();
        println!("{result}");
        assert!((result - 0.9972602739726028).abs() < 0.0001);
    }

    #[test]
    fn test_year_frac_basis_4() {
        // =YEARFRAC("2023-01-01", "2023-12-31", 4) in US format
        // =YEARFRAC("2023-01-01"; "2023-12-31"; 4) in German format
        let start_date = create_date(2023, 1, 1);
        let end_date = create_date(2023, 12, 31);
        let result = codcel_year_frac(start_date, end_date, Some(4)).unwrap();
        println!("{result}");
        assert!((result - 0.9972222222222222).abs() < 0.0001);
    }

    #[test]
    fn test_year_frac_default_basis() {
        // =YEARFRAC("2023-01-01", "2023-12-31") in US format
        // =YEARFRAC("2023-01-01"; "2023-12-31") in German format
        let start_date = create_date(2023, 1, 1);
        let end_date = create_date(2023, 12, 31);
        let result = codcel_year_frac(start_date, end_date, None).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_year_frac_partial_year() {
        // =YEARFRAC("2023-01-01", "2023-06-30", 0) in US format
        // =YEARFRAC("2023-01-01"; "2023-06-30"; 0) in German format
        let start_date = create_date(2023, 1, 1);
        let end_date = create_date(2023, 6, 30);
        let result = codcel_year_frac(start_date, end_date, Some(0)).unwrap();
        println!("{result}");
        assert!((result - 0.49722222222222223).abs() < 0.0001);
    }

    #[test]
    fn test_year_frac_invalid_basis() {
        // =YEARFRAC("2023-01-01", "2023-12-31", 5) in US format
        // =YEARFRAC("2023-01-01"; "2023-12-31"; 5) in German format
        let start_date = create_date(2023, 1, 1);
        let end_date = create_date(2023, 12, 31);
        let result = codcel_year_frac(start_date, end_date, Some(5));
        assert!(result.is_err());
    }

    #[test]
    fn test_year_frac_reversed_dates() {
        // =YEARFRAC("2023-12-31", "2023-01-01", 0) in US format
        // =YEARFRAC("2023-12-31"; "2023-01-01"; 0) in German format
        let start_date = create_date(2023, 12, 31);
        let end_date = create_date(2023, 1, 1);
        let result = codcel_year_frac(start_date, end_date, Some(0)).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_year_frac_basis_0_feb28_to_aug28() {
        // =YEARFRAC(DATE(2023,2,28),DATE(2023,8,28),0) = 0.49444444444444446
        let result =
            codcel_year_frac(create_date(2023, 2, 28), create_date(2023, 8, 28), Some(0)).unwrap();
        assert!((result - 0.49444444444444446).abs() < 0.000001);
    }

    #[test]
    fn test_year_frac_basis_0_feb29_to_aug29() {
        // =YEARFRAC(DATE(2024,2,29),DATE(2024,8,29),0) = 0.49722222222222223
        let result =
            codcel_year_frac(create_date(2024, 2, 29), create_date(2024, 8, 29), Some(0)).unwrap();
        assert!((result - 0.49722222222222223).abs() < 0.000001);
    }

    #[test]
    fn test_year_frac_basis_0_feb28_to_feb28_next_year() {
        // =YEARFRAC(DATE(2023,2,28),DATE(2024,2,28),0) = 0.9944444444444445
        let result =
            codcel_year_frac(create_date(2023, 2, 28), create_date(2024, 2, 28), Some(0)).unwrap();
        assert!((result - 0.9944444444444445).abs() < 0.000001);
    }

    #[test]
    fn test_year_frac_basis_0_feb29_to_feb28_next_year() {
        // =YEARFRAC(DATE(2024,2,29),DATE(2025,2,28),0) = 1.0
        let result =
            codcel_year_frac(create_date(2024, 2, 29), create_date(2025, 2, 28), Some(0)).unwrap();
        assert!((result - 1.0).abs() < 0.000001);
    }

    #[test]
    fn test_year_frac_basis_0_feb29_to_aug31() {
        // =YEARFRAC(DATE(2024,2,29),DATE(2024,8,31),0) = 0.5027777777777778
        let result =
            codcel_year_frac(create_date(2024, 2, 29), create_date(2024, 8, 31), Some(0)).unwrap();
        assert!((result - 0.5027777777777778).abs() < 0.000001);
    }
}
