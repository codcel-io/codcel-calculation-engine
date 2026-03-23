// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use chrono::{DateTime, Datelike, TimeZone, Utc};
use std::error::Error;

/// Calculates the Macaulay duration for a security with periodic interest payments.
///
/// # Arguments
/// * `settlement` - The settlement date of the security.
/// * `maturity` - The maturity date of the security.
/// * `coupon` - The annual coupon rate of the security.
/// * `yield_rate` - The annual yield of the security.
/// * `frequency` - The number of coupon payments per year (1, 2, or 4).
/// * `basis` - The day count basis to use (0-4, optional, defaults to 0).
///
/// # Returns
/// The Macaulay duration for the security.
pub fn codcel_duration(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    coupon: f64,
    yield_rate: f64,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if settlement >= maturity {
        return Err("DURATION: Settlement must be before maturity".into());
    }
    if coupon < 0.0 {
        return Err("DURATION: Coupon must be non-negative".into());
    }
    if yield_rate < 0.0 {
        return Err("DURATION: Yield must be non-negative".into());
    }
    if ![1, 2, 4].contains(&frequency) {
        return Err("DURATION: Frequency must be 1, 2, or 4".into());
    }

    let basis = basis.unwrap_or(0);
    if ![0, 1, 2, 3, 4].contains(&basis) {
        return Err("Basis must be 0, 1, 2, 3, or 4".into());
    }

    let cpn = 100.0 * coupon / frequency as f64;
    let yld = yield_rate / frequency as f64;

    let (pcd, ncd) = get_coupon_dates(settlement, maturity, frequency);

    let dsc_over_e = match basis {
        0 => {
            let a_days = get_days_30_360_us(pcd, settlement);
            let e_days = 360.0 / frequency as f64;
            (e_days - a_days) / e_days
        }
        1 => {
            let a_days = (settlement - pcd).num_days() as f64;
            let e_days = (ncd - pcd).num_days() as f64;
            (e_days - a_days) / e_days
        }
        2 => {
            let a_days = (settlement - pcd).num_days() as f64;
            let e_days = 360.0 / frequency as f64;
            (e_days - a_days) / e_days
        }
        3 => {
            let a_days = (settlement - pcd).num_days() as f64;
            let e_days = 365.0 / frequency as f64;
            (e_days - a_days) / e_days
        }
        4 => {
            let a_days = get_days_30_360_eu(pcd, settlement);
            let e_days = 360.0 / frequency as f64;
            (e_days - a_days) / e_days
        }
        _ => unreachable!(),
    };

    let n = count_coupon_periods(ncd, maturity, frequency);

    if n == 1 {
        return Ok(dsc_over_e / frequency as f64);
    }

    let mut weighted = 0.0;
    let mut dirty_price = 0.0;

    for k in 1..=n {
        let exponent = (k - 1) as f64 + dsc_over_e;
        let t = exponent / frequency as f64;
        let disc = 1.0 / crate::portable_math::powf(1.0 + yld, exponent);

        if k < n {
            weighted += t * cpn * disc;
            dirty_price += cpn * disc;
        } else {
            weighted += t * (cpn + 100.0) * disc;
            dirty_price += (cpn + 100.0) * disc;
        }
    }

    Ok(weighted / dirty_price)
}

fn count_coupon_periods(start_date: DateTime<Utc>, maturity: DateTime<Utc>, frequency: i32) -> i32 {
    let months_per_period = 12 / frequency;
    let mut count = 0;
    let mut current = start_date;

    while current <= maturity {
        count += 1;
        current = add_months(current, months_per_period);
    }

    count
}

fn get_coupon_dates(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    frequency: i32,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let months = 12 / frequency;
    let mut current = maturity;

    loop {
        let prev = add_months(current, -months);
        if prev <= settlement {
            return (prev, current);
        }
        current = prev;
    }
}

fn add_months(date: DateTime<Utc>, months: i32) -> DateTime<Utc> {
    let mut year = date.year();
    let mut month = date.month() as i32 + months;

    while month > 12 {
        month -= 12;
        year += 1;
    }
    while month < 1 {
        month += 12;
        year -= 1;
    }

    let original_day = date.day();
    let max_day_in_target_month = days_in_month(year, month);

    let day = if original_day >= days_in_month(date.year(), date.month() as i32) {
        max_day_in_target_month
    } else {
        original_day.min(max_day_in_target_month)
    };

    Utc.with_ymd_and_hms(year, month as u32, day, 0, 0, 0)
        .unwrap()
}

