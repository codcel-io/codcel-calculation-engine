// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `LOGNORM.INV` that returns the inverse of the lognormal cumulative distribution.
/// - `p`: the probability associated with the lognormal distribution (0 to 1, exclusive).
/// - `mean`: the mean of ln(x).
/// - `std_dev`: the standard deviation of ln(x) (must be > 0).
///
/// Returns the value x such that LOGNORM.DIST(x, mean, std_dev, TRUE) = p,
/// or an error when inputs are outside the allowed range.
pub fn codcel_log_norm_inv(
    p: f64,
    mean: f64,
    std_dev: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if p <= 0.0 || p >= 1.0 {
        return Err("LOGNORM.INV: p must be between 0 and 1 (exclusive)".into());
    }
    if std_dev <= 0.0 {
        return Err("LOGNORM.INV: standard deviation must be greater than 0".into());
    }

    // Calculate the inverse of the log-normal cumulative distribution
    let z = statrs::distribution::Normal::new(0.0, 1.0)?.inverse_cdf(p);
    let result = (mean + z * std_dev).exp();

    Ok(result)
}

pub fn codcel_log_norm_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("LOGNORM.INV: : Must have 3 parameters.".into());
    }

    codcel_log_norm_inv(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_norm_inv_median() {
        // =LOGNORM.INV(0.5, 0, 1) in US format
        // =LOGNORM.INV(0,5; 0; 1) in German format
        let result = codcel_log_norm_inv(0.5, 0.0, 1.0).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_inv_basic() {
        // =LOGNORM.INV(0.75, 0, 1) in US format
        // =LOGNORM.INV(0,75; 0; 1) in German format
        let result = codcel_log_norm_inv(0.75, 0.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 1.9630310841582572).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_inv_small_probability() {
        // =LOGNORM.INV(0.1, 0, 1) in US format
        // =LOGNORM.INV(0,1; 0; 1) in German format
        let result = codcel_log_norm_inv(0.1, 0.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 0.27760624185200977).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_inv_large_probability() {
        // =LOGNORM.INV(0.9, 0, 1) in US format
        // =LOGNORM.INV(0,9; 0; 1) in German format
        let result = codcel_log_norm_inv(0.9, 0.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 3.602224479279158).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_inv_non_zero_mean() {
        // =LOGNORM.INV(0.5, 2, 1) in US format
        // =LOGNORM.INV(0,5; 2; 1) in German format
        let result = codcel_log_norm_inv(0.5, 2.0, 1.0).unwrap();
        assert!((result - 7.3891).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_inv_different_std_dev() {
        // =LOGNORM.INV(0.5, 0, 0.5) in US format
        // =LOGNORM.INV(0,5; 0; 0,5) in German format
        let result = codcel_log_norm_inv(0.5, 0.0, 0.5).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_inv_non_standard_params() {
        // =LOGNORM.INV(0.75, 1, 0.5) in US format
        // =LOGNORM.INV(0,75; 1; 0,5) in German format
        let result = codcel_log_norm_inv(0.75, 1.0, 0.5).unwrap();
        println!("{result}");
        assert!((result - 3.808536044832714).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_inv_zero_probability() {
        // =LOGNORM.INV(0, 0, 1) in US format
        // =LOGNORM.INV(0; 0; 1) in German format
        let result = codcel_log_norm_inv(0.0, 0.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_norm_inv_one_probability() {
        // =LOGNORM.INV(1, 0, 1) in US format
        // =LOGNORM.INV(1; 0; 1) in German format
        let result = codcel_log_norm_inv(1.0, 0.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_norm_inv_negative_std_dev() {
        // Negative standard deviation should return an error
        let result = codcel_log_norm_inv(0.5, 0.0, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_norm_inv_zero_std_dev() {
        // Zero standard deviation should return an error
        let result = codcel_log_norm_inv(0.5, 0.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_norm_inv_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![0.5, 0.0, 1.0];
        let result = codcel_log_norm_inv_vec(inputs).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_log_norm_inv_vec_invalid() {
        // Test the vector version with invalid inputs (wrong number of parameters)
        let inputs = vec![0.5, 0.0];
        let result = codcel_log_norm_inv_vec(inputs);
        assert!(result.is_err());
    }
}
