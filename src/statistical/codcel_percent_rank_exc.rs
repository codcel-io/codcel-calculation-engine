// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `PERCENTRANK.EXC` that returns the rank of a value as a percentage (exclusive).
/// - `array`: an array of numeric values.
/// - `value`: the value for which to determine the rank.
/// - `significance`: optional number of significant digits for the returned percentage (default 3).
///
/// Returns the rank of the value as a percentage (0 to 1, exclusive),
/// or an error when the array is empty or value is outside the range.
pub fn codcel_percent_rank_exc(
    array: Vec<f64>,
    value: f64,
    significance: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("PERCENTRANK.EXC: The array cannot be empty.".into());
    }

    let significance = significance.unwrap_or(3);

    if significance < 1 {
        return Err("PERCENTRANK.EXC: Significance must be >= 1.".into());
    }

    let mut sorted_array = array.clone();
    sorted_array.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if value < sorted_array[0] || value > sorted_array[sorted_array.len() - 1] {
        return Err("PERCENTRANK.EXC: Value is out of the range of the array.".into());
    }

    let n = sorted_array.len() as f64;

    // Find position and calculate exclusive rank using rank / (n + 1)
    let mut percent_rank = 0.0;

    for (i, &x) in sorted_array.iter().enumerate() {
        if x == value {
            // Exact match: exclusive rank = (1-based position) / (n + 1)
            percent_rank = (i as f64 + 1.0) / (n + 1.0);
            break;
        } else if x > value {
            // Interpolation between adjacent positions
            let lower_index = i.saturating_sub(1);
            let lower_value = sorted_array[lower_index];
            let upper_value = x;
            let lower_rank = (lower_index as f64 + 1.0) / (n + 1.0);
            let upper_rank = (i as f64 + 1.0) / (n + 1.0);
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
    fn test_percent_rank_exc_basic() {
        // =PERCENTRANK.EXC({1,2,3,4,5}, 3) in US format
        // =PERCENTRANK.EXC({1;2;3;4;5}; 3) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_exc(array, 3.0, None).unwrap();
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_exc_unsorted() {
        // =PERCENTRANK.EXC({5,3,1,4,2}, 3) in US format
        // =PERCENTRANK.EXC({5;3;1;4;2}; 3) in German format
        let array = vec![5.0, 3.0, 1.0, 4.0, 2.0];
        let result = codcel_percent_rank_exc(array, 3.0, None).unwrap();
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_exc_min_value() {
        // =PERCENTRANK.EXC({1,2,3,4,5}, 1) in US format
        // =PERCENTRANK.EXC({1;2;3;4;5}; 1) in German format
        // Exclusive: 1/(5+1) = 0.166..., truncated to 3 digits = 0.166
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_exc(array, 1.0, None).unwrap();
        assert!((result - 0.166).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_exc_max_value() {
        // =PERCENTRANK.EXC({1,2,3,4,5}, 5) in US format
        // =PERCENTRANK.EXC({1;2;3;4;5}; 5) in German format
        // Exclusive: 5/(5+1) = 0.833..., truncated to 3 digits = 0.833
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_exc(array, 5.0, None).unwrap();
        assert!((result - 0.833).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_exc_interpolation() {
        // =PERCENTRANK.EXC({1,2,3,4,5}, 2.5) in US format
        // =PERCENTRANK.EXC({1;2;3;4;5}; 2,5) in German format
        // Ranks: 2/6=0.333, 3/6=0.5, interpolated: 0.333 + 0.5*(0.5-0.333) = 0.416
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_exc(array, 2.5, None).unwrap();
        assert!((result - 0.416).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_exc_custom_significance() {
        // =PERCENTRANK.EXC({1,2,3,4,5}, 2.5, 2) in US format
        // =PERCENTRANK.EXC({1;2;3;4;5}; 2,5; 2) in German format
        // 0.416... truncated to 2 digits = 0.41
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_exc(array, 2.5, Some(2)).unwrap();
        assert!((result - 0.41).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_exc_empty_array() {
        // Empty array should return an error
        let array: Vec<f64> = vec![];
        let result = codcel_percent_rank_exc(array, 3.0, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_percent_rank_exc_value_out_of_range_low() {
        // Value below the minimum should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_exc(array, 0.5, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_percent_rank_exc_value_out_of_range_high() {
        // Value above the maximum should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_exc(array, 5.5, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_percent_rank_exc_zero_significance() {
        // Significance < 1 should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_exc(array, 3.0, Some(0));
        assert!(result.is_err());
    }

    #[test]
    fn test_percent_rank_exc_duplicate_values() {
        // =PERCENTRANK.EXC({1,2,2,3,4,5}, 2) in US format
        // =PERCENTRANK.EXC({1;2;2;3;4;5}; 2) in German format
        // First occurrence of 2 at position 2 (1-based): 2/(6+1) = 0.285..., truncated = 0.285
        let array = vec![1.0, 2.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percent_rank_exc(array, 2.0, None).unwrap();
        assert!((result - 0.285).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_exc_microsoft_example() {
        // Microsoft documentation example: =PERCENTRANK.EXC({1,2,3,6,6,6,7,8,9}, 7) = 0.7
        let array = vec![1.0, 2.0, 3.0, 6.0, 6.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_percent_rank_exc(array.clone(), 7.0, None).unwrap();
        assert!((result - 0.7).abs() < 0.0001);

        // =PERCENTRANK.EXC({1,2,3,6,6,6,7,8,9}, 5.43) = 0.381
        let result = codcel_percent_rank_exc(array.clone(), 5.43, None).unwrap();
        assert!((result - 0.381).abs() < 0.0001);

        // =PERCENTRANK.EXC({1,2,3,6,6,6,7,8,9}, 5.43, 1) = 0.3 (truncated, not rounded)
        let result = codcel_percent_rank_exc(array, 5.43, Some(1)).unwrap();
        assert!((result - 0.3).abs() < 0.0001);
    }
}