fn days_in_month(year: i32, month: i32) -> u32 {
    match month {
        1 => 31,
        2 if is_leap_year(year) => 29,
        2 => 28,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn get_days_30_360_us(start: DateTime<Utc>, end: DateTime<Utc>) -> f64 {
    let d1 = if start.day() == 31 { 30 } else { start.day() };
    let d2 = if end.day() == 31 && d1 == 30 {
        30
    } else {
        end.day()
    };
    ((end.year() - start.year()) * 360
        + (end.month() as i32 - start.month() as i32) * 30
        + (d2 as i32 - d1 as i32)) as f64
}

fn get_days_30_360_eu(start: DateTime<Utc>, end: DateTime<Utc>) -> f64 {
    let d1 = if start.day() == 31 { 30 } else { start.day() };
    let d2 = if end.day() == 31 { 30 } else { end.day() };
    ((end.year() - start.year()) * 360
        + (end.month() as i32 - start.month() as i32) * 30
        + (d2 as i32 - d1 as i32)) as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};

    fn excel_serial_to_date(serial: f64) -> DateTime<Utc> {
        // Excel epoch is 1899-12-30 (accounting for the Lotus 1-2-3 bug)
        let epoch = Utc.with_ymd_and_hms(1899, 12, 30, 0, 0, 0).unwrap();
        epoch + Duration::days(serial as i64)
    }

    #[test]
    fn test_duration_no_basis() {
        // Settlement: 45292, Maturity: 45658, Coupon: 0.08, Yield: 0.09, Freq: 2, No basis
        let settlement = excel_serial_to_date(45292.0);
        let maturity = excel_serial_to_date(45658.0);
        let result = codcel_duration(settlement, maturity, 0.08, 0.09, 2, None).unwrap();
        assert!((result - 0.9806803475688667).abs() < 0.000001);
    }

    #[test]
    fn test_duration_quarterly() {
        // Settlement: 45292, Maturity: 45658, Coupon: 0.08, Yield: 0.09, Freq: 4
        let settlement = excel_serial_to_date(45292.0);
        let maturity = excel_serial_to_date(45658.0);
        let result = codcel_duration(settlement, maturity, 0.08, 0.09, 4, None).unwrap();
        assert!((result - 0.9708120200916209).abs() < 0.000001);
    }

    #[test]
    fn test_duration_zero_coupon_5yr() {
        // Zero coupon: duration = time to maturity
        let settlement = excel_serial_to_date(45000.0);
        let maturity = excel_serial_to_date(46827.0);
        let result = codcel_duration(settlement, maturity, 0.0, 0.06, 2, Some(0)).unwrap();
        assert!((result - 5.0).abs() < 0.000001);
    }

    #[test]
    fn test_duration_10yr_quarterly_basis_0() {
        let settlement = excel_serial_to_date(44927.0);
        let maturity = excel_serial_to_date(48579.0);
        let result = codcel_duration(settlement, maturity, 0.055, 0.06, 4, Some(0)).unwrap();
        assert!((result - 7.703325993796554).abs() < 0.000001);
    }

    #[test]
    fn test_duration_10yr_semi_basis_0() {
        let settlement = excel_serial_to_date(44927.0);
        let maturity = excel_serial_to_date(48579.0);
        let result = codcel_duration(settlement, maturity, 0.055, 0.06, 2, Some(0)).unwrap();
        assert!((result - 7.771168327105721).abs() < 0.000001);
    }

    #[test]
    fn test_duration_short_date_annual_basis_0() {
        // Single period case: settlement: 45351 (2024-02-29), maturity: 45443 (2024-05-31), freq: 1, basis: 0
        let settlement = excel_serial_to_date(45351.0);
        let maturity = excel_serial_to_date(45443.0);
        let result = codcel_duration(settlement, maturity, 0.02, 0.03, 1, Some(0)).unwrap();
        assert!((result - 0.2527777777777778).abs() < 0.000001);
    }

    #[test]
    fn test_duration_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();

        // Settlement must be before maturity
        assert!(codcel_duration(settlement, maturity, 0.05, 0.06, 2, Some(0)).is_err());

        // Coupon must be non-negative
        let maturity = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
        assert!(codcel_duration(settlement, maturity, -0.05, 0.06, 2, Some(0)).is_err());

        // Yield must be non-negative
        assert!(codcel_duration(settlement, maturity, 0.05, -0.06, 2, Some(0)).is_err());

        // Frequency must be 1, 2, or 4
        assert!(codcel_duration(settlement, maturity, 0.05, 0.06, 3, Some(0)).is_err());
    }
}
