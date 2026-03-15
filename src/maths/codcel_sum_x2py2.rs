// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `SUMX2PY2` that returns the sum of the sum of squares.
/// - `x`: the first array of numbers.
/// - `y`: the second array of numbers.
///
/// Returns Σ(x² + y²) or an error when arrays have different lengths.
pub fn codcel_sum_x2py2(x: Vec<f64>, y: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if arrays have the same length
    if x.len() != y.len() {
        return Err("SUMX2PY2: Arrays must have the same length".into());
    }

    if x.is_empty() {
        return Ok(0.0);
    }

    // Calculate the sum of x^2 + y^2 for each pair of elements
    let result = x
        .iter()
        .zip(y.iter())
        .map(|(x_val, y_val)| x_val.powi(2) + y_val.powi(2))
        .sum();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_x2py2_positive_numbers() {
        // =SUMX2PY2({1,2,3},{4,5,6}) in US format
        // =SUMX2PY2({1;2;3};{4;5;6}) in German format
        let result = codcel_sum_x2py2(vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]).unwrap();
        assert_eq!(result, 91.0); // (1^2 + 4^2) + (2^2 + 5^2) + (3^2 + 6^2) = (1 + 16) + (4 + 25) + (9 + 36) = 17 + 29 + 45 = 91
    }

    #[test]
    fn test_sum_x2py2_negative_numbers() {
        // =SUMX2PY2({-1,-2,-3},{-4,-5,-6}) in US format
        // =SUMX2PY2({-1;-2;-3};{-4;-5;-6}) in German format
        let result = codcel_sum_x2py2(vec![-1.0, -2.0, -3.0], vec![-4.0, -5.0, -6.0]).unwrap();
        assert_eq!(result, 91.0); // ((-1)^2 + (-4)^2) + ((-2)^2 + (-5)^2) + ((-3)^2 + (-6)^2) = (1 + 16) + (4 + 25) + (9 + 36) = 17 + 29 + 45 = 91
    }

    #[test]
    fn test_sum_x2py2_mixed_numbers() {
        // =SUMX2PY2({1,-2,3},{-4,5,-6}) in US format
        // =SUMX2PY2({1;-2;3};{-4;5;-6}) in German format
        let result = codcel_sum_x2py2(vec![1.0, -2.0, 3.0], vec![-4.0, 5.0, -6.0]).unwrap();
        assert_eq!(result, 91.0); // (1^2 + (-4)^2) + ((-2)^2 + 5^2) + (3^2 + (-6)^2) = (1 + 16) + (4 + 25) + (9 + 36) = 17 + 29 + 45 = 91
    }

    #[test]
    fn test_sum_x2py2_single_element() {
        // =SUMX2PY2({5},{3}) in US format
        // =SUMX2PY2({5};{3}) in German format
        let result = codcel_sum_x2py2(vec![5.0], vec![3.0]).unwrap();
        assert_eq!(result, 34.0); // 5^2 + 3^2 = 25 + 9 = 34
    }

    #[test]
    fn test_sum_x2py2_empty_arrays() {
        // =SUMX2PY2({},{}) in US format
        // =SUMX2PY2({};{}) in German format
        let result = codcel_sum_x2py2(vec![], vec![]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sum_x2py2_different_lengths() {
        // =SUMX2PY2({1,2,3},{4,5}) in US format (returns #N/A error)
        // =SUMX2PY2({1;2;3};{4;5}) in German format (returns #N/A error)
        let result = codcel_sum_x2py2(vec![1.0, 2.0, 3.0], vec![4.0, 5.0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_x2py2_decimals() {
        // =SUMX2PY2({1.5,2.5},{3.5,4.5}) in US format
        // =SUMX2PY2({1,5;2,5};{3,5;4,5}) in German format
        let result = codcel_sum_x2py2(vec![1.5, 2.5], vec![3.5, 4.5]).unwrap();
        assert!((result - 41.0).abs() < 1e-10); // (1.5^2 + 3.5^2) + (2.5^2 + 4.5^2) = (2.25 + 12.25) + (6.25 + 20.25) = 14.5 + 26.5 = 41
    }

    #[test]
    fn test_sum_x2py2_zeros() {
        // =SUMX2PY2({0,0,0},{0,0,0}) in US format
        // =SUMX2PY2({0;0;0};{0;0;0}) in German format
        let result = codcel_sum_x2py2(vec![0.0, 0.0, 0.0], vec![0.0, 0.0, 0.0]).unwrap();
        assert_eq!(result, 0.0); // (0^2 + 0^2) + (0^2 + 0^2) + (0^2 + 0^2) = 0 + 0 + 0 = 0
    }
}
