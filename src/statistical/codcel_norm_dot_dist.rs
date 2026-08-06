// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compatibility::codcel_norm_dist::codcel_norm_dist;
use std::error::Error;

/// Excel-compatible `NORM.DIST` that returns the normal distribution.
/// - `x`: the value at which to evaluate the distribution.
/// - `mean`: the arithmetic mean of the distribution.
/// - `std_dev`: the standard deviation of the distribution (must be > 0).
/// - `cumulative`: if `true`, returns the cumulative distribution function (CDF);
///   if `false`, returns the probability density function (PDF).
///
/// Returns the distribution value or an error when inputs are outside the allowed range.
/// This is equivalent to the older NORMDIST function.
pub fn codcel_norm_dot_dist(
    x: f64,
    mean: f64,
    std_dev: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // NORM.DIST is exactly as NORMDIST
    codcel_norm_dist(x, mean, std_dev, cumulative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_dot_dist_pdf_standard_mean() {
        // =NORM.DIST(0, 0, 1, FALSE) in US format
        // =NORM.DIST(0; 0; 1; FALSE) in German format
        let result = codcel_norm_dot_dist(0.0, 0.0, 1.0, false).unwrap();
        assert!((result - 0.3989).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_pdf_standard_positive() {
        // =NORM.DIST(1, 0, 1, FALSE) in US format
        // =NORM.DIST(1; 0; 1; FALSE) in German format
        let result = codcel_norm_dot_dist(1.0, 0.0, 1.0, false).unwrap();
        assert!((result - 0.2420).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_pdf_standard_negative() {
        // =NORM.DIST(-1, 0, 1, FALSE) in US format
        // =NORM.DIST(-1; 0; 1; FALSE) in German format
        let result = codcel_norm_dot_dist(-1.0, 0.0, 1.0, false).unwrap();
        assert!((result - 0.2420).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_pdf_non_standard_mean() {
        // =NORM.DIST(10, 10, 1, FALSE) in US format
        // =NORM.DIST(10; 10; 1; FALSE) in German format
        let result = codcel_norm_dot_dist(10.0, 10.0, 1.0, false).unwrap();
        assert!((result - 0.3989).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_pdf_non_standard_std_dev() {
        // =NORM.DIST(0, 0, 2, FALSE) in US format
        // =NORM.DIST(0; 0; 2; FALSE) in German format
        let result = codcel_norm_dot_dist(0.0, 0.0, 2.0, false).unwrap();
        assert!((result - 0.1995).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_pdf_non_standard_both() {
        // =NORM.DIST(10, 8, 2, FALSE) in US format
        // =NORM.DIST(10; 8; 2; FALSE) in German format
        let result = codcel_norm_dot_dist(10.0, 8.0, 2.0, false).unwrap();
        assert!((result - 0.1210).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_cdf_standard_mean() {
        // =NORM.DIST(0, 0, 1, TRUE) in US format
        // =NORM.DIST(0; 0; 1; TRUE) in German format
        let result = codcel_norm_dot_dist(0.0, 0.0, 1.0, true).unwrap();
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_cdf_standard_positive() {
        // =NORM.DIST(1, 0, 1, TRUE) in US format
        // =NORM.DIST(1; 0; 1; TRUE) in German format
        let result = codcel_norm_dot_dist(1.0, 0.0, 1.0, true).unwrap();
        assert!((result - 0.8413).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_cdf_standard_negative() {
        // =NORM.DIST(-1, 0, 1, TRUE) in US format
        // =NORM.DIST(-1; 0; 1; TRUE) in German format
        let result = codcel_norm_dot_dist(-1.0, 0.0, 1.0, true).unwrap();
        assert!((result - 0.1587).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_cdf_non_standard_mean() {
        // =NORM.DIST(10, 10, 1, TRUE) in US format
        // =NORM.DIST(10; 10; 1; TRUE) in German format
        let result = codcel_norm_dot_dist(10.0, 10.0, 1.0, true).unwrap();
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_cdf_non_standard_std_dev() {
        // =NORM.DIST(1, 0, 2, TRUE) in US format
        // =NORM.DIST(1; 0; 2; TRUE) in German format
        let result = codcel_norm_dot_dist(1.0, 0.0, 2.0, true).unwrap();
        assert!((result - 0.6915).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_cdf_non_standard_both() {
        // =NORM.DIST(10, 8, 2, TRUE) in US format
        // =NORM.DIST(10; 8; 2; TRUE) in German format
        let result = codcel_norm_dot_dist(10.0, 8.0, 2.0, true).unwrap();
        assert!((result - 0.8413).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_dist_negative_std_dev() {
        // Negative standard deviation should return an error
        let result = codcel_norm_dot_dist(0.0, 0.0, -1.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_dist_zero_std_dev() {
        // Zero standard deviation should return an error
        let result = codcel_norm_dot_dist(0.0, 0.0, 0.0, true);
        assert!(result.is_err());
    }
}
