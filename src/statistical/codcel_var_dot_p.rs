// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `VAR.P` that returns the variance based on an entire population.
/// - `data`: an array of numeric values representing the entire population.
///
/// Returns the population variance (divides by n),
/// or an error when the data is empty.
pub fn codcel_var_dot_p(data: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if data.is_empty() {
        return Err("VAR.P: Data set cannot be empty.".into());
    }

    let mean = data.iter().sum::<f64>() / data.len() as f64;

    let variance = data.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64;

    Ok(variance)
}

pub fn codcel_var_p(data: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // VARP is the same as VAR.P
    codcel_var_dot_p(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_dot_p_basic() {
        // =VAR.P(1,2,3,4,5) in US format
        // =VAR.P(1;2;3;4;5) in German format
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_var_dot_p(data).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_var_dot_p_specific_variance() {
        // =VAR.P(4,5,8,7,11,4,3) in US format
        // =VAR.P(4;5;8;7;11;4;3) in German format
        let data = vec![4.0, 5.0, 8.0, 7.0, 11.0, 4.0, 3.0];
        let result = codcel_var_dot_p(data).unwrap();
        println!("{result}");
        assert!((result - 6.857142857142857).abs() < 0.0001);
    }

    #[test]
    fn test_var_dot_p_single_value() {
        // =VAR.P(5) in US format
        // =VAR.P(5) in German format
        let data = vec![5.0];
        let result = codcel_var_dot_p(data).unwrap();
        println!("{result}");
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_var_dot_p_negative_values() {
        // =VAR.P(-1,-2,-3,-4,-5) in US format
        // =VAR.P(-1;-2;-3;-4;-5) in German format
        let data = vec![-1.0, -2.0, -3.0, -4.0, -5.0];
        let result = codcel_var_dot_p(data).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_var_dot_p_mixed_values() {
        // =VAR.P(-2,-1,0,1,2) in US format
        // =VAR.P(-2;-1;0;1;2) in German format
        let data = vec![-2.0, -1.0, 0.0, 1.0, 2.0];
        let result = codcel_var_dot_p(data).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_var_dot_p_empty_data() {
        // Empty data should return an error
        let data: Vec<f64> = vec![];
        let result = codcel_var_dot_p(data);
        assert!(result.is_err());
    }

    #[test]
    fn test_var_p_alias() {
        // Test that VARP works the same as VAR.P
        // =VARP(1,2,3,4,5) in US format
        // =VARP(1;2;3;4;5) in German format
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_var_p(data).unwrap();
        println!("{result}");
        assert!((result - 2.0).abs() < 0.0001);
    }
}
