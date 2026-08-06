// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

/// Excel-compatible `STDEVP`/`STDEV.P` function.
/// Returns the population standard deviation (uses `n` in the denominator).
/// - `array`: array of numeric values (must not be empty).
///
/// Returns an error when the array is empty.
pub fn codcel_st_dev_p(array: Vec<f64>) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    if array.is_empty() {
        return Err("STDEVP: Array cannot be empty".into());
    }

    let mean = array.iter().sum::<f64>() / array.len() as f64;
    let variance = array.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / array.len() as f64;

    Ok(crate::portable_math::sqrt(variance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_st_dev_p_basic() {
        // =STDEVP({1,2,3,4,5}) in US format
        // =STDEVP({1;2;3;4;5}) in German format
        let result = codcel_st_dev_p(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        println!("{result}");
        assert!((result - 1.4142135).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_p_larger_array() {
        // =STDEVP({10,20,30,40,50,60,70,80,90,100}) in US format
        // =STDEVP({10;20;30;40;50;60;70;80;90;100}) in German format
        let result = codcel_st_dev_p(vec![
            10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0,
        ])
        .unwrap();
        println!("{result}");
        assert!((result - 28.7228).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_p_decimal_values() {
        // =STDEVP({1.5,2.5,3.5,4.5,5.5}) in US format
        // =STDEVP({1,5;2,5;3,5;4,5;5,5}) in German format
        let result = codcel_st_dev_p(vec![1.5, 2.5, 3.5, 4.5, 5.5]).unwrap();
        println!("{result}");
        assert!((result - 1.4142135).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_p_negative_values() {
        // =STDEVP({-5,-4,-3,-2,-1}) in US format
        // =STDEVP({-5;-4;-3;-2;-1}) in German format
        let result = codcel_st_dev_p(vec![-5.0, -4.0, -3.0, -2.0, -1.0]).unwrap();
        println!("{result}");
        assert!((result - 1.4142135).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_p_mixed_values() {
        // =STDEVP({-2,-1,0,1,2}) in US format
        // =STDEVP({-2;-1;0;1;2}) in German format
        let result = codcel_st_dev_p(vec![-2.0, -1.0, 0.0, 1.0, 2.0]).unwrap();
        println!("{result}");
        assert!((result - 1.4142135).abs() < 0.0001);
    }

    #[test]
    fn test_st_dev_p_single_element() {
        // =STDEVP({1}) in US format
        // =STDEVP({1}) in German format
        let result = codcel_st_dev_p(vec![1.0]).unwrap();
        println!("{result}");
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_st_dev_p_empty_array() {
        // =STDEVP({}) in US format
        // =STDEVP({}) in German format
        let result = codcel_st_dev_p(vec![]);
        assert!(result.is_err());
    }
}
