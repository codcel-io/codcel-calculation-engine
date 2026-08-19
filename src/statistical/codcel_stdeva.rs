// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use std::error::Error;

/// Excel-compatible `STDEVA` that returns the sample standard deviation,
/// including text and logical values in the calculation.
/// - `values`: an array of numeric values (must have at least 2 values).
///
/// Text values are treated as 0, `TRUE` as 1, `FALSE` as 0 (coercion is
/// handled at the wrapper layer before calling this function).
///
/// Returns the sample standard deviation (divides by n-1),
/// or an error when the array has fewer than 2 values.
pub fn codcel_stdeva(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.len() < 2 {
        return Err(
            "STDEVA: At least two values are required to calculate standard deviation.".into(),
        );
    }

    let n = values.len() as f64;
    let mean = values.iter().compensated_sum() / n;

    let variance = values.iter().map(|x| (x - mean).powi(2)).compensated_sum() / (n - 1.0);

    Ok(crate::portable_math::sqrt(variance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stdeva_basic() {
        // =STDEVA({2,4,6,8,10}) in US format
        let values = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = codcel_stdeva(values).unwrap();
        assert!((result - 3.1623).abs() < 0.0001);
    }

    #[test]
    fn test_stdeva_same_values() {
        // =STDEVA({5,5,5,5,5}) in US format
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_stdeva(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_stdeva_two_values() {
        // =STDEVA({2,8}) in US format
        let values = vec![2.0, 8.0];
        let result = codcel_stdeva(values).unwrap();
        assert!((result - 4.2426).abs() < 0.0001);
    }

    #[test]
    fn test_stdeva_with_zeros_from_text() {
        // Simulates STDEVA where text values were coerced to 0
        // =STDEVA({1, "hello", 3}) would become {1, 0, 3}
        let values = vec![1.0, 0.0, 3.0];
        let result = codcel_stdeva(values).unwrap();
        // mean = 4/3 ≈ 1.3333, variance = ((1-4/3)^2 + (0-4/3)^2 + (3-4/3)^2) / 2
        // = (1/9 + 16/9 + 25/9) / 2 = (42/9) / 2 = 42/18 ≈ 2.3333
        // stddev = sqrt(2.3333) ≈ 1.5275
        assert!((result - 1.5275).abs() < 0.0001);
    }

    #[test]
    fn test_stdeva_with_boolean_coercion() {
        // Simulates STDEVA where TRUE=1, FALSE=0
        // =STDEVA({TRUE, FALSE, 5}) would become {1, 0, 5}
        let values = vec![1.0, 0.0, 5.0];
        let result = codcel_stdeva(values).unwrap();
        // mean = 2.0, variance = ((1-2)^2 + (0-2)^2 + (5-2)^2) / 2
        // = (1 + 4 + 9) / 2 = 7.0
        // stddev = sqrt(7) ≈ 2.6458
        assert!((result - 2.6458).abs() < 0.0001);
    }

    #[test]
    fn test_stdeva_negative_values() {
        // =STDEVA({-2,-4,-6,-8,-10}) in US format
        let values = vec![-2.0, -4.0, -6.0, -8.0, -10.0];
        let result = codcel_stdeva(values).unwrap();
        assert!((result - 3.1623).abs() < 0.0001);
    }

    #[test]
    fn test_stdeva_single_value() {
        let values = vec![7.0];
        let result = codcel_stdeva(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_stdeva_empty_array() {
        let values: Vec<f64> = vec![];
        let result = codcel_stdeva(values);
        assert!(result.is_err());
    }
}
