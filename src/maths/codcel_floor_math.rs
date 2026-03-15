// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `FLOOR.MATH` that rounds a number down to the nearest integer or multiple.
/// - `number`: the value to round.
/// - `significance`: optional multiple to round to (defaults to 1).
/// - `mode`: optional mode (non-zero rounds toward zero for negatives).
///
/// Returns the rounded value or an error when significance is zero.
pub fn codcel_floor_math(
    number: f64,
    significance: Option<f64>,
    mode: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let significance = significance.unwrap_or(1.0);
    if significance == 0.0 {
        return Err("FLOOR.MATH: Significance cannot be zero".into());
    }

    // Default mode is 0
    let mode = mode.unwrap_or(0);

    let result = if number >= 0.0 {
        // For non-negative numbers, always round down
        (number / significance).floor() * significance
    } else if mode == 0 {
        // Mode 0: Round towards negative infinity
        (number / significance).floor() * significance
    } else {
        // Mode 1: Round towards zero
        (number / significance).ceil() * significance
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_math_positive_default_significance_default_mode() {
        // =FLOOR.MATH(2.5) in US format
        // =FLOOR.MATH(2,5) in German format
        let result = codcel_floor_math(2.5, None, None).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_floor_math_negative_default_significance_default_mode() {
        // =FLOOR.MATH(-2.5) in US format
        // =FLOOR.MATH(-2,5) in German format
        let result = codcel_floor_math(-2.5, None, None).unwrap();
        assert_eq!(result, -3.0);
    }

    #[test]
    fn test_floor_math_positive_custom_significance_default_mode() {
        // =FLOOR.MATH(2.5, 0.5) in US format
        // =FLOOR.MATH(2,5; 0,5) in German format
        let result = codcel_floor_math(2.5, Some(0.5), None).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_floor_math_negative_custom_significance_default_mode() {
        // =FLOOR.MATH(-2.5, 0.5) in US format
        // =FLOOR.MATH(-2,5; 0,5) in German format
        let result = codcel_floor_math(-2.5, Some(0.5), None).unwrap();
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_floor_math_negative_default_significance_mode_one() {
        // =FLOOR.MATH(-2.5, 1, 1) in US format
        // =FLOOR.MATH(-2,5; 1; 1) in German format
        let result = codcel_floor_math(-2.5, None, Some(1)).unwrap();
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_floor_math_negative_custom_significance_mode_one() {
        // =FLOOR.MATH(-2.5, 0.5, 1) in US format
        // =FLOOR.MATH(-2,5; 0,5; 1) in German format
        let result = codcel_floor_math(-2.5, Some(0.5), Some(1)).unwrap();
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_floor_math_positive_negative_significance() {
        // =FLOOR.MATH(2.5, -0.5) in US format
        // =FLOOR.MATH(2,5; -0,5) in German format
        let result = codcel_floor_math(2.5, Some(-0.5), None).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_floor_math_zero() {
        // =FLOOR.MATH(0, 1.5) in US format
        // =FLOOR.MATH(0; 1,5) in German format
        let result = codcel_floor_math(0.0, Some(1.5), None).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_floor_math_not_exactly_divisible() {
        // =FLOOR.MATH(12.34, 0.1) in US format
        // =FLOOR.MATH(12,34; 0,1) in German format
        let result = codcel_floor_math(12.34, Some(0.1), None).unwrap();
        let expected = 12.3;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_floor_math_zero_significance() {
        // =FLOOR.MATH(2.5, 0) in US format - should return an error
        // =FLOOR.MATH(2,5; 0) in German format - should return an error
        let result = codcel_floor_math(2.5, Some(0.0), None);
        assert!(result.is_err());
    }
}
