// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `CEILING.MATH` that rounds a number up to the nearest integer or multiple.
/// - `number`: the value to round.
/// - `significance`: optional multiple to round to (defaults to 1).
/// - `mode`: optional mode (-1 rounds toward negative infinity for negatives).
///
/// Returns the rounded value or an error when significance is zero.
pub fn codcel_ceiling_math(
    number: f64,
    significance: Option<f64>,
    mode: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("CEILING.MATH", number)?;
    let significance = significance.unwrap_or(1.0);

    if significance == 0.0 {
        return Err("CEILING.MATH: Significance cannot be zero.".into());
    }

    Ok(if mode.unwrap_or(0) != 0 && number < 0.0 {
        (number / significance.abs()).floor() * significance.abs()
    } else {
        (number / significance.abs()).ceil() * significance.abs()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceiling_math_positive_default_significance_default_mode() {
        // =CEILING.MATH(2.5) in US format
        // =CEILING.MATH(2,5) in German format
        let result = codcel_ceiling_math(2.5, None, None).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_ceiling_math_negative_default_significance_default_mode() {
        // =CEILING.MATH(-2.5) in US format
        // =CEILING.MATH(-2,5) in German format
        let result = codcel_ceiling_math(-2.5, None, None).unwrap();
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_ceiling_math_positive_custom_significance_default_mode() {
        // =CEILING.MATH(2.5, 0.5) in US format
        // =CEILING.MATH(2,5; 0,5) in German format
        let result = codcel_ceiling_math(2.5, Some(0.5), None).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_ceiling_math_negative_custom_significance_default_mode() {
        // =CEILING.MATH(-2.5, 0.5) in US format
        // =CEILING.MATH(-2,5; 0,5) in German format
        let result = codcel_ceiling_math(-2.5, Some(0.5), None).unwrap();
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_ceiling_math_positive_custom_significance_mode_negative_one() {
        // =CEILING.MATH(2.5, 0.5, -1) in US format
        // =CEILING.MATH(2,5; 0,5; -1) in German format
        let result = codcel_ceiling_math(2.5, Some(0.5), Some(-1)).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_ceiling_math_negative_custom_significance_mode_negative_one() {
        // =CEILING.MATH(-2.5, 0.5, -1) in US format
        // =CEILING.MATH(-2,5; 0,5; -1) in German format
        let result = codcel_ceiling_math(-2.5, Some(0.5), Some(-1)).unwrap();
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_ceiling_math_zero() {
        // =CEILING.MATH(0, 1.5) in US format
        // =CEILING.MATH(0; 1,5) in German format
        let result = codcel_ceiling_math(0.0, Some(1.5), None).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_ceiling_math_zero_significance() {
        // =CEILING.MATH(2.5, 0) in US format - should return an error
        // =CEILING.MATH(2,5; 0) in German format - should return an error
        let result = codcel_ceiling_math(2.5, Some(0.0), None);
        assert!(result.is_err());
    }
}
