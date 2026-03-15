// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

/// Excel-compatible `PERCENTRANK`/`PERCENTRANK.INC` function.
/// Returns the rank of a value within an array as a percentage of the array size.
/// - `array`: array of numeric values.
/// - `value`: value for which to find the percentile rank.
/// - `significance`: optional number of significant digits for rounding.
///
/// Returns an error when the array is empty or the value lies outside the array bounds.
pub fn codcel_percent_rank(
    array: Vec<f64>,
    value: f64,
    significance: Option<i32>,
) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    if array.is_empty() {
        return Err("PERCENTRANK: Array cannot be empty".into());
    }

    let mut sorted_array = array.clone();
    sorted_array.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    if value < sorted_array[0] || value > sorted_array[sorted_array.len() - 1] {
        return Err("PERCENTRANK: Value is outside the bounds of the array".into());
    }

    let mut rank = None;

    for (i, v) in sorted_array.iter().enumerate() {
        if *v == value {
            rank = Some(i as f64);
            break;
        } else if value < *v {
            let prev_value = if i > 0 { sorted_array[i - 1] } else { *v };
            rank = Some((i as f64) - 1.0 + (value - prev_value) / (*v - prev_value));
            break;
        }
    }

    let percentage = rank
        .map(|r| r / (sorted_array.len() as f64 - 1.0))
        .ok_or("PERCENTRANK: Unable to calculate rank")?;

    let sig = significance.unwrap_or(3);
    let factor = 10f64.powi(sig);
    Ok((percentage * factor).floor() / factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_rank_exact_match() {
        // =PERCENTRANK({1,2,3,4,5}, 3) in US format - default sig=3
        let result = codcel_percent_rank(vec![1.0, 2.0, 3.0, 4.0, 5.0], 3.0, None).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_interpolation() {
        // =PERCENTRANK({1,2,3,4,5}, 2.5) in US format - default sig=3, truncated
        let result = codcel_percent_rank(vec![1.0, 2.0, 3.0, 4.0, 5.0], 2.5, None).unwrap();
        println!("{result}");
        assert!((result - 0.375).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_beginning() {
        // =PERCENTRANK({1,2,3,4,5}, 1) in US format
        let result = codcel_percent_rank(vec![1.0, 2.0, 3.0, 4.0, 5.0], 1.0, None).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_end() {
        // =PERCENTRANK({1,2,3,4,5}, 5) in US format
        let result = codcel_percent_rank(vec![1.0, 2.0, 3.0, 4.0, 5.0], 5.0, None).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_with_significance() {
        // =PERCENTRANK({1,2,3,4,5}, 2.5, 2) in US format - sig=2, truncated
        let result = codcel_percent_rank(vec![1.0, 2.0, 3.0, 4.0, 5.0], 2.5, Some(2)).unwrap();
        println!("{result}");
        assert!((result - 0.37).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_unsorted_array() {
        // =PERCENTRANK({5,3,1,4,2}, 3) in US format
        // =PERCENTRANK({5;3;1;4;2}; 3) in German format
        let result = codcel_percent_rank(vec![5.0, 3.0, 1.0, 4.0, 2.0], 3.0, None).unwrap();
        println!("{result}");
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_percent_rank_empty_array() {
        // =PERCENTRANK({}, 3) in US format
        // =PERCENTRANK({}; 3) in German format
        let result = codcel_percent_rank(vec![], 3.0, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_percent_rank_value_out_of_bounds() {
        // =PERCENTRANK({1,2,3,4,5}, 6) in US format
        // =PERCENTRANK({1;2;3;4;5}; 6) in German format
        let result = codcel_percent_rank(vec![1.0, 2.0, 3.0, 4.0, 5.0], 6.0, None);
        assert!(result.is_err());
    }
}
