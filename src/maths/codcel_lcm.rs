// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::maths::codcel_gcd::gcd;
use std::error::Error;

/// Excel-compatible `LCM` that returns the least common multiple of one or more numbers.
/// - `numbers`: a list of numbers to find the LCM for (decimals are truncated).
///
/// Returns the least common multiple or an error for empty list, zeros, or overflow.
pub fn codcel_lcm(numbers: Vec<f64>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    // Handle empty input
    if numbers.is_empty() {
        return Err("LCM: Cannot calculate LCM of an empty list".into());
    }

    // Convert f64 to i32 (truncate decimal parts as Excel does)
    let mut integers = Vec::with_capacity(numbers.len());
    for &num in &numbers {
        // Handle values that can't be converted to i32
        if !num.is_finite() {
            return Err("LCM: Cannot calculate LCM with non-finite values".into());
        }

        if num.abs() > i32::MAX as f64 {
            return Err("LCM: Input value too large for i32 representation".into());
        }

        // Truncate to integer as Excel does
        let int_val = num.trunc() as i32;

        // Check for zero
        if int_val == 0 {
            return Err("LCM: Cannot calculate LCM when a number is zero".into());
        }

        integers.push(int_val);
    }

    // Take absolute values since LCM works with positive numbers
    let integers: Vec<i32> = integers.iter().map(|&n| n.abs()).collect();

    // Calculate the LCM
    let mut result = 1;
    for &num in &integers {
        result = lcm(result, num);

        // Check for potential overflow
        if result <= 0 {
            return Err("LCM: Calculation resulted in integer overflow".into());
        }
    }

    Ok(result)
}

/// Calculates the Least Common Multiple (LCM) of two numbers
fn lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        0
    } else {
        let gcd_value = gcd(a, b);
        // Use checked multiplication to handle potential overflow
        (a / gcd_value).checked_mul(b).unwrap_or(i32::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcm_single_number() {
        // =LCM(12) in US format
        // =LCM(12) in German format
        let result = codcel_lcm(vec![12.0]).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_lcm_two_numbers() {
        // =LCM(12, 18) in US format
        // =LCM(12; 18) in German format
        let result = codcel_lcm(vec![12.0, 18.0]).unwrap();
        assert_eq!(result, 36);
    }

    #[test]
    fn test_lcm_multiple_numbers() {
        // =LCM(12, 18, 24) in US format
        // =LCM(12; 18; 24) in German format
        let result = codcel_lcm(vec![12.0, 18.0, 24.0]).unwrap();
        assert_eq!(result, 72);
    }

    #[test]
    fn test_lcm_coprime_numbers() {
        // =LCM(7, 13) in US format
        // =LCM(7; 13) in German format
        let result = codcel_lcm(vec![7.0, 13.0]).unwrap();
        assert_eq!(result, 91);
    }

    #[test]
    fn test_lcm_decimal_numbers() {
        // =LCM(12.7, 18.3) in US format
        // =LCM(12,7; 18,3) in German format
        let result = codcel_lcm(vec![12.7, 18.3]).unwrap();
        assert_eq!(result, 36); // Decimals are truncated to 12 and 18
    }

    #[test]
    fn test_lcm_negative_numbers() {
        // =LCM(-12, 18, -24) in US format
        // =LCM(-12; 18; -24) in German format
        let result = codcel_lcm(vec![-12.0, 18.0, -24.0]).unwrap();
        assert_eq!(result, 72); // LCM uses absolute values
    }

    #[test]
    fn test_lcm_with_zero() {
        // =LCM(0, 12, 18) in US format - should return an error
        // =LCM(0; 12; 18) in German format - should return an error
        let result = codcel_lcm(vec![0.0, 12.0, 18.0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_lcm_empty_vector() {
        // Empty vector should return an error
        let result = codcel_lcm(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_lcm_large_numbers() {
        // =LCM(1071, 462) in US format
        // =LCM(1071; 462) in German format
        let result = codcel_lcm(vec![1071.0, 462.0]).unwrap();
        assert_eq!(result, 23562);
    }
}
