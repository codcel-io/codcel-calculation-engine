// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `COVARIANCE.S` that returns the sample covariance of two data sets.
/// - `x`: the first array of values.
/// - `y`: the second array of values (must have the same length as `x`).
///
/// Returns the sample covariance (divides by n-1),
/// or an error when arrays are empty or have different lengths.
pub fn codcel_covariance_s(x: Vec<f64>, y: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if x.is_empty() || y.is_empty() {
        return Err("COVARIANCE.S: Input arrays must not be empty.".into());
    }
    if x.len() != y.len() {
        return Err("COVARIANCE.S: Input arrays must have the same length.".into());
    }

    // Calculate means
    let x_mean = x.iter().sum::<f64>() / x.len() as f64;
    let y_mean = y.iter().sum::<f64>() / y.len() as f64;

    // Calculate covariance
    let mut covariance = 0.0;

    for (&x_val, &y_val) in x.iter().zip(y.iter()) {
        covariance += (x_val - x_mean) * (y_val - y_mean);
    }

    Ok(covariance / (x.len() - 1) as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covariance_s_positive_covariance() {
        // =COVARIANCE.S({3,4,5,6,7},{8,9,10,11,12}) in US format
        // =COVARIANCE.S({3;4;5;6;7};{8;9;10;11;12}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![8.0, 9.0, 10.0, 11.0, 12.0];
        let result = codcel_covariance_s(x, y).unwrap();
        assert!((result - 2.5).abs() < 0.0000001);
    }

    #[test]
    fn test_covariance_s_negative_covariance() {
        // =COVARIANCE.S({3,4,5,6,7},{12,11,10,9,8}) in US format
        // =COVARIANCE.S({3;4;5;6;7};{12;11;10;9;8}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![12.0, 11.0, 10.0, 9.0, 8.0];
        let result = codcel_covariance_s(x, y).unwrap();
        assert!((result - (-2.5)).abs() < 0.0000001);
    }

    #[test]
    fn test_covariance_s_no_covariance() {
        // =COVARIANCE.S({3,4,5,6,7},{10,10,10,10,10}) in US format
        // =COVARIANCE.S({3;4;5;6;7};{10;10;10;10;10}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![10.0, 10.0, 10.0, 10.0, 10.0];
        let result = codcel_covariance_s(x, y).unwrap();
        assert!((result - 0.0).abs() < 0.0000001);
    }

    #[test]
    fn test_covariance_s_mixed_values() {
        // =COVARIANCE.S({3,4,5,6,7},{8,7,9,10,12}) in US format
        // =COVARIANCE.S({3;4;5;6;7};{8;7;9;10;12}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![8.0, 7.0, 9.0, 10.0, 12.0];
        let result = codcel_covariance_s(x, y).unwrap();
        println!("{result}");
        assert!((result - 2.75).abs() < 0.0001);
    }

    #[test]
    fn test_covariance_s_single_pair() {
        // =COVARIANCE.S({5},{10}) in US format
        // =COVARIANCE.S({5};{10}) in German format
        let x = vec![5.0];
        let y = vec![10.0];
        // With a single pair, division by (n-1) where n=1 would cause division by zero
        let result = codcel_covariance_s(x, y);
        assert!(result.is_err() || result.unwrap().is_nan());
    }

    #[test]
    fn test_covariance_s_empty_arrays() {
        // Empty arrays should return an error
        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];
        let result = codcel_covariance_s(x, y);
        assert!(result.is_err());
    }

    #[test]
    fn test_covariance_s_different_lengths() {
        // Arrays of different lengths should return an error
        let x = vec![3.0, 4.0, 5.0];
        let y = vec![8.0, 9.0, 10.0, 11.0];
        let result = codcel_covariance_s(x, y);
        assert!(result.is_err());
    }

    #[test]
    fn test_covariance_s_decimal_values() {
        // =COVARIANCE.S({2.5,3.5,4.5,5.5,6.5},{7.5,8.5,9.5,10.5,11.5}) in US format
        // =COVARIANCE.S({2,5;3,5;4,5;5,5;6,5};{7,5;8,5;9,5;10,5;11,5}) in German format
        let x = vec![2.5, 3.5, 4.5, 5.5, 6.5];
        let y = vec![7.5, 8.5, 9.5, 10.5, 11.5];
        let result = codcel_covariance_s(x, y).unwrap();
        assert!((result - 2.5).abs() < 0.0000001);
    }

    #[test]
    fn test_covariance_s_compare_with_p() {
        // Sample covariance should be larger than population covariance by a factor of n/(n-1)
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![8.0, 9.0, 10.0, 11.0, 12.0];
        let result_s = codcel_covariance_s(x.clone(), y.clone()).unwrap();

        // Calculate population covariance manually
        let x_mean = x.iter().sum::<f64>() / x.len() as f64;
        let y_mean = y.iter().sum::<f64>() / y.len() as f64;
        let mut covariance_p = 0.0;
        for (&x_val, &y_val) in x.iter().zip(y.iter()) {
            covariance_p += (x_val - x_mean) * (y_val - y_mean);
        }
        covariance_p /= x.len() as f64;

        // Check that result_s = covariance_p * n/(n-1)
        let n = x.len() as f64;
        assert!((result_s - covariance_p * n / (n - 1.0)).abs() < 0.0000001);
    }
}
