// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::{Beta, ContinuousCDF};
use std::error::Error;

/// Excel-compatible `BETA.INV` that returns the inverse of the cumulative beta distribution.
/// - `probability`: the probability for which to find the corresponding x value (0 to 1).
/// - `alpha` / `beta`: shape parameters that must be positive.
/// - `a` / `b`: optional lower/upper bounds for scaling the distribution (default 0–1).
///
/// Returns the x value such that BETA.DIST(x, alpha, beta, TRUE, a, b) = probability,
/// or an error when inputs are outside the allowed range.
pub fn codcel_beta_dot_inv(
    probability: f64,
    alpha: f64,
    beta: f64,
    a: Option<f64>,
    b: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Parameter validation
    if !(0.0..=1.0).contains(&probability) {
        return Err("BETA.INV: Probability must be between 0 and 1".into());
    }

    if alpha <= 0.0 {
        return Err("BETA.INV: Alpha parameter must be greater than 0".into());
    }

    if beta <= 0.0 {
        return Err("BETA.INV: Beta parameter must be greater than 0".into());
    }

    let lower_bound = a.unwrap_or(0.0);
    let upper_bound = b.unwrap_or(1.0);

    if lower_bound >= upper_bound {
        return Err("BETA.INV: Lower bound must be less than upper bound".into());
    }

    // Create a beta distribution with the given shape parameters
    let beta_dist = Beta::new(alpha, beta)?;

    // Calculate the inverse CDF (quantile function)
    let standard_result = beta_dist.inverse_cdf(probability);

    // Scale the result to the specified bounds [a, b]
    let scaled_result = lower_bound + (upper_bound - lower_bound) * standard_result;

    Ok(scaled_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beta_inv_basic() {
        // =BETA.INV(0.6875,2,3) in US format
        // =BETA.INV(0,6875;2;3) in German format
        let result = codcel_beta_dot_inv(0.6875, 2.0, 3.0, None, None).unwrap();
        assert!((result - 0.5).abs() < 1e-10); // Inverse of Beta(2,3) CDF at p=0.6875 is x=0.5
    }

    #[test]
    fn test_beta_inv_custom_bounds() {
        // =BETA.INV(0.6875,2,3,1,5) in US format
        // =BETA.INV(0,6875;2;3;1;5) in German format
        let result = codcel_beta_dot_inv(0.6875, 2.0, 3.0, Some(1.0), Some(5.0)).unwrap();
        assert!((result - 3.0).abs() < 1e-10); // Scaled from [0,1] to [1,5], so x=0.5 becomes x=3
    }

    #[test]
    fn test_beta_inv_symmetric() {
        // =BETA.INV(0.5,2,2) in US format
        // =BETA.INV(0,5;2;2) in German format
        let result = codcel_beta_dot_inv(0.5, 2.0, 2.0, None, None).unwrap();
        assert!((result - 0.5).abs() < 1e-10); // Inverse of symmetric Beta(2,2) CDF at p=0.5 is x=0.5
    }

    #[test]
    fn test_beta_inv_alpha_greater() {
        // =BETA.INV(0.87808,5,2) in US format
        // =BETA.INV(0,87808;5;2) in German format
        let result = codcel_beta_dot_inv(0.87808, 5.0, 2.0, None, None).unwrap();
        assert!((result - 0.896151303).abs() < 1e-5); // Inverse of Beta(5,2) CDF at p=0.87808 is x=0.7
    }

    #[test]
    fn test_beta_inv_beta_greater() {
        // =BETA.INV(0.52822,2,5) in US format
        // =BETA.INV(0,52822;2;5) in German format
        let result = codcel_beta_dot_inv(0.52822, 2.0, 5.0, None, None).unwrap();
        assert!((result - 0.276732496).abs() < 1e-5); // Inverse of Beta(2,5) CDF at p=0.52822 is x=0.3
    }

    #[test]
    fn test_beta_inv_boundary_p_zero() {
        // =BETA.INV(0,2,3) in US format
        // =BETA.INV(0;2;3) in German format
        let result = codcel_beta_dot_inv(0.0, 2.0, 3.0, None, None).unwrap();
        assert_eq!(result, 0.0); // Inverse of Beta(2,3) CDF at p=0 is x=0
    }

    #[test]
    fn test_beta_inv_boundary_p_one() {
        // =BETA.INV(1,2,3) in US format
        // =BETA.INV(1;2;3) in German format
        let result = codcel_beta_dot_inv(1.0, 2.0, 3.0, None, None).unwrap();
        assert_eq!(result, 1.0); // Inverse of Beta(2,3) CDF at p=1 is x=1
    }

    #[test]
    fn test_beta_inv_invalid_probability_low() {
        // =BETA.INV(-0.1,2,3) in US format (returns #NUM! error)
        // =BETA.INV(-0,1;2;3) in German format (returns #NUM! error)
        let result = codcel_beta_dot_inv(-0.1, 2.0, 3.0, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_beta_inv_invalid_probability_high() {
        // =BETA.INV(1.1,2,3) in US format (returns #NUM! error)
        // =BETA.INV(1,1;2;3) in German format (returns #NUM! error)
        let result = codcel_beta_dot_inv(1.1, 2.0, 3.0, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_beta_inv_invalid_alpha() {
        // =BETA.INV(0.5,0,3) in US format (returns #NUM! error)
        // =BETA.INV(0,5;0;3) in German format (returns #NUM! error)
        let result = codcel_beta_dot_inv(0.5, 0.0, 3.0, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_beta_inv_invalid_beta() {
        // =BETA.INV(0.5,2,-1) in US format (returns #NUM! error)
        // =BETA.INV(0,5;2;-1) in German format (returns #NUM! error)
        let result = codcel_beta_dot_inv(0.5, 2.0, -1.0, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_beta_inv_invalid_bounds() {
        // =BETA.INV(0.5,2,3,5,1) in US format (returns #NUM! error)
        // =BETA.INV(0,5;2;3;5;1) in German format (returns #NUM! error)
        let result = codcel_beta_dot_inv(0.5, 2.0, 3.0, Some(5.0), Some(1.0));
        assert!(result.is_err());
    }
}
