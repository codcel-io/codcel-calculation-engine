// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `ATANH` that returns the inverse hyperbolic tangent of a number.
/// - `value`: any real number between -1 and 1 (exclusive).
///
/// Returns the inverse hyperbolic tangent or an error when `value` is outside (-1, 1).
pub fn codcel_atanh(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if the number is within the valid range for the atanh function
    if value <= -1.0 || value >= 1.0 {
        Err(
            format!("ATANH: Number {value:} supplied is outside the exclusive range -1 to +1")
                .into(),
        )
    } else {
        Ok(crate::portable_math::atanh(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atanh_zero() {
        // =ATANH(0) in US format
        // =ATANH(0) in German format
        let result = codcel_atanh(0.0).unwrap();
        let expected = 0.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atanh_half() {
        // =ATANH(0.5) in US format
        // =ATANH(0,5) in German format
        let result = codcel_atanh(0.5).unwrap();
        // The expected value is approximately 0.5493061443340548
        let expected = 0.5493061443340548;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atanh_negative_half() {
        // =ATANH(-0.5) in US format
        // =ATANH(-0,5) in German format
        let result = codcel_atanh(-0.5).unwrap();
        // The expected value is approximately -0.5493061443340548
        let expected = -0.5493061443340548;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atanh_near_one() {
        // =ATANH(0.99) in US format
        // =ATANH(0,99) in German format
        let result = codcel_atanh(0.99).unwrap();
        // The expected value is approximately 2.6466524123622457
        let expected = 2.6466524123622457;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atanh_near_negative_one() {
        // =ATANH(-0.99) in US format
        // =ATANH(-0,99) in German format
        let result = codcel_atanh(-0.99).unwrap();
        // The expected value is approximately -2.6466524123622457
        let expected = -2.6466524123622457;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_atanh_out_of_range() {
        // =ATANH(1) in US format - should return an error
        // =ATANH(1) in German format - should return an error
        let result = codcel_atanh(1.0);
        assert!(result.is_err());

        // =ATANH(-1) in US format - should return an error
        // =ATANH(-1) in German format - should return an error
        let result = codcel_atanh(-1.0);
        assert!(result.is_err());

        // =ATANH(2) in US format - should return an error
        // =ATANH(2) in German format - should return an error
        let result = codcel_atanh(2.0);
        assert!(result.is_err());

        // =ATANH(-2) in US format - should return an error
        // =ATANH(-2) in German format - should return an error
        let result = codcel_atanh(-2.0);
        assert!(result.is_err());
    }
}
