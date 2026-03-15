// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `CORREL` that returns the correlation coefficient between two data sets.
/// - `x`: the first array of values.
/// - `y`: the second array of values (must have the same length as `x`).
///
/// Returns the Pearson correlation coefficient (between -1 and 1),
/// or an error when arrays are empty, have different lengths, or have zero variance.
pub fn codcel_correl(x: Vec<f64>, y: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if x.is_empty() || y.is_empty() {
        return Err("CORREL: Input arrays must not be empty.".into());
    }
    if x.len() != y.len() {
        return Err("CORREL: Input arrays must have the same length.".into());
    }

    // Calculate means
    let x_mean = x.iter().sum::<f64>() / x.len() as f64;
    let y_mean = y.iter().sum::<f64>() / y.len() as f64;

    // Calculate the correlation components
    let mut numerator = 0.0;
    let mut x_denominator = 0.0;
    let mut y_denominator = 0.0;

    for (&x_val, &y_val) in x.iter().zip(y.iter()) {
        let x_diff = x_val - x_mean;
        let y_diff = y_val - y_mean;
        numerator += x_diff * y_diff;
        x_denominator += x_diff.powi(2);
        y_denominator += y_diff.powi(2);
    }

    if x_denominator == 0.0 || y_denominator == 0.0 {
        return Err("CORREL: Division by zero.".into());
    }

    // Calculate and return the correlation coefficient
    Ok(numerator / (x_denominator.sqrt() * y_denominator.sqrt()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correl_positive_correlation() {
        // =CORREL({3,4,5,6,7},{8,9,10,11,12}) in US format
        // =CORREL({3;4;5;6;7};{8;9;10;11;12}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![8.0, 9.0, 10.0, 11.0, 12.0];
        let result = codcel_correl(x, y).unwrap();
        assert!((result - 1.0).abs() < 0.0000001);
    }

    #[test]
    fn test_correl_negative_correlation() {
        // =CORREL({3,4,5,6,7},{12,11,10,9,8}) in US format
        // =CORREL({3;4;5;6;7};{12;11;10;9;8}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![12.0, 11.0, 10.0, 9.0, 8.0];
        let result = codcel_correl(x, y).unwrap();
        assert!((result - (-1.0)).abs() < 0.0000001);
    }

    #[test]
    fn test_correl_no_correlation() {
        // =CORREL({3,4,5,6,7},{10,8,12,9,11}) in US format
        // =CORREL({3;4;5;6;7};{10;8;12;9;11}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![10.0, 8.0, 12.0, 9.0, 11.0];
        let result = codcel_correl(x, y).unwrap();
        println!("{result}");
        assert!((result - 0.3).abs() < 0.0001);
    }

    #[test]
    fn test_correl_partial_correlation() {
        // =CORREL({3,4,5,6,7},{8,7,9,10,12}) in US format
        // =CORREL({3;4;5;6;7};{8;7;9;10;12}) in German format
        let x = vec![3.0, 4.0, 5.0, 6.0, 7.0];
        let y = vec![8.0, 7.0, 9.0, 10.0, 12.0];
        let result = codcel_correl(x, y).unwrap();
        println!("{result}");
        assert!((result - 0.904194).abs() < 0.0001);
    }

    #[test]
    fn test_correl_single_pair() {
        // =CORREL({5},{10}) in US format
        // =CORREL({5};{10}) in German format
        let x = vec![5.0];
        let y = vec![10.0];
        // With a single pair, correlation is undefined (division by zero)
        let result = codcel_correl(x, y);
        assert!(result.is_err());
    }

    #[test]
    fn test_correl_empty_arrays() {
        // Empty arrays should return an error
        let x: Vec<f64> = vec![];
        let y: Vec<f64> = vec![];
        let result = codcel_correl(x, y);
        assert!(result.is_err());
    }

    #[test]
    fn test_correl_different_lengths() {
        // Arrays of different lengths should return an error
        let x = vec![3.0, 4.0, 5.0];
        let y = vec![8.0, 9.0, 10.0, 11.0];
        let result = codcel_correl(x, y);
        assert!(result.is_err());
    }

    #[test]
    fn test_correl_constant_array() {
        // When one array is constant, correlation is undefined (division by zero)
        let x = vec![5.0, 5.0, 5.0, 5.0];
        let y = vec![8.0, 9.0, 10.0, 11.0];
        let result = codcel_correl(x, y);
        assert!(result.is_err());
    }
}
