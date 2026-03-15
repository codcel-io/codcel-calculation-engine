// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `DEC2OCT` that converts a decimal number to octal.
/// - `number`: the decimal integer to convert (must be between -536,870,912 and 536,870,911).
/// - `places`: optional number of characters for the result; pads with leading zeros.
///   Returns the octal string (30-bit two's complement for negatives), or an error when input is out of range or result exceeds places.
pub fn codcel_dec_2_oct(
    number: i32,
    places: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Excel's DEC2OCT constraints:
    // 1. Number must be between -536,870,912 and 536,870,911 inclusive
    const MIN_NUMBER: i64 = -(1i64 << 29);
    const MAX_NUMBER: i64 = (1i64 << 29) - 1;
    let number = number as i64;

    if !(MIN_NUMBER..=MAX_NUMBER).contains(&number) {
        return Err("DEC2OCT: Decimal number must be between -536,870,912 and 536,870,911".into());
    }

    // Convert to octal representation
    let oct = if number < 0 {
        // For negative numbers, Excel uses 30-bit two's complement representation
        format!("{:o}", (number + (1i64 << 30)) % (1i64 << 30))
    } else {
        format!("{number:o}")
    };

    // Apply zero-padding if 'places' is specified
    let result = if let Some(p) = places {
        if oct.len() > p as usize {
            return Err("DEC2OCT: Number exceeds specified places length".into());
        }
        format!("{:0>width$}", oct, width = p as usize)
    } else {
        oct
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dec_2_oct_positive() {
        // =DEC2OCT(10) in US format
        // =DEC2OCT(10) in German format
        let result = codcel_dec_2_oct(10, None).unwrap();
        println!("{result}");
        assert_eq!(result, "12");
    }

    #[test]
    fn test_dec_2_oct_zero() {
        // =DEC2OCT(0) in US format
        // =DEC2OCT(0) in German format
        let result = codcel_dec_2_oct(0, None).unwrap();
        println!("{result}");
        assert_eq!(result, "0");
    }

    #[test]
    fn test_dec_2_oct_negative() {
        // =DEC2OCT(-10) in US format
        // =DEC2OCT(-10) in German format
        let result = codcel_dec_2_oct(-10, None).unwrap();
        println!("{result}");
        assert_eq!(result, "7777777766");
    }

    #[test]
    fn test_dec_2_oct_with_places() {
        // =DEC2OCT(10, 4) in US format
        // =DEC2OCT(10; 4) in German format
        let result = codcel_dec_2_oct(10, Some(4)).unwrap();
        println!("{result}");
        assert_eq!(result, "0012");
    }

    #[test]
    fn test_dec_2_oct_large_number() {
        // =DEC2OCT(100000) in US format
        // =DEC2OCT(100000) in German format
        let result = codcel_dec_2_oct(100000, None).unwrap();
        println!("{result}");
        assert_eq!(result, "303240");
    }

    #[test]
    fn test_dec_2_oct_places_too_small() {
        // =DEC2OCT(100, 2) in US format
        // =DEC2OCT(100; 2) in German format
        let result = codcel_dec_2_oct(100, Some(2));
        assert!(result.is_err());
    }
}
