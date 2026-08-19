// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::standard_normal::std_normal_inv;
use std::error::Error;

/// Excel-compatible `NORMINV`/`NORM.INV` function.
/// Returns the inverse normal distribution for a cumulative probability.
/// - `probability`: cumulative probability value, strictly between 0 and 1.
/// - `mean`: arithmetic mean of the distribution.
/// - `std_dev`: standard deviation of the distribution (must be greater than 0).
///
/// Returns an error on probabilities outside `(0, 1)` or non-positive standard deviation.
pub fn codcel_norm_inv(
    probability: f64,
    mean: f64,
    std_dev: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Excel gives #NUM! at both endpoints, where the result is infinite.
    if !(probability > 0.0 && probability < 1.0) {
        return Err("NORMINV: Probability must be between 0 and 1 exclusive".into());
    }

    if std_dev <= 0.0 {
        return Err("NORMINV: Standard deviation must be greater than 0".into());
    }

    Ok(mean + std_dev * std_normal_inv(probability))
}

/// Convenience wrapper for `NORMINV` that accepts `[probability, mean, std_dev]`.
/// Errors if the vector does not contain exactly three values.
pub fn codcel_norm_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("NORMINV: Must have 3 parameters.".into());
    }

    codcel_norm_inv(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    // Expected values were computed with mpmath at 60 decimal digits and rounded to the nearest
    // f64.

    #[test]
    fn test_norm_inv_basic() {
        // =NORMINV(0.5, 0, 1) in US format
        // =NORMINV(0,5; 0; 1) in German format
        let result = codcel_norm_inv(0.5, 0.0, 1.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_norm_inv_low_probability() {
        // =NORMINV(0.1, 0, 1) in US format
        // =NORMINV(0,1; 0; 1) in German format
        let result = codcel_norm_inv(0.1, 0.0, 1.0).unwrap();
        assert!((result + 1.2815515655446004).abs() < 1e-15);
    }

    #[test]
    fn test_norm_inv_high_probability() {
        // =NORMINV(0.9, 0, 1) in US format
        // =NORMINV(0,9; 0; 1) in German format
        let result = codcel_norm_inv(0.9, 0.0, 1.0).unwrap();
        assert!((result - 1.2815515655446004).abs() < 1e-15);
    }

    #[test]
    fn test_norm_inv_different_mean_std_dev() {
        // =NORMINV(0.5, 10, 2) in US format
        // =NORMINV(0,5; 10; 2) in German format
        let result = codcel_norm_inv(0.5, 10.0, 2.0).unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_norm_inv_extreme_probability() {
        // =NORMINV(0.999, 0, 1) in US format
        // =NORMINV(0,999; 0; 1) in German format
        let result = codcel_norm_inv(0.999, 0.0, 1.0).unwrap();
        assert!((result - 3.0902323061678136).abs() < 1e-15);
    }

    #[test]
    fn test_norm_inv_scaled_and_shifted() {
        // =NORMINV(0.975, 10, 2) in US format
        // =NORMINV(0,975; 10; 2) in German format
        let result = codcel_norm_inv(0.975, 10.0, 2.0).unwrap();
        assert!((result - 13.919927969080108).abs() < 1e-14);
    }

    #[test]
    fn test_norm_inv_inverts_norm_dist() {
        use crate::compatibility::codcel_norm_dist::codcel_norm_dist;
        for x in [-3.0, -1.0, 0.0, 1.0, 3.0] {
            let probability = codcel_norm_dist(x, 2.0, 1.5, true).unwrap();
            let round_tripped = codcel_norm_inv(probability, 2.0, 1.5).unwrap();
            assert!(
                (round_tripped - x).abs() < 1e-12 * x.abs().max(1.0),
                "round trip of {x} gave {round_tripped}"
            );
        }
    }

    #[test]
    fn test_norm_inv_invalid_probability() {
        // =NORMINV(1.5, 0, 1) in US format
        // =NORMINV(1,5; 0; 1) in German format
        let result = codcel_norm_inv(1.5, 0.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_inv_probability_endpoints() {
        // =NORMINV(0, 0, 1) and =NORMINV(1, 0, 1) both give #NUM! in Excel.
        assert!(codcel_norm_inv(0.0, 0.0, 1.0).is_err());
        assert!(codcel_norm_inv(1.0, 0.0, 1.0).is_err());
    }

    #[test]
    fn test_norm_inv_nan_probability() {
        let result = codcel_norm_inv(f64::NAN, 0.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_inv_zero_std_dev() {
        // =NORMINV(0.5, 0, 0) in US format
        // =NORMINV(0,5; 0; 0) in German format
        let result = codcel_norm_inv(0.5, 0.0, 0.0);
        assert!(result.is_err());
    }
}
