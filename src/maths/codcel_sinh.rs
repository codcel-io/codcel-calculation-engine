// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `SINH` that returns the hyperbolic sine of a number.
/// - `x`: any real number.
///
/// Returns the hyperbolic sine of x.
pub fn codcel_sinh(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("SINH", x)?;
    Ok(x.sinh())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sinh_zero() {
        // =SINH(0) in US format
        // =SINH(0) in German format
        let result = codcel_sinh(0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sinh_positive() {
        // =SINH(1) in US format
        // =SINH(1) in German format
        let result = codcel_sinh(1.0).unwrap();
        let expected = 1.1752011936438014;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sinh_negative() {
        // =SINH(-1) in US format
        // =SINH(-1) in German format
        let result = codcel_sinh(-1.0).unwrap();
        let expected = -1.1752011936438014;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sinh_large_value() {
        // =SINH(5) in US format
        // =SINH(5) in German format
        let result = codcel_sinh(5.0).unwrap();
        let expected = 74.20321057778875;
        let epsilon = 1e-13;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sinh_small_value() {
        // =SINH(0.1) in US format
        // =SINH(0,1) in German format
        let result = codcel_sinh(0.1).unwrap();
        let expected = 0.10016675001984403;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sinh_odd_function() {
        // =SINH(2) in US format
        // =SINH(2) in German format
        let result_positive = codcel_sinh(2.0).unwrap();

        // =SINH(-2) in US format
        // =SINH(-2) in German format
        let result_negative = codcel_sinh(-2.0).unwrap();

        // SINH is an odd function, so SINH(-x) = -SINH(x)
        let epsilon = 1e-14;
        assert!((result_positive + result_negative).abs() < epsilon);
    }
}
