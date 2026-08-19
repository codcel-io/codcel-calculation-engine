// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::financial::codcel_odd_f_price::codcel_odd_f_price;
use crate::financial::root_finding::solve_rate_numeric;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the yield of a security with an odd first coupon period.
///
/// Implements the logic of Excel's `ODDFYIELD`, solving for the annual yield
/// that produces the observed price given coupon rate, redemption value,
/// payment frequency, and day-count basis.
///
/// Uses Newton-Raphson with numerical derivatives via the working ODDFPRICE function.
///
/// # Arguments
/// * `settlement` - Settlement date of the security.
/// * `maturity` - Maturity date of the security.
/// * `issue` - Issue date of the security.
/// * `first_coupon` - Date of the first coupon payment.
/// * `rate` - Annual coupon rate.
/// * `price` - Observed price per $100 face value.
/// * `redemption` - Redemption value per $100 face value.
/// * `frequency` - Number of coupon payments per year (1, 2, or 4).
/// * `basis` - Optional day-count basis (0-4).
///
/// # Errors
/// Returns an error when inputs are out of range or date order is invalid.
#[allow(clippy::too_many_arguments)]
pub fn codcel_odd_f_yield(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    issue: DateTime<Utc>,
    first_coupon: DateTime<Utc>,
    rate: f64,
    price: f64,
    redemption: f64,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if settlement >= maturity {
        return Err("Settlement date must be before maturity date".into());
    }
    if rate < 0.0 || price <= 0.0 || redemption <= 0.0 {
        return Err("Rate, price and redemption must be positive".into());
    }
    if ![1, 2, 4].contains(&frequency) {
        return Err("Frequency must be 1, 2, or 4".into());
    }
    let basis_val = basis.unwrap_or(0);
    if !(0..=4).contains(&basis_val) {
        return Err("Basis must be 0, 1, 2, 3, or 4".into());
    }

    // Helper closure: compute ODDFPRICE at a given yield
    let price_at_yield = |y: f64| -> Result<f64, Box<dyn Error + Send + Sync>> {
        codcel_odd_f_price(
            settlement,
            maturity,
            issue,
            first_coupon,
            rate,
            y,
            redemption,
            frequency,
            basis,
        )
    };

    // Newton-Raphson with numerical derivative
    let mut yield_guess = rate; // Start with coupon rate as initial guess
    if yield_guess <= 0.0 {
        yield_guess = 0.05; // Default guess for zero-coupon
    }

    // The objective inverts the odd-first-period price: find the yield at which the computed
    // price equals `price`. An inner error surfaces as NaN so the solver rejects that point
    // rather than aborting the search.
    let price_delta = |candidate_yield: f64| -> f64 {
        match price_at_yield(candidate_yield) {
            Ok(computed) => computed - price,
            Err(_) => f64::NAN,
        }
    };

    // Prices are quoted per 100 of face value, so the residual scale is the price itself.
    let scale = price.abs() + redemption.abs();

    solve_rate_numeric(price_delta, yield_guess, scale, "ODDFYIELD")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn dt(y: i32, m: u32, d: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_odd_f_yield_basic() {
        let result = codcel_odd_f_yield(
            dt(2022, 1, 1),
            dt(2027, 1, 1),
            dt(2021, 7, 1),
            dt(2022, 7, 1),
            0.05,
            95.0,
            100.0,
            2,
            Some(0),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_odd_f_yield_at_par() {
        // Exact dates from failing test: serial 44635=2022-03-15, 46553=2027-06-15, 44593=2022-02-01, 44727=2022-06-15
        // Rate: 0.08, Price: 100, Redemption: 100, Freq: 2, Basis: 0
        // Expected (Excel): 0.08000057149191912
        let result = codcel_odd_f_yield(
            dt(2022, 3, 15),
            dt(2027, 6, 15),
            dt(2022, 2, 1),
            dt(2022, 6, 15),
            0.08,
            100.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();

        println!("ODDFYIELD at-par result: {result}");
        assert!(
            (result - 0.08000057149191912).abs() < 0.000001,
            "Expected yield 0.08000057149191912, got {result}"
        );
    }

    use chrono::Duration;

    /// Excel serial to date, so schedules can be copied straight out of a spreadsheet.
    /// The epoch is 1899-12-30, accounting for the Lotus 1-2-3 1900 leap year bug.
    fn excel_serial_to_date(serial: i64) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(1899, 12, 30, 0, 0, 0).unwrap() + Duration::days(serial)
    }

    // Expected values below are Excel's own cached results, taken from
    // codcel-tests/financial-all.xlsx sheet "OddFYield" (column E).

    #[test]
    fn test_odd_f_yield_short_semi_b_0() {
        // =ODDFYIELD(B1,B2,B3,B4,B5,B6,B7,B8,B9) -> 0.050499382022420394
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.06,
            102.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.050499382022420394).abs() < 0.000001,
            "Expected 0.050499382022420394, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_short_semi_b_1() {
        // =ODDFYIELD(B1,B2,B3,B4,B5,B6,B7,B8,B10) -> 0.050494255129592706
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.06,
            102.0,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.050494255129592706).abs() < 0.000001,
            "Expected 0.050494255129592706, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_short_semi_b_2() {
        // =ODDFYIELD(B1,B2,B3,B4,B5,B6,B7,B8,B11) -> 0.050499382022420394
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.06,
            102.0,
            100.0,
            2,
            Some(2),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.050499382022420394).abs() < 0.000001,
            "Expected 0.050499382022420394, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_short_semi_b_3() {
        // =ODDFYIELD(B1,B2,B3,B4,B5,B6,B7,B8,B12) -> 0.05048665035048677
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.06,
            102.0,
            100.0,
            2,
            Some(3),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05048665035048677).abs() < 0.000001,
            "Expected 0.05048665035048677, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_short_semi_b_4() {
        // =ODDFYIELD(B1,B2,B3,B4,B5,B6,B7,B8,B13) -> 0.050499382022420394
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.06,
            102.0,
            100.0,
            2,
            Some(4),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.050499382022420394).abs() < 0.000001,
            "Expected 0.050499382022420394, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_short_semi_no_bas() {
        // =ODDFYIELD(B1,B2,B3,B4,B5,B6,B7,B8) -> 0.050499382022420394
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.06,
            102.0,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.050499382022420394).abs() < 0.000001,
            "Expected 0.050499382022420394, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_short_ann_b_0() {
        // =ODDFYIELD(B27,B28,B29,B30,B5,B6,B7,B14,B9) -> 0.053621051050158026
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45488),
            excel_serial_to_date(46767),
            excel_serial_to_date(45427),
            excel_serial_to_date(45672),
            0.06,
            102.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.053621051050158026).abs() < 0.000001,
            "Expected 0.053621051050158026, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_short_ann_b_1() {
        // =ODDFYIELD(B27,B28,B29,B30,B5,B6,B7,B14,B10) -> 0.05362507379901424
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45488),
            excel_serial_to_date(46767),
            excel_serial_to_date(45427),
            excel_serial_to_date(45672),
            0.06,
            102.0,
            100.0,
            1,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05362507379901424).abs() < 0.000001,
            "Expected 0.05362507379901424, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_short_qtr_b_0() {
        // =ODDFYIELD(B33,B34,B35,B36,B5,B6,B7,B15,B9) -> 0.04682064685177602
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45703),
            excel_serial_to_date(46280),
            excel_serial_to_date(45672),
            excel_serial_to_date(45731),
            0.06,
            102.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04682064685177602).abs() < 0.000001,
            "Expected 0.04682064685177602, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_short_qtr_b_1() {
        // =ODDFYIELD(B33,B34,B35,B36,B5,B6,B7,B15,B10) -> 0.04677645395151198
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45703),
            excel_serial_to_date(46280),
            excel_serial_to_date(45672),
            excel_serial_to_date(45731),
            0.06,
            102.0,
            100.0,
            4,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04677645395151198).abs() < 0.000001,
            "Expected 0.04677645395151198, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_disc_semi_b_0() {
        // =ODDFYIELD(B1,B2,B3,B4,B16,B17,B7,B8,B9) -> 0.0900529579311175
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.08,
            98.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0900529579311175).abs() < 0.000001,
            "Expected 0.0900529579311175, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_disc_semi_b_1() {
        // =ODDFYIELD(B1,B2,B3,B4,B16,B17,B7,B8,B10) -> 0.09005943008099483
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.08,
            98.0,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.09005943008099483).abs() < 0.000001,
            "Expected 0.09005943008099483, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_disc_ann_b_0() {
        // =ODDFYIELD(B27,B28,B29,B30,B16,B17,B7,B14,B9) -> 0.08695879061311224
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45488),
            excel_serial_to_date(46767),
            excel_serial_to_date(45427),
            excel_serial_to_date(45672),
            0.08,
            98.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.08695879061311224).abs() < 0.000001,
            "Expected 0.08695879061311224, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_disc_qtr_b_0() {
        // =ODDFYIELD(B33,B34,B35,B36,B16,B17,B7,B15,B9) -> 0.09373913900324989
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45703),
            excel_serial_to_date(46280),
            excel_serial_to_date(45672),
            excel_serial_to_date(45731),
            0.08,
            98.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.09373913900324989).abs() < 0.000001,
            "Expected 0.09373913900324989, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_par_semi_b_0() {
        // =ODDFYIELD(B1,B2,B3,B4,B18,B19,B7,B8,B9) -> 0.05001193167417072
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
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
            (result - 0.05001193167417072).abs() < 0.000001,
            "Expected 0.05001193167417072, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_par_ann_b_0() {
        // =ODDFYIELD(B27,B28,B29,B30,B18,B19,B7,B14,B9) -> 0.050030832942498604
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45488),
            excel_serial_to_date(46767),
            excel_serial_to_date(45427),
            excel_serial_to_date(45672),
            0.05,
            100.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.050030832942498604).abs() < 0.000001,
            "Expected 0.050030832942498604, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_par_qtr_b_0() {
        // =ODDFYIELD(B33,B34,B35,B36,B18,B19,B7,B15,B9) -> 0.049999968596651165
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45703),
            excel_serial_to_date(46280),
            excel_serial_to_date(45672),
            excel_serial_to_date(45731),
            0.05,
            100.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.049999968596651165).abs() < 0.000001,
            "Expected 0.049999968596651165, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_high_cpn_semi() {
        // =ODDFYIELD(B1,B2,B3,B4,B25,B26,B7,B8,B9) -> 0.07072180684718006
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.1,
            106.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07072180684718006).abs() < 0.000001,
            "Expected 0.07072180684718006, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_high_cpn_b_1() {
        // =ODDFYIELD(B1,B2,B3,B4,B25,B26,B7,B8,B10) -> 0.07070653989711813
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.1,
            106.0,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07070653989711813).abs() < 0.000001,
            "Expected 0.07070653989711813, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_high_cpn_b_3() {
        // =ODDFYIELD(B1,B2,B3,B4,B25,B26,B7,B8,B12) -> 0.07068389955399292
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.1,
            106.0,
            100.0,
            2,
            Some(3),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07068389955399292).abs() < 0.000001,
            "Expected 0.07068389955399292, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_high_cpn_ann() {
        // =ODDFYIELD(B27,B28,B29,B30,B25,B26,B7,B14,B9) -> 0.07978169481238288
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45488),
            excel_serial_to_date(46767),
            excel_serial_to_date(45427),
            excel_serial_to_date(45672),
            0.1,
            106.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07978169481238288).abs() < 0.000001,
            "Expected 0.07978169481238288, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_low_cpn_semi() {
        // =ODDFYIELD(B1,B2,B3,B4,B31,B32,B7,B8,B9) -> 0.0441595367228209
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.03,
            97.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0441595367228209).abs() < 0.000001,
            "Expected 0.0441595367228209, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_low_cpn_b_1() {
        // =ODDFYIELD(B1,B2,B3,B4,B31,B32,B7,B8,B10) -> 0.04416811748021397
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.03,
            97.0,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04416811748021397).abs() < 0.000001,
            "Expected 0.04416811748021397, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_low_cpn_ann() {
        // =ODDFYIELD(B27,B28,B29,B30,B31,B32,B7,B14,B9) -> 0.039357115186185
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45488),
            excel_serial_to_date(46767),
            excel_serial_to_date(45427),
            excel_serial_to_date(45672),
            0.03,
            97.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.039357115186185).abs() < 0.000001,
            "Expected 0.039357115186185, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_low_cpn_qtr() {
        // =ODDFYIELD(B33,B34,B35,B36,B31,B32,B7,B15,B9) -> 0.049822203160273
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45703),
            excel_serial_to_date(46280),
            excel_serial_to_date(45672),
            excel_serial_to_date(45731),
            0.03,
            97.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.049822203160273).abs() < 0.000001,
            "Expected 0.049822203160273, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_semi_b_0() {
        // =ODDFYIELD(B21,B22,B23,B24,B5,B6,B7,B8,B9) -> 0.05196053239342352
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.06,
            102.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05196053239342352).abs() < 0.000001,
            "Expected 0.05196053239342352, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_semi_b_1() {
        // =ODDFYIELD(B21,B22,B23,B24,B5,B6,B7,B8,B10) -> 0.051961135546083746
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.06,
            102.0,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.051961135546083746).abs() < 0.000001,
            "Expected 0.051961135546083746, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_semi_b_2() {
        // =ODDFYIELD(B21,B22,B23,B24,B5,B6,B7,B8,B11) -> 0.0517872561247868
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.06,
            102.0,
            100.0,
            2,
            Some(2),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0517872561247868).abs() < 0.000001,
            "Expected 0.0517872561247868, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_semi_b_3() {
        // =ODDFYIELD(B21,B22,B23,B24,B5,B6,B7,B8,B12) -> 0.051932500665125524
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.06,
            102.0,
            100.0,
            2,
            Some(3),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.051932500665125524).abs() < 0.000001,
            "Expected 0.051932500665125524, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_semi_b_4() {
        // =ODDFYIELD(B21,B22,B23,B24,B5,B6,B7,B8,B13) -> 0.05196053239342352
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.06,
            102.0,
            100.0,
            2,
            Some(4),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05196053239342352).abs() < 0.000001,
            "Expected 0.05196053239342352, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_ann_b_0() {
        // =ODDFYIELD(B39,B40,B41,B42,B5,B6,B7,B14,B9) -> 0.05418213427714807
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47284),
            excel_serial_to_date(45550),
            excel_serial_to_date(46188),
            0.06,
            102.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05418213427714807).abs() < 0.000001,
            "Expected 0.05418213427714807, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_ann_b_1() {
        // =ODDFYIELD(B39,B40,B41,B42,B5,B6,B7,B14,B10) -> 0.05418052247901968
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47284),
            excel_serial_to_date(45550),
            excel_serial_to_date(46188),
            0.06,
            102.0,
            100.0,
            1,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05418052247901968).abs() < 0.000001,
            "Expected 0.05418052247901968, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_qtr_b_0() {
        // =ODDFYIELD(B43,B44,B45,B46,B5,B6,B7,B15,B9) -> 0.04862239666390504
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45689),
            excel_serial_to_date(46371),
            excel_serial_to_date(45627),
            excel_serial_to_date(45823),
            0.06,
            102.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04862239666390504).abs() < 0.000001,
            "Expected 0.04862239666390504, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_qtr_b_1() {
        // =ODDFYIELD(B43,B44,B45,B46,B5,B6,B7,B15,B10) -> 0.048591121133179545
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45689),
            excel_serial_to_date(46371),
            excel_serial_to_date(45627),
            excel_serial_to_date(45823),
            0.06,
            102.0,
            100.0,
            4,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.048591121133179545).abs() < 0.000001,
            "Expected 0.048591121133179545, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_disc_semi() {
        // =ODDFYIELD(B21,B22,B23,B24,B16,B17,B7,B8,B9) -> 0.0872960930678678
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.08,
            98.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0872960930678678).abs() < 0.000001,
            "Expected 0.0872960930678678, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_high_semi() {
        // =ODDFYIELD(B21,B22,B23,B24,B25,B26,B7,B8,B9) -> 0.07521420023058599
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.1,
            106.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07521420023058599).abs() < 0.000001,
            "Expected 0.07521420023058599, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_low_semi() {
        // =ODDFYIELD(B21,B22,B23,B24,B31,B32,B7,B8,B9) -> 0.04118631865810492
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.03,
            97.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04118631865810492).abs() < 0.000001,
            "Expected 0.04118631865810492, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_redemp_105() {
        // =ODDFYIELD(B1,B2,B3,B4,B5,B6,B20,B8,B9) -> 0.07112812580454017
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.06,
            102.0,
            105.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07112812580454017).abs() < 0.000001,
            "Expected 0.07112812580454017, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_redemp_105() {
        // =ODDFYIELD(B21,B22,B23,B24,B5,B6,B20,B8,B9) -> 0.06794771897864046
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.06,
            102.0,
            105.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.06794771897864046).abs() < 0.000001,
            "Expected 0.06794771897864046, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_high_rate_semi() {
        // =ODDFYIELD(B1,B2,B3,B4,B37,B38,B7,B8,B9) -> 0.0997728793779703
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.12,
            104.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0997728793779703).abs() < 0.000001,
            "Expected 0.0997728793779703, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_high_rate_ann() {
        // =ODDFYIELD(B27,B28,B29,B30,B37,B38,B7,B14,B9) -> 0.10589626758805112
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45488),
            excel_serial_to_date(46767),
            excel_serial_to_date(45427),
            excel_serial_to_date(45672),
            0.12,
            104.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.10589626758805112).abs() < 0.000001,
            "Expected 0.10589626758805112, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_high_rate_qtr() {
        // =ODDFYIELD(B33,B34,B35,B36,B37,B38,B7,B15,B9) -> 0.0925497577821185
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45703),
            excel_serial_to_date(46280),
            excel_serial_to_date(45672),
            excel_serial_to_date(45731),
            0.12,
            104.0,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0925497577821185).abs() < 0.000001,
            "Expected 0.0925497577821185, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_classic_short() {
        // =ODDFYIELD(B47,B48,B49,B50,B5,B6,B7,B8,B9) -> 0.053305961938760846
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45611),
            excel_serial_to_date(46813),
            excel_serial_to_date(45580),
            excel_serial_to_date(45717),
            0.06,
            102.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.053305961938760846).abs() < 0.000001,
            "Expected 0.053305961938760846, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_classic_b_1() {
        // =ODDFYIELD(B47,B48,B49,B50,B5,B6,B7,B8,B10) -> 0.05330248418938877
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45611),
            excel_serial_to_date(46813),
            excel_serial_to_date(45580),
            excel_serial_to_date(45717),
            0.06,
            102.0,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05330248418938877).abs() < 0.000001,
            "Expected 0.05330248418938877, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_classic_b_2() {
        // =ODDFYIELD(B47,B48,B49,B50,B5,B6,B7,B8,B11) -> 0.05330511392391657
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45611),
            excel_serial_to_date(46813),
            excel_serial_to_date(45580),
            excel_serial_to_date(45717),
            0.06,
            102.0,
            100.0,
            2,
            Some(2),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05330511392391657).abs() < 0.000001,
            "Expected 0.05330511392391657, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_classic_b_3() {
        // =ODDFYIELD(B47,B48,B49,B50,B5,B6,B7,B8,B12) -> 0.05329858083536727
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45611),
            excel_serial_to_date(46813),
            excel_serial_to_date(45580),
            excel_serial_to_date(45717),
            0.06,
            102.0,
            100.0,
            2,
            Some(3),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05329858083536727).abs() < 0.000001,
            "Expected 0.05329858083536727, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_classic_b_4() {
        // =ODDFYIELD(B47,B48,B49,B50,B5,B6,B7,B8,B13) -> 0.053305961938760846
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45611),
            excel_serial_to_date(46813),
            excel_serial_to_date(45580),
            excel_serial_to_date(45717),
            0.06,
            102.0,
            100.0,
            2,
            Some(4),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.053305961938760846).abs() < 0.000001,
            "Expected 0.053305961938760846, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_classic_disc() {
        // =ODDFYIELD(B47,B48,B49,B50,B16,B17,B7,B8,B9) -> 0.08712703762491926
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45611),
            excel_serial_to_date(46813),
            excel_serial_to_date(45580),
            excel_serial_to_date(45717),
            0.08,
            98.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.08712703762491926).abs() < 0.000001,
            "Expected 0.08712703762491926, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_classic_high() {
        // =ODDFYIELD(B47,B48,B49,B50,B25,B26,B7,B8,B9) -> 0.07897490167428305
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45611),
            excel_serial_to_date(46813),
            excel_serial_to_date(45580),
            excel_serial_to_date(45717),
            0.1,
            106.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07897490167428305).abs() < 0.000001,
            "Expected 0.07897490167428305, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_classic_low() {
        // =ODDFYIELD(B47,B48,B49,B50,B31,B32,B7,B8,B9) -> 0.03980876008996431
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45611),
            excel_serial_to_date(46813),
            excel_serial_to_date(45580),
            excel_serial_to_date(45717),
            0.03,
            97.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.03980876008996431).abs() < 0.000001,
            "Expected 0.03980876008996431, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_redemp_95_semi() {
        // =ODDFYIELD(B1,B2,B3,B4,B5,B6,B51,B8,B9) -> 0.02909677371914508
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.06,
            102.0,
            95.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.02909677371914508).abs() < 0.000001,
            "Expected 0.02909677371914508, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_redemp_95_ann() {
        // =ODDFYIELD(B27,B28,B29,B30,B5,B6,B51,B14,B9) -> 0.04020084589941719
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45488),
            excel_serial_to_date(46767),
            excel_serial_to_date(45427),
            excel_serial_to_date(45672),
            0.06,
            102.0,
            95.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04020084589941719).abs() < 0.000001,
            "Expected 0.04020084589941719, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_v_high_cpn_semi() {
        // =ODDFYIELD(B1,B2,B3,B4,B52,B53,B7,B8,B9) -> 0.0993835785506861
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.15,
            110.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0993835785506861).abs() < 0.000001,
            "Expected 0.0993835785506861, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_v_high_cpn_ann() {
        // =ODDFYIELD(B27,B28,B29,B30,B52,B53,B7,B14,B9) -> 0.11399053693666666
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45488),
            excel_serial_to_date(46767),
            excel_serial_to_date(45427),
            excel_serial_to_date(45672),
            0.15,
            110.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.11399053693666666).abs() < 0.000001,
            "Expected 0.11399053693666666, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_v_low_cpn_semi() {
        // =ODDFYIELD(B1,B2,B3,B4,B54,B55,B7,B8,B9) -> 0.02848106553368347
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45641),
            excel_serial_to_date(46461),
            excel_serial_to_date(45611),
            excel_serial_to_date(45731),
            0.01,
            96.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.02848106553368347).abs() < 0.000001,
            "Expected 0.02848106553368347, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_v_low_cpn_ann() {
        // =ODDFYIELD(B27,B28,B29,B30,B54,B55,B7,B14,B9) -> 0.022002131875582656
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45488),
            excel_serial_to_date(46767),
            excel_serial_to_date(45427),
            excel_serial_to_date(45672),
            0.01,
            96.0,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.022002131875582656).abs() < 0.000001,
            "Expected 0.022002131875582656, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_redm_105_b_1() {
        // =ODDFYIELD(B21,B22,B23,B24,B16,B17,B20,B8,B10) -> 0.10298010552814228
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.08,
            98.0,
            105.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.10298010552814228).abs() < 0.000001,
            "Expected 0.10298010552814228, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_redm_95() {
        // =ODDFYIELD(B21,B22,B23,B24,B5,B6,B51,B8,B9) -> 0.03534664139915193
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.06,
            102.0,
            95.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.03534664139915193).abs() < 0.000001,
            "Expected 0.03534664139915193, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_v_high_cpn() {
        // =ODDFYIELD(B21,B22,B23,B24,B52,B53,B7,B8,B9) -> 0.1062922167915208
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.15,
            110.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1062922167915208).abs() < 0.000001,
            "Expected 0.1062922167915208, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_long_v_low_cpn() {
        // =ODDFYIELD(B21,B22,B23,B24,B54,B55,B7,B8,B9) -> 0.024676187594899545
        let result = codcel_odd_f_yield(
            excel_serial_to_date(45519),
            excel_serial_to_date(46553),
            excel_serial_to_date(45444),
            excel_serial_to_date(45823),
            0.01,
            96.0,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.024676187594899545).abs() < 0.000001,
            "Expected 0.024676187594899545, got {result}"
        );
    }

    #[test]
    fn test_odd_f_yield_error_cases() {
        // Settlement must be before maturity
        assert!(codcel_odd_f_yield(
            dt(2022, 1, 1),
            dt(2022, 1, 1),
            dt(2021, 7, 1),
            dt(2022, 7, 1),
            0.05,
            95.0,
            100.0,
            2,
            Some(0)
        )
        .is_err());
    }
}
