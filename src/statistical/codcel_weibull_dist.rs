// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `WEIBULL.DIST` that returns the Weibull distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `alpha`: the shape parameter (must be > 0).
/// - `beta`: the scale parameter (must be > 0).
/// - `cumulative`: if `true`, returns the cumulative distribution function (CDF);
///   if `false`, returns the probability density function (PDF).
///
/// Returns the distribution value or an error when inputs are outside the allowed range.
pub fn codcel_weibull_dist(
    x: f64,
    alpha: f64,
    beta: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x < 0.0 {
        return Err("WEIBULL.DIST: x must be non-negative.".into());
    }
    if alpha <= 0.0 {
        return Err("WEIBULL.DIST: alpha (shape parameter) must be greater than 0.".into());
    }
    if beta <= 0.0 {
        return Err("WEIBULL.DIST: beta (scale parameter) must be greater than 0.".into());
    }

    if cumulative {
        // Cumulative distribution function
        let cumulative_result = 1.0 - (-((x / beta).powf(alpha))).exp();
        Ok(cumulative_result)
    } else {
        // Probability density function
        let pdf_result =
            (alpha / beta) * (x / beta).powf(alpha - 1.0) * (-((x / beta).powf(alpha))).exp();
        Ok(pdf_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weibull_dist_pdf_basic() {
        // =WEIBULL.DIST(1, 2, 3, FALSE) in US format
        // =WEIBULL.DIST(1; 2; 3; FALSE) in German format
        let result = codcel_weibull_dist(1.0, 2.0, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.1988531815143044).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_dist_cdf_basic() {
        // =WEIBULL.DIST(1, 2, 3, TRUE) in US format
        // =WEIBULL.DIST(1; 2; 3; TRUE) in German format
        let result = codcel_weibull_dist(1.0, 2.0, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.105_160_683_185_633_3).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_dist_pdf_different_x() {
        // =WEIBULL.DIST(2, 2, 3, FALSE) in US format
        // =WEIBULL.DIST(2; 2; 3; FALSE) in German format
        let result = codcel_weibull_dist(2.0, 2.0, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.28496906152442425).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_dist_cdf_different_x() {
        // =WEIBULL.DIST(2, 2, 3, TRUE) in US format
        // =WEIBULL.DIST(2; 2; 3; TRUE) in German format
        let result = codcel_weibull_dist(2.0, 2.0, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.3588196115700454).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_dist_pdf_different_alpha() {
        // =WEIBULL.DIST(1, 3, 3, FALSE) in US format
        // =WEIBULL.DIST(1; 3; 3; FALSE) in German format
        let result = codcel_weibull_dist(1.0, 3.0, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.10707116047792069).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_dist_cdf_different_alpha() {
        // =WEIBULL.DIST(1, 3, 3, TRUE) in US format
        // =WEIBULL.DIST(1; 3; 3; TRUE) in German format
        let result = codcel_weibull_dist(1.0, 3.0, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.03635955569871374).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_dist_pdf_different_beta() {
        // =WEIBULL.DIST(1, 2, 4, FALSE) in US format
        // =WEIBULL.DIST(1; 2; 4; FALSE) in German format
        let result = codcel_weibull_dist(1.0, 2.0, 4.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.11752011936438012).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_dist_cdf_different_beta() {
        // =WEIBULL.DIST(1, 2, 4, TRUE) in US format
        // =WEIBULL.DIST(1; 2; 4; TRUE) in German format
        let result = codcel_weibull_dist(1.0, 2.0, 4.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.06058693718652421).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_dist_pdf_zero_x() {
        // =WEIBULL.DIST(0, 2, 3, FALSE) in US format
        // =WEIBULL.DIST(0; 2; 3; FALSE) in German format
        let result = codcel_weibull_dist(0.0, 2.0, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_dist_cdf_zero_x() {
        // =WEIBULL.DIST(0, 2, 3, TRUE) in US format
        // =WEIBULL.DIST(0; 2; 3; TRUE) in German format
        let result = codcel_weibull_dist(0.0, 2.0, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_dist_negative_x() {
        // Negative x should return an error
        let result = codcel_weibull_dist(-1.0, 2.0, 3.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_weibull_dist_zero_alpha() {
        // Alpha = 0 should return an error
        let result = codcel_weibull_dist(1.0, 0.0, 3.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_weibull_dist_negative_alpha() {
        // Negative alpha should return an error
        let result = codcel_weibull_dist(1.0, -1.0, 3.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_weibull_dist_zero_beta() {
        // Beta = 0 should return an error
        let result = codcel_weibull_dist(1.0, 2.0, 0.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_weibull_dist_negative_beta() {
        // Negative beta should return an error
        let result = codcel_weibull_dist(1.0, 2.0, -1.0, true);
        assert!(result.is_err());
    }
}
