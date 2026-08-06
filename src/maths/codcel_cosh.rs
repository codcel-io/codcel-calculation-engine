// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `COSH` that returns the hyperbolic cosine of a number.
/// - `value`: any real number.
///
/// Returns the hyperbolic cosine or an error when input is NaN or infinite.
pub fn codcel_cosh(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("COSH", value)?;

    Ok(crate::portable_math::cosh(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosh_zero() {
        // =COSH(0) in US format
        // =COSH(0) in German format
        let result = codcel_cosh(0.0).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_cosh_positive() {
        // =COSH(1) in US format
        // =COSH(1) in German format
        let result = codcel_cosh(1.0).unwrap();
        let expected = 1.5430806348152437;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cosh_negative() {
        // =COSH(-1) in US format
        // =COSH(-1) in German format
        let result = codcel_cosh(-1.0).unwrap();
        let expected = 1.5430806348152437;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cosh_large_value() {
        // =COSH(5) in US format
        // =COSH(5) in German format
        let result = codcel_cosh(5.0).unwrap();
        let expected = 74.20994852478785;
        let epsilon = 1e-13;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cosh_symmetry() {
        // =COSH(2) in US format
        // =COSH(2) in German format
        let result_positive = codcel_cosh(2.0).unwrap();

        // =COSH(-2) in US format
        // =COSH(-2) in German format
        let result_negative = codcel_cosh(-2.0).unwrap();

        // COSH is an even function, so COSH(x) = COSH(-x)
        let epsilon = 1e-14;
        assert!((result_positive - result_negative).abs() < epsilon);
    }

    #[test]
    fn test_cosh_decimal() {
        // =COSH(0.5) in US format
        // =COSH(0,5) in German format
        let result = codcel_cosh(0.5).unwrap();
        let expected = 1.1276259652063807;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
