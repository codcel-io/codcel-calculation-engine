// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use std::error::Error;

/// Excel-compatible `VARPA` that returns the population variance,
/// including text and logical values in the calculation.
/// - `values`: an array of numeric values (must have at least 1 value).
///
/// Text values are treated as 0, `TRUE` as 1, `FALSE` as 0 (coercion is
/// handled at the wrapper layer before calling this function).
///
/// Returns the population variance (divides by n),
/// or an error when the array is empty.
pub fn codcel_varpa(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("VARPA: At least one value is required to calculate variance.".into());
    }

    let n = values.len() as f64;
    let mean = values.iter().compensated_sum() / n;

    let variance = values.iter().map(|x| (x - mean).powi(2)).compensated_sum() / n;

    Ok(variance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varpa_basic() {
        // =VARPA({2,4,6,8,10})
        let values = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = codcel_varpa(values).unwrap();
        // population variance = 8.0
        assert!((result - 8.0).abs() < 0.0001);
    }

    #[test]
    fn test_varpa_same_values() {
        // =VARPA({5,5,5,5,5})
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_varpa(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_varpa_two_values() {
        // =VARPA({2,8})
        let values = vec![2.0, 8.0];
        let result = codcel_varpa(values).unwrap();
        // mean=5, variance = ((2-5)^2 + (8-5)^2) / 2 = 9.0
        assert!((result - 9.0).abs() < 0.0001);
    }

    #[test]
    fn test_varpa_with_zeros_from_text() {
        // Simulates VARPA where text values were coerced to 0
        // =VARPA({1, "hello", 3}) would become {1, 0, 3}
        let values = vec![1.0, 0.0, 3.0];
        let result = codcel_varpa(values).unwrap();
        // mean = 4/3, variance = ((1-4/3)^2 + (0-4/3)^2 + (3-4/3)^2) / 3
        // = (42/9) / 3 = 42/27 ≈ 1.5556
        assert!((result - 1.5556).abs() < 0.0001);
    }

    #[test]
    fn test_varpa_single_value() {
        // =VARPA({7}) => 0 (single value has zero variance)
        let values = vec![7.0];
        let result = codcel_varpa(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_varpa_empty_array() {
        let values: Vec<f64> = vec![];
        let result = codcel_varpa(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_varpa_negative_values() {
        // =VARPA({-2,-4,-6,-8,-10})
        let values = vec![-2.0, -4.0, -6.0, -8.0, -10.0];
        let result = codcel_varpa(values).unwrap();
        // Same population variance as positive counterpart = 8.0
        assert!((result - 8.0).abs() < 0.0001);
    }
}
