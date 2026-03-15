// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `SKEW.P` that returns the skewness of a distribution (population-based).
/// - `values`: an array of numeric values.
///
/// Returns the population skewness, which characterizes the degree of asymmetry.
/// Unlike SKEW, this function calculates skewness based on a population rather than a sample.
pub fn codcel_skew_p(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check for minimum number of values
    if values.is_empty() {
        return Err("SKEW.P: At least one value is required to calculate skewness".into());
    }

    // Calculate mean
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;

    // Calculate standard deviation
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n; // Population variance
    let std_dev = variance.sqrt();

    if std_dev == 0.0 {
        return Err("SKEW.P: Standard deviation is zero, skewness cannot be computed.".into());
    }

    // Calculate skewness (population method)
    let skewness = values
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(3))
        .sum::<f64>()
        / n;

    Ok(skewness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skew_p_symmetric_distribution() {
        // =SKEW.P({1,2,3,4,5,6,7,8,9}) in US format
        // =SKEW.P({1;2;3;4;5;6;7;8;9}) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_skew_p(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_skew_p_positive_skew() {
        // =SKEW.P({1,2,3,4,5,6,7,8,100}) in US format
        // =SKEW.P({1;2;3;4;5;6;7;8;100}) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 100.0];
        let result = codcel_skew_p(values).unwrap();
        println!("{result}");
        assert!(result > 0.0);
        assert!((result - 2.4503122570409803).abs() < 0.0001);
    }

    #[test]
    fn test_skew_p_negative_skew() {
        // =SKEW.P({100,2,3,4,5,6,7,8,9}) in US format
        // =SKEW.P({100;2;3;4;5;6;7;8;9}) in German format
        let values = vec![100.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_skew_p(values).unwrap();
        println!("{result}");
        assert!((result - 2.449793276430584).abs() < 0.0001);
    }

    #[test]
    fn test_skew_p_small_dataset() {
        // =SKEW.P({3,4,5,2,3,4,5,6,4,7}) in US format
        // =SKEW.P({3;4;5;2;3;4;5;6;4;7}) in German format
        let values = vec![3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 6.0, 4.0, 7.0];
        let result = codcel_skew_p(values).unwrap();
        println!("{result}");
        assert!((result - 0.3031933393541438).abs() < 0.0001);
    }

    #[test]
    fn test_skew_p_single_value() {
        // =SKEW.P({5}) in US format
        // =SKEW.P({5}) in German format
        // This should error because standard deviation is zero
        let values = vec![5.0];
        let result = codcel_skew_p(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_skew_p_empty_array() {
        // Empty array should return an error
        let values: Vec<f64> = vec![];
        let result = codcel_skew_p(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_skew_p_identical_values() {
        // Identical values should result in an error due to zero standard deviation
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_skew_p(values);
        assert!(result.is_err());
    }
}
