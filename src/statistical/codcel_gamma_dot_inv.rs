// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compatibility::codcel_gamma_inv::{codcel_gamma_inv, codcel_gamma_inv_vec};
use std::error::Error;

/// Excel-compatible `GAMMA.INV` that returns the inverse of the gamma cumulative distribution.
/// - `probability`: the probability associated with the gamma distribution (0 to 1).
/// - `alpha`: the shape parameter (must be > 0).
/// - `beta`: the scale parameter (must be > 0).
///
/// Returns the value x such that GAMMA.DIST(x, alpha, beta, TRUE) = probability,
/// or an error when inputs are outside the allowed range.
/// This is equivalent to the older GAMMAINV function.
pub fn codcel_gamma_dot_inv(
    probability: f64,
    alpha: f64,
    beta: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Exactly the same as GAMMAINV
    codcel_gamma_inv(probability, alpha, beta)
}

pub fn codcel_gamma_dot_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Exactly the same as GAMMAINV
    codcel_gamma_inv_vec(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_dot_inv_basic() {
        // =GAMMA.INV(0.3233, 3, 2) in US format
        // =GAMMA.INV(0,3233; 3; 2) in German format
        let result = codcel_gamma_dot_inv(0.3233, 3.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 3.999825737853526).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dot_inv_small_probability() {
        // =GAMMA.INV(0.01, 3, 2) in US format
        // =GAMMA.INV(0,01; 3; 2) in German format
        let result = codcel_gamma_dot_inv(0.01, 3.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 0.8720903301565874).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dot_inv_large_probability() {
        // =GAMMA.INV(0.99, 3, 2) in US format
        // =GAMMA.INV(0,99; 3; 2) in German format
        let result = codcel_gamma_dot_inv(0.99, 3.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 16.81189382977093).abs() < 0.1);
    }

    #[test]
    fn test_gamma_dot_inv_different_alpha() {
        // =GAMMA.INV(0.5, 5, 2) in US format
        // =GAMMA.INV(0,5; 5; 2) in German format
        let result = codcel_gamma_dot_inv(0.5, 5.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 9.3).abs() < 0.1);
    }

    #[test]
    fn test_gamma_dot_inv_different_beta() {
        // =GAMMA.INV(0.5, 3, 4) in US format
        // =GAMMA.INV(0,5; 3; 4) in German format
        let result = codcel_gamma_dot_inv(0.5, 3.0, 4.0).unwrap();
        println!("{result}");
        assert!((result - 10.696241254894268).abs() < 0.1);
    }

    #[test]
    fn test_gamma_dot_inv_invalid_probability_low() {
        // Probability <= 0 should return an error
        let result = codcel_gamma_dot_inv(0.0, 3.0, 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dot_inv_invalid_probability_high() {
        // Probability >= 1 should return an error
        let result = codcel_gamma_dot_inv(1.0, 3.0, 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dot_inv_zero_alpha() {
        // Zero alpha should return an error
        let result = codcel_gamma_dot_inv(0.5, 0.0, 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dot_inv_zero_beta() {
        // Zero beta should return an error
        let result = codcel_gamma_dot_inv(0.5, 3.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dot_inv_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![0.5, 3.0];
        let result = codcel_gamma_dot_inv_vec(inputs);
        assert!(result.is_err());
    }
}
