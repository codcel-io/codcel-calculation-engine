// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `ACOTH` that returns the inverse hyperbolic cotangent of a number.
/// - `value`: any real number with absolute value > 1.
///
/// Returns the inverse hyperbolic cotangent or an error when `value` is in (-1, 1).
pub fn codcel_acoth(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if value.abs() <= 1.0 {
        Err(format!("ACOTH: Number {value} supplied must be less than -1 or greater than 1").into())
    } else if value > 1.0 {
        Ok(0.5 * crate::portable_math::ln((value + 1.0) / (value - 1.0)))
    } else {
        // x < -1.0
        Ok(0.5 * crate::portable_math::ln((value + 1.0) / (value - 1.0)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acoth_greater_than_one() {
        // =ACOTH(2) in US format
        // =ACOTH(2) in German format
        let result = codcel_acoth(2.0).unwrap();
        // The expected value is approximately 0.5493061443340548
        let expected = 0.5493061443340548;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acoth_less_than_negative_one() {
        // =ACOTH(-2) in US format
        // =ACOTH(-2) in German format
        let result = codcel_acoth(-2.0).unwrap();
        println!("ACOTH result {result:?}");
        // For negative inputs, the result is the negative of the result for the absolute value
        let expected = -0.5493061443340548;
        // Use a larger epsilon to account for potential differences in implementation
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acoth_large_value() {
        // =ACOTH(10) in US format
        // =ACOTH(10) in German format
        let result = codcel_acoth(10.0).unwrap();
        // The expected value is approximately 0.10033534773107558
        let expected = 0.10033534773107558;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_acoth_invalid_range() {
        // =ACOTH(0.5) in US format - should return an error
        // =ACOTH(0,5) in German format - should return an error
        let result = codcel_acoth(0.5);
        assert!(result.is_err());

        // =ACOTH(0) in US format - should return an error
        // =ACOTH(0) in German format - should return an error
        let result = codcel_acoth(0.0);
        assert!(result.is_err());

        // =ACOTH(1) in US format - should return an error
        // =ACOTH(1) in German format - should return an error
        let result = codcel_acoth(1.0);
        assert!(result.is_err());

        // =ACOTH(-1) in US format - should return an error
        // =ACOTH(-1) in German format - should return an error
        let result = codcel_acoth(-1.0);
        assert!(result.is_err());

        // =ACOTH(-0.5) in US format - should return an error
        // =ACOTH(-0,5) in German format - should return an error
        let result = codcel_acoth(-0.5);
        assert!(result.is_err());
    }
}
