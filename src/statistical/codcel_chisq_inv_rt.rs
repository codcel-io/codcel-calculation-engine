// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `CHISQ.INV.RT` that returns the inverse of the right-tailed chi-squared distribution.
/// - `probability`: the right-tailed probability associated with the chi-squared distribution (0 to 1).
/// - `degrees_of_freedom`: degrees of freedom (must be > 0).
///
/// Returns the value x such that CHISQ.DIST.RT(x, df) = probability,
/// or an error when inputs are outside the allowed range.
pub fn codcel_chisq_inv_rt(
    probability: f64,
    degrees_of_freedom: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if !(0.0..=1.0).contains(&probability) {
        return Err("CHISQ.INV.RT: Probability must be in the range [0, 1].".into());
    }
    if degrees_of_freedom <= 0.0 {
        return Err("CHISQ.INV.RT: Degrees of freedom must be greater than 0.".into());
    }

    // Create chi-squared distribution and calculate the right-tailed inverse CDF
    match statrs::distribution::ChiSquared::new(degrees_of_freedom) {
        Ok(dist) => {
            let left_tail_probability = 1.0 - probability;
            let result = dist.inverse_cdf(left_tail_probability);
            if result.is_nan() || result < 0.0 {
                Err("CHISQ.INV.RT: Failed to compute inverse chi-squared distribution.".into())
            } else {
                Ok(result)
            }
        }
        Err(_) => Err("CHISQ.INV.RT: Error creating chi-squared distribution.".into()),
    }
}

pub fn codcel_chisq_inv_rt_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("CHISQ.INV.RT: Must have 2 parameters".into());
    }

    codcel_chisq_inv_rt(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chisq_inv_rt_basic() {
        // =CHISQ.INV.RT(0.5,3) in US format
        // =CHISQ.INV.RT(0,5;3) in German format
        let result = codcel_chisq_inv_rt(0.5, 3.0).unwrap();
        assert!((result - 2.3659738).abs() < 1e-6); // Value where right-tailed probability of Chi-squared(3) = 0.5
    }

    #[test]
    fn test_chisq_inv_rt_low_probability() {
        // =CHISQ.INV.RT(0.1,3) in US format
        // =CHISQ.INV.RT(0,1;3) in German format
        let result = codcel_chisq_inv_rt(0.1, 3.0).unwrap();
        assert!((result - 6.2513886).abs() < 1e-6); // Value where right-tailed probability of Chi-squared(3) = 0.1
    }

    #[test]
    fn test_chisq_inv_rt_high_probability() {
        // =CHISQ.INV.RT(0.9,3) in US format
        // =CHISQ.INV.RT(0,9;3) in German format
        let result = codcel_chisq_inv_rt(0.9, 3.0).unwrap();
        println!("{result}");
        assert!((result - 0.58437437).abs() < 1e-6); // Value where right-tailed probability of Chi-squared(3) = 0.9
    }

    #[test]
    fn test_chisq_inv_rt_low_df() {
        // =CHISQ.INV.RT(0.5,1) in US format
        // =CHISQ.INV.RT(0,5;1) in German format
        let result = codcel_chisq_inv_rt(0.5, 1.0).unwrap();
        assert!((result - 0.4549364).abs() < 1e-6); // Value where right-tailed probability of Chi-squared(1) = 0.5
    }

    #[test]
    fn test_chisq_inv_rt_high_df() {
        // =CHISQ.INV.RT(0.5,10) in US format
        // =CHISQ.INV.RT(0,5;10) in German format
        let result = codcel_chisq_inv_rt(0.5, 10.0).unwrap();
        println!("{result}");
        assert!((result - 9.34181777).abs() < 1e-6); // Value where right-tailed probability of Chi-squared(10) = 0.5
    }

    #[test]
    fn test_chisq_inv_rt_fractional_df() {
        // =CHISQ.INV.RT(0.5,2.5) in US format
        // =CHISQ.INV.RT(0,5;2,5) in German format
        let result = codcel_chisq_inv_rt(0.5, 2.5).unwrap();
        assert!((result - 1.873847767780878).abs() < 1e-6); // Value where right-tailed probability of Chi-squared(2.5) = 0.5
    }

    #[test]
    fn test_chisq_inv_rt_zero_probability() {
        // =CHISQ.INV.RT(0,3) in US format
        // =CHISQ.INV.RT(0;3) in German format
        let result = codcel_chisq_inv_rt(0.0, 3.0).unwrap();
        assert!(result > 1e10); // Value where right-tailed probability of Chi-squared(3) = 0 is infinity
    }

    #[test]
    fn test_chisq_inv_rt_one_probability() {
        // =CHISQ.INV.RT(1,3) in US format
        // =CHISQ.INV.RT(1;3) in German format
        let result = codcel_chisq_inv_rt(1.0, 3.0).unwrap();
        assert!(result < 1e-10); // Value where right-tailed probability of Chi-squared(3) = 1 is approximately 0
    }

    #[test]
    fn test_chisq_inv_rt_invalid_probability_low() {
        // =CHISQ.INV.RT(-0.1,3) in US format (returns #NUM! error)
        // =CHISQ.INV.RT(-0,1;3) in German format (returns #NUM! error)
        let result = codcel_chisq_inv_rt(-0.1, 3.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_inv_rt_invalid_probability_high() {
        // =CHISQ.INV.RT(1.1,3) in US format (returns #NUM! error)
        // =CHISQ.INV.RT(1,1;3) in German format (returns #NUM! error)
        let result = codcel_chisq_inv_rt(1.1, 3.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_inv_rt_invalid_df_zero() {
        // =CHISQ.INV.RT(0.5,0) in US format (returns #NUM! error)
        // =CHISQ.INV.RT(0,5;0) in German format (returns #NUM! error)
        let result = codcel_chisq_inv_rt(0.5, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_inv_rt_invalid_df_negative() {
        // =CHISQ.INV.RT(0.5,-1) in US format (returns #NUM! error)
        // =CHISQ.INV.RT(0,5;-1) in German format (returns #NUM! error)
        let result = codcel_chisq_inv_rt(0.5, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_inv_rt_vec_valid() {
        // =CHISQ.INV.RT(0.5,3) in US format
        // =CHISQ.INV.RT(0,5;3) in German format
        let result = codcel_chisq_inv_rt_vec(vec![0.5, 3.0]).unwrap();
        assert!((result - 2.3659738).abs() < 1e-6);
    }

    #[test]
    fn test_chisq_inv_rt_vec_invalid_length() {
        // =CHISQ.INV.RT(0.5) in US format (returns #VALUE! error)
        // =CHISQ.INV.RT(0,5) in German format (returns #VALUE! error)
        let result = codcel_chisq_inv_rt_vec(vec![0.5]);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_inv_rt_vec_too_many_params() {
        // =CHISQ.INV.RT(0.5,3,4) in US format (ignores extra parameters)
        // =CHISQ.INV.RT(0,5;3;4) in German format (ignores extra parameters)
        let result = codcel_chisq_inv_rt_vec(vec![0.5, 3.0, 4.0]);
        assert!(result.is_err());
    }
}
