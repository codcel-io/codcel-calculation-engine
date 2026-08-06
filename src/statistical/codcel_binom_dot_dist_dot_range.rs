// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::binomial_pmf::binomial_pmf;
use std::error::Error;

/// Excel-compatible `BINOM.DIST.RANGE` that returns the probability of a trial result using a binomial distribution.
/// - `trials`: the number of independent trials.
/// - `probability`: the probability of success on each trial (0 to 1).
/// - `number_s`: the minimum number of successes in trials.
/// - `number_s2`: optional maximum number of successes (defaults to `number_s` for exact count).
///
/// Returns the probability that the number of successful trials falls between `number_s` and `number_s2`,
/// or an error when inputs are outside the allowed range.
pub fn codcel_binom_dot_dist_dot_range(
    trials: i32,
    probability: f64,
    number_s: i32,
    number_s2: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if trials < 0 {
        return Err("BINOM.DIST.RANGE: Number of trials must be non-negative".into());
    }

    if number_s < 0 || number_s > trials {
        return Err(
            "BINOM.DIST.RANGE: Lower bound of successes must be between 0 and the number of trials"
                .into(),
        );
    }

    let upper_bound = number_s2.unwrap_or(number_s);

    if upper_bound < number_s || upper_bound > trials {
        return Err("BINOM.DIST.RANGE: Upper bound of successes must be between the lower bound and the number of trials".into());
    }

    if !(0.0..=1.0).contains(&probability) {
        return Err("BINOM.DIST.RANGE: Probability must be between 0 and 1".into());
    }

    // Calculate the sum of probabilities for all successes in the range
    let mut result = 0.0;
    for successes in number_s..=upper_bound {
        result += binomial_pmf(successes, trials, probability)?;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binom_dist_range_single_success() {
        // =BINOM.DIST.RANGE(10,0.5,6) in US format
        // =BINOM.DIST.RANGE(10;0,5;6) in German format
        let result = codcel_binom_dot_dist_dot_range(10, 0.5, 6, None).unwrap();
        assert!((result - 0.205078125).abs() < 1e-10); // PMF of Binomial(10, 0.5) at x=6
    }

    #[test]
    fn test_binom_dist_range_multiple_successes() {
        // =BINOM.DIST.RANGE(10,0.5,4,6) in US format
        // =BINOM.DIST.RANGE(10;0,5;4;6) in German format
        let result = codcel_binom_dot_dist_dot_range(10, 0.5, 4, Some(6)).unwrap();
        assert!((result - 0.65625).abs() < 1e-10); // Sum of PMF of Binomial(10, 0.5) for x=4,5,6
    }

    #[test]
    fn test_binom_dist_range_all_successes() {
        // =BINOM.DIST.RANGE(10,0.5,0,10) in US format
        // =BINOM.DIST.RANGE(10;0,5;0;10) in German format
        let result = codcel_binom_dot_dist_dot_range(10, 0.5, 0, Some(10)).unwrap();
        assert!((result - 1.0).abs() < 1e-12); // Sum of all probabilities should be 1
    }

    #[test]
    fn test_binom_dist_range_zero_probability() {
        // =BINOM.DIST.RANGE(10,0,5) in US format
        // =BINOM.DIST.RANGE(10;0;5) in German format
        let result = codcel_binom_dot_dist_dot_range(10, 0.0, 5, None).unwrap();
        assert!((result - 0.0).abs() < 1e-12); // Probability of 5 successes with p=0 is 0
    }

    #[test]
    fn test_binom_dist_range_certainty() {
        // =BINOM.DIST.RANGE(10,1,10) in US format
        // =BINOM.DIST.RANGE(10;1;10) in German format
        let result = codcel_binom_dot_dist_dot_range(10, 1.0, 10, None).unwrap();
        println!("{result:?}");
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_binom_dist_range_low_probability() {
        // =BINOM.DIST.RANGE(20,0.1,0,2) in US format
        // =BINOM.DIST.RANGE(20;0,1;0;2) in German format
        let result = codcel_binom_dot_dist_dot_range(20, 0.1, 0, Some(2)).unwrap();
        println!("{result:?}");
        assert!((result - 0.6769268051894661).abs() < 1e-10); // Sum of PMF of Binomial(20, 0.1) for x=0,1,2
    }

    #[test]
    fn test_binom_dist_range_high_probability() {
        // =BINOM.DIST.RANGE(20,0.9,18,20) in US format
        // =BINOM.DIST.RANGE(20;0,9;18;20) in German format
        let result = codcel_binom_dot_dist_dot_range(20, 0.9, 18, Some(20)).unwrap();
        assert!((result - 0.6769268051894658).abs() < 1e-10); // Sum of PMF of Binomial(20, 0.9) for x=18,19,20
    }

    #[test]
    fn test_binom_dist_range_zero_trials() {
        // =BINOM.DIST.RANGE(0,0.5,0) in US format
        // =BINOM.DIST.RANGE(0;0,5;0) in German format
        let result = codcel_binom_dot_dist_dot_range(0, 0.5, 0, None).unwrap();
        assert_eq!(result, 1.0); // Probability of 0 successes with 0 trials is 1
    }

    #[test]
    fn test_binom_dist_range_invalid_trials() {
        // =BINOM.DIST.RANGE(-1,0.5,0) in US format (returns #NUM! error)
        // =BINOM.DIST.RANGE(-1;0,5;0) in German format (returns #NUM! error)
        let result = codcel_binom_dot_dist_dot_range(-1, 0.5, 0, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_dist_range_invalid_lower_bound() {
        // =BINOM.DIST.RANGE(10,0.5,-1) in US format (returns #NUM! error)
        // =BINOM.DIST.RANGE(10;0,5;-1) in German format (returns #NUM! error)
        let result = codcel_binom_dot_dist_dot_range(10, 0.5, -1, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_dist_range_invalid_upper_bound() {
        // =BINOM.DIST.RANGE(10,0.5,5,11) in US format (returns #NUM! error)
        // =BINOM.DIST.RANGE(10;0,5;5;11) in German format (returns #NUM! error)
        let result = codcel_binom_dot_dist_dot_range(10, 0.5, 5, Some(11));
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_dist_range_invalid_probability_low() {
        // =BINOM.DIST.RANGE(10,-0.1,5) in US format (returns #NUM! error)
        // =BINOM.DIST.RANGE(10;-0,1;5) in German format (returns #NUM! error)
        let result = codcel_binom_dot_dist_dot_range(10, -0.1, 5, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_dist_range_invalid_probability_high() {
        // =BINOM.DIST.RANGE(10,1.1,5) in US format (returns #NUM! error)
        // =BINOM.DIST.RANGE(10;1,1;5) in German format (returns #NUM! error)
        let result = codcel_binom_dot_dist_dot_range(10, 1.1, 5, None);
        assert!(result.is_err());
    }
}
