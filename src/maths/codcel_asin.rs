// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `ASIN` that returns the arcsine of a number in radians.
/// - `value`: the sine of the angle you want, must be between -1 and 1.
///
/// Returns the angle in radians (-π/2 to π/2) or an error when `value` is outside [-1, 1].
pub fn codcel_asin(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if the number is within the valid range for the asin function
    if !(-1.0..=1.0).contains(&value) {
        Err(format!("ASIN: Number {value:} supplied is outside the range -1 to +1").into())
    } else {
        Ok(crate::portable_math::asin(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_asin_zero() {
        // =ASIN(0) in US format
        // =ASIN(0) in German format
        let result = codcel_asin(0.0).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_asin_one() {
        // =ASIN(1) in US format
        // =ASIN(1) in German format
        let result = codcel_asin(1.0).unwrap();
        let expected = PI / 2.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_asin_negative_one() {
        // =ASIN(-1) in US format
        // =ASIN(-1) in German format
        let result = codcel_asin(-1.0).unwrap();
        let expected = -PI / 2.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_asin_half() {
        // =ASIN(0.5) in US format
        // =ASIN(0,5) in German format
        let result = codcel_asin(0.5).unwrap();
        let expected = PI / 6.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_asin_out_of_range() {
        // =ASIN(2) in US format - should return an error
        // =ASIN(2) in German format - should return an error
        let result = codcel_asin(2.0);
        assert!(result.is_err());

        // =ASIN(-2) in US format - should return an error
        // =ASIN(-2) in German format - should return an error
        let result = codcel_asin(-2.0);
        assert!(result.is_err());
    }
}
