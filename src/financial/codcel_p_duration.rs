// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Calculates the number of periods required for an investment to reach a specific value.
///
/// # Arguments
/// * `rate` - The interest rate per period.
/// * `present_value` - The present value of the investment.
/// * `future_value` - The future value of the investment.
///
/// # Returns
/// The number of periods required for the investment to reach the future value.
pub fn codcel_p_duration(
    rate: f64,
    present_value: f64,
    future_value: f64,
) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    if rate <= 0.0 {
        return Err("PDURATION: Rate should be greater than 0.".into());
    }
    if present_value <= 0.0 {
        return Err("PDURATION: Present value should be greater than 0.".into());
    }
    if future_value <= 0.0 {
        return Err("PDURATION: Future value should be greater than 0.".into());
    }

    let periods = (future_value / present_value).ln() / (1.0 + rate).ln();
    Ok(periods)
}

/// Convenience wrapper that accepts a vector of inputs for PDURATION.
///
/// # Arguments
/// * `inputs` - Vector containing `[rate, present_value, future_value]`.
///
/// # Errors
/// Returns an error when the vector does not contain exactly three values or
/// the underlying `codcel_p_duration` validation fails.
pub fn codcel_p_duration_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("PDURATION: Must have 3 parameters".into());
    }

    codcel_p_duration(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_duration_basic() {
        let result = codcel_p_duration(0.05, 1000.0, 2000.0).unwrap();
        assert!((result - 14.2067).abs() < 0.0001);
    }

    #[test]
    fn test_p_duration_errors() {
        // Rate should be greater than 0
        assert!(codcel_p_duration(0.0, 1000.0, 2000.0).is_err());
        assert!(codcel_p_duration(-0.05, 1000.0, 2000.0).is_err());

        // Present value should be greater than 0
        assert!(codcel_p_duration(0.05, 0.0, 2000.0).is_err());
        assert!(codcel_p_duration(0.05, -1000.0, 2000.0).is_err());

        // Future value should be greater than 0
        assert!(codcel_p_duration(0.05, 1000.0, 0.0).is_err());
        assert!(codcel_p_duration(0.05, 1000.0, -2000.0).is_err());
    }

    #[test]
    fn test_p_duration_vec() {
        let result = codcel_p_duration_vec(vec![0.05, 1000.0, 2000.0]).unwrap();
        assert!((result - 14.2067).abs() < 0.0001);

        // Must have 3 parameters
        assert!(codcel_p_duration_vec(vec![0.05, 1000.0]).is_err());
    }
}
