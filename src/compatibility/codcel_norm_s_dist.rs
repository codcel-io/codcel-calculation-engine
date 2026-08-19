// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::codcel_norm_dot_s_dot_dist::codcel_norm_dot_s_dot_dist;
use std::error::Error;

/// Excel-compatible `NORMSDIST`/`NORM.S.DIST` function (cumulative).
/// Returns the standard normal cumulative distribution.
/// - `z`: z-score value (must be finite).
///
/// Returns an error when `z` is not finite (NaN or infinite).
pub fn codcel_norm_s_dist(z: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Checked here rather than left to NORM.S.DIST so the message names the function the caller
    // actually wrote.
    if !z.is_finite() {
        return Err("NORMSDIST: Input must be a finite number".into());
    }

    // NORMSDIST is the cumulative branch of NORM.S.DIST.
    codcel_norm_dot_s_dot_dist(z, true)
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

    // Expected values were computed with mpmath at 60 decimal digits and rounded to the nearest
    // f64.

    #[test]
    fn test_norm_s_dist_zero() {
        // =NORMSDIST(0) in US format
        // =NORMSDIST(0) in German format
        let result = codcel_norm_s_dist(0.0).unwrap();
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_norm_s_dist_positive() {
        // =NORMSDIST(1) in US format
        // =NORMSDIST(1) in German format
        let result = codcel_norm_s_dist(1.0).unwrap();
        assert!((result - 0.8413447460685429).abs() < 1e-15);
    }

    #[test]
    fn test_norm_s_dist_negative() {
        // =NORMSDIST(-1) in US format
        // =NORMSDIST(-1) in German format
        let result = codcel_norm_s_dist(-1.0).unwrap();
        assert!((result - 0.15865525393145705).abs() < 1e-16);
    }

    #[test]
    fn test_norm_s_dist_large_positive() {
        // =NORMSDIST(3) in US format
        // =NORMSDIST(3) in German format
        let result = codcel_norm_s_dist(3.0).unwrap();
        assert!((result - 0.9986501019683699).abs() < 1e-15);
    }

    #[test]
    fn test_norm_s_dist_large_negative() {
        // =NORMSDIST(-3) in US format
        // =NORMSDIST(-3) in German format
        let result = codcel_norm_s_dist(-3.0).unwrap();
        assert!((result - 0.0013498980316300946).abs() < 1e-17);
    }

    #[test]
    fn test_norm_s_dist_decimal() {
        // =NORMSDIST(0.5) in US format
        // =NORMSDIST(0,5) in German format
        let result = codcel_norm_s_dist(0.5).unwrap();
        assert!((result - 0.6914624612740131).abs() < 1e-16);
    }

    #[test]
    fn test_norm_s_dist_agrees_with_norm_dot_s_dot_dist() {
        // The two names are one Excel function; they must not drift apart again.
        for z in [-5.0, -2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0, 5.0] {
            assert_eq!(
                codcel_norm_s_dist(z).unwrap(),
                codcel_norm_dot_s_dot_dist(z, true).unwrap()
            );
        }
    }

    #[test]
    fn test_norm_s_dist_infinity() {
        // =NORMSDIST(∞) in US format
        // =NORMSDIST(∞) in German format
        let result = codcel_norm_s_dist(f64::INFINITY);
        assert!(result.is_err());
    }
}
