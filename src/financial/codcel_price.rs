// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, TimeZone, Utc};

use crate::compensated_sum::CompensatedSum;

/// Calculates the price per $100 face value for a security that pays periodic interest.
///
/// This mirrors Excel's `PRICE` function, discounting each coupon payment and the
/// redemption value using the supplied yield, frequency, and day-count basis.
///
/// # Arguments
/// * `settlement` - Settlement date of the security.
/// * `maturity` - Maturity date of the security.
/// * `rate` - Annual coupon rate of the security.
/// * `yld` - Annual yield expected by the investor.
/// * `redemption` - Redemption value per $100 face value.
/// * `frequency` - Number of coupon payments per year (1, 2, or 4).
/// * `basis` - Optional day-count basis (0-4).
///
/// # Errors
/// Returns an error when dates, frequency, or basis are invalid.
pub fn codcel_price(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    rate: f64,
    yld: f64,
    redemption: f64,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    if settlement >= maturity {
        return Err("Settlement must be before maturity".into());
    }
    if ![1, 2, 4].contains(&frequency) {
        return Err("Frequency must be 1, 2, or 4".into());
    }

    let basis = basis.unwrap_or(0);
    if !(0..=4).contains(&basis) {
        return Err("Basis must be between 0 and 4".into());
    }

    let coupon = 100.0 * rate / frequency as f64;
    let yld_per_period = yld / frequency as f64;

    let (pcd, ncd) = get_coupon_dates(settlement, maturity, frequency);

    // Calculate day counts based on the specific basis method.
    // Excel's PRICE uses these semantics (matching ExcelFinancialFunctions library):
    //   A = days from previous coupon date to settlement (COUPDAYBS)
    //   E = days in the coupon period (COUPDAYS)
    //   DSC = E - A (always computed as the difference, not independently)
    // The definitions of A and E vary by basis:
    //   Basis 0 (US 30/360):       E = 360/frequency, A = dateDiff360Us(pcd, settlement)
    //   Basis 1 (Actual/Actual):    E = actual days (ncd - pcd), A = actual days (settlement - pcd)
    //   Basis 2 (Actual/360):       E = 360/frequency, A = actual days (settlement - pcd)
    //   Basis 3 (Actual/365):       E = 365/frequency, A = actual days (settlement - pcd)
    //   Basis 4 (European 30/360):  E = 360/frequency, A = dateDiff360Eu(pcd, settlement)
    let (a, e) = match basis {
        0 => {
            // US (NASD) 30/360
            (
                get_days_30_360_us(pcd, settlement),
                360.0 / frequency as f64,
            )
        }
        1 => {
            // Actual/Actual ICMA
            (
                (settlement - pcd).num_days() as f64,
                (ncd - pcd).num_days() as f64,
            )
        }
        2 => {
            // Actual/360
            (
                (settlement - pcd).num_days() as f64,
                360.0 / frequency as f64,
            )
        }
        3 => {
            // Actual/365
            (
                (settlement - pcd).num_days() as f64,
                365.0 / frequency as f64,
            )
        }
        4 => {
            // European 30/360
            (
                get_days_30_360_eu(pcd, settlement),
                360.0 / frequency as f64,
            )
        }
        _ => {
            // Fallback to basis 0
            (
                get_days_30_360_us(pcd, settlement),
                360.0 / frequency as f64,
            )
        }
    };
    let dsc = e - a;
    let a_over_e = a / e;
    let dsc_over_e = dsc / e;

    // Accrued interest = coupon * A/E
    let accrued_interest = coupon * a_over_e;

    // Count number of coupon periods from next coupon date to maturity
    let n = count_coupon_periods(ncd, maturity, frequency);

    if n == 1 {
        // N=1 (one coupon period or less): Excel uses simple interest discounting
        // PRICE = (redemption + coupon) / (1 + DSC/E * yld/frequency) - accrued_interest
        let price = (redemption + coupon) / (1.0 + dsc_over_e * yld_per_period) - accrued_interest;
        Ok(price)
    } else {
        // N>1: compound discounting
        let mut price = CompensatedSum::new();

        // Present value of coupon payments
        for k in 1..=n {
            price.add(
                coupon
                    / crate::portable_math::powf(1.0 + yld_per_period, (k - 1) as f64 + dsc_over_e),
            );
        }

        // Present value of redemption
        price.add(
            redemption
                / crate::portable_math::powf(1.0 + yld_per_period, (n - 1) as f64 + dsc_over_e),
        );

        Ok(price.total() - accrued_interest)
    }
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

    // Work backwards from maturity to find coupon dates around settlement
    loop {
        let prev = add_months(current, -months);
        if prev <= settlement {
            // prev is the previous coupon date (on or before settlement)
            // current is the next coupon date (after settlement)
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

    // For end-of-month dates, preserve end-of-month behavior
    let day = if original_day >= days_in_month(date.year(), date.month() as i32) {
        // Original date was last day of month, so use last day of target month
        max_day_in_target_month
    } else {
        // Use original day or max day in target month, whichever is smaller
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
    use chrono::TimeZone;

    #[test]
    fn test_price_basic() {
        // =PRICE(DATE(2022,1,1),DATE(2027,1,1),0.05,0.06,100,2,0) in US format
        // =PRICE(DATE(2022;1;1);DATE(2027;1;1);0,05;0,06;100;2;0) in German format
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.05, 0.06, 100.0, 2, Some(0)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.7348986).abs() < 0.0001);
    }

    #[test]
    fn test_price_basis_1() {
        // =PRICE(DATE(2022,1,1),DATE(2027,1,1),0.05,0.06,100,2,1) in US format
        // =PRICE(DATE(2022;1;1);DATE(2027;1;1);0,05;0,06;100;2;1) in German format
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.05, 0.06, 100.0, 2, Some(1)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.7348986).abs() < 0.0001);
    }

    #[test]
    fn test_price_basis_2() {
        // =PRICE(DATE(2022,1,1),DATE(2027,1,1),0.05,0.06,100,2,2) in US format
        // =PRICE(DATE(2022;1;1);DATE(2027;1;1);0,05;0,06;100;2;2) in German format
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.05, 0.06, 100.0, 2, Some(2)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.7348986).abs() < 0.0001);
    }

    #[test]
    fn test_price_basis_3() {
        // =PRICE(DATE(2022,1,1),DATE(2027,1,1),0.05,0.06,100,2,3) in US format
        // =PRICE(DATE(2022;1;1);DATE(2027;1;1);0,05;0,06;100;2;3) in German format
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.05, 0.06, 100.0, 2, Some(3)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.7348986).abs() < 0.0001);
    }

    #[test]
    fn test_price_basis_4() {
        // =PRICE(DATE(2022,1,1),DATE(2027,1,1),0.05,0.06,100,2,4) in US format
        // =PRICE(DATE(2022;1;1);DATE(2027;1;1);0,05;0,06;100;2;4) in German format
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.05, 0.06, 100.0, 2, Some(4)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.7348986).abs() < 0.0001);
    }

    #[test]
    fn test_price_formula() {
        // =PRICE(DATE(2023,1,15),DATE(2030,1,15),0.04,0.05,100,2)
        let settlement = Utc.with_ymd_and_hms(2023, 1, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2030, 1, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.04, 0.05, 100.0, 2, None).unwrap();
        println!("{:?}", result);
        assert!((result - 94.154544).abs() < 0.0001);
    }

    #[test]
    fn test_price_error_cases() {
        // Invalid frequency test
        // =PRICE(DATE(2022,1,1),DATE(2027,1,1),0.05,0.06,100,3,0) in US format
        // =PRICE(DATE(2022;1;1);DATE(2027;1;1);0,05;0,06;100;3;0) in German format
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();

        // Invalid frequency
        assert!(codcel_price(settlement, maturity, 0.05, 0.06, 100.0, 3, Some(0)).is_err());

        // Invalid basis test
        // =PRICE(DATE(2022,1,1),DATE(2027,1,1),0.05,0.06,100,2,5) in US format
        // =PRICE(DATE(2022;1;1);DATE(2027;1;1);0,05;0,06;100;2;5) in German format
        // Invalid basis
        assert!(codcel_price(settlement, maturity, 0.05, 0.06, 100.0, 2, Some(5)).is_err());
    }

    // Additional test cases with different parameters

    // Test case 1: Different settlement and maturity dates
    #[test]
    fn test_price_different_dates_basis_0() {
        // =PRICE(DATE(2023,6,15),DATE(2028,12,31),0.045,0.055,100,2,0) in US format
        // =PRICE(DATE(2023;6;15);DATE(2028;12;31);0,045;0,055;100;2;0) in German format
        let settlement = Utc.with_ymd_and_hms(2023, 6, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2028, 12, 31, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.045, 0.055, 100.0, 2, Some(0)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.2761608).abs() < 0.0001);
    }

    #[test]
    fn test_price_different_dates_basis_1() {
        // =PRICE(DATE(2023,6,15),DATE(2028,12,31),0.045,0.055,100,2,1) in US format
        // =PRICE(DATE(2023;6;15);DATE(2028;12;31);0,045;0,055;100;2;1) in German format
        let settlement = Utc.with_ymd_and_hms(2023, 6, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2028, 12, 31, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.045, 0.055, 100.0, 2, Some(1)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.2763406).abs() < 0.0001);
    }
    #[test]
    fn test_price_different_dates_basis_2() {
        // =PRICE(DATE(2023,6,15),DATE(2028,12,31),0.045,0.055,100,2,2) in US format
        // =PRICE(DATE(2023;6;15);DATE(2028;12;31);0,045;0,055;100;2;2) in German format
        let settlement = Utc.with_ymd_and_hms(2023, 6, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2028, 12, 31, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.045, 0.055, 100.0, 2, Some(2)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.2783322).abs() < 0.0001);
    }
    #[test]
    fn test_price_different_dates_basis_3() {
        // =PRICE(DATE(2023,6,15),DATE(2028,12,31),0.045,0.055,100,2,3) in US format
        // =PRICE(DATE(2023;6;15);DATE(2028;12;31);0,045;0,055;100;2;3) in German format
        let settlement = Utc.with_ymd_and_hms(2023, 6, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2028, 12, 31, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.045, 0.055, 100.0, 2, Some(3)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.2733976).abs() < 0.0001);
    }
    #[test]
    fn test_price_different_dates_basis_4() {
        // =PRICE(DATE(2023,6,15),DATE(2028,12,31),0.045,0.055,100,2,4) in US format
        // =PRICE(DATE(2023;6;15);DATE(2028;12;31);0,045;0,055;100;2;4) in German format
        let settlement = Utc.with_ymd_and_hms(2023, 6, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2028, 12, 31, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.045, 0.055, 100.0, 2, Some(4)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.2761608).abs() < 0.0001);
    }

    // Test case 2: Different rate and yield values
    #[test]
    fn test_price_different_rates_basis_0() {
        // =PRICE(DATE(2024,3,1),DATE(2029,3,1),0.07,0.08,100,2,0) in US format
        // =PRICE(DATE(2024;3;1);DATE(2029;3;1);0,07;0,08;100;2;0) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 3, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.07, 0.08, 100.0, 2, Some(0)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.9445521).abs() < 0.0001);
    }

    #[test]
    fn test_price_different_rates_basis_1() {
        // =PRICE(DATE(2024,3,1),DATE(2029,3,1),0.07,0.08,100,2,1) in US format
        // =PRICE(DATE(2024;3;1);DATE(2029;3;1);0,07;0,08;100;2;1) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 3, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.07, 0.08, 100.0, 2, Some(1)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.9445521).abs() < 0.0001);
    }

    #[test]
    fn test_price_different_rates_basis_2() {
        // =PRICE(DATE(2024,3,1),DATE(2029,3,1),0.07,0.08,100,2,2) in US format
        // =PRICE(DATE(2024;3;1);DATE(2029;3;1);0,07;0,08;100;2;2) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 3, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.07, 0.08, 100.0, 2, Some(2)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.9445521).abs() < 0.0001);
    }

    #[test]
    fn test_price_different_rates_basis_3() {
        // =PRICE(DATE(2024,3,1),DATE(2029,3,1),0.07,0.08,100,2,3) in US format
        // =PRICE(DATE(2024;3;1);DATE(2029;3;1);0,07;0,08;100;2;3) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 3, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.07, 0.08, 100.0, 2, Some(3)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.9445521).abs() < 0.0001);
    }

    #[test]
    fn test_price_different_rates_basis_4() {
        // =PRICE(DATE(2024,3,1),DATE(2029,3,1),0.07,0.08,100,2,4) in US format
        // =PRICE(DATE(2024;3;1);DATE(2029;3;1);0,07;0,08;100;2;4) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 3, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 3, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.07, 0.08, 100.0, 2, Some(4)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.9445521).abs() < 0.0001);
    }

    // Test case 3: Different redemption value
    #[test]
    fn test_price_different_redemption_basis_0() {
        // =PRICE(DATE(2024,7,15),DATE(2030,7,15),0.055,0.065,110,2,0) in US format
        // =PRICE(DATE(2024;7;15);DATE(2030;7;15);0,055;0,065;110;2;0) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 7, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2030, 7, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.055, 0.065, 110.0, 2, Some(0)).unwrap();
        println!("{:?}", result);
        assert!((result - 101.909162).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_different_redemption_basis_1() {
        // =PRICE(DATE(2024,7,15),DATE(2030,7,15),0.055,0.065,110,2,1) in US format
        // =PRICE(DATE(2024;7;15);DATE(2030;7;15);0,055;0,065;110;2;1) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 7, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2030, 7, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.055, 0.065, 110.0, 2, Some(1)).unwrap();
        println!("{:?}", result);
        assert!((result - 101.909162).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_different_redemption_basis_2() {
        // =PRICE(DATE(2024,7,15),DATE(2030,7,15),0.055,0.065,110,2,2) in US format
        // =PRICE(DATE(2024;7;15);DATE(2030;7;15);0,055;0,065;110;2;2) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 7, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2030, 7, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.055, 0.065, 110.0, 2, Some(2)).unwrap();
        println!("{:?}", result);
        assert!((result - 101.909162).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_different_redemption_basis_3() {
        // =PRICE(DATE(2024,7,15),DATE(2030,7,15),0.055,0.065,110,2,3) in US format
        // =PRICE(DATE(2024;7;15);DATE(2030;7;15);0,055;0,065;110;2;3) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 7, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2030, 7, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.055, 0.065, 110.0, 2, Some(3)).unwrap();
        println!("{:?}", result);
        assert!((result - 101.909162).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_different_redemption_basis_4() {
        // =PRICE(DATE(2024,7,15),DATE(2030,7,15),0.055,0.065,110,2,4) in US format
        // =PRICE(DATE(2024;7;15);DATE(2030;7;15);0,055;0,065;110;2;4) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 7, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2030, 7, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.055, 0.065, 110.0, 2, Some(4)).unwrap();
        println!("{:?}", result);
        assert!((result - 101.909162).abs() < 0.0001); // Approximate check
    }

    // Test case 4: Different frequency values
    #[test]
    fn test_price_frequency_1_basis_0() {
        // =PRICE(DATE(2024,5,1),DATE(2029,5,1),0.06,0.07,100,1,0) in US format
        // =PRICE(DATE(2024;5;1);DATE(2029;5;1);0,06;0,07;100;1;0) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 5, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.06, 0.07, 100.0, 1, Some(0)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.8998026).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_frequency_1_basis_1() {
        // =PRICE(DATE(2024,5,1),DATE(2029,5,1),0.06,0.07,100,1,1) in US format
        // =PRICE(DATE(2024;5;1);DATE(2029;5;1);0,06;0,07;100;1;1) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 5, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.06, 0.07, 100.0, 1, Some(1)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.8998026).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_frequency_1_basis_2() {
        // =PRICE(DATE(2024,5,1),DATE(2029,5,1),0.06,0.07,100,1,2) in US format
        // =PRICE(DATE(2024;5;1);DATE(2029;5;1);0,06;0,07;100;1;2) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 5, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.06, 0.07, 100.0, 1, Some(2)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.8998026).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_frequency_1_basis_3() {
        // =PRICE(DATE(2024,5,1),DATE(2029,5,1),0.06,0.07,100,1,3) in US format
        // =PRICE(DATE(2024;5;1);DATE(2029;5;1);0,06;0,07;100;1;3) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 5, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.06, 0.07, 100.0, 1, Some(3)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.8998026).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_frequency_1_basis_4() {
        // =PRICE(DATE(2024,5,1),DATE(2029,5,1),0.06,0.07,100,1,4) in US format
        // =PRICE(DATE(2024;5;1);DATE(2029;5;1);0,06;0,07;100;1;4) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 5, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 5, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.06, 0.07, 100.0, 1, Some(4)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.8998026).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_frequency_4_basis_0() {
        // =PRICE(DATE(2024,9,15),DATE(2029,9,15),0.065,0.075,100,4,0) in US format
        // =PRICE(DATE(2024;9;15);DATE(2029;9;15);0,065;0,075;100;4;0) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 9, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 9, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.065, 0.075, 100.0, 4, Some(0)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.8623986).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_frequency_4_basis_1() {
        // =PRICE(DATE(2024,9,15),DATE(2029,9,15),0.065,0.075,100,4,1) in US format
        // =PRICE(DATE(2024;9;15);DATE(2029;9;15);0,065;0,075;100;4;1) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 9, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 9, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.065, 0.075, 100.0, 4, Some(1)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.8623986).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_frequency_4_basis_2() {
        // =PRICE(DATE(2024,9,15),DATE(2029,9,15),0.065,0.075,100,4,2) in US format
        // =PRICE(DATE(2024;9;15);DATE(2029;9;15);0,065;0,075;100;4;2) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 9, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 9, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.065, 0.075, 100.0, 4, Some(2)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.8623986).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_frequency_4_basis_3() {
        // =PRICE(DATE(2024,9,15),DATE(2029,9,15),0.065,0.075,100,4,3) in US format
        // =PRICE(DATE(2024;9;15);DATE(2029;9;15);0,065;0,075;100;4;3) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 9, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 9, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.065, 0.075, 100.0, 4, Some(3)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.8623986).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_frequency_4_basis_4() {
        // =PRICE(DATE(2024,9,15),DATE(2029,9,15),0.065,0.075,100,4,4) in US format
        // =PRICE(DATE(2024;9;15);DATE(2029;9;15);0,065;0,075;100;4;4) in German format
        let settlement = Utc.with_ymd_and_hms(2024, 9, 15, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2029, 9, 15, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.065, 0.075, 100.0, 4, Some(4)).unwrap();
        println!("{:?}", result);
        assert!((result - 95.8623986).abs() < 0.0001); // Approximate check
    }

    // Test case 5: Edge case - settlement date equals maturity date
    #[test]
    fn test_price_settlement_equals_maturity() {
        // =PRICE(DATE(2025,1,1),DATE(2025,1,1),0.05,0.06,100,2,0) in US format
        // =PRICE(DATE(2025;1;1);DATE(2025;1;1);0,05;0,06;100;2;0) in German format
        let settlement = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.05, 0.06, 100.0, 2, Some(0));
        // This should return an error or a specific value
        println!("{:?}", result);
    }

    // Test case 6: Edge case - very long term bond
    #[test]
    fn test_price_long_term_bond_basis_0() {
        // =PRICE(DATE(2025,1,1),DATE(2055,1,1),0.04,0.045,100,2,0) in US format
        // =PRICE(DATE(2025;1;1);DATE(2055;1;1);0,04;0,045;100;2;0) in German format
        let settlement = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2055, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.04, 0.045, 100.0, 2, Some(0)).unwrap();
        println!("{:?}", result);
        assert!((result - 91.8127618).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_long_term_bond_basis_1() {
        // =PRICE(DATE(2025,1,1),DATE(2055,1,1),0.04,0.045,100,2,1) in US format
        // =PRICE(DATE(2025;1;1);DATE(2055;1;1);0,04;0,045;100;2;1) in German format
        let settlement = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2055, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.04, 0.045, 100.0, 2, Some(1)).unwrap();
        println!("{:?}", result);
        assert!((result - 91.8127618).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_long_term_bond_basis_2() {
        // =PRICE(DATE(2025,1,1),DATE(2055,1,1),0.04,0.045,100,2,2) in US format
        // =PRICE(DATE(2025;1;1);DATE(2055;1;1);0,04;0,045;100;2;2) in German format
        let settlement = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2055, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.04, 0.045, 100.0, 2, Some(2)).unwrap();
        println!("{:?}", result);
        assert!((result - 91.8127618).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_long_term_bond_basis_3() {
        // =PRICE(DATE(2025,1,1),DATE(2055,1,1),0.04,0.045,100,2,3) in US format
        // =PRICE(DATE(2025;1;1);DATE(2055;1;1);0,04;0,045;100;2;3) in German format
        let settlement = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2055, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.04, 0.045, 100.0, 2, Some(3)).unwrap();
        println!("{:?}", result);
        assert!((result - 91.8127618).abs() < 0.0001); // Approximate check
    }

    #[test]
    fn test_price_long_term_bond_basis_4() {
        // =PRICE(DATE(2025,1,1),DATE(2055,1,1),0.04,0.045,100,2,4) in US format
        // =PRICE(DATE(2025;1;1);DATE(2055;1;1);0,04;0,045;100;2;4) in German format
        let settlement = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2055, 1, 1, 0, 0, 0).unwrap();
        let result = codcel_price(settlement, maturity, 0.04, 0.045, 100.0, 2, Some(4)).unwrap();
        println!("{:?}", result);
        assert!((result - 91.8127618).abs() < 0.0001); // Approximate check
    }
}
