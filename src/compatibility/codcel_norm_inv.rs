// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::{ContinuousCDF, Normal};
use std::error::Error;

/// Excel-compatible `NORMINV`/`NORM.INV` function.
/// Returns the inverse normal distribution for a cumulative probability.
/// - `probability`: cumulative probability value in `[0, 1]`.
/// - `mean`: arithmetic mean of the distribution.
/// - `std_dev`: standard deviation of the distribution (must be greater than 0).
///
/// Returns an error on probabilities outside `[0, 1]` or non-positive standard deviation.
pub fn codcel_norm_inv(
    probability: f64,
    mean: f64,
    std_dev: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if !(0.0..=1.0).contains(&probability) {
        return Err("NORMINV: Probability must be between 0 and 1 inclusive".into());
    }

    if std_dev <= 0.0 {
        return Err("NORMINV: Standard deviation must be greater than 0".into());
    }

    // Create a normal distribution with the given mean and standard deviation
    let normal_dist = Normal::new(mean, std_dev)
        .map_err(|e| format!("NORMINV: Failed to create normal distribution: {e}"))?;

    // Calculate the inverse cumulative distribution (percent-point function)
    let result = normal_dist.inverse_cdf(probability);

    Ok(result)
}

/// Convenience wrapper for `NORMINV` that accepts `[probability, mean, std_dev]`.
/// Errors if the vector does not contain exactly three values.
pub fn codcel_norm_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("NORMINV: Must have 3 parameters.".into());
    }

    codcel_norm_inv(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_inv_basic() {
        // =NORMINV(0.5, 0, 1) in US format
        // =NORMINV(0,5; 0; 1) in German format
        let result = codcel_norm_inv(0.5, 0.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_norm_inv_low_probability() {
        // =NORMINV(0.1, 0, 1) in US format
        // =NORMINV(0,1; 0; 1) in German format
        let result = codcel_norm_inv(0.1, 0.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - (-1.2815515655)).abs() < 0.0001);
    }

    #[test]
    fn test_norm_inv_high_probability() {
        // =NORMINV(0.9, 0, 1) in US format
        // =NORMINV(0,9; 0; 1) in German format
        let result = codcel_norm_inv(0.9, 0.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 1.2815515655).abs() < 0.0001);
    }

    #[test]
    fn test_norm_inv_different_mean_std_dev() {
        // =NORMINV(0.5, 10, 2) in US format
        // =NORMINV(0,5; 10; 2) in German format
        let result = codcel_norm_inv(0.5, 10.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_norm_inv_extreme_probability() {
        // =NORMINV(0.999, 0, 1) in US format
        // =NORMINV(0,999; 0; 1) in German format
        let result = codcel_norm_inv(0.999, 0.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 3.0902323062).abs() < 0.0001);
    }

    #[test]
    fn test_norm_inv_invalid_probability() {
        // =NORMINV(1.5, 0, 1) in US format
        // =NORMINV(1,5; 0; 1) in German format
        let result = codcel_norm_inv(1.5, 0.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_inv_zero_std_dev() {
        // =NORMINV(0.5, 0, 0) in US format
        // =NORMINV(0,5; 0; 0) in German format
        let result = codcel_norm_inv(0.5, 0.0, 0.0);
        assert!(result.is_err());
    }
}
