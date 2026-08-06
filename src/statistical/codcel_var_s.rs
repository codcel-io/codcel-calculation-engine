// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `VAR.S` that returns the variance based on a sample.
/// - `data`: an array of numeric values representing a sample (must have at least 2 values).
///
/// Returns the sample variance (divides by n-1),
/// or an error when the data has fewer than 2 values.
pub fn codcel_var_s(data: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if data.len() < 2 {
        return Err("VAR.S: Data set must contain at least two elements.".into());
    }

    let mean = data.iter().sum::<f64>() / data.len() as f64;

    let variance =
        data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / (data.len() as f64 - 1.0);

    Ok(variance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_s_basic() {
        // =VAR.S(1,2,3,4,5) in US format
        // =VAR.S(1;2;3;4;5) in German format
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_var_s(data).unwrap();
        println!("{result}");
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_var_s_specific_variance() {
        // =VAR.S(4,5,8,7,11,4,3) in US format
        // =VAR.S(4;5;8;7;11;4;3) in German format
        let data = vec![4.0, 5.0, 8.0, 7.0, 11.0, 4.0, 3.0];
        let result = codcel_var_s(data).unwrap();
        println!("{result}");
        assert!((result - 8.0).abs() < 0.0001);
    }

    #[test]
    fn test_var_s_negative_values() {
        // =VAR.S(-1,-2,-3,-4,-5) in US format
        // =VAR.S(-1;-2;-3;-4;-5) in German format
        let data = vec![-1.0, -2.0, -3.0, -4.0, -5.0];
        let result = codcel_var_s(data).unwrap();
        println!("{result}");
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_var_s_mixed_values() {
        // =VAR.S(-2,-1,0,1,2) in US format
        // =VAR.S(-2;-1;0;1;2) in German format
        let data = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let result = codcel_var_s(data).unwrap();
        println!("{result}");
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_var_s_two_values() {
        // =VAR.S(1,2) in US format
        // =VAR.S(1;2) in German format
        let data = vec![1.0, 2.0];
        let result = codcel_var_s(data).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_var_s_empty_data() {
        // Empty data should return an error
        let data: Vec<f64> = vec![];
        let result = codcel_var_s(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_var_s_single_value() {
        // Single value should return an error (need at least 2 values)
        let data = vec![5.0];
        let result = codcel_var_s(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_var_s_compare_with_var_p() {
        // VAR.S should be larger than VAR.P for the same data
        // This test doesn't directly call VAR.P but verifies the mathematical relationship
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_var_s(data.clone()).unwrap();
        let n = data.len() as f64;
        let mean = data.iter().sum::<f64>() / n;
        let var_p = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / n;
        let expected_var_s = var_p * n / (n - 1.0);
        println!("VAR.S: {result}, Expected: {expected_var_s}");
        assert!((result - expected_var_s).abs() < 0.0001);
    }
}
