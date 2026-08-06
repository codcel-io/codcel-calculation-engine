// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `TANH` that returns the hyperbolic tangent of a number.
/// - `x`: any real number.
///
/// Returns the hyperbolic tangent of x.
pub fn codcel_tanh(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("TANH", x)?;
    Ok(crate::portable_math::tanh(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tanh_zero() {
        // =TANH(0) in US format
        // =TANH(0) in German format
        let result = codcel_tanh(0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_tanh_positive() {
        // =TANH(1) in US format
        // =TANH(1) in German format
        let result = codcel_tanh(1.0).unwrap();
        let expected = 0.7615941559557649;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tanh_negative() {
        // =TANH(-1) in US format
        // =TANH(-1) in German format
        let result = codcel_tanh(-1.0).unwrap();
        let expected = -0.7615941559557649;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tanh_large_value() {
        // =TANH(5) in US format
        // =TANH(5) in German format
        let result = codcel_tanh(5.0).unwrap();
        let expected = 0.9999092042625951;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tanh_small_value() {
        // =TANH(0.1) in US format
        // =TANH(0,1) in German format
        let result = codcel_tanh(0.1).unwrap();
        let expected = 0.09966799462495582;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tanh_very_large_value() {
        // =TANH(20) in US format
        // =TANH(20) in German format
        let result = codcel_tanh(20.0).unwrap();
        // For very large values, TANH approaches 1
        let expected = 1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tanh_very_negative_value() {
        // =TANH(-20) in US format
        // =TANH(-20) in German format
        let result = codcel_tanh(-20.0).unwrap();
        // For very negative values, TANH approaches -1
        let expected = -1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_tanh_odd_function() {
        // =TANH(2) in US format
        // =TANH(2) in German format
        let result_positive = codcel_tanh(2.0).unwrap();

        // =TANH(-2) in US format
        // =TANH(-2) in German format
        let result_negative = codcel_tanh(-2.0).unwrap();

        // TANH is an odd function, so TANH(-x) = -TANH(x)
        let epsilon = 1e-14;
        assert!((result_positive + result_negative).abs() < epsilon);
    }
}
