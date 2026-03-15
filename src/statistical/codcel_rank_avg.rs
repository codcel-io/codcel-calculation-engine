// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `RANK.AVG` that returns the rank of a value, averaging ties.
/// - `value`: the value for which to find the rank.
/// - `values`: an array of numeric values to rank against.
/// - `order`: if `false` or omitted, ranks in descending order; if `true`, ranks in ascending order.
///
/// Returns the average rank when there are ties,
/// or an error when the array is empty.
pub fn codcel_rank_avg(
    value: f64,
    values: Vec<f64>,
    order: Option<bool>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("RANK.AVG: The values list cannot be empty.".into());
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

    // Calculate the rank(s) of the value
    let mut rank_sum = 0.0;
    let mut count = 0;

    for (i, &v) in sorted_values.iter().enumerate() {
        if v == value {
            rank_sum += (i + 1) as f64; // 1-based rank
            count += 1;
        }
    }

    if count == 0 {
        return Err("RANK.AVG: The value is not found in the list.".into());
    }

    // Average the ranks for tied values
    let avg_rank = rank_sum / count as f64;
    Ok(avg_rank)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_avg_descending() {
        // =RANK.AVG(3.5, {7,3.5,3.5,1,2}, FALSE) in US format
        // =RANK.AVG(3,5; {7;3,5;3,5;1;2}; FALSE) in German format
        let value = 3.5;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, Some(false)).unwrap();
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_rank_avg_ascending() {
        // =RANK.AVG(3.5, {7,3.5,3.5,1,2}, TRUE) in US format
        // =RANK.AVG(3,5; {7;3,5;3,5;1;2}; TRUE) in German format
        let value = 3.5;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, Some(true)).unwrap();
        assert!((result - 3.5).abs() < 0.0001);
    }

    #[test]
    fn test_rank_avg_default_order() {
        // =RANK.AVG(3.5, {7,3.5,3.5,1,2}) in US format
        // =RANK.AVG(3,5; {7;3,5;3,5;1;2}) in German format
        let value = 3.5;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, None).unwrap();
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_rank_avg_single_occurrence() {
        // =RANK.AVG(7, {7,3.5,3.5,1,2}, FALSE) in US format
        // =RANK.AVG(7; {7;3,5;3,5;1;2}; FALSE) in German format
        let value = 7.0;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, Some(false)).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_rank_avg_multiple_ties() {
        // =RANK.AVG(3, {7,3,3,3,1,2}, FALSE) in US format
        // =RANK.AVG(3; {7;3;3;3;1;2}; FALSE) in German format
        let value = 3.0;
        let values = vec![7.0, 3.0, 3.0, 3.0, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, Some(false)).unwrap();
        assert!((result - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_rank_avg_empty_array() {
        // Empty array should return an error
        let value = 3.5;
        let values: Vec<f64> = vec![];
        let result = codcel_rank_avg(value, values, Some(false));
        assert!(result.is_err());
    }

    #[test]
    fn test_rank_avg_value_not_found() {
        // Value not in the list should return an error
        let value = 10.0;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, Some(false));
        assert!(result.is_err());
    }
}
