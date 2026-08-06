// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `TINV`/`T.INV.2T` function.
/// Returns the two-tailed inverse of the Student's t distribution.
/// - `probability`: two-tailed probability value in `(0, 1)`.
/// - `degrees_freedom`: degrees of freedom (must be greater than 0).
///
/// Returns an error when the probability is outside `(0, 1)` or degrees of freedom are non-positive.
pub fn codcel_t_inv(
    probability: f64,
    degrees_freedom: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if probability <= 0.0 || probability >= 1.0 {
        return Err("TINV: Probability must be between 0 and 1 (exclusive).".into());
    }
    if degrees_freedom <= 0.0 {
        return Err("TINV: Degrees of freedom must be greater than 0.".into());
    }

    // One-tailed probability
    let one_tailed_p = 1.0 - probability / 2.0;

    // Create a t-distribution
    let t_distribution = statrs::distribution::StudentsT::new(0.0, 1.0, degrees_freedom)?;

    // Calculate the inverse cumulative distribution function (inverse CDF or quantile)
    let t_inv = t_distribution.inverse_cdf(one_tailed_p);

    Ok(t_inv)
}

/// Convenience wrapper for `TINV` that accepts `[probability, degrees_freedom]`.
/// Errors if the vector does not contain exactly two values.
pub fn codcel_t_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("TINV: Must have 2 parameters.".into());
    }

    codcel_t_inv(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_inv_basic() {
        // =TINV(0.05, 10) in US format
        // =TINV(0,05; 10) in German format
        let result = codcel_t_inv(0.05, 10.0).unwrap();
        println!("{result}");
        assert!((result - 2.2281).abs() < 0.001);
    }

    #[test]
    fn test_t_inv_different_df() {
        // =TINV(0.05, 20) in US format
        // =TINV(0,05; 20) in German format
        let result = codcel_t_inv(0.05, 20.0).unwrap();
        println!("{result}");
        assert!((result - 2.0860).abs() < 0.001);
    }

    #[test]
    fn test_t_inv_different_probability() {
        // =TINV(0.01, 10) in US format
        // =TINV(0,01; 10) in German format
        let result = codcel_t_inv(0.01, 10.0).unwrap();
        println!("{result}");
        assert!((result - 3.1693).abs() < 0.001);
    }

    #[test]
    fn test_t_inv_median() {
        // =TINV(0.5, 10) in US format
        // =TINV(0,5; 10) in German format
        let result = codcel_t_inv(0.5, 10.0).unwrap();
        println!("{result}");
        assert!((result - 0.6998120613124311).abs() < 0.001);
    }

    #[test]
    fn test_t_inv_large_df() {
        // =TINV(0.05, 100) in US format
        // =TINV(0,05; 100) in German format
        let result = codcel_t_inv(0.05, 100.0).unwrap();
        println!("{result}");
        assert!((result - 1.9840).abs() < 0.001);
    }

    #[test]
    fn test_t_inv_zero_probability() {
        // =TINV(0, 10) in US format
        // =TINV(0; 10) in German format
        let result = codcel_t_inv(0.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_inv_one_probability() {
        // =TINV(1, 10) in US format
        // =TINV(1; 10) in German format
        let result = codcel_t_inv(1.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_inv_zero_df() {
        // =TINV(0.05, 0) in US format
        // =TINV(0,05; 0) in German format
        let result = codcel_t_inv(0.05, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_inv_vec_valid() {
        // Test the vector version with valid input
        let inputs = vec![0.05, 10.0];
        let result = codcel_t_inv_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 2.2281).abs() < 0.001);
    }

    #[test]
    fn test_t_inv_vec_invalid() {
        // Test the vector version with invalid input (too many parameters)
        let inputs = vec![0.05, 10.0, 2.0];
        let result = codcel_t_inv_vec(inputs);
        assert!(result.is_err());
    }
}
