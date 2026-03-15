// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

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
        .sum();

    Ok(npv)
}

#[cfg(test)]
mod tests {
    use super::*;

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
