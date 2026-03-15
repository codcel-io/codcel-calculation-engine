// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `TRUNC` that truncates a number to an integer.
/// - `value`: the number to truncate.
/// - `decimals`: optional number of decimal places to keep (defaults to 0).
///
/// Returns the truncated value toward zero.
pub fn codcel_trunc(
    value: f64,
    decimals: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if let Some(decimals) = decimals {
        if decimals != 0 {
            let multiplier = 10f64.powi(decimals);
            return Ok((value * multiplier).trunc() / multiplier);
        }
    }

    Ok(value.trunc())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trunc_positive_integer() {
        // =TRUNC(5) in US format
        // =TRUNC(5) in German format
        let result = codcel_trunc(5.0, None).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_trunc_positive_decimal() {
        // =TRUNC(5.678) in US format
        // =TRUNC(5,678) in German format
        let result = codcel_trunc(5.678, None).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_trunc_negative_integer() {
        // =TRUNC(-5) in US format
        // =TRUNC(-5) in German format
        let result = codcel_trunc(-5.0, None).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_trunc_negative_decimal() {
        // =TRUNC(-5.678) in US format
        // =TRUNC(-5,678) in German format
        let result = codcel_trunc(-5.678, None).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_trunc_positive_with_decimals() {
        // =TRUNC(5.678, 2) in US format
        // =TRUNC(5,678; 2) in German format
        let result = codcel_trunc(5.678, Some(2)).unwrap();
        assert_eq!(result, 5.67);
    }

    #[test]
    fn test_trunc_negative_with_decimals() {
        // =TRUNC(-5.678, 2) in US format
        // =TRUNC(-5,678; 2) in German format
        let result = codcel_trunc(-5.678, Some(2)).unwrap();
        assert_eq!(result, -5.67);
    }

    #[test]
    fn test_trunc_with_zero_decimals() {
        // =TRUNC(5.678, 0) in US format
        // =TRUNC(5,678; 0) in German format
        let result = codcel_trunc(5.678, Some(0)).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_trunc_with_negative_decimals() {
        // =TRUNC(12345.678, -2) in US format
        // =TRUNC(12345,678; -2) in German format
        let result = codcel_trunc(12345.678, Some(-2)).unwrap();
        assert_eq!(result, 12300.0);
    }

    #[test]
    fn test_trunc_zero() {
        // =TRUNC(0) in US format
        // =TRUNC(0) in German format
        let result = codcel_trunc(0.0, None).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_trunc_large_number() {
        // =TRUNC(1234567.89) in US format
        // =TRUNC(1234567,89) in German format
        let result = codcel_trunc(1234567.89, None).unwrap();
        assert_eq!(result, 1234567.0);
    }
}
