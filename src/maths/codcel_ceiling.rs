// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `CEILING` that rounds a number up to the nearest multiple of significance.
/// - `number`: the value to round.
/// - `significance`: the multiple to round up to.
///
/// Returns the rounded value or an error when significance is zero or signs conflict.
pub fn codcel_ceiling(number: f64, significance: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("CEILING", number)?;
    if significance == 0.0 {
        return Err("CEILING: Significance cannot be zero.".into());
    }

    if number > 0.0 && significance < 0.0 {
        return Err(format!("CEILING: Number {number:} and significance {significance:} should have the same sign or one of them should be zero.").into());
    }

    let result = (number / significance).ceil() * significance;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceiling_positive_number_positive_significance() {
        // =CEILING(2.5, 1) in US format
        // =CEILING(2,5; 1) in German format
        let result = codcel_ceiling(2.5, 1.0).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_ceiling_negative_number_positive_significance() {
        // =CEILING(-5.2, 1) in US format
        // =CEILING(-5,2; 1) in German format
        let result = codcel_ceiling(-5.2, 1.0).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_ceiling_negative_number_negative_significance() {
        // =CEILING(-2.5, -1) in US format
        // =CEILING(-2,5; -1) in German format
        let result = codcel_ceiling(-2.5, -1.0).unwrap();
        assert_eq!(result, -3.0);
    }

    #[test]
    fn test_ceiling_exactly_divisible() {
        // =CEILING(10, 5) in US format
        // =CEILING(10; 5) in German format
        let result = codcel_ceiling(10.0, 5.0).unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_ceiling_not_exactly_divisible() {
        // =CEILING(12.34, 0.1) in US format
        // =CEILING(12,34; 0,1) in German format
        let result = codcel_ceiling(12.34, 0.1).unwrap();
        // The expected value is 12.4
        let expected = 12.4;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_ceiling_zero_number() {
        // =CEILING(0, 1.5) in US format
        // =CEILING(0; 1,5) in German format
        let result = codcel_ceiling(0.0, 1.5).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_ceiling_positive_number_negative_significance() {
        // =CEILING(2.5, -1) in US format - should return an error
        // =CEILING(2,5; -1) in German format - should return an error
        let result = codcel_ceiling(2.5, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_ceiling_zero_significance() {
        // =CEILING(2.5, 0) in US format - should return an error
        // =CEILING(2,5; 0) in German format - should return an error
        let result = codcel_ceiling(2.5, 0.0);
        assert!(result.is_err());
    }
}
