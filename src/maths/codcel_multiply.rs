// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_bin_op;
use std::error::Error;

/// Excel-compatible multiplication that mirrors the `*` operator.
/// - `lhs`: the first number (multiplicand).
/// - `rhs`: the second number (multiplier).
///
/// Returns the product of the two numbers.
pub fn codcel_multiply(lhs: f64, rhs: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_bin_op(lhs, rhs, "MULTIPLY")?;
    Ok(lhs * rhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_positive_numbers() {
        // =5*3 in US format
        // =5*3 in German format
        let result = codcel_multiply(5.0, 3.0).unwrap();
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_multiply_negative_and_positive() {
        // =-5*3 in US format
        // =-5*3 in German format
        let result = codcel_multiply(-5.0, 3.0).unwrap();
        assert_eq!(result, -15.0);
    }

    #[test]
    fn test_multiply_negative_numbers() {
        // =(-5)*(-3) in US format
        // =(-5)*(-3) in German format
        let result = codcel_multiply(-5.0, -3.0).unwrap();
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_multiply_by_zero() {
        // =5*0 in US format
        // =5*0 in German format
        let result = codcel_multiply(5.0, 0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_multiply_zero_by_zero() {
        // =0*0 in US format
        // =0*0 in German format
        let result = codcel_multiply(0.0, 0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_multiply_decimal_numbers() {
        // =2.5*1.5 in US format
        // =2,5*1,5 in German format
        let result = codcel_multiply(2.5, 1.5).unwrap();
        assert_eq!(result, 3.75);
    }

    #[test]
    fn test_multiply_large_numbers() {
        // =1000000*2000000 in US format
        // =1000000*2000000 in German format
        let result = codcel_multiply(1000000.0, 2000000.0).unwrap();
        assert_eq!(result, 2000000000000.0);
    }

    #[test]
    fn test_multiply_small_numbers() {
        // =0.0001*0.0002 in US format
        // =0,0001*0,0002 in German format
        let result = codcel_multiply(0.0001, 0.0002).unwrap();
        let expected = 0.00000002;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
