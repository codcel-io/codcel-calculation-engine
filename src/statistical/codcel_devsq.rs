// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `DEVSQ` that returns the sum of squares of deviations from the mean.
/// - `data`: an array of numeric values.
///
/// Returns the sum of the squared deviations of each data point from the sample mean,
/// or an error when the input array is empty.
pub fn codcel_devsq(data: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if data.is_empty() {
        return Err("DEVSQ: Input array must not be empty.".into());
    }

    // Calculate the mean
    let mean = data.iter().sum::<f64>() / data.len() as f64;

    // Calculate the sum of squared deviations
    let devsq = data.iter().map(|&value| (value - mean).powi(2)).sum();

    Ok(devsq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devsq_basic() {
        // =DEVSQ({3,4,5,6,7}) in US format
        // =DEVSQ({3;4;5;6;7}) in German format
        let data = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let result = codcel_devsq(data).unwrap();
        assert!((result - 10.0).abs() < 0.0000001);
    }

    #[test]
    fn test_devsq_negative_values() {
        // =DEVSQ({-3,-4,-5,-6,-7}) in US format
        // =DEVSQ({-3;-4;-5;-6;-7}) in German format
        let data = vec![-3.0, -4.0, -5.0, -6.0, -7.0];
        let result = codcel_devsq(data).unwrap();
        assert!((result - 10.0).abs() < 0.0000001);
    }

    #[test]
    fn test_devsq_mixed_values() {
        // =DEVSQ({-2,-1,0,1,2}) in US format
        // =DEVSQ({-2;-1;0;1;2}) in German format
        let data = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let result = codcel_devsq(data).unwrap();
        assert!((result - 10.0).abs() < 0.0000001);
    }

    #[test]
    fn test_devsq_single_value() {
        // =DEVSQ({5}) in US format
        // =DEVSQ({5}) in German format
        let data = vec![5.0];
        let result = codcel_devsq(data).unwrap();
        // With a single value, the deviation from the mean is always 0
        assert!((result - 0.0).abs() < 0.0000001);
    }

    #[test]
    fn test_devsq_same_values() {
        // =DEVSQ({5,5,5,5,5}) in US format
        // =DEVSQ({5;5;5;5;5}) in German format
        let data = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_devsq(data).unwrap();
        // With all same values, the deviation from the mean is always 0
        assert!((result - 0.0).abs() < 0.0000001);
    }

    #[test]
    fn test_devsq_decimal_values() {
        // =DEVSQ({2.5,3.5,4.5,5.5,6.5}) in US format
        // =DEVSQ({2,5;3,5;4,5;5,5;6,5}) in German format
        let data = vec![2.5, 3.5, 4.5, 5.5, 6.5];
        let result = codcel_devsq(data).unwrap();
        assert!((result - 10.0).abs() < 0.0000001);
    }

    #[test]
    fn test_devsq_large_values() {
        // =DEVSQ({1000,2000,3000,4000,5000}) in US format
        // =DEVSQ({1000;2000;3000;4000;5000}) in German format
        let data = vec![1000.0, 2000.0, 3000.0, 4000.0, 5000.0];
        let result = codcel_devsq(data).unwrap();
        assert!((result - 10000000.0).abs() < 0.0000001);
    }

    #[test]
    fn test_devsq_empty_array() {
        // Empty array should return an error
        let data: Vec<f64> = vec![];
        let result = codcel_devsq(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_devsq_formula_verification() {
        // Verify that DEVSQ equals the sum of (x - mean)^2
        let data = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let result = codcel_devsq(data.clone()).unwrap();

        // Calculate manually
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let manual_devsq = data
            .iter()
            .map(|&value| (value - mean).powi(2))
            .sum::<f64>();

        assert!((result - manual_devsq).abs() < 0.0000001);
    }
}
