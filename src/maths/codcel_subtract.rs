// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_bin_op;
use std::error::Error;

/// Excel-compatible subtraction that mirrors the `-` operator.
/// - `lhs`: the first number (minuend).
/// - `rhs`: the second number (subtrahend).
///
/// Returns the difference (lhs - rhs).
pub fn codcel_subtract(lhs: f64, rhs: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_bin_op(lhs, rhs, "SUBTRACT")?;
    Ok(lhs - rhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtract_positive_numbers() {
        // =5-3 in US format
        // =5-3 in German format
        let result = codcel_subtract(5.0, 3.0).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_subtract_negative_numbers() {
        // =-5-(-3) in US format
        // =-5-(-3) in German format
        let result = codcel_subtract(-5.0, -3.0).unwrap();
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_subtract_mixed_signs() {
        // =5-(-3) in US format
        // =5-(-3) in German format
        let result = codcel_subtract(5.0, -3.0).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_subtract_from_zero() {
        // =0-5 in US format
        // =0-5 in German format
        let result = codcel_subtract(0.0, 5.0).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_subtract_zero() {
        // =5-0 in US format
        // =5-0 in German format
        let result = codcel_subtract(5.0, 0.0).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_subtract_decimals() {
        // =5.5-2.2 in US format
        // =5,5-2,2 in German format
        let result = codcel_subtract(5.5, 2.2).unwrap();
        assert!((result - 3.3).abs() < 1e-10);
    }

    #[test]
    fn test_subtract_large_numbers() {
        // =1000000-999999 in US format
        // =1000000-999999 in German format
        let result = codcel_subtract(1000000.0, 999999.0).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_subtract_small_decimals() {
        // =0.0001-0.00001 in US format
        // =0,0001-0,00001 in German format
        let result = codcel_subtract(0.0001, 0.00001).unwrap();
        assert!((result - 0.00009).abs() < 1e-10);
    }
}
