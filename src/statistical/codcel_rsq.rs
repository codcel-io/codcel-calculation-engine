// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::{CompensatedSum, CompensatedSumExt};
use std::error::Error;

/// Excel-compatible `RSQ` that returns the square of the Pearson correlation coefficient (R²).
/// - `known_ys`: the dependent array of known y-values.
/// - `known_xs`: the independent array of known x-values (must have the same length as `known_ys`).
///
/// Returns R², the coefficient of determination (0 to 1),
/// or an error when arrays are empty or have different lengths.
pub fn codcel_rsq(
    known_ys: Vec<f64>,
    known_xs: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if known_ys.len() != known_xs.len() {
        return Err("RSQ: The length of known_ys and known_xs must be the same.".into());
    }

    if known_ys.is_empty() {
        return Err("RSQ: Input arrays must not be empty.".into());
    }

    let n = known_ys.len() as f64;

    // Compute means
    let mean_y = known_ys.iter().compensated_sum() / n;
    let mean_x = known_xs.iter().compensated_sum() / n;

    // Compute sums for covariance and variances
    let mut sum_covariance = CompensatedSum::new();
    let mut sum_variance_x = CompensatedSum::new();
    let mut sum_variance_y = CompensatedSum::new();

    for (&y, &x) in known_ys.iter().zip(&known_xs) {
        let dev_y = y - mean_y;
        let dev_x = x - mean_x;

        sum_covariance.add(dev_x * dev_y);
        sum_variance_x.add(dev_x * dev_x);
        sum_variance_y.add(dev_y * dev_y);
    }

    // If either variance is zero, correlation cannot be computed
    if sum_variance_x.total() == 0.0 || sum_variance_y.total() == 0.0 {
        return Err("RSQ: Variance is zero, correlation cannot be computed.".into());
    }

    // Calculate r-squared
    let r_squared = (sum_covariance.total() * sum_covariance.total())
        / (sum_variance_x.total() * sum_variance_y.total());

    Ok(r_squared)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsq_perfect_correlation() {
        // =RSQ({2,3,4,5,6}, {1,2,3,4,5}) in US format
        // =RSQ({2;3;4;5;6}; {1;2;3;4;5}) in German format
        let known_ys = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_rsq(known_ys, known_xs).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_rsq_high_correlation() {
        // =RSQ({2,3,9,1,8}, {7,5,4,5,2}) in US format
        // =RSQ({2;3;9;1;8}; {7;5;4;5;2}) in German format
        let known_ys = vec![2.0, 3.0, 9.0, 1.0, 8.0];
        let known_xs = vec![7.0, 5.0, 4.0, 5.0, 2.0];
        let result = codcel_rsq(known_ys, known_xs).unwrap();
        println!("{result}");
        assert!((result - 0.5582706766917294).abs() < 0.0001);
    }

    #[test]
    fn test_rsq_negative_correlation() {
        // =RSQ({9,7,5,3,1}, {1,2,3,4,5}) in US format
        // =RSQ({9;7;5;3;1}; {1;2;3;4;5}) in German format
        let known_ys = vec![9.0, 7.0, 5.0, 3.0, 1.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_rsq(known_ys, known_xs).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_rsq_no_correlation() {
        // =RSQ({1,2,3,4,5}, {5,5,5,5,5}) in US format
        // =RSQ({1;2;3;4;5}; {5;5;5;5;5}) in German format
        let known_ys = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let known_xs = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_rsq(known_ys, known_xs);
        assert!(result.is_err()); // Zero variance in x
    }

    #[test]
    fn test_rsq_different_length_arrays() {
        // Different length arrays should return an error
        let known_ys = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_rsq(known_ys, known_xs);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsq_empty_arrays() {
        // Empty arrays should return an error
        let known_ys: Vec<f64> = vec![];
        let known_xs: Vec<f64> = vec![];
        let result = codcel_rsq(known_ys, known_xs);
        assert!(result.is_err());
    }

    #[test]
    fn test_rsq_zero_variance_y() {
        // Zero variance in y should return an error
        let known_ys = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_rsq(known_ys, known_xs);
        assert!(result.is_err());
    }
}
