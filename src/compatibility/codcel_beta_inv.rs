// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `BETAINV`/`BETA.INV` function.
/// Computes the inverse of the cumulative beta distribution.
/// - `probability`: cumulative probability value in the range `[0, 1]`.
/// - `alpha` / `beta`: shape parameters that must be positive.
/// - `a` / `b`: optional lower/upper bounds (default 0 and 1).
///
/// Returns an error when probability is outside `[0, 1]`, parameters are non-positive, or `a >= b`.
pub fn codcel_beta_inv(
    probability: f64,
    alpha: f64,
    beta: f64,
    a: Option<f64>,
    b: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    check_value_f64("BETAINV probability", probability)?;
    if !(0.0..=1.0).contains(&probability) {
        return Err("BETAINV: Probability must be in the range [0, 1].".into());
    }

    check_value_f64("BETAINV alpha", alpha)?;
    if alpha <= 0.0 {
        return Err("BETAINV: Alpha must be greater than 0.".into());
    }

    check_value_f64("BETAINV beta", beta)?;
    if beta <= 0.0 {
        return Err("BETAINV: Beta must be greater than 0.".into());
    }

    let a = a.unwrap_or(0.0);
    let b = b.unwrap_or(1.0);

    check_value_f64("BETAINV a", a)?;
    check_value_f64("BETAINV b", b)?;

    if a >= b {
        return Err("BETAINV: a must be less than b.".into());
    }

    // Handle edge cases
    if probability == 0.0 {
        return Ok(a);
    }
    if probability == 1.0 {
        return Ok(b);
    }

    // Create beta distribution and calculate inverse CDF
    match statrs::distribution::Beta::new(alpha, beta) {
        Ok(dist) => {
            let x = dist.inverse_cdf(probability);
            if x.is_nan() {
                Err("BETAINV: Failed to compute inverse beta distribution.".into())
            } else {
                Ok(a + (b - a) * x)
            }
        }
        Err(_) => Err("BETAINV: Error creating beta distribution.".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beta_inv_basic() {
        // =BETAINV(0.5, 2, 3) in US format
        // =BETAINV(0,5; 2; 3) in German format
        let result = codcel_beta_inv(0.5, 2.0, 3.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 0.3857275681323894).abs() < 0.0001);
    }

    #[test]
    fn test_beta_inv_with_bounds() {
        // =BETAINV(0.5, 2, 3, 1, 5) in US format
        // =BETAINV(0,5; 2; 3; 1; 5) in German format
        let result = codcel_beta_inv(0.5, 2.0, 3.0, Some(1.0), Some(5.0)).unwrap();
        println!("{result}");
        assert!((result - 2.5429102725295576).abs() < 0.0001);
    }

    #[test]
    fn test_beta_inv_small_probability() {
        // =BETAINV(0.1, 2, 3) in US format
        // =BETAINV(0,1; 2; 3) in German format
        let result = codcel_beta_inv(0.1, 2.0, 3.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 0.1425593167100307).abs() < 0.0001);
    }

    #[test]
    fn test_beta_inv_large_probability() {
        // =BETAINV(0.9, 2, 3) in US format
        // =BETAINV(0,9; 2; 3) in German format
        let result = codcel_beta_inv(0.9, 2.0, 3.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 0.6795394162781818).abs() < 0.0001);
    }

    #[test]
    fn test_beta_inv_different_alpha() {
        // =BETAINV(0.5, 3, 3) in US format
        // =BETAINV(0,5; 3; 3) in German format
        let result = codcel_beta_inv(0.5, 3.0, 3.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_beta_inv_different_beta() {
        // =BETAINV(0.5, 2, 4) in US format
        // =BETAINV(0,5; 2; 4) in German format
        let result = codcel_beta_inv(0.5, 2.0, 4.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 0.3138101704556977).abs() < 0.0001);
    }

    #[test]
    fn test_beta_inv_zero_probability() {
        // =BETAINV(0, 2, 3) in US format
        // =BETAINV(0; 2; 3) in German format
        let result = codcel_beta_inv(0.0, 2.0, 3.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_beta_inv_one_probability() {
        // =BETAINV(1, 2, 3) in US format
        // =BETAINV(1; 2; 3) in German format
        let result = codcel_beta_inv(1.0, 2.0, 3.0, None, None).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_beta_inv_negative_alpha() {
        // Negative alpha should return an error
        let result = codcel_beta_inv(0.5, -2.0, 3.0, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_beta_inv_negative_beta() {
        // Negative beta should return an error
        let result = codcel_beta_inv(0.5, 2.0, -3.0, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_beta_inv_invalid_bounds() {
        // a >= b should return an error
        let result = codcel_beta_inv(0.5, 2.0, 3.0, Some(5.0), Some(5.0));
        assert!(result.is_err());
    }
}
