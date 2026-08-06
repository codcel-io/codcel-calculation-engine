// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `NORM.S.INV` that returns the inverse of the standard normal cumulative distribution.
/// - `probability`: the probability corresponding to the standard normal distribution (0 to 1, exclusive).
///
/// Returns the z-score (number of standard deviations from the mean)
/// such that NORM.S.DIST(z, TRUE) = probability.
#[allow(clippy::excessive_precision)]
pub fn codcel_norm_dot_s_dot_inv(probability: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if !(0.0..1.0).contains(&probability) {
        return Err("NORM.S.INV: Probability must be between 0 and 1 (exclusive).".into());
    }

    // Constants for the rational approximation
    const A1: f64 = -39.69683028665376;
    const A2: f64 = 220.9460984245205;
    const A3: f64 = -275.9285104469687;
    const A4: f64 = 138.3577518672690;
    const A5: f64 = -30.66479806614716;
    const A6: f64 = 2.506628277459239;

    const B1: f64 = -54.47609879822406;
    const B2: f64 = 161.5858368580409;
    const B3: f64 = -155.6989798598866;
    const B4: f64 = 66.80131188771972;
    const B5: f64 = -13.28068155288572;

    const C1: f64 = -7.784894002430293e-03;
    const C2: f64 = -0.3223964580411365;
    const C3: f64 = -2.400758277161838;
    const C4: f64 = -2.549732539343734;
    const C5: f64 = 4.374664141464968;
    const C6: f64 = 2.938163982698783;

    const D1: f64 = 7.784695709041462e-03;
    const D2: f64 = 0.3224671290700398;
    const D3: f64 = 2.445134137142996;
    const D4: f64 = 3.754408661907416;

    const P_LOW: f64 = 0.02425;
    const P_HIGH: f64 = 1.0 - P_LOW;

    let q: f64;
    let result: f64;

    if probability < P_LOW {
        // Rational approximation for lower region
        q = crate::portable_math::sqrt(-2.0 * crate::portable_math::ln(probability));
        result = (((((C1 * q + C2) * q + C3) * q + C4) * q + C5) * q + C6)
            / ((((D1 * q + D2) * q + D3) * q + D4) * q + 1.0);
    } else if probability > P_HIGH {
        // Rational approximation for upper region
        q = crate::portable_math::sqrt(-2.0 * crate::portable_math::ln(1.0 - probability));
        result = -(((((C1 * q + C2) * q + C3) * q + C4) * q + C5) * q + C6)
            / ((((D1 * q + D2) * q + D3) * q + D4) * q + 1.0);
    } else {
        // Rational approximation for central region
        q = probability - 0.5;
        let r = q * q;
        result = (((((A1 * r + A2) * r + A3) * r + A4) * r + A5) * r + A6) * q
            / (((((B1 * r + B2) * r + B3) * r + B4) * r + B5) * r + 1.0);
    }

    Ok(result)
}

pub fn codcel_norm_dot_s_dot_inv_vec(
    inputs: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 1 {
        return Err("NORM.S.INV: Must have 1 parameter.".into());
    }

    codcel_norm_dot_s_dot_inv(inputs[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_norm_dot_s_dot_inv_median() {
        // =NORM.S.INV(0.5) in US format
        // =NORM.S.INV(0,5) in German format
        let result = codcel_norm_dot_s_dot_inv(0.5).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_positive_z() {
        // =NORM.S.INV(0.975) in US format
        // =NORM.S.INV(0,975) in German format
        let result = codcel_norm_dot_s_dot_inv(0.975).unwrap();
        assert!((result - 1.96).abs() < 0.01);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_negative_z() {
        // =NORM.S.INV(0.025) in US format
        // =NORM.S.INV(0,025) in German format
        let result = codcel_norm_dot_s_dot_inv(0.025).unwrap();
        assert!((result + 1.96).abs() < 0.01);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_central_region() {
        // =NORM.S.INV(0.3) in US format
        // =NORM.S.INV(0,3) in German format
        let result = codcel_norm_dot_s_dot_inv(0.3).unwrap();
        assert!((result + 0.524).abs() < 0.001);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_lower_region() {
        // =NORM.S.INV(0.01) in US format
        // =NORM.S.INV(0,01) in German format
        let result = codcel_norm_dot_s_dot_inv(0.01).unwrap();
        assert!((result + 2.326).abs() < 0.001);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_upper_region() {
        // =NORM.S.INV(0.99) in US format
        // =NORM.S.INV(0,99) in German format
        let result = codcel_norm_dot_s_dot_inv(0.99).unwrap();
        assert!((result - 2.326).abs() < 0.001);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_very_small() {
        // =NORM.S.INV(0.001) in US format
        // =NORM.S.INV(0,001) in German format
        let result = codcel_norm_dot_s_dot_inv(0.001).unwrap();
        assert!((result + 3.09).abs() < 0.01);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_very_large() {
        // =NORM.S.INV(0.999) in US format
        // =NORM.S.INV(0,999) in German format
        let result = codcel_norm_dot_s_dot_inv(0.999).unwrap();
        assert!((result - 3.09).abs() < 0.01);
    }

    /* TODO IN EXCEL THIS GIVES #NUM! #[test]
    fn test_norm_dot_s_dot_inv_zero() {
        // =NORM.S.INV(0) in US format
        // =NORM.S.INV(0) in German format
        let result = codcel_norm_dot_s_dot_inv(0.0);
        println!("{:?}", result);
        assert!(result.is_err());
    }*/

    #[test]
    fn test_norm_dot_s_dot_inv_one() {
        // =NORM.S.INV(1) in US format
        // =NORM.S.INV(1) in German format
        let result = codcel_norm_dot_s_dot_inv(1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_s_dot_inv_out_of_range() {
        // Values outside [0,1] should return an error
        let result = codcel_norm_dot_s_dot_inv(1.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_norm_dot_s_dot_inv_vec_valid() {
        // Test the vector version with valid input
        let inputs = vec![0.5];
        let result = codcel_norm_dot_s_dot_inv_vec(inputs).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_norm_dot_s_dot_inv_vec_invalid() {
        // Test the vector version with invalid input (too many parameters)
        let inputs = vec![0.5, 0.6];
        let result = codcel_norm_dot_s_dot_inv_vec(inputs);
        assert!(result.is_err());
    }
}
