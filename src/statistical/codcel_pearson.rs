// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `PEARSON` that returns the Pearson product-moment correlation coefficient.
/// - `array1`: the first array of values.
/// - `array2`: the second array of values (must have the same length as `array1`).
///
/// Returns the Pearson correlation coefficient (between -1 and 1),
/// or an error when arrays are empty or have different lengths.
pub fn codcel_pearson(
    array1: Vec<f64>,
    array2: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if array1.len() != array2.len() {
        return Err("PEARSON: Both arrays must have the same length.".into());
    }
    if array1.is_empty() || array2.is_empty() {
        return Err("PEARSON: Input arrays must not be empty.".into());
    }

    let len = array1.len() as f64;

    // Calculate means
    let mean1 = array1.iter().sum::<f64>() / len;
    let mean2 = array2.iter().sum::<f64>() / len;

    // Compute covariance and standard deviations
    let mut covariance = 0.0;
    let mut variance1 = 0.0;
    let mut variance2 = 0.0;

    for i in 0..array1.len() {
        let diff1 = array1[i] - mean1;
        let diff2 = array2[i] - mean2;
        covariance += diff1 * diff2;
        variance1 += diff1 * diff1;
        variance2 += diff2 * diff2;
    }

    // Handle edge cases for zero variance
    if variance1 == 0.0 || variance2 == 0.0 {
        return Err("PEARSON: One of the inputs has zero variance.".into());
    }

    // Pearson correlation coefficient
    let result = covariance / (crate::portable_math::sqrt(variance1) * crate::portable_math::sqrt(variance2));

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pearson_perfect_positive_correlation() {
        // =PEARSON({1,2,3,4,5}, {1,2,3,4,5}) in US format
        // =PEARSON({1;2;3;4;5}; {1;2;3;4;5}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_pearson(array1, array2).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_pearson_perfect_negative_correlation() {
        // =PEARSON({1,2,3,4,5}, {5,4,3,2,1}) in US format
        // =PEARSON({1;2;3;4;5}; {5;4;3;2;1}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        let result = codcel_pearson(array1, array2).unwrap();
        assert!((result + 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_pearson_no_correlation() {
        // =PEARSON({1,2,3,4,5}, {5,2,4,1,3}) in US format
        // =PEARSON({1;2;3;4;5}; {5;2;4;1;3}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![5.0, 2.0, 4.0, 1.0, 3.0];
        let result = codcel_pearson(array1, array2).unwrap();
        println!("{result}");
        assert!((result - -0.5).abs() < 0.0001);
    }

    #[test]
    fn test_pearson_partial_correlation() {
        // =PEARSON({1,2,3,4,5}, {2,3,4,5,6}) in US format
        // =PEARSON({1;2;3;4;5}; {2;3;4;5;6}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![2.0, 3.0, 4.0, 5.0, 6.0];
        let result = codcel_pearson(array1, array2).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_pearson_real_data() {
        // =PEARSON({10,8,13,9,11,14,6,4,12,7,5}, {8.04,6.95,7.58,8.81,8.33,9.96,7.24,4.26,10.84,4.82,5.68}) in US format
        // =PEARSON({10;8;13;9;11;14;6;4;12;7;5}; {8,04;6,95;7,58;8,81;8,33;9,96;7,24;4,26;10,84;4,82;5,68}) in German format
        let array1 = vec![10.0, 8.0, 13.0, 9.0, 11.0, 14.0, 6.0, 4.0, 12.0, 7.0, 5.0];
        let array2 = vec![
            8.04, 6.95, 7.58, 8.81, 8.33, 9.96, 7.24, 4.26, 10.84, 4.82, 5.68,
        ];
        let result = codcel_pearson(array1, array2).unwrap();
        assert!((result - 0.816).abs() < 0.001);
    }

    #[test]
    fn test_pearson_different_length_arrays() {
        // Different length arrays should return an error
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![1.0, 2.0, 3.0, 4.0];
        let result = codcel_pearson(array1, array2);
        assert!(result.is_err());
    }

    #[test]
    fn test_pearson_empty_arrays() {
        // Empty arrays should return an error
        let array1: Vec<f64> = vec![];
        let array2: Vec<f64> = vec![];
        let result = codcel_pearson(array1, array2);
        assert!(result.is_err());
    }

    #[test]
    fn test_pearson_zero_variance() {
        // Zero variance should return an error
        let array1 = vec![1.0, 1.0, 1.0, 1.0, 1.0];
        let array2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_pearson(array1, array2);
        assert!(result.is_err());
    }
}
