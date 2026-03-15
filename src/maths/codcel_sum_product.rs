// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `SUMPRODUCT` that multiplies corresponding components and returns their sum.
/// - `values`: a list of arrays of equal size.
///
/// Returns the sum of products or an error when arrays are empty or have different lengths.
pub fn codcel_sum_product(values: Vec<Vec<f64>>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if there are any arrays to process
    if values.is_empty() {
        return Err("SUMPRODUCT: No arrays provided for calculation".into());
    }

    // Get the length of the first array
    let expected_length = values[0].len();

    // Check if the first array is empty
    if expected_length == 0 {
        return Err("SUMPRODUCT: Empty array provided for calculation".into());
    }

    // Check if all arrays have the same length
    for (i, array) in values.iter().enumerate() {
        if array.len() != expected_length {
            return Err(format!(
                "SUMPRODUCT: Array at index {} has length {}, but expected length {}",
                i,
                array.len(),
                expected_length
            )
            .into());
        }
    }

    // Calculate the sum of products
    let mut sum = 0.0;

    for i in 0..expected_length {
        let mut product = 1.0;

        for array in &values {
            product *= array[i];
        }

        sum += product;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_product_two_arrays() {
        // =SUMPRODUCT({1,2,3},{4,5,6}) in US format
        // =SUMPRODUCT({1;2;3};{4;5;6}) in German format
        let result = codcel_sum_product(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]).unwrap();
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_sum_product_three_arrays() {
        // =SUMPRODUCT({1,2,3},{4,5,6},{2,2,2}) in US format
        // =SUMPRODUCT({1;2;3};{4;5;6};{2;2;2}) in German format
        let result = codcel_sum_product(vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![2.0, 2.0, 2.0],
        ])
        .unwrap();
        assert_eq!(result, 64.0); // 1*4*2 + 2*5*2 + 3*6*2 = 8 + 20 + 36 = 64
    }

    #[test]
    fn test_sum_product_negative_numbers() {
        // =SUMPRODUCT({-1,-2,-3},{4,5,6}) in US format
        // =SUMPRODUCT({-1;-2;-3};{4;5;6}) in German format
        let result = codcel_sum_product(vec![vec![-1.0, -2.0, -3.0], vec![4.0, 5.0, 6.0]]).unwrap();
        assert_eq!(result, -32.0); // -1*4 + -2*5 + -3*6 = -4 - 10 - 18 = -32
    }

    #[test]
    fn test_sum_product_mixed_numbers() {
        // =SUMPRODUCT({-1,2,-3},{-4,5,-6}) in US format
        // =SUMPRODUCT({-1;2;-3};{-4;5;-6}) in German format
        let result =
            codcel_sum_product(vec![vec![-1.0, 2.0, -3.0], vec![-4.0, 5.0, -6.0]]).unwrap();
        assert_eq!(result, 32.0); // -1*-4 + 2*5 + -3*-6 = 4 + 10 + 18 = 32
    }

    #[test]
    fn test_sum_product_single_element() {
        // =SUMPRODUCT({5},{10}) in US format
        // =SUMPRODUCT({5};{10}) in German format
        let result = codcel_sum_product(vec![vec![5.0], vec![10.0]]).unwrap();
        assert_eq!(result, 50.0); // 5*10 = 50
    }

    #[test]
    fn test_sum_product_decimals() {
        // =SUMPRODUCT({1.5,2.5},{3.5,4.5}) in US format
        // =SUMPRODUCT({1,5;2,5};{3,5;4,5}) in German format
        let result = codcel_sum_product(vec![vec![1.5, 2.5], vec![3.5, 4.5]]).unwrap();
        assert!((result - 16.5).abs() < 1e-10); // 1.5*3.5 + 2.5*4.5 = 5.25 + 11.25 = 16.5
    }

    #[test]
    fn test_sum_product_no_arrays() {
        // =SUMPRODUCT() in US format (returns #VALUE! error)
        // =SUMPRODUCT() in German format (returns #VALUE! error)
        let result = codcel_sum_product(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_product_empty_array() {
        // =SUMPRODUCT({},{}) in US format (returns #VALUE! error)
        // =SUMPRODUCT({};{}) in German format (returns #VALUE! error)
        let result = codcel_sum_product(vec![vec![], vec![]]);
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_product_different_lengths() {
        // =SUMPRODUCT({1,2,3},{4,5}) in US format (returns #VALUE! error)
        // =SUMPRODUCT({1;2;3};{4;5}) in German format (returns #VALUE! error)
        let result = codcel_sum_product(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0]]);
        assert!(result.is_err());
    }
}
