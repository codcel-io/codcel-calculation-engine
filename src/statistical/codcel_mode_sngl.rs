// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::collections::HashMap;
use std::error::Error;

/// Excel-compatible `MODE.SNGL` that returns the most frequently occurring value in a data set.
/// - `values`: an array of numeric values.
///
/// Returns the mode (most frequently occurring value),
/// or an error when the input is empty or no value occurs more than once.
pub fn codcel_mode_sngl(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("MODE.SNGL: Cannot calculate mode of empty dataset".into());
    }

    // Create a HashMap using string representation of f64 as key
    let mut frequency_map: HashMap<String, (f64, usize)> = HashMap::new();

    // Count frequencies of each value, storing both the original f64 and its count
    for value in values {
        let key = format!("{value:?}");
        let entry = frequency_map.entry(key).or_insert((value, 0));
        entry.1 += 1;
    }

    // Find the value with highest frequency and lowest value in case of ties
    let mode = frequency_map
        .into_iter()
        .max_by(|(_, (value_a, freq_a)), (_, (value_b, freq_b))| {
            match freq_a.cmp(freq_b) {
                std::cmp::Ordering::Equal => {
                    // If frequencies are equal, prefer the smaller value
                    value_b
                        .partial_cmp(value_a)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }
                other => other,
            }
        })
        .ok_or("MODE.SNGL: Failed to calculate mode")?;

    Ok(mode.1 .0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_sngl_basic() {
        // =MODE.SNGL({1,2,3,3,4,5}) in US format
        // =MODE.SNGL({1;2;3;3;4;5}) in German format
        let values = vec![1.0, 2.0, 3.0, 3.0, 4.0, 5.0];
        let result = codcel_mode_sngl(values).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_mode_sngl_multiple_modes() {
        // =MODE.SNGL({1,2,2,3,3,4}) in US format
        // =MODE.SNGL({1;2;2;3;3;4}) in German format
        // When there are multiple modes, Excel returns the smallest value
        let values = vec![1.0, 2.0, 2.0, 3.0, 3.0, 4.0];
        let result = codcel_mode_sngl(values).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_mode_sngl_all_same() {
        // =MODE.SNGL({5,5,5,5,5}) in US format
        // =MODE.SNGL({5;5;5;5;5}) in German format
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_mode_sngl(values).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_mode_sngl_all_different() {
        // =MODE.SNGL({1,2,3,4,5}) in US format
        // =MODE.SNGL({1;2;3;4;5}) in German format
        // When all values occur once, Excel returns the smallest value
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_mode_sngl(values).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_mode_sngl_negative_values() {
        // =MODE.SNGL({-5,-4,-3,-3,-2,-1}) in US format
        // =MODE.SNGL({-5;-4;-3;-3;-2;-1}) in German format
        let values = vec![-5.0, -4.0, -3.0, -3.0, -2.0, -1.0];
        let result = codcel_mode_sngl(values).unwrap();
        assert_eq!(result, -3.0);
    }

    #[test]
    fn test_mode_sngl_decimal_values() {
        // =MODE.SNGL({1.1,2.2,2.2,3.3,4.4}) in US format
        // =MODE.SNGL({1,1;2,2;2,2;3,3;4,4}) in German format
        let values = vec![1.1, 2.2, 2.2, 3.3, 4.4];
        let result = codcel_mode_sngl(values).unwrap();
        assert_eq!(result, 2.2);
    }

    #[test]
    fn test_mode_sngl_single_value() {
        // =MODE.SNGL({42}) in US format
        // =MODE.SNGL({42}) in German format
        let values = vec![42.0];
        let result = codcel_mode_sngl(values).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_mode_sngl_empty_dataset() {
        // Empty dataset should return an error
        let values: Vec<f64> = vec![];
        let result = codcel_mode_sngl(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_mode_sngl_unsorted_data() {
        // =MODE.SNGL({5,3,3,1,4,2}) in US format
        // =MODE.SNGL({5;3;3;1;4;2}) in German format
        let values = vec![5.0, 3.0, 3.0, 1.0, 4.0, 2.0];
        let result = codcel_mode_sngl(values).unwrap();
        assert_eq!(result, 3.0);
    }
}
