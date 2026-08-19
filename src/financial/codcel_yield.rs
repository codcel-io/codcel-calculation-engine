// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use super::codcel_price::codcel_price;
use crate::financial::root_finding::solve_rate_numeric;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the yield on a security that pays periodic interest.
///
/// Uses Newton-Raphson iteration to find the yield where
/// PRICE(settlement, maturity, rate, yield, redemption, frequency, basis) == price.
///
/// # Arguments
/// * `settlement` - The settlement date of the security.
/// * `maturity` - The maturity date of the security.
/// * `rate` - The annual coupon rate of the security.
/// * `price` - The price per $100 face value of the security.
/// * `redemption` - The redemption value per $100 face value of the security.
/// * `frequency` - The number of coupon payments per year (1, 2, or 4).
/// * `basis` - The day count basis to use (0-4, optional, defaults to 0).
///
/// # Returns
/// The annual yield of the security.
#[allow(clippy::too_many_arguments)]
pub fn codcel_yield(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    rate: f64,
    price: f64,
    redemption: f64,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if frequency != 1 && frequency != 2 && frequency != 4 {
        return Err(
            "YIELD: Frequency must be 1 (annual), 2 (semiannual), or 4 (quarterly).".into(),
        );
    }

    let basis_val = basis.unwrap_or(0);
    if basis_val > 4 {
        return Err("YIELD: Basis must be between 0 and 4.".into());
    }

    if settlement >= maturity {
        return Err("YIELD: Maturity date must be after settlement date.".into());
    }

    // Initial guess: simple current yield approximation
    let coupon = 100.0 * rate;
    let mut yield_guess = (coupon + (redemption - price) / 5.0) / ((price + redemption) / 2.0);
    if yield_guess.is_nan() || yield_guess <= 0.0 {
        yield_guess = 0.1;
    }

    // The objective inverts PRICE: find the yield at which the computed price equals `price`.
    // PRICE is fallible, so an inner error surfaces as NaN and the solver rejects that point
    // rather than aborting the whole search.
    let price_delta = |candidate_yield: f64| -> f64 {
        match codcel_price(
            settlement,
            maturity,
            rate,
            candidate_yield,
            redemption,
            frequency,
            basis,
        ) {
            Ok(computed) => computed - price,
            Err(_) => f64::NAN,
        }
    };

    // Prices are quoted per 100 of face value, so the residual scale is the price itself.
    let scale = price.abs() + redemption.abs();

    solve_rate_numeric(price_delta, yield_guess, scale, "YIELD")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    use chrono::Duration;

    /// Excel serial to date, so schedules can be copied straight out of a spreadsheet.
    /// The epoch is 1899-12-30, accounting for the Lotus 1-2-3 1900 leap year bug.
    fn excel_serial_to_date(serial: i64) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(1899, 12, 30, 0, 0, 0).unwrap() + Duration::days(serial)
    }

    // Expected values below are Excel's own cached results, taken from
    // codcel-tests/financial-all.xlsx sheet "Yield" (column E).

    #[test]
    fn test_yield_basic() {
        // =YIELD(B1,B2,B3,B4,B5,B6) -> 0.049998163423874246
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            104.69,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.049998163423874246).abs() < 0.000001,
            "Expected 0.049998163423874246, got {result}"
        );
    }

    #[test]
    fn test_yield_bas_0() {
        // =YIELD(B1,B2,B3,B4,B5,B6,B7) -> 0.049998163423874246
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            104.69,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.049998163423874246).abs() < 0.000001,
            "Expected 0.049998163423874246, got {result}"
        );
    }

    #[test]
    fn test_yield_bas_1() {
        // =YIELD(B1,B2,B3,B4,B5,B6,B8) -> 0.049995106604142824
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            104.69,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.049995106604142824).abs() < 0.000001,
            "Expected 0.049995106604142824, got {result}"
        );
    }

    #[test]
    fn test_yield_bas_2() {
        // =YIELD(B1,B2,B3,B4,B5,B6,B9) -> 0.04999352724956309
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            104.69,
            100.0,
            2,
            Some(2),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04999352724956309).abs() < 0.000001,
            "Expected 0.04999352724956309, got {result}"
        );
    }

    #[test]
    fn test_yield_bas_3() {
        // =YIELD(B1,B2,B3,B4,B5,B6,B10) -> 0.04999549603427209
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            104.69,
            100.0,
            2,
            Some(3),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04999549603427209).abs() < 0.000001,
            "Expected 0.04999549603427209, got {result}"
        );
    }

    #[test]
    fn test_yield_bas_4() {
        // =YIELD(B1,B2,B3,B4,B5,B6,B11) -> 0.049998163423874246
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            104.69,
            100.0,
            2,
            Some(4),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.049998163423874246).abs() < 0.000001,
            "Expected 0.049998163423874246, got {result}"
        );
    }

    #[test]
    fn test_yield_ann_bas_0() {
        // =YIELD(B1,B2,B3,B4,B5,B12,B7) -> 0.049831365621974245
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            104.69,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.049831365621974245).abs() < 0.000001,
            "Expected 0.049831365621974245, got {result}"
        );
    }

    #[test]
    fn test_yield_qtr_bas_0() {
        // =YIELD(B1,B2,B3,B4,B5,B13,B7) -> 0.05005660480026067
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            104.69,
            100.0,
            4,
            Some(0),
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05005660480026067).abs() < 0.000001,
            "Expected 0.05005660480026067, got {result}"
        );
    }

    #[test]
    fn test_yield_disc_bond() {
        // =YIELD(B1,B2,B3,B14,B5,B6) -> 0.07127206085929284
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            95.0,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07127206085929284).abs() < 0.000001,
            "Expected 0.07127206085929284, got {result}"
        );
    }

    #[test]
    fn test_yield_prem_bond() {
        // =YIELD(B1,B2,B3,B15,B5,B6) -> 0.03931446806202855
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            110.0,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.03931446806202855).abs() < 0.000001,
            "Expected 0.03931446806202855, got {result}"
        );
    }

    #[test]
    fn test_yield_high_rate() {
        // =YIELD(B1,B2,B16,B4,B5,B6) -> 0.06944230936029355
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.08,
            104.69,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.06944230936029355).abs() < 0.000001,
            "Expected 0.06944230936029355, got {result}"
        );
    }

    #[test]
    fn test_yield_low_rate() {
        // =YIELD(B1,B2,B17,B4,B5,B6) -> 0.02079767239791315
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.03,
            104.69,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.02079767239791315).abs() < 0.000001,
            "Expected 0.02079767239791315, got {result}"
        );
    }

    #[test]
    fn test_yield_high_redm() {
        // =YIELD(B1,B2,B3,B4,B18,B6) -> 0.0652546202208007
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            104.69,
            110.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0652546202208007).abs() < 0.000001,
            "Expected 0.0652546202208007, got {result}"
        );
    }

    #[test]
    fn test_yield_low_redm() {
        // =YIELD(B1,B2,B3,B4,B19,B6) -> 0.03346155397036024
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            104.69,
            90.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.03346155397036024).abs() < 0.000001,
            "Expected 0.03346155397036024, got {result}"
        );
    }

    #[test]
    fn test_yield_short() {
        // =YIELD(B20,B21,B3,B22,B5,B6) -> 0.07053204569170282
        let result = codcel_yield(
            excel_serial_to_date(45383),
            excel_serial_to_date(45748),
            0.06,
            99.0,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07053204569170282).abs() < 0.000001,
            "Expected 0.07053204569170282, got {result}"
        );
    }

    #[test]
    fn test_yield_par_bond() {
        // =YIELD(B20,B21,B3,B23,B5,B6) -> 0.06
        let result = codcel_yield(
            excel_serial_to_date(45383),
            excel_serial_to_date(45748),
            0.06,
            100.0,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.06).abs() < 0.000001,
            "Expected 0.06, got {result}"
        );
    }

    #[test]
    fn test_yield_long() {
        // =YIELD(B1,B24,B3,B4,B5,B6) -> 0.05385115362815531
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(49310),
            0.06,
            104.69,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05385115362815531).abs() < 0.000001,
            "Expected 0.05385115362815531, got {result}"
        );
    }

    #[test]
    fn test_yield_long_ann() {
        // =YIELD(B1,B24,B3,B4,B5,B12) -> 0.05378792054728795
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(49310),
            0.06,
            104.69,
            100.0,
            1,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05378792054728795).abs() < 0.000001,
            "Expected 0.05378792054728795, got {result}"
        );
    }

    #[test]
    fn test_yield_long_qtr() {
        // =YIELD(B1,B24,B3,B4,B5,B13) -> 0.05388302559564229
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(49310),
            0.06,
            104.69,
            100.0,
            4,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.05388302559564229).abs() < 0.000001,
            "Expected 0.05388302559564229, got {result}"
        );
    }

    #[test]
    fn test_yield_deep_disc() {
        // =YIELD(B1,B2,B3,B26,B5,B6) -> 0.08329416874556464
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            90.0,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.08329416874556464).abs() < 0.000001,
            "Expected 0.08329416874556464, got {result}"
        );
    }

    #[test]
    fn test_yield_v_deep_disc() {
        // =YIELD(B1,B2,B3,B27,B5,B6) -> 0.10995317802274851
        let result = codcel_yield(
            excel_serial_to_date(45672),
            excel_serial_to_date(47649),
            0.06,
            80.0,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.10995317802274851).abs() < 0.000001,
            "Expected 0.10995317802274851, got {result}"
        );
    }

    #[test]
    fn test_yield_low_cpn() {
        // =YIELD(B29,B30,B28,B14,B5,B6) -> 0.04630324709360327
        let result = codcel_yield(
            excel_serial_to_date(45474),
            excel_serial_to_date(49126),
            0.04,
            95.0,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04630324709360327).abs() < 0.000001,
            "Expected 0.04630324709360327, got {result}"
        );
    }

    #[test]
    fn test_yield_hi_cpn_sh() {
        // =YIELD(B20,B21,B16,B15,B5,B6) -> -0.01860667341161028
        let result = codcel_yield(
            excel_serial_to_date(45383),
            excel_serial_to_date(45748),
            0.08,
            110.0,
            100.0,
            2,
            None,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.01860667341161028).abs() < 0.000001,
            "Expected -0.01860667341161028, got {result}"
        );
    }

    #[test]
    fn test_yield_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();

        // Maturity date must be after settlement date
        assert!(codcel_yield(settlement, maturity, 0.05, 95.0, 100.0, 2, Some(0)).is_err());

        // Frequency must be 1, 2, or 4
        let maturity = Utc.with_ymd_and_hms(2027, 1, 1, 0, 0, 0).unwrap();
        assert!(codcel_yield(settlement, maturity, 0.05, 95.0, 100.0, 3, Some(0)).is_err());

        // Basis must be between 0 and 4
        assert!(codcel_yield(settlement, maturity, 0.05, 95.0, 100.0, 2, Some(5)).is_err());
    }
}
