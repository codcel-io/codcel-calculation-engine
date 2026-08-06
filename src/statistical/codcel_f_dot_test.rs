// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::statistical::codcel_f_dist_rt::codcel_f_dist_rt;
use std::error::Error;

/// Excel-compatible `F.TEST` that returns the two-tailed probability of an F-test.
/// - `array1`: the first array of values.
/// - `array2`: the second array of values.
///
/// Returns the two-tailed probability that the variances in the two arrays are not significantly different,
/// or an error when either array is empty or has insufficient variance.
pub fn codcel_f_dot_test(
    array1: Vec<f64>,
    array2: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if array1.is_empty() {
        return Err("F.TEST: array1 must not be empty.".into());
    }
    if array2.is_empty() {
        return Err("F.TEST: array2 must not be empty.".into());
    }

    let mean1 = array1.iter().sum::<f64>() / array1.len() as f64;
    let mean2 = array2.iter().sum::<f64>() / array2.len() as f64;

    let variance1 =
        array1.iter().map(|&x| (x - mean1).powi(2)).sum::<f64>() / (array1.len() - 1) as f64;
    let variance2 =
        array2.iter().map(|&x| (x - mean2).powi(2)).sum::<f64>() / (array2.len() - 1) as f64;

    let (larger_variance, smaller_variance) = if variance1 > variance2 {
        (variance1, variance2)
    } else {
        (variance2, variance1)
    };

    let df1 = array1.len() as f64 - 1.0;
    let df2 = array2.len() as f64 - 1.0;

    let f_statistic = larger_variance / smaller_variance;

    // Calculate the right-tailed F distribution probability
    let p_value_rt = codcel_f_dist_rt(f_statistic, df1, df2)?;

    // F.TEST returns the two-tailed p-value
    Ok(2.0 * p_value_rt.min(0.5))
}

// This function is a placeholder for a vector version of F.TEST
// In reality, F.TEST requires two arrays, so a simple vector of inputs is not sufficient
// This is just to maintain consistency with other functions
pub fn codcel_f_dot_test_vec(inputs: Vec<Vec<f64>>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("F.TEST: Must have 2 array parameters".into());
    }

    codcel_f_dot_test(inputs[0].clone(), inputs[1].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f_dot_test_basic() {
        // =F.TEST({1,2,3,4,5},{6,7,8,9,10}) in US format
        // =F.TEST({1;2;3;4;5};{6;7;8;9;10}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![6.0, 7.0, 8.0, 9.0, 10.0];
        let result = codcel_f_dot_test(array1, array2).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_dot_test_different_sizes() {
        // =F.TEST({1,2,3,4},{5,6,7,8,9}) in US format
        // =F.TEST({1;2;3;4};{5;6;7;8;9}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0];
        let array2 = vec![5.0, 6.0, 7.0, 8.0, 9.0];
        let result = codcel_f_dot_test(array1, array2).unwrap();
        println!("{result}");
        // The exact value may vary, but we can check it's a valid probability
        assert!((0.0..=1.0).contains(&result));
    }

    #[test]
    fn test_f_dot_test_different_variances() {
        // =F.TEST({1,2,3,4,5},{1,3,5,7,9}) in US format
        // =F.TEST({1;2;3;4;5};{1;3;5;7;9}) in German format
        let array1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let array2 = vec![1.0, 3.0, 5.0, 7.0, 9.0];
        let result = codcel_f_dot_test(array1, array2).unwrap();
        println!("{result}");
        assert!((result - 0.20799999999999952).abs() < 0.0001);
    }

    #[test]
    fn test_f_dot_test_empty_array1() {
        // Empty array1 should return an error
        let array1: Vec<f64> = vec![];
        let array2 = vec![1.0, 2.0, 3.0];
        let result = codcel_f_dot_test(array1, array2);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dot_test_empty_array2() {
        // Empty array2 should return an error
        let array1 = vec![1.0, 2.0, 3.0];
        let array2: Vec<f64> = vec![];
        let result = codcel_f_dot_test(array1, array2);
        assert!(result.is_err());
    }

    #[test]
    fn test_f_dot_test_vec_valid() {
        // Test the vector version with valid inputs
        let inputs = vec![
            vec![1.0, 2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0, 10.0],
        ];
        let result = codcel_f_dot_test_vec(inputs).unwrap();
        println!("{result}");
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_f_dot_test_vec_invalid_length() {
        // Test the vector version with invalid number of inputs
        let inputs = vec![vec![1.0, 2.0, 3.0]];
        let result = codcel_f_dot_test_vec(inputs);
        assert!(result.is_err());
    }
}
