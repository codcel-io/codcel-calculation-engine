// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::standard_normal::{std_normal_cdf, std_normal_pdf};
use std::error::Error;

/// Excel-compatible `LOGNORM.DIST` that returns the lognormal distribution.
/// - `x`: the value at which to evaluate the distribution (must be > 0).
/// - `mean`: the mean of ln(x).
/// - `std_dev`: the standard deviation of ln(x) (must be > 0).
/// - `cumulative`: if `true`, returns the cumulative distribution function (CDF);
///   if `false`, returns the probability density function (PDF).
///
/// Returns the distribution value or an error when inputs are outside the allowed range.
pub fn codcel_log_norm_dot_dist(
    x: f64,
    mean: f64,
    std_dev: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if x <= 0.0 {
        return Err("x must be greater than 0".into());
    }
    if std_dev <= 0.0 {
        return Err("standard deviation must be greater than 0".into());
    }

    // ln(x) is normally distributed, so the lognormal is the standard normal in z.
    let z = (crate::portable_math::ln(x) - mean) / std_dev;

    if cumulative {
        Ok(std_normal_cdf(z))
    } else {
        // The Jacobian of the change of variable from ln(x) to x contributes the 1/(x * std_dev).
        Ok(std_normal_pdf(z) / (x * std_dev))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_norm_dot_dist_pdf_basic() {
        // =LOGNORM.DIST(4, 3.5, 1.2, FALSE) in US format
        // =LOGNORM.DIST(4; 3,5; 1,2; FALSE) in German format
        let result = codcel_log_norm_dot_dist(4.0, 3.5, 1.2, false).unwrap();
        println!("{result}");
        assert!((result - 0.017617596681819232).abs() < 1e-10);
    }

    #[test]
    fn test_log_norm_dot_dist_cdf_basic() {
        // =LOGNORM.DIST(4, 3.5, 1.2, TRUE) in US format
        // =LOGNORM.DIST(4; 3,5; 1,2; TRUE) in German format
        let result = codcel_log_norm_dot_dist(4.0, 3.5, 1.2, true).unwrap();
        println!("{result}");
        assert!((result - 0.03908355570841965).abs() < 1e-10);
    }

    #[test]
    fn test_log_norm_dot_dist_pdf_small_x() {
        // =LOGNORM.DIST(0.5, 3.5, 1.2, FALSE) in US format
        // =LOGNORM.DIST(0,5; 3,5; 1,2; FALSE) in German format
        let result = codcel_log_norm_dot_dist(0.5, 3.5, 1.2, false).unwrap();
        println!("{result}");
        assert!((result - 0.0014838105184792278).abs() < 1e-10);
    }

    #[test]
    fn test_log_norm_dot_dist_cdf_small_x() {
        // =LOGNORM.DIST(0.5, 3.5, 1.2, TRUE) in US format
        // =LOGNORM.DIST(0,5; 3,5; 1,2; TRUE) in German format
        let result = codcel_log_norm_dot_dist(0.5, 3.5, 1.2, true).unwrap();
        println!("{result}");
        assert!((result - 0.0002376628038792128).abs() < 1e-10);
    }

    #[test]
    fn test_log_norm_dot_dist_pdf_large_x() {
        // =LOGNORM.DIST(100, 3.5, 1.2, FALSE) in US format
        // =LOGNORM.DIST(100; 3,5; 1,2; FALSE) in German format
        let result = codcel_log_norm_dot_dist(100.0, 3.5, 1.2, false).unwrap();
        println!("{result}");
        assert!((result - 0.0021754322598185943).abs() < 1e-10);
    }

    #[test]
    fn test_log_norm_dot_dist_cdf_large_x() {
        // =LOGNORM.DIST(100, 3.5, 1.2, TRUE) in US format
        // =LOGNORM.DIST(100; 3,5; 1,2; TRUE) in German format
        let result = codcel_log_norm_dot_dist(100.0, 3.5, 1.2, true).unwrap();
        println!("{result}");
        assert!((result - 0.8214683007104536).abs() < 1e-10);
    }

    #[test]
    fn test_log_norm_dot_dist_zero_x() {
        // =LOGNORM.DIST(0, 3.5, 1.2, FALSE) in US format
        // =LOGNORM.DIST(0; 3,5; 1,2; FALSE) in German format
        let result = codcel_log_norm_dot_dist(0.0, 3.5, 1.2, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_norm_dot_dist_negative_x() {
        // =LOGNORM.DIST(-1, 3.5, 1.2, FALSE) in US format
        // =LOGNORM.DIST(-1; 3,5; 1,2; FALSE) in German format
        let result = codcel_log_norm_dot_dist(-1.0, 3.5, 1.2, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_norm_dot_dist_zero_std_dev() {
        // =LOGNORM.DIST(4, 3.5, 0, FALSE) in US format
        // =LOGNORM.DIST(4; 3,5; 0; FALSE) in German format
        let result = codcel_log_norm_dot_dist(4.0, 3.5, 0.0, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_norm_dot_dist_negative_std_dev() {
        // =LOGNORM.DIST(4, 3.5, -1.2, FALSE) in US format
        // =LOGNORM.DIST(4; 3,5; -1,2; FALSE) in German format
        let result = codcel_log_norm_dot_dist(4.0, 3.5, -1.2, false);
        assert!(result.is_err());
    }
}
