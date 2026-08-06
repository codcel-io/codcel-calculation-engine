// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `T.DIST.RT` that returns the right-tailed Student's t-distribution.
/// - `x`: the value at which to evaluate the distribution.
/// - `degrees_freedom`: degrees of freedom (must be > 0).
///
/// Returns the right-tailed probability (1 - CDF),
/// or an error when inputs are outside the allowed range.
pub fn codcel_t_dist_rt(x: f64, degrees_freedom: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if degrees_freedom <= 0.0 {
        return Err("T.DIST.RT: Degrees of freedom must be greater than 0.".into());
    }
    if x < 0.0 {
        return Err("T.DIST.RT: x must be non-negative.".into());
    }

    // Create a t-distribution
    let t_distribution = statrs::distribution::StudentsT::new(0.0, 1.0, degrees_freedom)?;

    // Calculate the right-tailed probability
    let p = 1.0 - t_distribution.cdf(x);

    Ok(p)
}

pub fn codcel_t_dist_rt_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("T.DIST.RT: Must have 2 parameters.".into());
    }

    codcel_t_dist_rt(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_dist_rt_basic() {
        // =T.DIST.RT(2, 10) in US format
        // =T.DIST.RT(2; 10) in German format
        let result = codcel_t_dist_rt(2.0, 10.0).unwrap();
        assert!((result - 0.0366).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_rt_large_df() {
        // =T.DIST.RT(1.96, 1000) in US format
        // =T.DIST.RT(1,96; 1000) in German format
        let result = codcel_t_dist_rt(1.96, 1000.0).unwrap();
        println!("{result}");
        assert!((result - 0.025136592477877806).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_rt_small_df() {
        // =T.DIST.RT(1, 1) in US format
        // =T.DIST.RT(1; 1) in German format
        let result = codcel_t_dist_rt(1.0, 1.0).unwrap();
        assert!((result - 0.25).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_rt_zero_x() {
        // =T.DIST.RT(0, 5) in US format
        // =T.DIST.RT(0; 5) in German format
        let result = codcel_t_dist_rt(0.0, 5.0).unwrap();
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_rt_large_x() {
        // =T.DIST.RT(10, 5) in US format
        // =T.DIST.RT(10; 5) in German format
        let result = codcel_t_dist_rt(10.0, 5.0).unwrap();
        assert!((result - 0.0001).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_rt_negative_x() {
        // Negative x should return an error
        let result = codcel_t_dist_rt(-1.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dist_rt_zero_df() {
        // Zero degrees of freedom should return an error
        let result = codcel_t_dist_rt(2.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dist_rt_negative_df() {
        // Negative degrees of freedom should return an error
        let result = codcel_t_dist_rt(2.0, -5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dist_rt_vec_basic() {
        // =T.DIST.RT(2, 10) in US format
        // =T.DIST.RT(2; 10) in German format
        let inputs = vec![2.0, 10.0];
        let result = codcel_t_dist_rt_vec(inputs).unwrap();
        assert!((result - 0.0366).abs() < 0.0001);
    }

    #[test]
    fn test_t_dist_rt_vec_wrong_length() {
        // Wrong number of parameters should return an error
        let inputs = vec![2.0, 10.0, 5.0];
        let result = codcel_t_dist_rt_vec(inputs);
        assert!(result.is_err());
    }
}
