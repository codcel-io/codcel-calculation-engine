// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `F.INV` that returns the inverse of the left-tailed F probability distribution.
/// - `p`: the probability associated with the F distribution (0 to 1, exclusive).
/// - `df1`: numerator degrees of freedom (must be > 0).
/// - `df2`: denominator degrees of freedom (must be > 0).
///
/// Returns the value x such that F.DIST(x, df1, df2, TRUE) = p,
/// or an error when inputs are outside the allowed range.
pub fn codcel_f_dot_inv(p: f64, df1: f64, df2: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if p <= 0.0 || p >= 1.0 {
        return Err("F.INV: p must be in the range (0, 1).".into());
    }
    if df1 <= 0.0 {
        return Err("F.INV: df1 (numerator degrees of freedom) must be greater than 0.".into());
    }
    if df2 <= 0.0 {
        return Err("F.INV: df2 (denominator degrees of freedom) must be greater than 0.".into());
    }

    // Create the F distribution
    let f_dist = statrs::distribution::FisherSnedecor::new(df1, df2)
        .map_err(|_| "F.INV: Error creating F distribution.")?;

    // Calculate the inverse cumulative distribution function (quantile)
    Ok(f_dist.inverse_cdf(p))
}

pub fn codcel_f_dot_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("F.INV: Must have 3 parameters".into());
    }

    codcel_f_dot_inv(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_dot_inv_basic() {
        // =F.INV(0.8358050, 5, 10) in US format
        // =F.INV(0,8358050; 5; 10) in German format
        let result = codcel_f_dot_inv(0.8358050, 5.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_dot_inv_equal_df() {
        // =F.INV(0.7674887, 5, 5) in US format
        // =F.INV(0,7674887; 5; 5) in German format
        let result = codcel_f_dot_inv(0.7674887, 5.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_dot_inv_large_df() {
        // =F.INV(0.9983139, 50, 100) in US format
        // =F.INV(0,9983139; 50; 100) in German format
        let result = codcel_f_dot_inv(0.9983139, 50.0, 100.0).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_dot_inv_small_p() {
        // =F.INV(0.1, 5, 10) in US format
        // =F.INV(0,1; 5; 10) in German format
        let result = codcel_f_dot_inv(0.1, 5.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 0.30326908902107264).abs() < 0.0001);
    }

    #[test]
    fn test_f_dot_inv_large_p() {
        // =F.INV(0.95, 5, 10) in US format
        // =F.INV(0,95; 5; 10) in German format
        let result = codcel_f_dot_inv(0.95, 5.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 3.3258).abs() < 0.0001);
    }

    #[test]
    fn test_f_dot_inv_invalid_p_low() {
        // p <= 0 should return an error
        let result = codcel_f_dot_inv(0.0, 5.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dot_inv_invalid_p_high() {
        // p >= 1 should return an error
        let result = codcel_f_dot_inv(1.0, 5.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dot_inv_zero_df1() {
        // Zero df1 should return an error
        let result = codcel_f_dot_inv(0.5, 0.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dot_inv_zero_df2() {
        // Zero df2 should return an error
        let result = codcel_f_dot_inv(0.5, 5.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dot_inv_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![0.8358050, 5.0, 10.0];
        let result = codcel_f_dot_inv_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_dot_inv_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![0.5, 5.0];
        let result = codcel_f_dot_inv_vec(inputs);
        assert!(result.is_err());
    }
}
