// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `GAMMAINV`/`GAMMA.INV` function.
/// Returns the inverse cumulative gamma value.
/// - `probability`: cumulative probability value in `(0, 1)`.
/// - `alpha`: shape parameter (must be greater than 0).
/// - `beta`: scale parameter (must be greater than 0).
///
/// Returns an error on probabilities outside `(0, 1)` or non-positive parameters.
pub fn codcel_gamma_inv(
    probability: f64,
    alpha: f64,
    beta: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if probability <= 0.0 || probability >= 1.0 {
        return Err("GAMMAINV: Probability must be between 0 and 1 (exclusive).".into());
    }
    if alpha <= 0.0 {
        return Err("GAMMAINV: alpha must be greater than 0.".into());
    }
    if beta <= 0.0 {
        return Err("GAMMAINV: beta must be greater than 0.".into());
    }

    // Convert beta from scale to rate by taking its reciprocal
    let rate = 1.0 / beta;

    let gamma = statrs::distribution::Gamma::new(alpha, rate)
        .map_err(|_| "GAMMAINV: Failed to create gamma distribution")?;

    let result = gamma.inverse_cdf(probability);

    println!("result: {result:?}");

    Ok(result)
}

/// Convenience wrapper for `GAMMAINV` that accepts `[probability, alpha, beta]`.
/// Errors if the vector does not contain exactly three values.
pub fn codcel_gamma_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("GAMMAINV: Must have 3 parameters.".into());
    }

    let probability = inputs[0];
    let alpha = inputs[1];
    let beta = inputs[2];

    codcel_gamma_inv(probability, alpha, beta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_inv_basic() {
        // =GAMMAINV(0.5, 3, 2) in US format
        // =GAMMAINV(0,5; 3; 2) in German format
        let result = codcel_gamma_inv(0.5, 3.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 5.348120627447135).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_inv_small_probability() {
        // =GAMMAINV(0.1, 3, 2) in US format
        // =GAMMAINV(0,1; 3; 2) in German format
        let result = codcel_gamma_inv(0.1, 3.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 2.2041306564986454).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_inv_large_probability() {
        // =GAMMAINV(0.9, 3, 2) in US format
        // =GAMMAINV(0,9; 3; 2) in German format
        let result = codcel_gamma_inv(0.9, 3.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 10.644640675668409).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_inv_small_alpha() {
        // =GAMMAINV(0.5, 1, 2) in US format
        // =GAMMAINV(0,5; 1; 2) in German format
        let result = codcel_gamma_inv(0.5, 1.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 1.3863).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_inv_large_alpha() {
        // =GAMMAINV(0.5, 5, 2) in US format
        // =GAMMAINV(0,5; 5; 2) in German format
        let result = codcel_gamma_inv(0.5, 5.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 9.3418).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_inv_small_beta() {
        // =GAMMAINV(0.5, 3, 1) in US format
        // =GAMMAINV(0,5; 3; 1) in German format
        let result = codcel_gamma_inv(0.5, 3.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 2.6740603137235675).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_inv_large_beta() {
        // =GAMMAINV(0.5, 3, 5) in US format
        // =GAMMAINV(0,5; 3; 5) in German format
        let result = codcel_gamma_inv(0.5, 3.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 13.37030156861783).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_inv_probability_zero() {
        // Probability = 0 should return an error
        let result = codcel_gamma_inv(0.0, 3.0, 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_inv_probability_one() {
        // Probability = 1 should return an error
        let result = codcel_gamma_inv(1.0, 3.0, 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_inv_zero_alpha() {
        // Zero alpha should return an error
        let result = codcel_gamma_inv(0.5, 0.0, 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_inv_negative_alpha() {
        // Negative alpha should return an error
        let result = codcel_gamma_inv(0.5, -1.0, 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_inv_zero_beta() {
        // Zero beta should return an error
        let result = codcel_gamma_inv(0.5, 3.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_inv_negative_beta() {
        // Negative beta should return an error
        let result = codcel_gamma_inv(0.5, 3.0, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_inv_vec_basic() {
        // =GAMMAINV(0.5, 3, 2) in US format
        // =GAMMAINV(0,5; 3; 2) in German format
        let result = codcel_gamma_inv_vec(vec![0.5, 3.0, 2.0]).unwrap();
        println!("{result}");
        assert!((result - 5.348120627447135).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_inv_vec_wrong_params() {
        // Wrong number of parameters should return an error
        let result = codcel_gamma_inv_vec(vec![0.5, 3.0]);
        assert!(result.is_err());
    }
}
