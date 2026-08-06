// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::neg_binom_pmf::neg_binom_pmf;
use std::error::Error;

/// Excel-compatible `NEGBINOM.DIST` that returns the negative binomial distribution.
/// - `failures`: the number of failures before the specified number of successes.
/// - `successes`: the threshold number of successes.
/// - `probability`: the probability of a success (0 to 1).
/// - `cumulative`: if `true`, returns the cumulative distribution function;
///   if `false`, returns the probability mass function.
///
/// Returns the probability or an error when inputs are outside the allowed range.
pub fn codcel_neg_binom_dot_dist(
    failures: i32,
    successes: i32,
    probability: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if !(0.0..=1.0).contains(&probability) {
        return Err("PNEGBINOM.DIST: robability must be between 0 and 1, inclusive.".into());
    }

    if cumulative {
        // Cumulative mode: sum probabilities from 0 to `failures`
        let mut cumulative_probability = 0.0;
        for k in 0..=failures {
            cumulative_probability += neg_binom_pmf(k as u32, successes as u32, probability)?;
        }
        Ok(cumulative_probability)
    } else {
        // Probability mass function for the exact number of failures
        neg_binom_pmf(failures as u32, successes as u32, probability)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neg_binom_dot_dist_pmf_basic() {
        // =NEGBINOM.DIST(10, 5, 0.5, FALSE) in US format
        // =NEGBINOM.DIST(10; 5; 0,5; FALSE) in German format
        let result = codcel_neg_binom_dot_dist(10, 5, 0.5, false).unwrap();
        println!("{result}");
        assert!((result - 0.030548095703125).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dot_dist_cdf_basic() {
        // =NEGBINOM.DIST(10, 5, 0.5, TRUE) in US format
        // =NEGBINOM.DIST(10; 5; 0,5; TRUE) in German format
        let result = codcel_neg_binom_dot_dist(10, 5, 0.5, true).unwrap();
        println!("{result}");
        assert!((result - 0.940765380859375).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dot_dist_pmf_zero_failures() {
        // =NEGBINOM.DIST(0, 5, 0.5, FALSE) in US format
        // =NEGBINOM.DIST(0; 5; 0,5; FALSE) in German format
        let result = codcel_neg_binom_dot_dist(0, 5, 0.5, false).unwrap();
        assert!((result - 0.0313).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dot_dist_cdf_zero_failures() {
        // =NEGBINOM.DIST(0, 5, 0.5, TRUE) in US format
        // =NEGBINOM.DIST(0; 5; 0,5; TRUE) in German format
        let result = codcel_neg_binom_dot_dist(0, 5, 0.5, true).unwrap();
        assert!((result - 0.0313).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dot_dist_pmf_high_probability() {
        // =NEGBINOM.DIST(3, 10, 0.9, FALSE) in US format
        // =NEGBINOM.DIST(3; 10; 0,9; FALSE) in German format
        let result = codcel_neg_binom_dot_dist(3, 10, 0.9, false).unwrap();
        println!("{result}");
        assert!((result - 0.07670925682199999).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dot_dist_cdf_high_probability() {
        // =NEGBINOM.DIST(3, 10, 0.9, TRUE) in US format
        // =NEGBINOM.DIST(3; 10; 0,9; TRUE) in German format
        let result = codcel_neg_binom_dot_dist(3, 10, 0.9, true).unwrap();
        println!("{result}");
        assert!((result - 0.9658392790770002).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dot_dist_pmf_low_probability() {
        // =NEGBINOM.DIST(15, 5, 0.1, FALSE) in US format
        // =NEGBINOM.DIST(15; 5; 0,1; FALSE) in German format
        let result = codcel_neg_binom_dot_dist(15, 5, 0.1, false).unwrap();
        println!("{result}");
        assert!((result - 0.007980340279988604).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dot_dist_cdf_low_probability() {
        // =NEGBINOM.DIST(15, 5, 0.1, TRUE) in US format
        // =NEGBINOM.DIST(15; 5; 0,1; TRUE) in German format
        let result = codcel_neg_binom_dot_dist(15, 5, 0.1, true).unwrap();
        println!("{result}");
        assert!((result - 0.04317449528446343).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dot_dist_probability_zero() {
        // =NEGBINOM.DIST(10, 5, 0, FALSE) in US format
        // =NEGBINOM.DIST(10; 5; 0; FALSE) in German format
        let result = codcel_neg_binom_dot_dist(10, 5, 0.0, false).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dot_dist_probability_one() {
        // =NEGBINOM.DIST(0, 5, 1, FALSE) in US format
        // =NEGBINOM.DIST(0; 5; 1; FALSE) in German format
        let result = codcel_neg_binom_dot_dist(0, 5, 1.0, false).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_neg_binom_dot_dist_probability_out_of_range() {
        // Probability outside [0,1] should return an error
        let result = codcel_neg_binom_dot_dist(10, 5, 1.5, false);
        assert!(result.is_err());
    }
}
