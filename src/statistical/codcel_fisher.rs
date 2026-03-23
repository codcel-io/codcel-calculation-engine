// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `FISHER` that returns the Fisher transformation.
/// - `x`: the value for which to compute the transformation (must be strictly between -1 and 1).
///
/// Returns the Fisher transformation: 0.5 * ln((1+x)/(1-x)),
/// or an error when x is outside the range (-1, 1).
pub fn codcel_fischer(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x <= -1.0 || x >= 1.0 {
        return Err("FISHER: x must be in the range (-1, 1).".into());
    }

    Ok(0.5 * (crate::portable_math::ln(1.0 + x) - crate::portable_math::ln(1.0 - x)))
}

pub fn codcel_fischer_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("FISHER: Must have 1 parameter".into());
    }

    codcel_fischer(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fischer_basic() {
        // =FISHER(0.5) in US format
        // =FISHER(0,5) in German format
        let result = codcel_fischer(0.5).unwrap();
        println!("{result}");
        assert!((result - 0.5493).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_zero() {
        // =FISHER(0) in US format
        // =FISHER(0) in German format
        let result = codcel_fischer(0.0).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_negative() {
        // =FISHER(-0.5) in US format
        // =FISHER(-0,5) in German format
        let result = codcel_fischer(-0.5).unwrap();
        println!("{result}");
        assert!((result - (-0.5493)).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_close_to_limit_positive() {
        // =FISHER(0.999) in US format
        // =FISHER(0,999) in German format
        let result = codcel_fischer(0.999).unwrap();
        println!("{result}");
        assert!((result - 3.8002).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_close_to_limit_negative() {
        // =FISHER(-0.999) in US format
        // =FISHER(-0,999) in German format
        let result = codcel_fischer(-0.999).unwrap();
        println!("{result}");
        assert!((result - (-3.8002)).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_invalid_high() {
        // x >= 1 should return an error
        let result = codcel_fischer(1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_fischer_invalid_low() {
        // x <= -1 should return an error
        let result = codcel_fischer(-1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_fischer_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![0.5];
        let result = codcel_fischer_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 0.5493).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![0.5, 0.6];
        let result = codcel_fischer_vec(inputs);
        assert!(result.is_err());
    }
}
