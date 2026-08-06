// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `TRIMMEAN` that returns the mean of the interior of a data set.
/// - `data`: an array of numeric values.
/// - `percent`: the fractional number of data points to exclude (0 to 1).
///
/// Returns the mean after excluding the specified percentage from top and bottom,
/// or an error when the data is empty or percent is out of range.
pub fn codcel_trim_mean(data: Vec<f64>, percent: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if data.is_empty() {
        return Err("TRIMMEAN: Data set cannot be empty.".into());
    }

    if !(0.0..=1.0).contains(&percent) {
        return Err("TRIMMEAN: Percent must be between 0 and 1.".into());
    }

    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let total_count = sorted_data.len();
    let remove_count = ((total_count as f64) * percent / 2.0).floor() as usize;

    if remove_count * 2 >= total_count {
        return Err("TRIMMEAN: Percentage excludes all data points.".into());
    }

    let trimmed_data = &sorted_data[remove_count..(total_count - remove_count)];
    let trimmed_mean = trimmed_data.iter().sum::<f64>() / trimmed_data.len() as f64;

    Ok(trimmed_mean)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_mean_basic() {
        // =TRIMMEAN({1,2,3,4,5,6,7,8,9,10}, 0.2) in US format
        // =TRIMMEAN({1;2;3;4;5;6;7;8;9;10}; 0,2) in German format
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_trim_mean(data, 0.2).unwrap();
        println!("{result}");
        assert!((result - 5.5).abs() < 0.0001);
    }

    #[test]
    fn test_trim_mean_unsorted_data() {
        // =TRIMMEAN({10,2,8,4,6,1,7,3,9,5}, 0.2) in US format
        // =TRIMMEAN({10;2;8;4;6;1;7;3;9;5}; 0,2) in German format
        let data = vec![10.0, 2.0, 8.0, 4.0, 6.0, 1.0, 7.0, 3.0, 9.0, 5.0];
        let result = codcel_trim_mean(data, 0.2).unwrap();
        println!("{result}");
        assert!((result - 5.5).abs() < 0.0001);
    }

    #[test]
    fn test_trim_mean_zero_percent() {
        // =TRIMMEAN({1,2,3,4,5,6,7,8,9,10}, 0) in US format
        // =TRIMMEAN({1;2;3;4;5;6;7;8;9;10}; 0) in German format
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_trim_mean(data, 0.0).unwrap();
        println!("{result}");
        assert!((result - 5.5).abs() < 0.0001);
    }

    #[test]
    fn test_trim_mean_high_percent() {
        // =TRIMMEAN({1,2,3,4,5,6,7,8,9,10}, 0.6) in US format
        // =TRIMMEAN({1;2;3;4;5;6;7;8;9;10}; 0,6) in German format
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_trim_mean(data, 0.6).unwrap();
        println!("{result}");
        assert!((result - 5.5).abs() < 0.0001);
    }

    #[test]
    fn test_trim_mean_odd_length() {
        // =TRIMMEAN({1,2,3,4,5,6,7,8,9}, 0.2) in US format
        // =TRIMMEAN({1;2;3;4;5;6;7;8;9}; 0,2) in German format
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_trim_mean(data, 0.2).unwrap();
        println!("{result}");
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_trim_mean_negative_values() {
        // =TRIMMEAN({-5,-4,-3,-2,-1,0,1,2,3,4,5}, 0.2) in US format
        // =TRIMMEAN({-5;-4;-3;-2;-1;0;1;2;3;4;5}; 0,2) in German format
        let data = vec![-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_trim_mean(data, 0.2).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_trim_mean_empty_data() {
        // Empty data should return an error
        let data: Vec<f64> = vec![];
        let result = codcel_trim_mean(data, 0.2);
        assert!(result.is_err());
    }

    #[test]
    fn test_trim_mean_negative_percent() {
        // Negative percent should return an error
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_trim_mean(data, -0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_trim_mean_percent_too_high() {
        // Percent > 1 should return an error
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_trim_mean(data, 1.1);
        assert!(result.is_err());
    }
}
