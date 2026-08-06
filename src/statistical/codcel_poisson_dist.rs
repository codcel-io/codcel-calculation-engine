// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::poisson_probability::poisson_probability;
use std::error::Error;

/// Excel-compatible `POISSON.DIST` that returns the Poisson distribution.
/// - `x`: the number of events (must be >= 0).
/// - `mean`: the expected number of events (must be >= 0).
/// - `cumulative`: if `true`, returns the cumulative distribution function;
///   if `false`, returns the probability mass function.
///
/// Returns the probability or an error when inputs are outside the allowed range.
pub fn codcel_poisson_dist(
    x: i32,
    mean: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x < 0 {
        return Err("POISSON.DIST: x must be non-negative.".into());
    }

    if mean < 0.0 {
        return Err("POISSON.DIST: mean must be non-negative.".into());
    }

    if cumulative {
        // Calculate the cumulative probability: P(X <= x)
        let mut cumulative_prob = 0.0;
        for i in 0..=x {
            cumulative_prob += poisson_probability(i, mean);
        }
        Ok(cumulative_prob)
    } else {
        // Calculate the probability mass function: P(X = x)
        Ok(poisson_probability(x, mean))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poisson_dist_pmf_basic() {
        // =POISSON.DIST(2, 3, FALSE) in US format
        // =POISSON.DIST(2; 3; FALSE) in German format
        let result = codcel_poisson_dist(2, 3.0, false).unwrap();
        assert!((result - 0.22404180765538775).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_dist_cdf_basic() {
        // =POISSON.DIST(2, 3, TRUE) in US format
        // =POISSON.DIST(2; 3; TRUE) in German format
        let result = codcel_poisson_dist(2, 3.0, true).unwrap();
        assert!((result - 0.42319008112684353).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_dist_pmf_zero_x() {
        // =POISSON.DIST(0, 3, FALSE) in US format
        // =POISSON.DIST(0; 3; FALSE) in German format
        let result = codcel_poisson_dist(0, 3.0, false).unwrap();
        assert!((result - 0.04978706836786395).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_dist_cdf_zero_x() {
        // =POISSON.DIST(0, 3, TRUE) in US format
        // =POISSON.DIST(0; 3; TRUE) in German format
        let result = codcel_poisson_dist(0, 3.0, true).unwrap();
        assert!((result - 0.04978706836786395).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_dist_pmf_large_x() {
        // =POISSON.DIST(10, 3, FALSE) in US format
        // =POISSON.DIST(10; 3; FALSE) in German format
        let result = codcel_poisson_dist(10, 3.0, false).unwrap();
        assert!((result - 0.0008101512996502705).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_dist_cdf_large_x() {
        // =POISSON.DIST(10, 3, TRUE) in US format
        // =POISSON.DIST(10; 3; TRUE) in German format
        let result = codcel_poisson_dist(10, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.9997076630493528).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_dist_negative_x() {
        // Negative x should return an error
        let result = codcel_poisson_dist(-1, 3.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_poisson_dist_negative_mean() {
        // Negative mean should return an error
        let result = codcel_poisson_dist(2, -3.0, true);
        assert!(result.is_err());
    }
}
