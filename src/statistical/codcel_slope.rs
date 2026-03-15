// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `SLOPE` that returns the slope of a linear regression line.
/// - `known_ys`: the dependent array of known y-values.
/// - `known_xs`: the independent array of known x-values (must have the same length as `known_ys`).
///
/// Returns the slope (m) of the best-fit line y = mx + b,
/// or an error when arrays are empty or have different lengths.
pub fn codcel_slope(
    known_ys: Vec<f64>,
    known_xs: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if known_ys.len() != known_xs.len() {
        return Err("SLOPE: The length of known_ys and known_xs must be the same.".into());
    }

    if known_ys.is_empty() {
        return Err("SLOPE: Input arrays must not be empty.".into());
    }

    let n = known_ys.len() as f64;

    // Compute means
    let mean_y = known_ys.iter().sum::<f64>() / n;
    let mean_x = known_xs.iter().sum::<f64>() / n;

    // Compute sums for covariance and variance
    let mut sum_covariance = 0.0;
    let mut sum_variance_x = 0.0;

    for (&y, &x) in known_ys.iter().zip(&known_xs) {
        let dev_y = y - mean_y;
        let dev_x = x - mean_x;

        sum_covariance += dev_x * dev_y;
        sum_variance_x += dev_x * dev_x;
    }

    if sum_variance_x == 0.0 {
        return Err("SLOPE: Variance of x is zero, slope cannot be computed.".into());
    }

    // Calculate slope
    let slope = sum_covariance / sum_variance_x;

    Ok(slope)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope_positive() {
        // =SLOPE({2,3,4,5,6}, {1,2,3,4,5}) in US format
        // =SLOPE({2;3;4;5;6}; {1;2;3;4;5}) in German format
        let known_ys = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_slope(known_ys, known_xs).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_slope_negative() {
        // =SLOPE({6,5,4,3,2}, {1,2,3,4,5}) in US format
        // =SLOPE({6;5;4;3;2}; {1;2;3;4;5}) in German format
        let known_ys = vec![6.0, 5.0, 4.0, 3.0, 2.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_slope(known_ys, known_xs).unwrap();
        assert!((result - (-1.0)).abs() < 0.0001);
    }

    #[test]
    fn test_slope_zero() {
        // =SLOPE({5,5,5,5,5}, {1,2,3,4,5}) in US format
        // =SLOPE({5;5;5;5;5}; {1;2;3;4;5}) in German format
        let known_ys = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_slope(known_ys, known_xs).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_slope_scattered_data() {
        // =SLOPE({2,3,9,1,8}, {7,5,4,5,2}) in US format
        // =SLOPE({2;3;9;1;8}; {7;5;4;5;2}) in German format
        let known_ys = vec![2.0, 3.0, 9.0, 1.0, 8.0];
        let known_xs = vec![7.0, 5.0, 4.0, 5.0, 2.0];
        let result = codcel_slope(known_ys, known_xs).unwrap();
        println!("{result}");
        assert!((result - (-1.5000000000000002)).abs() < 0.0001);
    }

    #[test]
    fn test_slope_different_length_arrays() {
        // Different length arrays should return an error
        let known_ys = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_slope(known_ys, known_xs);
        assert!(result.is_err());
    }

    #[test]
    fn test_slope_empty_arrays() {
        // Empty arrays should return an error
        let known_ys: Vec<f64> = vec![];
        let known_xs: Vec<f64> = vec![];
        let result = codcel_slope(known_ys, known_xs);
        assert!(result.is_err());
    }

    #[test]
    fn test_slope_zero_variance_x() {
        // Zero variance in x should return an error
        let known_ys = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let known_xs = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_slope(known_ys, known_xs);
        assert!(result.is_err());
    }
}
