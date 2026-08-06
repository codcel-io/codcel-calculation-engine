// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `FINV`/`F.INV.RT` function.
/// Computes the inverse right-tailed F statistic.
/// - `p`: right-tailed probability value in `(0, 1)`.
/// - `d1`: numerator degrees of freedom (must be greater than 0).
/// - `d2`: denominator degrees of freedom (must be greater than 0).
///
/// Returns an error when `p` is outside `(0, 1)` or degrees of freedom are not positive.
pub fn codcel_f_inv(p: f64, d1: f64, d2: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if !(0.0 < p && p < 1.0) {
        return Err("FINV: Probability (p) must be between 0 and 1".into());
    }
    if d1 <= 0.0 {
        return Err(
            "FINV: Degrees of freedom for the numerator (d1) must be greater than 0".into(),
        );
    }
    if d2 <= 0.0 {
        return Err(
            "FINV: Degrees of freedom for the denominator (d2) must be greater than 0".into(),
        );
    }

    let f_dist = statrs::distribution::FisherSnedecor::new(d1, d2)
        .map_err(|_| "FINV: Invalid degrees of freedom")?;
    let f_value = f_dist.inverse_cdf(1.0 - p);

    Ok(f_value)
}

/// Convenience wrapper for `FINV` that accepts `[p, d1, d2]` in a vector.
/// Errors if the vector does not contain exactly three values.
pub fn codcel_f_inv_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("FINV: Must have 3 parameters.".into());
    }

    codcel_f_inv(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_inv_basic() {
        // =FINV(0.05, 3, 5) in US format
        // =FINV(0,05; 3; 5) in German format
        let result = codcel_f_inv(0.05, 3.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 5.4094).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_small_p() {
        // =FINV(0.01, 3, 5) in US format
        // =FINV(0,01; 3; 5) in German format
        let result = codcel_f_inv(0.01, 3.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 12.059953691651945).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_large_p() {
        // =FINV(0.9, 3, 5) in US format
        // =FINV(0,9; 3; 5) in German format
        let result = codcel_f_inv(0.9, 3.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 0.18835381894483483).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_small_d1() {
        // =FINV(0.05, 1, 5) in US format
        // =FINV(0,05; 1; 5) in German format
        let result = codcel_f_inv(0.05, 1.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 6.6079).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_large_d1() {
        // =FINV(0.05, 10, 5) in US format
        // =FINV(0,05; 10; 5) in German format
        let result = codcel_f_inv(0.05, 10.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 4.7351).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_small_d2() {
        // =FINV(0.05, 3, 1) in US format
        // =FINV(0,05; 3; 1) in German format
        let result = codcel_f_inv(0.05, 3.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 215.7073).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_large_d2() {
        // =FINV(0.05, 3, 20) in US format
        // =FINV(0,05; 3; 20) in German format
        let result = codcel_f_inv(0.05, 3.0, 20.0).unwrap();
        println!("{result}");
        assert!((result - 3.0984).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_p_zero() {
        // p = 0 should return an error
        let result = codcel_f_inv(0.0, 3.0, 5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_inv_p_one() {
        // p = 1 should return an error
        let result = codcel_f_inv(1.0, 3.0, 5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_inv_zero_d1() {
        // Zero d1 should return an error
        let result = codcel_f_inv(0.05, 0.0, 5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_inv_negative_d1() {
        // Negative d1 should return an error
        let result = codcel_f_inv(0.05, -1.0, 5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_inv_zero_d2() {
        // Zero d2 should return an error
        let result = codcel_f_inv(0.05, 3.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_inv_negative_d2() {
        // Negative d2 should return an error
        let result = codcel_f_inv(0.05, 3.0, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_inv_vec_basic() {
        // =FINV(0.05, 3, 5) in US format
        // =FINV(0,05; 3; 5) in German format
        let result = codcel_f_inv_vec(vec![0.05, 3.0, 5.0]).unwrap();
        println!("{result}");
        assert!((result - 5.4094).abs() < 0.0001);
    }

    #[test]
    fn test_f_inv_vec_wrong_params() {
        // Wrong number of parameters should return an error
        let result = codcel_f_inv_vec(vec![0.05, 3.0]);
        assert!(result.is_err());
    }
}
