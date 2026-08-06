// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `F.DIST.RT` that returns the right-tailed F probability distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `df1`: numerator degrees of freedom (must be > 0).
/// - `df2`: denominator degrees of freedom (must be > 0).
///
/// Returns the right-tailed probability (1 - CDF) or an error when inputs are outside the allowed range.
pub fn codcel_f_dist_rt(x: f64, df1: f64, df2: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if x < 0.0 {
        return Err("F.DIST.RT: x must be non-negative.".into());
    }
    if df1 <= 0.0 {
        return Err("F.DIST.RT: df1 (numerator degrees of freedom) must be greater than 0.".into());
    }
    if df2 <= 0.0 {
        return Err(
            "F.DIST.RT: df2 (denominator degrees of freedom) must be greater than 0.".into(),
        );
    }

    // Calculate the F distribution
    let f_dist = statrs::distribution::FisherSnedecor::new(df1, df2)
        .map_err(|_| "F.DIST.RT: Error creating F distribution.")?;

    // Right-tailed probability (1 - CDF)
    Ok(1.0 - f_dist.cdf(x))
}

pub fn codcel_f_dist_rt_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("F.DIST.RT: Must have 3 parameters".into());
    }

    codcel_f_dist_rt(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_dist_rt_basic() {
        // =F.DIST.RT(2, 5, 10) in US format
        // =F.DIST.RT(2; 5; 10) in German format
        let result = codcel_f_dist_rt(2.0, 5.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 0.164195).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dist_rt_equal_df() {
        // =F.DIST.RT(2, 5, 5) in US format
        // =F.DIST.RT(2; 5; 5) in German format
        let result = codcel_f_dist_rt(2.0, 5.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 0.23251131913037815).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dist_rt_large_df() {
        // =F.DIST.RT(2, 50, 100) in US format
        // =F.DIST.RT(2; 50; 100) in German format
        let result = codcel_f_dist_rt(2.0, 50.0, 100.0).unwrap();
        println!("{result}");
        assert!((result - 0.001686116667074078).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dist_rt_small_x() {
        // =F.DIST.RT(0.5, 5, 10) in US format
        // =F.DIST.RT(0,5; 5; 10) in German format
        let result = codcel_f_dist_rt(0.5, 5.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 0.7700248806501028).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dist_rt_large_x() {
        // =F.DIST.RT(10, 5, 10) in US format
        // =F.DIST.RT(10; 5; 10) in German format
        let result = codcel_f_dist_rt(10.0, 5.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 0.0012057806486995837).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dist_rt_zero_x() {
        // =F.DIST.RT(0, 5, 10) in US format
        // =F.DIST.RT(0; 5; 10) in German format
        let result = codcel_f_dist_rt(0.0, 5.0, 10.0).unwrap();
        assert!((result - 1.0).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dist_rt_small_df1() {
        // =F.DIST.RT(2, 1, 10) in US format
        // =F.DIST.RT(2; 1; 10) in German format
        let result = codcel_f_dist_rt(2.0, 1.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 0.18766987086960307).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dist_rt_small_df2() {
        // =F.DIST.RT(2, 5, 1) in US format
        // =F.DIST.RT(2; 5; 1) in German format
        let result = codcel_f_dist_rt(2.0, 5.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 0.488916).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dist_rt_negative_x() {
        // Negative x should return an error
        let result = codcel_f_dist_rt(-1.0, 5.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dist_rt_zero_df1() {
        // Zero df1 should return an error
        let result = codcel_f_dist_rt(2.0, 0.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dist_rt_zero_df2() {
        // Zero df2 should return an error
        let result = codcel_f_dist_rt(2.0, 5.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dist_rt_negative_df1() {
        // Negative df1 should return an error
        let result = codcel_f_dist_rt(2.0, -5.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dist_rt_negative_df2() {
        // Negative df2 should return an error
        let result = codcel_f_dist_rt(2.0, 5.0, -10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dist_rt_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![2.0, 5.0, 10.0];
        let result = codcel_f_dist_rt_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 0.164194951).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dist_rt_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![2.0, 5.0];
        let result = codcel_f_dist_rt_vec(inputs);
        assert!(result.is_err());
    }
}
