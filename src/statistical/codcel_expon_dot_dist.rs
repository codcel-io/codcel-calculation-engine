// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `EXPON.DIST` that returns the exponential distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `lambda`: the rate parameter (must be > 0).
/// - `cumulative`: if `true`, returns the cumulative distribution function (CDF);
///   if `false`, returns the probability density function (PDF).
///
/// Returns the distribution value or an error when inputs are outside the allowed range.
pub fn codcel_expon_dot_dist(
    x: f64,
    lambda: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if x < 0.0 {
        return Err("EXPON.DIST: x must be non-negative.".into());
    }
    if lambda <= 0.0 {
        return Err("EXPON.DIST: Lambda must be greater than 0.".into());
    }

    if cumulative {
        // Calculate the cumulative distribution function (CDF)
        Ok(1.0 - (-lambda * x).exp())
    } else {
        // Calculate the probability density function (PDF)
        Ok(lambda * (-lambda * x).exp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expon_dot_dist_pdf_basic() {
        // =EXPON.DIST(2, 0.5, FALSE) in US format
        // =EXPON.DIST(2; 0,5; FALSCH) in German format
        let result = codcel_expon_dot_dist(2.0, 0.5, false).unwrap();
        assert!((result - 0.1839397).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dot_dist_cdf_basic() {
        // =EXPON.DIST(2, 0.5, TRUE) in US format
        // =EXPON.DIST(2; 0,5; WAHR) in German format
        let result = codcel_expon_dot_dist(2.0, 0.5, true).unwrap();
        assert!((result - 0.6321206).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dot_dist_pdf_zero_x() {
        // =EXPON.DIST(0, 0.5, FALSE) in US format
        // =EXPON.DIST(0; 0,5; FALSCH) in German format
        let result = codcel_expon_dot_dist(0.0, 0.5, false).unwrap();
        assert!((result - 0.5).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dot_dist_cdf_zero_x() {
        // =EXPON.DIST(0, 0.5, TRUE) in US format
        // =EXPON.DIST(0; 0,5; WAHR) in German format
        let result = codcel_expon_dot_dist(0.0, 0.5, true).unwrap();
        assert!((result - 0.0).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dot_dist_pdf_large_x() {
        // =EXPON.DIST(10, 0.5, FALSE) in US format
        // =EXPON.DIST(10; 0,5; FALSCH) in German format
        let result = codcel_expon_dot_dist(10.0, 0.5, false).unwrap();
        println!("{result}");
        assert!((result - 0.003369).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dot_dist_cdf_large_x() {
        // =EXPON.DIST(10, 0.5, TRUE) in US format
        // =EXPON.DIST(10; 0,5; WAHR) in German format
        let result = codcel_expon_dot_dist(10.0, 0.5, true).unwrap();
        assert!((result - 0.9932621).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dot_dist_pdf_large_lambda() {
        // =EXPON.DIST(2, 5, FALSE) in US format
        // =EXPON.DIST(2; 5; FALSCH) in German format
        let result = codcel_expon_dot_dist(2.0, 5.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.00022699964881242428).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dot_dist_cdf_large_lambda() {
        // =EXPON.DIST(2, 5, TRUE) in US format
        // =EXPON.DIST(2; 5; WAHR) in German format
        let result = codcel_expon_dot_dist(2.0, 5.0, true).unwrap();
        assert!((result - 0.9999546).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dot_dist_negative_x() {
        // Negative x should return an error
        let result = codcel_expon_dot_dist(-1.0, 0.5, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_expon_dot_dist_zero_lambda() {
        // Zero lambda should return an error
        let result = codcel_expon_dot_dist(2.0, 0.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_expon_dot_dist_negative_lambda() {
        // Negative lambda should return an error
        let result = codcel_expon_dot_dist(2.0, -0.5, true);
        assert!(result.is_err());
    }
}
