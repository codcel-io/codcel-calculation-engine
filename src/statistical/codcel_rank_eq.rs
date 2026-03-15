// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `RANK.EQ` that returns the rank of a value, with ties receiving the same rank.
/// - `value`: the value for which to find the rank.
/// - `values`: an array of numeric values to rank against.
/// - `order`: if `false` or omitted, ranks in descending order; if `true`, ranks in ascending order.
///
/// Returns the rank (ties share the top rank),
/// or an error when the array is empty.
pub fn codcel_rank_eq(
    value: f64,
    values: Vec<f64>,
    order: Option<bool>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("RANK.EQ: The values list cannot be empty.".into());
    }

    // Default to descending order if `order` is None
    let sort_ascending = order.unwrap_or(false);

    // Sort the values in ascending or descending order
    let mut sorted_values = values.clone();
    if sort_ascending {
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        sorted_values.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    }

    // Find the rank of the value
    for (i, &v) in sorted_values.iter().enumerate() {
        if v == value {
            return Ok((i + 1) as i32); // 1-based rank
        }
    }

    Err("RANK.EQ: The value is not found in the list.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_eq_descending() {
        // =RANK.EQ(3.5, {7,3.5,3.5,1,2}, FALSE) in US format
        // =RANK.EQ(3,5; {7;3,5;3,5;1;2}; FALSE) in German format
        let value = 3.5;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_eq(value, values, Some(false)).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_rank_eq_ascending() {
        // =RANK.EQ(3.5, {7,3.5,3.5,1,2}, TRUE) in US format
        // =RANK.EQ(3,5; {7;3,5;3,5;1;2}; TRUE) in German format
        let value = 3.5;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_eq(value, values, Some(true)).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_rank_eq_default_order() {
        // =RANK.EQ(3.5, {7,3.5,3.5,1,2}) in US format
        // =RANK.EQ(3,5; {7;3,5;3,5;1;2}) in German format
        let value = 3.5;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_eq(value, values, None).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_rank_eq_highest_value() {
        // =RANK.EQ(7, {7,3.5,3.5,1,2}, FALSE) in US format
        // =RANK.EQ(7; {7;3,5;3,5;1;2}; FALSE) in German format
        let value = 7.0;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_eq(value, values, Some(false)).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_rank_eq_lowest_value() {
        // =RANK.EQ(1, {7,3.5,3.5,1,2}, FALSE) in US format
        // =RANK.EQ(1; {7;3,5;3,5;1;2}; FALSE) in German format
        let value = 1.0;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_eq(value, values, Some(false)).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_rank_eq_duplicate_values() {
        // =RANK.EQ(3, {7,3,3,3,1,2}, FALSE) in US format
        // =RANK.EQ(3; {7;3;3;3;1;2}; FALSE) in German format
        let value = 3.0;
        let values = vec![7.0, 3.0, 3.0, 3.0, 1.0, 2.0];
        let result = codcel_rank_eq(value, values, Some(false)).unwrap();
        println!("{result}");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_rank_eq_empty_array() {
        // Empty array should return an error
        let value = 3.5;
        let values: Vec<f64> = vec![];
        let result = codcel_rank_eq(value, values, Some(false));
        assert!(result.is_err());
    }

    #[test]
    fn test_rank_eq_value_not_found() {
        // Value not in the list should return an error
        let value = 10.0;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_eq(value, values, Some(false));
        assert!(result.is_err());
    }
}
