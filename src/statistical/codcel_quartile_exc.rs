// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `QUARTILE.EXC` that returns the quartile of a data set (exclusive method).
/// - `values`: an array of numeric values.
/// - `quart`: the quartile to return (1 = 25%, 2 = 50%, 3 = 75%).
///
/// Returns the specified quartile value using the exclusive interpolation method,
/// or an error when the array is empty or quart is invalid.
pub fn codcel_quartile_exc(
    values: Vec<f64>,
    quart: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let mut values = values;

    if values.is_empty() {
        return Err("QUARTILE.EXC: Values array must not be empty.".into());
    }

    if !(1..=3).contains(&quart) {
        return Err("QUARTILE.EXC: Quart must be between 1 and 3.".into());
    }

    // Sort the values in ascending order
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = values.len() as f64;

    // For QUARTILE.EXC, the positions are calculated differently; exclude 0% and 100%
    let pos = quart as f64 * (n + 1.0) / 4.0;

    if pos < 1.0 || pos > n {
        return Err("QUARTILE.EXC: Quartile position is out of range.".into());
    }

    let lower_index = pos.floor() as usize - 1;
    let upper_index = pos.ceil() as usize - 1;

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
    fn test_quartile_exc_first() {
        // =QUARTILE.EXC({1,2,3,4,5,6,7,8,9,10}, 1) in US format
        // =QUARTILE.EXC({1;2;3;4;5;6;7;8;9;10}; 1) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_quartile_exc(values, 1).unwrap();
        assert!((result - 2.75).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_exc_second() {
        // =QUARTILE.EXC({1,2,3,4,5,6,7,8,9,10}, 2) in US format
        // =QUARTILE.EXC({1;2;3;4;5;6;7;8;9;10}; 2) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_quartile_exc(values, 2).unwrap();
        assert!((result - 5.5).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_exc_third() {
        // =QUARTILE.EXC({1,2,3,4,5,6,7,8,9,10}, 3) in US format
        // =QUARTILE.EXC({1;2;3;4;5;6;7;8;9;10}; 3) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_quartile_exc(values, 3).unwrap();
        assert!((result - 8.25).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_exc_unsorted() {
        // =QUARTILE.EXC({10,9,8,7,6,5,4,3,2,1}, 2) in US format
        // =QUARTILE.EXC({10;9;8;7;6;5;4;3;2;1}; 2) in German format
        let values = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
        let result = codcel_quartile_exc(values, 2).unwrap();
        assert!((result - 5.5).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_exc_odd_count() {
        // =QUARTILE.EXC({1,2,3,4,5,6,7,8,9}, 2) in US format
        // =QUARTILE.EXC({1;2;3;4;5;6;7;8;9}; 2) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_quartile_exc(values, 2).unwrap();
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_exc_empty_array() {
        // Empty array should return an error
        let values: Vec<f64> = vec![];
        let result = codcel_quartile_exc(values, 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_quartile_exc_invalid_quart() {
        // Quart outside 1-3 should return an error
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_quartile_exc(values.clone(), 0);
        assert!(result.is_err());

        let result = codcel_quartile_exc(values, 4);
        assert!(result.is_err());
    }

    #[test]
    fn test_quartile_exc_position_out_of_range() {
        // Small dataset where quartile position is out of range
        let values = vec![1.0, 2.0];
        let result = codcel_quartile_exc(values, 3);
        assert!(result.is_err());
    }
}
