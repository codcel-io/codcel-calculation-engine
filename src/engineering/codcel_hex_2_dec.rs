// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `HEX2DEC` that converts a hexadecimal number to decimal.
/// - `hex`: the hexadecimal string to convert (up to 10 characters; uses 40-bit two's complement).
///   Returns the signed decimal value, or an error when the input contains invalid hexadecimal digits.
pub fn codcel_hex_2_dec(hex: String) -> Result<i64, Box<dyn Error + Send + Sync>> {
    let hex_trimmed = hex.trim().trim_start_matches("0x").trim_start_matches("0X");

    if hex_trimmed.is_empty() || hex_trimmed.len() > 10 {
        return Err("HEX2DEC: Invalid hexadecimal input".into());
    }

    // Parse as unsigned 64-bit
    let value =
        u64::from_str_radix(hex_trimmed, 16).map_err(|_| "HEX2DEC: Invalid hexadecimal input")?;

    // Apply 40-bit two's complement for 10-digit hex values
    const TWO_POW_39: u64 = 1u64 << 39; // 549755813888
    const TWO_POW_40: u64 = 1u64 << 40; // 1099511627776

    if value >= TWO_POW_39 {
        Ok(value as i64 - TWO_POW_40 as i64)
    } else {
        Ok(value as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_2_dec_positive() {
        assert_eq!(codcel_hex_2_dec("A5".to_string()).unwrap(), 165);
    }

    #[test]
    fn test_hex_2_dec_neg_one() {
        // 10-digit hex FFFFFFFFFF = -1 in 40-bit two's complement
        assert_eq!(codcel_hex_2_dec("FFFFFFFFFF".to_string()).unwrap(), -1);
    }

    #[test]
    fn test_hex_2_dec_max_pos() {
        // 7FFFFFFFFF = 549755813887 (max positive 40-bit)
        assert_eq!(
            codcel_hex_2_dec("7FFFFFFFFF".to_string()).unwrap(),
            549755813887
        );
    }

    #[test]
    fn test_hex_2_dec_min_neg() {
        // 8000000000 = -549755813888 (min negative 40-bit)
        assert_eq!(
            codcel_hex_2_dec("8000000000".to_string()).unwrap(),
            -549755813888
        );
    }

    #[test]
    fn test_hex_2_dec_small() {
        assert_eq!(codcel_hex_2_dec("FF".to_string()).unwrap(), 255);
    }

    #[test]
    fn test_hex_2_dec_invalid_input() {
        assert!(codcel_hex_2_dec("XYZ".to_string()).is_err());
    }
}
