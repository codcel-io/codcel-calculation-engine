// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `QUARTILE.INC` that returns the quartile of a data set (inclusive method).
/// - `values`: an array of numeric values.
/// - `quart`: the quartile to return (0 = min, 1 = 25%, 2 = 50%, 3 = 75%, 4 = max).
///
/// Returns the specified quartile value using the inclusive interpolation method,
/// or an error when the array is empty or quart is invalid.
pub fn codcel_quartile_inc(
    values: Vec<f64>,
    quart: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let mut values = values;

    if values.is_empty() {
        return Err("QUARTILE.INC: Values array must not be empty.".into());
    }

    if !(0..=4).contains(&quart) {
        return Err("QUARTILE.INC: Quart must be between 0 and 4.".into());
    }

    // Sort the values in ascending order
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = values.len() as f64;

    // For QUARTILE.INC, positions include 0% and 100%
    let pos = quart as f64 * (n - 1.0) / 4.0;

    let lower_index = pos.floor() as usize;
    let upper_index = pos.ceil() as usize;

    if lower_index == upper_index {
        Ok(values[lower_index])
    } else {
        // Interpolate between indices
        let lower_value = values[lower_index];
        let upper_value = values[upper_index];
        let weight = pos - pos.floor();

        Ok(lower_value + weight * (upper_value - lower_value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quartile_inc_min() {
        // =QUARTILE.INC({1,2,3,4,5,6,7,8,9,10}, 0) in US format
        // =QUARTILE.INC({1;2;3;4;5;6;7;8;9;10}; 0) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_quartile_inc(values, 0).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_inc_first() {
        // =QUARTILE.INC({1,2,3,4,5,6,7,8,9,10}, 1) in US format
        // =QUARTILE.INC({1;2;3;4;5;6;7;8;9;10}; 1) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_quartile_inc(values, 1).unwrap();
        assert!((result - 3.25).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_inc_second() {
        // =QUARTILE.INC({1,2,3,4,5,6,7,8,9,10}, 2) in US format
        // =QUARTILE.INC({1;2;3;4;5;6;7;8;9;10}; 2) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_quartile_inc(values, 2).unwrap();
        assert!((result - 5.5).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_inc_third() {
        // =QUARTILE.INC({1,2,3,4,5,6,7,8,9,10}, 3) in US format
        // =QUARTILE.INC({1;2;3;4;5;6;7;8;9;10}; 3) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_quartile_inc(values, 3).unwrap();
        assert!((result - 7.75).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_inc_max() {
        // =QUARTILE.INC({1,2,3,4,5,6,7,8,9,10}, 4) in US format
        // =QUARTILE.INC({1;2;3;4;5;6;7;8;9;10}; 4) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_quartile_inc(values, 4).unwrap();
        assert!((result - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_inc_unsorted() {
        // =QUARTILE.INC({10,9,8,7,6,5,4,3,2,1}, 2) in US format
        // =QUARTILE.INC({10;9;8;7;6;5;4;3;2;1}; 2) in German format
        let values = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let result = codcel_quartile_inc(values, 2).unwrap();
        assert!((result - 5.5).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_inc_odd_count() {
        // =QUARTILE.INC({1,2,3,4,5,6,7,8,9}, 2) in US format
        // =QUARTILE.INC({1;2;3;4;5;6;7;8;9}; 2) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_quartile_inc(values, 2).unwrap();
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_inc_empty_array() {
        // Empty array should return an error
        let values: Vec<f64> = vec![];
        let result = codcel_quartile_inc(values, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_quartile_inc_invalid_quart() {
        // Quart outside 0-4 should return an error
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_quartile_inc(values.clone(), -1);
        assert!(result.is_err());

        let result = codcel_quartile_inc(values, 5);
        assert!(result.is_err());
    }
}
