// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `CHISQ.DIST.RT` that returns the right-tailed probability of the chi-squared distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `degrees_of_freedom`: degrees of freedom (must be > 0).
///
/// Returns the right-tailed probability (1 - CDF) or an error when inputs are outside the allowed range.
pub fn codcel_chisq_dist_rt(
    x: f64,
    degrees_of_freedom: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if x < 0.0 {
        return Err("CHISQ.DIST.RT: x must be greater than or equal to 0.".into());
    }
    if degrees_of_freedom <= 0.0 {
        return Err("CHISQ.DIST.RT: Degrees of freedom must be greater than 0.".into());
    }

    // Calculate the right-tailed chi-square distribution value
    match statrs::distribution::ChiSquared::new(degrees_of_freedom) {
        Ok(dist) => {
            let result = 1.0 - dist.cdf(x);
            Ok(result)
        }
        Err(_) => Err("CHISQ.DIST.RT: Error creating chi-squared distribution.".into()),
    }
}

pub fn codcel_chisq_dist_rt_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("CHISQ.DIST.RT: Must have 2 parameters".into());
    }

    codcel_chisq_dist_rt(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chisq_dist_rt_basic() {
        // =CHISQ.DIST.RT(2,3) in US format
        // =CHISQ.DIST.RT(2;3) in German format
        let result = codcel_chisq_dist_rt(2.0, 3.0).unwrap();
        assert!((result - 0.5724067044708807).abs() < 1e-10); // Right-tailed probability of Chi-squared(3) at x=2
    }

    #[test]
    fn test_chisq_dist_rt_zero_x() {
        // =CHISQ.DIST.RT(0,3) in US format
        // =CHISQ.DIST.RT(0;3) in German format
        let result = codcel_chisq_dist_rt(0.0, 3.0).unwrap();
        assert_eq!(result, 1.0); // Right-tailed probability of Chi-squared(3) at x=0 is 1
    }

    #[test]
    fn test_chisq_dist_rt_large_x() {
        // =CHISQ.DIST.RT(10,3) in US format
        // =CHISQ.DIST.RT(10;3) in German format
        let result = codcel_chisq_dist_rt(10.0, 3.0).unwrap();
        println!("{result:?}");
        assert!((result - 0.0185661354630432).abs() < 1e-10); // Right-tailed probability of Chi-squared(3) at x=10
    }

    #[test]
    fn test_chisq_dist_rt_low_df() {
        // =CHISQ.DIST.RT(2,1) in US format
        // =CHISQ.DIST.RT(2;1) in German format
        let result = codcel_chisq_dist_rt(2.0, 1.0).unwrap();
        assert!((result - 0.1572992071).abs() < 1e-10); // Right-tailed probability of Chi-squared(1) at x=2
    }

    #[test]
    fn test_chisq_dist_rt_high_df() {
        // =CHISQ.DIST.RT(20,10) in US format
        // =CHISQ.DIST.RT(20;10) in German format
        let result = codcel_chisq_dist_rt(20.0, 10.0).unwrap();
        assert!((result - 0.02925268807696102).abs() < 1e-10); // Right-tailed probability of Chi-squared(10) at x=20
    }

    #[test]
    fn test_chisq_dist_rt_fractional_df() {
        // =CHISQ.DIST.RT(2,2.5) in US format
        // =CHISQ.DIST.RT(2;2,5) in German format
        let result = codcel_chisq_dist_rt(2.0, 2.5).unwrap();
        assert!((result - 0.473788779643342).abs() < 1e-10); // Right-tailed probability of Chi-squared(2.5) at x=2
    }

    #[test]
    fn test_chisq_dist_rt_very_large_x() {
        // =CHISQ.DIST.RT(100,3) in US format
        // =CHISQ.DIST.RT(100;3) in German format
        let result = codcel_chisq_dist_rt(100.0, 3.0).unwrap();
        assert!(result < 1e-10); // Right-tailed probability of Chi-squared(3) at x=100 is approximately 0
    }

    #[test]
    fn test_chisq_dist_rt_invalid_x() {
        // =CHISQ.DIST.RT(-1,3) in US format (returns #NUM! error)
        // =CHISQ.DIST.RT(-1;3) in German format (returns #NUM! error)
        let result = codcel_chisq_dist_rt(-1.0, 3.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_dist_rt_invalid_df_zero() {
        // =CHISQ.DIST.RT(2,0) in US format (returns #NUM! error)
        // =CHISQ.DIST.RT(2;0) in German format (returns #NUM! error)
        let result = codcel_chisq_dist_rt(2.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_dist_rt_invalid_df_negative() {
        // =CHISQ.DIST.RT(2,-1) in US format (returns #NUM! error)
        // =CHISQ.DIST.RT(2;-1) in German format (returns #NUM! error)
        let result = codcel_chisq_dist_rt(2.0, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_dist_rt_vec_valid() {
        // =CHISQ.DIST.RT(2,3) in US format
        // =CHISQ.DIST.RT(2;3) in German format
        let result = codcel_chisq_dist_rt_vec(vec![2.0, 3.0]).unwrap();
        println!("{result:?}");
        assert!((result - 0.5724067044708807).abs() < 1e-10);
    }

    #[test]
    fn test_chisq_dist_rt_vec_invalid_length() {
        // =CHISQ.DIST.RT(2) in US format (returns #VALUE! error)
        // =CHISQ.DIST.RT(2) in German format (returns #VALUE! error)
        let result = codcel_chisq_dist_rt_vec(vec![2.0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_dist_rt_vec_too_many_params() {
        // =CHISQ.DIST.RT(2,3,4) in US format (ignores extra parameters)
        // =CHISQ.DIST.RT(2;3;4) in German format (ignores extra parameters)
        let result = codcel_chisq_dist_rt_vec(vec![2.0, 3.0, 4.0]);
        assert!(result.is_err());
    }
}
