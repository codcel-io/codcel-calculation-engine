// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::cmp::Ordering;

/// Excel-compatible `RANK`/`RANK.EQ` function.
/// Returns the rank of a value within an array.
/// - `value`: value for which to find the rank.
/// - `array`: array of numeric values.
/// - `order`: `Some(true)` for ascending rank, `None` or `Some(false)` for descending.
///
/// Returns an error when the array is empty or the value is not present.
pub fn codcel_rank(
    value: f64,
    array: Vec<f64>,
    order: Option<bool>,
) -> Result<i32, Box<dyn std::error::Error + Send + Sync>> {
    if array.is_empty() {
        return Err("RANK: Array cannot be empty".into());
    }

    let mut sorted_array = array.clone();

    // Sort ascending if `order` is true, otherwise sort descending
    if order.unwrap_or(false) {
        sorted_array.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    } else {
        sorted_array.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
    }

    // Find the rank based on the sorted array
    let rank = sorted_array
        .iter()
        .position(|&x| x == value)
        .map(|index| index as i32 + 1)
        .ok_or("RANK: Value not found in the array")?;

    Ok(rank)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_descending() {
        // =RANK(3, {1,2,3,4,5}) in US format
        // =RANK(3; {1;2;3;4;5}) in German format
        let result = codcel_rank(3.0, vec![1.0, 2.0, 3.0, 4.0, 5.0], None).unwrap();
        println!("{result}");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_rank_ascending() {
        // =RANK(3, {1,2,3,4,5}, 1) in US format
        // =RANK(3; {1;2;3;4;5}; 1) in German format
        let result = codcel_rank(3.0, vec![1.0, 2.0, 3.0, 4.0, 5.0], Some(true)).unwrap();
        println!("{result}");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_rank_duplicates() {
        // =RANK(3, {1,2,3,3,4,5}) in US format
        // =RANK(3; {1;2;3;3;4;5}) in German format
        let result = codcel_rank(3.0, vec![1.0, 2.0, 3.0, 3.0, 4.0, 5.0], None).unwrap();
        println!("{result}");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_rank_beginning() {
        // =RANK(5, {1,2,3,4,5}) in US format
        // =RANK(5; {1;2;3;4;5}) in German format
        let result = codcel_rank(5.0, vec![1.0, 2.0, 3.0, 4.0, 5.0], None).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_rank_end() {
        // =RANK(1, {1,2,3,4,5}) in US format
        // =RANK(1; {1;2;3;4;5}) in German format
        let result = codcel_rank(1.0, vec![1.0, 2.0, 3.0, 4.0, 5.0], None).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_rank_ascending_beginning() {
        // =RANK(1, {1,2,3,4,5}, 1) in US format
        // =RANK(1; {1;2;3;4;5}; 1) in German format
        let result = codcel_rank(1.0, vec![1.0, 2.0, 3.0, 4.0, 5.0], Some(true)).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_rank_ascending_end() {
        // =RANK(5, {1,2,3,4,5}, 1) in US format
        // =RANK(5; {1;2;3;4;5}; 1) in German format
        let result = codcel_rank(5.0, vec![1.0, 2.0, 3.0, 4.0, 5.0], Some(true)).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_rank_empty_array() {
        // =RANK(3, {}) in US format
        // =RANK(3; {}) in German format
        let result = codcel_rank(3.0, vec![], None);
        assert!(result.is_err());
    }

    #[test]
    fn test_rank_value_not_found() {
        // =RANK(6, {1,2,3,4,5}) in US format
        // =RANK(6; {1;2;3;4;5}) in German format
        let result = codcel_rank(6.0, vec![1.0, 2.0, 3.0, 4.0, 5.0], None);
        assert!(result.is_err());
    }
}
