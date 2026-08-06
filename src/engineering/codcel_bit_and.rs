// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `BITAND` that returns the bitwise AND of two integers.
/// - `n1`: first non-negative integer.
/// - `n2`: second non-negative integer.
///   Returns the bitwise AND result, or an error when either operand is negative.
pub fn codcel_bit_and(n1: i32, n2: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if n1 < 0 || n2 < 0 {
        return Err("BITAND: Both numbers must be non-negative".into());
    }

    Ok(n1 & n2)
}

/// Excel-compatible `BITAND` that accepts its arguments as a vector.
/// - `inputs`: a vector containing exactly two non-negative integers.
///   Returns the bitwise AND result, or an error when the argument count differs from two or values are negative.
pub fn codcel_bit_and_vec(inputs: Vec<i32>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("BITAND: Must have 2 parameters".into());
    }

    codcel_bit_and(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_and_basic() {
        // =BITAND(13, 25) in US format
        // =BITAND(13; 25) in German format
        let result = codcel_bit_and(13, 25).unwrap();
        println!("{result}");
        assert_eq!(result, 9);
    }

    #[test]
    fn test_bit_and_zero() {
        // =BITAND(0, 5) in US format
        // =BITAND(0; 5) in German format
        let result = codcel_bit_and(0, 5).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_bit_and_same_number() {
        // =BITAND(7, 7) in US format
        // =BITAND(7; 7) in German format
        let result = codcel_bit_and(7, 7).unwrap();
        println!("{result}");
        assert_eq!(result, 7);
    }

    #[test]
    fn test_bit_and_large_numbers() {
        // =BITAND(1234567, 9876543) in US format
        // =BITAND(1234567; 9876543) in German format
        let result = codcel_bit_and(1234567, 9876543).unwrap();
        println!("{result}");
        assert_eq!(result, 1234567 & 9876543);
    }

    #[test]
    fn test_bit_and_negative_number() {
        // =BITAND(-1, 5) in US format
        // =BITAND(-1; 5) in German format
        let result = codcel_bit_and(-1, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_bit_and_vec_basic() {
        // =BITAND(13, 25) in US format
        // =BITAND(13; 25) in German format
        let result = codcel_bit_and_vec(vec![13, 25]).unwrap();
        println!("{result}");
        assert_eq!(result, 9);
    }

    #[test]
    fn test_bit_and_vec_invalid_length() {
        // =BITAND(13, 25, 7) in US format (invalid in Excel)
        // =BITAND(13; 25; 7) in German format (invalid in Excel)
        let result = codcel_bit_and_vec(vec![13, 25, 7]);
        assert!(result.is_err());
    }
}
