// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::function::erf::erf;
use std::error::Error;

/// Excel-compatible `LOGNORMDIST`/`LOGNORM.DIST` function.
/// Returns the cumulative log-normal distribution.
/// - `x`: value at which to evaluate (must be greater than 0).
/// - `mean`: mean of ln(x).
/// - `std_dev`: standard deviation of ln(x) (must be greater than 0).
///
/// Returns an error when `x` is non-positive or the standard deviation is not greater than zero.
pub fn codcel_log_norm_dist(
    x: f64,
    mean: f64,
    std_dev: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if x <= 0.0 {
        return Err("x must be greater than 0".into());
    }
    if std_dev <= 0.0 {
        return Err("standard deviation must be greater than 0".into());
    }

    // Calculate the cumulative lognormal distribution
    let ln_x = x.ln();

    // Using the error function (erf) to calculate the cumulative distribution
    let z = (ln_x - mean) / (std_dev * 2.0_f64.sqrt());

    // The cumulative distribution is related to the error function:
    // CDF = 0.5 * (1 + erf(z/√2))
    let result = 0.5 * (1.0 + erf(z));

    Ok(result)
}

/// Convenience wrapper for `LOGNORMDIST` that accepts `[x, mean, std_dev]`.
/// Errors if the vector does not contain exactly three values.
pub fn codcel_log_norm_dist_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("LOGNORMDIST: Must have 3 parameters.".into());
    }

    codcel_log_norm_dist(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_norm_dist_basic() {
        // =LOGNORMDIST(4, 3, 1) in US format
        // =LOGNORMDIST(4; 3; 1) in German format
        let result = codcel_log_norm_dist(4.0, 3.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 0.05329564513556023).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_dist_small_x() {
        // =LOGNORMDIST(0.5, 3, 1) in US format
        // =LOGNORMDIST(0,5; 3; 1) in German format
        let result = codcel_log_norm_dist(0.5, 3.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 0.00011074787075693315).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_dist_large_x() {
        // =LOGNORMDIST(50, 3, 1) in US format
        // =LOGNORMDIST(50; 3; 1) in German format
        let result = codcel_log_norm_dist(50.0, 3.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 0.8191216963540108).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_dist_zero_x() {
        // =LOGNORMDIST(0, 3, 1) in US format
        // =LOGNORMDIST(0; 3; 1) in German format
        let result = codcel_log_norm_dist(0.0, 3.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_norm_dist_zero_std_dev() {
        // =LOGNORMDIST(4, 3, 0) in US format
        // =LOGNORMDIST(4; 3; 0) in German format
        let result = codcel_log_norm_dist(4.0, 3.0, 0.0);
        assert!(result.is_err());
    }
}
