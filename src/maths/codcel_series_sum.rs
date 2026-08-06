// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `SERIESSUM` that returns the sum of a power series.
/// - `x`: the input value of the power series.
/// - `n`: the initial power to which x is raised.
/// - `m`: the step by which to increase n for each term.
/// - `coefficients`: a list of coefficients for each term.
///
/// Returns the sum of the series or an error when coefficients is empty.
pub fn codcel_series_sum(
    x: f64,
    n: f64,
    m: f64,
    coefficients: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if coefficients.is_empty() {
        return Err("SERIESSUM: Coefficients array cannot be empty".into());
    }

    // Calculate the sum of the series
    let mut result = 0.0;
    let mut current_power = n;

    for &coefficient in &coefficients {
        result += coefficient * crate::portable_math::powf(x, current_power);
        current_power += m;
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_series_sum_polynomial() {
        // =SERIESSUM(2,0,1,{1,3,5}) in US format (calculates 1*2^0 + 3*2^1 + 5*2^2 = 1 + 6 + 20 = 27)
        // =SERIESSUM(2;0;1;{1;3;5}) in German format
        let result = codcel_series_sum(2.0, 0.0, 1.0, vec![1.0, 3.0, 5.0]).unwrap();
        assert_eq!(result, 27.0); // 1*2^0 + 3*2^1 + 5*2^2 = 1 + 6 + 20 = 27
    }

    #[test]
    fn test_series_sum_power_series() {
        // =SERIESSUM(2,1,2,{1,1,1}) in US format (calculates 1*2^1 + 1*2^3 + 1*2^5 = 2 + 8 + 32 = 42)
        // =SERIESSUM(2;1;2;{1;1;1}) in German format
        let result = codcel_series_sum(2.0, 1.0, 2.0, vec![1.0, 1.0, 1.0]).unwrap();
        assert_eq!(result, 42.0); // 1*2^1 + 1*2^3 + 1*2^5 = 2 + 8 + 32 = 42
    }

    #[test]
    fn test_series_sum_taylor_sin() {
        // Taylor series for sin(x) around x=0: x - x^3/3! + x^5/5! - ...
        // =SERIESSUM(0.5,1,2,{1,-0.166666667,0.008333333}) in US format
        // =SERIESSUM(0,5;1;2;{1;-0,166666667;0,008333333}) in German format
        let result = codcel_series_sum(0.5, 1.0, 2.0, vec![1.0, -1.0 / 6.0, 1.0 / 120.0]).unwrap();
        assert!((result - 0.47942708333333334).abs() < 1e-10); // Approximation of sin(0.5)
    }

    #[test]
    fn test_series_sum_negative_x() {
        // =SERIESSUM(-2,0,1,{1,3,5}) in US format (calculates 1*(-2)^0 + 3*(-2)^1 + 5*(-2)^2 = 1 - 6 + 20 = 15)
        // =SERIESSUM(-2;0;1;{1;3;5}) in German format
        let result = codcel_series_sum(-2.0, 0.0, 1.0, vec![1.0, 3.0, 5.0]).unwrap();
        assert_eq!(result, 15.0); // 1*(-2)^0 + 3*(-2)^1 + 5*(-2)^2 = 1 - 6 + 20 = 15
    }

    #[test]
    fn test_series_sum_negative_powers() {
        // =SERIESSUM(2,-2,1,{1,2,3}) in US format (calculates 1*2^(-2) + 2*2^(-1) + 3*2^0 = 0.25 + 1 + 3 = 4.25)
        // =SERIESSUM(2;-2;1;{1;2;3}) in German format
        let result = codcel_series_sum(2.0, -2.0, 1.0, vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(result, 4.25); // 1*2^(-2) + 2*2^(-1) + 3*2^0 = 0.25 + 1 + 3 = 4.25
    }

    #[test]
    fn test_series_sum_single_coefficient() {
        // =SERIESSUM(3,2,1,{4}) in US format (calculates 4*3^2 = 4*9 = 36)
        // =SERIESSUM(3;2;1;{4}) in German format
        let result = codcel_series_sum(3.0, 2.0, 1.0, vec![4.0]).unwrap();
        assert_eq!(result, 36.0); // 4*3^2 = 4*9 = 36
    }

    #[test]
    fn test_series_sum_zero_x() {
        // =SERIESSUM(0,1,1,{1,2,3}) in US format (calculates 1*0^1 + 2*0^2 + 3*0^3 = 0)
        // =SERIESSUM(0;1;1;{1;2;3}) in German format
        let result = codcel_series_sum(0.0, 1.0, 1.0, vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(result, 0.0); // 1*0^1 + 2*0^2 + 3*0^3 = 0
    }

    #[test]
    fn test_series_sum_zero_step() {
        // =SERIESSUM(2,1,0,{1,2,3}) in US format (calculates 1*2^1 + 2*2^1 + 3*2^1 = 2 + 4 + 6 = 12)
        // =SERIESSUM(2;1;0;{1;2;3}) in German format
        let result = codcel_series_sum(2.0, 1.0, 0.0, vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(result, 12.0); // 1*2^1 + 2*2^1 + 3*2^1 = 2 + 4 + 6 = 12
    }

    #[test]
    fn test_series_sum_empty_coefficients() {
        // =SERIESSUM(2,1,1,{}) in US format (returns #VALUE! error)
        // =SERIESSUM(2;1;1;{}) in German format (returns #VALUE! error)
        let result = codcel_series_sum(2.0, 1.0, 1.0, vec![]);
        assert!(result.is_err());
    }
}
