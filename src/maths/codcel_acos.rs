// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `ACOS` that returns the arc cosine of a number in radians.
/// - `value`: the cosine of the angle you want, must be between -1 and 1.
///
/// Returns the angle in radians (0 to π) or an error when `value` is outside [-1, 1].
pub fn codcel_acos(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if !(-1.0..=1.0).contains(&value) {
        Err(format!("ACOS: Number {value:} supplied is outside the range -1 to +1").into())
    } else {
        Ok(crate::portable_math::acos(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_acos_zero() {
        // =ACOS(0) in US format
        // =ACOS(0) in German format
        let result = codcel_acos(0.0).unwrap();
        let expected = PI / 2.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acos_one() {
        // =ACOS(1) in US format
        // =ACOS(1) in German format
        let result = codcel_acos(1.0).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acos_negative_one() {
        // =ACOS(-1) in US format
        // =ACOS(-1) in German format
        let result = codcel_acos(-1.0).unwrap();
        let expected = PI;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acos_half() {
        // =ACOS(0.5) in US format
        // =ACOS(0,5) in German format
        let result = codcel_acos(0.5).unwrap();
        let expected = PI / 3.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acos_out_of_range() {
        // =ACOS(2) in US format - should return an error
        // =ACOS(2) in German format - should return an error
        let result = codcel_acos(2.0);
        assert!(result.is_err());

        // =ACOS(-2) in US format - should return an error
        // =ACOS(-2) in German format - should return an error
        let result = codcel_acos(-2.0);
        assert!(result.is_err());
    }
}
