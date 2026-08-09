// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_bin_op;
use std::error::Error;

/// Excel-compatible addition that mirrors the `+` operator.
/// - `lhs`: the first number (left-hand side).
/// - `rhs`: the second number (right-hand side).
///
/// Returns the sum of the two numbers or an error when inputs are NaN or infinite.
pub fn codcel_add(lhs: f64, rhs: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_bin_op(lhs, rhs, "ADD")?;
    Ok(lhs + rhs)
}

#[cfg(test)]
mod tests {
    // Literals such as 3.14159 and 1.41421 are Excel-visible values under test,
    // not stand-ins for std::f64::consts.
    #![allow(clippy::approx_constant)]
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        // =5+3 in US format
        // =5+3 in German format
        let result = codcel_add(5.0, 3.0).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_add_positive_and_negative() {
        // =5+(-3) in US format
        // =5+(-3) in German format
        let result = codcel_add(5.0, -3.0).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_add_negative_numbers() {
        // =(-7)+(-3) in US format
        // =(-7)+(-3) in German format
        let result = codcel_add(-7.0, -3.0).unwrap();
        assert_eq!(result, -10.0);
    }

    #[test]
    fn test_add_zero() {
        // =42+0 in US format
        // =42+0 in German format
        let result = codcel_add(42.0, 0.0).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_add_large_numbers() {
        // =1000000+2000000 in US format
        // =1000000+2000000 in German format
        let result = codcel_add(1000000.0, 2000000.0).unwrap();
        assert_eq!(result, 3000000.0);
    }

    #[test]
    fn test_add_decimals() {
        // =3.14+2.86 in US format
        // =3,14+2,86 in German format
        let result = codcel_add(3.14, 2.86).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_add_small_decimals() {
        // =0.0001+0.0002 in US format
        // =0,0001+0,0002 in German format
        let result = codcel_add(0.0001, 0.0002).unwrap();
        // Use approximate equality for floating-point numbers
        let epsilon = 1e-14;
        assert!((result - 0.0003).abs() < epsilon);
    }
}
