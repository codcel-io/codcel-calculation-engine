// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compatibility::codcel_norm_inv::{codcel_norm_inv, codcel_norm_inv_vec};
use std::error::Error;

/// Excel-compatible `NORM.INV` that returns the inverse of the normal cumulative distribution.
/// - `probability`: the probability corresponding to the normal distribution (0 to 1).
/// - `mean`: the arithmetic mean of the distribution.
/// - `std_dev`: the standard deviation of the distribution (must be > 0).
///
/// Returns the value x such that NORM.DIST(x, mean, std_dev, TRUE) = probability,
/// or an error when inputs are outside the allowed range.
/// This is equivalent to the older NORMINV function.
pub fn codcel_norm_dot_inv(
    probability: f64,
    mean: f64,
    std_dev: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // NORM.INV is the same as NORMINV
    codcel_norm_inv(probability, mean, std_dev)
}

pub fn codcel_norm_dot_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // NORM.INV is the same as NORMINV
    codcel_norm_inv_vec(inputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values were computed with mpmath at 60 decimal digits and rounded to the nearest
    // f64. These tests previously used tolerances as loose as 1e-2 against values Excel reports
    // to 15 significant digits.

    #[test]
    fn test_norm_dot_inv_standard_median() {
        // =NORM.INV(0.5, 0, 1) in US format
        // =NORM.INV(0,5; 0; 1) in German format
        let result = codcel_norm_dot_inv(0.5, 0.0, 1.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_norm_dot_inv_standard_positive_z() {
        // =NORM.INV(0.975, 0, 1) in US format
        // =NORM.INV(0,975; 0; 1) in German format
        let result = codcel_norm_dot_inv(0.975, 0.0, 1.0).unwrap();
        assert!((result - 1.9599639845400543).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_inv_standard_negative_z() {
        // =NORM.INV(0.025, 0, 1) in US format
        // =NORM.INV(0,025; 0; 1) in German format
        let result = codcel_norm_dot_inv(0.025, 0.0, 1.0).unwrap();
        assert!((result + 1.9599639845400543).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_inv_non_standard_mean() {
        // =NORM.INV(0.5, 10, 1) in US format
        // =NORM.INV(0,5; 10; 1) in German format
        let result = codcel_norm_dot_inv(0.5, 10.0, 1.0).unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_norm_dot_inv_non_standard_std_dev() {
        // =NORM.INV(0.975, 0, 2) in US format
        // =NORM.INV(0,975; 0; 2) in German format
        let result = codcel_norm_dot_inv(0.975, 0.0, 2.0).unwrap();
        assert!((result - 3.9199279690801085).abs() < 1e-14);
    }

    #[test]
    fn test_norm_dot_inv_non_standard_both() {
        // =NORM.INV(0.975, 10, 2) in US format
        // =NORM.INV(0,975; 10; 2) in German format
        let result = codcel_norm_dot_inv(0.975, 10.0, 2.0).unwrap();
        assert!((result - 13.919927969080108).abs() < 1e-14);
    }

    #[test]
    fn test_norm_dot_inv_agrees_with_norm_dot_s_dot_inv() {
        // NORM.INV(p, 0, 1) and NORM.S.INV(p) are the same Excel function. They used to run
        // through two unrelated rational approximations.
        use crate::statistical::codcel_norm_dot_s_dot_inv::codcel_norm_dot_s_dot_inv;
        for p in [1e-8, 0.001, 0.025, 0.1, 0.3, 0.5, 0.9, 0.975, 0.999] {
            assert_eq!(
                codcel_norm_dot_inv(p, 0.0, 1.0).unwrap(),
                codcel_norm_dot_s_dot_inv(p).unwrap(),
                "disagreement at p = {p}"
            );
        }
    }

    #[test]
    fn test_norm_dot_inv_probability_zero() {
        // =NORM.INV(0, 0, 1) in US format
        // =NORM.INV(0; 0; 1) in German format
        // Excel gives #NUM! rather than -infinity.
        let result = codcel_norm_dot_inv(0.0, 0.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_inv_probability_one() {
        // =NORM.INV(1, 0, 1) in US format
        // =NORM.INV(1; 0; 1) in German format
        // Excel gives #NUM! rather than +infinity.
        let result = codcel_norm_dot_inv(1.0, 0.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_inv_probability_out_of_range() {
        // Probability outside [0,1] should return an error
        let result = codcel_norm_dot_inv(1.5, 0.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_inv_negative_std_dev() {
        // Negative standard deviation should return an error
        let result = codcel_norm_dot_inv(0.5, 0.0, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_inv_zero_std_dev() {
        // Zero standard deviation should return an error
        let result = codcel_norm_dot_inv(0.5, 0.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_inv_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![0.5, 0.0, 1.0];
        let result = codcel_norm_dot_inv_vec(inputs).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_norm_dot_inv_vec_invalid() {
        // Test the vector version with invalid inputs (wrong number of parameters)
        let inputs = vec![0.5, 0.0];
        let result = codcel_norm_dot_inv_vec(inputs);
        assert!(result.is_err());
    }
}
