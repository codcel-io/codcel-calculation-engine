// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `FLOOR.PRECISE` that rounds a number down to the nearest integer or multiple.
/// - `value`: the value to round.
/// - `significance`: optional multiple to round to (defaults to 1, sign is ignored).
///
/// Returns the rounded value (0 if either argument is 0).
pub fn codcel_floor_precise(
    value: f64,
    significance: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check the value first
    check_value_f64("FLOOR.PRECISE", value)?;

    // Use 1.0 as default significance if not provided
    let significance = significance.unwrap_or(1.0);

    // Excel FLOOR.PRECISE returns 0 if either value or significance is zero
    if value == 0.0 || significance == 0.0 {
        return Ok(0.0);
    }

    // Take absolute value of significance as per Excel's behavior
    let abs_significance = significance.abs();

    // Calculate the floor based on Excel's FLOOR.PRECISE logic
    let result = (value / abs_significance).floor() * abs_significance;

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_floor_precise_positive_default_significance() {
        // =FLOOR.PRECISE(2.5) in US format
        // =FLOOR.PRECISE(2,5) in German format
        let result = codcel_floor_precise(2.5, None).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_floor_precise_negative_default_significance() {
        // =FLOOR.PRECISE(-2.5) in US format
        // =FLOOR.PRECISE(-2,5) in German format
        let result = codcel_floor_precise(-2.5, None).unwrap();
        assert_eq!(result, -3.0);
    }

    #[test]
    fn test_floor_precise_positive_custom_significance() {
        // =FLOOR.PRECISE(2.5, 0.5) in US format
        // =FLOOR.PRECISE(2,5; 0,5) in German format
        let result = codcel_floor_precise(2.5, Some(0.5)).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_floor_precise_negative_custom_significance() {
        // =FLOOR.PRECISE(-2.5, 0.5) in US format
        // =FLOOR.PRECISE(-2,5; 0,5) in German format
        let result = codcel_floor_precise(-2.5, Some(0.5)).unwrap();
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_floor_precise_positive_negative_significance() {
        // =FLOOR.PRECISE(2.5, -0.5) in US format
        // =FLOOR.PRECISE(2,5; -0,5) in German format
        let result = codcel_floor_precise(2.5, Some(-0.5)).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_floor_precise_negative_negative_significance() {
        // =FLOOR.PRECISE(-2.5, -0.5) in US format
        // =FLOOR.PRECISE(-2,5; -0,5) in German format
        let result = codcel_floor_precise(-2.5, Some(-0.5)).unwrap();
        assert_eq!(result, -2.5);
    }

    #[test]
    fn test_floor_precise_zero_value() {
        // =FLOOR.PRECISE(0, 1.5) in US format
        // =FLOOR.PRECISE(0; 1,5) in German format
        let result = codcel_floor_precise(0.0, Some(1.5)).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_floor_precise_zero_significance() {
        // =FLOOR.PRECISE(2.5, 0) in US format
        // =FLOOR.PRECISE(2,5; 0) in German format
        let result = codcel_floor_precise(2.5, Some(0.0)).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_floor_precise_not_exactly_divisible() {
        // =FLOOR.PRECISE(12.34, 0.1) in US format
        // =FLOOR.PRECISE(12,34; 0,1) in German format
        let result = codcel_floor_precise(12.34, Some(0.1)).unwrap();
        let expected = 12.3;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }
}
