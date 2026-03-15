// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `INTERCEPT` that returns the y-intercept of a linear regression line.
/// - `known_ys`: the dependent array of known y-values.
/// - `known_xs`: the independent array of known x-values (must have the same length as `known_ys`).
///
/// Returns the point at which the regression line crosses the y-axis,
/// or an error when arrays are empty or have different lengths.
pub fn codcel_intercept(
    known_ys: Vec<f64>,
    known_xs: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if known_ys.is_empty() || known_xs.is_empty() {
        return Err("INTERCEPT: Both x_values and y_values must not be empty.".into());
    }
    if known_ys.len() != known_xs.len() {
        return Err("INTERCEPT: x_values and y_values must have the same length.".into());
    }

    let n = known_ys.len() as f64;
    let sum_x: f64 = known_xs.iter().sum();
    let sum_y: f64 = known_ys.iter().sum();
    let sum_xy: f64 = known_xs.iter().zip(&known_ys).map(|(&x, &y)| x * y).sum();
    let sum_x2: f64 = known_xs.iter().map(|&x| x * x).sum();

    let denominator = n * sum_x2 - sum_x.powi(2);
    if denominator == 0.0 {
        return Err("INTERCEPT: Division by zero error due to collinear data.".into());
    }

    let slope = (n * sum_xy - sum_x * sum_y) / denominator;
    let intercept = (sum_y - slope * sum_x) / n;

    Ok(intercept)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intercept_basic() {
        // =INTERCEPT({2,3,9,1,8}, {6,5,11,7,5}) in US format
        // =INTERCEPT({2;3;9;1;8}; {6;5;11;7;5}) in German format
        let known_ys = vec![2.0, 3.0, 9.0, 1.0, 8.0];
        let known_xs = vec![6.0, 5.0, 11.0, 7.0, 5.0];
        let result = codcel_intercept(known_ys, known_xs).unwrap();
        assert!((result - 0.0483870967741935).abs() < 1e-10);
    }

    #[test]
    fn test_intercept_perfect_line() {
        // =INTERCEPT({1,2,3,4,5}, {1,2,3,4,5}) in US format
        // =INTERCEPT({1;2;3;4;5}; {1;2;3;4;5}) in German format
        let known_ys = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_intercept(known_ys, known_xs).unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_intercept_negative_slope() {
        // =INTERCEPT({5,4,3,2,1}, {1,2,3,4,5}) in US format
        // =INTERCEPT({5;4;3;2;1}; {1;2;3;4;5}) in German format
        let known_ys = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_intercept(known_ys, known_xs).unwrap();
        assert!((result - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_intercept_decimal_values() {
        // =INTERCEPT({1.5,2.5,3.5,4.5}, {1.1,2.2,3.3,4.4}) in US format
        // =INTERCEPT({1,5;2,5;3,5;4,5}; {1,1;2,2;3,3;4,4}) in German format
        let known_ys = vec![1.5, 2.5, 3.5, 4.5];
        let known_xs = vec![1.1, 2.2, 3.3, 4.4];
        let result = codcel_intercept(known_ys, known_xs).unwrap();
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_intercept_different_lengths() {
        // =INTERCEPT({1,2,3}, {1,2}) in US format
        // =INTERCEPT({1;2;3}; {1;2}) in German format
        let known_ys = vec![1.0, 2.0, 3.0];
        let known_xs = vec![1.0, 2.0];
        let result = codcel_intercept(known_ys, known_xs);
        assert!(result.is_err());
    }

    #[test]
    fn test_intercept_empty_arrays() {
        // =INTERCEPT({}, {}) in US format
        // =INTERCEPT({}; {}) in German format
        let known_ys: Vec<f64> = vec![];
        let known_xs: Vec<f64> = vec![];
        let result = codcel_intercept(known_ys, known_xs);
        assert!(result.is_err());
    }

    #[test]
    fn test_intercept_collinear_data() {
        // =INTERCEPT({1,2,3}, {5,5,5}) in US format
        // =INTERCEPT({1;2;3}; {5;5;5}) in German format
        let known_ys = vec![1.0, 2.0, 3.0];
        let known_xs = vec![5.0, 5.0, 5.0];
        let result = codcel_intercept(known_ys, known_xs);
        assert!(result.is_err());
    }
}
