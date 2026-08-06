// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `FORECAST` that predicts a value based on linear regression.
/// - `x`: the data point for which to predict a value.
/// - `known_ys`: the dependent array of known values.
/// - `known_xs`: the independent array of known values (must have the same length as `known_ys`).
///
/// Returns the predicted y value for the given x using linear regression,
/// or an error when arrays are empty, have different lengths, or have zero variance in x.
pub fn codcel_forecast(
    x: f64,
    known_ys: Vec<f64>,
    known_xs: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if known_ys.len() != known_xs.len() {
        return Err("FORECAST: known_ys and known_xs must have the same size.".into());
    }
    if known_ys.is_empty() || known_xs.is_empty() {
        return Err("FORECAST: known_ys and known_xs must not be empty.".into());
    }

    let mean_x = known_xs.iter().sum::<f64>() / known_xs.len() as f64;
    let mean_y = known_ys.iter().sum::<f64>() / known_ys.len() as f64;

    let covariance = known_xs
        .iter()
        .zip(&known_ys)
        .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
        .sum::<f64>();

    let variance_x = known_xs
        .iter()
        .map(|&xi| (xi - mean_x).powi(2))
        .sum::<f64>();

    if variance_x == 0.0 {
        return Err("FORECAST: Division by zero due to zero variance in known_xs.".into());
    }

    let slope = covariance / variance_x;
    let intercept = mean_y - slope * mean_x;

    Ok(slope * x + intercept)
}

// This function is a placeholder for a vector version of FORECAST
// In reality, FORECAST requires one value and two arrays, so a simple vector of inputs is not sufficient
// This is just to maintain consistency with other functions
pub fn codcel_forecast_vec(
    inputs: Vec<f64>,
    known_ys: Vec<f64>,
    known_xs: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("FORECAST: Must have 1 x parameter".into());
    }

    codcel_forecast(inputs[0], known_ys, known_xs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forecast_basic() {
        // =FORECAST(6, {2,4,6,8}, {1,2,3,4}) in US format
        // =FORECAST(6; {2;4;6;8}; {1;2;3;4}) in German format
        let known_ys = vec![2.0, 4.0, 6.0, 8.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_forecast(6.0, known_ys, known_xs).unwrap();
        println!("{result}");
        assert!((result - 12.0).abs() < 0.0001);
    }

    #[test]
    fn test_forecast_non_linear_data() {
        // =FORECAST(6, {1,3,6,10}, {1,2,3,4}) in US format
        // =FORECAST(6; {1;3;6;10}; {1;2;3;4}) in German format
        let known_ys = vec![1.0, 3.0, 6.0, 10.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_forecast(6.0, known_ys, known_xs).unwrap();
        println!("{result}");
        assert!((result - 15.5).abs() < 0.0001);
    }

    #[test]
    fn test_forecast_negative_values() {
        // =FORECAST(0, {-2,-1,0,1,2}, {-2,-1,0,1,2}) in US format
        // =FORECAST(0; {-2;-1;0;1;2}; {-2;-1;0;1;2}) in German format
        let known_ys = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let known_xs = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let result = codcel_forecast(0.0, known_ys, known_xs).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_forecast_different_length_arrays() {
        // Different length arrays should return an error
        let known_ys = vec![2.0, 4.0, 6.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_forecast(6.0, known_ys, known_xs);
        assert!(result.is_err());
    }

    #[test]
    fn test_forecast_empty_arrays() {
        // Empty arrays should return an error
        let known_ys: Vec<f64> = vec![];
        let known_xs: Vec<f64> = vec![];
        let result = codcel_forecast(6.0, known_ys, known_xs);
        assert!(result.is_err());
    }

    #[test]
    fn test_forecast_zero_variance() {
        // Zero variance in known_xs should return an error
        let known_ys = vec![2.0, 4.0, 6.0, 8.0];
        let known_xs = vec![1.0, 1.0, 1.0, 1.0];
        let result = codcel_forecast(6.0, known_ys, known_xs);
        assert!(result.is_err());
    }

    #[test]
    fn test_forecast_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![6.0];
        let known_ys = vec![2.0, 4.0, 6.0, 8.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_forecast_vec(inputs, known_ys, known_xs).unwrap();
        println!("{result}");
        assert!((result - 12.0).abs() < 0.0001);
    }

    #[test]
    fn test_forecast_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![6.0, 7.0];
        let known_ys = vec![2.0, 4.0, 6.0, 8.0];
        let known_xs = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_forecast_vec(inputs, known_ys, known_xs);
        assert!(result.is_err());
    }
}
