// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use std::error::Error;

/// Calculates the net present value of an investment based on a series of cash flows and a discount rate.
///
/// # Arguments
/// * `rate` - The discount rate over the length of one period.
/// * `cash_flows` - A series of cash flows that correspond to a schedule of payments in periods.
///
/// # Returns
/// The net present value of the investment.
pub fn codcel_npv(rate: f64, cash_flows: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if cash_flows.is_empty() {
        return Err("NPV: Cash flows cannot be empty".into());
    }

    let npv = cash_flows
        .iter()
        .enumerate()
        .map(|(i, &cf)| cf / (1.0 + rate).powi(i as i32 + 1))
        .compensated_sum();

    Ok(npv)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values below are Excel's own cached results, taken from
    // codcel-tests/financial-all.xlsx sheet "Npv". NPV is closed form, so the tolerance
    // is tighter than the 1e-6 used for the iterative solvers.

    #[test]
    fn test_npv_basic_indiv() {
        // =NPV(B1,B2,B3,B4,B5) -> 7547.981695239395
        let result = codcel_npv(0.1, vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 7547.981695239395).abs() < 1e-9,
            "Expected 7547.981695239395, got {result}"
        );
    }

    #[test]
    fn test_npv_basic_range() {
        // =NPV(B1,B2:B5) -> 7547.981695239395
        let result = codcel_npv(0.1, vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 7547.981695239395).abs() < 1e-9,
            "Expected 7547.981695239395, got {result}"
        );
    }

    #[test]
    fn test_npv_five_val_range() {
        // =NPV(B6,B7:B11) -> 9368.780667405535
        let result = codcel_npv(0.08, vec![500.0, 1500.0, 2500.0, 3500.0, 4500.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 9368.780667405535).abs() < 1e-9,
            "Expected 9368.780667405535, got {result}"
        );
    }

    #[test]
    fn test_npv_mixed_range_indiv() {
        // =NPV(B6,B7:B9,B10,B11) -> 9368.780667405535
        let result = codcel_npv(0.08, vec![500.0, 1500.0, 2500.0, 3500.0, 4500.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 9368.780667405535).abs() < 1e-9,
            "Expected 9368.780667405535, got {result}"
        );
    }

    #[test]
    fn test_npv_high_rate() {
        // =NPV(B12,B13:B15) -> 24444.44444444445
        let result = codcel_npv(0.5, vec![10000.0, 20000.0, 30000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 24444.44444444445).abs() < 1e-9,
            "Expected 24444.44444444445, got {result}"
        );
    }

    #[test]
    fn test_npv_zero_rate() {
        // =NPV(B16,B17:B19) -> 6000.0
        let result = codcel_npv(0.0, vec![1000.0, 2000.0, 3000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 6000.0).abs() < 1e-9,
            "Expected 6000.0, got {result}"
        );
    }

    #[test]
    fn test_npv_small_rate() {
        // =NPV(B20,B21:B23) -> 14970.049925104862
        let result = codcel_npv(0.001, vec![5000.0, 5000.0, 5000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 14970.049925104862).abs() < 1e-9,
            "Expected 14970.049925104862, got {result}"
        );
    }

    #[test]
    fn test_npv_all_negative() {
        // =NPV(B24,B25:B27) -> -7024.41690962099
        let result = codcel_npv(0.12, vec![-2000.0, -3000.0, -4000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - -7024.41690962099).abs() < 1e-9,
            "Expected -7024.41690962099, got {result}"
        );
    }

    #[test]
    fn test_npv_mixed_signs() {
        // =NPV(B28,B29:B32) -> 6589.813501238206
        let result = codcel_npv(0.15, vec![-5000.0, 8000.0, -3000.0, 12000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 6589.813501238206).abs() < 1e-9,
            "Expected 6589.813501238206, got {result}"
        );
    }

    #[test]
    fn test_npv_single_cf() {
        // =NPV(B33,B34) -> 7142.857142857142
        let result = codcel_npv(0.05, vec![7500.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 7142.857142857142).abs() < 1e-9,
            "Expected 7142.857142857142, got {result}"
        );
    }

    #[test]
    fn test_npv_double_rate() {
        // =NPV(B35,B36:B38) -> 15000.0
        let result = codcel_npv(1.0, vec![10000.0, 20000.0, 40000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 15000.0).abs() < 1e-9,
            "Expected 15000.0, got {result}"
        );
    }

    #[test]
    fn test_npv_ten_values() {
        // =NPV(B39,B40:B49) -> 2903.590921979832
        let result = codcel_npv(
            0.1,
            vec![
                100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0, 1000.0,
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 2903.590921979832).abs() < 1e-9,
            "Expected 2903.590921979832, got {result}"
        );
    }

    #[test]
    fn test_npv_decimal_cf() {
        // =NPV(B55,B56:B58) -> 6024.355589150748
        let result = codcel_npv(0.07, vec![1234.56, 2345.67, 3456.78]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 6024.355589150748).abs() < 1e-9,
            "Expected 6024.355589150748, got {result}"
        );
    }

    #[test]
    fn test_npv_large_values() {
        // =NPV(B59,B60:B62) -> 5601490.582734754
        let result = codcel_npv(0.03, vec![1000000.0, 2000000.0, 3000000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 5601490.582734754).abs() < 1e-9,
            "Expected 5601490.582734754, got {result}"
        );
    }

    #[test]
    fn test_npv_equal_cf() {
        // =NPV(B63,B64:B68) -> 5378.5599999999995
        let result = codcel_npv(0.25, vec![2000.0, 2000.0, 2000.0, 2000.0, 2000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 5378.5599999999995).abs() < 1e-9,
            "Expected 5378.5599999999995, got {result}"
        );
    }

    #[test]
    fn test_npv_neg_rate() {
        // =NPV(B69,B70:B72) -> 34000.0
        let result = codcel_npv(-0.5, vec![1000.0, 2000.0, 3000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 34000.0).abs() < 1e-9,
            "Expected 34000.0, got {result}"
        );
    }

    #[test]
    fn test_npv_tiny_values() {
        // =NPV(B73,B74:B76) -> 3.4548611111111116
        let result = codcel_npv(0.2, vec![0.5, 1.25, 3.75]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 3.4548611111111116).abs() < 1e-9,
            "Expected 3.4548611111111116, got {result}"
        );
    }

    #[test]
    fn test_npv_indiv_three() {
        // =NPV(B77,B78,B79,B80) -> 4815.927873779113
        let result = codcel_npv(0.1, vec![1000.0, 2000.0, 3000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 4815.927873779113).abs() < 1e-9,
            "Expected 4815.927873779113, got {result}"
        );
    }

    #[test]
    fn test_npv_range_three() {
        // =NPV(B77,B78:B80) -> 4815.927873779113
        let result = codcel_npv(0.1, vec![1000.0, 2000.0, 3000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 4815.927873779113).abs() < 1e-9,
            "Expected 4815.927873779113, got {result}"
        );
    }

    #[test]
    fn test_npv_part_range() {
        // =NPV(B1,B2:B3,B4:B5) -> 7547.981695239395
        let result = codcel_npv(0.1, vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 7547.981695239395).abs() < 1e-9,
            "Expected 7547.981695239395, got {result}"
        );
    }

    #[test]
    fn test_npv_single_first() {
        // =NPV(B1,B2,B3:B5) -> 7547.981695239395
        let result = codcel_npv(0.1, vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 7547.981695239395).abs() < 1e-9,
            "Expected 7547.981695239395, got {result}"
        );
    }

    #[test]
    fn test_npv_single_last() {
        // =NPV(B1,B2:B4,B5) -> 7547.981695239395
        let result = codcel_npv(0.1, vec![1000.0, 2000.0, 3000.0, 4000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 7547.981695239395).abs() < 1e-9,
            "Expected 7547.981695239395, got {result}"
        );
    }

    #[test]
    fn test_npv_reused_rate() {
        // =NPV(B1,B7:B11) -> 8757.194925830943
        let result = codcel_npv(0.1, vec![500.0, 1500.0, 2500.0, 3500.0, 4500.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 8757.194925830943).abs() < 1e-9,
            "Expected 8757.194925830943, got {result}"
        );
    }

    #[test]
    fn test_npv_cross_group() {
        // =NPV(B6,B13:B15) -> 50221.00289590001
        let result = codcel_npv(0.08, vec![10000.0, 20000.0, 30000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 50221.00289590001).abs() < 1e-9,
            "Expected 50221.00289590001, got {result}"
        );
    }

    #[test]
    fn test_npv_high_rate_indiv() {
        // =NPV(B12,B13,B14,B15) -> 24444.44444444445
        let result = codcel_npv(0.5, vec![10000.0, 20000.0, 30000.0]).unwrap();
        println!("{result:?}");
        assert!(
            (result - 24444.44444444445).abs() < 1e-9,
            "Expected 24444.44444444445, got {result}"
        );
    }

    #[test]
    fn test_npv_basic() {
        let cash_flows = vec![100.0, 200.0, 300.0];
        let result = codcel_npv(0.1, cash_flows).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_npv_with_negative_flows() {
        let cash_flows = vec![-100.0, 200.0, 300.0];
        let result = codcel_npv(0.1, cash_flows).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_npv_error_cases() {
        // Empty cash flows
        let cash_flows = vec![];
        assert!(codcel_npv(0.1, cash_flows).is_err());
    }
}
