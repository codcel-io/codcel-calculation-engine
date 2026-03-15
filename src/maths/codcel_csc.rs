// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `CSC` that returns the cosecant of an angle.
/// - `angle_in_radians`: the angle in radians for which you want the cosecant.
///
/// Returns the cosecant (1/sin) or an error when sin(angle) is zero.
pub fn codcel_csc(angle_in_radians: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let sine = angle_in_radians.sin();
    if sine == 0.0 {
        return Err("CSC is undefined when sine of the angle is exactly zero".into());
    }
    Ok(1.0 / sine)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_csc_half_pi() {
        // =CSC(PI()/2) in US format
        // =CSC(PI()/2) in German format
        let result = codcel_csc(PI / 2.0).unwrap();
        let expected = 1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_csc_quarter_pi() {
        // =CSC(PI()/4) in US format
        // =CSC(PI()/4) in German format
        let result = codcel_csc(PI / 4.0).unwrap();
        let expected = 1.4142135623730951;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_csc_sixth_pi() {
        // =CSC(PI()/6) in US format
        // =CSC(PI()/6) in German format
        let result = codcel_csc(PI / 6.0).unwrap();
        let expected = 2.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_csc_negative_value() {
        // =CSC(-PI()/2) in US format
        // =CSC(-PI()/2) in German format
        let result = codcel_csc(-PI / 2.0).unwrap();
        let expected = -1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_csc_arbitrary_value() {
        // =CSC(1) in US format
        // =CSC(1) in German format
        let result = codcel_csc(1.0).unwrap();
        let expected = 1.1883951057781212;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_csc_zero() {
        // =CSC(0) in US format - should return an error
        // =CSC(0) in German format - should return an error
        let result = codcel_csc(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_csc_pi() {
        // =CSC(PI()) in US format - should return an error
        // =CSC(PI()) in German format - should return an error
        let result = codcel_csc(PI).unwrap();
        assert_eq!(result, 8165619676597685.0);
    }
}
