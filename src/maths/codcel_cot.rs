// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;
use std::f64::consts::PI;

/// Excel-compatible `COT` that returns the cotangent of an angle.
/// - `angle_in_radians`: the angle in radians for which you want the cotangent.
///
/// Returns the cotangent or an error when the angle is a multiple of π.
pub fn codcel_cot(angle_in_radians: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("COT", angle_in_radians)?;

    // Check if the angle is a multiple of π (undefined cotangent)
    if angle_in_radians % PI == 0.0 {
        return Err("COT: Cotangent is undefined for multiples of π".into());
    }

    // Calculate cotangent as 1 / tan(angle)
    let tangent = crate::portable_math::tan(angle_in_radians);
    if tangent == 0.0 {
        return Err("COT: Cotangent is undefined due to division by zero".into());
    }

    Ok(1.0 / tangent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_cot_quarter_pi() {
        // =COT(PI()/4) in US format
        // =COT(PI()/4) in German format
        let result = codcel_cot(PI / 4.0).unwrap();
        let expected = 1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cot_third_pi() {
        // =COT(PI()/3) in US format
        // =COT(PI()/3) in German format
        let result = codcel_cot(PI / 3.0).unwrap();
        let expected = 1.0 / 1.7320508075688772;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cot_sixth_pi() {
        // =COT(PI()/6) in US format
        // =COT(PI()/6) in German format
        let result = codcel_cot(PI / 6.0).unwrap();
        let expected = 1.7320508075688772;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cot_negative_value() {
        // =COT(-PI()/4) in US format
        // =COT(-PI()/4) in German format
        let result = codcel_cot(-PI / 4.0).unwrap();
        let expected = -1.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cot_arbitrary_value() {
        // =COT(1) in US format
        // =COT(1) in German format
        let result = codcel_cot(1.0).unwrap();
        let expected = 0.6420926159343306;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_cot_multiple_of_pi() {
        // =COT(PI()) in US format - should return an error
        // =COT(PI()) in German format - should return an error
        let result = codcel_cot(PI);
        assert!(result.is_err());

        // =COT(2*PI()) in US format - should return an error
        // =COT(2*PI()) in German format - should return an error
        let result = codcel_cot(2.0 * PI);
        assert!(result.is_err());
    }

    #[test]
    fn test_cot_half_pi() {
        // =COT(PI()/2) in US format - should return a very large number (approaching infinity)
        // =COT(PI()/2) in German format - should return a very large number (approaching infinity)
        let result = codcel_cot(PI / 2.0).unwrap();
        let expected = 6.123233995736766e-17;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
