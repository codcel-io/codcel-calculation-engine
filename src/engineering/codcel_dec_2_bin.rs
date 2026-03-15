// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `DEC2BIN` that converts a decimal number to binary.
/// - `number`: the decimal integer to convert (must be between -512 and 511).
/// - `places`: optional number of characters for the result; pads with leading zeros.
///   Returns the binary string (10-bit two's complement for negatives), or an error when input is out of range or result exceeds places.
pub fn codcel_dec_2_bin(
    number: i32,
    places: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Excel's DEC2BIN constraints:
    // 1. Number must be between -512 and 511 inclusive
    if !(-512..=511).contains(&number) {
        return Err("DEC2BIN: Decimal number must be between -512 and 511".into());
    }

    // Convert to binary representation
    let bin = if number < 0 {
        // For negative numbers, Excel uses 10-bit two's complement representation
        format!("{:b}", (number + 1024) % 1024)
    } else {
        format!("{number:b}")
    };

    // Apply zero-padding if 'places' is specified
    let result = if let Some(p) = places {
        if bin.len() > p as usize {
            return Err("DEC2BIN: Number exceeds specified places length".into());
        }
        format!("{:0>width$}", bin, width = p as usize)
    } else {
        bin
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dec_2_bin_positive() {
        // =DEC2BIN(10) in US format
        // =DEC2BIN(10) in German format
        let result = codcel_dec_2_bin(10, None).unwrap();
        println!("{result}");
        assert_eq!(result, "1010");
    }

    #[test]
    fn test_dec_2_bin_zero() {
        // =DEC2BIN(0) in US format
        // =DEC2BIN(0) in German format
        let result = codcel_dec_2_bin(0, None).unwrap();
        println!("{result}");
        assert_eq!(result, "0");
    }

    #[test]
    fn test_dec_2_bin_negative() {
        // =DEC2BIN(-10) in US format
        // =DEC2BIN(-10) in German format
        let result = codcel_dec_2_bin(-10, None).unwrap();
        println!("{result}");
        assert_eq!(result, "1111110110");
    }

    #[test]
    fn test_dec_2_bin_with_places() {
        // =DEC2BIN(10, 8) in US format
        // =DEC2BIN(10; 8) in German format
        let result = codcel_dec_2_bin(10, Some(8)).unwrap();
        println!("{result}");
        assert_eq!(result, "00001010");
    }

    #[test]
    fn test_dec_2_bin_max_value() {
        // =DEC2BIN(511) in US format
        // =DEC2BIN(511) in German format
        let result = codcel_dec_2_bin(511, None).unwrap();
        println!("{result}");
        assert_eq!(result, "111111111");
    }

    #[test]
    fn test_dec_2_bin_min_value() {
        // =DEC2BIN(-512) in US format
        // =DEC2BIN(-512) in German format
        let result = codcel_dec_2_bin(-512, None).unwrap();
        println!("{result}");
        assert_eq!(result, "1000000000");
    }

    #[test]
    fn test_dec_2_bin_out_of_range() {
        // =DEC2BIN(512) in US format
        // =DEC2BIN(512) in German format
        let result = codcel_dec_2_bin(512, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_dec_2_bin_places_too_small() {
        // =DEC2BIN(10, 2) in US format
        // =DEC2BIN(10; 2) in German format
        let result = codcel_dec_2_bin(10, Some(2));
        assert!(result.is_err());
    }
}
