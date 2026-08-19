// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use std::error::Error;

/// Excel-compatible `STDEV.P` that returns the standard deviation based on an entire population.
/// - `values`: an array of numeric values representing the entire population.
///
/// Returns the population standard deviation (divides by n),
/// or an error when the array is empty.
pub fn codcel_st_dev_dot_p(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check for minimum number of values
    if values.is_empty() {
        return Err(
            "STDEV.P: At least one value is required to calculate standard deviation.".into(),
        );
    }

    // Calculate mean
    let n = values.len() as f64;
    let mean = values.iter().compensated_sum() / n;

    // Calculate variance
    let variance = values.iter().map(|x| (x - mean).powi(2)).compensated_sum() / n;

    // Return standard deviation
    Ok(crate::portable_math::sqrt(variance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_st_dev_dot_p_basic() {
        // =STDEV.P({2,4,6,8,10}) in US format
        // =STDEV.P({2;4;6;8;10}) in German format
        let values = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = codcel_st_dev_dot_p(values).unwrap();
        assert!((result - 2.8284).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_dot_p_same_values() {
        // =STDEV.P({5,5,5,5,5}) in US format
        // =STDEV.P({5;5;5;5;5}) in German format
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_st_dev_dot_p(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_dot_p_single_value() {
        // =STDEV.P({7}) in US format
        // =STDEV.P({7}) in German format
        let values = vec![7.0];
        let result = codcel_st_dev_dot_p(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_dot_p_negative_values() {
        // =STDEV.P({-2,-4,-6,-8,-10}) in US format
        // =STDEV.P({-2;-4;-6;-8;-10}) in German format
        let values = vec![-2.0, -4.0, -6.0, -8.0, -10.0];
        let result = codcel_st_dev_dot_p(values).unwrap();
        assert!((result - 2.8284).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_dot_p_mixed_values() {
        // =STDEV.P({-10,0,10,20,30}) in US format
        // =STDEV.P({-10;0;10;20;30}) in German format
        let values = vec![-10.0, 0.0, 10.0, 20.0, 30.0];
        let result = codcel_st_dev_dot_p(values).unwrap();
        assert!((result - 14.1421).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_dot_p_empty_array() {
        // Empty array should return an error
        let values: Vec<f64> = vec![];
        let result = codcel_st_dev_dot_p(values);
        assert!(result.is_err());
    }
}
