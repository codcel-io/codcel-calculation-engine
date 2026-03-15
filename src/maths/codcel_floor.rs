// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `FLOOR` that rounds a number down to the nearest multiple of significance.
/// - `number`: the value to round.
/// - `significance`: the multiple to round down to.
///
/// Returns the rounded value or an error when significance is zero or signs conflict.
pub fn codcel_floor(number: f64, significance: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if significance == 0.0 {
        return Err("FLOOR: Significance cannot be zero".into());
    }

    // Excel returns an error only when number is positive and significance is negative
    if number > 0.0 && significance < 0.0 {
        return Err("FLOOR: Significance must be positive if number is positive".into());
    }

    Ok((number / significance).floor() * significance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_positive_number_positive_significance() {
        // =FLOOR(2.5, 1) in US format
        // =FLOOR(2,5; 1) in German format
        let result = codcel_floor(2.5, 1.0).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_floor_negative_number_negative_significance() {
        // =FLOOR(-2.5, -1) in US format
        // =FLOOR(-2,5; -1) in German format
        let result = codcel_floor(-2.5, -1.0).unwrap();
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_floor_positive_number_negative_significance() {
        // =FLOOR(2.5, -1) in US format
        // =FLOOR(2,5; -1) in German format
        let result = codcel_floor(2.5, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_floor_negative_number_positive_significance() {
        // =FLOOR(-2.5, 1) in US format
        // =FLOOR(-2,5; 1) in German format
        let result = codcel_floor(-2.5, 1.0).unwrap();
        assert_eq!(result, -3.0);
    }

    #[test]
    fn test_floor_exactly_divisible() {
        // =FLOOR(10, 5) in US format
        // =FLOOR(10; 5) in German format
        let result = codcel_floor(10.0, 5.0).unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_floor_not_exactly_divisible() {
        // =FLOOR(12.34, 0.1) in US format
        // =FLOOR(12,34; 0,1) in German format
        let result = codcel_floor(12.34, 0.1).unwrap();
        let expected = 12.3;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_floor_zero_number() {
        // =FLOOR(0, 1.5) in US format
        // =FLOOR(0; 1,5) in German format
        let result = codcel_floor(0.0, 1.5).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    #[should_panic(expected = "FLOOR: Significance cannot be zero")]
    fn test_floor_zero_significance() {
        // =FLOOR(2.5, 0) in US format - should return an error
        // =FLOOR(2,5; 0) in German format - should return an error
        let _ = codcel_floor(2.5, 0.0).unwrap();
    }
}
