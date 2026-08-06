// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `FDIST`/`F.DIST.RT` function.
/// Returns the right-tailed probability from the F distribution.
/// - `x`: F-statistic value (must be non-negative).
/// - `d1`: numerator degrees of freedom (must be greater than 0).
/// - `d2`: denominator degrees of freedom (must be greater than 0).
///
/// Returns an error on negative `x` or non-positive degrees of freedom.
pub fn codcel_f_dist(x: f64, d1: f64, d2: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x < 0.0 {
        return Err("FDIST: F-statistic (x) must be non-negative".into());
    }
    if d1 <= 0.0 {
        return Err(
            "FDIST: Degrees of freedom for the numerator (d1) must be greater than 0".into(),
        );
    }
    if d2 <= 0.0 {
        return Err(
            "FDIST: Degrees of freedom for the denominator (d2) must be greater than 0".into(),
        );
    }

    let f_dist = statrs::distribution::FisherSnedecor::new(d1, d2)
        .map_err(|_| "FDIST: Invalid degrees of freedom")?;
    let p_value = 1.0 - f_dist.cdf(x);

    Ok(p_value)
}

/// Convenience wrapper for `FDIST` that accepts `[x, d1, d2]` in a vector.
/// Errors if the vector does not contain exactly three values.
pub fn codcel_f_dist_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("FDIST: Must have 3 parameters.".into());
    }

    codcel_f_dist(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_dist_basic() {
        // =FDIST(2, 3, 5) in US format
        // =FDIST(2; 3; 5) in German format
        let result = codcel_f_dist(2.0, 3.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 0.2326239180000771).abs() < 0.0001);
    }

    #[test]
    fn test_f_dist_small_x() {
        // =FDIST(0.5, 3, 5) in US format
        // =FDIST(0,5; 3; 5) in German format
        let result = codcel_f_dist(0.5, 3.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 0.6984526373049262).abs() < 0.0001);
    }

    #[test]
    fn test_f_dist_large_x() {
        // =FDIST(10, 3, 5) in US format
        // =FDIST(10; 3; 5) in German format
        let result = codcel_f_dist(10.0, 3.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 0.014888525723791513).abs() < 0.0001);
    }

    #[test]
    fn test_f_dist_small_d1() {
        // =FDIST(2, 1, 5) in US format
        // =FDIST(2; 1; 5) in German format
        let result = codcel_f_dist(2.0, 1.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 0.2165063).abs() < 0.0001);
    }

    #[test]
    fn test_f_dist_large_d1() {
        // =FDIST(2, 10, 5) in US format
        // =FDIST(2; 10; 5) in German format
        let result = codcel_f_dist(2.0, 10.0, 5.0).unwrap();
        println!("{result}");
        assert!((result - 0.22997511934989712).abs() < 0.0001);
    }

    #[test]
    fn test_f_dist_small_d2() {
        // =FDIST(2, 3, 1) in US format
        // =FDIST(2; 3; 1) in German format
        let result = codcel_f_dist(2.0, 3.0, 1.0).unwrap();
        println!("{result}");
        assert!((result - 0.4695222290670419).abs() < 0.0001);
    }

    #[test]
    fn test_f_dist_large_d2() {
        // =FDIST(2, 3, 20) in US format
        // =FDIST(2; 3; 20) in German format
        let result = codcel_f_dist(2.0, 3.0, 20.0).unwrap();
        println!("{result}");
        assert!((result - 0.1463929).abs() < 0.0001);
    }

    #[test]
    fn test_f_dist_zero_x() {
        // =FDIST(0, 3, 5) in US format
        // =FDIST(0; 3; 5) in German format
        let result = codcel_f_dist(0.0, 3.0, 5.0).unwrap();
        println!("{result}");
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_f_dist_negative_x() {
        // Negative x should return an error
        let result = codcel_f_dist(-1.0, 3.0, 5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dist_zero_d1() {
        // Zero d1 should return an error
        let result = codcel_f_dist(2.0, 0.0, 5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dist_negative_d1() {
        // Negative d1 should return an error
        let result = codcel_f_dist(2.0, -1.0, 5.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dist_zero_d2() {
        // Zero d2 should return an error
        let result = codcel_f_dist(2.0, 3.0, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dist_negative_d2() {
        // Negative d2 should return an error
        let result = codcel_f_dist(2.0, 3.0, -1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dist_vec_basic() {
        // =FDIST(2, 3, 5) in US format
        // =FDIST(2; 3; 5) in German format
        let result = codcel_f_dist_vec(vec![2.0, 3.0, 5.0]).unwrap();
        println!("{result}");
        assert!((result - 0.2326239180000771).abs() < 0.0001);
    }

    #[test]
    fn test_f_dist_vec_wrong_params() {
        // Wrong number of parameters should return an error
        let result = codcel_f_dist_vec(vec![2.0, 3.0]);
        assert!(result.is_err());
    }
}
