// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::factorial::factorial;
use std::error::Error;

/// Excel-compatible `POISSON`/`POISSON.DIST` function.
/// Evaluates the Poisson distribution.
/// - `x`: number of events (must be non-negative).
/// - `mean`: expected number of events (must be non-negative).
/// - `cumulative`: `true` for cumulative probability `P(X <= x)`, `false` for probability mass `P(X = x)`.
///
/// Returns an error on negative counts or negative mean.
pub fn codcel_poisson(
    x: i32,
    mean: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x < 0 {
        return Err("POISSON: x must be non-negative.".into());
    }

    if mean < 0.0 {
        return Err("POISSON: mean must be non-negative.".into());
    }

    if cumulative {
        // Calculate cumulative probability: P(X <= x)
        let mut cumulative_prob = 0.0;
        for i in 0..=x {
            cumulative_prob += crate::portable_math::exp(-mean) * mean.powi(i) / factorial(i);
        }
        Ok(cumulative_prob)
    } else {
        // Calculate probability mass function: P(X = x)
        let prob = crate::portable_math::exp(-mean) * mean.powi(x) / factorial(x);
        Ok(prob)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poisson_pmf_basic() {
        // =POISSON(2, 3, FALSE) in US format
        // =POISSON(2; 3; FALSE) in German format
        let result = codcel_poisson(2, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.2240418).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_cdf_basic() {
        // =POISSON(2, 3, TRUE) in US format
        // =POISSON(2; 3; TRUE) in German format
        let result = codcel_poisson(2, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.4231901).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_pmf_zero_x() {
        // =POISSON(0, 3, FALSE) in US format
        // =POISSON(0; 3; FALSE) in German format
        let result = codcel_poisson(0, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.0497871).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_cdf_zero_x() {
        // =POISSON(0, 3, TRUE) in US format
        // =POISSON(0; 3; TRUE) in German format
        let result = codcel_poisson(0, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.0497871).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_pmf_large_x() {
        // =POISSON(10, 3, FALSE) in US format
        // =POISSON(10; 3; FALSE) in German format
        let result = codcel_poisson(10, 3.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.0008102).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_cdf_large_x() {
        // =POISSON(10, 3, TRUE) in US format
        // =POISSON(10; 3; TRUE) in German format
        let result = codcel_poisson(10, 3.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.9997077).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_different_mean() {
        // =POISSON(5, 5, FALSE) in US format
        // =POISSON(5; 5; FALSE) in German format
        let result = codcel_poisson(5, 5.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.1755280).abs() < 0.0001);
    }

    #[test]
    fn test_poisson_negative_x() {
        // =POISSON(-1, 3, TRUE) in US format
        // =POISSON(-1; 3; TRUE) in German format
        let result = codcel_poisson(-1, 3.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_poisson_negative_mean() {
        // =POISSON(2, -3, TRUE) in US format
        // =POISSON(2; -3; TRUE) in German format
        let result = codcel_poisson(2, -3.0, true);
        assert!(result.is_err());
    }
}
