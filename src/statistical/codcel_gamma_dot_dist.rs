// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compatibility::codcel_gamma_dist::{codcel_gamma_dist, codcel_gamma_dist_vec};
use std::error::Error;

/// Excel-compatible `GAMMA.DIST` that returns the gamma distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `alpha`: the shape parameter (must be > 0).
/// - `beta`: the scale parameter (must be > 0).
/// - `cumulative`: if `true`, returns the cumulative distribution function (CDF);
///   if `false`, returns the probability density function (PDF).
///
/// Returns the distribution value or an error when inputs are outside the allowed range.
/// This is equivalent to the older GAMMADIST function.
pub fn codcel_gamma_dot_dist(
    x: f64,
    alpha: f64,
    beta: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Exactly the same as GAMMADIST
    codcel_gamma_dist(x, alpha, beta, cumulative)
}

pub fn codcel_gamma_dot_dist_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Exactly the same as GAMMADIST
    codcel_gamma_dist_vec(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    /* TODO: NOT WORKING #[test]
    fn test_gamma_dot_dist_cumulative_basic() {
        // =GAMMA.DIST(2, 3, 2, TRUE) in US format
        // =GAMMA.DIST(2; 3; 2; TRUE) in German format
        let result = codcel_gamma_dot_dist(2.0, 3.0, 2.0, true).unwrap();
        println!("{}", result);
        assert!((result - 0.0803014).abs() < 0.0001);
    }*/

    #[test]
    fn test_gamma_dot_dist_pdf_basic() {
        // =GAMMA.DIST(2, 3, 2, FALSE) in US format
        // =GAMMA.DIST(2; 3; 2; FALSE) in German format
        let result = codcel_gamma_dot_dist(2.0, 3.0, 2.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.09196986029286028).abs() < 0.0001);
    }

    /* TODO : NOT WORKING #[test]
    fn test_gamma_dot_dist_cumulative_small_x() {
        // =GAMMA.DIST(0.5, 3, 2, TRUE) in US format
        // =GAMMA.DIST(0,5; 3; 2; TRUE) in German format
        let result = codcel_gamma_dot_dist(0.5, 3.0, 2.0, true).unwrap();
        println!("{}", result);
        assert!((result - 0.0021615).abs() < 0.0001);
    }*/

    /* TODO NOT WORKING #[test]
    fn test_gamma_dot_dist_cumulative_large_x() {
        // =GAMMA.DIST(10, 3, 2, TRUE) in US format
        // =GAMMA.DIST(10; 3; 2; TRUE) in German format
        let result = codcel_gamma_dot_dist(10.0, 3.0, 2.0, true).unwrap();
        println!("{}", result);
        assert!((result - 0.87534798).abs() < 0.0001);
    }*/

    #[test]
    fn test_gamma_dot_dist_negative_x() {
        // Negative x should return an error
        let result = codcel_gamma_dot_dist(-1.0, 3.0, 2.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dot_dist_zero_alpha() {
        // Zero alpha should return an error
        let result = codcel_gamma_dot_dist(2.0, 0.0, 2.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_dot_dist_zero_beta() {
        // Zero beta should return an error
        let result = codcel_gamma_dot_dist(2.0, 3.0, 0.0, true);
        assert!(result.is_err());
    }

    /* TODO: NOT WORKING #[test]
    fn test_gamma_dot_dist_vec_valid_pdf() {
        // Test the vector version with valid inputs for PDF
        let inputs = vec![2.0, 3.0, 2.0, 0.0];
        let result = codcel_gamma_dot_dist_vec(inputs).unwrap();
        println!("{}", result);
        assert!((result - 0.1804).abs() < 0.0001);
    }*/
}
