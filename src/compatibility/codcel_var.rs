// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use std::error::Error;

/// Excel-compatible `VAR`/`VAR.S` function.
/// Returns the sample variance (uses `n-1` in the denominator).
/// - `data`: array of numeric values (must contain at least two values).
///
/// Returns an error when fewer than two observations are supplied.
pub fn codcel_var(data: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if data.len() < 2 {
        return Err("VAR: Data set must contain at least two elements.".into());
    }

    let mean = data.iter().compensated_sum() / data.len() as f64;

    let variance =
        data.iter().map(|&x| (x - mean).powi(2)).compensated_sum() / (data.len() as f64 - 1.0);

    Ok(variance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_basic() {
        // =VAR(1,2,3,4,5) in US format
        // =VAR(1;2;3;4;5) in German format
        let result = codcel_var(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        println!("{result}");
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_var_larger_array() {
        // =VAR(10,20,30,40,50,60,70,80,90,100) in US format
        // =VAR(10;20;30;40;50;60;70;80;90;100) in German format
        let result = codcel_var(vec![
            10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0,
        ])
        .unwrap();
        println!("{result}");
        assert!((result - 916.6666667).abs() < 0.0001);
    }

    #[test]
    fn test_var_decimal_values() {
        // =VAR(1.5,2.5,3.5,4.5,5.5) in US format
        // =VAR(1,5;2,5;3,5;4,5;5,5) in German format
        let result = codcel_var(vec![1.5, 2.5, 3.5, 4.5, 5.5]).unwrap();
        println!("{result}");
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_var_negative_values() {
        // =VAR(-5,-4,-3,-2,-1) in US format
        // =VAR(-5;-4;-3;-2;-1) in German format
        let result = codcel_var(vec![-5.0, -4.0, -3.0, -2.0, -1.0]).unwrap();
        println!("{result}");
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_var_mixed_values() {
        // =VAR(-2,-1,0,1,2) in US format
        // =VAR(-2;-1;0;1;2) in German format
        let result = codcel_var(vec![-2.0, -1.0, 0.0, 1.0, 2.0]).unwrap();
        println!("{result}");
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_var_same_values() {
        // =VAR(1,1,1,1,1) in US format
        // =VAR(1;1;1;1;1) in German format
        let result = codcel_var(vec![1.0, 1.0, 1.0, 1.0, 1.0]).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_var_empty_array() {
        // =VAR() in US format
        // =VAR() in German format
        let result = codcel_var(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_var_single_element() {
        // =VAR(1) in US format
        // =VAR(1) in German format
        let result = codcel_var(vec![1.0]);
        assert!(result.is_err());
    }
}
