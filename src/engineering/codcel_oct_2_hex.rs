// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `OCT2HEX` that converts an octal number to hexadecimal.
/// - `octal`: the octal string to convert (up to 10 digits; uses 30-bit two's complement for negatives).
/// - `places`: optional number of characters for the result (1–10); pads with leading zeros.
///   Returns the hexadecimal string (40-bit two's complement for negatives), or an error.
pub fn codcel_oct_2_hex(
    octal: String,
    places: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let octal = octal.trim();

    // Validate octal digits only
    if octal.is_empty() || !octal.chars().all(|c| c.is_digit(8)) {
        return Err("OCT2HEX: Invalid octal number format".into());
    }

    // Parse as unsigned
    let value =
        u64::from_str_radix(octal, 8).map_err(|_| "OCT2HEX: Failed to parse octal number")?;

    // Apply 30-bit two's complement to get signed value
    const TWO_POW_29: u64 = 1u64 << 29;
    const TWO_POW_30: u64 = 1u64 << 30;
    let signed_value = if value >= TWO_POW_29 {
        value as i64 - TWO_POW_30 as i64
    } else {
        value as i64
    };

    // Convert to 40-bit two's complement hex
    let hex = if signed_value < 0 {
        format!("{:X}", (signed_value + (1i64 << 40)) as u64)
    } else {
        format!("{:X}", signed_value)
    };

    // Handle optional digit padding
    if let Some(digits) = places {
        if digits <= 0 {
            return Err("OCT2HEX: num_of_digits must be greater than 0".into());
        } else if digits > 10 {
            return Err("OCT2HEX: num_of_digits cannot be greater than 10".into());
        }
        let digits = digits as usize;
        if hex.len() > digits && signed_value >= 0 {
            return Err("OCT2HEX: Number exceeds specified places length".into());
        }
        let padded_hex = format!("{hex:0>digits$}");
        return Ok(padded_hex);
    }

    Ok(hex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oct_2_hex_simple() {
        assert_eq!(codcel_oct_2_hex("10".to_string(), None).unwrap(), "8");
    }

    #[test]
    fn test_oct_2_hex_neg_512() {
        // 7777777000 in 30-bit two's complement = -512
        // -512 in 40-bit two's complement hex = FFFFFFFE00
        assert_eq!(
            codcel_oct_2_hex("7777777000".to_string(), None).unwrap(),
            "FFFFFFFE00"
        );
    }

    #[test]
    fn test_oct_2_hex_with_places() {
        assert_eq!(
            codcel_oct_2_hex("10".to_string(), Some(4)).unwrap(),
            "0008"
        );
    }

    #[test]
    fn test_oct_2_hex_zero() {
        assert_eq!(codcel_oct_2_hex("0".to_string(), None).unwrap(), "0");
    }

    #[test]
    fn test_oct_2_hex_invalid() {
        assert!(codcel_oct_2_hex("9".to_string(), None).is_err());
    }
}
