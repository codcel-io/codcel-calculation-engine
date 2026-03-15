// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `STANDARDIZE` that returns a normalized value (z-score).
/// - `x`: the value to normalize.
/// - `mean`: the arithmetic mean of the distribution.
/// - `standard_dev`: the standard deviation of the distribution (must be > 0).
///
/// Returns (x - mean) / standard_dev, the number of standard deviations from the mean.
pub fn codcel_standardize(
    x: f64,
    mean: f64,
    standard_dev: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if standard_dev <= 0.0 {
        return Err("STANDARDIZE: Standard deviation must be a positive value.".into());
    }

    Ok((x - mean) / standard_dev)
}

pub fn codcel_standardize_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("STANDARDIZE: Must have 3 parameters.".into());
    }

    codcel_standardize(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standardize_basic() {
        // =STANDARDIZE(42, 40, 1.5) in US format
        // =STANDARDIZE(42; 40; 1,5) in German format
        let result = codcel_standardize(42.0, 40.0, 1.5).unwrap();
        assert!((result - 1.3333).abs() < 0.0001);
    }

    #[test]
    fn test_standardize_negative_value() {
        // =STANDARDIZE(-3, 0, 1) in US format
        // =STANDARDIZE(-3; 0; 1) in German format
        let result = codcel_standardize(-3.0, 0.0, 1.0).unwrap();
        assert!((result - (-3.0)).abs() < 0.0001);
    }

    #[test]
    fn test_standardize_equal_to_mean() {
        // =STANDARDIZE(10, 10, 2) in US format
        // =STANDARDIZE(10; 10; 2) in German format
        let result = codcel_standardize(10.0, 10.0, 2.0).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_standardize_negative_mean() {
        // =STANDARDIZE(5, -5, 2.5) in US format
        // =STANDARDIZE(5; -5; 2,5) in German format
        let result = codcel_standardize(5.0, -5.0, 2.5).unwrap();
        assert!((result - 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_standardize_zero_standard_dev() {
        // Zero standard deviation should return an error
        let result = codcel_standardize(42.0, 40.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_standardize_negative_standard_dev() {
        // Negative standard deviation should return an error
        let result = codcel_standardize(42.0, 40.0, -1.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_standardize_vec_basic() {
        // =STANDARDIZE(42, 40, 1.5) in US format
        // =STANDARDIZE(42; 40; 1,5) in German format
        let inputs = vec![42.0, 40.0, 1.5];
        let result = codcel_standardize_vec(inputs).unwrap();
        assert!((result - 1.3333).abs() < 0.0001);
    }

    #[test]
    fn test_standardize_vec_wrong_length() {
        // Wrong number of parameters should return an error
        let inputs = vec![42.0, 40.0];
        let result = codcel_standardize_vec(inputs);
        assert!(result.is_err());
    }
}
