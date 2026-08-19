// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Calculate the payment for an annuity based on constant payments and a constant interest rate.
///
/// # Arguments
/// - `rate`: The interest rate per period.
/// - `nper`: The number of payment periods.
/// - `pv`: The present value (the amount of money today).
/// - `fv`: The future value (optional, default is 0.0, final balance after last payment).
/// - `type_`: Payment type (0 = end of period, 1 = beginning of period, default is 0).
///
/// # Returns
/// - Returns the payment amount as `f64` or an error if invalid arguments are provided.
///
/// # Errors
/// - Returns an error if `nper` is less than or equal to 0.
///
/// This function replicates the behavior of the Excel `PMT` function.
pub fn codcel_pmt(
    rate: f64,
    nper: f64,
    pv: f64,
    fv: Option<f64>,
    type_: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if nper <= 0.0 {
        return Err("PMT: Number of periods (nper) must be greater than 0.".into());
    }

    let fv = fv.unwrap_or(0.0);
    let type_ = type_.unwrap_or(0);

    if type_ != 0 && type_ != 1 {
        return Err("PMT: Type must be 0 (end of period) or 1 (beginning of period).".into());
    }

    if rate == 0.0 {
        // If the interest rate is 0, the payment is simply the principal + future value divided by periods.
        return Ok(-(pv + fv) / nper);
    }

    let rate_per_period = rate;
    let discount_factor = crate::portable_math::powf(1.0 + rate_per_period, nper);

    // PMT formula derived from the financial formula for present value of an annuity
    let payment = -rate_per_period * (pv * discount_factor + fv)
        / ((1.0 + rate_per_period * type_ as f64) * (discount_factor - 1.0));

    Ok(payment)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values below are Excel's own cached results, taken from
    // codcel-tests/financial-all.xlsx sheet "PMT". This is a closed-form function, so
    // the tolerance is tighter than the 1e-6 used for the iterative solvers.

    #[test]
    fn test_pmt_basic_3_arg() {
        // =PMT(B1,B2,B3) -> -1128.2541002081541
        let result = codcel_pmt(0.05, 12.0, 10000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1128.2541002081541).abs() < 1e-9,
            "Expected -1128.2541002081541, got {result}"
        );
    }

    #[test]
    fn test_pmt_basic_5_arg() {
        // =PMT(B1,B2,B3,B4,B5) -> -1128.2541002081541
        let result = codcel_pmt(0.05, 12.0, 10000.0, Some(0.0), Some(0)).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1128.2541002081541).abs() < 1e-9,
            "Expected -1128.2541002081541, got {result}"
        );
    }

    #[test]
    fn test_pmt_with_fv() {
        // =PMT(B11,B12,B13,B14) -> -567.798658104127
        let result = codcel_pmt(0.1, 24.0, 5000.0, Some(1000.0), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -567.798658104127).abs() < 1e-9,
            "Expected -567.798658104127, got {result}"
        );
    }

    #[test]
    fn test_pmt_zero_rate() {
        // =PMT(B15,B16,B17) -> -1000.0
        let result = codcel_pmt(0.0, 10.0, 10000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1000.0).abs() < 1e-9,
            "Expected -1000.0, got {result}"
        );
    }

    #[test]
    fn test_pmt_zero_rate_fv() {
        // =PMT(B15,B16,B17,B14) -> -1100.0
        let result = codcel_pmt(0.0, 10.0, 10000.0, Some(1000.0), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1100.0).abs() < 1e-9,
            "Expected -1100.0, got {result}"
        );
    }

    #[test]
    fn test_pmt_single_period() {
        // =PMT(B23,B24,B25) -> -1030.0
        let result = codcel_pmt(0.03, 1.0, 1000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1030.0).abs() < 1e-9,
            "Expected -1030.0, got {result}"
        );
    }

    #[test]
    fn test_pmt_long_term() {
        // =PMT(B29,B30,B31) -> -1110.2050194164945
        let result = codcel_pmt(0.005, 120.0, 100000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1110.2050194164945).abs() < 1e-9,
            "Expected -1110.2050194164945, got {result}"
        );
    }

    #[test]
    fn test_pmt_savings_goal() {
        // =PMT(B32,B33,B34,B35) -> -6989.789987916406
        let result = codcel_pmt(0.07, 6.0, 0.0, Some(50000.0), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -6989.789987916406).abs() < 1e-9,
            "Expected -6989.789987916406, got {result}"
        );
    }

    #[test]
    fn test_pmt_savings_goal_beg() {
        // =PMT(B32,B33,B34,B35,B10) -> -6532.51400739851
        let result = codcel_pmt(0.07, 6.0, 0.0, Some(50000.0), Some(1)).unwrap();
        println!("{result:?}");
        assert!(
            (result - -6532.51400739851).abs() < 1e-9,
            "Expected -6532.51400739851, got {result}"
        );
    }

    #[test]
    fn test_pmt_very_high_rate() {
        // =PMT(B41,B42,B43) -> -10322.58064516129
        let result = codcel_pmt(1.0, 5.0, 10000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -10322.58064516129).abs() < 1e-9,
            "Expected -10322.58064516129, got {result}"
        );
    }

    #[test]
    fn test_pmt_save_fv_beg() {
        // =PMT(B44,B45,B46,B47,B48) -> -2504.1474942874142
        let result = codcel_pmt(0.15, 3.0, 0.0, Some(10000.0), Some(1)).unwrap();
        println!("{result:?}");
        assert!(
            (result - -2504.1474942874142).abs() < 1e-9,
            "Expected -2504.1474942874142, got {result}"
        );
    }

    #[test]
    fn test_pmt_small_rate_long() {
        // =PMT(B49,B50,B51) -> -1109.195195707824
        let result = codcel_pmt(0.0025, 240.0, 200000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1109.195195707824).abs() < 1e-9,
            "Expected -1109.195195707824, got {result}"
        );
    }

    #[test]
    fn test_pmt_zero_rate_only() {
        // =PMT(B15,B52,B53) -> -1000.0
        let result = codcel_pmt(0.0, 5.0, 5000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1000.0).abs() < 1e-9,
            "Expected -1000.0, got {result}"
        );
    }

    #[test]
    fn test_pmt_half_rate() {
        // =PMT(B54,B55,B56) -> -5088.2378285221885
        let result = codcel_pmt(0.5, 10.0, 10000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -5088.2378285221885).abs() < 1e-9,
            "Expected -5088.2378285221885, got {result}"
        );
    }

    #[test]
    fn test_pmt_std_mortgage_30() {
        // =PMT(B57,B58,B59,B4,B60) -> -5143.062984627522
        let result = codcel_pmt(0.01, 360.0, 500000.0, Some(0.0), Some(0)).unwrap();
        println!("{result:?}");
        assert!(
            (result - -5143.062984627522).abs() < 1e-9,
            "Expected -5143.062984627522, got {result}"
        );
    }

    #[test]
    fn test_pmt_mortgage_15_beg() {
        // =PMT(B61,B62,B63,B4,B64) -> -6053.746019624688
        let result = codcel_pmt(0.02, 180.0, 300000.0, Some(0.0), Some(1)).unwrap();
        println!("{result:?}");
        assert!(
            (result - -6053.746019624688).abs() < 1e-9,
            "Expected -6053.746019624688, got {result}"
        );
    }

    #[test]
    fn test_pmt_pure_annuity() {
        // =PMT(B65,B66,B67,B68) -> -1241.2664617375929
        let result = codcel_pmt(0.09, 12.0, 0.0, Some(25000.0), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1241.2664617375929).abs() < 1e-9,
            "Expected -1241.2664617375929, got {result}"
        );
    }

    #[test]
    fn test_pmt_pure_annuity_beg() {
        // =PMT(B65,B66,B67,B68,B10) -> -1138.7765704014614
        let result = codcel_pmt(0.09, 12.0, 0.0, Some(25000.0), Some(1)).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1138.7765704014614).abs() < 1e-9,
            "Expected -1138.7765704014614, got {result}"
        );
    }

    #[test]
    fn test_pmt_auto_loan_beg() {
        // =PMT(B69,B70,B71,B72,B73) -> -718.6188801009773
        let result = codcel_pmt(0.0075, 84.0, 45000.0, Some(0.0), Some(1)).unwrap();
        println!("{result:?}");
        assert!(
            (result - -718.6188801009773).abs() < 1e-9,
            "Expected -718.6188801009773, got {result}"
        );
    }

    #[test]
    fn test_pmt_tiny_rate() {
        // =PMT(B74,B75,B76) -> -100.00650011916606
        let result = codcel_pmt(1e-05, 12.0, 1200.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -100.00650011916606).abs() < 1e-9,
            "Expected -100.00650011916606, got {result}"
        );
    }

    #[test]
    fn test_pmt_high_rate_short() {
        // =PMT(B77,B78,B79,B80,B81) -> -42344.17344173441
        let result = codcel_pmt(0.25, 4.0, 100000.0, Some(0.0), Some(0)).unwrap();
        println!("{result:?}");
        assert!(
            (result - -42344.17344173441).abs() < 1e-9,
            "Expected -42344.17344173441, got {result}"
        );
    }

    #[test]
    fn test_pmt_neg_pv_no_fv() {
        // =PMT(B1,B2,B82) -> 564.1270501040771
        let result = codcel_pmt(0.05, 12.0, -5000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 564.1270501040771).abs() < 1e-9,
            "Expected 564.1270501040771, got {result}"
        );
    }

    #[test]
    fn test_pmt_literal_3_arg() {
        // =PMT(0.05,12,10000) -> -1128.2541002081541
        let result = codcel_pmt(0.05, 12.0, 10000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1128.2541002081541).abs() < 1e-9,
            "Expected -1128.2541002081541, got {result}"
        );
    }

    #[test]
    fn test_pmt_literal_5_arg() {
        // =PMT(0.05,12,10000,0,0) -> -1128.2541002081541
        let result = codcel_pmt(0.05, 12.0, 10000.0, Some(0.0), Some(0)).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1128.2541002081541).abs() < 1e-9,
            "Expected -1128.2541002081541, got {result}"
        );
    }

    #[test]
    fn test_pmt_literal_fv() {
        // =PMT(0.1,24,5000,1000) -> -567.798658104127
        let result = codcel_pmt(0.1, 24.0, 5000.0, Some(1000.0), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -567.798658104127).abs() < 1e-9,
            "Expected -567.798658104127, got {result}"
        );
    }

    #[test]
    fn test_pmt_literal_zero() {
        // =PMT(0,10,10000) -> -1000.0
        let result = codcel_pmt(0.0, 10.0, 10000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1000.0).abs() < 1e-9,
            "Expected -1000.0, got {result}"
        );
    }

    #[test]
    fn test_pmt_literal_save() {
        // =PMT(0.07,6,0,50000) -> -6989.789987916406
        let result = codcel_pmt(0.07, 6.0, 0.0, Some(50000.0), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -6989.789987916406).abs() < 1e-9,
            "Expected -6989.789987916406, got {result}"
        );
    }

    #[test]
    fn test_pmt_literal_neg_pv() {
        // =PMT(0.05,12,-5000) -> 564.1270501040771
        let result = codcel_pmt(0.05, 12.0, -5000.0, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 564.1270501040771).abs() < 1e-9,
            "Expected 564.1270501040771, got {result}"
        );
    }

    #[test]
    fn test_pmt_fv_only() {
        // =PMT(B11,B12,0,B14) -> -11.299776350687807
        let result = codcel_pmt(0.1, 24.0, 0.0, Some(1000.0), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -11.299776350687807).abs() < 1e-9,
            "Expected -11.299776350687807, got {result}"
        );
    }

    #[test]
    fn test_pmt_both_pv_fv() {
        // =PMT(B11,B12,B13,B14) -> -567.798658104127
        let result = codcel_pmt(0.1, 24.0, 5000.0, Some(1000.0), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - -567.798658104127).abs() < 1e-9,
            "Expected -567.798658104127, got {result}"
        );
    }

    #[test]
    fn test_pmt_errors() {
        // Test with nper <= 0
        assert!(codcel_pmt(0.01, 0.0, 1000.0, None, None).is_err());
        assert!(codcel_pmt(0.01, -12.0, 1000.0, None, None).is_err());

        // Test with invalid type
        assert!(codcel_pmt(0.01, 12.0, 1000.0, None, Some(2)).is_err());
        assert!(codcel_pmt(0.01, 12.0, 1000.0, None, Some(-1)).is_err());
    }
}
