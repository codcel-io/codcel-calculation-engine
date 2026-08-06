// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::codcel_norm_dot_s_dot_inv::codcel_norm_dot_s_dot_inv;
use std::error::Error;

/// Excel-compatible `NORMSINV`/`NORM.S.INV` function.
/// Returns the inverse of the standard normal cumulative distribution.
/// - `probability`: cumulative probability value in `(0, 1)`.
///
/// Returns an error when the probability is outside `(0, 1)`.
pub fn codcel_norm_s_inv(probability: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // NORMSINV is the same as NORM.S.INV
    codcel_norm_dot_s_dot_inv(probability)
}

/// Convenience wrapper for `NORMSINV` that accepts a single probability value
/// in a one-element vector.
pub fn codcel_norm_s_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("NORMSINV: Must have 1 parameter.".into());
    }

    codcel_norm_s_inv(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_s_inv_median() {
        // =NORMSINV(0.5) in US format
        // =NORMSINV(0,5) in German format
        let result = codcel_norm_s_inv(0.5).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_norm_s_inv_central_region() {
        // =NORMSINV(0.3) in US format
        // =NORMSINV(0,3) in German format
        let result = codcel_norm_s_inv(0.3).unwrap();
        println!("{result}");
        assert!((result + 0.524).abs() < 0.001);
    }

    #[test]
    fn test_norm_s_inv_lower_region() {
        // =NORMSINV(0.01) in US format
        // =NORMSINV(0,01) in German format
        let result = codcel_norm_s_inv(0.01).unwrap();
        println!("{result}");
        assert!((result + 2.326).abs() < 0.001);
    }

    #[test]
    fn test_norm_s_inv_upper_region() {
        // =NORMSINV(0.99) in US format
        // =NORMSINV(0,99) in German format
        let result = codcel_norm_s_inv(0.99).unwrap();
        println!("{result}");
        assert!((result - 2.326).abs() < 0.001);
    }

    #[test]
    fn test_norm_s_inv_very_small() {
        // =NORMSINV(0.001) in US format
        // =NORMSINV(0,001) in German format
        let result = codcel_norm_s_inv(0.001).unwrap();
        println!("{result}");
        assert!((result + 3.09).abs() < 0.01);
    }

    #[test]
    fn test_norm_s_inv_very_large() {
        // =NORMSINV(0.999) in US format
        // =NORMSINV(0,999) in German format
        let result = codcel_norm_s_inv(0.999).unwrap();
        println!("{result}");
        assert!((result - 3.09).abs() < 0.01);
    }

    #[test]
    fn test_norm_s_inv_one() {
        // =NORMSINV(1) in US format
        // =NORMSINV(1) in German format
        let result = codcel_norm_s_inv(1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_s_inv_out_of_range() {
        // =NORMSINV(1.5) in US format
        // =NORMSINV(1,5) in German format
        let result = codcel_norm_s_inv(1.5);
        assert!(result.is_err());
    }
}
