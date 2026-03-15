// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `CONFIDENCE` function (z-based margin of error).
/// Computes the confidence interval half-width using the standard normal distribution.
/// - `alpha`: significance level (must be in `(0, 1)`).
/// - `standard_dev`: population standard deviation (must be greater than 0).
/// - `size`: sample size (must be greater than 0).
///
/// Returns an error for invalid parameter values.
pub fn codcel_confidence(
    alpha: f64,
    standard_dev: f64,
    size: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if !(0.0 < alpha && alpha < 1.0) {
        return Err("CONFIDENCE: Alpha must be between 0 and 1".into());
    }

    if standard_dev <= 0.0 {
        return Err("CONFIDENCE: Standard deviation (standard_dev) must be greater than 0".into());
    }

    if size == 0 {
        return Err("CONFIDENCE: Sample size (size) must be greater than 0".into());
    }

    // Convert alpha to z-score: z = NORM.S.INV(1 - alpha / 2)
    let z_score = statrs::distribution::Normal::new(0.0, 1.0)
        .map_err(|_| "CONFIDENCE: Unable to create normal distribution")?
        .inverse_cdf(1.0 - alpha / 2.0);

    // Calculate margin of error: z * (standard_dev / sqrt(size))
    let margin_of_error = z_score * (standard_dev / (size as f64).sqrt());

    Ok(margin_of_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_basic() {
        // =CONFIDENCE(0.05, 2.5, 50) in US format
        // =CONFIDENCE(0,05; 2,5; 50) in German format
        let result = codcel_confidence(0.05, 2.5, 50).unwrap();
        println!("{result}");
        assert!((result - 0.6929519121748388).abs() < 0.0001);
    }

    #[test]
    fn test_confidence_small_alpha() {
        // =CONFIDENCE(0.01, 2.5, 50) in US format
        // =CONFIDENCE(0,01; 2,5; 50) in German format
        let result = codcel_confidence(0.01, 2.5, 50).unwrap();
        println!("{result}");
        assert!((result - 0.9106931838592248).abs() < 0.0001);
    }

    #[test]
    fn test_confidence_large_alpha() {
        // =CONFIDENCE(0.1, 2.5, 50) in US format
        // =CONFIDENCE(0,1; 2,5; 50) in German format
        let result = codcel_confidence(0.1, 2.5, 50).unwrap();
        println!("{result}");
        assert!((result - 0.5815435768383369).abs() < 0.0001);
    }

    #[test]
    fn test_confidence_small_std_dev() {
        // =CONFIDENCE(0.05, 0.5, 50) in US format
        // =CONFIDENCE(0,05; 0,5; 50) in German format
        let result = codcel_confidence(0.05, 0.5, 50).unwrap();
        println!("{result}");
        assert!((result - 0.13859038243496777).abs() < 0.0001);
    }

    #[test]
    fn test_confidence_large_std_dev() {
        // =CONFIDENCE(0.05, 10, 50) in US format
        // =CONFIDENCE(0,05; 10; 50) in German format
        let result = codcel_confidence(0.05, 10.0, 50).unwrap();
        println!("{result}");
        assert!((result - 2.771807648699355).abs() < 0.0001);
    }

    #[test]
    fn test_confidence_small_size() {
        // =CONFIDENCE(0.05, 2.5, 10) in US format
        // =CONFIDENCE(0,05; 2,5; 10) in German format
        let result = codcel_confidence(0.05, 2.5, 10).unwrap();
        println!("{result}");
        assert!((result - 1.5494875807614037).abs() < 0.0001);
    }

    #[test]
    fn test_confidence_large_size() {
        // =CONFIDENCE(0.05, 2.5, 1000) in US format
        // =CONFIDENCE(0,05; 2,5; 1000) in German format
        let result = codcel_confidence(0.05, 2.5, 1000).unwrap();
        println!("{result}");
        assert!((result - 0.1549487580761404).abs() < 0.0001);
    }

    #[test]
    fn test_confidence_alpha_zero() {
        // Alpha = 0 should return an error
        let result = codcel_confidence(0.0, 2.5, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_confidence_alpha_one() {
        // Alpha = 1 should return an error
        let result = codcel_confidence(1.0, 2.5, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_confidence_negative_std_dev() {
        // Negative standard deviation should return an error
        let result = codcel_confidence(0.05, -2.5, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_confidence_zero_std_dev() {
        // Zero standard deviation should return an error
        let result = codcel_confidence(0.05, 0.0, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_confidence_zero_size() {
        // Zero size should return an error
        let result = codcel_confidence(0.05, 2.5, 0);
        assert!(result.is_err());
    }
}
