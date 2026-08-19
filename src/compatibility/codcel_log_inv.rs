// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::codcel_log_norm_inv::codcel_log_norm_inv;
use std::error::Error;

/// Excel-compatible `LOGINV`/`LOGNORM.INV` function.
/// Returns the inverse of the log-normal cumulative distribution.
/// - `p`: cumulative probability value in `(0, 1)`.
/// - `mean`: mean of ln(x).
/// - `std_dev`: standard deviation of ln(x) (must be greater than 0).
///
/// Returns an error when `p` is outside `(0, 1)` or the standard deviation is non-positive.
pub fn codcel_log_inv(
    p: f64,
    mean: f64,
    std_dev: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validated here rather than left to LOGNORM.INV so the message names the function the
    // caller actually wrote.
    if !(p > 0.0 && p < 1.0) {
        return Err("LOGINV: p must be between 0 and 1 (exclusive)".into());
    }
    if std_dev <= 0.0 {
        return Err("LOGINV: standard deviation must be greater than 0".into());
    }

    // LOGINV is the old name for LOGNORM.INV.
    codcel_log_norm_inv(p, mean, std_dev)
}

/// Convenience wrapper for `LOGINV` that accepts `[p, mean, std_dev]`.
/// Errors if the vector does not contain exactly three values.
pub fn codcel_log_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("LOGINV: Must have 3 parameters.".into());
    }

    codcel_log_inv(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_inv_basic() {
        // =LOGINV(0.5, 3, 1) in US format
        // =LOGINV(0,5; 3; 1) in German format
        let result = codcel_log_inv(0.5, 3.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 20.085536923187668).abs() < 0.0001);
    }

    #[test]
    fn test_log_inv_small_p() {
        // =LOGINV(0.1, 3, 1) in US format
        // =LOGINV(0,1; 3; 1) in German format
        let result = codcel_log_inv(0.1, 3.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 5.5758704208259084).abs() < 0.0001);
    }

    #[test]
    fn test_log_inv_large_p() {
        // =LOGINV(0.9, 3, 1) in US format
        // =LOGINV(0,9; 3; 1) in German format
        let result = codcel_log_inv(0.9, 3.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 72.35261278417202).abs() < 0.0001);
    }

    #[test]
    fn test_log_inv_different_mean() {
        // =LOGINV(0.5, 2, 1) in US format
        // =LOGINV(0,5; 2; 1) in German format
        let result = codcel_log_inv(0.5, 2.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 7.389056098930649).abs() < 0.0001);
    }

    #[test]
    fn test_log_inv_different_std_dev() {
        // =LOGINV(0.5, 3, 2) in US format
        // =LOGINV(0,5; 3; 2) in German format
        let result = codcel_log_inv(0.5, 3.0, 2.0).unwrap();
        println!("{result}");
        assert!((result - 20.085536923187668).abs() < 0.0001);
    }

    #[test]
    fn test_log_inv_p_zero() {
        // p = 0 should return an error
        let result = codcel_log_inv(0.0, 3.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_inv_p_one() {
        // p = 1 should return an error
        let result = codcel_log_inv(1.0, 3.0, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_log_inv_negative_std_dev() {
        // Negative standard deviation should return an error
        let result = codcel_log_inv(0.5, 3.0, -1.0);
        assert!(result.is_err());
    }
}
