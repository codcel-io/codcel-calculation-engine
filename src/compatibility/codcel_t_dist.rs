// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `TDIST`/`T.DIST.RT` function.
/// Returns the right-tailed Student's t distribution.
/// - `x`: t-statistic value (must be non-negative).
/// - `degrees_freedom`: degrees of freedom (must be greater than 0).
/// - `tails`: 1 for one-tailed, 2 for two-tailed (result is doubled).
///
/// Returns an error on non-positive degrees of freedom, invalid tails, or negative `x`.
pub fn codcel_t_dist(
    x: f64,
    degrees_freedom: f64,
    tails: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if degrees_freedom <= 0.0 {
        return Err("TDIST: Degrees of freedom must be greater than 0.".into());
    }
    if tails != 1 && tails != 2 {
        return Err("TDIST: Tails parameter must be 1 or 2.".into());
    }
    if x < 0.0 {
        return Err("TDIST: x must be non-negative.".into());
    }

    // Create a t-distribution
    let t_distribution = statrs::distribution::StudentsT::new(0.0, 1.0, degrees_freedom)?;

    // Calculate the cumulative probability
    let p = 1.0 - t_distribution.cdf(x);

    // Adjust for one-tailed or two-tailed
    match tails {
        1 => Ok(p),
        2 => Ok(p * 2.0),
        _ => unreachable!(), // Should not be reached due to the earlier check
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_dist_one_tail() {
        // =TDIST(2, 10, 1) in US format
        // =TDIST(2; 10; 1) in German format
        let result = codcel_t_dist(2.0, 10.0, 1).unwrap();
        println!("{result}");
        assert!((result - 0.0367).abs() < 0.001);
    }

    #[test]
    fn test_t_dist_two_tail() {
        // =TDIST(2, 10, 2) in US format
        // =TDIST(2; 10; 2) in German format
        let result = codcel_t_dist(2.0, 10.0, 2).unwrap();
        println!("{result}");
        assert!((result - 0.0734).abs() < 0.001);
    }

    #[test]
    fn test_t_dist_different_df() {
        // =TDIST(2, 20, 1) in US format
        // =TDIST(2; 20; 1) in German format
        let result = codcel_t_dist(2.0, 20.0, 1).unwrap();
        println!("{result}");
        assert!((result - 0.0297).abs() < 0.001);
    }

    #[test]
    fn test_t_dist_zero_x() {
        // =TDIST(0, 10, 1) in US format
        // =TDIST(0; 10; 1) in German format
        let result = codcel_t_dist(0.0, 10.0, 1).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_t_dist_large_x() {
        // =TDIST(5, 10, 1) in US format
        // =TDIST(5; 10; 1) in German format
        let result = codcel_t_dist(5.0, 10.0, 1).unwrap();
        println!("{result}");
        assert!((result - 0.0003).abs() < 0.001);
    }

    #[test]
    fn test_t_dist_negative_x() {
        // =TDIST(-1, 10, 1) in US format
        // =TDIST(-1; 10; 1) in German format
        let result = codcel_t_dist(-1.0, 10.0, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dist_zero_df() {
        // =TDIST(2, 0, 1) in US format
        // =TDIST(2; 0; 1) in German format
        let result = codcel_t_dist(2.0, 0.0, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dist_invalid_tails() {
        // =TDIST(2, 10, 3) in US format
        // =TDIST(2; 10; 3) in German format
        let result = codcel_t_dist(2.0, 10.0, 3);
        assert!(result.is_err());
    }
}
