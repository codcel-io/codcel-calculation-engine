// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `COTH` that returns the hyperbolic cotangent of a number.
/// - `x`: any real number except 0.
///
/// Returns the hyperbolic cotangent or an error when x is zero.
pub fn codcel_coth(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Hyperbolic cotangent is undefined for x = 0
    if x == 0.0 {
        return Err("COTH is undefined for x = 0".into());
    }

    // Calculate coth(x) using the formula: cosh(x) / sinh(x)
    let sinh = crate::portable_math::sinh(x);
    if sinh == 0.0 {
        return Err("COTH is undefined due to division by zero".into());
    }

    Ok(crate::portable_math::cosh(x) / sinh)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coth_positive() {
        // =COTH(1) in US format
        // =COTH(1) in German format
        let result = codcel_coth(1.0).unwrap();
        let expected = 1.3130352854993315;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_coth_negative() {
        // =COTH(-1) in US format
        // =COTH(-1) in German format
        let result = codcel_coth(-1.0).unwrap();
        let expected = -1.3130352854993315;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_coth_large_value() {
        // =COTH(5) in US format
        // =COTH(5) in German format
        let result = codcel_coth(5.0).unwrap();
        let expected = 1.0000908039820193;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_coth_small_value() {
        // =COTH(0.1) in US format
        // =COTH(0,1) in German format
        let result = codcel_coth(0.1).unwrap();
        let expected = 10.03331113225399;
        let epsilon = 1e-13;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_coth_zero() {
        // =COTH(0) in US format - should return an error
        // =COTH(0) in German format - should return an error
        let result = codcel_coth(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_coth_odd_function() {
        // =COTH(2) in US format
        // =COTH(2) in German format
        let result_positive = codcel_coth(2.0).unwrap();

        // =COTH(-2) in US format
        // =COTH(-2) in German format
        let result_negative = codcel_coth(-2.0).unwrap();

        // COTH is an odd function, so COTH(-x) = -COTH(x)
        let epsilon = 1e-14;
        assert!((result_positive + result_negative).abs() < epsilon);
    }
}
