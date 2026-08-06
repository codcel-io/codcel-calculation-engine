// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::function::erf::erf;
use std::error::Error;

/// Excel-compatible `ERF.PRECISE` that returns the error function integrated from 0 to x.
/// - `x`: the upper integration bound.
///   Returns the error function value erf(x).
pub fn codcel_erf_precise(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(erf(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erf_precise_positive() {
        // =ERF.PRECISE(0.5) in US format
        // =ERF.PRECISE(0,5) in German format
        let result = codcel_erf_precise(0.5).unwrap();
        println!("{result}");
        assert!((result - 0.5204998778130465).abs() < 0.0001);
    }

    #[test]
    fn test_erf_precise_zero() {
        // =ERF.PRECISE(0) in US format
        // =ERF.PRECISE(0) in German format
        let result = codcel_erf_precise(0.0).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_erf_precise_negative() {
        // =ERF.PRECISE(-0.5) in US format
        // =ERF.PRECISE(-0,5) in German format
        let result = codcel_erf_precise(-0.5).unwrap();
        println!("{result}");
        assert!((result + 0.5204998778130465).abs() < 0.0001);
    }

    #[test]
    fn test_erf_precise_large() {
        // =ERF.PRECISE(2) in US format
        // =ERF.PRECISE(2) in German format
        let result = codcel_erf_precise(2.0).unwrap();
        println!("{result}");
        assert!((result - 0.9953222650189527).abs() < 0.0001);
    }

    #[test]
    fn test_erf_precise_very_large() {
        // =ERF.PRECISE(5) in US format
        // =ERF.PRECISE(5) in German format
        let result = codcel_erf_precise(5.0).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }
}
