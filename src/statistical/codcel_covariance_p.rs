// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::{CompensatedSum, CompensatedSumExt};
use std::error::Error;

/// Excel-compatible `COVARIANCE.P` that returns the population covariance of two data sets.
/// - `x`: the first array of values.
/// - `y`: the second array of values (must have the same length as `x`).
///
/// Returns the population covariance (divides by n),
/// or an error when arrays are empty or have different lengths.
pub fn codcel_covariance_p(x: Vec<f64>, y: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if x.is_empty() || y.is_empty() {
        return Err("COVARIANCE.P: Input arrays must not be empty.".into());
    }
    if x.len() != y.len() {
        return Err("COVARIANCE.P: Input arrays must have the same length.".into());
    }

    // Calculate means
    let x_mean = x.iter().compensated_sum() / x.len() as f64;
    let y_mean = y.iter().compensated_sum() / y.len() as f64;

    // Calculate covariance
    let mut covariance = CompensatedSum::new();

    for (&x_val, &y_val) in x.iter().zip(y.iter()) {
        covariance.add((x_val - x_mean) * (y_val - y_mean));
    }

    Ok(covariance.total() / x.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covariance_p_positive_covariance() {
        // =COVARIANCE.P({3,4,5,6,7},{8,9,10,11,12}) in US format
        // =COVARIANCE.P({3;4;5;6;7};{8;9;10;11;12}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![8.0, 9.0, 10.0, 11.0, 12.0];
        let result = codcel_covariance_p(x, y).unwrap();
        assert!((result - 2.0).abs() < 0.0000001);
    }

    #[test]
    fn test_covariance_p_negative_covariance() {
        // =COVARIANCE.P({3,4,5,6,7},{12,11,10,9,8}) in US format
        // =COVARIANCE.P({3;4;5;6;7};{12;11;10;9;8}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![12.0, 11.0, 10.0, 9.0, 8.0];
        let result = codcel_covariance_p(x, y).unwrap();
        assert!((result - (-2.0)).abs() < 0.0000001);
    }

    #[test]
    fn test_covariance_p_no_covariance() {
        // =COVARIANCE.P({3,4,5,6,7},{10,10,10,10,10}) in US format
        // =COVARIANCE.P({3;4;5;6;7};{10;10;10;10;10}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![10.0, 10.0, 10.0, 10.0, 10.0];
        let result = codcel_covariance_p(x, y).unwrap();
        assert!((result - 0.0).abs() < 0.0000001);
    }

    #[test]
    fn test_covariance_p_mixed_values() {
        // =COVARIANCE.P({3,4,5,6,7},{8,7,9,10,12}) in US format
        // =COVARIANCE.P({3;4;5;6;7};{8;7;9;10;12}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![8.0, 7.0, 9.0, 10.0, 12.0];
        let result = codcel_covariance_p(x, y).unwrap();
        println!("{result}");
        assert!((result - 2.2).abs() < 0.0001);
    }

    #[test]
    fn test_covariance_p_single_pair() {
        // =COVARIANCE.P({5},{10}) in US format
        // =COVARIANCE.P({5};{10}) in German format
        let x = vec![5.0];
        let y = vec![10.0];
        let result = codcel_covariance_p(x, y).unwrap();
        // With a single pair, covariance is 0
        assert!((result - 0.0).abs() < 0.0000001);
    }

    #[test]
    fn test_covariance_p_empty_arrays() {
        // Empty arrays should return an error
        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];
        let result = codcel_covariance_p(x, y);
        assert!(result.is_err());
    }

    #[test]
    fn test_covariance_p_different_lengths() {
        // Arrays of different lengths should return an error
        let x = vec![3.0, 4.0, 5.0];
        let y = vec![8.0, 9.0, 10.0, 11.0];
        let result = codcel_covariance_p(x, y);
        assert!(result.is_err());
    }

    #[test]
    fn test_covariance_p_decimal_values() {
        // =COVARIANCE.P({2.5,3.5,4.5,5.5,6.5},{7.5,8.5,9.5,10.5,11.5}) in US format
        // =COVARIANCE.P({2,5;3,5;4,5;5,5;6,5};{7,5;8,5;9,5;10,5;11,5}) in German format
        let x = vec![2.5, 3.5, 4.5, 5.5, 6.5];
        let y = vec![7.5, 8.5, 9.5, 10.5, 11.5];
        let result = codcel_covariance_p(x, y).unwrap();
        assert!((result - 2.0).abs() < 0.0000001);
    }
}
