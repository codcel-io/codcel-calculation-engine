// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;
use std::ops::{Add, Sub};

/// Excel-compatible `FISHERINV` that returns the inverse of the Fisher transformation.
/// - `y`: the value for which to compute the inverse transformation.
///
/// Returns the inverse Fisher transformation: (e^(2y) - 1) / (e^(2y) + 1),
/// which maps values back to the range (-1, 1).
pub fn codcel_fischer_inv(y: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok((2.0 * y).exp().sub(1.0) / (2.0 * y).exp().add(1.0))
}

pub fn codcel_fischer_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("FISHERINV: Must have 1 parameter".into());
    }

    codcel_fischer_inv(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fischer_inv_basic() {
        // =FISHERINV(0.5493) in US format
        // =FISHERINV(0,5493) in German format
        let result = codcel_fischer_inv(0.5493).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_inv_zero() {
        // =FISHERINV(0) in US format
        // =FISHERINV(0) in German format
        let result = codcel_fischer_inv(0.0).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_inv_negative() {
        // =FISHERINV(-0.5493) in US format
        // =FISHERINV(-0,5493) in German format
        let result = codcel_fischer_inv(-0.5493).unwrap();
        println!("{result}");
        assert!((result - (-0.5)).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_inv_large_positive() {
        // =FISHERINV(3.8002) in US format
        // =FISHERINV(3,8002) in German format
        let result = codcel_fischer_inv(3.8002).unwrap();
        println!("{result}");
        assert!((result - 0.999).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_inv_large_negative() {
        // =FISHERINV(-3.8002) in US format
        // =FISHERINV(-3,8002) in German format
        let result = codcel_fischer_inv(-3.8002).unwrap();
        println!("{result}");
        assert!((result - (-0.999)).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_inv_very_large() {
        // =FISHERINV(100) in US format
        // =FISHERINV(100) in German format
        let result = codcel_fischer_inv(100.0).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0000001);
    }

    #[test]
    fn test_fischer_inv_very_small() {
        // =FISHERINV(-100) in US format
        // =FISHERINV(-100) in German format
        let result = codcel_fischer_inv(-100.0).unwrap();
        println!("{result}");
        assert!((result - (-1.0)).abs() < 0.0000001);
    }

    #[test]
    fn test_fischer_inv_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![0.5493];
        let result = codcel_fischer_inv_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_fischer_inv_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![0.5, 0.6];
        let result = codcel_fischer_inv_vec(inputs);
        assert!(result.is_err());
    }
}
