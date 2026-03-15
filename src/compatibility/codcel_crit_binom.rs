// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::Discrete;
use std::error::Error;

/// Excel-compatible `CRITBINOM`/`CRITBINOMIAL` function.
/// Returns the smallest number of successes for which the cumulative binomial probability >= `alpha`.
/// - `trials`: number of Bernoulli trials (must be non-negative).
/// - `probability`: probability of success on each trial, in `[0, 1]`.
/// - `alpha`: criterion probability, in `[0, 1]`.
///
/// Returns an error on invalid probabilities or negative trial counts.
pub fn codcel_crit_binom(
    trials: i32,
    probability: f64,
    alpha: f64,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if trials < 0 {
        return Err("CRITBINOM: Number of trials must be non-negative".into());
    }

    if !(0.0..=1.0).contains(&probability) {
        return Err("CRITBINOM: Probability must be between 0 and 1".into());
    }

    if !(0.0..=1.0).contains(&alpha) {
        return Err("CRITBINOM: Alpha must be between 0 and 1".into());
    }

    let binomial = statrs::distribution::Binomial::new(probability, trials as u64)
        .map_err(|_| "CRITBINOM: Unable to create binomial distribution")?;

    let mut cumulative = 0.0;

    for x in 0..=trials {
        cumulative += binomial.pmf(x as u64);

        if cumulative >= alpha {
            return Ok(x);
        }
    }

    Ok(trials) // If alpha is never reached, return the maximum number of trials
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crit_binom_basic() {
        // =CRITBINOM(10, 0.5, 0.5) in US format
        // =CRITBINOM(10; 0,5; 0,5) in German format
        let result = codcel_crit_binom(10, 0.5, 0.5).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_crit_binom_small_alpha() {
        // =CRITBINOM(10, 0.5, 0.1) in US format
        // =CRITBINOM(10; 0,5; 0,1) in German format
        let result = codcel_crit_binom(10, 0.5, 0.1).unwrap();
        println!("{result}");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_crit_binom_large_alpha() {
        // =CRITBINOM(10, 0.5, 0.9) in US format
        // =CRITBINOM(10; 0,5; 0,9) in German format
        let result = codcel_crit_binom(10, 0.5, 0.9).unwrap();
        println!("{result}");
        assert_eq!(result, 7);
    }

    #[test]
    fn test_crit_binom_small_probability() {
        // =CRITBINOM(10, 0.2, 0.5) in US format
        // =CRITBINOM(10; 0,2; 0,5) in German format
        let result = codcel_crit_binom(10, 0.2, 0.5).unwrap();
        println!("{result}");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_crit_binom_large_probability() {
        // =CRITBINOM(10, 0.8, 0.5) in US format
        // =CRITBINOM(10; 0,8; 0,5) in German format
        let result = codcel_crit_binom(10, 0.8, 0.5).unwrap();
        println!("{result}");
        assert_eq!(result, 8);
    }

    #[test]
    fn test_crit_binom_small_trials() {
        // =CRITBINOM(5, 0.5, 0.5) in US format
        // =CRITBINOM(5; 0,5; 0,5) in German format
        let result = codcel_crit_binom(5, 0.5, 0.5).unwrap();
        println!("{result}");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_crit_binom_large_trials() {
        // =CRITBINOM(20, 0.5, 0.5) in US format
        // =CRITBINOM(20; 0,5; 0,5) in German format
        let result = codcel_crit_binom(20, 0.5, 0.5).unwrap();
        println!("{result}");
        assert_eq!(result, 10);
    }

    #[test]
    fn test_crit_binom_zero_trials() {
        // =CRITBINOM(0, 0.5, 0.5) in US format
        // =CRITBINOM(0; 0,5; 0,5) in German format
        let result = codcel_crit_binom(0, 0.5, 0.5).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_crit_binom_alpha_zero() {
        // =CRITBINOM(10, 0.5, 0) in US format
        // =CRITBINOM(10; 0,5; 0) in German format
        let result = codcel_crit_binom(10, 0.5, 0.0).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_crit_binom_alpha_one() {
        // =CRITBINOM(10, 0.5, 1) in US format
        // =CRITBINOM(10; 0,5; 1) in German format
        let result = codcel_crit_binom(10, 0.5, 1.0).unwrap();
        println!("{result}");
        assert_eq!(result, 10);
    }

    #[test]
    fn test_crit_binom_negative_trials() {
        // Negative trials should return an error
        let result = codcel_crit_binom(-1, 0.5, 0.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_crit_binom_probability_out_of_range() {
        // Probability < 0 should return an error
        let result = codcel_crit_binom(10, -0.1, 0.5);
        assert!(result.is_err());

        // Probability > 1 should return an error
        let result = codcel_crit_binom(10, 1.1, 0.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_crit_binom_alpha_out_of_range() {
        // Alpha < 0 should return an error
        let result = codcel_crit_binom(10, 0.5, -0.1);
        assert!(result.is_err());

        // Alpha > 1 should return an error
        let result = codcel_crit_binom(10, 0.5, 1.1);
        assert!(result.is_err());
    }
}
