// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `ABS` that returns the absolute value of a number.
/// - `value`: the number whose absolute value you want.
///
/// Returns the magnitude of `value` without its sign.
pub fn codcel_abs(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(value.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs_positive() {
        // =ABS(2.5) in US format
        // =ABS(2,5) in German format
        let result = codcel_abs(2.5).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_abs_negative() {
        // =ABS(-3.7) in US format
        // =ABS(-3,7) in German format
        let result = codcel_abs(-3.7).unwrap();
        assert_eq!(result, 3.7);
    }

    #[test]
    fn test_abs_zero() {
        // =ABS(0) in US format
        // =ABS(0) in German format
        let result = codcel_abs(0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_abs_large_number() {
        // =ABS(-1000000.5) in US format
        // =ABS(-1000000,5) in German format
        let result = codcel_abs(-1000000.5).unwrap();
        assert_eq!(result, 1000000.5);
    }

    #[test]
    fn test_abs_small_decimal() {
        // =ABS(-0.00001) in US format
        // =ABS(-0,00001) in German format
        let result = codcel_abs(-0.00001).unwrap();
        assert_eq!(result, 0.00001);
    }
}
