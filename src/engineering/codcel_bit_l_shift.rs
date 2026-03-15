// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `BITLSHIFT` that shifts a number left by the specified number of bits.
/// - `n`: the number to shift (must be between 0 and 2^48-1).
/// - `shift`: the number of bits to shift left (must be between -53 and 53; negative values shift right).
///   Returns the shifted value, or an error when inputs are outside the allowed range or result overflows.
pub fn codcel_bit_l_shift(n: i32, shift: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    // Excel's BITLSHIFT has these constraints:
    // 1. Number must be >= 0 and <= 2^48 - 1
    // 2. Shift must be >= -53 and <= 53
    // 3. Result must be >= 0 and <= 2^48 - 1
    const MAX_NUMBER: i64 = (1i64 << 48) - 1;

    // Validate input number
    if n < 0 || n as i64 > MAX_NUMBER {
        return Err("BITLSHIFT: Number must be between 0 and 2^48 - 1".into());
    }

    // Validate shift amount
    if !(-53..=53).contains(&shift) {
        return Err("BITLSHIFT: Shift must be between -53 and 53".into());
    }

    // Handle negative shifts (right shift)
    let result = if shift < 0 {
        n >> -shift
    } else {
        // Check if shift would cause overflow
        let shifted = (n as i64) << shift;
        if shifted > MAX_NUMBER {
            return Err("BITLSHIFT: Result exceeds maximum allowed value (2^48 - 1)".into());
        }
        shifted as i32
    };

    Ok(result)
}

/// Excel-compatible `BITLSHIFT` that accepts its arguments as a vector.
/// - `inputs`: a vector containing `[number, shift]`.
///   Returns the shifted value, or an error when the argument count differs from two or validation fails.
pub fn codcel_bit_l_shift_vec(inputs: Vec<i32>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("BITLSHIFT: Must have 2 parameters".into());
    }

    codcel_bit_l_shift(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_l_shift_basic() {
        // =BITLSHIFT(4, 2) in US format
        // =BITLSHIFT(4; 2) in German format
        let result = codcel_bit_l_shift(4, 2).unwrap();
        println!("{result}");
        assert_eq!(result, 16);
    }

    #[test]
    fn test_bit_l_shift_zero() {
        // =BITLSHIFT(0, 5) in US format
        // =BITLSHIFT(0; 5) in German format
        let result = codcel_bit_l_shift(0, 5).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_bit_l_shift_zero_shift() {
        // =BITLSHIFT(7, 0) in US format
        // =BITLSHIFT(7; 0) in German format
        let result = codcel_bit_l_shift(7, 0).unwrap();
        println!("{result}");
        assert_eq!(result, 7);
    }

    #[test]
    fn test_bit_l_shift_negative_shift() {
        // =BITLSHIFT(16, -2) in US format
        // =BITLSHIFT(16; -2) in German format
        let result = codcel_bit_l_shift(16, -2).unwrap();
        println!("{result}");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_bit_l_shift_large_number() {
        // =BITLSHIFT(1000000, 3) in US format
        // =BITLSHIFT(1000000; 3) in German format
        let result = codcel_bit_l_shift(1000000, 3).unwrap();
        println!("{result}");
        assert_eq!(result, 8000000);
    }

    #[test]
    fn test_bit_l_shift_negative_number() {
        // =BITLSHIFT(-1, 5) in US format
        // =BITLSHIFT(-1; 5) in German format
        let result = codcel_bit_l_shift(-1, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_bit_l_shift_invalid_shift() {
        // =BITLSHIFT(4, 54) in US format
        // =BITLSHIFT(4; 54) in German format
        let result = codcel_bit_l_shift(4, 54);
        assert!(result.is_err());
    }

    #[test]
    fn test_bit_l_shift_vec_basic() {
        // =BITLSHIFT(4, 2) in US format
        // =BITLSHIFT(4; 2) in German format
        let result = codcel_bit_l_shift_vec(vec![4, 2]).unwrap();
        println!("{result}");
        assert_eq!(result, 16);
    }

    #[test]
    fn test_bit_l_shift_vec_invalid_length() {
        // =BITLSHIFT(4, 2, 1) in US format (invalid in Excel)
        // =BITLSHIFT(4; 2; 1) in German format (invalid in Excel)
        let result = codcel_bit_l_shift_vec(vec![4, 2, 1]);
        assert!(result.is_err());
    }
}
