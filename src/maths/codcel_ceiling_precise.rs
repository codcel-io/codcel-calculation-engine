// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `CEILING.PRECISE` that rounds a number up to the nearest integer or multiple.
/// - `number`: the value to round.
/// - `significance`: optional multiple to round to (defaults to 1, sign is ignored).
///
/// Returns the rounded value or an error when significance is zero.
pub fn codcel_ceiling_precise(
    number: f64,
    significance: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let significance = significance.unwrap_or(1.0).abs();

    if significance == 0.0 {
        return Err("CEILING.PRECISE: Significance cannot be zero.".into());
    }

    let quotient = number / significance;

    let result = if number >= 0.0 {
        quotient.ceil() * significance
    } else {
        quotient.trunc() * significance
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ceiling_precise_positive_default_significance() {
        // =CEILING.PRECISE(2.5) in US format
        // =CEILING.PRECISE(2,5) in German format
        let result = codcel_ceiling_precise(2.5, None).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_ceiling_precise_negative_default_significance() {
        // =CEILING.PRECISE(-2.5) in US format
        // =CEILING.PRECISE(-2,5) in German format
        let result = codcel_ceiling_precise(-2.5, None).unwrap();
        assert_eq!(result, -2.0);
    }

    #[test]
    fn test_ceiling_precise_positive_custom_significance() {
        // =CEILING.PRECISE(2.5, 0.5) in US format
        // =CEILING.PRECISE(2,5; 0,5) in German format
        let result = codcel_ceiling_precise(2.5, Some(0.5)).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_ceiling_precise_negative_custom_significance() {
        // =CEILING.PRECISE(-2.5, 0.5) in US format
        // =CEILING.PRECISE(-2,5; 0,5) in German format
        let result = codcel_ceiling_precise(-2.5, Some(0.5)).unwrap();
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_ceiling_precise_positive_negative_significance() {
        // =CEILING.PRECISE(2.5, -0.5) in US format
        // =CEILING.PRECISE(2,5; -0,5) in German format
        let result = codcel_ceiling_precise(2.5, Some(-0.5)).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_ceiling_precise_negative_negative_significance() {
        // =CEILING.PRECISE(-2.5, -0.5) in US format
        // =CEILING.PRECISE(-2,5; -0,5) in German format
        let result = codcel_ceiling_precise(-2.5, Some(-0.5)).unwrap();
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_ceiling_precise_zero() {
        // =CEILING.PRECISE(0, 1.5) in US format
        // =CEILING.PRECISE(0; 1,5) in German format
        let result = codcel_ceiling_precise(0.0, Some(1.5)).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_ceiling_precise_not_exactly_divisible() {
        // =CEILING.PRECISE(12.34, 0.1) in US format
        // =CEILING.PRECISE(12,34; 0,1) in German format
        let result = codcel_ceiling_precise(12.34, Some(0.1)).unwrap();
        let expected = 12.4;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_ceiling_precise_zero_significance() {
        // =CEILING.PRECISE(2.5, 0) in US format - should return an error
        // =CEILING.PRECISE(2,5; 0) in German format - should return an error
        let result = codcel_ceiling_precise(2.5, Some(0.0));
        assert!(result.is_err());
    }
}
