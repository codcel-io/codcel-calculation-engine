// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use std::error::Error;

/// Excel-compatible `SUM` that adds all the numbers in a range.
/// - `values`: a list of numbers to add.
///
/// Returns the sum of all values (0 if empty).
pub fn codcel_sum(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(0.0);
    }

    Ok(values.iter().compensated_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_positive_numbers() {
        // =SUM(1,2,3,4,5) in US format
        // =SUM(1;2;3;4;5) in German format
        let result = codcel_sum(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_sum_negative_numbers() {
        // =SUM(-1,-2,-3,-4,-5) in US format
        // =SUM(-1;-2;-3;-4;-5) in German format
        let result = codcel_sum(vec![-1.0, -2.0, -3.0, -4.0, -5.0]).unwrap();
        assert_eq!(result, -15.0);
    }

    #[test]
    fn test_sum_mixed_numbers() {
        // =SUM(1,-2,3,-4,5) in US format
        // =SUM(1;-2;3;-4;5) in German format
        let result = codcel_sum(vec![1.0, -2.0, 3.0, -4.0, 5.0]).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_sum_decimals() {
        // =SUM(1.1,2.2,3.3) in US format
        // =SUM(1,1;2,2;3,3) in German format
        let result = codcel_sum(vec![1.1, 2.2, 3.3]).unwrap();
        assert!((result - 6.6).abs() < 1e-10);
    }

    #[test]
    fn test_sum_single_number() {
        // =SUM(42) in US format
        // =SUM(42) in German format
        let result = codcel_sum(vec![42.0]).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_sum_empty() {
        // =SUM() in US format
        // =SUM() in German format
        let result = codcel_sum(vec![]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sum_large_numbers() {
        // =SUM(1000000,2000000) in US format
        // =SUM(1000000;2000000) in German format
        let result = codcel_sum(vec![1000000.0, 2000000.0]).unwrap();
        assert_eq!(result, 3000000.0);
    }

    #[test]
    fn test_sum_recovers_value_lost_to_cancellation() {
        // =SUM(1E+16,1,-1E+16) in US format
        // =SUM(1E+16;1;-1E+16) in German format
        // A left-to-right fold rounds the 1 away entirely and returns 0.
        let values = vec![1e16, 1.0, -1e16];
        assert_eq!(values.iter().sum::<f64>(), 0.0);
        assert_eq!(codcel_sum(values).unwrap(), 1.0);
    }

    #[test]
    fn test_sum_no_drift_over_long_range() {
        // =SUM(A1:A10000) where every cell is 0.1, in US format
        // =SUM(A1:A10000) where every cell is 0,1, in German format
        let values = vec![0.1; 10_000];
        assert!((values.iter().sum::<f64>() - 1000.0).abs() > 0.0);
        assert_eq!(codcel_sum(values).unwrap(), 1000.0);
    }

    #[test]
    fn test_sum_preserves_infinity() {
        // =SUM(1,1E+308*10) in US format — Excel surfaces an error from an overflowing sum
        // =SUM(1;1E+308*10) in German format
        let result = codcel_sum(vec![1.0, f64::INFINITY]).unwrap();
        assert!(result.is_infinite() && result.is_sign_positive());
    }

    #[test]
    fn test_sum_small_decimals() {
        // =SUM(0.0001,0.0002,0.0003) in US format
        // =SUM(0,0001;0,0002;0,0003) in German format
        let result = codcel_sum(vec![0.0001, 0.0002, 0.0003]).unwrap();
        assert!((result - 0.0006).abs() < 1e-10);
    }
}
