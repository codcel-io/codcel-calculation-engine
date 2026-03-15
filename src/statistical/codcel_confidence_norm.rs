// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `CONFIDENCE.NORM` that returns the confidence interval for a population mean using a normal distribution.
/// - `alpha`: the significance level (0 to 1, exclusive).
/// - `standard_deviation`: the population standard deviation (must be > 0).
/// - `size`: the sample size (must be > 0).
///
/// Returns the margin of error for a confidence interval at the (1 - alpha) confidence level,
/// or an error when inputs are outside the allowed range.
pub fn codcel_confidence_norm(
    alpha: f64,
    standard_deviation: f64,
    size: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if !(0.0..1.0).contains(&alpha) {
        return Err("CONFIDENCE.NORM: Alpha must be in the range (0, 1).".into());
    }
    if standard_deviation <= 0.0 {
        return Err("CONFIDENCE.NORM: Standard deviation must be greater than 0.".into());
    }
    if size == 0 {
        return Err("CONFIDENCE.NORM: Sample size must be greater than 0.".into());
    }

    // Calculate the z-score for the given alpha
    let z = match statrs::distribution::Normal::new(0.0, 1.0) {
        Ok(normal_dist) => normal_dist.inverse_cdf(1.0 - alpha / 2.0),
        Err(_) => return Err("CONFIDENCE.NORM: Error creating normal distribution.".into()),
    };

    // Calculate the confidence interval width
    let confidence = z * (standard_deviation / (size as f64).sqrt());

    Ok(confidence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_norm_basic() {
        // =CONFIDENCE.NORM(0.05, 2.5, 50) in US format
        // =CONFIDENCE.NORM(0,05; 2,5; 50) in German format
        let result = codcel_confidence_norm(0.05, 2.5, 50).unwrap();
        assert!((result - 0.6929519121748388).abs() < 0.0000001);
    }

    #[test]
    fn test_confidence_norm_high_confidence() {
        // =CONFIDENCE.NORM(0.01, 2.5, 50) in US format
        // =CONFIDENCE.NORM(0,01; 2,5; 50) in German format
        let result = codcel_confidence_norm(0.01, 2.5, 50).unwrap();
        println!("{result}");
        assert!((result - 0.91069318).abs() < 0.0000001);
    }

    #[test]
    fn test_confidence_norm_small_sample() {
        // =CONFIDENCE.NORM(0.05, 2.5, 10) in US format
        // =CONFIDENCE.NORM(0,05; 2,5; 10) in German format
        let result = codcel_confidence_norm(0.05, 2.5, 10).unwrap();
        println!("{result}");
        assert!((result - 1.54948758).abs() < 0.0000001);
    }

    #[test]
    fn test_confidence_norm_large_sample() {
        // =CONFIDENCE.NORM(0.05, 2.5, 1000) in US format
        // =CONFIDENCE.NORM(0,05; 2,5; 1000) in German format
        let result = codcel_confidence_norm(0.05, 2.5, 1000).unwrap();
        assert!((result - 0.1549487580761404).abs() < 0.0000001);
    }

    #[test]
    fn test_confidence_norm_small_std_dev() {
        // =CONFIDENCE.NORM(0.05, 0.1, 50) in US format
        // =CONFIDENCE.NORM(0,05; 0,1; 50) in German format
        let result = codcel_confidence_norm(0.05, 0.1, 50).unwrap();
        assert!((result - 0.027718076486993554).abs() < 0.0000001);
    }

    #[test]
    fn test_confidence_norm_invalid_alpha() {
        // Alpha outside (0,1) range
        let result = codcel_confidence_norm(1.5, 2.5, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_confidence_norm_invalid_std_dev() {
        // Negative standard deviation
        let result = codcel_confidence_norm(0.05, -2.5, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_confidence_norm_invalid_size() {
        // Zero sample size
        let result = codcel_confidence_norm(0.05, 2.5, 0);
        assert!(result.is_err());
    }
}
