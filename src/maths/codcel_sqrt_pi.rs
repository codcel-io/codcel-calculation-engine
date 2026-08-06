// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `SQRTPI` that returns the square root of (number × π).
/// - `value`: a non-negative number.
///
/// Returns √(value × π) or an error when value is negative.
pub fn codcel_sqrt_pi(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("SQRTPI", value)?;
    if value < 0.0 {
        return Err(format!("SQRTPI: Input must be non-negative: {value:}").into());
    }
    Ok(crate::portable_math::sqrt(value * std::f64::consts::PI))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_pi_positive_integer() {
        // =SQRTPI(1) in US format
        // =SQRTPI(1) in German format
        let result = codcel_sqrt_pi(1.0).unwrap();
        assert!((result - 1.7724538509055159).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_pi_positive_decimal() {
        // =SQRTPI(2.25) in US format
        // =SQRTPI(2,25) in German format
        let result = codcel_sqrt_pi(2.25).unwrap();
        assert!((result - 2.6586807763582738).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_pi_zero() {
        // =SQRTPI(0) in US format
        // =SQRTPI(0) in German format
        let result = codcel_sqrt_pi(0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sqrt_pi_large_number() {
        // =SQRTPI(100) in US format
        // =SQRTPI(100) in German format
        let result = codcel_sqrt_pi(100.0).unwrap();
        assert!((result - 17.724538509055159).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_pi_small_decimal() {
        // =SQRTPI(0.0001) in US format
        // =SQRTPI(0,0001) in German format
        let result = codcel_sqrt_pi(0.0001).unwrap();
        assert!((result - 0.017_724_538_509_055_16).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_pi_negative() {
        // =SQRTPI(-1) in US format (returns #NUM! error)
        // =SQRTPI(-1) in German format (returns #NUM! error)
        let result = codcel_sqrt_pi(-1.0);
        assert!(result.is_err());
    }
}
