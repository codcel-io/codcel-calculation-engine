// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::standard_normal::std_normal_pdf;
use std::error::Error;

/// Excel-compatible `PHI` that returns the value of the density function for a standard normal distribution.
/// - `x`: the value for which to evaluate the density function.
///
/// Returns the probability density of the standard normal distribution at x.
pub fn codcel_phi(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(std_normal_pdf(x))
}

pub fn codcel_phi_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("PHI: Must have 1 parameter.".into());
    }

    codcel_phi(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi_zero() {
        // =PHI(0) in US format
        // =PHI(0) in German format
        let result = codcel_phi(0.0).unwrap();
        assert!((result - 0.3989422804014327).abs() < 0.0001);
    }

    #[test]
    fn test_phi_positive() {
        // =PHI(1) in US format
        // =PHI(1) in German format
        let result = codcel_phi(1.0).unwrap();
        assert!((result - 0.24197072451914337).abs() < 0.0001);
    }

    #[test]
    fn test_phi_negative() {
        // =PHI(-1) in US format
        // =PHI(-1) in German format
        let result = codcel_phi(-1.0).unwrap();
        assert!((result - 0.24197072451914337).abs() < 0.0001);
    }

    #[test]
    fn test_phi_large_positive() {
        // =PHI(3) in US format
        // =PHI(3) in German format
        let result = codcel_phi(3.0).unwrap();
        assert!((result - 0.004431848411938008).abs() < 0.0001);
    }

    #[test]
    fn test_phi_large_negative() {
        // =PHI(-3) in US format
        // =PHI(-3) in German format
        let result = codcel_phi(-3.0).unwrap();
        assert!((result - 0.004431848411938008).abs() < 0.0001);
    }

    #[test]
    fn test_phi_decimal() {
        // =PHI(0.5) in US format
        // =PHI(0,5) in German format
        let result = codcel_phi(0.5).unwrap();
        assert!((result - 0.3520653267642995).abs() < 0.0001);
    }

    #[test]
    fn test_phi_vec_valid() {
        // Test the vector version with valid input
        let inputs = vec![0.0];
        let result = codcel_phi_vec(inputs).unwrap();
        assert!((result - 0.3989422804014327).abs() < 0.0001);
    }

    #[test]
    fn test_phi_vec_invalid() {
        // Test the vector version with invalid input (too many parameters)
        let inputs = vec![0.0, 1.0];
        let result = codcel_phi_vec(inputs);
        assert!(result.is_err());
    }
}
