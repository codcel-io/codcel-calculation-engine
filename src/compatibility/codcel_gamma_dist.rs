// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `GAMMADIST`/`GAMMA.DIST` function.
/// Evaluates the gamma distribution.
/// - `x`: value at which to evaluate (must be non-negative).
/// - `alpha`: shape parameter (must be greater than 0).
/// - `beta`: scale parameter (must be greater than 0).
/// - `cumulative`: `true` for cumulative distribution (CDF), `false` for probability density (PDF).
///
/// Returns an error on negative `x` or non-positive parameters.
pub fn codcel_gamma_dist(
    x: f64,
    alpha: f64,
    beta: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x < 0.0 {
        return Err("GAMMADIST: x must be greater than or equal to 0.".into());
    }
    if alpha <= 0.0 {
        return Err("GAMMADIST: alpha must be greater than 0.".into());
    }
    if beta <= 0.0 {
        return Err("GAMMADIST: beta must be greater than 0.".into());
    }

    if cumulative {
        // Calculate the cumulative gamma distribution
        // Convert beta from scale to rate by taking its reciprocal
        let rate = 1.0 / beta;
        Ok(statrs::distribution::Gamma::new(alpha, rate)
            .map_err(|_| "GAMMADIST: Failed to create gamma distribution")?
            .cdf(x))
    } else {
        // Calculate the gamma probability density function
        let gamma_pdf = crate::portable_math::powf(x, alpha - 1.0) * crate::portable_math::exp(-x / beta)
            / (crate::portable_math::powf(beta, alpha) * statrs::function::gamma::gamma(alpha));
        Ok(gamma_pdf)
    }
}

/// Convenience wrapper for `GAMMADIST` that accepts `[x, alpha, beta, cumulative]`.
/// The last entry is treated as a boolean flag (non-zero => cumulative CDF).
pub fn codcel_gamma_dist_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 4 {
        return Err("GAMMADIST: Must have 4 parameters.".into());
    }

    let x = inputs[0];
    let alpha = inputs[1];
    let beta = inputs[2];
    let cumulative = inputs[3] != 0.0;

    codcel_gamma_dist(x, alpha, beta, cumulative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_dist_pdf_basic() {
        // =GAMMADIST(2, 3, 2, FALSE) in US format
        // =GAMMADIST(2; 3; 2; FALSE) in German format
        let result = codcel_gamma_dist(2.0, 3.0, 2.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.09196986029286028).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_cdf_basic() {
        // =GAMMADIST(2, 3, 2, TRUE) in US format
        // =GAMMADIST(2; 3; 2; TRUE) in German format
        let result = codcel_gamma_dist(2.0, 3.0, 2.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.080301397).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_pdf_zero_x() {
        // =GAMMADIST(0, 3, 2, FALSE) in US format
        // =GAMMADIST(0; 3; 2; FALSCH) in German format
        let result = codcel_gamma_dist(0.0, 3.0, 2.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_cdf_zero_x() {
        // =GAMMADIST(0, 3, 2, TRUE) in US format
        // =GAMMADIST(0; 3; 2; WAHR) in German format
        let result = codcel_gamma_dist(0.0, 3.0, 2.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_pdf_large_x() {
        // =GAMMADIST(10, 3, 2, FALSE) in US format
        // =GAMMADIST(10; 3; 2; FALSE) in German format
        let result = codcel_gamma_dist(10.0, 3.0, 2.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.04211216874428403).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_cdf_large_x() {
        // =GAMMADIST(10, 3, 2, TRUE) in US format
        // =GAMMADIST(10; 3; 2; TRUE) in German format
        let result = codcel_gamma_dist(10.0, 3.0, 2.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.875347981).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_pdf_small_alpha() {
        // =GAMMADIST(2, 1, 2, FALSE) in US format
        // =GAMMADIST(2; 1; 2; FALSCH) in German format
        let result = codcel_gamma_dist(2.0, 1.0, 2.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.1839397).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_cdf_small_alpha() {
        // =GAMMADIST(2, 1, 2, TRUE) in US format
        // =GAMMADIST(2; 1; 2; TRUE) in German format
        let result = codcel_gamma_dist(2.0, 1.0, 2.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.632120559).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_pdf_large_alpha() {
        // =GAMMADIST(2, 5, 2, FALSE) in US format
        // =GAMMADIST(2; 5; 2; FALSE) in German format
        let result = codcel_gamma_dist(2.0, 5.0, 2.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.0076641550244050125).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_cdf_large_alpha() {
        // =GAMMADIST(2, 5, 2, TRUE) in US format
        // =GAMMADIST(2; 5; 2; TRUE) in German format
        let result = codcel_gamma_dist(2.0, 5.0, 2.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.003659847).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_pdf_small_beta() {
        // =GAMMADIST(2, 3, 1, FALSE) in US format
        // =GAMMADIST(2; 3; 1; FALSCH) in German format
        let result = codcel_gamma_dist(2.0, 3.0, 1.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.2706705).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_cdf_small_beta() {
        // =GAMMADIST(2, 3, 1, TRUE) in US format
        // =GAMMADIST(2; 3; 1; TRUE) in German format
        let result = codcel_gamma_dist(2.0, 3.0, 1.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.3233235838169354).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_pdf_large_beta() {
        // =GAMMADIST(2, 3, 5, FALSE) in US format
        // =GAMMADIST(2; 3; 5; FALSE) in German format
        let result = codcel_gamma_dist(2.0, 3.0, 5.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.010725120736570193).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_cdf_large_beta() {
        // =GAMMADIST(2, 3, 5, TRUE) in US format
        // =GAMMADIST(2; 3; 5; TRUE) in German format
        let result = codcel_gamma_dist(2.0, 3.0, 5.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.007926332).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_negative_x() {
        // Negative x should return an error
        let result = codcel_gamma_dist(-1.0, 3.0, 2.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dist_zero_alpha() {
        // Zero alpha should return an error
        let result = codcel_gamma_dist(2.0, 0.0, 2.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dist_negative_alpha() {
        // Negative alpha should return an error
        let result = codcel_gamma_dist(2.0, -1.0, 2.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dist_zero_beta() {
        // Zero beta should return an error
        let result = codcel_gamma_dist(2.0, 3.0, 0.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dist_negative_beta() {
        // Negative beta should return an error
        let result = codcel_gamma_dist(2.0, 3.0, -1.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dist_vec_basic() {
        // =GAMMADIST(2, 3, 2, TRUE) in US format
        // =GAMMADIST(2; 3; 2; TRUE) in German format
        let result = codcel_gamma_dist_vec(vec![2.0, 3.0, 2.0, 1.0]).unwrap();
        println!("{result}");
        assert!((result - 0.080301397).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_dist_vec_wrong_params() {
        // Wrong number of parameters should return an error
        let result = codcel_gamma_dist_vec(vec![2.0, 3.0, 2.0]);
        assert!(result.is_err());
    }
}
