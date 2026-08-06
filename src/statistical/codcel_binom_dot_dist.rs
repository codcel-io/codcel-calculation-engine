// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compatibility::codcel_binom_dist::codcel_binom_dist;
use std::error::Error;

/// Excel-compatible `BINOM.DIST` that returns the binomial distribution probability.
/// - `successes`: the number of successes in trials.
/// - `trials`: the number of independent trials.
/// - `probability`: the probability of success on each trial (0 to 1).
/// - `cumulative`: if `true`, returns the cumulative distribution function;
///   if `false`, returns the probability mass function.
///
/// Returns the probability or an error when inputs are outside the allowed range.
/// Note: This is equivalent to the older BINOMDIST function.
pub fn codcel_binom_dot_dist(
    successes: i32,
    trials: i32,
    probability: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    codcel_binom_dist(successes, trials, probability, cumulative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binom_dist_pmf() {
        // =BINOM.DIST(6,10,0.5,FALSE) in US format
        // =BINOM.DIST(6;10;0,5;FALSE) in German format
        let result = codcel_binom_dot_dist(6, 10, 0.5, false).unwrap();
        assert!((result - 0.205078125).abs() < 1e-10); // PMF of Binomial(10, 0.5) at x=6
    }

    #[test]
    fn test_binom_dist_cdf() {
        // =BINOM.DIST(6,10,0.5,TRUE) in US format
        // =BINOM.DIST(6;10;0,5;TRUE) in German format
        let result = codcel_binom_dot_dist(6, 10, 0.5, true).unwrap();
        assert!((result - 0.828125).abs() < 1e-10); // CDF of Binomial(10, 0.5) at x=6
    }

    #[test]
    fn test_binom_dist_zero_successes() {
        // =BINOM.DIST(0,10,0.5,FALSE) in US format
        // =BINOM.DIST(0;10;0,5;FALSE) in German format
        let result = codcel_binom_dot_dist(0, 10, 0.5, false).unwrap();
        assert!((result - 0.0009765625).abs() < 1e-10); // PMF of Binomial(10, 0.5) at x=0
    }

    #[test]
    fn test_binom_dist_all_successes() {
        // =BINOM.DIST(10,10,0.5,FALSE) in US format
        // =BINOM.DIST(10;10;0,5;FALSE) in German format
        let result = codcel_binom_dot_dist(10, 10, 0.5, false).unwrap();
        assert!((result - 0.0009765625).abs() < 1e-10); // PMF of Binomial(10, 0.5) at x=10
    }

    #[test]
    fn test_binom_dist_low_probability() {
        // =BINOM.DIST(2,10,0.1,FALSE) in US format
        // =BINOM.DIST(2;10;0,1;FALSE) in German format
        let result = codcel_binom_dot_dist(2, 10, 0.1, false).unwrap();
        assert!((result - 0.1937102445).abs() < 1e-10); // PMF of Binomial(10, 0.1) at x=2
    }

    #[test]
    fn test_binom_dist_high_probability() {
        // =BINOM.DIST(8,10,0.9,FALSE) in US format
        // =BINOM.DIST(8;10;0,9;FALSE) in German format
        let result = codcel_binom_dot_dist(8, 10, 0.9, false).unwrap();
        assert!((result - 0.1937102445).abs() < 1e-10); // PMF of Binomial(10, 0.9) at x=8
    }

    #[test]
    fn test_binom_dist_certainty() {
        // =BINOM.DIST(10,10,1,FALSE) in US format
        // =BINOM.DIST(10;10;1;FALSE) in German format
        let result = codcel_binom_dot_dist(10, 10, 1.0, false).unwrap();
        assert_eq!(result, 1.0); // PMF of Binomial(10, 1.0) at x=10 is 1
    }

    #[test]
    fn test_binom_dist_impossibility() {
        // =BINOM.DIST(0,10,1,FALSE) in US format
        // =BINOM.DIST(0;10;1;FALSE) in German format
        let result = codcel_binom_dot_dist(0, 10, 1.0, false).unwrap();
        assert_eq!(result, 0.0); // PMF of Binomial(10, 1.0) at x=0 is 0
    }

    #[test]
    fn test_binom_dist_invalid_trials() {
        // =BINOM.DIST(1,0,0.5,FALSE) in US format (returns #NUM! error)
        // =BINOM.DIST(1;0;0,5;FALSE) in German format (returns #NUM! error)
        let result = codcel_binom_dot_dist(1, 0, 0.5, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_dist_invalid_probability_low() {
        // =BINOM.DIST(1,10,-0.1,FALSE) in US format (returns #NUM! error)
        // =BINOM.DIST(1;10;-0,1;FALSE) in German format (returns #NUM! error)
        let result = codcel_binom_dot_dist(1, 10, -0.1, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_dist_invalid_probability_high() {
        // =BINOM.DIST(1,10,1.1,FALSE) in US format (returns #NUM! error)
        // =BINOM.DIST(1;10;1,1;FALSE) in German format (returns #NUM! error)
        let result = codcel_binom_dot_dist(1, 10, 1.1, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_dist_invalid_successes() {
        // =BINOM.DIST(11,10,0.5,FALSE) in US format (returns #NUM! error)
        // =BINOM.DIST(11;10;0,5;FALSE) in German format (returns #NUM! error)
        let result = codcel_binom_dot_dist(11, 10, 0.5, false);
        assert!(result.is_err());
    }
}
