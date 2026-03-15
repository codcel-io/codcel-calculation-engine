// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_bin_op;
use std::error::Error;

/// Excel-compatible division that mirrors the `/` operator.
/// - `lhs`: the dividend (numerator).
/// - `rhs`: the divisor (denominator).
///
/// Returns the quotient or an error for division by zero.
pub fn codcel_divide(lhs: f64, rhs: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_bin_op(lhs, rhs, "DIVIDE")?;
    if rhs == 0.0 {
        Err("Cannot divide by zero".into())
    } else {
        Ok(lhs / rhs)
    }
}

/// Division guard for inlined pure-math expressions.
/// Returns `lhs / rhs` or an error when `rhs` is zero.
/// Unlike `codcel_divide`, this skips the NaN/Inf input checks because
/// the caller (generated inline math) checks the final result instead.
#[inline]
pub fn codcel_safe_div(lhs: f64, rhs: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if rhs == 0.0 {
        Err("Cannot divide by zero".into())
    } else {
        Ok(lhs / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_positive_numbers() {
        // =10/2 in US format
        // =10/2 in German format
        let result = codcel_divide(10.0, 2.0).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_divide_negative_by_positive() {
        // =-10/2 in US format
        // =-10/2 in German format
        let result = codcel_divide(-10.0, 2.0).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_divide_positive_by_negative() {
        // =10/(-2) in US format
        // =10/(-2) in German format
        let result = codcel_divide(10.0, -2.0).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_divide_negative_numbers() {
        // =(-10)/(-2) in US format
        // =(-10)/(-2) in German format
        let result = codcel_divide(-10.0, -2.0).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_divide_by_one() {
        // =10/1 in US format
        // =10/1 in German format
        let result = codcel_divide(10.0, 1.0).unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_divide_zero_by_number() {
        // =0/5 in US format
        // =0/5 in German format
        let result = codcel_divide(0.0, 5.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_divide_decimal_numbers() {
        // =3.5/1.5 in US format
        // =3,5/1,5 in German format
        let result = codcel_divide(3.5, 1.5).unwrap();
        let expected = 2.3333333333333335;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_divide_by_zero() {
        // =10/0 in US format - should return an error
        // =10/0 in German format - should return an error
        let result = codcel_divide(10.0, 0.0);
        assert!(result.is_err());
    }
}
