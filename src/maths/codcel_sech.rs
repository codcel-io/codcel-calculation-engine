// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `SECH` that returns the hyperbolic secant of a number.
/// - `x`: any real number.
///
/// Returns the hyperbolic secant (1/cosh).
pub fn codcel_sech(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("SECH", x)?;
    Ok(1.0 / crate::portable_math::cosh(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sech_zero() {
        // =SECH(0) in US format
        // =SECH(0) in German format
        let result = codcel_sech(0.0).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_sech_positive() {
        // =SECH(1) in US format
        // =SECH(1) in German format
        let result = codcel_sech(1.0).unwrap();
        let expected = 0.6480542736638853;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sech_negative() {
        // =SECH(-1) in US format
        // =SECH(-1) in German format
        let result = codcel_sech(-1.0).unwrap();
        let expected = 0.6480542736638855;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sech_large_value() {
        // =SECH(5) in US format
        // =SECH(5) in German format
        let result = codcel_sech(5.0).unwrap();
        println!("{result}");
        let expected = 0.013475282221304556;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sech_small_value() {
        // =SECH(0.1) in US format
        // =SECH(0,1) in German format
        let result = codcel_sech(0.1).unwrap();
        println!("{result}");
        let expected = 0.9950207489532265;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sech_symmetry() {
        // =SECH(2) in US format
        // =SECH(2) in German format
        let result_positive = codcel_sech(2.0).unwrap();

        // =SECH(-2) in US format
        // =SECH(-2) in German format
        let result_negative = codcel_sech(-2.0).unwrap();

        // SECH is an even function, so SECH(x) = SECH(-x)
        let epsilon = 1e-14;
        assert!((result_positive - result_negative).abs() < epsilon);
    }
}
