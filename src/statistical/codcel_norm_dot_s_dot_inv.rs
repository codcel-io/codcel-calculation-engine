// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::standard_normal::std_normal_inv;
use std::error::Error;

/// Excel-compatible `NORM.S.INV` that returns the inverse of the standard normal cumulative distribution.
/// - `probability`: the probability corresponding to the standard normal distribution (0 to 1, exclusive).
///
/// Returns the z-score (number of standard deviations from the mean)
/// such that NORM.S.DIST(z, TRUE) = probability.
pub fn codcel_norm_dot_s_dot_inv(probability: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Excel gives #NUM! at both endpoints, where the z-score is infinite.
    if !(probability > 0.0 && probability < 1.0) {
        return Err("NORM.S.INV: Probability must be between 0 and 1 (exclusive).".into());
    }

    Ok(std_normal_inv(probability))
}

pub fn codcel_norm_dot_s_dot_inv_vec(
    inputs: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("NORM.S.INV: Must have 1 parameter.".into());
    }

    codcel_norm_dot_s_dot_inv(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values were computed with mpmath at 60 decimal digits and rounded to the nearest
    // f64. The previous tolerances here were as loose as 1e-2 against values Excel reports to 15
    // significant digits, which could not have caught a precision regression.

    #[test]
    fn test_norm_dot_s_dot_inv_median() {
        // =NORM.S.INV(0.5) in US format
        // =NORM.S.INV(0,5) in German format
        let result = codcel_norm_dot_s_dot_inv(0.5).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_positive_z() {
        // =NORM.S.INV(0.975) in US format
        // =NORM.S.INV(0,975) in German format
        let result = codcel_norm_dot_s_dot_inv(0.975).unwrap();
        assert!((result - 1.9599639845400543).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_negative_z() {
        // =NORM.S.INV(0.025) in US format
        // =NORM.S.INV(0,025) in German format
        let result = codcel_norm_dot_s_dot_inv(0.025).unwrap();
        assert!((result + 1.9599639845400543).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_central_region() {
        // =NORM.S.INV(0.3) in US format
        // =NORM.S.INV(0,3) in German format
        let result = codcel_norm_dot_s_dot_inv(0.3).unwrap();
        assert!((result + 0.5244005127080408).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_lower_region() {
        // =NORM.S.INV(0.01) in US format
        // =NORM.S.INV(0,01) in German format
        let result = codcel_norm_dot_s_dot_inv(0.01).unwrap();
        assert!((result + 2.326347874040841).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_upper_region() {
        // =NORM.S.INV(0.99) in US format
        // =NORM.S.INV(0,99) in German format
        let result = codcel_norm_dot_s_dot_inv(0.99).unwrap();
        assert!((result - 2.326347874040841).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_very_small() {
        // =NORM.S.INV(0.001) in US format
        // =NORM.S.INV(0,001) in German format
        let result = codcel_norm_dot_s_dot_inv(0.001).unwrap();
        assert!((result + 3.0902323061678136).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_very_large() {
        // =NORM.S.INV(0.999) in US format
        // =NORM.S.INV(0,999) in German format
        let result = codcel_norm_dot_s_dot_inv(0.999).unwrap();
        assert!((result - 3.0902323061678136).abs() < 1e-15);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_extreme_tail() {
        // =NORM.S.INV(0.00000001) in US format
        // =NORM.S.INV(0,00000001) in German format
        let result = codcel_norm_dot_s_dot_inv(1e-8).unwrap();
        assert!((result + 5.612001244174789).abs() < 1e-14);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_inverts_norm_dot_s_dot_dist() {
        use crate::statistical::codcel_norm_dot_s_dot_dist::codcel_norm_dot_s_dot_dist;
        for z in [-5.0, -3.0, -2.0, -1.0, -0.5, 0.0] {
            let probability = codcel_norm_dot_s_dot_dist(z, true).unwrap();
            let round_tripped = codcel_norm_dot_s_dot_inv(probability).unwrap();
            assert!(
                (round_tripped - z).abs() < 1e-14 * z.abs().max(1.0),
                "round trip of {z} gave {round_tripped}"
            );
        }
    }

    #[test]
    fn test_norm_dot_s_dot_inv_zero() {
        // =NORM.S.INV(0) in US format
        // =NORM.S.INV(0) in German format
        // Excel gives #NUM! here.
        let result = codcel_norm_dot_s_dot_inv(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_s_dot_inv_one() {
        // =NORM.S.INV(1) in US format
        // =NORM.S.INV(1) in German format
        let result = codcel_norm_dot_s_dot_inv(1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_s_dot_inv_out_of_range() {
        // Values outside [0,1] should return an error
        let result = codcel_norm_dot_s_dot_inv(1.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_s_dot_inv_nan() {
        // NaN must not slip through the range check
        let result = codcel_norm_dot_s_dot_inv(f64::NAN);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_s_dot_inv_vec_valid() {
        // Test the vector version with valid input
        let inputs = vec![0.5];
        let result = codcel_norm_dot_s_dot_inv_vec(inputs).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_vec_invalid() {
        // Test the vector version with invalid input (too many parameters)
        let inputs = vec![0.5, 0.6];
        let result = codcel_norm_dot_s_dot_inv_vec(inputs);
        assert!(result.is_err());
    }
}
