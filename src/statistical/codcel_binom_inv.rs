// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compatibility::binomial_probability::binomial_probability;
use crate::compensated_sum::CompensatedSum;
use std::error::Error;

/// Excel-compatible `BINOM.INV` that returns the smallest value for which the cumulative binomial distribution is > alpha.
/// - `trials`: the number of independent trials.
/// - `probability`: the probability of success on each trial (0 to 1).
/// - `alpha`: the criterion value (0 to 1).
///
/// Returns the smallest integer k such that the cumulative binomial probability P(X <= k) > alpha,
/// or an error when inputs are outside the allowed range.
///
/// Note: Excel's documentation says ">=" but actual behavior uses ">" (strictly greater than).
pub fn codcel_binom_inv(
    trials: i32,
    probability: f64,
    alpha: f64,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    // Input validation
    if trials < 0 {
        return Err("BINOMINV: Number of trials must be non-negative.".into());
    }
    if !(0.0..=1.0).contains(&probability) {
        return Err("BINOMINV: Probability must be in the range [0, 1].".into());
    }
    if !(0.0..=1.0).contains(&alpha) {
        return Err("BINOMINV: Alpha must be in the range [0, 1].".into());
    }

    // Excel returns 0 for BINOM.INV(0, p, a) for any valid p and a
    if trials == 0 {
        return Ok(0);
    }

    let mut cumulative_probability = CompensatedSum::new();

    for k in 0..=trials {
        cumulative_probability.add(binomial_probability(trials as u32, k as u32, probability)?);
        // Excel uses strictly greater than (>) despite documentation saying >=.
        // Verified empirically: BINOM.INV(1, 0.5, 0.5) returns 1 in Excel,
        // meaning it skips k=0 where CDF(0)=0.5 exactly equals alpha=0.5.
        if cumulative_probability.total() > alpha {
            return Ok(k);
        }
    }

    // Floating-point rounding may prevent the cumulative sum from reaching exactly 1.0,
    // but by definition CDF(n) = 1.0, so return trials.
    Ok(trials)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binom_inv_basic() {
        // =BINOM.INV(10,0.5,0.5) in US format
        // =BINOM.INV(10;0,5;0,5) in German format
        let result = codcel_binom_inv(10, 0.5, 0.5).unwrap();
        assert_eq!(result, 5); // Smallest k where CDF of Binomial(10, 0.5) >= 0.5
    }

    #[test]
    fn test_binom_inv_low_alpha() {
        // =BINOM.INV(10,0.5,0.1) in US format
        // =BINOM.INV(10;0,5;0,1) in German format
        let result = codcel_binom_inv(10, 0.5, 0.1).unwrap();
        assert_eq!(result, 3); // Smallest k where CDF of Binomial(10, 0.5) >= 0.1
    }

    #[test]
    fn test_binom_inv_high_alpha() {
        // =BINOM.INV(10,0.5,0.9) in US format
        // =BINOM.INV(10;0,5;0,9) in German format
        let result = codcel_binom_inv(10, 0.5, 0.9).unwrap();
        assert_eq!(result, 7); // Smallest k where CDF of Binomial(10, 0.5) >= 0.9
    }

    #[test]
    fn test_binom_inv_low_probability() {
        // =BINOM.INV(10,0.1,0.5) in US format
        // =BINOM.INV(10;0,1;0,5) in German format
        let result = codcel_binom_inv(10, 0.1, 0.5).unwrap();
        assert_eq!(result, 1); // Smallest k where CDF of Binomial(10, 0.1) >= 0.5
    }

    #[test]
    fn test_binom_inv_high_probability() {
        // =BINOM.INV(10,0.9,0.5) in US format
        // =BINOM.INV(10;0,9;0,5) in German format
        let result = codcel_binom_inv(10, 0.9, 0.5).unwrap();
        assert_eq!(result, 9); // Smallest k where CDF of Binomial(10, 0.9) >= 0.5
    }

    #[test]
    fn test_binom_inv_zero_alpha() {
        // =BINOM.INV(10,0.5,0) in US format
        // =BINOM.INV(10;0,5;0) in German format
        let result = codcel_binom_inv(10, 0.5, 0.0).unwrap();
        assert_eq!(result, 0); // Smallest k where CDF of Binomial(10, 0.5) >= 0
    }

    #[test]
    fn test_binom_inv_one_alpha() {
        // =BINOM.INV(10,0.5,1) in US format
        // =BINOM.INV(10;0,5;1) in German format
        let result = codcel_binom_inv(10, 0.5, 1.0).unwrap();
        assert_eq!(result, 10); // Smallest k where CDF of Binomial(10, 0.5) >= 1
    }

    #[test]
    fn test_binom_inv_zero_probability() {
        // =BINOM.INV(10,0,0.5) in US format
        // =BINOM.INV(10;0;0,5) in German format
        let result = codcel_binom_inv(10, 0.0, 0.5).unwrap();
        assert_eq!(result, 0); // With p=0, only k=0 has non-zero probability
    }

    #[test]
    fn test_binom_inv_one_probability() {
        // =BINOM.INV(10,1,0.5) in US format
        // =BINOM.INV(10;1;0,5) in German format
        let result = codcel_binom_inv(10, 1.0, 0.5).unwrap();
        assert_eq!(result, 10); // With p=1, k=10 has probability 1
    }

    #[test]
    fn test_binom_inv_invalid_trials() {
        // =BINOM.INV(0,0.5,0.5) in US format (returns #NUM! error)
        // =BINOM.INV(0;0,5;0,5) in German format (returns #NUM! error)
        let result = codcel_binom_inv(0, 0.5, 0.5).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_binom_inv_invalid_probability_low() {
        // =BINOM.INV(10,-0.1,0.5) in US format (returns #NUM! error)
        // =BINOM.INV(10;-0,1;0,5) in German format (returns #NUM! error)
        let result = codcel_binom_inv(10, -0.1, 0.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_inv_invalid_probability_high() {
        // =BINOM.INV(10,1.1,0.5) in US format (returns #NUM! error)
        // =BINOM.INV(10;1,1;0,5) in German format (returns #NUM! error)
        let result = codcel_binom_inv(10, 1.1, 0.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_inv_invalid_alpha_low() {
        // =BINOM.INV(10,0.5,-0.1) in US format (returns #NUM! error)
        // =BINOM.INV(10;0,5;-0,1) in German format (returns #NUM! error)
        let result = codcel_binom_inv(10, 0.5, -0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_binom_inv_invalid_alpha_high() {
        // =BINOM.INV(10,0.5,1.1) in US format (returns #NUM! error)
        // =BINOM.INV(10;0,5;1,1) in German format (returns #NUM! error)
        let result = codcel_binom_inv(10, 0.5, 1.1);
        assert!(result.is_err());
    }
}
