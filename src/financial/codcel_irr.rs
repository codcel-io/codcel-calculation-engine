// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Calculates the internal rate of return (IRR) for a series of cash flows.
///
/// The function uses Newton-Raphson iteration to find the discount rate that
/// makes the net present value of `cash_flows` equal to zero.
///
/// # Arguments
/// * `cash_flows` - Sequence of cash flows where the first value is typically the initial investment.
/// * `guess` - Optional initial guess for the rate (defaults to `0.1`).
///
/// # Errors
/// Returns an error when the cash flows are empty, all have the same sign, or
/// the iteration cannot converge to a valid rate.
pub fn codcel_irr(
    cash_flows: Vec<f64>,
    guess: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if cash_flows.is_empty() {
        return Err("IRR: Cash flows must not be empty".into());
    }
    if cash_flows.iter().all(|&cf| cf >= 0.0) || cash_flows.iter().all(|&cf| cf <= 0.0) {
        return Err(
            "IRR: Cash flows must contain at least one positive and one negative value".into(),
        );
    }

    let mut rate = guess.unwrap_or(0.1); // Default guess is 10%
    let max_iterations = 100; // Maximum number of iterations
    let tolerance = 1e-6; // Tolerance for result accuracy

    for _ in 0..max_iterations {
        let mut npv = 0.0;
        let mut d_npv = 0.0; // Derivative of NPV

        for (i, &cf) in cash_flows.iter().enumerate() {
            let discount_factor = (1.0 + rate).powi(i as i32);
            if discount_factor == 0.0 {
                return Err("IRR: Division by zero encountered".into());
            }
            npv += cf / discount_factor;
            d_npv -= i as f64 * cf / discount_factor.powi(2);
        }

        if d_npv.abs() < tolerance {
            // Avoid division by near-zero derivative
            return Err("IRR: Derivative too small for Newton-Raphson method".into());
        }

        let new_rate = rate - npv / d_npv;

        if (new_rate - rate).abs() < tolerance {
            return Ok(new_rate);
        }

        rate = new_rate;
    }

    Err("IRR: Failed to converge to a result".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irr_error_cases() {
        // Empty cash flows
        assert!(codcel_irr(vec![], None).is_err());

        // All positive cash flows
        assert!(codcel_irr(vec![100.0, 200.0, 300.0], None).is_err());

        // All negative cash flows
        assert!(codcel_irr(vec![-100.0, -200.0, -300.0], None).is_err());
    }
}
