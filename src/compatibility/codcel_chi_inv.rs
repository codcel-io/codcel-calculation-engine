// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::{ChiSquared, ContinuousCDF};
use std::error::Error;

/// Excel-compatible `CHIINV`/`CHI.INV.RT` function.
/// Computes the inverse right-tailed chi-squared statistic.
/// - `p`: right-tailed probability value in `[0, 1]`.
/// - `df`: degrees of freedom (must be greater than 0).
///
/// Returns an error when `p` is outside `[0, 1]` or `df` is not positive.
pub fn codcel_chi_inv(p: f64, df: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if !(0.0..=1.0).contains(&p) {
        return Err("CHIINV: Probability (p) must be between 0 and 1".into());
    }
    if df <= 0.0 {
        return Err("CHIINV: Degrees of freedom (df) must be greater than 0".into());
    }

    let chi_squared = ChiSquared::new(df).map_err(|_| "CHIINV: Invalid degrees of freedom")?;
    let chi_value = chi_squared.inverse_cdf(1.0 - p);

    Ok(chi_value)
}

/// Convenience wrapper for `CHIINV` that accepts `[p, df]` in a vector.
/// Errors if the vector does not contain exactly two values.
pub fn codcel_chi_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("CHIINV: Must have 2 parameters.".into());
    }

    codcel_chi_inv(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chi_inv_basic() {
        // =CHIINV(0.5, 3) in US format
        // =CHIINV(0,5; 3) in German format
        let result = codcel_chi_inv(0.5, 3.0).unwrap();
        println!("{result}");
        assert!((result - 2.3659738843753).abs() < 0.0001);
    }

    #[test]
    fn test_chi_inv_small_p() {
        // =CHIINV(0.1, 3) in US format
        // =CHIINV(0,1; 3) in German format
        let result = codcel_chi_inv(0.1, 3.0).unwrap();
        println!("{result}");
        assert!((result - 6.251388631553).abs() < 0.0001);
    }

    #[test]
    fn test_chi_inv_large_p() {
        // =CHIINV(0.9, 3) in US format
        // =CHIINV(0,9; 3) in German format
        let result = codcel_chi_inv(0.9, 3.0).unwrap();
        println!("{result}");
        assert!((result - 0.5844149371087).abs() < 0.0001);
    }

    #[test]
    fn test_chi_inv_small_df() {
        // =CHIINV(0.5, 1) in US format
        // =CHIINV(0,5; 1) in German format
        let result = codcel_chi_inv(0.5, 1.0).unwrap();
        println!("{result}");
        assert!((result - 0.4549364256084).abs() < 0.0001);
    }

    #[test]
    fn test_chi_inv_large_df() {
        // =CHIINV(0.5, 10) in US format
        // =CHIINV(0,5; 10) in German format
        let result = codcel_chi_inv(0.5, 10.0).unwrap();
        println!("{result}");
        assert!((result - 9.3417703155).abs() < 0.0001);
    }

    #[test]
    fn test_chi_inv_p_zero() {
        // =CHIINV(0, 3) in US format
        // =CHIINV(0; 3) in German format
        let result = codcel_chi_inv(0.0, 3.0).unwrap();
        println!("{result}");
        // This should be infinity, but we'll check for a very large value
        assert!(result > 1e10);
    }

    #[test]
    fn test_chi_inv_p_one() {
        // =CHIINV(1, 3) in US format
        // =CHIINV(1; 3) in German format
        let result = codcel_chi_inv(1.0, 3.0).unwrap();
        println!("{result}");
        assert!(result < 1e-10);
    }

    #[test]
    fn test_chi_inv_p_out_of_range() {
        // p < 0 should return an error
        let result = codcel_chi_inv(-0.1, 3.0);
        assert!(result.is_err());

        // p > 1 should return an error
        let result = codcel_chi_inv(1.1, 3.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chi_inv_zero_df() {
        // Zero degrees of freedom should return an error
        let result = codcel_chi_inv(0.5, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chi_inv_negative_df() {
        // Negative degrees of freedom should return an error
        let result = codcel_chi_inv(0.5, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chi_inv_vec_basic() {
        // =CHIINV(0.5, 3) in US format
        // =CHIINV(0,5; 3) in German format
        let result = codcel_chi_inv_vec(vec![0.5, 3.0]).unwrap();
        println!("{result}");
        assert!((result - 2.3659738843753).abs() < 0.0001);
    }

    #[test]
    fn test_chi_inv_vec_wrong_params() {
        // Wrong number of parameters should return an error
        let result = codcel_chi_inv_vec(vec![0.5]);
        assert!(result.is_err());
    }
}
