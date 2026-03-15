// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `GAUSS` that returns the probability that a standard normal random variable
/// falls between the mean (0) and z.
/// - `z`: the number of standard deviations from the mean.
///
/// Returns the probability that a value from a standard normal distribution
/// falls between 0 and z, which equals Φ(z) - 0.5.
pub fn codcel_gauss(z: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // GAUSS function calculates the probability that a standard normal random variable
    // is in the range between the mean (0) and z standard deviations from the mean.
    if z.is_nan() {
        return Err("GAUSS: Input must not be NaN.".into());
    }

    // Standard normal cumulative distribution function
    let cdf = statrs::distribution::Normal::new(0.0, 1.0)
        .map_err(|_| "GAUSS: Failed to create standard normal distribution.")?
        .cdf(z);

    // Subtract 0.5 to get the cumulative probability for the range (0, z)
    Ok(cdf - 0.5)
}

pub fn codcel_gauss_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("GAUSS: Must have 1 parameter.".into());
    }

    codcel_gauss(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gauss_zero() {
        // =GAUSS(0) in US format
        // =GAUSS(0) in German format
        let result = codcel_gauss(0.0).unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_positive() {
        // =GAUSS(1) in US format
        // =GAUSS(1) in German format
        let result = codcel_gauss(1.0).unwrap();
        assert!((result - 0.3413447460685429).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_negative() {
        // =GAUSS(-1) in US format
        // =GAUSS(-1) in German format
        let result = codcel_gauss(-1.0).unwrap();
        assert!((result - (-0.3413447460685429)).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_large_positive() {
        // =GAUSS(3) in US format
        // =GAUSS(3) in German format
        let result = codcel_gauss(3.0).unwrap();
        assert!((result - 0.4986501019683699).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_large_negative() {
        // =GAUSS(-3) in US format
        // =GAUSS(-3) in German format
        let result = codcel_gauss(-3.0).unwrap();
        assert!((result - (-0.4986501019683699)).abs() < 1e-10);
    }

    #[test]
    fn test_gauss_decimal() {
        // =GAUSS(1.5) in US format
        // =GAUSS(1,5) in German format
        let result = codcel_gauss(1.5).unwrap();
        assert!((result - 0.4331927987311419).abs() < 1e-10);
    }
}
