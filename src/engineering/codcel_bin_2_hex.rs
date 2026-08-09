// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `BIN2HEX` that converts a binary number to hexadecimal.
/// - `binary`: a string of up to 10 binary digits; 10-digit values starting with `1` use two's complement.
/// - `places`: optional number of characters for the result (1–10); pads with leading zeros.
///   Returns the hexadecimal string (40-bit two's complement for negatives), or an error.
pub fn codcel_bin_2_hex<S: AsRef<str>>(
    binary: S,
    places: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let binary_str = binary.as_ref().trim();

    if binary_str.is_empty() {
        return Err("BIN2HEX: Empty input string".into());
    }
    if binary_str.len() > 10 {
        return Err("BIN2HEX: Binary number is too large".into());
    }
    if !binary_str.chars().all(|c| c == '0' || c == '1') {
        return Err("BIN2HEX: Invalid binary number".into());
    }

    if let Some(p) = places {
        if !(1..=10).contains(&p) {
            return Err("BIN2HEX: Places must be between 1 and 10".into());
        }
    }

    // Parse binary and apply 10-bit two's complement
    let raw_value = u64::from_str_radix(binary_str, 2)
        .map_err(|_| "BIN2HEX: Failed to convert binary to decimal")?;

    let signed_value = if binary_str.len() == 10 && binary_str.starts_with('1') {
        // 10-bit two's complement: value >= 512 means negative
        raw_value as i64 - 1024
    } else {
        raw_value as i64
    };

    // Convert to 40-bit two's complement hex
    let hex = if signed_value < 0 {
        format!("{:X}", (signed_value + (1i64 << 40)) as u64)
    } else {
        format!("{:X}", signed_value)
    };

    // Apply padding
    if let Some(p) = places {
        let p = p as usize;
        Ok(format!("{:0>width$}", hex, width = p.max(hex.len())))
    } else {
        Ok(hex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_2_hex_basic() {
        assert_eq!(codcel_bin_2_hex("1010", None).unwrap(), "A");
    }

    #[test]
    fn test_bin_2_hex_min_val() {
        // 1000000000 in 10-bit two's complement = -512
        // -512 in 40-bit two's complement hex = FFFFFFFE00
        assert_eq!(codcel_bin_2_hex("1000000000", None).unwrap(), "FFFFFFFE00");
    }

    #[test]
    fn test_bin_2_hex_neg_one() {
        assert_eq!(codcel_bin_2_hex("1111111111", None).unwrap(), "FFFFFFFFFF");
    }

    #[test]
    fn test_bin_2_hex_with_places() {
        assert_eq!(codcel_bin_2_hex("1010", Some(4)).unwrap(), "000A");
    }

    #[test]
    fn test_bin_2_hex_zero() {
        assert_eq!(codcel_bin_2_hex("0", None).unwrap(), "0");
    }

    #[test]
    fn test_bin_2_hex_invalid() {
        assert!(codcel_bin_2_hex("10102", None).is_err());
    }
}
