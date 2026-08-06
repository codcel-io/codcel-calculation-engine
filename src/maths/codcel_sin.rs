// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `SIN` that returns the sine of an angle.
/// - `x`: the angle in radians.
///
/// Returns the sine of the angle.
pub fn codcel_sin(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("SIN", x)?;
    Ok(crate::portable_math::sin(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_sin_zero() {
        // =SIN(0) in US format
        // =SIN(0) in German format
        let result = codcel_sin(0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sin_pi() {
        // =SIN(PI()) in US format
        // =SIN(PI()) in German format
        let result = codcel_sin(PI).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sin_half_pi() {
        // =SIN(PI()/2) in US format
        // =SIN(PI()/2) in German format
        let result = codcel_sin(PI / 2.0).unwrap();
        let expected = 1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sin_quarter_pi() {
        // =SIN(PI()/4) in US format
        // =SIN(PI()/4) in German format
        let result = codcel_sin(PI / 4.0).unwrap();
        let expected = 0.7071067811865475;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sin_negative_pi() {
        // =SIN(-PI()) in US format
        // =SIN(-PI()) in German format
        let result = codcel_sin(-PI).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sin_negative_half_pi() {
        // =SIN(-PI()/2) in US format
        // =SIN(-PI()/2) in German format
        let result = codcel_sin(-PI / 2.0).unwrap();
        let expected = -1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sin_two_pi() {
        // =SIN(2*PI()) in US format
        // =SIN(2*PI()) in German format
        let result = codcel_sin(2.0 * PI).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_sin_arbitrary_value() {
        // =SIN(1) in US format
        // =SIN(1) in German format
        let result = codcel_sin(1.0).unwrap();
        let expected = 0.8414709848078965;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
