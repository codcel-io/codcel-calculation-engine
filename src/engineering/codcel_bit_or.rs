// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `BITOR` that returns the bitwise OR of two integers.
/// - `n1`: first non-negative integer.
/// - `n2`: second non-negative integer.
///   Returns the bitwise OR result, or an error when either operand is negative.
pub fn codcel_bit_or(n1: i32, n2: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if n1 < 0 || n2 < 0 {
        return Err("BITOR: Both numbers must be non-negative".into());
    }

    Ok(n1 | n2)
}

/// Excel-compatible `BITOR` that accepts its arguments as a vector.
/// - `inputs`: a vector containing exactly two non-negative integers.
///   Returns the bitwise OR result, or an error when the argument count differs from two or values are negative.
pub fn codcel_bit_or_vec(inputs: Vec<i32>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("BITOR: Must have 2 parameters".into());
    }

    codcel_bit_or(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_or_basic() {
        // =BITOR(9, 10) in US format
        // =BITOR(9; 10) in German format
        let result = codcel_bit_or(9, 10).unwrap();
        println!("{result}");
        assert_eq!(result, 11);
    }

    #[test]
    fn test_bit_or_zero() {
        // =BITOR(0, 5) in US format
        // =BITOR(0; 5) in German format
        let result = codcel_bit_or(0, 5).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_bit_or_same_number() {
        // =BITOR(7, 7) in US format
        // =BITOR(7; 7) in German format
        let result = codcel_bit_or(7, 7).unwrap();
        println!("{result}");
        assert_eq!(result, 7);
    }

    #[test]
    fn test_bit_or_large_numbers() {
        // =BITOR(1234567, 9876543) in US format
        // =BITOR(1234567; 9876543) in German format
        let result = codcel_bit_or(1234567, 9876543).unwrap();
        println!("{result}");
        assert_eq!(result, 1234567 | 9876543);
    }

    #[test]
    fn test_bit_or_negative_number() {
        // =BITOR(-1, 5) in US format
        // =BITOR(-1; 5) in German format
        let result = codcel_bit_or(-1, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_bit_or_vec_basic() {
        // =BITOR(9, 10) in US format
        // =BITOR(9; 10) in German format
        let result = codcel_bit_or_vec(vec![9, 10]).unwrap();
        println!("{result}");
        assert_eq!(result, 11);
    }

    #[test]
    fn test_bit_or_vec_invalid_length() {
        // =BITOR(9, 10, 7) in US format (invalid in Excel)
        // =BITOR(9; 10; 7) in German format (invalid in Excel)
        let result = codcel_bit_or_vec(vec![9, 10, 7]);
        assert!(result.is_err());
    }
}
