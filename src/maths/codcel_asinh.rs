// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `ASINH` that returns the inverse hyperbolic sine of a number.
/// - `value`: any real number.
///
/// Returns the inverse hyperbolic sine of the argument.
pub fn codcel_asinh(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(crate::portable_math::asinh(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asinh_zero() {
        // =ASINH(0) in US format
        // =ASINH(0) in German format
        let result = codcel_asinh(0.0).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_asinh_one() {
        // =ASINH(1) in US format
        // =ASINH(1) in German format
        let result = codcel_asinh(1.0).unwrap();
        // The expected value is approximately 0.8813735870195429
        let expected = 0.8813735870195429;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_asinh_negative_one() {
        // =ASINH(-1) in US format
        // =ASINH(-1) in German format
        let result = codcel_asinh(-1.0).unwrap();
        // The expected value is approximately -0.8813735870195429
        let expected = -0.8813735870195429;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_asinh_large_positive() {
        // =ASINH(100) in US format
        // =ASINH(100) in German format
        let result = codcel_asinh(100.0).unwrap();
        // The expected value is approximately 5.298342365610589
        let expected = 5.298342365610589;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_asinh_large_negative() {
        // =ASINH(-100) in US format
        // =ASINH(-100) in German format
        let result = codcel_asinh(-100.0).unwrap();
        // The expected value is approximately -5.298342365610589
        let expected = -5.298342365610589;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
