// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `ATAN2` that returns the arctangent from x and y coordinates.
/// - `x`: the x-coordinate of the point.
/// - `y`: the y-coordinate of the point.
///
/// Returns the angle in radians between -π and π.
pub fn codcel_atan2(x: f64, y: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(crate::portable_math::atan2(y, x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_atan2_x1_y0() {
        // =ATAN2(1, 0) in US format
        // =ATAN2(1; 0) in German format
        let result = codcel_atan2(1.0, 0.0).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atan2_x0_y1() {
        // =ATAN2(0, 1) in US format
        // =ATAN2(0; 1) in German format
        let result = codcel_atan2(0.0, 1.0).unwrap();
        let expected = PI / 2.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atan2_x0_yneg1() {
        // =ATAN2(0, -1) in US format
        // =ATAN2(0; -1) in German format
        let result = codcel_atan2(0.0, -1.0).unwrap();
        let expected = -PI / 2.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atan2_xneg1_y0() {
        // =ATAN2(-1, 0) in US format
        // =ATAN2(-1; 0) in German format
        let result = codcel_atan2(-1.0, 0.0).unwrap();
        let expected = PI;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atan2_x1_y1() {
        // =ATAN2(1, 1) in US format
        // =ATAN2(1; 1) in German format
        let result = codcel_atan2(1.0, 1.0).unwrap();
        let expected = PI / 4.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atan2_xneg1_yneg1() {
        // =ATAN2(-1, -1) in US format
        // =ATAN2(-1; -1) in German format
        let result = codcel_atan2(-1.0, -1.0).unwrap();
        let expected = -3.0 * PI / 4.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
