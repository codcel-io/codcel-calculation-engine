// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `LOG` that returns the logarithm of a number in a specified base.
/// - `number`: a positive real number.
/// - `base`: optional base of the logarithm (defaults to 10).
///
/// Returns log_base(number) or an error for invalid inputs.
pub fn codcel_log(number: f64, base: Option<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("LOG", number)?;
    if number <= 0.0 {
        return Err("LOG: Number must be greater than 0.".into());
    }

    let base = base.unwrap_or(10.0);

    if base <= 0.0 || base == 1.0 {
        return Err(format!("LOG: Invalid base {base}. Base must be > 0 and ≠ 1.").into());
    }

    Ok(crate::portable_math::ln(number) / crate::portable_math::ln(base))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_default_base() {
        // =LOG(100) in US format
        // =LOG(100) in German format
        let result = codcel_log(100.0, None).unwrap();
        assert_eq!(result, 2.0); // log_10(100) = 2
    }

    #[test]
    fn test_log_base_2() {
        // =LOG(8,2) in US format
        // =LOG(8;2) in German format
        let result = codcel_log(8.0, Some(2.0)).unwrap();
        assert_eq!(result, 3.0); // log_2(8) = 3
    }

    #[test]
    fn test_log_base_e() {
        // =LOG(2.718281828459045,2.718281828459045) in US format
        // =LOG(2,718281828459045;2,718281828459045) in German format
        let result = codcel_log(std::f64::consts::E, Some(std::f64::consts::E)).unwrap();
        assert_eq!(result, 1.0); // log_e(e) = 1
    }

    #[test]
    fn test_log_decimal() {
        // =LOG(0.1) in US format
        // =LOG(0,1) in German format
        let result = codcel_log(0.1, None).unwrap();
        assert!((result + 1.0).abs() < 1e-12); // log_10(0.1) = -1
    }

    #[test]
    fn test_log_large_number() {
        // =LOG(1000000) in US format
        // =LOG(1000000) in German format
        let result = codcel_log(1000000.0, None).unwrap();
        assert!((result - 6.0).abs() < 1e-12);
    }

    #[test]
    fn test_log_negative_number() {
        // =LOG(-10) in US format (returns #NUM! error)
        // =LOG(-10) in German format (returns #NUM! error)
        let result = codcel_log(-10.0, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_zero() {
        // =LOG(0) in US format (returns #NUM! error)
        // =LOG(0) in German format (returns #NUM! error)
        let result = codcel_log(0.0, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_negative_base() {
        // =LOG(10,-2) in US format (returns #NUM! error)
        // =LOG(10;-2) in German format (returns #NUM! error)
        let result = codcel_log(10.0, Some(-2.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_log_base_one() {
        // =LOG(10,1) in US format (returns #NUM! error)
        // =LOG(10;1) in German format (returns #NUM! error)
        let result = codcel_log(10.0, Some(1.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_log_base_zero() {
        // =LOG(10,0) in US format (returns #NUM! error)
        // =LOG(10;0) in German format (returns #NUM! error)
        let result = codcel_log(10.0, Some(0.0));
        assert!(result.is_err());
    }
}
