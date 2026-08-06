// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `T.INV.2T` that returns the inverse of the two-tailed Student's t-distribution.
/// - `probability`: the two-tailed probability associated with the t-distribution (0 to 1, exclusive).
/// - `degrees_freedom`: degrees of freedom (must be > 0).
///
/// Returns the positive t-value such that the two-tailed probability equals the input probability,
/// or an error when inputs are outside the allowed range.
pub fn codcel_t_inv_2t(
    probability: f64,
    degrees_freedom: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if probability <= 0.0 || probability >= 1.0 {
        return Err("T.INV.2T: Probability must be between 0 and 1 (exclusive).".into());
    }
    if degrees_freedom <= 0.0 {
        return Err("T.INV.2T: Degrees of freedom must be greater than 0.".into());
    }

    // For two-tailed, split the probability into one tail
    let one_tailed_p = 1.0 - probability / 2.0;

    // Create a t-distribution with specified degrees of freedom
    let t_distribution = statrs::distribution::StudentsT::new(0.0, 1.0, degrees_freedom)?;

    // Calculate the inverse cumulative distribution function (inverse CDF or quantile)
    let t_inv = t_distribution.inverse_cdf(one_tailed_p);

    Ok(t_inv)
}

pub fn codcel_t_inv_2t_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("T.INV.2T: Must have 2 parameters.".into());
    }

    codcel_t_inv_2t(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_inv_2t_basic() {
        // =T.INV.2T(0.05, 10) in US format
        // =T.INV.2T(0,05; 10) in German format
        let result = codcel_t_inv_2t(0.05, 10.0).unwrap();
        println!("{result}");
        assert!((result - 2.2281388519649).abs() < 0.0001);
    }

    #[test]
    fn test_t_inv_2t_different_probability() {
        // =T.INV.2T(0.1, 10) in US format
        // =T.INV.2T(0,1; 10) in German format
        let result = codcel_t_inv_2t(0.1, 10.0).unwrap();
        println!("{result}");
        assert!((result - 1.8124611228107).abs() < 0.0001);
    }

    #[test]
    fn test_t_inv_2t_different_degrees() {
        // =T.INV.2T(0.05, 20) in US format
        // =T.INV.2T(0,05; 20) in German format
        let result = codcel_t_inv_2t(0.05, 20.0).unwrap();
        println!("{result}");
        assert!((result - 2.0859634472658).abs() < 0.0001);
    }

    #[test]
    fn test_t_inv_2t_large_degrees() {
        // =T.INV.2T(0.05, 100) in US format
        // =T.INV.2T(0,05; 100) in German format
        let result = codcel_t_inv_2t(0.05, 100.0).unwrap();
        println!("{result}");
        assert!((result - 1.9839792323667).abs() < 0.0001);
    }

    #[test]
    fn test_t_inv_2t_small_probability() {
        // =T.INV.2T(0.01, 10) in US format
        // =T.INV.2T(0,01; 10) in German format
        let result = codcel_t_inv_2t(0.01, 10.0).unwrap();
        println!("{result}");
        assert!((result - 3.1692677795869).abs() < 0.0001);
    }

    #[test]
    fn test_t_inv_2t_invalid_probability_zero() {
        // Probability of 0 should return an error
        let result = codcel_t_inv_2t(0.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_inv_2t_invalid_probability_one() {
        // Probability of 1 should return an error
        let result = codcel_t_inv_2t(1.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_inv_2t_invalid_degrees_zero() {
        // Degrees of freedom of 0 should return an error
        let result = codcel_t_inv_2t(0.05, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_inv_2t_invalid_degrees_negative() {
        // Negative degrees of freedom should return an error
        let result = codcel_t_inv_2t(0.05, -1.0);
        assert!(result.is_err());
    }
}
