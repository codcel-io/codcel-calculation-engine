// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use statrs::function::erf::erf;
use std::error::Error;

/// Excel-compatible `ERFC` that returns the complementary error function.
/// - `x`: the lower integration bound.
///   Returns `1 - erf(x)`, which is the integral of the error function from `x` to infinity.
pub fn codcel_erfc(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(1.0 - erf(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erfc_positive() {
        // =ERFC(0.5) in US format
        // =ERFC(0,5) in German format
        let result = codcel_erfc(0.5).unwrap();
        println!("{result}");
        assert!((result - 0.4795001221869535).abs() < 0.0001);
    }

    #[test]
    fn test_erfc_zero() {
        // =ERFC(0) in US format
        // =ERFC(0) in German format
        let result = codcel_erfc(0.0).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_erfc_negative() {
        // =ERFC(-0.5) in US format
        // =ERFC(-0,5) in German format
        let result = codcel_erfc(-0.5).unwrap();
        println!("{result}");
        assert!((result - 1.5204998778130465).abs() < 0.0001);
    }

    #[test]
    fn test_erfc_large() {
        // =ERFC(2) in US format
        // =ERFC(2) in German format
        let result = codcel_erfc(2.0).unwrap();
        println!("{result}");
        assert!((result - 0.0046777349810472645).abs() < 0.0001);
    }

    #[test]
    fn test_erfc_very_large() {
        // =ERFC(5) in US format
        // =ERFC(5) in German format
        let result = codcel_erfc(5.0).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }
}
