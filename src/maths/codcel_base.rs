// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `BASE` that converts a number to a text representation in another base.
/// - `number`: the number to convert (non-negative integer).
/// - `radix`: the base to convert to (2–36).
/// - `min_length`: optional minimum length for the result, padded with leading zeros.
///
/// Returns the text representation or an error when radix is outside 2–36.
pub fn codcel_base(
    number: i32,
    radix: i32,
    min_length: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let number = number as u32;
    let radix = radix as u32;

    if !(2..=36).contains(&radix) {
        return Err(format!("BASE: Radix must be between 2 and 36, got {radix}").into());
    }

    // Convert the number to the specified base
    let mut result = String::new();
    let mut num = number;

    if num == 0 {
        result.push('0');
    } else {
        while num > 0 {
            let digit = (num % radix) as u8;
            result.push(match digit {
                0..=9 => (b'0' + digit) as char,
                10..=35 => (b'A' + digit - 10) as char,
                _ => unreachable!(),
            });
            num /= radix;
        }
    }

    // Reverse the string to get the correct representation
    result = result.chars().rev().collect();

    // Pad with leading zeros if `min_length` is specified
    if let Some(min_len) = min_length {
        let min_len = min_len as usize;
        if result.len() < min_len {
            result = "0".repeat(min_len - result.len()) + &result;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_decimal_to_binary() {
        // =BASE(10, 2) in US format
        // =BASE(10; 2) in German format
        let result = codcel_base(10, 2, None).unwrap();
        assert_eq!(result, "1010");
    }

    #[test]
    fn test_base_decimal_to_octal() {
        // =BASE(100, 8) in US format
        // =BASE(100; 8) in German format
        let result = codcel_base(100, 8, None).unwrap();
        assert_eq!(result, "144");
    }

    #[test]
    fn test_base_decimal_to_hexadecimal() {
        // =BASE(255, 16) in US format
        // =BASE(255; 16) in German format
        let result = codcel_base(255, 16, None).unwrap();
        assert_eq!(result, "FF");
    }

    #[test]
    fn test_base_decimal_to_base36() {
        // =BASE(1234, 36) in US format
        // =BASE(1234; 36) in German format
        let result = codcel_base(1234, 36, None).unwrap();
        assert_eq!(result, "YA");
    }

    #[test]
    fn test_base_zero() {
        // =BASE(0, 2) in US format
        // =BASE(0; 2) in German format
        let result = codcel_base(0, 2, None).unwrap();
        assert_eq!(result, "0");
    }

    #[test]
    fn test_base_with_min_length() {
        // =BASE(10, 2, 8) in US format
        // =BASE(10; 2; 8) in German format
        let result = codcel_base(10, 2, Some(8)).unwrap();
        assert_eq!(result, "00001010");
    }

    #[test]
    fn test_base_with_invalid_radix() {
        // =BASE(10, 1) in US format - should return an error
        // =BASE(10; 1) in German format - should return an error
        let result = codcel_base(10, 1, None);
        assert!(result.is_err());

        // =BASE(10, 37) in US format - should return an error
        // =BASE(10; 37) in German format - should return an error
        let result = codcel_base(10, 37, None);
        assert!(result.is_err());
    }
}
