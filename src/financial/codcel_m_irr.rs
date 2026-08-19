// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use std::error::Error;

/// Calculates the modified internal rate of return for a series of cash flows.
///
/// # Arguments
/// * `cash_flows` - A vector of cash flows.
/// * `finance_rate` - The interest rate paid on money used in cash flows.
/// * `reinvest_rate` - The interest rate received on reinvestment of cash flows.
///
/// # Returns
/// The modified internal rate of return.
pub fn codcel_m_irr(
    cash_flows: Vec<f64>,
    finance_rate: f64,
    reinvest_rate: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if cash_flows.len() < 2 {
        return Err("MIRR: At least two cash flows are required".into());
    }

    let n_periods = (cash_flows.len() - 1) as f64;

    // Split positive and negative cash flows
    let mut positive_flows: Vec<f64> = vec![0.0; cash_flows.len()];
    let mut negative_flows: Vec<f64> = vec![0.0; cash_flows.len()];

    for (i, &flow) in cash_flows.iter().enumerate() {
        if flow >= 0.0 {
            positive_flows[i] = flow;
        } else {
            negative_flows[i] = flow;
        }
    }

    // Calculate NPV of negative cash flows at finance rate
    let npv_negative = negative_flows
        .iter()
        .enumerate()
        .map(|(i, &flow)| flow / (1.0 + finance_rate).powi(i as i32))
        .compensated_sum();

    if npv_negative == 0.0 {
        return Err("MIRR: No negative cash flows found".into());
    }

    // Calculate future value of positive cash flows at reinvestment rate
    let fv_positive = positive_flows
        .iter()
        .enumerate()
        .map(|(i, &flow)| flow * (1.0 + reinvest_rate).powi((cash_flows.len() - 1 - i) as i32))
        .compensated_sum();

    if fv_positive == 0.0 {
        return Err("MIRR: No positive cash flows found".into());
    }

    // Calculate MIRR
    let mirr = crate::portable_math::powf(-fv_positive / npv_negative, 1.0 / n_periods) - 1.0;

    Ok(mirr)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values below are Excel's own cached results, taken from
    // codcel-tests/financial-all.xlsx sheet "MIrr". This is a closed-form function, so
    // the tolerance is tighter than the 1e-6 used for the iterative solvers.

    #[test]
    fn test_m_irr_mirr_basic_range() {
        // =MIRR(B1:B5,B6,B7) -> 0.1579584529319069
        let result =
            codcel_m_irr(vec![-10000.0, 3000.0, 4200.0, 5800.0, 2000.0], 0.1, 0.12).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1579584529319069).abs() < 1e-9,
            "Expected 0.1579584529319069, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_array_literal() {
        // =MIRR({-10000,3000,4200,5800,2000},0.1,0.12) -> 0.1579584529319069
        let result =
            codcel_m_irr(vec![-10000.0, 3000.0, 4200.0, 5800.0, 2000.0], 0.1, 0.12).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1579584529319069).abs() < 1e-9,
            "Expected 0.1579584529319069, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_equal_pmts() {
        // =MIRR(B8:B13,B14,B15) -> 0.07082760437040503
        let result = codcel_m_irr(
            vec![-5000.0, 1200.0, 1200.0, 1200.0, 1200.0, 1200.0],
            0.08,
            0.08,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07082760437040503).abs() < 1e-9,
            "Expected 0.07082760437040503, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_equal_rates() {
        // =MIRR(B8:B13,B14,B14) -> 0.07082760437040503
        let result = codcel_m_irr(
            vec![-5000.0, 1200.0, 1200.0, 1200.0, 1200.0, 1200.0],
            0.08,
            0.08,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07082760437040503).abs() < 1e-9,
            "Expected 0.07082760437040503, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_many_periods() {
        // =MIRR(B16:B24,B25,B26) -> 0.08618336818171568
        let result = codcel_m_irr(
            vec![
                -50000.0, 8000.0, 9000.0, 10000.0, 11000.0, 12000.0, 13000.0, 7000.0, 5000.0,
            ],
            0.05,
            0.07,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.08618336818171568).abs() < 1e-9,
            "Expected 0.08618336818171568, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_high_fin_rate() {
        // =MIRR(B27:B31,B32,B33) -> -0.036437332381664356
        let result = codcel_m_irr(
            vec![-100000.0, 20000.0, 20000.0, 20000.0, 20000.0],
            0.15,
            0.05,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.036437332381664356).abs() < 1e-9,
            "Expected -0.036437332381664356, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_mixed_signs() {
        // =MIRR(B34:B37,B38,B39) -> 0.15703852965509713
        let result = codcel_m_irr(vec![-10000.0, 5000.0, -2000.0, 12000.0], 0.1, 0.1).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.15703852965509713).abs() < 1e-9,
            "Expected 0.15703852965509713, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_two_period() {
        // =MIRR(B40:B41,B42,B43) -> 0.19999999999999996
        let result = codcel_m_irr(vec![-1000.0, 1200.0], 0.1, 0.1).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.19999999999999996).abs() < 1e-9,
            "Expected 0.19999999999999996, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_high_return() {
        // =MIRR(B44:B45,B46,B47) -> 4.0
        let result = codcel_m_irr(vec![-1000.0, 5000.0], 0.05, 0.05).unwrap();
        println!("{result:?}");
        assert!((result - 4.0).abs() < 1e-9, "Expected 4.0, got {result}");
    }

    #[test]
    fn test_m_irr_mirr_with_zeros() {
        // =MIRR(B48:B51,B52,B53) -> 0.12222912055942481
        let result = codcel_m_irr(vec![0.0, -7500.0, 0.0, 10000.0], 0.06, 0.09).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.12222912055942481).abs() < 1e-9,
            "Expected 0.12222912055942481, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_large_invest() {
        // =MIRR(B54:B57,B58,B59) -> -0.009421825331612066
        let result = codcel_m_irr(vec![-250000.0, 50000.0, 75000.0, 100000.0], 0.07, 0.1).unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.009421825331612066).abs() < 1e-9,
            "Expected -0.009421825331612066, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_small_vals() {
        // =MIRR(B60:B65,B66,B67) -> 0.11701262143477176
        let result = codcel_m_irr(vec![-100.0, 10.0, 20.0, 30.0, 40.0, 50.0], 0.09, 0.11).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.11701262143477176).abs() < 1e-9,
            "Expected 0.11701262143477176, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_both_zero_rate() {
        // =MIRR(B1:B5,B68,B69) -> 0.1066819197003217
        let result =
            codcel_m_irr(vec![-10000.0, 3000.0, 4200.0, 5800.0, 2000.0], 0.0, 0.0).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1066819197003217).abs() < 1e-9,
            "Expected 0.1066819197003217, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_zero_fin_rate() {
        // =MIRR(B1:B5,B70,B71) -> 0.1579584529319069
        let result =
            codcel_m_irr(vec![-10000.0, 3000.0, 4200.0, 5800.0, 2000.0], 0.0, 0.12).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1579584529319069).abs() < 1e-9,
            "Expected 0.1579584529319069, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_zero_rein_rate() {
        // =MIRR(B1:B5,B72,B73) -> 0.1066819197003217
        let result =
            codcel_m_irr(vec![-10000.0, 3000.0, 4200.0, 5800.0, 2000.0], 0.1, 0.0).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1066819197003217).abs() < 1e-9,
            "Expected 0.1066819197003217, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_neg_result() {
        // =MIRR(B74:B78,B79,B80) -> -0.18974360692743508
        let result =
            codcel_m_irr(vec![-10000.0, 1000.0, 1000.0, 1000.0, 1000.0], 0.05, 0.05).unwrap();
        println!("{result:?}");
        assert!(
            (result - -0.18974360692743508).abs() < 1e-9,
            "Expected -0.18974360692743508, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_short_range() {
        // =MIRR(B81:B83,B84,B85) -> 0.10679718105893277
        let result = codcel_m_irr(vec![-20000.0, 15000.0, 8000.0], 0.05, 0.1).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.10679718105893277).abs() < 1e-9,
            "Expected 0.10679718105893277, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_high_fin_low_rein() {
        // =MIRR(B1:B5,B86,B87) -> 0.1195150945107546
        let result =
            codcel_m_irr(vec![-10000.0, 3000.0, 4200.0, 5800.0, 2000.0], 0.2, 0.03).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1195150945107546).abs() < 1e-9,
            "Expected 0.1195150945107546, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_large_project() {
        // =MIRR(B88:B92,B93,B94) -> 0.10654968440138957
        let result = codcel_m_irr(
            vec![-500000.0, 100000.0, 150000.0, 200000.0, 250000.0],
            0.04,
            0.06,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.10654968440138957).abs() < 1e-9,
            "Expected 0.10654968440138957, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_equal_high_rates() {
        // =MIRR(B1:B5,B95,B96) -> 0.21331655063455268
        let result =
            codcel_m_irr(vec![-10000.0, 3000.0, 4200.0, 5800.0, 2000.0], 0.25, 0.25).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.21331655063455268).abs() < 1e-9,
            "Expected 0.21331655063455268, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_cell_ref_rates() {
        // =MIRR(B97:B100,B6,B7) -> 0.15555971946779223
        let result = codcel_m_irr(vec![-10000.0, 3500.0, 4500.0, 6000.0], 0.1, 0.12).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.15555971946779223).abs() < 1e-9,
            "Expected 0.15555971946779223, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_array_mixed() {
        // =MIRR({-5000,1200,1200,1200,1200,1200},0.08,0.08) -> 0.07082760437040503
        let result = codcel_m_irr(
            vec![-5000.0, 1200.0, 1200.0, 1200.0, 1200.0, 1200.0],
            0.08,
            0.08,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07082760437040503).abs() < 1e-9,
            "Expected 0.07082760437040503, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_array_high_ret() {
        // =MIRR({-1000,5000},0.05,0.05) -> 4.0
        let result = codcel_m_irr(vec![-1000.0, 5000.0], 0.05, 0.05).unwrap();
        println!("{result:?}");
        assert!((result - 4.0).abs() < 1e-9, "Expected 4.0, got {result}");
    }

    #[test]
    fn test_m_irr_mirr_array_zeros() {
        // =MIRR({0,-7500,0,10000},0.06,0.09) -> 0.12222912055942481
        let result = codcel_m_irr(vec![0.0, -7500.0, 0.0, 10000.0], 0.06, 0.09).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.12222912055942481).abs() < 1e-9,
            "Expected 0.12222912055942481, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_array_long() {
        // =MIRR({-50000,15000,15000,15000,15000},0.1,0.1) -> 0.0862585448951958
        let result =
            codcel_m_irr(vec![-50000.0, 15000.0, 15000.0, 15000.0, 15000.0], 0.1, 0.1).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0862585448951958).abs() < 1e-9,
            "Expected 0.0862585448951958, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_range_new_rates() {
        // =MIRR(B16:B24,B93,B94) -> 0.08121715978020316
        let result = codcel_m_irr(
            vec![
                -50000.0, 8000.0, 9000.0, 10000.0, 11000.0, 12000.0, 13000.0, 7000.0, 5000.0,
            ],
            0.04,
            0.06,
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.08121715978020316).abs() < 1e-9,
            "Expected 0.08121715978020316, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_partial_range() {
        // =MIRR(B1:B4,B6,B7) -> 0.1257611310229061
        let result = codcel_m_irr(vec![-10000.0, 3000.0, 4200.0, 5800.0], 0.1, 0.12).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1257611310229061).abs() < 1e-9,
            "Expected 0.1257611310229061, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_three_period() {
        // =MIRR({-10000,3500,4500,6000},0.1,0.12) -> 0.15555971946779223
        let result = codcel_m_irr(vec![-10000.0, 3500.0, 4500.0, 6000.0], 0.1, 0.12).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.15555971946779223).abs() < 1e-9,
            "Expected 0.15555971946779223, got {result}"
        );
    }

    #[test]
    fn test_m_irr_mirr_break_even() {
        // =MIRR({-1000,1000},0.1,0.1) -> -1.1102230246251565e-16
        let result = codcel_m_irr(vec![-1000.0, 1000.0], 0.1, 0.1).unwrap();
        println!("{result:?}");
        assert!(
            (result - -1.1102230246251565e-16).abs() < 1e-9,
            "Expected -1.1102230246251565e-16, got {result}"
        );
    }

    #[test]
    fn test_m_irr_basic() {
        // Initial investment of -1000, followed by returns of 500, 500, and 500
        let result = codcel_m_irr(vec![-1000.0, 500.0, 500.0, 500.0], 0.1, 0.12).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_m_irr_error_cases() {
        // Less than two cash flows
        assert!(codcel_m_irr(vec![-1000.0], 0.1, 0.12).is_err());

        // No negative cash flows
        assert!(codcel_m_irr(vec![100.0, 200.0, 300.0], 0.1, 0.12).is_err());

        // No positive cash flows
        assert!(codcel_m_irr(vec![-100.0, -200.0, -300.0], 0.1, 0.12).is_err());
    }
}
