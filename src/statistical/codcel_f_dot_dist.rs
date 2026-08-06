// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::{Continuous, ContinuousCDF};
use std::error::Error;

/// Excel-compatible `F.DIST` that returns the F probability distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `df1`: numerator degrees of freedom (must be > 0).
/// - `df2`: denominator degrees of freedom (must be > 0).
/// - `cumulative`: if `true`, returns the cumulative distribution function (CDF);
///   if `false`, returns the probability density function (PDF).
///
/// Returns the distribution value or an error when inputs are outside the allowed range.
pub fn codcel_f_dot_dist(
    x: f64,
    df1: f64,
    df2: f64,
    cumulative: bool,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if x < 0.0 {
        return Err("F.DIST: x must be non-negative.".into());
    }
    if df1 <= 0.0 {
        return Err("F.DIST: df1 (numerator degrees of freedom) must be greater than 0.".into());
    }
    if df2 <= 0.0 {
        return Err("F.DIST: df2 (denominator degrees of freedom) must be greater than 0.".into());
    }

    // Calculate the F distribution
    let f_dist = statrs::distribution::FisherSnedecor::new(df1, df2)
        .map_err(|_| "F.DIST: Error creating F distribution.")?;

    if cumulative {
        // Cumulative distribution function (CDF)
        Ok(f_dist.cdf(x))
    } else {
        // Probability density function (PDF)
        Ok(f_dist.pdf(x))
    }
}

pub fn codcel_f_dot_dist_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 4 {
        return Err("F.DIST: Must have 4 parameters".into());
    }

    let cumulative = inputs[3] != 0.0;
    codcel_f_dot_dist(inputs[0], inputs[1], inputs[2], cumulative)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_dot_dist_cumulative_basic() {
        // =F.DIST(2, 5, 10, TRUE) in US format
        // =F.DIST(2; 5; 10; TRUE) in German format
        let result = codcel_f_dot_dist(2.0, 5.0, 10.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.8358050).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dot_dist_pdf_basic() {
        // =F.DIST(2, 5, 10, FALSE) in US format
        // =F.DIST(2; 5; 10; FALSE) in German format
        let result = codcel_f_dot_dist(2.0, 5.0, 10.0, false).unwrap();
        println!("{result}");
        assert!((result - 0.16200574218011402).abs() < 0.0001);
    }

    #[test]
    fn test_f_dot_dist_cumulative_equal_df() {
        // =F.DIST(2, 5, 5, TRUE) in US format
        // =F.DIST(2; 5; 5; TRUE) in German format
        let result = codcel_f_dot_dist(2.0, 5.0, 5.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.7674887).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dot_dist_cumulative_large_df() {
        // =F.DIST(2, 50, 100, TRUE) in US format
        // =F.DIST(2; 50; 100; TRUE) in German format
        let result = codcel_f_dot_dist(2.0, 50.0, 100.0, true).unwrap();
        println!("{result}");
        assert!((result - 0.9983139).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dot_dist_negative_x() {
        // Negative x should return an error
        let result = codcel_f_dot_dist(-1.0, 5.0, 10.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dot_dist_zero_df1() {
        // Zero df1 should return an error
        let result = codcel_f_dot_dist(2.0, 0.0, 10.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dot_dist_zero_df2() {
        // Zero df2 should return an error
        let result = codcel_f_dot_dist(2.0, 5.0, 0.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dot_dist_vec_valid() {
        // Test the vector version with valid inputs for cumulative
        let inputs = vec![2.0, 5.0, 10.0, 1.0];
        let result = codcel_f_dot_dist_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 0.8358050).abs() < 0.0000001);
    }

    #[test]
    fn test_f_dot_dist_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![2.0, 5.0, 10.0];
        let result = codcel_f_dot_dist_vec(inputs);
        assert!(result.is_err());
    }
}
