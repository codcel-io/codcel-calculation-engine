// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::function::erf::erf;
use std::error::Error;

/// Excel-compatible `ERF` that returns the error function integrated between two limits.
/// - `lower`: the lower integration bound (or the upper bound if `upper` is not specified).
/// - `upper`: optional upper integration bound; if omitted, integrates from `0` to `lower`.
///   Returns the error function value between the specified limits.
pub fn codcel_erf(lower: f64, upper: Option<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    match upper {
        Some(upper_val) => {
            // If upper bound is provided, calculate ERF between lower and upper bounds
            let erf_lower = erf(lower);
            let erf_upper = erf(upper_val);
            Ok(erf_upper - erf_lower)
        }
        None => {
            // If no upper bound is given, calculate ERF between 0 and the lower bound
            Ok(erf(lower))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erf_single_parameter() {
        // =ERF(0.5) in US format
        // =ERF(0,5) in German format
        let result = codcel_erf(0.5, None).unwrap();
        println!("{result}");
        assert!((result - 0.5204998778130465).abs() < 0.0001);
    }

    #[test]
    fn test_erf_two_parameters() {
        // =ERF(0.5, 1) in US format
        // =ERF(0,5; 1) in German format
        let result = codcel_erf(0.5, Some(1.0)).unwrap();
        println!("{result}");
        assert!((result - 0.32220091517906113).abs() < 0.0001);
    }

    #[test]
    fn test_erf_zero() {
        // =ERF(0) in US format
        // =ERF(0) in German format
        let result = codcel_erf(0.0, None).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_erf_negative() {
        // =ERF(-0.5) in US format
        // =ERF(-0,5) in German format
        let result = codcel_erf(-0.5, None).unwrap();
        println!("{result}");
        assert!((result + 0.5204998778130465).abs() < 0.0001);
    }

    #[test]
    fn test_erf_negative_range() {
        // =ERF(-1, -0.5) in US format
        // =ERF(-1; -0,5) in German format
        let result = codcel_erf(-1.0, Some(-0.5)).unwrap();
        println!("{result}");
        assert!((result - 0.32220091517906113).abs() < 0.0001);
    }
}
