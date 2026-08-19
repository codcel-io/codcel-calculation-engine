// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use std::error::Error;

/// Excel-compatible `SUMX2MY2` that returns the sum of the difference of squares.
/// - `x`: the first array of numbers.
/// - `y`: the second array of numbers.
///
/// Returns Σ(x² - y²) or an error when arrays have different lengths.
pub fn codcel_sum_x2my2(x: Vec<f64>, y: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if arrays have the same length
    if x.len() != y.len() {
        return Err("SUMX2MY2: Arrays must have the same length".into());
    }

    if x.is_empty() {
        return Ok(0.0);
    }

    // Calculate the sum of x^2 - y^2 for each pair of elements
    let result = x
        .iter()
        .zip(y.iter())
        .map(|(x_val, y_val)| x_val.powi(2) - y_val.powi(2))
        .compensated_sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_x2my2_positive_numbers() {
        // =SUMX2MY2({1,2,3},{4,5,6}) in US format
        // =SUMX2MY2({1;2;3};{4;5;6}) in German format
        let result = codcel_sum_x2my2(vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]).unwrap();
        assert_eq!(result, -63.0); // (1^2 - 4^2) + (2^2 - 5^2) + (3^2 - 6^2) = (1 - 16) + (4 - 25) + (9 - 36) = -15 - 21 - 27 = -63
    }

    #[test]
    fn test_sum_x2my2_negative_numbers() {
        // =SUMX2MY2({-1,-2,-3},{-4,-5,-6}) in US format
        // =SUMX2MY2({-1;-2;-3};{-4;-5;-6}) in German format
        let result = codcel_sum_x2my2(vec![-1.0, -2.0, -3.0], vec![-4.0, -5.0, -6.0]).unwrap();
        assert_eq!(result, -63.0); // ((-1)^2 - (-4)^2) + ((-2)^2 - (-5)^2) + ((-3)^2 - (-6)^2) = (1 - 16) + (4 - 25) + (9 - 36) = -15 - 21 - 27 = -63
    }

    #[test]
    fn test_sum_x2my2_mixed_numbers() {
        // =SUMX2MY2({1,-2,3},{-4,5,-6}) in US format
        // =SUMX2MY2({1;-2;3};{-4;5;-6}) in German format
        let result = codcel_sum_x2my2(vec![1.0, -2.0, 3.0], vec![-4.0, 5.0, -6.0]).unwrap();
        assert_eq!(result, -63.0); // (1^2 - (-4)^2) + ((-2)^2 - 5^2) + (3^2 - (-6)^2) = (1 - 16) + (4 - 25) + (9 - 36) = -15 - 21 - 27 = -63
    }

    #[test]
    fn test_sum_x2my2_single_element() {
        // =SUMX2MY2({5},{3}) in US format
        // =SUMX2MY2({5};{3}) in German format
        let result = codcel_sum_x2my2(vec![5.0], vec![3.0]).unwrap();
        assert_eq!(result, 16.0); // 5^2 - 3^2 = 25 - 9 = 16
    }

    #[test]
    fn test_sum_x2my2_empty_arrays() {
        // =SUMX2MY2({},{}) in US format
        // =SUMX2MY2({};{}) in German format
        let result = codcel_sum_x2my2(vec![], vec![]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sum_x2my2_different_lengths() {
        // =SUMX2MY2({1,2,3},{4,5}) in US format (returns #N/A error)
        // =SUMX2MY2({1;2;3};{4;5}) in German format (returns #N/A error)
        let result = codcel_sum_x2my2(vec![1.0, 2.0, 3.0], vec![4.0, 5.0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_x2my2_decimals() {
        // =SUMX2MY2({1.5,2.5},{3.5,4.5}) in US format
        // =SUMX2MY2({1,5;2,5};{3,5;4,5}) in German format
        let result = codcel_sum_x2my2(vec![1.5, 2.5], vec![3.5, 4.5]).unwrap();
        assert!((result - (-24.0)).abs() < 1e-10); // (1.5^2 - 3.5^2) + (2.5^2 - 4.5^2) = (2.25 - 12.25) + (6.25 - 20.25) = -10 - 14 = -24
    }

    #[test]
    fn test_sum_x2my2_equal_values() {
        // =SUMX2MY2({1,2,3},{1,2,3}) in US format
        // =SUMX2MY2({1;2;3};{1;2;3}) in German format
        let result = codcel_sum_x2my2(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(result, 0.0); // (1^2 - 1^2) + (2^2 - 2^2) + (3^2 - 3^2) = 0 + 0 + 0 = 0
    }
}
