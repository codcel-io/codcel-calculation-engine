// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `F.INV.RT` that returns the inverse of the right-tailed F probability distribution.
/// - `p`: the right-tailed probability associated with the F distribution (0 to 1, exclusive).
/// - `df1`: numerator degrees of freedom (must be > 0).
/// - `df2`: denominator degrees of freedom (must be > 0).
///
/// Returns the value x such that F.DIST.RT(x, df1, df2) = p,
/// or an error when inputs are outside the allowed range.
pub fn codcel_f_inv_rt(p: f64, df1: f64, df2: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if p <= 0.0 || p >= 1.0 {
        return Err("F.INV.RT: p must be in the range (0, 1).".into());
    }
    if df1 <= 0.0 {
        return Err("F.INV.RT: df1 (numerator degrees of freedom) must be greater than 0.".into());
    }
    if df2 <= 0.0 {
        return Err(
            "F.INV.RT: df2 (denominator degrees of freedom) must be greater than 0.".into(),
        );
    }

    // Create the F distribution
    let f_dist = statrs::distribution::FisherSnedecor::new(df1, df2)
        .map_err(|_| "F.INV.RT: Error creating F distribution.")?;

    // Calculate the inverse of the right-tailed cumulative distribution function
    Ok(f_dist.inverse_cdf(1.0 - p))
}

pub fn codcel_f_inv_rt_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("F.INV.RT: Must have 3 parameters".into());
    }

    codcel_f_inv_rt(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_inv_rt_basic() {
        // =F.INV.RT(0.164195, 5, 10) in US format
        // =F.INV.RT(0,164195; 5; 10) in German format
        let result = codcel_f_inv_rt(0.164195, 5.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_rt_equal_df() {
        // =F.INV.RT(0.232511, 5, 5) in US format
        // =F.INV.RT(0,232511; 5; 5) in German format
        let result = codcel_f_inv_rt(0.232511, 5.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_rt_large_df() {
        // =F.INV.RT(0.001686, 50, 100) in US format
        // =F.INV.RT(0,001686; 50; 100) in German format
        let result = codcel_f_inv_rt(0.001686, 50.0, 100.0).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_rt_small_p() {
        // =F.INV.RT(0.01, 5, 10) in US format
        // =F.INV.RT(0,01; 5; 10) in German format
        let result = codcel_f_inv_rt(0.01, 5.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 5.636326187669065).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_rt_large_p() {
        // =F.INV.RT(0.9, 5, 10) in US format
        // =F.INV.RT(0,9; 5; 10) in German format
        let result = codcel_f_inv_rt(0.9, 5.0, 10.0).unwrap();
        println!("{result}");
        assert!((result - 0.3032690890210724).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_rt_invalid_p_low() {
        // p <= 0 should return an error
        let result = codcel_f_inv_rt(0.0, 5.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_inv_rt_invalid_p_high() {
        // p >= 1 should return an error
        let result = codcel_f_inv_rt(1.0, 5.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_inv_rt_zero_df1() {
        // Zero df1 should return an error
        let result = codcel_f_inv_rt(0.5, 0.0, 10.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_inv_rt_zero_df2() {
        // Zero df2 should return an error
        let result = codcel_f_inv_rt(0.5, 5.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_inv_rt_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![0.164195, 5.0, 10.0];
        let result = codcel_f_inv_rt_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_rt_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![0.5, 5.0];
        let result = codcel_f_inv_rt_vec(inputs);
        assert!(result.is_err());
    }
}
