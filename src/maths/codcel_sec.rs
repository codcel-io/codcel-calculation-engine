// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `SEC` that returns the secant of an angle.
/// - `x`: the angle in radians.
///
/// Returns the secant (1/cos) or an error when cos(x) is zero.
pub fn codcel_sec(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("SEC", x)?;
    let cos_x = crate::portable_math::cos(x);
    if cos_x.abs() < f64::EPSILON {
        Err("SEC: cannot divide by zero".into())
    } else {
        Ok(1.0 / cos_x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_sec_zero() {
        // =SEC(0) in US format
        // =SEC(0) in German format
        let result = codcel_sec(0.0).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_sec_pi() {
        // =SEC(PI()) in US format
        // =SEC(PI()) in German format
        let result = codcel_sec(PI).unwrap();
        let expected = -1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sec_half_pi() {
        // =SEC(PI()/2) in US format - should return an error
        // =SEC(PI()/2) in German format - should return an error
        let result = codcel_sec(PI / 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_sec_quarter_pi() {
        // =SEC(PI()/4) in US format
        // =SEC(PI()/4) in German format
        let result = codcel_sec(PI / 4.0).unwrap();
        let expected = 1.4142135623730951;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sec_negative_pi() {
        // =SEC(-PI()) in US format
        // =SEC(-PI()) in German format
        let result = codcel_sec(-PI).unwrap();
        let expected = -1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sec_two_pi() {
        // =SEC(2*PI()) in US format
        // =SEC(2*PI()) in German format
        let result = codcel_sec(2.0 * PI).unwrap();
        let expected = 1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sec_arbitrary_value() {
        // =SEC(1) in US format
        // =SEC(1) in German format
        let result = codcel_sec(1.0).unwrap();
        let expected = 1.8508157176809255;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sec_three_half_pi() {
        // =SEC(3*PI()/2) in US format - should return an error
        // =SEC(3*PI()/2) in German format - should return an error
        let result = codcel_sec(3.0 * PI / 2.0);
        assert!(result.is_err());
    }
}
