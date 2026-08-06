// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `STDEV.S` that returns the standard deviation based on a sample.
/// - `values`: an array of numeric values representing a sample (must have at least 2 values).
///
/// Returns the sample standard deviation (divides by n-1),
/// or an error when the array has fewer than 2 values.
pub fn codcel_st_dev_s(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check for minimum number of values
    if values.len() < 2 {
        return Err(
            "STDEV.S: At least two values are required to calculate standard deviation.".into(),
        );
    }

    // Calculate mean
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;

    // Calculate variance (sample variance)
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);

    // Return standard deviation
    Ok(crate::portable_math::sqrt(variance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_st_dev_s_basic() {
        // =STDEV.S({2,4,6,8,10}) in US format
        // =STDEV.S({2;4;6;8;10}) in German format
        let values = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = codcel_st_dev_s(values).unwrap();
        assert!((result - 3.1623).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_s_same_values() {
        // =STDEV.S({5,5,5,5,5}) in US format
        // =STDEV.S({5;5;5;5;5}) in German format
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_st_dev_s(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_s_two_values() {
        // =STDEV.S({2,8}) in US format
        // =STDEV.S({2;8}) in German format
        let values = vec![2.0, 8.0];
        let result = codcel_st_dev_s(values).unwrap();
        assert!((result - 4.2426).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_s_negative_values() {
        // =STDEV.S({-2,-4,-6,-8,-10}) in US format
        // =STDEV.S({-2;-4;-6;-8;-10}) in German format
        let values = vec![-2.0, -4.0, -6.0, -8.0, -10.0];
        let result = codcel_st_dev_s(values).unwrap();
        assert!((result - 3.1623).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_s_mixed_values() {
        // =STDEV.S({-10,0,10,20,30}) in US format
        // =STDEV.S({-10;0;10;20;30}) in German format
        let values = vec![-10.0, 0.0, 10.0, 20.0, 30.0];
        let result = codcel_st_dev_s(values).unwrap();
        assert!((result - 15.8114).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_s_single_value() {
        // Single value should return an error
        let values = vec![7.0];
        let result = codcel_st_dev_s(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_st_dev_s_empty_array() {
        // Empty array should return an error
        let values: Vec<f64> = vec![];
        let result = codcel_st_dev_s(values);
        assert!(result.is_err());
    }
}
