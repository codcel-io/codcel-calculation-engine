// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::collections::HashMap;
use std::error::Error;

/// Excel-compatible `MODE.MULT` that returns a vertical array of the most frequently occurring values.
/// - `numbers`: an array of numeric values.
///
/// Returns all modes (most frequently occurring values) if there are ties,
/// or an error when the input is empty or no value occurs more than once.
pub fn codcel_mode_mult(numbers: Vec<f64>) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
    if numbers.is_empty() {
        return Err("MODE.MULT: Cannot calculate mode of empty dataset".into());
    }

    // Track frequency and first-appearance position for each unique value
    // Using string key for f64 to handle NaN/precision issues
    let mut frequency_map: HashMap<String, (f64, usize, usize)> = HashMap::new(); // (value, count, first_index)

    for (i, value) in numbers.iter().enumerate() {
        let key = format!("{value:?}");
        let entry = frequency_map.entry(key).or_insert((*value, 0, i));
        entry.1 += 1;
    }

    // Find the maximum frequency
    let max_frequency = frequency_map
        .values()
        .map(|(_, freq, _)| *freq)
        .max()
        .ok_or("MODE.MULT: Failed to calculate mode")?;

    // Collect all values with the maximum frequency, preserving first-appearance order
    let mut modes: Vec<(f64, usize)> = frequency_map
        .into_iter()
        .filter(|(_, (_, freq, _))| *freq == max_frequency)
        .map(|(_, (value, _, first_idx))| (value, first_idx))
        .collect();

    // Sort by first-appearance index (Excel returns modes in order of first occurrence)
    modes.sort_by_key(|(_, idx)| *idx);

    Ok(modes.into_iter().map(|(value, _)| value).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_mult_basic() {
        // =MODE.MULT({1,2,3,3,4,5}) in US format
        // =MODE.MULT({1;2;3;3;4;5}) in German format
        let values = vec![1.0, 2.0, 3.0, 3.0, 4.0, 5.0];
        let result = codcel_mode_mult(values).unwrap();
        assert_eq!(result, vec![3.0]);
    }

    #[test]
    fn test_mode_mult_multiple_modes() {
        // =MODE.MULT({1,2,2,3,3,4}) in US format
        // When there are multiple modes, MODE.MULT returns them in first-appearance order
        let values = vec![1.0, 2.0, 2.0, 3.0, 3.0, 4.0];
        let result = codcel_mode_mult(values).unwrap();
        assert_eq!(result, vec![2.0, 3.0]); // 2 appears first at index 1, 3 at index 3
    }

    #[test]
    fn test_mode_mult_all_same() {
        // =MODE.MULT({5,5,5,5,5}) in US format
        // =MODE.MULT({5;5;5;5;5}) in German format
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_mode_mult(values).unwrap();
        assert_eq!(result, vec![5.0]);
    }

    #[test]
    fn test_mode_mult_all_different() {
        // =MODE.MULT({1,2,3,4,5}) in US format
        // =MODE.MULT({1;2;3;4;5}) in German format
        // When all values occur once, all values are modes
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_mode_mult(values).unwrap();
        assert_eq!(result, vec![1.0, 2.0, 3.0, 4.0, 5.0]); // first-appearance order
    }

    #[test]
    fn test_mode_mult_negative_values() {
        // =MODE.MULT({-5,-4,-3,-3,-2,-1}) in US format
        // =MODE.MULT({-5;-4;-3;-3;-2;-1}) in German format
        let values = vec![-5.0, -4.0, -3.0, -3.0, -2.0, -1.0];
        let result = codcel_mode_mult(values).unwrap();
        assert_eq!(result, vec![-3.0]);
    }

    #[test]
    fn test_mode_mult_decimal_values() {
        // =MODE.MULT({1.1,2.2,2.2,3.3,4.4}) in US format
        // =MODE.MULT({1,1;2,2;2,2;3,3;4,4}) in German format
        let values = vec![1.1, 2.2, 2.2, 3.3, 4.4];
        let result = codcel_mode_mult(values).unwrap();
        assert_eq!(result, vec![2.2]);
    }

    #[test]
    fn test_mode_mult_single_value() {
        // =MODE.MULT({42}) in US format
        // =MODE.MULT({42}) in German format
        let values = vec![42.0];
        let result = codcel_mode_mult(values).unwrap();
        assert_eq!(result, vec![42.0]);
    }

    #[test]
    fn test_mode_mult_empty_dataset() {
        // Empty dataset should return an error
        let values: Vec<f64> = vec![];
        let result = codcel_mode_mult(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_mode_mult_unsorted_data() {
        // =MODE.MULT({5,3,3,1,4,2,5}) in US format
        // =MODE.MULT({5;3;3;1;4;2;5}) in German format
        // Multiple modes (5 and 3 both appear twice); 5 first at idx 0, 3 first at idx 1
        let values = vec![5.0, 3.0, 3.0, 1.0, 4.0, 2.0, 5.0];
        let result = codcel_mode_mult(values).unwrap();
        assert_eq!(result, vec![5.0, 3.0]); // first-appearance order
    }

    #[test]
    fn test_mode_mult_three_modes() {
        // Test with three different values having the same frequency
        let values = vec![1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_mode_mult(values).unwrap();
        assert_eq!(result, vec![1.0, 2.0, 3.0]); // first-appearance order
    }
}
