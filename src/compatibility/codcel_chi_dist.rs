// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::{ChiSquared, ContinuousCDF};
use std::error::Error;

/// Excel-compatible `CHIDIST`/`CHI.DIST.RT` function.
/// Returns the right-tailed probability of the chi-squared distribution.
/// - `x`: chi-squared statistic value (must be non-negative).
/// - `df`: degrees of freedom (must be greater than 0).
///
/// Returns an error on negative `x` or non-positive degrees of freedom.
pub fn codcel_chi_dist(x: f64, df: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x < 0.0 {
        return Err("CHIDIST: Chi-squared value (x) must be non-negative".into());
    }
    if df <= 0.0 {
        return Err("CHIDIST: Degrees of freedom (df) must be greater than 0".into());
    }

    let chi_squared = ChiSquared::new(df).map_err(|_| "CHIDIST: Invalid degrees of freedom")?;
    let p_value = 1.0 - chi_squared.cdf(x);

    Ok(p_value)
}

/// Convenience wrapper for `CHIDIST` that accepts `[x, df]` in a vector.
/// Errors if the vector does not contain exactly two values.
pub fn codcel_chi_dist_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("CHIDIST: Must have 2 parameters.".into());
    }

    codcel_chi_dist(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chi_dist_basic() {
        // =CHIDIST(2, 3) in US format
        // =CHIDIST(2; 3) in German format
        let result = codcel_chi_dist(2.0, 3.0).unwrap();
        println!("{result}");
        assert!((result - 0.5724067044708798).abs() < 0.0001);
    }

    #[test]
    fn test_chi_dist_small_x() {
        // =CHIDIST(0.5, 3) in US format
        // =CHIDIST(0,5; 3) in German format
        let result = codcel_chi_dist(0.5, 3.0).unwrap();
        println!("{result}");
        assert!((result - 0.918891411654676).abs() < 0.0001);
    }

    #[test]
    fn test_chi_dist_large_x() {
        // =CHIDIST(10, 3) in US format
        // =CHIDIST(10; 3) in German format
        let result = codcel_chi_dist(10.0, 3.0).unwrap();
        println!("{result}");
        assert!((result - 0.01857594393206).abs() < 0.0001);
    }

    #[test]
    fn test_chi_dist_small_df() {
        // =CHIDIST(2, 1) in US format
        // =CHIDIST(2; 1) in German format
        let result = codcel_chi_dist(2.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 0.15729920705028513).abs() < 0.0001);
    }

    #[test]
    fn test_chi_dist_large_df() {
        // =CHIDIST(2, 10) in US format
        // =CHIDIST(2; 10) in German format
        let result = codcel_chi_dist(2.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 0.9963401531726563).abs() < 0.0001);
    }

    #[test]
    fn test_chi_dist_zero_x() {
        // =CHIDIST(0, 3) in US format
        // =CHIDIST(0; 3) in German format
        let result = codcel_chi_dist(0.0, 3.0).unwrap();
        println!("{result}");
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_chi_dist_negative_x() {
        // Negative x should return an error
        let result = codcel_chi_dist(-1.0, 3.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chi_dist_zero_df() {
        // Zero degrees of freedom should return an error
        let result = codcel_chi_dist(2.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chi_dist_negative_df() {
        // Negative degrees of freedom should return an error
        let result = codcel_chi_dist(2.0, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chi_dist_vec_basic() {
        // =CHIDIST(2, 3) in US format
        // =CHIDIST(2; 3) in German format
        let result = codcel_chi_dist_vec(vec![2.0, 3.0]).unwrap();
        println!("{result}");
        assert!((result - 0.5724067044708798).abs() < 0.0001);
    }

    #[test]
    fn test_chi_dist_vec_wrong_params() {
        // Wrong number of parameters should return an error
        let result = codcel_chi_dist_vec(vec![2.0]);
        assert!(result.is_err());
    }
}
