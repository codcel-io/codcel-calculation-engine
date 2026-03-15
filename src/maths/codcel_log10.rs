// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `LOG10` that returns the base-10 logarithm of a number.
/// - `value`: a positive real number.
///
/// Returns log₁₀(value) or an error when value ≤ 0.
pub fn codcel_log10(value: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if value <= 0.0 {
        return Err("LOG10: Input must be a positive number".into());
    }

    Ok(value.log10())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log10_positive() {
        // =LOG10(100) in US format
        // =LOG10(100) in German format
        let result = codcel_log10(100.0).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_log10_one() {
        // =LOG10(1) in US format
        // =LOG10(1) in German format
        let result = codcel_log10(1.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_log10_decimal() {
        // =LOG10(0.1) in US format
        // =LOG10(0,1) in German format
        let result = codcel_log10(0.1).unwrap();
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_log10_large_number() {
        // =LOG10(1000000) in US format
        // =LOG10(1000000) in German format
        let result = codcel_log10(1000000.0).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_log10_small_positive() {
        // =LOG10(0.000001) in US format
        // =LOG10(0,000001) in German format
        let result = codcel_log10(0.000001).unwrap();
        assert_eq!(result, -6.0);
    }

    #[test]
    fn test_log10_zero() {
        // =LOG10(0) in US format (returns #NUM! error)
        // =LOG10(0) in German format (returns #NUM! error)
        let result = codcel_log10(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_log10_negative() {
        // =LOG10(-10) in US format (returns #NUM! error)
        // =LOG10(-10) in German format (returns #NUM! error)
        let result = codcel_log10(-10.0);
        assert!(result.is_err());
    }
}
