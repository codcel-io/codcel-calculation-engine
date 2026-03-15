// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `HEX2OCT` that converts a hexadecimal number to octal.
/// - `hex`: the hexadecimal string to convert (up to 10 characters; uses 40-bit two's complement).
///   The resulting decimal value must be in the 30-bit range.
/// - `places`: optional number of characters for the result; pads with leading zeros.
///   Returns the octal string (30-bit two's complement for negatives), or an error.
pub fn codcel_hex_2_oct(
    hex: String,
    places: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let hex_trimmed = hex.trim().trim_start_matches("0x").trim_start_matches("0X");

    if hex_trimmed.is_empty() || hex_trimmed.len() > 10 {
        return Err("HEX2OCT: Invalid hexadecimal input".into());
    }

    // Parse as unsigned 64-bit
    let value =
        u64::from_str_radix(hex_trimmed, 16).map_err(|_| "HEX2OCT: Invalid hexadecimal input")?;

    // Apply 40-bit two's complement to get signed value
    const TWO_POW_39: u64 = 1u64 << 39;
    const TWO_POW_40: u64 = 1u64 << 40;
    let signed_value = if value >= TWO_POW_39 {
        value as i64 - TWO_POW_40 as i64
    } else {
        value as i64
    };

    // Check range for octal output: must be in 30-bit range
    const MIN_OCT: i64 = -(1i64 << 29); // -536870912
    const MAX_OCT: i64 = (1i64 << 29) - 1; // 536870911

    if !(MIN_OCT..=MAX_OCT).contains(&signed_value) {
        return Err("HEX2OCT: Number out of range for octal conversion".into());
    }

    // Convert to 30-bit two's complement octal
    let oct = if signed_value < 0 {
        format!("{:o}", (signed_value + (1i64 << 30)) as u64)
    } else {
        format!("{:o}", signed_value)
    };

    // Apply zero-padding if 'places' is specified
    if let Some(p) = places {
        if oct.len() > p as usize {
            return Err("HEX2OCT: Number exceeds specified places length".into());
        }
        Ok(format!("{:0>width$}", oct, width = p as usize))
    } else {
        Ok(oct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_2_oct_positive() {
        assert_eq!(codcel_hex_2_oct("A5".to_string(), None).unwrap(), "245");
    }

    #[test]
    fn test_hex_2_oct_neg() {
        // FFFFFFE000 in 40-bit two's complement = -8192
        // -8192 in 30-bit two's complement octal = 7777760000
        assert_eq!(
            codcel_hex_2_oct("FFFFFFE000".to_string(), None).unwrap(),
            "7777760000"
        );
    }

    #[test]
    fn test_hex_2_oct_with_places() {
        assert_eq!(
            codcel_hex_2_oct("A5".to_string(), Some(5)).unwrap(),
            "00245"
        );
    }

    #[test]
    fn test_hex_2_oct_invalid() {
        assert!(codcel_hex_2_oct("XYZ".to_string(), None).is_err());
    }
}
