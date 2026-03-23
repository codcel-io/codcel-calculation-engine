// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `EXP` that returns e raised to the power of a number.
/// - `x`: the exponent applied to the base e.
///
/// Returns e^x.
pub fn codcel_exp(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(crate::portable_math::exp(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exp_positive() {
        // =EXP(1) in US format
        // =EXP(1) in German format
        let result = codcel_exp(1.0).unwrap();
        assert!((result - std::f64::consts::E).abs() < 1e-10); // e^1 = e ≈ 2.718281828459045
    }

    #[test]
    fn test_exp_zero() {
        // =EXP(0) in US format
        // =EXP(0) in German format
        let result = codcel_exp(0.0).unwrap();
        assert_eq!(result, 1.0); // e^0 = 1
    }

    #[test]
    fn test_exp_negative() {
        // =EXP(-1) in US format
        // =EXP(-1) in German format
        let result = codcel_exp(-1.0).unwrap();
        assert!((result - 0.36787944117144233).abs() < 1e-10); // e^(-1) ≈ 0.36787944117144233
    }

    #[test]
    fn test_exp_decimal() {
        // =EXP(0.5) in US format
        // =EXP(0,5) in German format
        let result = codcel_exp(0.5).unwrap();
        assert!((result - 1.6487212707001282).abs() < 1e-10); // e^0.5 ≈ 1.6487212707001282
    }

    #[test]
    fn test_exp_large_positive() {
        // =EXP(10) in US format
        // =EXP(10) in German format
        let result = codcel_exp(10.0).unwrap();
        assert!((result - 22026.465794806718).abs() < 1e-10); // e^10 ≈ 22026.465794806718
    }

    #[test]
    fn test_exp_large_negative() {
        // =EXP(-10) in US format
        // =EXP(-10) in German format
        let result = codcel_exp(-10.0).unwrap();
        assert!((result - 0.00004539992976248485).abs() < 1e-10); // e^(-10) ≈ 0.00004539992976248485
    }

    #[test]
    fn test_exp_small_decimal() {
        // =EXP(0.01) in US format
        // =EXP(0,01) in German format
        let result = codcel_exp(0.01).unwrap();
        assert!((result - 1.0100501670841682).abs() < 1e-10); // e^0.01 ≈ 1.0100501670841682
    }
}
