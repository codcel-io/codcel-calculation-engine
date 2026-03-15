// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `SQRT` that returns the square root of a number.
/// - `value`: a non-negative number.
///
/// Returns the positive square root or an error when value is negative.
pub fn codcel_sqrt(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    check_value_f64("SQRT", value)?;
    if value < 0.0 {
        return Err(format!("SQRT: Input must be non-negative: {value:}").into());
    }
    Ok(value.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_positive() {
        // =SQRT(4) in US format
        // =SQRT(4) in German format
        let result = codcel_sqrt(4.0).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_sqrt_decimal() {
        // =SQRT(2.25) in US format
        // =SQRT(2,25) in German format
        let result = codcel_sqrt(2.25).unwrap();
        assert_eq!(result, 1.5);
    }

    #[test]
    fn test_sqrt_zero() {
        // =SQRT(0) in US format
        // =SQRT(0) in German format
        let result = codcel_sqrt(0.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sqrt_large_number() {
        // =SQRT(1000000) in US format
        // =SQRT(1000000) in German format
        let result = codcel_sqrt(1000000.0).unwrap();
        assert_eq!(result, 1000.0);
    }

    #[test]
    fn test_sqrt_small_decimal() {
        // =SQRT(0.0001) in US format
        // =SQRT(0,0001) in German format
        let result = codcel_sqrt(0.0001).unwrap();
        assert_eq!(result, 0.01);
    }

    #[test]
    fn test_sqrt_irrational() {
        // =SQRT(2) in US format
        // =SQRT(2) in German format
        let result = codcel_sqrt(2.0).unwrap();
        assert!((result - 1.4142135623730951).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt_negative() {
        // =SQRT(-4) in US format (returns #NUM! error)
        // =SQRT(-4) in German format (returns #NUM! error)
        let result = codcel_sqrt(-4.0);
        assert!(result.is_err());
    }
}
