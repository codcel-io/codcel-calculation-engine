// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `GAMMA` that returns the gamma function value.
/// - `x`: the value at which to evaluate the gamma function.
///
/// Returns Γ(x), which for positive integers equals (x-1)!,
/// or an error when x is a non-positive integer.
pub fn codcel_gamma(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x <= 0.0 && x == x.floor() {
        return Err("GAMMA: x must not be a non-positive integer.".into());
    }

    Ok(libm::tgamma(x))
}

pub fn codcel_gamma_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("GAMMA: Must have 1 parameter".into());
    }

    codcel_gamma(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_basic() {
        // =GAMMA(2.5) in US format
        // =GAMMA(2,5) in German format
        let result = codcel_gamma(2.5).unwrap();
        println!("{result}");
        assert!((result - 1.3293).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_integer() {
        // =GAMMA(5) in US format
        // =GAMMA(5) in German format
        let result = codcel_gamma(5.0).unwrap();
        println!("{result}");
        assert!((result - 24.0).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_small_value() {
        // =GAMMA(0.5) in US format
        // =GAMMA(0,5) in German format
        let result = codcel_gamma(0.5).unwrap();
        println!("{result}");
        assert!((result - 1.7725).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_negative_non_integer() {
        // =GAMMA(-0.5) in US format
        // =GAMMA(-0,5) in German format
        let result = codcel_gamma(-0.5).unwrap();
        println!("{result}");
        assert!((result - (-3.5449)).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_negative_integer() {
        // Negative integers should return an error
        let result = codcel_gamma(-5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_zero() {
        // Zero should return an error
        let result = codcel_gamma(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![2.5];
        let result = codcel_gamma_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 1.3293).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![2.5, 3.0];
        let result = codcel_gamma_vec(inputs);
        assert!(result.is_err());
    }
}
