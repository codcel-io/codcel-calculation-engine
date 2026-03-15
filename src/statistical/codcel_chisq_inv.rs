// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `CHISQ.INV` that returns the inverse of the left-tailed chi-squared distribution.
/// - `probability`: the probability associated with the chi-squared distribution (0 to 1).
/// - `degrees_of_freedom`: degrees of freedom (must be > 0).
///
/// Returns the value x such that CHISQ.DIST(x, df, TRUE) = probability,
/// or an error when inputs are outside the allowed range.
pub fn codcel_chisq_inv(
    probability: f64,
    degrees_of_freedom: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if !(0.0..=1.0).contains(&probability) {
        return Err("CHISQ.INV: Probability must be in the range [0, 1].".into());
    }
    if degrees_of_freedom <= 0.0 {
        return Err("CHISQ.INV: Degrees of freedom must be greater than 0.".into());
    }

    // Create chi-squared distribution and calculate inverse CDF
    match statrs::distribution::ChiSquared::new(degrees_of_freedom) {
        Ok(dist) => {
            let result = dist.inverse_cdf(probability);
            if result.is_nan() || result < 0.0 {
                Err("CHISQ.INV: Failed to compute inverse chi-squared distribution.".into())
            } else {
                Ok(result)
            }
        }
        Err(_) => Err("CHISQ.INV: Error creating chi-squared distribution.".into()),
    }
}

pub fn codcel_chisq_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("CHISQ.INV: Must have 2 parameters".into());
    }

    codcel_chisq_inv(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chisq_inv_basic() {
        // =CHISQ.INV(0.5,3) in US format
        // =CHISQ.INV(0,5;3) in German format
        let result = codcel_chisq_inv(0.5, 3.0).unwrap();
        assert!((result - 2.3659738).abs() < 1e-6); // Value where CDF of Chi-squared(3) = 0.5
    }

    #[test]
    fn test_chisq_inv_low_probability() {
        // =CHISQ.INV(0.1,3) in US format
        // =CHISQ.INV(0,1;3) in German format
        let result = codcel_chisq_inv(0.1, 3.0).unwrap();
        println!("{result}");
        assert!((result - 0.58437437).abs() < 1e-6); // Value where CDF of Chi-squared(3) = 0.1
    }

    #[test]
    fn test_chisq_inv_high_probability() {
        // =CHISQ.INV(0.9,3) in US format
        // =CHISQ.INV(0,9;3) in German format
        let result = codcel_chisq_inv(0.9, 3.0).unwrap();
        assert!((result - 6.2513886).abs() < 1e-6); // Value where CDF of Chi-squared(3) = 0.9
    }

    #[test]
    fn test_chisq_inv_low_df() {
        // =CHISQ.INV(0.5,1) in US format
        // =CHISQ.INV(0,5;1) in German format
        let result = codcel_chisq_inv(0.5, 1.0).unwrap();
        assert!((result - 0.4549364).abs() < 1e-6); // Value where CDF of Chi-squared(1) = 0.5
    }

    #[test]
    fn test_chisq_inv_high_df() {
        // =CHISQ.INV(0.5,10) in US format
        // =CHISQ.INV(0,5;10) in German format
        let result = codcel_chisq_inv(0.5, 10.0).unwrap();
        println!("{result}");
        assert!((result - 9.34181777).abs() < 1e-6); // Value where CDF of Chi-squared(10) = 0.5
    }

    #[test]
    fn test_chisq_inv_fractional_df() {
        // =CHISQ.INV(0.5,2.5) in US format
        // =CHISQ.INV(0,5;2,5) in German format
        let result = codcel_chisq_inv(0.5, 2.5).unwrap();
        assert!((result - 1.87384776778088).abs() < 1e-10);
    }

    #[test]
    fn test_chisq_inv_zero_probability() {
        // =CHISQ.INV(0,3) in US format
        // =CHISQ.INV(0;3) in German format
        let result = codcel_chisq_inv(0.0, 3.0).unwrap();
        assert!(result < 1e-10); // Value where CDF of Chi-squared(3) = 0 is approximately 0
    }

    #[test]
    fn test_chisq_inv_one_probability() {
        // =CHISQ.INV(1,3) in US format
        // =CHISQ.INV(1;3) in German format
        let result = codcel_chisq_inv(1.0, 3.0).unwrap();
        assert!(result > 1e10); // Value where CDF of Chi-squared(3) = 1 is infinity, but we get a very large number
    }

    #[test]
    fn test_chisq_inv_invalid_probability_low() {
        // =CHISQ.INV(-0.1,3) in US format (returns #NUM! error)
        // =CHISQ.INV(-0,1;3) in German format (returns #NUM! error)
        let result = codcel_chisq_inv(-0.1, 3.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_inv_invalid_probability_high() {
        // =CHISQ.INV(1.1,3) in US format (returns #NUM! error)
        // =CHISQ.INV(1,1;3) in German format (returns #NUM! error)
        let result = codcel_chisq_inv(1.1, 3.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_inv_invalid_df_zero() {
        // =CHISQ.INV(0.5,0) in US format (returns #NUM! error)
        // =CHISQ.INV(0,5;0) in German format (returns #NUM! error)
        let result = codcel_chisq_inv(0.5, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_inv_invalid_df_negative() {
        // =CHISQ.INV(0.5,-1) in US format (returns #NUM! error)
        // =CHISQ.INV(0,5;-1) in German format (returns #NUM! error)
        let result = codcel_chisq_inv(0.5, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_inv_vec_valid() {
        // =CHISQ.INV(0.5,3) in US format
        // =CHISQ.INV(0,5;3) in German format
        let result = codcel_chisq_inv_vec(vec![0.5, 3.0]).unwrap();
        assert!((result - 2.3659738).abs() < 1e-6);
    }

    #[test]
    fn test_chisq_inv_vec_invalid_length() {
        // =CHISQ.INV(0.5) in US format (returns #VALUE! error)
        // =CHISQ.INV(0,5) in German format (returns #VALUE! error)
        let result = codcel_chisq_inv_vec(vec![0.5]);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_inv_vec_too_many_params() {
        // =CHISQ.INV(0.5,3,4) in US format (ignores extra parameters)
        // =CHISQ.INV(0,5;3;4) in German format (ignores extra parameters)
        let result = codcel_chisq_inv_vec(vec![0.5, 3.0, 4.0]);
        assert!(result.is_err());
    }
}
