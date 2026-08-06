// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `PERCENTILE.EXC` that returns the k-th percentile (exclusive method).
/// - `array`: an array of numeric values.
/// - `k`: the percentile value (0 to 1, exclusive).
///
/// Returns the value at the k-th percentile using interpolation (exclusive method),
/// or an error when the array is empty or k is out of range.
pub fn codcel_percentile_exc(array: Vec<f64>, k: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("PERCENTILE.EXC: Input array must not be empty.".into());
    }

    if k <= 0.0 || k >= 1.0 {
        return Err("PERCENTILE.EXC: k must be between 0 and 1 (exclusive).".into());
    }

    let mut sorted_array = array.clone();
    sorted_array.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted_array.len() as f64;
    let pos = k * (n + 1.0);

    if pos <= 1.0 {
        return Err("PERCENTILE.EXC: k is too small for the given array.".into());
    }

    if pos >= n {
        return Err("PERCENTILE.EXC: k is too large for the given array.".into());
    }

    let lower_index = pos.floor() as usize - 1;
    let upper_index = pos.ceil() as usize - 1;

    let lower_value = sorted_array[lower_index];
    let upper_value = sorted_array[upper_index];

    let weight = pos - lower_index as f64 - 1.0;
    let result = lower_value + weight * (upper_value - lower_value);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentile_exc_basic() {
        // =PERCENTILE.EXC({1,2,3,4,5,6,7,8,9}, 0.5) in US format
        // =PERCENTILE.EXC({1;2;3;4;5;6;7;8;9}; 0,5) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_percentile_exc(array, 0.5).unwrap();
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_exc_unsorted() {
        // =PERCENTILE.EXC({9,7,5,3,1,8,6,4,2}, 0.5) in US format
        // =PERCENTILE.EXC({9;7;5;3;1;8;6;4;2}; 0,5) in German format
        let array = vec![9.0, 7.0, 5.0, 3.0, 1.0, 8.0, 6.0, 4.0, 2.0];
        let result = codcel_percentile_exc(array, 0.5).unwrap();
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_exc_interpolation() {
        // =PERCENTILE.EXC({1,2,3,4,5,6,7,8,9}, 0.25) in US format
        // =PERCENTILE.EXC({1;2;3;4;5;6;7;8;9}; 0,25) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_percentile_exc(array, 0.25).unwrap();
        println!("{result}");
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_exc_interpolation_decimal() {
        // =PERCENTILE.EXC({1,2,3,4,5,6,7,8,9}, 0.3) in US format
        // =PERCENTILE.EXC({1;2;3;4;5;6;7;8;9}; 0,3) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_percentile_exc(array, 0.3).unwrap();
        println!("{result}");
        assert!((result - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_exc_k_too_small() {
        // k too small should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percentile_exc(array, 0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_percentile_exc_k_too_large() {
        // k too large should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percentile_exc(array, 0.9);
        assert!(result.is_err());
    }

    #[test]
    fn test_percentile_exc_k_zero() {
        // k = 0 should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percentile_exc(array, 0.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_percentile_exc_k_one() {
        // k = 1 should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percentile_exc(array, 1.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_percentile_exc_empty_array() {
        // Empty array should return an error
        let array: Vec<f64> = vec![];
        let result = codcel_percentile_exc(array, 0.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_percentile_exc_larger_array() {
        // =PERCENTILE.EXC({1,2,3,4,5,6,7,8,9,10,11,12,13,14,15}, 0.4) in US format
        // =PERCENTILE.EXC({1;2;3;4;5;6;7;8;9;10;11;12;13;14;15}; 0,4) in German format
        let array = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
        ];
        let result = codcel_percentile_exc(array, 0.4).unwrap();
        assert!((result - 6.4).abs() < 0.0001);
    }
}
