// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `MEDIAN` that returns the median of the given numbers.
/// - `values`: an array of numeric values.
///
/// Returns the middle value when sorted (or the average of the two middle values if even count),
/// or an error when the input is empty.
pub fn codcel_median(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("MEDIAN: Input vector is empty.".into());
    }

    let mut sorted_values = values;
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let len = sorted_values.len();
    let median = if len.is_multiple_of(2) {
        // Average of the two middle elements
        (sorted_values[len / 2 - 1] + sorted_values[len / 2]) / 2.0
    } else {
        // Middle element
        sorted_values[len / 2]
    };

    Ok(median)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_odd_count() {
        // =MEDIAN(1,2,3,4,5) in US format
        // =MEDIAN(1;2;3;4;5) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_median(values).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_median_even_count() {
        // =MEDIAN(1,2,3,4) in US format
        // =MEDIAN(1;2;3;4) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_median(values).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_median_unsorted() {
        // =MEDIAN(5,3,1,4,2) in US format
        // =MEDIAN(5;3;1;4;2) in German format
        let values = vec![5.0, 3.0, 1.0, 4.0, 2.0];
        let result = codcel_median(values).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_median_negative_values() {
        // =MEDIAN(-5,-4,-3,-2,-1) in US format
        // =MEDIAN(-5;-4;-3;-2;-1) in German format
        let values = vec![-5.0, -4.0, -3.0, -2.0, -1.0];
        let result = codcel_median(values).unwrap();
        assert_eq!(result, -3.0);
    }

    #[test]
    fn test_median_mixed_values() {
        // =MEDIAN(-3,-1,0,2,4) in US format
        // =MEDIAN(-3;-1;0;2;4) in German format
        let values = vec![-3.0, -1.0, 0.0, 2.0, 4.0];
        let result = codcel_median(values).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_median_decimal_values() {
        // =MEDIAN(1.1,2.2,3.3,4.4,5.5) in US format
        // =MEDIAN(1,1;2,2;3,3;4,4;5,5) in German format
        let values = vec![1.1, 2.2, 3.3, 4.4, 5.5];
        let result = codcel_median(values).unwrap();
        assert_eq!(result, 3.3);
    }

    #[test]
    fn test_median_single_value() {
        // =MEDIAN(42) in US format
        // =MEDIAN(42) in German format
        let values = vec![42.0];
        let result = codcel_median(values).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_median_empty_dataset() {
        // Empty dataset should return an error
        let values: Vec<f64> = vec![];
        let result = codcel_median(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_median_same_values() {
        // =MEDIAN(5,5,5,5,5) in US format
        // =MEDIAN(5;5;5;5;5) in German format
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_median(values).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_median_large_dataset() {
        // =MEDIAN(1,2,3,4,5,6,7,8,9,10,11) in US format
        // =MEDIAN(1;2;3;4;5;6;7;8;9;10;11) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0];
        let result = codcel_median(values).unwrap();
        assert_eq!(result, 6.0);
    }
}
