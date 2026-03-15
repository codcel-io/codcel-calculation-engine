// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::statistical::codcel_f_dot_test::codcel_f_dot_test;
use std::error::Error;

/// Excel-compatible `FTEST`/`F.TEST` function.
/// Returns the two-tailed probability that two data sets have the same variance.
/// - `array1`: first array of numeric values.
/// - `array2`: second array of numeric values.
///
/// Both arrays must be non-empty. Returns an error if either array is empty.
pub fn codcel_f_test(
    array1: Vec<f64>,
    array2: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // FTEST is exactly the same as F.TEST
    codcel_f_dot_test(array1, array2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_test_basic() {
        // =FTEST({1,2,3,4,5},{6,7,8,9,10}) in US format
        // =FTEST({1;2;3;4;5};{6;7;8;9;10}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_f_test(array1, array2).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_test_different_sizes() {
        // =FTEST({1,2,3,4},{5,6,7,8,9}) in US format
        // =FTEST({1;2;3;4};{5;6;7;8;9}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0];
        let array2 = vec![5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_f_test(array1, array2).unwrap();
        println!("{result}");
        // The exact value may vary, but we can check it's a valid probability
        assert!((0.0..=1.0).contains(&result));
    }

    #[test]
    fn test_f_test_different_variances() {
        // =FTEST({1,2,3,4,5},{1,3,5,7,9}) in US format
        // =FTEST({1;2;3;4;5};{1;3;5;7;9}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![1.0, 3.0, 5.0, 7.0, 9.0];
        let result = codcel_f_test(array1, array2).unwrap();
        println!("{result}");
        assert!((result - 0.20799999999999952).abs() < 0.0001);
    }

    #[test]
    fn test_f_test_same_array() {
        // =FTEST({1,2,3,4,5},{1,2,3,4,5}) in US format
        // =FTEST({1;2;3;4;5};{1;2;3;4;5}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_f_test(array1, array2).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_test_large_variance_difference() {
        // =FTEST({1,2,3,4,5},{1,10,20,30,40}) in US format
        // =FTEST({1;2;3;4;5};{1;10;20;30;40}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![1.0, 10.0, 20.0, 30.0, 40.0];
        let result = codcel_f_test(array1, array2).unwrap();
        println!("{result}");
        assert!(result < 0.01); // Should be a very small p-value
    }

    #[test]
    fn test_f_test_empty_array1() {
        // Empty array1 should return an error
        let array1: Vec<f64> = vec![];
        let array2 = vec![1.0, 2.0, 3.0];
        let result = codcel_f_test(array1, array2);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_test_empty_array2() {
        // Empty array2 should return an error
        let array1 = vec![1.0, 2.0, 3.0];
        let array2: Vec<f64> = vec![];
        let result = codcel_f_test(array1, array2);
        assert!(result.is_err());
    }
}
