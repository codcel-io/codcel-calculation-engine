// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `PERCENTRANK.INC` that returns the rank of a value as a percentage (inclusive).
/// - `array`: an array of numeric values.
/// - `value`: the value for which to determine the rank.
/// - `significance`: optional number of significant digits for the returned percentage (default 3).
///
/// Returns the rank of the value as a percentage (0 to 1, inclusive),
/// or an error when the array is empty or value is outside the range.
pub fn codcel_percent_rank_inc(
    array: Vec<f64>,
    value: f64,
    significance: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("PERCENTRANK.INC: The array cannot be empty.".into());
    }

    let significance = significance.unwrap_or(3);

    if significance < 1 {
        return Err("PERCENTRANK.INC: Significance must be >= 1.".into());
    }

    let mut sorted_array = array.clone();
    sorted_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if value < sorted_array[0] || value > sorted_array[sorted_array.len() - 1] {
        return Err("PERCENTRANK.INC: Value is out of the range of the array.".into());
    }

    let n = sorted_array.len() as f64;

    // Find position and calculate inclusive rank using rank / (n - 1)
    let mut percent_rank = 0.0;

    for (i, &x) in sorted_array.iter().enumerate() {
        if x == value {
            // Exact match: inclusive rank = (0-based position) / (n - 1)
            percent_rank = (i as f64) / (n - 1.0);
            break;
        } else if x > value {
            // Interpolation between adjacent positions
            let lower_index = i.saturating_sub(1);
            let lower_value = sorted_array[lower_index];
            let upper_value = x;
            let lower_rank = (lower_index as f64) / (n - 1.0);
            let upper_rank = (i as f64) / (n - 1.0);
            let fraction = (value - lower_value) / (upper_value - lower_value);
            percent_rank = lower_rank + fraction * (upper_rank - lower_rank);
            break;
        }
    }

    // Truncate (not round) to the specified number of significant digits
    let factor = 10f64.powi(significance);
    Ok((factor * percent_rank).floor() / factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_rank_inc_basic() {
        // =PERCENTRANK.INC({1,2,3,4,5}, 3) in US format
        // =PERCENTRANK.INC({1;2;3;4;5}; 3) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_inc(array, 3.0, None).unwrap();
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_inc_unsorted() {
        // =PERCENTRANK.INC({5,3,1,4,2}, 3) in US format
        // =PERCENTRANK.INC({5;3;1;4;2}; 3) in German format
        let array = vec![5.0, 3.0, 1.0, 4.0, 2.0];
        let result = codcel_percent_rank_inc(array, 3.0, None).unwrap();
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_inc_min_value() {
        // =PERCENTRANK.INC({1,2,3,4,5}, 1) in US format
        // =PERCENTRANK.INC({1;2;3;4;5}; 1) in German format
        // Inclusive: 0/(5-1) = 0.0
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_inc(array, 1.0, None).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_inc_max_value() {
        // =PERCENTRANK.INC({1,2,3,4,5}, 5) in US format
        // =PERCENTRANK.INC({1;2;3;4;5}; 5) in German format
        // Inclusive: 4/(5-1) = 1.0
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_inc(array, 5.0, None).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_inc_interpolation() {
        // =PERCENTRANK.INC({1,2,3,4,5}, 2.5) in US format
        // =PERCENTRANK.INC({1;2;3;4;5}; 2,5) in German format
        // Ranks: 1/4=0.25, 2/4=0.5, interpolated: 0.25 + 0.5*(0.5-0.25) = 0.375
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_inc(array, 2.5, None).unwrap();
        assert!((result - 0.375).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_inc_custom_significance() {
        // =PERCENTRANK.INC({1,2,3,4,5}, 2.5, 2) in US format
        // =PERCENTRANK.INC({1;2;3;4;5}; 2,5; 2) in German format
        // 0.375 truncated to 2 digits = 0.37
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_inc(array, 2.5, Some(2)).unwrap();
        assert!((result - 0.37).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_inc_empty_array() {
        // Empty array should return an error
        let array: Vec<f64> = vec![];
        let result = codcel_percent_rank_inc(array, 3.0, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_percent_rank_inc_value_out_of_range_low() {
        // Value below the minimum should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_inc(array, 0.5, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_percent_rank_inc_value_out_of_range_high() {
        // Value above the maximum should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_inc(array, 5.5, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_percent_rank_inc_zero_significance() {
        // Significance < 1 should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_inc(array, 3.0, Some(0));
        assert!(result.is_err());
    }

    #[test]
    fn test_percent_rank_inc_duplicate_values() {
        // =PERCENTRANK.INC({1,2,2,3,4,5}, 2) in US format
        // =PERCENTRANK.INC({1;2;2;3;4;5}; 2) in German format
        // First occurrence of 2 at position 1 (0-based): 1/(6-1) = 0.2
        let array = vec![1.0, 2.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_inc(array, 2.0, None).unwrap();
        assert!((result - 0.2).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_inc_microsoft_example() {
        // Microsoft documentation example: =PERCENTRANK.INC({13,12,11,8,4,3,2,1,1,1}, 2)
        // Expected result: 0.333
        let array = vec![13.0, 12.0, 11.0, 8.0, 4.0, 3.0, 2.0, 1.0, 1.0, 1.0];
        let result = codcel_percent_rank_inc(array.clone(), 2.0, None).unwrap();
        assert!((result - 0.333).abs() < 0.0001);

        // =PERCENTRANK.INC({13,12,11,8,4,3,2,1,1,1}, 4)
        // Position of 4 is index 4 in sorted [1,1,1,2,3,4,8,11,12,13] -> index 5 (0-based)
        // 5/(10-1) = 0.555..., truncated to 3 = 0.555
        let result = codcel_percent_rank_inc(array.clone(), 4.0, None).unwrap();
        assert!((result - 0.555).abs() < 0.0001);

        // =PERCENTRANK.INC({13,12,11,8,4,3,2,1,1,1}, 8, 2)
        // Position of 8 is index 7 in sorted [1,1,1,2,3,4,8,11,12,13] -> index 6 (0-based)
        // 6/(10-1) = 0.666..., truncated to 2 = 0.66
        let result = codcel_percent_rank_inc(array, 8.0, Some(2)).unwrap();
        assert!((result - 0.66).abs() < 0.0001);
    }
}
