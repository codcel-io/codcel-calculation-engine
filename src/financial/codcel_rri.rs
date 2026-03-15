// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Calculate the equivalent interest rate for the growth of an investment.
///
/// # Arguments
/// * `nper` - The number of periods for the investment.
/// * `pv` - The present value of the investment.
/// * `fv` - The future value of the investment.
///
/// # Returns
/// The interest rate per period.
pub fn codcel_rri(nper: f64, pv: f64, fv: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if nper <= 0.0 {
        return Err("RRI: The number of periods (nper) must be greater than zero.".into());
    }
    if pv <= 0.0 || fv <= 0.0 {
        return Err(
            "RRI: Both present value (pv) and future value (fv) must be greater than zero.".into(),
        );
    }

    // Calculate the rate using the formula:
    // RATE = (FV / PV)^(1 / NPER) - 1
    let rate = (fv / pv).powf(1.0 / nper) - 1.0;

    Ok(rate)
}

/// Vector version of codcel_rri for compatibility with array inputs.
///
/// # Arguments
/// * `inputs` - A vector containing [nper, pv, fv].
///
/// # Returns
/// The interest rate per period.
pub fn codcel_rri_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("RRI: Must have 3 parameters".into());
    }

    codcel_rri(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rri_basic() {
        // Test with nper = 10, pv = 1000, fv = 2000
        // This should give a rate around 7.18%
        let result = codcel_rri(10.0, 1000.0, 2000.0).unwrap();
        assert!((result - 0.0718).abs() < 0.0001);
    }

    #[test]
    fn test_rri_vec() {
        // Test vector version
        let result = codcel_rri_vec(vec![10.0, 1000.0, 2000.0]).unwrap();
        assert!((result - 0.0718).abs() < 0.0001);
    }

    #[test]
    fn test_rri_error_cases() {
        // Test with negative nper
        let result = codcel_rri(-10.0, 1000.0, 2000.0);
        assert!(result.is_err());

        // Test with zero pv
        let result = codcel_rri(10.0, 0.0, 2000.0);
        assert!(result.is_err());

        // Test with zero fv
        let result = codcel_rri(10.0, 1000.0, 0.0);
        assert!(result.is_err());

        // Test vector with wrong number of parameters
        let result = codcel_rri_vec(vec![10.0, 1000.0]);
        assert!(result.is_err());
    }
}
