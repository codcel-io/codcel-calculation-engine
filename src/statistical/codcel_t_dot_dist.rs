// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::{Continuous, ContinuousCDF};
use std::error::Error;

/// Excel-compatible `T.DIST` that returns the Student's t-distribution.
/// - `x`: the value at which to evaluate the distribution.
/// - `degrees_freedom`: degrees of freedom (must be > 0).
/// - `cumulative`: if `true`, returns the cumulative distribution function (CDF);
///   if `false`, returns the probability density function (PDF).
///
/// Returns the distribution value or an error when inputs are outside the allowed range.
pub fn codcel_t_dot_dist(
    x: f64,
    degrees_freedom: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if degrees_freedom <= 0.0 {
        return Err("T.DIST: Degrees of freedom must be greater than 0.".into());
    }

    if cumulative {
        // Create a t-distribution
        let t_distribution = statrs::distribution::StudentsT::new(0.0, 1.0, degrees_freedom)?;
        // Return the cumulative distribution function (CDF)
        Ok(t_distribution.cdf(x))
    } else {
        // Create a t-distribution
        let t_distribution = statrs::distribution::StudentsT::new(0.0, 1.0, degrees_freedom)?;
        // Return the probability density function (PDF)
        Ok(t_distribution.pdf(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_dot_dist_pdf_basic() {
        // =T.DIST(2, 10, FALSE) in US format
        // =T.DIST(2; 10; FALSE) in German format
        let result = codcel_t_dot_dist(2.0, 10.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.061145766321218174).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_dist_cdf_basic() {
        // =T.DIST(2, 10, TRUE) in US format
        // =T.DIST(2; 10; TRUE) in German format
        let result = codcel_t_dot_dist(2.0, 10.0, true).unwrap();
        assert!((result - 0.9634).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_dist_pdf_negative_x() {
        // =T.DIST(-1, 5, FALSE) in US format
        // =T.DIST(-1; 5; FALSE) in German format
        let result = codcel_t_dot_dist(-1.0, 5.0, false).unwrap();
        assert!((result - 0.2196).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_dist_cdf_negative_x() {
        // =T.DIST(-1, 5, TRUE) in US format
        // =T.DIST(-1; 5; TRUE) in German format
        let result = codcel_t_dot_dist(-1.0, 5.0, true).unwrap();
        assert!((result - 0.1816).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_dist_pdf_zero_x() {
        // =T.DIST(0, 5, FALSE) in US format
        // =T.DIST(0; 5; FALSE) in German format
        let result = codcel_t_dot_dist(0.0, 5.0, false).unwrap();
        assert!((result - 0.3796).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_dist_cdf_zero_x() {
        // =T.DIST(0, 5, TRUE) in US format
        // =T.DIST(0; 5; TRUE) in German format
        let result = codcel_t_dot_dist(0.0, 5.0, true).unwrap();
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_dist_pdf_large_df() {
        // =T.DIST(1.96, 1000, FALSE) in US format
        // =T.DIST(1,96; 1000; FALSE) in German format
        let result = codcel_t_dot_dist(1.96, 1000.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.058529428361501386).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_dist_cdf_large_df() {
        // =T.DIST(1.96, 1000, TRUE) in US format
        // =T.DIST(1,96; 1000; TRUE) in German format
        let result = codcel_t_dot_dist(1.96, 1000.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.9748634075221222).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_dist_zero_df() {
        // Zero degrees of freedom should return an error
        let result = codcel_t_dot_dist(2.0, 0.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dot_dist_negative_df() {
        // Negative degrees of freedom should return an error
        let result = codcel_t_dot_dist(2.0, -5.0, true);
        assert!(result.is_err());
    }
}
