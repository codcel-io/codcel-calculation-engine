// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `ATAN` that returns the arctangent of a number in radians.
/// - `value`: the tangent of the angle you want.
///
/// Returns the angle in radians (-π/2 to π/2).
pub fn codcel_atan(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(crate::portable_math::atan(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_atan_zero() {
        // =ATAN(0) in US format
        // =ATAN(0) in German format
        let result = codcel_atan(0.0).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atan_one() {
        // =ATAN(1) in US format
        // =ATAN(1) in German format
        let result = codcel_atan(1.0).unwrap();
        let expected = PI / 4.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atan_negative_one() {
        // =ATAN(-1) in US format
        // =ATAN(-1) in German format
        let result = codcel_atan(-1.0).unwrap();
        let expected = -PI / 4.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atan_large_positive() {
        // =ATAN(100) in US format
        // =ATAN(100) in German format
        let result = codcel_atan(100.0).unwrap();
        // As x approaches infinity, ATAN(x) approaches PI/2
        let expected = 1.5607966601082315;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atan_large_negative() {
        // =ATAN(-100) in US format
        // =ATAN(-100) in German format
        let result = codcel_atan(-100.0).unwrap();
        // As x approaches negative infinity, ATAN(x) approaches -PI/2
        let expected = -1.5607966601082315;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
