// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `HEX2BIN` that converts a hexadecimal number to binary.
/// - `hex`: the hexadecimal string to convert (up to 10 characters; uses 40-bit two's complement).
///   The resulting decimal value must be between -512 and 511.
/// - `places`: optional number of characters for the result; pads with leading zeros.
///   Returns the binary string (10-bit two's complement for negatives), or an error.
pub fn codcel_hex_2_bin(
    hex: String,
    places: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let hex_trimmed = hex.trim().trim_start_matches("0x").trim_start_matches("0X");

    if hex_trimmed.is_empty() || hex_trimmed.len() > 10 {
        return Err("HEX2BIN: Invalid hexadecimal input".into());
    }

    // Parse as unsigned 64-bit
    let value =
        u64::from_str_radix(hex_trimmed, 16).map_err(|_| "HEX2BIN: Invalid hexadecimal input")?;

    // Apply 40-bit two's complement to get signed value
    const TWO_POW_39: u64 = 1u64 << 39;
    const TWO_POW_40: u64 = 1u64 << 40;
    let signed_value = if value >= TWO_POW_39 {
        value as i64 - TWO_POW_40 as i64
    } else {
        value as i64
    };

    // Check range for binary output: must be -512 to 511
    if !(-512..=511).contains(&signed_value) {
        return Err("HEX2BIN: Number must be between -512 and 511 decimal".into());
    }

    // Convert to 10-bit two's complement binary
    let bin = if signed_value < 0 {
        format!("{:010b}", (signed_value + 1024) as u64)
    } else {
        format!("{:b}", signed_value)
    };

    // Apply zero-padding if 'places' is specified
    if let Some(p) = places {
        if bin.len() > p as usize {
            return Err("HEX2BIN: Number exceeds specified places length".into());
        }
        Ok(format!("{:0>width$}", bin, width = p as usize))
    } else {
        Ok(bin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_2_bin_positive() {
        assert_eq!(codcel_hex_2_bin("A".to_string(), None).unwrap(), "1010");
    }

    #[test]
    fn test_hex_2_bin_min_neg() {
        // FFFFFFFE00 in 40-bit two's complement = -512
        // -512 in 10-bit two's complement = 1000000000
        assert_eq!(
            codcel_hex_2_bin("FFFFFFFE00".to_string(), None).unwrap(),
            "1000000000"
        );
    }

    #[test]
    fn test_hex_2_bin_max_value() {
        assert_eq!(
            codcel_hex_2_bin("1FF".to_string(), None).unwrap(),
            "111111111"
        );
    }

    #[test]
    fn test_hex_2_bin_with_places() {
        assert_eq!(
            codcel_hex_2_bin("A".to_string(), Some(8)).unwrap(),
            "00001010"
        );
    }

    #[test]
    fn test_hex_2_bin_invalid_input() {
        assert!(codcel_hex_2_bin("XYZ".to_string(), None).is_err());
    }
}
