// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `BIN2DEC` that converts a binary number to decimal.
/// - `binary`: a string of up to 10 binary digits (0s and 1s); 10-digit values starting with `1` use two's complement for negatives.
///   Returns the decimal integer value, or an error when the input is empty or contains invalid characters.
pub fn codcel_bin_2_dec<S: AsRef<str>>(binary: S) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let binary = binary.as_ref();
    // Validate the input binary string
    if binary.is_empty() {
        return Err("BIN2DEC: Input binary string is empty".into());
    }

    // Handle negative binary numbers (two's complement notation with up to 10 bits)
    let is_negative = binary.starts_with('1') && binary.len() == 10;
    let unsigned_value = i32::from_str_radix(binary, 2)
        .map_err(|_| "BIN2DEC: Invalid binary string, must only contain 0s and 1s")?;

    if is_negative {
        // For a 10-bit two's complement binary, subtract 2^10 to obtain the signed value
        Ok(unsigned_value - (1 << 10))
    } else {
        Ok(unsigned_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_2_dec_basic() {
        // =BIN2DEC("1010") in US format
        // =BIN2DEC("1010") in German format
        let result = codcel_bin_2_dec("1010").unwrap();
        println!("{result}");
        assert_eq!(result, 10);
    }

    #[test]
    fn test_bin_2_dec_zero() {
        // =BIN2DEC("0") in US format
        // =BIN2DEC("0") in German format
        let result = codcel_bin_2_dec("0").unwrap();
        println!("{result}");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_bin_2_dec_one() {
        // =BIN2DEC("1") in US format
        // =BIN2DEC("1") in German format
        let result = codcel_bin_2_dec("1").unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_bin_2_dec_negative() {
        // =BIN2DEC("1111111111") in US format
        // =BIN2DEC("1111111111") in German format
        let result = codcel_bin_2_dec("1111111111").unwrap();
        println!("{result}");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_bin_2_dec_negative_value() {
        // =BIN2DEC("1111111110") in US format
        // =BIN2DEC("1111111110") in German format
        let result = codcel_bin_2_dec("1111111110").unwrap();
        println!("{result}");
        assert_eq!(result, -2);
    }

    #[test]
    fn test_bin_2_dec_invalid_chars() {
        // =BIN2DEC("10102") in US format
        // =BIN2DEC("10102") in German format
        let result = codcel_bin_2_dec("10102");
        assert!(result.is_err());
    }

    #[test]
    fn test_bin_2_dec_empty_string() {
        // =BIN2DEC("") in US format
        // =BIN2DEC("") in German format
        let result = codcel_bin_2_dec("");
        assert!(result.is_err());
    }
}
