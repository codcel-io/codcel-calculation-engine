// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

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
        .sum::<f64>();

    if npv_negative == 0.0 {
        return Err("MIRR: No negative cash flows found".into());
    }

    // Calculate future value of positive cash flows at reinvestment rate
    let fv_positive = positive_flows
        .iter()
        .enumerate()
        .map(|(i, &flow)| flow * (1.0 + reinvest_rate).powi((cash_flows.len() - 1 - i) as i32))
        .sum::<f64>();

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
