// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;
use std::f64::consts::PI;

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
        // Calculate the cumulative distribution function (CDF)
        const P: f64 = 0.2316419;
        const B1: f64 = 0.319381530;
        const B2: f64 = -0.356563782;
        const B3: f64 = 1.781477937;
        const B4: f64 = -1.821255978;
        const B5: f64 = 1.330274429;

        let x = if z < 0.0 { -z } else { z };

        let pdf = (-x * x / 2.0).exp() / (2.0 * PI).sqrt();
        let t = 1.0 / (1.0 + P * x);
        let poly = t * (B1 + t * (B2 + t * (B3 + t * (B4 + t * B5))));
        let cdf = 1.0 - pdf * poly;

        let result = if z < 0.0 { 1.0 - cdf } else { cdf };
        Ok(result)
    } else {
        // Calculate the probability density function (PDF)
        let pdf = (-z * z / 2.0).exp() / (2.0 * PI).sqrt();
        Ok(pdf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_dot_s_dot_dist_pdf_zero() {
        // =NORM.S.DIST(0, FALSE) in US format
        // =NORM.S.DIST(0; FALSE) in German format
        let result = codcel_norm_dot_s_dot_dist(0.0, false).unwrap();
        assert!((result - 0.3989).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_pdf_positive() {
        // =NORM.S.DIST(1, FALSE) in US format
        // =NORM.S.DIST(1; FALSE) in German format
        let result = codcel_norm_dot_s_dot_dist(1.0, false).unwrap();
        assert!((result - 0.2420).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_pdf_negative() {
        // =NORM.S.DIST(-1, FALSE) in US format
        // =NORM.S.DIST(-1; FALSE) in German format
        let result = codcel_norm_dot_s_dot_dist(-1.0, false).unwrap();
        assert!((result - 0.2420).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_pdf_large_positive() {
        // =NORM.S.DIST(3, FALSE) in US format
        // =NORM.S.DIST(3; FALSE) in German format
        let result = codcel_norm_dot_s_dot_dist(3.0, false).unwrap();
        assert!((result - 0.0044).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_pdf_large_negative() {
        // =NORM.S.DIST(-3, FALSE) in US format
        // =NORM.S.DIST(-3; FALSE) in German format
        let result = codcel_norm_dot_s_dot_dist(-3.0, false).unwrap();
        assert!((result - 0.0044).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_zero() {
        // =NORM.S.DIST(0, TRUE) in US format
        // =NORM.S.DIST(0; TRUE) in German format
        let result = codcel_norm_dot_s_dot_dist(0.0, true).unwrap();
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_positive() {
        // =NORM.S.DIST(1, TRUE) in US format
        // =NORM.S.DIST(1; TRUE) in German format
        let result = codcel_norm_dot_s_dot_dist(1.0, true).unwrap();
        assert!((result - 0.8413).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_negative() {
        // =NORM.S.DIST(-1, TRUE) in US format
        // =NORM.S.DIST(-1; TRUE) in German format
        let result = codcel_norm_dot_s_dot_dist(-1.0, true).unwrap();
        assert!((result - 0.1587).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_large_positive() {
        // =NORM.S.DIST(3, TRUE) in US format
        // =NORM.S.DIST(3; TRUE) in German format
        let result = codcel_norm_dot_s_dot_dist(3.0, true).unwrap();
        assert!((result - 0.9987).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_cdf_large_negative() {
        // =NORM.S.DIST(-3, TRUE) in US format
        // =NORM.S.DIST(-3; TRUE) in German format
        let result = codcel_norm_dot_s_dot_dist(-3.0, true).unwrap();
        assert!((result - 0.0013).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_dist_non_finite() {
        // Non-finite input should return an error
        let result = codcel_norm_dot_s_dot_dist(f64::INFINITY, true);
        assert!(result.is_err());
    }
}
