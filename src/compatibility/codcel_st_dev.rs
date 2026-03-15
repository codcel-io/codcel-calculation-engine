// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `STDEV`/`STDEV.S` function.
/// Returns the sample standard deviation (uses `n-1` in the denominator).
/// - `array`: array of numeric values (must contain at least two values).
///
/// Returns an error when fewer than two data points are provided.
pub fn codcel_st_dev(array: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("STDEV: Input array must not be empty.".into());
    }

    let len = array.len();
    if len < 2 {
        return Err("STDEV: At least two data points are required.".into());
    }

    let mean = array.iter().sum::<f64>() / len as f64;

    let variance = array
        .iter()
        .map(|value| (value - mean).powi(2))
        .sum::<f64>()
        / (len as f64 - 1.0);

    Ok(variance.sqrt())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_st_dev_basic() {
        // =STDEV({1,2,3,4,5}) in US format
        // =STDEV({1;2;3;4;5}) in German format
        let result = codcel_st_dev(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        println!("{result}");
        assert!((result - 1.5811388).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_larger_array() {
        // =STDEV({10,20,30,40,50,60,70,80,90,100}) in US format
        // =STDEV({10;20;30;40;50;60;70;80;90;100}) in German format
        let result = codcel_st_dev(vec![
            10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0,
        ])
        .unwrap();
        println!("{result}");
        assert!((result - 30.2765).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_decimal_values() {
        // =STDEV({1.5,2.5,3.5,4.5,5.5}) in US format
        // =STDEV({1,5;2,5;3,5;4,5;5,5}) in German format
        let result = codcel_st_dev(vec![1.5, 2.5, 3.5, 4.5, 5.5]).unwrap();
        println!("{result}");
        assert!((result - 1.5811388).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_negative_values() {
        // =STDEV({-5,-4,-3,-2,-1}) in US format
        // =STDEV({-5;-4;-3;-2;-1}) in German format
        let result = codcel_st_dev(vec![-5.0, -4.0, -3.0, -2.0, -1.0]).unwrap();
        println!("{result}");
        assert!((result - 1.5811388).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_mixed_values() {
        // =STDEV({-2,-1,0,1,2}) in US format
        // =STDEV({-2;-1;0;1;2}) in German format
        let result = codcel_st_dev(vec![-2.0, -1.0, 0.0, 1.0, 2.0]).unwrap();
        println!("{result}");
        assert!((result - 1.5811388).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_empty_array() {
        // =STDEV({}) in US format
        // =STDEV({}) in German format
        let result = codcel_st_dev(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_st_dev_single_element() {
        // =STDEV({1}) in US format
        // =STDEV({1}) in German format
        let result = codcel_st_dev(vec![1.0]);
        assert!(result.is_err());
    }
}
