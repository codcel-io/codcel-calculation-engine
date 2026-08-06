// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `BITRSHIFT` that shifts a number right by the specified number of bits.
/// - `n`: the number to shift (must be between 0 and 2^48-1).
/// - `shift`: the number of bits to shift right (must be between -53 and 53; negative values shift left).
///   Returns the shifted value, or an error when inputs are outside the allowed range or result overflows.
pub fn codcel_bit_r_shift(n: i32, shift: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    // Excel's BITRSHIFT has these constraints:
    // 1. Number must be >= 0 and <= 2^48 - 1
    // 2. Shift must be >= -53 and <= 53
    // 3. Result must be >= 0 and <= 2^48 - 1
    const MAX_NUMBER: i64 = (1i64 << 48) - 1;

    // Validate input number
    if n < 0 || n as i64 > MAX_NUMBER {
        return Err("BITRSHIFT: Number must be between 0 and 2^48 - 1".into());
    }

    // Validate shift amount
    if !(-53..=53).contains(&shift) {
        return Err("BITRSHIFT: Shift must be between -53 and 53".into());
    }

    // Handle negative shifts (left shift)
    let result = if shift < 0 {
        // Check if negative shift would cause overflow
        let shifted = (n as i64) << -shift;
        if shifted > MAX_NUMBER {
            return Err("BITRSHIFT: Result exceeds maximum allowed value (2^48 - 1)".into());
        }
        shifted as i32
    } else {
        n >> shift
    };

    Ok(result)
}

/// Excel-compatible `BITRSHIFT` that accepts its arguments as a vector.
/// - `inputs`: a vector containing `[number, shift]`.
///   Returns the shifted value, or an error when the argument count differs from two or validation fails.
pub fn codcel_bit_r_shift_vec(inputs: Vec<i32>) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("BITRSHIFT: Must have 2 parameters".into());
    }

    codcel_bit_r_shift(inputs[0], inputs[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_r_shift_basic() {
        // =BITRSHIFT(16, 2) in US format
        // =BITRSHIFT(16; 2) in German format
        let result = codcel_bit_r_shift(16, 2).unwrap();
        println!("{result}");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_bit_r_shift_zero() {
        // =BITRSHIFT(0, 5) in US format
        // =BITRSHIFT(0; 5) in German format
        let result = codcel_bit_r_shift(0, 5).unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_bit_r_shift_zero_shift() {
        // =BITRSHIFT(7, 0) in US format
        // =BITRSHIFT(7; 0) in German format
        let result = codcel_bit_r_shift(7, 0).unwrap();
        println!("{result}");
        assert_eq!(result, 7);
    }

    #[test]
    fn test_bit_r_shift_negative_shift() {
        // =BITRSHIFT(4, -2) in US format
        // =BITRSHIFT(4; -2) in German format
        let result = codcel_bit_r_shift(4, -2).unwrap();
        println!("{result}");
        assert_eq!(result, 16);
    }

    #[test]
    fn test_bit_r_shift_large_number() {
        // =BITRSHIFT(8000000, 3) in US format
        // =BITRSHIFT(8000000; 3) in German format
        let result = codcel_bit_r_shift(8000000, 3).unwrap();
        println!("{result}");
        assert_eq!(result, 1000000);
    }

    #[test]
    fn test_bit_r_shift_negative_number() {
        // =BITRSHIFT(-1, 5) in US format
        // =BITRSHIFT(-1; 5) in German format
        let result = codcel_bit_r_shift(-1, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_bit_r_shift_invalid_shift() {
        // =BITRSHIFT(4, 54) in US format
        // =BITRSHIFT(4; 54) in German format
        let result = codcel_bit_r_shift(4, 54);
        assert!(result.is_err());
    }

    #[test]
    fn test_bit_r_shift_vec_basic() {
        // =BITRSHIFT(16, 2) in US format
        // =BITRSHIFT(16; 2) in German format
        let result = codcel_bit_r_shift_vec(vec![16, 2]).unwrap();
        println!("{result}");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_bit_r_shift_vec_invalid_length() {
        // =BITRSHIFT(16, 2, 1) in US format (invalid in Excel)
        // =BITRSHIFT(16; 2; 1) in German format (invalid in Excel)
        let result = codcel_bit_r_shift_vec(vec![16, 2, 1]);
        assert!(result.is_err());
    }
}
