// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `GAMMALN.PRECISE` that returns the natural logarithm of the gamma function.
/// - `x`: the value at which to evaluate ln(Γ(x)) (must be > 0).
///
/// Returns ln(Γ(x)), or an error when x is not positive.
/// This function provides higher precision than GAMMALN for certain inputs.
pub fn codcel_gamma_ln_precise(x: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x <= 0.0 {
        return Err("GAMMALN.PRECISE: x must be greater than 0.".into());
    }

    Ok(statrs::function::gamma::ln_gamma(x))
}

pub fn codcel_gamma_ln_precise_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("GAMMALN.PRECISE: Must have 1 parameter.".into());
    }

    codcel_gamma_ln_precise(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gamma_ln_precise_basic() {
        // =GAMMALN.PRECISE(1) in US format
        // =GAMMALN.PRECISE(1) in German format
        let result = codcel_gamma_ln_precise(1.0).unwrap();
        assert!((result - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_gamma_ln_precise_small_value() {
        // =GAMMALN.PRECISE(0.5) in US format
        // =GAMMALN.PRECISE(0,5) in German format
        let result = codcel_gamma_ln_precise(0.5).unwrap();
        assert!((result - 0.5723649429247001).abs() < 1e-10);
    }

    #[test]
    fn test_gamma_ln_precise_large_value() {
        // =GAMMALN.PRECISE(10) in US format
        // =GAMMALN.PRECISE(10) in German format
        let result = codcel_gamma_ln_precise(10.0).unwrap();
        assert!((result - 12.801827480081469).abs() < 1e-10);
    }

    #[test]
    fn test_gamma_ln_precise_decimal_value() {
        // =GAMMALN.PRECISE(2.5) in US format
        // =GAMMALN.PRECISE(2,5) in German format
        let result = codcel_gamma_ln_precise(2.5).unwrap();
        println!("{result}");
        assert!((result - 0.2846828704729223).abs() < 1e-10);
    }

    #[test]
    fn test_gamma_ln_precise_error() {
        // =GAMMALN.PRECISE(0) in US format
        // =GAMMALN.PRECISE(0) in German format
        let result = codcel_gamma_ln_precise(0.0);
        assert!(result.is_err());

        // =GAMMALN.PRECISE(-1) in US format
        // =GAMMALN.PRECISE(-1) in German format
        let result = codcel_gamma_ln_precise(-1.0);
        assert!(result.is_err());
    }
}
