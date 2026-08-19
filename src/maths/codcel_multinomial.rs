// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSum;
use std::error::Error;

/// Excel-compatible `MULTINOMIAL` that returns the multinomial of a set of numbers.
/// - `numbers`: a list of non-negative integers.
///
/// Returns (sum of numbers)! / (product of factorials) or an error for invalid inputs.
pub fn codcel_multinomial(numbers: Vec<i32>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    // Check if the input array is empty
    if numbers.is_empty() {
        return Err("MULTINOMIAL: Input array cannot be empty".into());
    }

    // Check for negative numbers
    if numbers.iter().any(|&n| n < 0) {
        return Err("MULTINOMIAL: Negative numbers are not allowed".into());
    }

    // Calculate the sum of all numbers
    let sum: i32 = numbers.iter().sum();

    // Use a floating-point approach to avoid integer overflow
    // Log of factorial: log(n!) = log(1) + log(2) + ... + log(n)
    let mut log_result = CompensatedSum::new();

    // Add log of numerator (sum!)
    for i in 1..=sum {
        log_result.add(crate::portable_math::ln(i as f64));
    }

    // Subtract log of denominator (n1! * n2! * ... * nk!)
    for &n in &numbers {
        for i in 1..=n {
            log_result.add(-crate::portable_math::ln(i as f64));
        }
    }

    // Convert back from logarithm
    let result = crate::portable_math::exp(log_result.total()).round() as i64;

    // Check if the result fits in i32
    if result > i32::MAX as i64 {
        return Err("MULTINOMIAL: Result exceeds i32 range".into());
    }

    Ok(result as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multinomial_single_number() {
        // =MULTINOMIAL(5) in US format
        // =MULTINOMIAL(5) in German format
        let result = codcel_multinomial(vec![5]).unwrap();
        assert_eq!(result, 1); // Multinomial of a single number is always 1
    }

    #[test]
    fn test_multinomial_two_numbers() {
        // =MULTINOMIAL(2, 3) in US format
        // =MULTINOMIAL(2; 3) in German format
        let result = codcel_multinomial(vec![2, 3]).unwrap();
        assert_eq!(result, 10); // (2+3)! / (2! * 3!) = 5! / (2! * 3!) = 10
    }

    #[test]
    fn test_multinomial_three_numbers() {
        // =MULTINOMIAL(2, 3, 4) in US format
        // =MULTINOMIAL(2; 3; 4) in German format
        let result = codcel_multinomial(vec![2, 3, 4]).unwrap();
        assert_eq!(result, 1260); // (2+3+4)! / (2! * 3! * 4!) = 9! / (2! * 3! * 4!) = 1260
    }

    #[test]
    fn test_multinomial_with_zeros() {
        // =MULTINOMIAL(2, 0, 3) in US format
        // =MULTINOMIAL(2; 0; 3) in German format
        let result = codcel_multinomial(vec![2, 0, 3]).unwrap();
        assert_eq!(result, 10); // (2+0+3)! / (2! * 0! * 3!) = 5! / (2! * 1 * 3!) = 10
    }

    #[test]
    fn test_multinomial_all_ones() {
        // =MULTINOMIAL(1, 1, 1, 1) in US format
        // =MULTINOMIAL(1; 1; 1; 1) in German format
        let result = codcel_multinomial(vec![1, 1, 1, 1]).unwrap();
        assert_eq!(result, 24); // (1+1+1+1)! / (1! * 1! * 1! * 1!) = 4! / (1 * 1 * 1 * 1) = 24
    }

    #[test]
    fn test_multinomial_empty_array() {
        // Empty array should return an error
        let result = codcel_multinomial(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_multinomial_negative_number() {
        // =MULTINOMIAL(2, -3, 4) in US format - should return an error
        // =MULTINOMIAL(2; -3; 4) in German format - should return an error
        let result = codcel_multinomial(vec![2, -3, 4]);
        assert!(result.is_err());
    }

    #[test]
    fn test_multinomial_larger_numbers() {
        // =MULTINOMIAL(5, 5, 5) in US format
        // =MULTINOMIAL(5; 5; 5) in German format
        let result = codcel_multinomial(vec![5, 5, 5]).unwrap();
        assert_eq!(result, 756756); // (5+5+5)! / (5! * 5! * 5!) = 15! / (5! * 5! * 5!) = 756756
    }
}
