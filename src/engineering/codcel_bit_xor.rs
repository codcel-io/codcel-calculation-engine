// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `BITXOR` that returns the bitwise XOR of two integers.
/// - `n1`: first non-negative integer.
/// - `n2`: second non-negative integer.
///   Returns the bitwise XOR result, or an error when either operand is negative.
pub fn codcel_bit_xor(n1: i32, n2: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if n1 < 0 || n2 < 0 {
        return Err("BITXOR: Both numbers must be non-negative".into());
    }

    Ok(n1 ^ n2)
}

/// Excel-compatible `BITXOR` that accepts its arguments as a vector.
/// - `inputs`: a vector containing exactly two non-negative integers.
///   Returns the bitwise XOR result, or an error when the argument count differs from two or values are negative.
pub fn codcel_bit_xor_vec(inputs: Vec<i32>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("BITXOR: Must have 2 parameters".into());
    }

    codcel_bit_xor(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_xor_basic() {
        // =BITXOR(5, 3) in US format
        // =BITXOR(5; 3) in German format
        let result = codcel_bit_xor(5, 3).unwrap();
        println!("{result}");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_bit_xor_with_zero() {
        // =BITXOR(10, 0) in US format
        // =BITXOR(10; 0) in German format
        let result = codcel_bit_xor(10, 0).unwrap();
        println!("{result}");
        assert_eq!(result, 10);
    }

    #[test]
    fn test_bit_xor_same_number() {
        // =BITXOR(7, 7) in US format
        // =BITXOR(7; 7) in German format
        let result = codcel_bit_xor(7, 7).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_bit_xor_large_numbers() {
        // =BITXOR(255, 128) in US format
        // =BITXOR(255; 128) in German format
        let result = codcel_bit_xor(255, 128).unwrap();
        println!("{result}");
        assert_eq!(result, 127);
    }

    #[test]
    fn test_bit_xor_negative_number() {
        // =BITXOR(-1, 5) in US format
        // =BITXOR(-1; 5) in German format
        let result = codcel_bit_xor(-1, 5);
        assert!(result.is_err());
    }
}
