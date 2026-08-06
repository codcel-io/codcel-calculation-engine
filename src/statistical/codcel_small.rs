// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `SMALL` that returns the k-th smallest value in a data set.
/// - `values`: an array of numeric values.
/// - `k`: the position (from the smallest) in the array to return (1-based).
///
/// Returns the k-th smallest value, or an error when k is out of range.
pub fn codcel_small(values: Vec<f64>, k: i32) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let k = k as usize;
    if k < 1 || k > values.len() {
        return Err("SMALL: k is out of the range of the input values.".into());
    }

    let mut sorted_values = values;
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    Ok(sorted_values[k - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_basic() {
        // =SMALL({1,2,3,4,5}, 1) in US format
        // =SMALL({1;2;3;4;5}; 1) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_small(values, 1).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_small_second_smallest() {
        // =SMALL({1,2,3,4,5}, 2) in US format
        // =SMALL({1;2;3;4;5}; 2) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_small(values, 2).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_small_last() {
        // =SMALL({1,2,3,4,5}, 5) in US format
        // =SMALL({1;2;3;4;5}; 5) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_small(values, 5).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_small_unsorted_values() {
        // =SMALL({5,3,1,4,2}, 3) in US format
        // =SMALL({5;3;1;4;2}; 3) in German format
        let values = vec![5.0, 3.0, 1.0, 4.0, 2.0];
        let result = codcel_small(values, 3).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_small_duplicate_values() {
        // =SMALL({5,5,3,3,1}, 2) in US format
        // =SMALL({5;5;3;3;1}; 2) in German format
        let values = vec![5.0, 5.0, 3.0, 3.0, 1.0];
        let result = codcel_small(values, 2).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_small_negative_values() {
        // =SMALL({-1,-2,-3,-4,-5}, 1) in US format
        // =SMALL({-1;-2;-3;-4;-5}; 1) in German format
        let values = vec![-1.0, -2.0, -3.0, -4.0, -5.0];
        let result = codcel_small(values, 1).unwrap();
        assert_eq!(result, -5.0);
    }

    #[test]
    fn test_small_decimal_values() {
        // =SMALL({1.5,2.5,3.5,4.5,5.5}, 3) in US format
        // =SMALL({1,5;2,5;3,5;4,5;5,5}; 3) in German format
        let values = vec![1.5, 2.5, 3.5, 4.5, 5.5];
        let result = codcel_small(values, 3).unwrap();
        assert_eq!(result, 3.5);
    }

    #[test]
    fn test_small_k_too_large() {
        // =SMALL({1,2,3,4,5}, 6) in US format
        // =SMALL({1;2;3;4;5}; 6) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_small(values, 6);
        assert!(result.is_err());
    }

    #[test]
    fn test_small_k_zero() {
        // =SMALL({1,2,3,4,5}, 0) in US format
        // =SMALL({1;2;3;4;5}; 0) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_small(values, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_small_k_negative() {
        // =SMALL({1,2,3,4,5}, -1) in US format
        // =SMALL({1;2;3;4;5}; -1) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_small(values, -1);
        assert!(result.is_err());
    }
}
