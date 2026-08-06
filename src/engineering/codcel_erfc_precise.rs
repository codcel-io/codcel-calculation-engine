// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::function::erf::erf;
use std::error::Error;

/// Excel-compatible `ERFC.PRECISE` that returns the complementary error function.
/// - `x`: the lower integration bound.
///   Returns `1 - erf(x)`, which is the integral of the error function from `x` to infinity.
pub fn codcel_erfc_precise(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(1.0 - erf(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erfc_precise_positive() {
        // =ERFC.PRECISE(0.5) in US format
        // =ERFC.PRECISE(0,5) in German format
        let result = codcel_erfc_precise(0.5).unwrap();
        println!("{result}");
        assert!((result - 0.4795001221869535).abs() < 0.0001);
    }

    #[test]
    fn test_erfc_precise_zero() {
        // =ERFC.PRECISE(0) in US format
        // =ERFC.PRECISE(0) in German format
        let result = codcel_erfc_precise(0.0).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_erfc_precise_negative() {
        // =ERFC.PRECISE(-0.5) in US format
        // =ERFC.PRECISE(-0,5) in German format
        let result = codcel_erfc_precise(-0.5).unwrap();
        println!("{result}");
        assert!((result - 1.5204998778130465).abs() < 0.0001);
    }

    #[test]
    fn test_erfc_precise_large() {
        // =ERFC.PRECISE(2) in US format
        // =ERFC.PRECISE(2) in German format
        let result = codcel_erfc_precise(2.0).unwrap();
        println!("{result}");
        assert!((result - 0.0046777349810472645).abs() < 0.0001);
    }

    #[test]
    fn test_erfc_precise_very_large() {
        // =ERFC.PRECISE(5) in US format
        // =ERFC.PRECISE(5) in German format
        let result = codcel_erfc_precise(5.0).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }
}
