// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;

/// Excel-compatible `ISO.CEILING` that rounds a number up to the nearest integer or multiple.
/// - `number`: the value to round.
/// - `significance`: optional multiple to round to (defaults to 1, sign is ignored).
///
/// Returns the rounded value or an error when significance is zero.
pub fn codcel_iso_ceiling(
    number: f64,
    significance: Option<f64>,
) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    check_value_f64("ISO.CEILING", number)?;
    let significance = significance.unwrap_or(1.0).abs();

    if significance == 0.0 {
        return Err("ISO.CEILING: Significance cannot be zero.".into());
    }

    if number == 0.0 {
        return Ok(0.0);
    }

    // Always round away from zero
    let quotient = number / significance;
    let rounded = quotient.ceil();

    Ok(rounded * significance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iso_ceiling_positive_default_significance() {
        // =ISO.CEILING(2.5) in US format
        // =ISO.CEILING(2,5) in German format
        let result = codcel_iso_ceiling(2.5, None).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_iso_ceiling_negative_default_significance() {
        // =ISO.CEILING(-2.5) in US format
        // =ISO.CEILING(-2,5) in German format
        let result = codcel_iso_ceiling(-2.5, None).unwrap();
        assert_eq!(result, -2.0); // Rounds towards zero for negative numbers
    }

    #[test]
    fn test_iso_ceiling_positive_custom_significance() {
        // =ISO.CEILING(2.5, 0.5) in US format
        // =ISO.CEILING(2,5; 0,5) in German format
        let result = codcel_iso_ceiling(2.5, Some(0.5)).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_iso_ceiling_negative_custom_significance() {
        // =ISO.CEILING(-2.5, 0.5) in US format
        // =ISO.CEILING(-2,5; 0,5) in German format
        let result = codcel_iso_ceiling(-2.5, Some(0.5)).unwrap();
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_iso_ceiling_positive_negative_significance() {
        // =ISO.CEILING(2.5, -0.5) in US format
        // =ISO.CEILING(2,5; -0,5) in German format
        let result = codcel_iso_ceiling(2.5, Some(-0.5)).unwrap();
        assert_eq!(result, 2.5); // Uses absolute value of significance
    }

    #[test]
    fn test_iso_ceiling_negative_negative_significance() {
        // =ISO.CEILING(-2.5, -0.5) in US format
        // =ISO.CEILING(-2,5; -0,5) in German format
        let result = codcel_iso_ceiling(-2.5, Some(-0.5)).unwrap();
        assert_eq!(result, -2.5); // Uses absolute value of significance
    }

    #[test]
    fn test_iso_ceiling_zero() {
        // =ISO.CEILING(0, 1.5) in US format
        // =ISO.CEILING(0; 1,5) in German format
        let result = codcel_iso_ceiling(0.0, Some(1.5)).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_iso_ceiling_not_exactly_divisible() {
        // =ISO.CEILING(12.34, 0.1) in US format
        // =ISO.CEILING(12,34; 0,1) in German format
        let result = codcel_iso_ceiling(12.34, Some(0.1)).unwrap();
        let expected = 12.4;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_iso_ceiling_zero_significance() {
        // =ISO.CEILING(2.5, 0) in US format - should return an error
        // =ISO.CEILING(2,5; 0) in German format - should return an error
        let result = codcel_iso_ceiling(2.5, Some(0.0));
        assert!(result.is_err());
    }
}
