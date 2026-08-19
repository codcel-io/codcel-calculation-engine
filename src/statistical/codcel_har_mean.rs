// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use std::error::Error;

/// Excel-compatible `HARMEAN` that returns the harmonic mean of a set of positive values.
/// - `values`: an array of positive numeric values.
///
/// Returns the harmonic mean (reciprocal of the arithmetic mean of reciprocals),
/// or an error when the array is empty or contains non-positive values.
pub fn codcel_har_mean(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("HARMEAN: Input vector must not be empty.".into());
    }
    if values.iter().any(|&x| x <= 0.0) {
        return Err("HARMEAN: All input values must be greater than 0.".into());
    }

    let reciprocal_sum: f64 = values.iter().map(|&x| 1.0 / x).compensated_sum();
    let n: f64 = values.len() as f64;

    Ok(n / reciprocal_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_har_mean_basic() {
        // =HARMEAN(1, 2, 3, 4) in US format
        // =HARMEAN(1; 2; 3; 4) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_har_mean(values).unwrap();
        assert!((result - 1.9200000000000004).abs() < 1e-10);
    }

    #[test]
    fn test_har_mean_same_values() {
        // =HARMEAN(2, 2, 2, 2) in US format
        // =HARMEAN(2; 2; 2; 2) in German format
        let values = vec![2.0, 2.0, 2.0, 2.0];
        let result = codcel_har_mean(values).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_har_mean_single_value() {
        // =HARMEAN(5) in US format
        // =HARMEAN(5) in German format
        let values = vec![5.0];
        let result = codcel_har_mean(values).unwrap();
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_har_mean_decimal_values() {
        // =HARMEAN(1.5, 2.5, 3.5) in US format
        // =HARMEAN(1,5; 2,5; 3,5) in German format
        let values = vec![1.5, 2.5, 3.5];
        let result = codcel_har_mean(values).unwrap();
        println!("{result}");
        assert!((result - 2.2183098591549295).abs() < 1e-10);
    }

    #[test]
    fn test_har_mean_large_values() {
        // =HARMEAN(100, 1000, 10000) in US format
        // =HARMEAN(100; 1000; 10000) in German format
        let values = vec![100.0, 1000.0, 10000.0];
        let result = codcel_har_mean(values).unwrap();
        println!("{result}");
        assert!((result - 270.2702702702703).abs() < 1e-10);
    }

    #[test]
    fn test_har_mean_negative_value() {
        // =HARMEAN(1, 2, -3) in US format
        // =HARMEAN(1; 2; -3) in German format
        let values = vec![1.0, 2.0, -3.0];
        let result = codcel_har_mean(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_har_mean_zero_value() {
        // =HARMEAN(1, 2, 0) in US format
        // =HARMEAN(1; 2; 0) in German format
        let values = vec![1.0, 2.0, 0.0];
        let result = codcel_har_mean(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_har_mean_empty() {
        // =HARMEAN() in US format
        // =HARMEAN() in German format
        let values: Vec<f64> = vec![];
        let result = codcel_har_mean(values);
        assert!(result.is_err());
    }
}
