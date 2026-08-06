// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `OCT2BIN` that converts an octal number to binary.
/// - `octal`: the octal string to convert (up to 10 digits; uses 30-bit two's complement).
/// - `places`: optional number of characters for the result (1–10); pads with leading zeros.
///   Returns the binary string (10-bit two's complement for negatives), or an error.
pub fn codcel_oct_2_bin(
    octal: String,
    places: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let octal = octal.trim();

    if octal.is_empty() || !octal.chars().all(|c| c.is_digit(8)) {
        return Err("OCT2BIN: Invalid octal number format".into());
    }

    // Parse as unsigned
    let value =
        u64::from_str_radix(octal, 8).map_err(|_| "OCT2BIN: Failed to parse octal number")?;

    // Apply 30-bit two's complement to get signed value
    const TWO_POW_29: u64 = 1u64 << 29;
    const TWO_POW_30: u64 = 1u64 << 30;
    let signed_value = if value >= TWO_POW_29 {
        value as i64 - TWO_POW_30 as i64
    } else {
        value as i64
    };

    // Check range for binary output: must be -512 to 511
    if !(-512..=511).contains(&signed_value) {
        return Err("OCT2BIN: Number out of range for binary conversion".into());
    }

    // Convert to 10-bit two's complement binary
    let binary = if signed_value < 0 {
        format!("{:010b}", (signed_value + 1024) as u64)
    } else {
        format!("{:b}", signed_value)
    };

    // Handle optional digit padding
    if let Some(digits) = places {
        if digits <= 0 || digits > 10 {
            return Err("OCT2BIN: Places must be between 1 and 10".into());
        }
        let digits = digits as usize;
        let padded = format!("{binary:0>digits$}");
        return Ok(padded);
    }

    Ok(binary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oct_2_bin_simple() {
        assert_eq!(codcel_oct_2_bin("10".to_string(), None).unwrap(), "1000");
    }

    #[test]
    fn test_oct_2_bin_neg_512() {
        // 7777777000 in 30-bit two's complement = -512
        // -512 in 10-bit two's complement = 1000000000
        assert_eq!(
            codcel_oct_2_bin("7777777000".to_string(), None).unwrap(),
            "1000000000"
        );
    }

    #[test]
    fn test_oct_2_bin_with_places() {
        assert_eq!(
            codcel_oct_2_bin("10".to_string(), Some(8)).unwrap(),
            "00001000"
        );
    }

    #[test]
    fn test_oct_2_bin_invalid() {
        assert!(codcel_oct_2_bin("9".to_string(), None).is_err());
    }
}
