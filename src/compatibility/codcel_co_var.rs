// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::{CompensatedSum, CompensatedSumExt};
use std::error::Error;

/// Excel-compatible `COVAR`/`COVARIANCE.P` function.
/// Returns the population covariance between two data sets.
/// - `array1`: first array of numeric values.
/// - `array2`: second array of numeric values.
///
/// Both arrays must be non-empty and have the same length.
///
/// Returns an error if arrays differ in length or are empty.
pub fn codcel_co_var(
    array1: Vec<f64>,
    array2: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if array1.len() != array2.len() {
        return Err("COVAR: Arrays must have the same length".into());
    }

    if array1.is_empty() {
        return Err("COVAR: Arrays cannot be empty".into());
    }

    let mean1: f64 = array1.iter().compensated_sum() / array1.len() as f64;
    let mean2: f64 = array2.iter().compensated_sum() / array2.len() as f64;

    let mut covariance = CompensatedSum::new();

    for (x1, x2) in array1.iter().zip(array2.iter()) {
        covariance.add((x1 - mean1) * (x2 - mean2));
    }

    Ok(covariance.total() / array1.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_co_var_basic() {
        // =COVAR({2,4,6,8},{1,3,5,7}) in US format
        // =COVAR({2;4;6;8};{1;3;5;7}) in German format
        let array1 = vec![2.0, 4.0, 6.0, 8.0];
        let array2 = vec![1.0, 3.0, 5.0, 7.0];
        let result = codcel_co_var(array1, array2).unwrap();
        println!("{result}");
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_co_var_negative_values() {
        // =COVAR({-2,-1,0,1,2},{2,1,0,-1,-2}) in US format
        // =COVAR({-2;-1;0;1;2};{2;1;0;-1;-2}) in German format
        let array1 = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let array2 = vec![2.0, 1.0, 0.0, -1.0, -2.0];
        let result = codcel_co_var(array1, array2).unwrap();
        println!("{result}");
        assert!((result - (-2.0)).abs() < 0.0001);
    }

    #[test]
    fn test_co_var_same_array() {
        // =COVAR({1,2,3,4,5},{1,2,3,4,5}) in US format
        // =COVAR({1;2;3;4;5};{1;2;3;4;5}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_co_var(array1, array2).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_co_var_uncorrelated() {
        // =COVAR({1,2,3,4,5},{5,5,5,5,5}) in US format
        // =COVAR({1;2;3;4;5};{5;5;5;5;5}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_co_var(array1, array2).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_co_var_decimal_values() {
        // =COVAR({1.5,2.5,3.5,4.5},{2.5,3.5,4.5,5.5}) in US format
        // =COVAR({1,5;2,5;3,5;4,5};{2,5;3,5;4,5;5,5}) in German format
        let array1 = vec![1.5, 2.5, 3.5, 4.5];
        let array2 = vec![2.5, 3.5, 4.5, 5.5];
        let result = codcel_co_var(array1, array2).unwrap();
        println!("{result}");
        assert!((result - 1.25).abs() < 0.0001);
    }

    #[test]
    fn test_co_var_different_lengths() {
        // Different lengths should return an error
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_co_var(array1, array2);
        assert!(result.is_err());
    }

    #[test]
    fn test_co_var_empty_arrays() {
        // Empty arrays should return an error
        let array1: Vec<f64> = vec![];
        let array2: Vec<f64> = vec![];
        let result = codcel_co_var(array1, array2);
        assert!(result.is_err());
    }
}
