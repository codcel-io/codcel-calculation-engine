// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `SUMXMY2` that returns the sum of squares of differences.
/// - `x`: the first array of numbers.
/// - `y`: the second array of numbers.
///
/// Returns Σ(x - y)² or an error when arrays have different lengths.
pub fn codcel_sum_xmy2(x: Vec<f64>, y: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if arrays have the same length
    if x.len() != y.len() {
        return Err("SUMXMY2: Arrays must have the same length".into());
    }

    if x.is_empty() {
        return Ok(0.0);
    }

    // Calculate the sum of (x - y)^2 for each pair of elements
    let result = x
        .iter()
        .zip(y.iter())
        .map(|(x_val, y_val)| (x_val - y_val).powi(2))
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_xmy2_positive_numbers() {
        // =SUMXMY2({1,2,3},{4,5,6}) in US format
        // =SUMXMY2({1;2;3};{4;5;6}) in German format
        let result = codcel_sum_xmy2(vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]).unwrap();
        assert_eq!(result, 27.0); // (1-4)^2 + (2-5)^2 + (3-6)^2 = 9 + 9 + 9 = 27
    }

    #[test]
    fn test_sum_xmy2_negative_numbers() {
        // =SUMXMY2({-1,-2,-3},{-4,-5,-6}) in US format
        // =SUMXMY2({-1;-2;-3};{-4;-5;-6}) in German format
        let result = codcel_sum_xmy2(vec![-1.0, -2.0, -3.0], vec![-4.0, -5.0, -6.0]).unwrap();
        assert_eq!(result, 27.0); // (-1-(-4))^2 + (-2-(-5))^2 + (-3-(-6))^2 = 3^2 + 3^2 + 3^2 = 9 + 9 + 9 = 27
    }

    #[test]
    fn test_sum_xmy2_mixed_numbers() {
        // =SUMXMY2({1,-2,3},{-4,5,-6}) in US format
        // =SUMXMY2({1;-2;3};{-4;5;-6}) in German format
        let result = codcel_sum_xmy2(vec![1.0, -2.0, 3.0], vec![-4.0, 5.0, -6.0]).unwrap();
        assert_eq!(result, 155.0); // (1-(-4))^2 + (-2-5)^2 + (3-(-6))^2 = 5^2 + (-7)^2 + 9^2 = 25 + 49 + 81 = 155
    }

    #[test]
    fn test_sum_xmy2_single_element() {
        // =SUMXMY2({5},{3}) in US format
        // =SUMXMY2({5};{3}) in German format
        let result = codcel_sum_xmy2(vec![5.0], vec![3.0]).unwrap();
        assert_eq!(result, 4.0); // (5-3)^2 = 2^2 = 4
    }

    #[test]
    fn test_sum_xmy2_empty_arrays() {
        // =SUMXMY2({},{}) in US format
        // =SUMXMY2({};{}) in German format
        let result = codcel_sum_xmy2(vec![], vec![]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sum_xmy2_different_lengths() {
        // =SUMXMY2({1,2,3},{4,5}) in US format (returns #N/A error)
        // =SUMXMY2({1;2;3};{4;5}) in German format (returns #N/A error)
        let result = codcel_sum_xmy2(vec![1.0, 2.0, 3.0], vec![4.0, 5.0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_xmy2_decimals() {
        // =SUMXMY2({1.5,2.5},{3.5,4.5}) in US format
        // =SUMXMY2({1,5;2,5};{3,5;4,5}) in German format
        let result = codcel_sum_xmy2(vec![1.5, 2.5], vec![3.5, 4.5]).unwrap();
        assert!((result - 8.0).abs() < 1e-10); // (1.5-3.5)^2 + (2.5-4.5)^2 = (-2)^2 + (-2)^2 = 4 + 4 = 8
    }

    #[test]
    fn test_sum_xmy2_equal_values() {
        // =SUMXMY2({1,2,3},{1,2,3}) in US format
        // =SUMXMY2({1;2;3};{1;2;3}) in German format
        let result = codcel_sum_xmy2(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(result, 0.0); // (1-1)^2 + (2-2)^2 + (3-3)^2 = 0 + 0 + 0 = 0
    }
}
