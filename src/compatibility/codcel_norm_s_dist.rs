// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;
use std::f64::consts::PI;

/// Excel-compatible `NORMSDIST`/`NORM.S.DIST` function (cumulative).
/// Returns the standard normal cumulative distribution.
/// - `z`: z-score value (must be finite).
///
/// Returns an error when `z` is not finite (NaN or infinite).
pub fn codcel_norm_s_dist(z: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if input is valid (not NaN or infinite)
    if !z.is_finite() {
        return Err("NORMSDIST: Input must be a finite number".into());
    }

    // Constants for the approximation
    const P: f64 = 0.2316419;
    const B1: f64 = 0.319381530;
    const B2: f64 = -0.356563782;
    const B3: f64 = 1.781477937;
    const B4: f64 = -1.821255978;
    const B5: f64 = 1.330274429;

    // If z is negative, we use the relationship CDF(z) = 1 - CDF(-z)
    let x = if z < 0.0 { -z } else { z };

    // Calculate PDF at x
    let pdf = (-x * x / 2.0).exp() / (2.0 * PI).sqrt();

    // Calculate t
    let t = 1.0 / (1.0 + P * x);

    // Calculate the polynomial approximation
    let poly = t * (B1 + t * (B2 + t * (B3 + t * (B4 + t * B5))));

    // Calculate CDF
    let cdf = 1.0 - pdf * poly;

    // Adjust if z was negative
    let result = if z < 0.0 { 1.0 - cdf } else { cdf };

    Ok(result)
}

/// Convenience wrapper for `NORMSDIST` that accepts a single-element vector.
/// Errors if the vector length is not exactly one.
pub fn codcel_norm_s_dist_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("NORMSDIST: Must have 1 parameter.".into());
    }

    codcel_norm_s_dist(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_s_dist_zero() {
        // =NORMSDIST(0) in US format
        // =NORMSDIST(0) in German format
        let result = codcel_norm_s_dist(0.0).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_norm_s_dist_positive() {
        // =NORMSDIST(1) in US format
        // =NORMSDIST(1) in German format
        let result = codcel_norm_s_dist(1.0).unwrap();
        println!("{result}");
        assert!((result - 0.8413447461).abs() < 0.0001);
    }

    #[test]
    fn test_norm_s_dist_negative() {
        // =NORMSDIST(-1) in US format
        // =NORMSDIST(-1) in German format
        let result = codcel_norm_s_dist(-1.0).unwrap();
        println!("{result}");
        assert!((result - 0.1586552539).abs() < 0.0001);
    }

    #[test]
    fn test_norm_s_dist_large_positive() {
        // =NORMSDIST(3) in US format
        // =NORMSDIST(3) in German format
        let result = codcel_norm_s_dist(3.0).unwrap();
        println!("{result}");
        assert!((result - 0.9986501020).abs() < 0.0001);
    }

    #[test]
    fn test_norm_s_dist_large_negative() {
        // =NORMSDIST(-3) in US format
        // =NORMSDIST(-3) in German format
        let result = codcel_norm_s_dist(-3.0).unwrap();
        println!("{result}");
        assert!((result - 0.0013498980).abs() < 0.0001);
    }

    #[test]
    fn test_norm_s_dist_decimal() {
        // =NORMSDIST(0.5) in US format
        // =NORMSDIST(0,5) in German format
        let result = codcel_norm_s_dist(0.5).unwrap();
        println!("{result}");
        assert!((result - 0.6914624613).abs() < 0.0001);
    }

    #[test]
    #[should_panic]
    fn test_norm_s_dist_infinity() {
        // =NORMSDIST(∞) in US format
        // =NORMSDIST(∞) in German format
        let result = codcel_norm_s_dist(f64::INFINITY).unwrap();
        println!("{result}");
    }
}
