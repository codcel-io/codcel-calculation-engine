// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `CONFIDENCE.T` that returns the confidence interval for a population mean using Student's t-distribution.
/// - `alpha`: the significance level (0 to 1, exclusive).
/// - `standard_deviation`: the sample standard deviation (must be > 0).
/// - `size`: the sample size (must be > 1).
///
/// Returns the margin of error for a confidence interval at the (1 - alpha) confidence level,
/// or an error when inputs are outside the allowed range.
pub fn codcel_confidence_t(
    alpha: f64,
    standard_deviation: f64,
    size: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if !(0.0..1.0).contains(&alpha) {
        return Err("CONFIDENCE.T: Alpha must be in the range (0, 1).".into());
    }
    if standard_deviation <= 0.0 {
        return Err("CONFIDENCE.T: Standard deviation must be greater than 0.".into());
    }
    if size <= 1 {
        return Err("CONFIDENCE.T: Sample size must be greater than 1.".into());
    }

    // Degrees of freedom
    let degrees_of_freedom = (size - 1) as f64;

    // Calculate the t-score for the given alpha and degrees of freedom
    let t = match statrs::distribution::StudentsT::new(0.0, 1.0, degrees_of_freedom) {
        Ok(t_dist) => t_dist.inverse_cdf(1.0 - alpha / 2.0),
        Err(_) => return Err("CONFIDENCE.T: Error creating Student's T distribution.".into()),
    };

    // Calculate the confidence interval width
    let confidence = t * (standard_deviation / crate::portable_math::sqrt(size as f64));

    Ok(confidence)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_t_basic() {
        // =CONFIDENCE.T(0.05, 2.5, 50) in US format
        // =CONFIDENCE.T(0,05; 2,5; 50) in German format
        let result = codcel_confidence_t(0.05, 2.5, 50).unwrap();
        assert!((result - 0.7104921387391621).abs() < 0.0000001);
    }

    #[test]
    fn test_confidence_t_high_confidence() {
        // =CONFIDENCE.T(0.01, 2.5, 50) in US format
        // =CONFIDENCE.T(0,01; 2,5; 50) in German format
        let result = codcel_confidence_t(0.01, 2.5, 50).unwrap();
        println!("{result:?}");
        assert!((result - 0.9475061069045437).abs() < 0.0000001);
    }

    #[test]
    fn test_confidence_t_small_sample() {
        // =CONFIDENCE.T(0.05, 2.5, 10) in US format
        // =CONFIDENCE.T(0,05; 2,5; 10) in German format
        let result = codcel_confidence_t(0.05, 2.5, 10).unwrap();
        println!("{result:?}");
        assert!((result - 1.7883922649266606).abs() < 0.0000001);
    }

    #[test]
    fn test_confidence_t_medium_sample() {
        // =CONFIDENCE.T(0.05, 2.5, 30) in US format
        // =CONFIDENCE.T(0,05; 2,5; 30) in German format
        let result = codcel_confidence_t(0.05, 2.5, 30).unwrap();
        assert!((result - 0.9335153418951071).abs() < 0.0000001);
    }

    #[test]
    fn test_confidence_t_small_std_dev() {
        // =CONFIDENCE.T(0.05, 0.1, 50) in US format
        // =CONFIDENCE.T(0,05; 0,1; 50) in German format
        let result = codcel_confidence_t(0.05, 0.1, 50).unwrap();
        assert!((result - 0.028419685549566486).abs() < 0.0000001);
    }

    #[test]
    fn test_confidence_t_invalid_alpha() {
        // Alpha outside (0,1) range
        let result = codcel_confidence_t(1.5, 2.5, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_confidence_t_invalid_std_dev() {
        // Negative standard deviation
        let result = codcel_confidence_t(0.05, -2.5, 50);
        assert!(result.is_err());
    }

    #[test]
    fn test_confidence_t_invalid_size() {
        // Sample size <= 1
        let result = codcel_confidence_t(0.05, 2.5, 1);
        assert!(result.is_err());
    }
}
