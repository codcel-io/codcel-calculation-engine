// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `ACOT` that returns the arccotangent of a number in radians.
/// - `value`: the cotangent of the angle you want.
///
/// Returns the angle in radians (0 to π) or an error when `value` is NaN or infinite.
pub fn codcel_acot(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("ACOT", value)?;
    let result = crate::portable_math::atan(1.0 / value);
    if value < 0.0 {
        Ok(std::f64::consts::PI + result)
    } else {
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_acot_one() {
        // =ACOT(1) in US format
        // =ACOT(1) in German format
        let result = codcel_acot(1.0).unwrap();
        let expected = PI / 4.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acot_positive() {
        // =ACOT(2) in US format
        // =ACOT(2) in German format
        let result = codcel_acot(2.0).unwrap();
        // The expected value is approximately 0.4636476090008061
        let expected = 0.4636476090008061;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acot_negative() {
        // =ACOT(-2) in US format
        // =ACOT(-2) in German format
        let result = codcel_acot(-2.0).unwrap();
        // Excel ACOT(-2) = PI + atan(1/-2) ≈ 2.677945044588987
        let expected = std::f64::consts::PI + (-0.5_f64).atan();
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acot_large_value() {
        // =ACOT(1000) in US format
        // =ACOT(1000) in German format
        let result = codcel_acot(1000.0).unwrap();
        // For large values, ACOT approaches 0
        let expected = 0.0009999996666666957;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acot_small_value() {
        // =ACOT(0.01) in US format
        // =ACOT(0,01) in German format
        let result = codcel_acot(0.01).unwrap();
        // For small values, ACOT approaches PI/2
        let expected = 1.5607966601082315;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acot_zero() {
        // =ACOT(0) in US format
        // =ACOT(0) in German format
        let result = codcel_acot(0.0).unwrap();
        // ACOT(0) should be PI/2
        let expected = PI / 2.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
