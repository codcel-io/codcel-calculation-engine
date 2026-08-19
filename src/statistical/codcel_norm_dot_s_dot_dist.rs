// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::standard_normal::{std_normal_cdf, std_normal_pdf};
use std::error::Error;

/// Excel-compatible `NORM.S.DIST` that returns the standard normal distribution.
/// - `z`: the value at which to evaluate the distribution (z-score).
/// - `cumulative`: if `true`, returns the cumulative distribution function (CDF);
///   if `false`, returns the probability density function (PDF).
///
/// Returns the distribution value for a standard normal distribution (mean=0, std_dev=1).
pub fn codcel_norm_dot_s_dot_dist(
    z: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if !z.is_finite() {
        return Err("NORM.S.DIST: Input must be a finite number.".into());
    }

    if cumulative {
        Ok(std_normal_cdf(z))
    } else {
        Ok(std_normal_pdf(z))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values were computed with mpmath at 60 decimal digits and rounded to the nearest
    // f64, so they pin the full precision Excel reports rather than the four displayed decimals.

    #[test]
    fn test_norm_dot_s_dot_dist_pdf_zero() {
        // =NORM.S.DIST(0, FALSE) in US format
        // =NORM.S.DIST(0; FALSE) in German format
        let result = codcel_norm_dot_s_dot_dist(0.0, false).unwrap();
        assert!((result - 0.3989422804014327).abs() < 1e-16);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_pdf_positive() {
        // =NORM.S.DIST(1, FALSE) in US format
        // =NORM.S.DIST(1; FALSE) in German format
        let result = codcel_norm_dot_s_dot_dist(1.0, false).unwrap();
        assert!((result - 0.24197072451914334).abs() < 1e-16);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_pdf_negative() {
        // =NORM.S.DIST(-1, FALSE) in US format
        // =NORM.S.DIST(-1; FALSE) in German format
        let result = codcel_norm_dot_s_dot_dist(-1.0, false).unwrap();
        assert!((result - 0.24197072451914334).abs() < 1e-16);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_pdf_large_positive() {
        // =NORM.S.DIST(3, FALSE) in US format
        // =NORM.S.DIST(3; FALSE) in German format
        let result = codcel_norm_dot_s_dot_dist(3.0, false).unwrap();
        assert!((result - 0.0044318484119380075).abs() < 1e-17);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_pdf_large_negative() {
        // =NORM.S.DIST(-3, FALSE) in US format
        // =NORM.S.DIST(-3; FALSE) in German format
        let result = codcel_norm_dot_s_dot_dist(-3.0, false).unwrap();
        assert!((result - 0.0044318484119380075).abs() < 1e-17);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_zero() {
        // =NORM.S.DIST(0, TRUE) in US format
        // =NORM.S.DIST(0; TRUE) in German format
        let result = codcel_norm_dot_s_dot_dist(0.0, true).unwrap();
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_positive() {
        // =NORM.S.DIST(1, TRUE) in US format
        // =NORM.S.DIST(1; TRUE) in German format
        let result = codcel_norm_dot_s_dot_dist(1.0, true).unwrap();
        assert!((result - 0.8413447460685429).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_negative() {
        // =NORM.S.DIST(-1, TRUE) in US format
        // =NORM.S.DIST(-1; TRUE) in German format
        let result = codcel_norm_dot_s_dot_dist(-1.0, true).unwrap();
        assert!((result - 0.15865525393145705).abs() < 1e-16);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_large_positive() {
        // =NORM.S.DIST(3, TRUE) in US format
        // =NORM.S.DIST(3; TRUE) in German format
        let result = codcel_norm_dot_s_dot_dist(3.0, true).unwrap();
        assert!((result - 0.9986501019683699).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_large_negative() {
        // =NORM.S.DIST(-3, TRUE) in US format
        // =NORM.S.DIST(-3; TRUE) in German format
        let result = codcel_norm_dot_s_dot_dist(-3.0, true).unwrap();
        assert!((result - 0.0013498980316300946).abs() < 1e-17);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_far_left_tail() {
        // =NORM.S.DIST(-8, TRUE) in US format
        // =NORM.S.DIST(-8; TRUE) in German format
        //
        // The Abramowitz & Stegun 26.2.17 approximation this function used to carry has an
        // absolute error bound of 7.5e-8, which says nothing at all about values this small: it
        // returned exactly 0.0 from about z = -9 outward. These cases exist to keep it that way.
        for (z, expected) in [
            (-6.0, 9.86587645037698e-10),
            (-8.0, 6.220960574271784e-16),
            (-10.0, 7.619853024160525e-24),
        ] {
            let result = codcel_norm_dot_s_dot_dist(z, true).unwrap();
            assert!(
                ((result - expected) / expected).abs() < 1e-13,
                "NORM.S.DIST({z}, TRUE) = {result}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_norm_dot_s_dot_dist_non_finite() {
        // Non-finite input should return an error
        let result = codcel_norm_dot_s_dot_dist(f64::INFINITY, true);
        assert!(result.is_err());
    }
}
