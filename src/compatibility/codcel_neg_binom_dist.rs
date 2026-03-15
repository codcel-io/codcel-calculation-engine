// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::statistical::neg_binom_pmf::neg_binom_pmf;
use std::error::Error;

/// Excel-compatible `NEGBINOMDIST`/`NEGBINOM.DIST` function.
/// Returns the probability mass of observing failures before achieving successes.
/// - `failures`: number of failures (must be non-negative).
/// - `successes`: threshold number of successes (must be non-negative).
/// - `probability`: probability of success on each trial, in `[0, 1]`.
///
/// Returns an error on negative counts or probabilities outside `[0, 1]`.
pub fn codcel_neg_binom_dist(
    failures: i32,
    successes: i32,
    probability: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if failures < 0 {
        return Err("NEGBINOMDIST: Failures cannot be negative.".into());
    }

    if successes < 0 {
        return Err("NEGBINOMDIST: Successes cannot be negative.".into());
    }

    if !(0.0..=1.0).contains(&probability) {
        return Err("NEGBINOMDIST: Probability must be between 0 and 1, inclusive.".into());
    }

    // Delegate to the PMF calculation
    neg_binom_pmf(failures as u32, successes as u32, probability)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg_binom_dist_basic() {
        // =NEGBINOMDIST(10, 5, 0.25) in US format
        // =NEGBINOMDIST(10; 5; 0,25) in German format
        let result = codcel_neg_binom_dist(10, 5, 0.25).unwrap();
        println!("{result}");
        assert!((result - 0.0550486).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dist_zero_failures() {
        // =NEGBINOMDIST(0, 5, 0.25) in US format
        // =NEGBINOMDIST(0; 5; 0,25) in German format
        let result = codcel_neg_binom_dist(0, 5, 0.25).unwrap();
        println!("{result}");
        assert!((result - 0.0009765625).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dist_high_probability() {
        // =NEGBINOMDIST(10, 5, 0.75) in US format
        // =NEGBINOMDIST(10; 5; 0,75) in German format
        let result = codcel_neg_binom_dist(10, 5, 0.75).unwrap();
        println!("{result}");
        assert!((result - 0.00022653769701719284).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dist_probability_one() {
        // =NEGBINOMDIST(0, 5, 1) in US format
        // =NEGBINOMDIST(0; 5; 1) in German format
        let result = codcel_neg_binom_dist(0, 5, 1.0).unwrap();
        println!("{result}");
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_neg_binom_dist_negative_failures() {
        // =NEGBINOMDIST(-1, 5, 0.25) in US format
        // =NEGBINOMDIST(-1; 5; 0,25) in German format
        let result = codcel_neg_binom_dist(-1, 5, 0.25);
        assert!(result.is_err());
    }

    #[test]
    fn test_neg_binom_dist_negative_successes() {
        // =NEGBINOMDIST(10, -1, 0.25) in US format
        // =NEGBINOMDIST(10; -1; 0,25) in German format
        let result = codcel_neg_binom_dist(10, -1, 0.25);
        assert!(result.is_err());
    }

    #[test]
    fn test_neg_binom_dist_invalid_probability() {
        // =NEGBINOMDIST(10, 5, 1.5) in US format
        // =NEGBINOMDIST(10; 5; 1,5) in German format
        let result = codcel_neg_binom_dist(10, 5, 1.5);
        assert!(result.is_err());
    }
}
