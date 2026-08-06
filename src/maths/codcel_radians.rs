// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `RADIANS` that converts degrees to radians.
/// - `degrees`: the angle in degrees to convert.
///
/// Returns the angle in radians.
pub fn codcel_radians(degrees: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("RADIANS", degrees)?;

    Ok(degrees * std::f64::consts::PI / 180.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radians_zero() {
        // =RADIANS(0) in US format
        // =RADIANS(0) in German format
        let result = codcel_radians(0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_radians_90_degrees() {
        // =RADIANS(90) in US format
        // =RADIANS(90) in German format
        let result = codcel_radians(90.0).unwrap();
        assert!((result - std::f64::consts::FRAC_PI_2).abs() < 1e-10); // 90 degrees = π/2 radians
    }

    #[test]
    fn test_radians_180_degrees() {
        // =RADIANS(180) in US format
        // =RADIANS(180) in German format
        let result = codcel_radians(180.0).unwrap();
        assert!((result - std::f64::consts::PI).abs() < 1e-10); // 180 degrees = π radians
    }

    #[test]
    fn test_radians_360_degrees() {
        // =RADIANS(360) in US format
        // =RADIANS(360) in German format
        let result = codcel_radians(360.0).unwrap();
        assert!((result - 2.0 * std::f64::consts::PI).abs() < 1e-10); // 360 degrees = 2π radians
    }

    #[test]
    fn test_radians_negative() {
        // =RADIANS(-90) in US format
        // =RADIANS(-90) in German format
        let result = codcel_radians(-90.0).unwrap();
        assert!((result - (-std::f64::consts::FRAC_PI_2)).abs() < 1e-10); // -90 degrees = -π/2 radians
    }

    #[test]
    fn test_radians_decimal() {
        // =RADIANS(45.5) in US format
        // =RADIANS(45,5) in German format
        let result = codcel_radians(45.5).unwrap();
        println!("{result}");
        assert!((result - 0.7941248096574199).abs() < 1e-10); // 45.5 degrees ≈ 0.794125 radians
    }

    #[test]
    fn test_radians_large_number() {
        // =RADIANS(1000) in US format
        // =RADIANS(1000) in German format
        let result = codcel_radians(1000.0).unwrap();
        assert!((result - 17.453292519943295).abs() < 1e-10); // 1000 degrees ≈ 17.45329 radians
    }

    #[test]
    fn test_radians_small_decimal() {
        // =RADIANS(0.1) in US format
        // =RADIANS(0,1) in German format
        let result = codcel_radians(0.1).unwrap();
        assert!((result - 0.0017453292519943296).abs() < 1e-10); // 0.1 degrees ≈ 0.00174533 radians
    }
}
