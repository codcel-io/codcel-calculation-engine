// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use std::error::Error;

/// Excel-compatible `SUMSQ` that returns the sum of squares of the arguments.
/// - `values`: a list of numbers to square and sum.
///
/// Returns the sum of squared values (0 if empty).
pub fn codcel_sum_sq(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(0.0);
    }

    let sum_sq = values.iter().map(|&x| x * x).compensated_sum();
    Ok(sum_sq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_sq_positive_numbers() {
        // =SUMSQ(1,2,3,4,5) in US format
        // =SUMSQ(1;2;3;4;5) in German format
        let result = codcel_sum_sq(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 55.0); // 1^2 + 2^2 + 3^2 + 4^2 + 5^2 = 1 + 4 + 9 + 16 + 25 = 55
    }

    #[test]
    fn test_sum_sq_negative_numbers() {
        // =SUMSQ(-1,-2,-3,-4,-5) in US format
        // =SUMSQ(-1;-2;-3;-4;-5) in German format
        let result = codcel_sum_sq(vec![-1.0, -2.0, -3.0, -4.0, -5.0]).unwrap();
        assert_eq!(result, 55.0); // (-1)^2 + (-2)^2 + (-3)^2 + (-4)^2 + (-5)^2 = 1 + 4 + 9 + 16 + 25 = 55
    }

    #[test]
    fn test_sum_sq_mixed_numbers() {
        // =SUMSQ(1,-2,3,-4,5) in US format
        // =SUMSQ(1;-2;3;-4;5) in German format
        let result = codcel_sum_sq(vec![1.0, -2.0, 3.0, -4.0, 5.0]).unwrap();
        assert_eq!(result, 55.0); // 1^2 + (-2)^2 + 3^2 + (-4)^2 + 5^2 = 1 + 4 + 9 + 16 + 25 = 55
    }

    #[test]
    fn test_sum_sq_decimals() {
        // =SUMSQ(1.5,2.5) in US format
        // =SUMSQ(1,5;2,5) in German format
        let result = codcel_sum_sq(vec![1.5, 2.5]).unwrap();
        assert!((result - 8.5).abs() < 1e-10); // 1.5^2 + 2.5^2 = 2.25 + 6.25 = 8.5
    }

    #[test]
    fn test_sum_sq_single_number() {
        // =SUMSQ(5) in US format
        // =SUMSQ(5) in German format
        let result = codcel_sum_sq(vec![5.0]).unwrap();
        assert_eq!(result, 25.0); // 5^2 = 25
    }

    #[test]
    fn test_sum_sq_empty() {
        // =SUMSQ() in US format
        // =SUMSQ() in German format
        let result = codcel_sum_sq(vec![]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_sum_sq_large_numbers() {
        // =SUMSQ(100,200) in US format
        // =SUMSQ(100;200) in German format
        let result = codcel_sum_sq(vec![100.0, 200.0]).unwrap();
        assert_eq!(result, 50000.0); // 100^2 + 200^2 = 10000 + 40000 = 50000
    }

    #[test]
    fn test_sum_sq_small_decimals() {
        // =SUMSQ(0.1,0.2,0.3) in US format
        // =SUMSQ(0,1;0,2;0,3) in German format
        let result = codcel_sum_sq(vec![0.1, 0.2, 0.3]).unwrap();
        assert!((result - 0.14).abs() < 1e-10); // 0.1^2 + 0.2^2 + 0.3^2 = 0.01 + 0.04 + 0.09 = 0.14
    }
}
