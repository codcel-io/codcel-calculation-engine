// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `DEGREES` that converts radians to degrees.
/// - `radians`: the angle in radians to convert.
///
/// Returns the angle in degrees.
pub fn codcel_degrees(radians: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("DEGRESS", radians)?;

    Ok(radians * 180.0 / std::f64::consts::PI)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_degrees_zero() {
        // =DEGREES(0) in US format
        // =DEGREES(0) in German format
        let result = codcel_degrees(0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_degrees_pi() {
        // =DEGREES(PI()) in US format
        // =DEGREES(PI()) in German format
        let result = codcel_degrees(PI).unwrap();
        assert_eq!(result, 180.0);
    }

    #[test]
    fn test_degrees_half_pi() {
        // =DEGREES(PI()/2) in US format
        // =DEGREES(PI()/2) in German format
        let result = codcel_degrees(PI / 2.0).unwrap();
        assert_eq!(result, 90.0);
    }

    #[test]
    fn test_degrees_quarter_pi() {
        // =DEGREES(PI()/4) in US format
        // =DEGREES(PI()/4) in German format
        let result = codcel_degrees(PI / 4.0).unwrap();
        let expected = 45.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_degrees_two_pi() {
        // =DEGREES(2*PI()) in US format
        // =DEGREES(2*PI()) in German format
        let result = codcel_degrees(2.0 * PI).unwrap();
        assert_eq!(result, 360.0);
    }

    #[test]
    fn test_degrees_negative_pi() {
        // =DEGREES(-PI()) in US format
        // =DEGREES(-PI()) in German format
        let result = codcel_degrees(-PI).unwrap();
        assert_eq!(result, -180.0);
    }

    #[test]
    fn test_degrees_arbitrary_value() {
        // =DEGREES(1) in US format
        // =DEGREES(1) in German format
        let result = codcel_degrees(1.0).unwrap();
        let expected = 180.0 / PI;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
