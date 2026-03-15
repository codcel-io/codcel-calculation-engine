// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::statistical::codcel_expon_dot_dist::codcel_expon_dot_dist;
use std::error::Error;

/// Excel-compatible `EXPONDIST`/`EXPON.DIST` function.
/// Evaluates the exponential distribution.
/// - `x`: value at which to evaluate (must be non-negative).
/// - `lambda`: rate parameter (must be greater than 0).
/// - `cumulative`: `true` for cumulative distribution (CDF), `false` for probability density (PDF).
///
/// Returns an error on negative `x` or non-positive `lambda`.
pub fn codcel_expon_dist(
    x: f64,
    lambda: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // EXPONDIST and EXPON.DIST are exactly the same
    codcel_expon_dot_dist(x, lambda, cumulative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expon_dist_pdf_basic() {
        // =EXPONDIST(2, 0.5, FALSE) in US format
        // =EXPONDIST(2; 0,5; FALSCH) in German format
        let result = codcel_expon_dist(2.0, 0.5, false).unwrap();
        println!("{result}");
        assert!((result - 0.1839397).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dist_cdf_basic() {
        // =EXPONDIST(2, 0.5, TRUE) in US format
        // =EXPONDIST(2; 0,5; WAHR) in German format
        let result = codcel_expon_dist(2.0, 0.5, true).unwrap();
        println!("{result}");
        assert!((result - 0.6321206).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dist_pdf_zero_x() {
        // =EXPONDIST(0, 0.5, FALSE) in US format
        // =EXPONDIST(0; 0,5; FALSCH) in German format
        let result = codcel_expon_dist(0.0, 0.5, false).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dist_cdf_zero_x() {
        // =EXPONDIST(0, 0.5, TRUE) in US format
        // =EXPONDIST(0; 0,5; WAHR) in German format
        let result = codcel_expon_dist(0.0, 0.5, true).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dist_pdf_large_x() {
        // =EXPONDIST(10, 0.5, FALSE) in US format
        // =EXPONDIST(10; 0,5; FALSCH) in German format
        let result = codcel_expon_dist(10.0, 0.5, false).unwrap();
        println!("{result}");
        assert!((result - 0.003369).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dist_cdf_large_x() {
        // =EXPONDIST(10, 0.5, TRUE) in US format
        // =EXPONDIST(10; 0,5; WAHR) in German format
        let result = codcel_expon_dist(10.0, 0.5, true).unwrap();
        println!("{result}");
        assert!((result - 0.9932621).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dist_pdf_large_lambda() {
        // =EXPONDIST(2, 5, FALSE) in US format
        // =EXPONDIST(2; 5; FALSCH) in German format
        let result = codcel_expon_dist(2.0, 5.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.00022699964881242428).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dist_cdf_large_lambda() {
        // =EXPONDIST(2, 5, TRUE) in US format
        // =EXPONDIST(2; 5; WAHR) in German format
        let result = codcel_expon_dist(2.0, 5.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.9999546).abs() < 0.0000001);
    }

    #[test]
    fn test_expon_dist_negative_x() {
        // Negative x should return an error
        let result = codcel_expon_dist(-1.0, 0.5, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_expon_dist_zero_lambda() {
        // Zero lambda should return an error
        let result = codcel_expon_dist(2.0, 0.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_expon_dist_negative_lambda() {
        // Negative lambda should return an error
        let result = codcel_expon_dist(2.0, -0.5, true);
        assert!(result.is_err());
    }
}
