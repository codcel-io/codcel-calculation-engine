// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `GCD` that returns the greatest common divisor of one or more integers.
/// - `numbers`: a list of integers to find the GCD for.
///
/// Returns the greatest common divisor or an error when the list is empty or all zeros.
pub fn codcel_gcd(numbers: Vec<i32>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    // Check if the vector is empty
    if numbers.is_empty() {
        return Err("GCD: Requires at least one number".into());
    }

    // Excel's GCD ignores zeros
    let integers: Vec<i32> = numbers
        .into_iter()
        .filter_map(|n| {
            // Skip zeros, take absolute value as GCD is always positive
            if n == 0 {
                None
            } else {
                Some(n.abs())
            }
        })
        .collect();

    // If no valid integers, Excel returns #NUM! error
    if integers.is_empty() {
        return Err("GCD: No valid integers found for GCD calculation".into());
    }

    // Start with the first number
    let mut result = integers[0];

    // Calculate GCD of all numbers
    for &num in &integers[1..] {
        result = gcd(result, num);
    }

    Ok(result)
}

pub(crate) fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_single_number() {
        // =GCD(12) in US format
        // =GCD(12) in German format
        let result = codcel_gcd(vec![12]).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn test_gcd_two_numbers() {
        // =GCD(12, 18) in US format
        // =GCD(12; 18) in German format
        let result = codcel_gcd(vec![12, 18]).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_gcd_multiple_numbers() {
        // =GCD(12, 18, 24) in US format
        // =GCD(12; 18; 24) in German format
        let result = codcel_gcd(vec![12, 18, 24]).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_gcd_coprime_numbers() {
        // =GCD(7, 13) in US format
        // =GCD(7; 13) in German format
        let result = codcel_gcd(vec![7, 13]).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_gcd_with_zeros() {
        // =GCD(0, 12, 18) in US format
        // =GCD(0; 12; 18) in German format
        let result = codcel_gcd(vec![0, 12, 18]).unwrap();
        assert_eq!(result, 6); // Zeros are ignored
    }

    #[test]
    fn test_gcd_all_zeros() {
        // =GCD(0, 0, 0) in US format - should return an error
        // =GCD(0; 0; 0) in German format - should return an error
        let result = codcel_gcd(vec![0, 0, 0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_gcd_negative_numbers() {
        // =GCD(-12, 18, -24) in US format
        // =GCD(-12; 18; -24) in German format
        let result = codcel_gcd(vec![-12, 18, -24]).unwrap();
        assert_eq!(result, 6); // GCD uses absolute values
    }

    #[test]
    fn test_gcd_large_numbers() {
        // =GCD(1071, 462) in US format
        // =GCD(1071; 462) in German format
        let result = codcel_gcd(vec![1071, 462]).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_gcd_empty_vector() {
        // Empty vector should return an error
        let result = codcel_gcd(vec![]);
        assert!(result.is_err());
    }
}
