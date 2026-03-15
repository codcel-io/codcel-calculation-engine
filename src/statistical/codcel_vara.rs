// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `VARA` that returns the sample variance,
/// including text and logical values in the calculation.
/// - `values`: an array of numeric values (must have at least 2 values).
///
/// Text values are treated as 0, `TRUE` as 1, `FALSE` as 0 (coercion is
/// handled at the wrapper layer before calling this function).
///
/// Returns the sample variance (divides by n-1),
/// or an error when the array has fewer than 2 values.
pub fn codcel_vara(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.len() < 2 {
        return Err(
            "VARA: At least two values are required to calculate variance.".into(),
        );
    }

    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;

    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);

    Ok(variance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vara_basic() {
        // =VARA({2,4,6,8,10})
        let values = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = codcel_vara(values).unwrap();
        // variance = 10.0 (sample variance)
        assert!((result - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_vara_same_values() {
        // =VARA({5,5,5,5,5})
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_vara(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_vara_two_values() {
        // =VARA({2,8})
        let values = vec![2.0, 8.0];
        let result = codcel_vara(values).unwrap();
        // mean=5, variance = ((2-5)^2 + (8-5)^2) / 1 = 18.0
        assert!((result - 18.0).abs() < 0.0001);
    }

    #[test]
    fn test_vara_with_zeros_from_text() {
        // Simulates VARA where text values were coerced to 0
        // =VARA({1, "hello", 3}) would become {1, 0, 3}
        let values = vec![1.0, 0.0, 3.0];
        let result = codcel_vara(values).unwrap();
        // mean = 4/3 ≈ 1.3333, variance = ((1-4/3)^2 + (0-4/3)^2 + (3-4/3)^2) / 2
        // = (1/9 + 16/9 + 25/9) / 2 = (42/9) / 2 = 42/18 ≈ 2.3333
        assert!((result - 2.3333).abs() < 0.0001);
    }

    #[test]
    fn test_vara_with_boolean_coercion() {
        // Simulates VARA where TRUE=1, FALSE=0
        // =VARA({TRUE, FALSE, 5}) would become {1, 0, 5}
        let values = vec![1.0, 0.0, 5.0];
        let result = codcel_vara(values).unwrap();
        // mean = 2.0, variance = ((1-2)^2 + (0-2)^2 + (5-2)^2) / 2
        // = (1 + 4 + 9) / 2 = 7.0
        assert!((result - 7.0).abs() < 0.0001);
    }

    #[test]
    fn test_vara_negative_values() {
        // =VARA({-2,-4,-6,-8,-10})
        let values = vec![-2.0, -4.0, -6.0, -8.0, -10.0];
        let result = codcel_vara(values).unwrap();
        // Same variance as positive counterpart = 10.0
        assert!((result - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_vara_single_value() {
        let values = vec![7.0];
        let result = codcel_vara(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_vara_empty_array() {
        let values: Vec<f64> = vec![];
        let result = codcel_vara(values);
        assert!(result.is_err());
    }
}
