// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `LN` that returns the natural logarithm of a number.
/// - `value`: a positive real number.
///
/// Returns ln(value) or an error when value ≤ 0.
pub fn codcel_ln(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("LN", value)?;
    if value <= 0.0 {
        // Return an error if the input is invalid (<= 0)
        return Err("Input to LN must be greater than 0".into());
    }
    Ok(crate::portable_math::ln(value)) // Compute the natural logarithm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ln_positive() {
        // =LN(2.5) in US format
        // =LN(2,5) in German format
        let result = codcel_ln(2.5).unwrap();
        assert!((result - 0.9162907318741551).abs() < 1e-10);
    }

    #[test]
    fn test_ln_one() {
        // =LN(1) in US format
        // =LN(1) in German format
        let result = codcel_ln(1.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_ln_small_positive() {
        // =LN(0.1) in US format
        // =LN(0,1) in German format
        let result = codcel_ln(0.1).unwrap();
        assert!((result - (-2.3025850929940455)).abs() < 1e-10);
    }

    #[test]
    fn test_ln_large_number() {
        // =LN(1000) in US format
        // =LN(1000) in German format
        let result = codcel_ln(1000.0).unwrap();
        assert!((result - 6.907755278982137).abs() < 1e-10);
    }

    #[test]
    fn test_ln_e() {
        // =LN(2.718281828459045) in US format
        // =LN(2,718281828459045) in German format
        let result = codcel_ln(std::f64::consts::E).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln_zero() {
        // =LN(0) in US format (returns #NUM! error)
        // =LN(0) in German format (returns #NUM! error)
        let result = codcel_ln(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_ln_negative() {
        // =LN(-1) in US format (returns #NUM! error)
        // =LN(-1) in German format (returns #NUM! error)
        let result = codcel_ln(-1.0);
        assert!(result.is_err());
    }
}
