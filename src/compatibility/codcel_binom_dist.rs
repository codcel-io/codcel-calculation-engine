// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compatibility::binomial_probability::binomial_probability;
use std::error::Error;

/// Excel-compatible `BINOMDIST`/`BINOM.DIST` function.
/// Evaluates the binomial distribution.
/// - `successes`: number of successes in trials.
/// - `trials`: number of independent trials (must be greater than 0).
/// - `probability`: probability of success on each trial, in `[0, 1]`.
/// - `cumulative`: `true` for cumulative probability up to `successes`, `false` for exact probability mass.
///
/// Returns an error for invalid counts or probabilities outside allowed ranges.
pub fn codcel_binom_dist(
    successes: i32,
    trials: i32,
    probability: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if trials == 0 {
        return Err("BINOMDIST: Number of trials must be greater than 0.".into());
    }
    if !(0.0..=1.0).contains(&probability) {
        return Err("BINOMDIST: Probability must be in the range [0, 1].".into());
    }
    if successes > trials {
        return Err("BINOMDIST: Number of successes cannot exceed the number of trials.".into());
    }

    // Calculate the binomial distribution value
    if cumulative {
        let mut result = 0.0;
        for k in 0..=successes {
            result += binomial_probability(trials as u32, k as u32, probability)?;
        }
        Ok(result)
    } else {
        binomial_probability(trials as u32, successes as u32, probability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binom_dist_pmf_basic() {
        // =BINOMDIST(2, 5, 0.5, FALSE) in US format
        // =BINOMDIST(2; 5; 0,5; FALSE) in German format
        let result = codcel_binom_dist(2, 5, 0.5, false).unwrap();
        println!("{result}");
        assert!((result - 0.3125).abs() < 0.0001);
    }

    #[test]
    fn test_binom_dist_cdf_basic() {
        // =BINOMDIST(2, 5, 0.5, TRUE) in US format
        // =BINOMDIST(2; 5; 0,5; TRUE) in German format
        let result = codcel_binom_dist(2, 5, 0.5, true).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_binom_dist_pmf_zero_successes() {
        // =BINOMDIST(0, 5, 0.5, FALSE) in US format
        // =BINOMDIST(0; 5; 0,5; FALSE) in German format
        let result = codcel_binom_dist(0, 5, 0.5, false).unwrap();
        println!("{result}");
        assert!((result - 0.03125).abs() < 0.0001);
    }

    #[test]
    fn test_binom_dist_cdf_zero_successes() {
        // =BINOMDIST(0, 5, 0.5, TRUE) in US format
        // =BINOMDIST(0; 5; 0,5; TRUE) in German format
        let result = codcel_binom_dist(0, 5, 0.5, true).unwrap();
        println!("{result}");
        assert!((result - 0.03125).abs() < 0.0001);
    }

    #[test]
    fn test_binom_dist_pmf_all_successes() {
        // =BINOMDIST(5, 5, 0.5, FALSE) in US format
        // =BINOMDIST(5; 5; 0,5; FALSE) in German format
        let result = codcel_binom_dist(5, 5, 0.5, false).unwrap();
        println!("{result}");
        assert!((result - 0.03125).abs() < 0.0001);
    }

    #[test]
    fn test_binom_dist_cdf_all_successes() {
        // =BINOMDIST(5, 5, 0.5, TRUE) in US format
        // =BINOMDIST(5; 5; 0,5; TRUE) in German format
        let result = codcel_binom_dist(5, 5, 0.5, true).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_binom_dist_pmf_different_probability() {
        // =BINOMDIST(2, 5, 0.3, FALSE) in US format
        // =BINOMDIST(2; 5; 0,3; FALSE) in German format
        let result = codcel_binom_dist(2, 5, 0.3, false).unwrap();
        println!("{result}");
        assert!((result - 0.3087).abs() < 0.0001);
    }

    #[test]
    fn test_binom_dist_cdf_different_probability() {
        // =BINOMDIST(2, 5, 0.3, TRUE) in US format
        // =BINOMDIST(2; 5; 0,3; TRUE) in German format
        let result = codcel_binom_dist(2, 5, 0.3, true).unwrap();
        println!("{result}");
        assert!((result - 0.8369199999999997).abs() < 0.0001);
    }

    #[test]
    fn test_binom_dist_zero_trials() {
        // Zero trials should return an error
        let result = codcel_binom_dist(0, 0, 0.5, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_dist_negative_probability() {
        // Negative probability should return an error
        let result = codcel_binom_dist(2, 5, -0.5, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_dist_probability_greater_than_one() {
        // Probability > 1 should return an error
        let result = codcel_binom_dist(2, 5, 1.5, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_dist_successes_exceed_trials() {
        // Successes > trials should return an error
        let result = codcel_binom_dist(6, 5, 0.5, true);
        assert!(result.is_err());
    }
}
