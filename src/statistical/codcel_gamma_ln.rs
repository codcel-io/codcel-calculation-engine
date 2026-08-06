// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `GAMMALN` that returns the natural logarithm of the gamma function.
/// - `x`: the value at which to evaluate ln(Γ(x)) (must be > 0).
///
/// Returns ln(Γ(x)), or an error when x is not positive.
pub fn codcel_gamma_ln(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x <= 0.0 {
        return Err("GAMMALN: x must be greater than 0.".into());
    }

    Ok(statrs::function::gamma::ln_gamma(x))
}

pub fn codcel_gamma_ln_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("GAMMALN: Must have 1 parameter.".into());
    }

    codcel_gamma_ln(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_ln_basic() {
        // =GAMMALN(2.5) in US format
        // =GAMMALN(2,5) in German format
        let result = codcel_gamma_ln(2.5).unwrap();
        println!("{result}");
        assert!((result - 0.2846).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_ln_integer() {
        // =GAMMALN(5) in US format
        // =GAMMALN(5) in German format
        let result = codcel_gamma_ln(5.0).unwrap();
        println!("{result}");
        assert!((result - 3.1781).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_ln_small_value() {
        // =GAMMALN(0.5) in US format
        // =GAMMALN(0,5) in German format
        let result = codcel_gamma_ln(0.5).unwrap();
        println!("{result}");
        assert!((result - 0.5724).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_ln_large_value() {
        // =GAMMALN(100) in US format
        // =GAMMALN(100) in German format
        let result = codcel_gamma_ln(100.0).unwrap();
        println!("{result}");
        assert!((result - 359.1342).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_ln_zero() {
        // Zero should return an error
        let result = codcel_gamma_ln(0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_ln_negative() {
        // Negative value should return an error
        let result = codcel_gamma_ln(-1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gamma_ln_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![2.5];
        let result = codcel_gamma_ln_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 0.2846).abs() < 0.0001);
    }

    #[test]
    fn test_gamma_ln_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![2.5, 3.0];
        let result = codcel_gamma_ln_vec(inputs);
        assert!(result.is_err());
    }
}
