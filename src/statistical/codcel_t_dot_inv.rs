// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `T.INV` that returns the inverse of the left-tailed Student's t-distribution.
/// - `probability`: the probability associated with the t-distribution (0 to 1, exclusive).
/// - `degrees_freedom`: degrees of freedom (must be > 0).
///
/// Returns the t-value such that T.DIST(t, df, TRUE) = probability,
/// or an error when inputs are outside the allowed range.
pub fn codcel_t_dot_inv(
    probability: f64,
    degrees_freedom: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if probability <= 0.0 || probability >= 1.0 {
        return Err("T.INV: Probability must be between 0 and 1 (exclusive).".into());
    }
    if degrees_freedom <= 0.0 {
        return Err("T.INV: Degrees of freedom must be greater than 0.".into());
    }

    // Create a t-distribution with specified degrees of freedom
    let t_distribution = statrs::distribution::StudentsT::new(0.0, 1.0, degrees_freedom)?;

    // Calculate the inverse cumulative distribution function (inverse CDF or quantile)
    let t_inv = t_distribution.inverse_cdf(probability);

    Ok(t_inv)
}

pub fn codcel_t_dot_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("T.INV: Must have 2 parameters.".into());
    }

    codcel_t_dot_inv(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_dot_inv_basic() {
        // =T.INV(0.9, 10) in US format
        // =T.INV(0,9; 10) in German format
        let result = codcel_t_dot_inv(0.9, 10.0).unwrap();
        assert!((result - 1.3722).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_inv_large_df() {
        // =T.INV(0.975, 1000) in US format
        // =T.INV(0,975; 1000) in German format
        let result = codcel_t_dot_inv(0.975, 1000.0).unwrap();
        assert!((result - 1.9623).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_inv_small_df() {
        // =T.INV(0.75, 1) in US format
        // =T.INV(0,75; 1) in German format
        let result = codcel_t_dot_inv(0.75, 1.0).unwrap();
        assert!((result - 1.0000).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_inv_half_probability() {
        // =T.INV(0.5, 5) in US format
        // =T.INV(0,5; 5) in German format
        let result = codcel_t_dot_inv(0.5, 5.0).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_inv_small_probability() {
        // =T.INV(0.05, 5) in US format
        // =T.INV(0,05; 5) in German format
        let result = codcel_t_dot_inv(0.05, 5.0).unwrap();
        assert!((result - (-2.0150)).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_inv_large_probability() {
        // =T.INV(0.95, 5) in US format
        // =T.INV(0,95; 5) in German format
        let result = codcel_t_dot_inv(0.95, 5.0).unwrap();
        assert!((result - 2.0150).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_inv_zero_probability() {
        // Zero probability should return an error
        let result = codcel_t_dot_inv(0.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dot_inv_one_probability() {
        // Probability of 1 should return an error
        let result = codcel_t_dot_inv(1.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dot_inv_negative_probability() {
        // Negative probability should return an error
        let result = codcel_t_dot_inv(-0.5, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dot_inv_zero_df() {
        // Zero degrees of freedom should return an error
        let result = codcel_t_dot_inv(0.5, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dot_inv_negative_df() {
        // Negative degrees of freedom should return an error
        let result = codcel_t_dot_inv(0.5, -5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dot_inv_vec_basic() {
        // =T.INV(0.9, 10) in US format
        // =T.INV(0,9; 10) in German format
        let inputs = vec![0.9, 10.0];
        let result = codcel_t_dot_inv_vec(inputs).unwrap();
        assert!((result - 1.3722).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_inv_vec_wrong_length() {
        // Wrong number of parameters should return an error
        let inputs = vec![0.9, 10.0, 5.0];
        let result = codcel_t_dot_inv_vec(inputs);
        assert!(result.is_err());
    }
}
