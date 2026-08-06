// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `DEC2HEX` that converts a decimal number to hexadecimal.
/// - `number`: the decimal integer to convert (must be between -2^39 and 2^39-1).
/// - `places`: optional number of characters for the result; pads with leading zeros.
///   Returns the hexadecimal string (40-bit two's complement for negatives), or an error when input is out of range or result exceeds places.
pub fn codcel_dec_2_hex(
    number: i64,
    places: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Excel's DEC2HEX constraints:
    // 1. Number must be between -2^39 and 2^39-1 inclusive
    const MIN_NUMBER: i64 = -(1i64 << 39);
    const MAX_NUMBER: i64 = (1i64 << 39) - 1;

    if !(MIN_NUMBER..=MAX_NUMBER).contains(&number) {
        return Err("DEC2HEX: Decimal number must be between -2^39 and 2^39-1".into());
    }

    // Convert to hexadecimal representation
    let hex = if number < 0 {
        // For negative numbers, Excel uses 40-bit two's complement representation
        format!("{:X}", (number + (1i64 << 40)) % (1i64 << 40))
    } else {
        format!("{number:X}")
    };

    // Apply zero-padding if 'places' is specified
    let result = if let Some(p) = places {
        if hex.len() > p as usize {
            return Err("DEC2HEX: Number exceeds specified places length".into());
        }
        format!("{:0>width$}", hex, width = p as usize)
    } else {
        hex
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dec_2_hex_positive() {
        // =DEC2HEX(255) in US format
        // =DEC2HEX(255) in German format
        let result = codcel_dec_2_hex(255, None).unwrap();
        println!("{result}");
        assert_eq!(result, "FF");
    }

    #[test]
    fn test_dec_2_hex_zero() {
        // =DEC2HEX(0) in US format
        // =DEC2HEX(0) in German format
        let result = codcel_dec_2_hex(0, None).unwrap();
        println!("{result}");
        assert_eq!(result, "0");
    }

    #[test]
    fn test_dec_2_hex_negative() {
        // =DEC2HEX(-10) in US format
        // =DEC2HEX(-10) in German format
        let result = codcel_dec_2_hex(-10, None).unwrap();
        println!("{result}");
        assert_eq!(result, "FFFFFFFFF6");
    }

    #[test]
    fn test_dec_2_hex_with_places() {
        // =DEC2HEX(255, 4) in US format
        // =DEC2HEX(255; 4) in German format
        let result = codcel_dec_2_hex(255, Some(4)).unwrap();
        println!("{result}");
        assert_eq!(result, "00FF");
    }

    #[test]
    fn test_dec_2_hex_large_number() {
        // =DEC2HEX(65535) in US format
        // =DEC2HEX(65535) in German format
        let result = codcel_dec_2_hex(65535, None).unwrap();
        println!("{result}");
        assert_eq!(result, "FFFF");
    }

    #[test]
    fn test_dec_2_hex_places_too_small() {
        // =DEC2HEX(255, 1) in US format
        // =DEC2HEX(255; 1) in German format
        let result = codcel_dec_2_hex(255, Some(1));
        assert!(result.is_err());
    }
}
