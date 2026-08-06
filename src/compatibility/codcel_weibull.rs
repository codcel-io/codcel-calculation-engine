// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::codcel_weibull_dist::codcel_weibull_dist;
use std::error::Error;

/// Excel-compatible `WEIBULL`/`WEIBULL.DIST` function.
/// Evaluates the Weibull distribution.
/// - `x`: value at which to evaluate (must be non-negative).
/// - `alpha`: shape parameter (must be greater than 0).
/// - `beta`: scale parameter (must be greater than 0).
/// - `cumulative`: `true` for cumulative distribution (CDF), `false` for probability density (PDF).
///
/// Returns an error on negative `x` or non-positive parameters.
pub fn codcel_weibull(
    x: f64,
    alpha: f64,
    beta: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Delegate to `xlsx_weibull_dist`
    codcel_weibull_dist(x, alpha, beta, cumulative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weibull_pdf_basic() {
        // =WEIBULL(1, 2, 3, FALSE) in US format
        // =WEIBULL(1; 2; 3; FALSE) in German format
        let result = codcel_weibull(1.0, 2.0, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.1988531815143044).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_cdf_basic() {
        // =WEIBULL(1, 2, 3, TRUE) in US format
        // =WEIBULL(1; 2; 3; TRUE) in German format
        let result = codcel_weibull(1.0, 2.0, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.105_160_683_185_633_3).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_pdf_different_x() {
        // =WEIBULL(2, 2, 3, FALSE) in US format
        // =WEIBULL(2; 2; 3; FALSE) in German format
        let result = codcel_weibull(2.0, 2.0, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.28496906152442425).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_cdf_different_x() {
        // =WEIBULL(2, 2, 3, TRUE) in US format
        // =WEIBULL(2; 2; 3; TRUE) in German format
        let result = codcel_weibull(2.0, 2.0, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.3588196115700454).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_pdf_different_alpha() {
        // =WEIBULL(1, 3, 3, FALSE) in US format
        // =WEIBULL(1; 3; 3; FALSE) in German format
        let result = codcel_weibull(1.0, 3.0, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.10707116047792069).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_cdf_different_alpha() {
        // =WEIBULL(1, 3, 3, TRUE) in US format
        // =WEIBULL(1; 3; 3; TRUE) in German format
        let result = codcel_weibull(1.0, 3.0, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.03635955569871374).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_pdf_different_beta() {
        // =WEIBULL(1, 2, 4, FALSE) in US format
        // =WEIBULL(1; 2; 4; FALSE) in German format
        let result = codcel_weibull(1.0, 2.0, 4.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.11752011936438012).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_cdf_different_beta() {
        // =WEIBULL(1, 2, 4, TRUE) in US format
        // =WEIBULL(1; 2; 4; TRUE) in German format
        let result = codcel_weibull(1.0, 2.0, 4.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.06058693718652421).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_pdf_zero_x() {
        // =WEIBULL(0, 2, 3, FALSE) in US format
        // =WEIBULL(0; 2; 3; FALSE) in German format
        let result = codcel_weibull(0.0, 2.0, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_cdf_zero_x() {
        // =WEIBULL(0, 2, 3, TRUE) in US format
        // =WEIBULL(0; 2; 3; TRUE) in German format
        let result = codcel_weibull(0.0, 2.0, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_weibull_negative_x() {
        // Negative x should return an error
        let result = codcel_weibull(-1.0, 2.0, 3.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_weibull_zero_alpha() {
        // Alpha = 0 should return an error
        let result = codcel_weibull(1.0, 0.0, 3.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_weibull_negative_alpha() {
        // Negative alpha should return an error
        let result = codcel_weibull(1.0, -1.0, 3.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_weibull_zero_beta() {
        // Beta = 0 should return an error
        let result = codcel_weibull(1.0, 2.0, 0.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_weibull_negative_beta() {
        // Negative beta should return an error
        let result = codcel_weibull(1.0, 2.0, -1.0, true);
        assert!(result.is_err());
    }
}
