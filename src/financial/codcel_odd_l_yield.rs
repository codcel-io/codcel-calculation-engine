// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::financial::codcel_odd_l_price::codcel_odd_l_price;
use crate::financial::root_finding::solve_rate_numeric;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the yield of a security with an odd last coupon period.
///
/// Mirrors Excel's `ODDLYIELD`, solving for the annual yield that prices the
/// security given its odd final coupon, redemption amount, frequency, and
/// day-count basis.
///
/// Uses Newton-Raphson with numerical derivatives via the working ODDLPRICE function.
///
/// # Arguments
/// * `settlement` - Settlement date of the security.
/// * `maturity` - Maturity date of the security.
/// * `last_interest` - Date of the last coupon payment before settlement.
/// * `rate` - Annual coupon rate.
/// * `price` - Price per $100 face value.
/// * `redemption` - Redemption value per $100 face value.
/// * `frequency` - Number of coupon payments per year (1, 2, or 4).
/// * `basis` - Optional day-count basis (0-4).
///
/// # Errors
/// Returns an error when inputs are invalid or dates are out of order.
#[allow(clippy::too_many_arguments)]
pub fn codcel_odd_l_yield(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    last_interest: DateTime<Utc>,
    rate: f64,
    price: f64,
    redemption: f64,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if settlement >= maturity {
        return Err("ODDLYIELD: Settlement date must be before maturity date".into());
    }
    if last_interest > settlement {
        return Err(
            "ODDLYIELD: Last interest date must be before or equal to settlement date".into(),
        );
    }
    if rate < 0.0 {
        return Err("ODDLYIELD: Rate cannot be negative".into());
    }
    if price <= 0.0 {
        return Err("ODDLYIELD: Price must be positive".into());
    }
    if redemption <= 0.0 {
        return Err("ODDLYIELD: Redemption value must be positive".into());
    }
    if ![1, 2, 4].contains(&frequency) {
        return Err("ODDLYIELD: Frequency must be 1, 2, or 4".into());
    }

    let basis_val = basis.unwrap_or(0);
    if !(0..=4).contains(&basis_val) {
        return Err("ODDLYIELD: Basis must be between 0 and 4".into());
    }

    // Helper closure: compute ODDLPRICE at a given yield
    let price_at_yield = |y: f64| -> Result<f64, Box<dyn Error + Send + Sync>> {
        codcel_odd_l_price(
            settlement,
            maturity,
            last_interest,
            rate,
            y,
            redemption,
            frequency,
            basis,
        )
    };

    // Special case: when rate=0 and price=redemption, yield is 0
    if rate == 0.0 && (price - redemption).abs() < 1e-12 {
        return Ok(0.0);
    }

    // Invert the odd-last-period price: find the yield at which the computed price equals
    // `price`. An inner error surfaces as NaN so the solver rejects that point rather than
    // aborting the search.
    let price_delta = |candidate_yield: f64| -> f64 {
        match price_at_yield(candidate_yield) {
            Ok(computed) => computed - price,
            Err(_) => f64::NAN,
        }
    };

    // Excel allows negative yields for deeply premium bonds. The solver's bracket search spans
    // negative rates, so no separate negative starting point is needed.
    let yield_guess = if rate > 0.0 { rate } else { 0.05 };

    // Prices are quoted per 100 of face value, so the residual scale is the price itself.
    let scale = price.abs() + redemption.abs();

    solve_rate_numeric(price_delta, yield_guess, scale, "ODDLYIELD")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn excel_date(serial: f64) -> DateTime<Utc> {
        let base = Utc.with_ymd_and_hms(1899, 12, 30, 0, 0, 0).unwrap();
        base + chrono::Duration::days(serial as i64)
    }

    #[test]
    fn test_odd_l_yield_ann_b_0() {
        // rate=0.075, price=101.68, freq=1, basis=0, redemption=100
        // Expected: 0.0600000144352254
        let result = codcel_odd_l_yield(
            excel_date(44986.0),
            excel_date(45519.0),
            excel_date(44788.0),
            0.075,
            101.68,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("ann_b_0: {result}");
        assert!(
            (result - 0.0600000144352254).abs() < 0.000001,
            "Expected 0.0600000144352254, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_basis_0_semi() {
        // rate=0.05, price=99.62, freq=2, basis=0, redemption=100
        // Expected: 0.059836289291541694
        let result = codcel_odd_l_yield(
            excel_date(43136.0),
            excel_date(43266.0),
            excel_date(43023.0),
            0.05,
            99.62,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("basis_0_semi: {result}");
        assert!(
            (result - 0.059836289291541694).abs() < 0.000001,
            "Expected 0.059836289291541694, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_both_zero() {
        // rate=0.0, price=100.0, freq=2, basis=0, redemption=100
        // Expected: 0.0
        let result = codcel_odd_l_yield(
            excel_date(44317.0),
            excel_date(44592.0),
            excel_date(44043.0),
            0.0,
            100.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("both_zero: {result}");
        assert!(result.abs() < 0.000001, "Expected 0.0, got {result}");
    }

    use chrono::Duration;

    /// Excel serial to date, so schedules can be copied straight out of a spreadsheet.
    /// The epoch is 1899-12-30, accounting for the Lotus 1-2-3 1900 leap year bug.
    fn excel_serial_to_date(serial: i64) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(1899, 12, 30, 0, 0, 0).unwrap() + Duration::days(serial)
    }

    // Expected values below are Excel's own cached results, taken from
    // codcel-tests/financial-all.xlsx sheet "OddLYield" (column E).

    #[test]
    fn test_odd_l_yield_semi_b_0() {
        // =ODDLYIELD(B1,B2,B3,B4,B5,B6,B7,B10) -> 0.0392156862745098
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.06,
            100.5,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0392156862745098).abs() < 0.000001,
            "Expected 0.0392156862745098, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_semi_b_1() {
        // =ODDLYIELD(B1,B2,B3,B4,B5,B6,B7,B11) -> 0.0392156862745098
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.06,
            100.5,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0392156862745098).abs() < 0.000001,
            "Expected 0.0392156862745098, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_semi_b_2() {
        // =ODDLYIELD(B1,B2,B3,B4,B5,B6,B7,B12) -> 0.0392156862745098
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.06,
            100.5,
            100.0,
            2,
            Some(2),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0392156862745098).abs() < 0.000001,
            "Expected 0.0392156862745098, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_semi_b_3() {
        // =ODDLYIELD(B1,B2,B3,B4,B5,B6,B7,B13) -> 0.0392156862745098
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.06,
            100.5,
            100.0,
            2,
            Some(3),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0392156862745098).abs() < 0.000001,
            "Expected 0.0392156862745098, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_semi_b_4() {
        // =ODDLYIELD(B1,B2,B3,B4,B5,B6,B7,B14) -> 0.0392156862745098
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.06,
            100.5,
            100.0,
            2,
            Some(4),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0392156862745098).abs() < 0.000001,
            "Expected 0.0392156862745098, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_semi_no_bas() {
        // =ODDLYIELD(B1,B2,B3,B4,B5,B6,B7) -> 0.0392156862745098
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.06,
            100.5,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0392156862745098).abs() < 0.000001,
            "Expected 0.0392156862745098, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_ann_b_0_workbook() {
        // =ODDLYIELD(B15,B16,B17,B4,B5,B6,B8,B10) -> 0.04830917874396135
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.06,
            100.5,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04830917874396135).abs() < 0.000001,
            "Expected 0.04830917874396135, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_ann_b_1() {
        // =ODDLYIELD(B15,B16,B17,B4,B5,B6,B8,B11) -> 0.04839947425707291
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.06,
            100.5,
            100.0,
            1,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04839947425707291).abs() < 0.000001,
            "Expected 0.04839947425707291, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_qtr_b_0() {
        // =ODDLYIELD(B18,B19,B20,B4,B5,B6,B9,B10) -> 0.0392156862745098
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45778),
            excel_serial_to_date(45870),
            excel_serial_to_date(45689),
            0.06,
            100.5,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0392156862745098).abs() < 0.000001,
            "Expected 0.0392156862745098, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_qtr_b_1() {
        // =ODDLYIELD(B18,B19,B20,B4,B5,B6,B9,B11) -> 0.0392156862745098
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45778),
            excel_serial_to_date(45870),
            excel_serial_to_date(45689),
            0.06,
            100.5,
            100.0,
            4,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0392156862745098).abs() < 0.000001,
            "Expected 0.0392156862745098, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_disc_semi_b_0() {
        // =ODDLYIELD(B1,B2,B3,B33,B34,B6,B7,B10) -> 0.1188118811881188
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.08,
            99.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1188118811881188).abs() < 0.000001,
            "Expected 0.1188118811881188, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_disc_semi_b_1() {
        // =ODDLYIELD(B1,B2,B3,B33,B34,B6,B7,B11) -> 0.1188118811881188
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.08,
            99.0,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1188118811881188).abs() < 0.000001,
            "Expected 0.1188118811881188, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_disc_ann_b_0() {
        // =ODDLYIELD(B15,B16,B17,B33,B34,B6,B8,B10) -> 0.0970873786407767
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.08,
            99.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0970873786407767).abs() < 0.000001,
            "Expected 0.0970873786407767, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_disc_qtr_b_0() {
        // =ODDLYIELD(B18,B19,B20,B33,B34,B6,B9,B10) -> 0.1188118811881188
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45778),
            excel_serial_to_date(45870),
            excel_serial_to_date(45689),
            0.08,
            99.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1188118811881188).abs() < 0.000001,
            "Expected 0.1188118811881188, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_par_semi_b_0() {
        // =ODDLYIELD(B1,B2,B3,B47,B48,B6,B7,B10) -> 0.04938271604938271
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.05,
            100.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04938271604938271).abs() < 0.000001,
            "Expected 0.04938271604938271, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_par_ann_b_0() {
        // =ODDLYIELD(B15,B16,B17,B47,B48,B6,B8,B10) -> 0.04878048780487805
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.05,
            100.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04878048780487805).abs() < 0.000001,
            "Expected 0.04878048780487805, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_par_qtr_b_0() {
        // =ODDLYIELD(B18,B19,B20,B47,B48,B6,B9,B10) -> 0.04938271604938271
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45778),
            excel_serial_to_date(45870),
            excel_serial_to_date(45689),
            0.05,
            100.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04938271604938271).abs() < 0.000001,
            "Expected 0.04938271604938271, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_prem_semi_b_0() {
        // =ODDLYIELD(B1,B2,B3,B35,B36,B6,B7,B10) -> -0.018957345971563982
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.1,
            103.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.018957345971563982).abs() < 0.000001,
            "Expected -0.018957345971563982, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_prem_semi_b_1() {
        // =ODDLYIELD(B1,B2,B3,B35,B36,B6,B7,B11) -> -0.018957345971563982
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.1,
            103.0,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.018957345971563982).abs() < 0.000001,
            "Expected -0.018957345971563982, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_prem_semi_b_3() {
        // =ODDLYIELD(B1,B2,B3,B35,B36,B6,B7,B13) -> -0.018957345971563982
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.1,
            103.0,
            100.0,
            2,
            Some(3),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.018957345971563982).abs() < 0.000001,
            "Expected -0.018957345971563982, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_prem_ann_b_0() {
        // =ODDLYIELD(B15,B16,B17,B35,B36,B6,B8,B10) -> 0.037037037037037035
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.1,
            103.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.037037037037037035).abs() < 0.000001,
            "Expected 0.037037037037037035, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_low_semi_b_0() {
        // =ODDLYIELD(B1,B2,B3,B37,B38,B6,B7,B10) -> 0.0906801007556675
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.03,
            98.5,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0906801007556675).abs() < 0.000001,
            "Expected 0.0906801007556675, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_low_semi_b_1() {
        // =ODDLYIELD(B1,B2,B3,B37,B38,B6,B7,B11) -> 0.0906801007556675
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.03,
            98.5,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0906801007556675).abs() < 0.000001,
            "Expected 0.0906801007556675, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_low_ann_b_0() {
        // =ODDLYIELD(B15,B16,B17,B37,B38,B6,B8,B10) -> 0.06
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.03,
            98.5,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.06).abs() < 0.000001,
            "Expected 0.06, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_low_qtr_b_0() {
        // =ODDLYIELD(B18,B19,B20,B37,B38,B6,B9,B10) -> 0.0906801007556675
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45778),
            excel_serial_to_date(45870),
            excel_serial_to_date(45689),
            0.03,
            98.5,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0906801007556675).abs() < 0.000001,
            "Expected 0.0906801007556675, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_4_semi_b_0() {
        // =ODDLYIELD(B21,B22,B23,B4,B5,B6,B7,B10) -> 0.029268292682926828
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45931),
            excel_serial_to_date(45992),
            excel_serial_to_date(45809),
            0.06,
            100.5,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.029268292682926828).abs() < 0.000001,
            "Expected 0.029268292682926828, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_4_semi_b_1() {
        // =ODDLYIELD(B21,B22,B23,B4,B5,B6,B7,B11) -> 0.029268292682926828
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45931),
            excel_serial_to_date(45992),
            excel_serial_to_date(45809),
            0.06,
            100.5,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.029268292682926828).abs() < 0.000001,
            "Expected 0.029268292682926828, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_4_semi_b_2() {
        // =ODDLYIELD(B21,B22,B23,B4,B5,B6,B7,B12) -> 0.029268292682926828
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45931),
            excel_serial_to_date(45992),
            excel_serial_to_date(45809),
            0.06,
            100.5,
            100.0,
            2,
            Some(2),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.029268292682926828).abs() < 0.000001,
            "Expected 0.029268292682926828, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_4_semi_b_3() {
        // =ODDLYIELD(B21,B22,B23,B4,B5,B6,B7,B13) -> 0.029268292682926828
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45931),
            excel_serial_to_date(45992),
            excel_serial_to_date(45809),
            0.06,
            100.5,
            100.0,
            2,
            Some(3),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.029268292682926828).abs() < 0.000001,
            "Expected 0.029268292682926828, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_4_semi_b_4() {
        // =ODDLYIELD(B21,B22,B23,B4,B5,B6,B7,B14) -> 0.029268292682926828
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45931),
            excel_serial_to_date(45992),
            excel_serial_to_date(45809),
            0.06,
            100.5,
            100.0,
            2,
            Some(4),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.029268292682926828).abs() < 0.000001,
            "Expected 0.029268292682926828, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_5_ann_b_0() {
        // =ODDLYIELD(B24,B25,B26,B4,B5,B6,B8,B10) -> 0.04830917874396135
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45853),
            excel_serial_to_date(46037),
            excel_serial_to_date(45672),
            0.06,
            100.5,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04830917874396135).abs() < 0.000001,
            "Expected 0.04830917874396135, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_5_ann_b_1() {
        // =ODDLYIELD(B24,B25,B26,B4,B5,B6,B8,B11) -> 0.04839947425707291
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45853),
            excel_serial_to_date(46037),
            excel_serial_to_date(45672),
            0.06,
            100.5,
            100.0,
            1,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04839947425707291).abs() < 0.000001,
            "Expected 0.04839947425707291, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_near_mat_semi_b_0() {
        // =ODDLYIELD(B27,B28,B29,B4,B5,B6,B7,B10) -> 0.029268292682926828
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45962),
            excel_serial_to_date(46023),
            excel_serial_to_date(45839),
            0.06,
            100.5,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.029268292682926828).abs() < 0.000001,
            "Expected 0.029268292682926828, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_near_mat_semi_b_1() {
        // =ODDLYIELD(B27,B28,B29,B4,B5,B6,B7,B11) -> 0.029106813348040154
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45962),
            excel_serial_to_date(46023),
            excel_serial_to_date(45839),
            0.06,
            100.5,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.029106813348040154).abs() < 0.000001,
            "Expected 0.029106813348040154, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_near_last_qtr_b_0() {
        // =ODDLYIELD(B30,B31,B32,B4,B5,B6,B9,B10) -> 0.0392156862745098
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45748),
            excel_serial_to_date(45839),
            excel_serial_to_date(45658),
            0.06,
            100.5,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0392156862745098).abs() < 0.000001,
            "Expected 0.0392156862745098, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_near_last_qtr_b_1() {
        // =ODDLYIELD(B30,B31,B32,B4,B5,B6,B9,B11) -> 0.0392156862745098
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45748),
            excel_serial_to_date(45839),
            excel_serial_to_date(45658),
            0.06,
            100.5,
            100.0,
            4,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0392156862745098).abs() < 0.000001,
            "Expected 0.0392156862745098, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_high_rt_semi_b_0() {
        // =ODDLYIELD(B1,B2,B3,B39,B40,B6,B7,B10) -> -0.07407407407407407
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.12,
            105.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.07407407407407407).abs() < 0.000001,
            "Expected -0.07407407407407407, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_high_rt_ann_b_0() {
        // =ODDLYIELD(B15,B16,B17,B39,B40,B6,B8,B10) -> 0.018018018018018018
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.12,
            105.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.018018018018018018).abs() < 0.000001,
            "Expected 0.018018018018018018, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_high_rt_qtr_b_0() {
        // =ODDLYIELD(B18,B19,B20,B39,B40,B6,B9,B10) -> -0.07407407407407407
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45778),
            excel_serial_to_date(45870),
            excel_serial_to_date(45689),
            0.12,
            105.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.07407407407407407).abs() < 0.000001,
            "Expected -0.07407407407407407, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_redemp_105_semi() {
        // =ODDLYIELD(B1,B2,B3,B4,B5,B41,B7,B10) -> 0.23529411764705882
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.06,
            100.5,
            105.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.23529411764705882).abs() < 0.000001,
            "Expected 0.23529411764705882, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_redemp_105_ann() {
        // =ODDLYIELD(B15,B16,B17,B4,B5,B41,B8,B10) -> 0.14492753623188406
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.06,
            100.5,
            105.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.14492753623188406).abs() < 0.000001,
            "Expected 0.14492753623188406, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_redemp_95_semi() {
        // =ODDLYIELD(B1,B2,B3,B4,B5,B42,B7,B10) -> -0.1568627450980392
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.06,
            100.5,
            95.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.1568627450980392).abs() < 0.000001,
            "Expected -0.1568627450980392, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_redemp_95_ann() {
        // =ODDLYIELD(B15,B16,B17,B4,B5,B42,B8,B10) -> -0.04830917874396135
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.06,
            100.5,
            95.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.04830917874396135).abs() < 0.000001,
            "Expected -0.04830917874396135, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_v_high_semi_b_0() {
        // =ODDLYIELD(B1,B2,B3,B43,B44,B6,B7,B10) -> -0.15212527964205816
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.15,
            108.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.15212527964205816).abs() < 0.000001,
            "Expected -0.15212527964205816, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_v_high_ann_b_0() {
        // =ODDLYIELD(B15,B16,B17,B43,B44,B6,B8,B10) -> -0.008658008658008658
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.15,
            108.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.008658008658008658).abs() < 0.000001,
            "Expected -0.008658008658008658, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_v_low_semi_b_0() {
        // =ODDLYIELD(B1,B2,B3,B45,B46,B6,B7,B10) -> 0.13367609254498714
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.01,
            97.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.13367609254498714).abs() < 0.000001,
            "Expected 0.13367609254498714, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_v_low_ann_b_0() {
        // =ODDLYIELD(B15,B16,B17,B45,B46,B6,B8,B10) -> 0.07179487179487179
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.01,
            97.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07179487179487179).abs() < 0.000001,
            "Expected 0.07179487179487179, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_sm_cpn_semi_b_0() {
        // =ODDLYIELD(B1,B2,B3,B49,B50,B6,B7,B10) -> 0.025094102885821833
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.005,
            99.5,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.025094102885821833).abs() < 0.000001,
            "Expected 0.025094102885821833, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_sm_cpn_ann_b_0() {
        // =ODDLYIELD(B15,B16,B17,B49,B50,B6,B8,B10) -> 0.015037593984962405
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45870),
            excel_serial_to_date(46054),
            excel_serial_to_date(45689),
            0.005,
            99.5,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.015037593984962405).abs() < 0.000001,
            "Expected 0.015037593984962405, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_sm_cpn_qtr_b_0() {
        // =ODDLYIELD(B18,B19,B20,B49,B50,B6,B9,B10) -> 0.025094102885821833
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45778),
            excel_serial_to_date(45870),
            excel_serial_to_date(45689),
            0.005,
            99.5,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.025094102885821833).abs() < 0.000001,
            "Expected 0.025094102885821833, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_4_disc_b_0() {
        // =ODDLYIELD(B21,B22,B23,B33,B34,B6,B7,B10) -> 0.13770491803278662
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45931),
            excel_serial_to_date(45992),
            excel_serial_to_date(45809),
            0.08,
            99.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.13770491803278662).abs() < 0.000001,
            "Expected 0.13770491803278662, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_4_prem_b_0() {
        // =ODDLYIELD(B21,B22,B23,B35,B36,B6,B7,B10) -> -0.07523510971786808
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45931),
            excel_serial_to_date(45992),
            excel_serial_to_date(45809),
            0.1,
            103.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.07523510971786808).abs() < 0.000001,
            "Expected -0.07523510971786808, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_5_disc_b_0() {
        // =ODDLYIELD(B24,B25,B26,B33,B34,B6,B8,B10) -> 0.0970873786407767
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45853),
            excel_serial_to_date(46037),
            excel_serial_to_date(45672),
            0.08,
            99.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0970873786407767).abs() < 0.000001,
            "Expected 0.0970873786407767, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_5_prem_b_0() {
        // =ODDLYIELD(B24,B25,B26,B35,B36,B6,B8,B10) -> 0.037037037037037035
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45853),
            excel_serial_to_date(46037),
            excel_serial_to_date(45672),
            0.1,
            103.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.037037037037037035).abs() < 0.000001,
            "Expected 0.037037037037037035, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_near_mat_disc_b_0() {
        // =ODDLYIELD(B27,B28,B29,B33,B34,B6,B7,B10) -> 0.13770491803278662
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45962),
            excel_serial_to_date(46023),
            excel_serial_to_date(45839),
            0.08,
            99.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.13770491803278662).abs() < 0.000001,
            "Expected 0.13770491803278662, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_near_mat_prem_b_0() {
        // =ODDLYIELD(B27,B28,B29,B35,B36,B6,B7,B10) -> -0.07523510971786808
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45962),
            excel_serial_to_date(46023),
            excel_serial_to_date(45839),
            0.1,
            103.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.07523510971786808).abs() < 0.000001,
            "Expected -0.07523510971786808, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_r_105_disc_semi() {
        // =ODDLYIELD(B1,B2,B3,B33,B34,B41,B7,B10) -> 0.31683168316831684
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.08,
            99.0,
            105.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.31683168316831684).abs() < 0.000001,
            "Expected 0.31683168316831684, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_r_95_prem_semi() {
        // =ODDLYIELD(B1,B2,B3,B35,B36,B42,B7,B10) -> -0.20853080568720378
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45823),
            excel_serial_to_date(45915),
            excel_serial_to_date(45731),
            0.1,
            103.0,
            95.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.20853080568720378).abs() < 0.000001,
            "Expected -0.20853080568720378, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_7_disc_qtr() {
        // =ODDLYIELD(B30,B31,B32,B33,B34,B6,B9,B10) -> 0.1188118811881188
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45748),
            excel_serial_to_date(45839),
            excel_serial_to_date(45658),
            0.08,
            99.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1188118811881188).abs() < 0.000001,
            "Expected 0.1188118811881188, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_scen_7_prem_qtr() {
        // =ODDLYIELD(B30,B31,B32,B35,B36,B6,B9,B10) -> -0.018957345971563982
        let result = codcel_odd_l_yield(
            excel_serial_to_date(45748),
            excel_serial_to_date(45839),
            excel_serial_to_date(45658),
            0.1,
            103.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.018957345971563982).abs() < 0.000001,
            "Expected -0.018957345971563982, got {result}"
        );
    }

    #[test]
    fn test_odd_l_yield_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let last_interest = Utc.with_ymd_and_hms(2021, 7, 1, 0, 0, 0).unwrap();

        assert!(codcel_odd_l_yield(
            settlement,
            maturity,
            last_interest,
            0.05,
            95.0,
            100.0,
            2,
            Some(0)
        )
        .is_err());
    }
}
