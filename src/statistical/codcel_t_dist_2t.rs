// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `T.DIST.2T` that returns the two-tailed Student's t-distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `degrees_freedom`: degrees of freedom (must be > 0).
///
/// Returns the two-tailed probability (area in both tails beyond ±x),
/// or an error when inputs are outside the allowed range.
pub fn codcel_t_dist_2t(x: f64, degrees_freedom: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if degrees_freedom <= 0.0 {
        return Err("T.DIST.2T: Degrees of freedom must be greater than 0.".into());
    }
    if x < 0.0 {
        return Err("T.DIST.2T: x must be non-negative.".into());
    }

    // Create a t-distribution
    let t_distribution = statrs::distribution::StudentsT::new(0.0, 1.0, degrees_freedom)?;

    // Calculate cumulative probability for the two-tailed t-distribution
    let p = (1.0 - t_distribution.cdf(x)) * 2.0;

    Ok(p)
}

pub fn codcel_t_dist_2t_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("T.DIST.2T: Must have 2 parameters.".into());
    }

    codcel_t_dist_2t(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_dist_2t_basic() {
        // =T.DIST.2T(2, 10) in US format
        // =T.DIST.2T(2; 10) in German format
        let result = codcel_t_dist_2t(2.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 0.0733880347707403).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_2t_large_df() {
        // =T.DIST.2T(1.96, 1000) in US format
        // =T.DIST.2T(1,96; 1000) in German format
        let result = codcel_t_dist_2t(1.96, 1000.0).unwrap();
        println!("{result}");
        assert!((result - 0.05027318495575561).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_2t_small_df() {
        // =T.DIST.2T(1, 1) in US format
        // =T.DIST.2T(1; 1) in German format
        let result = codcel_t_dist_2t(1.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_2t_zero_x() {
        // =T.DIST.2T(0, 5) in US format
        // =T.DIST.2T(0; 5) in German format
        let result = codcel_t_dist_2t(0.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_2t_large_x() {
        // =T.DIST.2T(10, 5) in US format
        // =T.DIST.2T(10; 5) in German format
        let result = codcel_t_dist_2t(10.0, 5.0).unwrap();
        assert!((result - 0.0002).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_2t_negative_x() {
        // Negative x should return an error
        let result = codcel_t_dist_2t(-1.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dist_2t_zero_df() {
        // Zero degrees of freedom should return an error
        let result = codcel_t_dist_2t(2.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dist_2t_negative_df() {
        // Negative degrees of freedom should return an error
        let result = codcel_t_dist_2t(2.0, -5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dist_2t_vec_basic() {
        // =T.DIST.2T(2, 10) in US format
        // =T.DIST.2T(2; 10) in German format
        let inputs = vec![2.0, 10.0];
        let result = codcel_t_dist_2t_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 0.0733880347707403).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_2t_vec_wrong_length() {
        // Wrong number of parameters should return an error
        let inputs = vec![2.0, 10.0, 5.0];
        let result = codcel_t_dist_2t_vec(inputs);
        assert!(result.is_err());
    }
}
