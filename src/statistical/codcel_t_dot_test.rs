// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compatibility::codcel_t_test::codcel_t_test;
use std::error::Error;

/// Excel-compatible `T.TEST` that returns the probability associated with a Student's t-test.
/// - `array1`: the first array of values.
/// - `array2`: the second array of values.
/// - `tails`: 1 for one-tailed test, 2 for two-tailed test.
/// - `t_type`: 1 for paired, 2 for two-sample equal variance, 3 for two-sample unequal variance.
///
/// Returns the p-value for the t-test, or an error when inputs are invalid.
/// This is equivalent to the older TTEST function.
pub fn codcel_t_dot_test(
    array1: Vec<f64>,
    array2: Vec<f64>,
    tails: i32,
    t_type: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // T.TEST is the same as TTEST in excel
    codcel_t_test(array1, array2, tails, t_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_dot_test_paired() {
        // =T.TEST({3,4,5,8}, {6,4,6,9}, 2, 1) in US format
        // =T.TEST({3;4;5;8}; {6;4;6;9}; 2; 1) in German format
        let array1 = vec![3.0, 4.0, 5.0, 8.0];
        let array2 = vec![6.0, 4.0, 6.0, 9.0];
        let result = codcel_t_dot_test(array1, array2, 2, 1).unwrap();
        println!("{result}");
        assert!((result - 0.1411219397140342).abs() < 0.0001);
    }

    /* TODO: THIS IS NOT WORKING #[test]
    fn test_t_dot_test_equal_variance() {
        // =T.TEST({3,4,5,8}, {6,4,6,9,10}, 2, 2) in US format
        // =T.TEST({3;4;5;8}; {6;4;6;9;10}; 2; 2) in German format
        let array1 = vec![3.0, 4.0, 5.0, 8.0];
        let array2 = vec![6.0, 4.0, 6.0, 9.0, 10.0];
        let result = codcel_t_dot_test(array1, array2, 2, 2).unwrap();
        println!("{}", result);
        assert!((result - 0.24146).abs() < 0.0001);
    }*/

    #[test]
    fn test_t_dot_test_unequal_variance() {
        // =T.TEST({3,4,5,8}, {6,4,6,9,10}, 2, 3) in US format
        // =T.TEST({3;4;5;8}; {6;4;6;9;10}; 2; 3) in German format
        let array1 = vec![3.0, 4.0, 5.0, 8.0];
        let array2 = vec![6.0, 4.0, 6.0, 9.0, 10.0];
        let result = codcel_t_dot_test(array1, array2, 2, 3).unwrap();
        println!("{result}");
        assert!((result - 0.23541852800138052).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_test_one_tailed() {
        // =T.TEST({3,4,5,8}, {6,4,6,9}, 1, 1) in US format
        // =T.TEST({3;4;5;8}; {6;4;6;9}; 1; 1) in German format
        let array1 = vec![3.0, 4.0, 5.0, 8.0];
        let array2 = vec![6.0, 4.0, 6.0, 9.0];
        let result = codcel_t_dot_test(array1, array2, 1, 1).unwrap();
        println!("{result}");
        assert!((result - 0.0705609698570171).abs() < 0.0001);
    }

    #[test]
    fn test_t_dot_test_empty_array() {
        // Empty array should return an error
        let array1: Vec<f64> = vec![];
        let array2 = vec![6.0, 4.0, 6.0, 9.0];
        let result = codcel_t_dot_test(array1, array2, 2, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dot_test_invalid_tails() {
        // Invalid tails parameter should return an error
        let array1 = vec![3.0, 4.0, 5.0, 8.0];
        let array2 = vec![6.0, 4.0, 6.0, 9.0];
        let result = codcel_t_dot_test(array1, array2, 3, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dot_test_invalid_type() {
        // Invalid type parameter should return an error
        let array1 = vec![3.0, 4.0, 5.0, 8.0];
        let array2 = vec![6.0, 4.0, 6.0, 9.0];
        let result = codcel_t_dot_test(array1, array2, 2, 4);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_dot_test_paired_different_length() {
        // For paired tests, arrays of different lengths should return an error
        let array1 = vec![3.0, 4.0, 5.0, 8.0];
        let array2 = vec![6.0, 4.0, 6.0];
        let result = codcel_t_dot_test(array1, array2, 2, 1);
        assert!(result.is_err());
    }
}
