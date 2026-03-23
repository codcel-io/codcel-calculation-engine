// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `SKEW` that returns the skewness of a distribution (sample-based).
/// - `values`: an array of numeric values (must have at least 3 values).
///
/// Returns the skewness, which characterizes the degree of asymmetry of a distribution.
/// A positive skew indicates a tail extending toward more positive values;
/// a negative skew indicates a tail extending toward more negative values.
pub fn codcel_skew(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check for minimum number of values
    if values.len() < 3 {
        return Err("SKEW: At least 3 values are required to calculate skewness".into());
    }

    // Calculate mean
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;

    // Calculate standard deviation
    let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    let std_dev = crate::portable_math::sqrt(variance);

    // Calculate skewness (Excel's method)
    let skewness = values
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(3))
        .sum::<f64>()
        * (n / ((n - 1.0) * (n - 2.0)));

    Ok(skewness)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skew_symmetric_distribution() {
        // =SKEW({1,2,3,4,5,6,7,8,9}) in US format
        // =SKEW({1;2;3;4;5;6;7;8;9}) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_skew(values).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_skew_positive_skew() {
        // =SKEW({1,2,3,4,5,6,7,8,100}) in US format
        // =SKEW({1;2;3;4;5;6;7;8;100}) in German format
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 100.0];
        let result = codcel_skew(values).unwrap();
        println!("{result}");
        assert!(result > 0.0);
        assert!((result - 2.970226993676901).abs() < 0.0001);
    }

    #[test]
    fn test_skew_negative_skew() {
        // =SKEW({100,2,3,4,5,6,7,8,9}) in US format
        // =SKEW({100;2;3;4;5;6;7;8;9}) in German format
        let values = vec![100.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_skew(values).unwrap();
        println!("{result}");
        assert!((result - 2.9695978941759025).abs() < 0.0001);
    }

    #[test]
    fn test_skew_small_dataset() {
        // =SKEW({3,4,5,2,3,4,5,6,4,7}) in US format
        // =SKEW({3;4;5;2;3;4;5;6;4;7}) in German format
        let values = vec![3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 6.0, 4.0, 7.0];
        let result = codcel_skew(values).unwrap();
        assert!((result - 0.3595).abs() < 0.0001);
    }

    #[test]
    fn test_skew_too_few_values() {
        // Too few values should return an error
        let values = vec![1.0, 2.0];
        let result = codcel_skew(values);
        assert!(result.is_err());
    }

    #[test]
    fn test_skew_identical_values() {
        // Identical values should result in a division by zero error
        let values = vec![5.0, 5.0, 5.0, 5.0, 5.0];
        let result = codcel_skew(values);
        // The result should be NaN because std_dev is 0
        assert!(result.unwrap().is_nan());
    }
}
