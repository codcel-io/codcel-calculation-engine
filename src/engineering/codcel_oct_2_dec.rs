// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `OCT2DEC` that converts an octal number to decimal.
/// - `octal`: the octal string to convert (up to 10 characters; uses 30-bit two's complement).
///   Returns the signed decimal value, or an error when the input is not valid base-8.
pub fn codcel_oct_2_dec(octal: String) -> Result<i64, Box<dyn Error + Send + Sync>> {
    let octal = octal.trim();

    if octal.is_empty() || octal.len() > 10 {
        return Err("OCT2DEC: Invalid octal number format".into());
    }

    // Validate octal digits only
    if !octal.chars().all(|c| c.is_digit(8)) {
        return Err("OCT2DEC: Invalid octal number format".into());
    }

    // Parse as unsigned 64-bit
    let value =
        u64::from_str_radix(octal, 8).map_err(|_| "OCT2DEC: Failed to parse octal number")?;

    // Apply 30-bit two's complement for 10-digit octal values
    const TWO_POW_29: u64 = 1u64 << 29; // 536870912
    const TWO_POW_30: u64 = 1u64 << 30; // 1073741824

    if value >= TWO_POW_29 {
        Ok(value as i64 - TWO_POW_30 as i64)
    } else {
        Ok(value as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oct_2_dec_simple() {
        assert_eq!(codcel_oct_2_dec("10".to_string()).unwrap(), 8);
    }

    #[test]
    fn test_oct_2_dec_neg_one() {
        // 7777777777 = -1 in 30-bit two's complement
        assert_eq!(codcel_oct_2_dec("7777777777".to_string()).unwrap(), -1);
    }

    #[test]
    fn test_oct_2_dec_neg_512() {
        // 7777777000 = -512 in 30-bit two's complement
        assert_eq!(codcel_oct_2_dec("7777777000".to_string()).unwrap(), -512);
    }

    #[test]
    fn test_oct_2_dec_zero() {
        assert_eq!(codcel_oct_2_dec("0".to_string()).unwrap(), 0);
    }

    #[test]
    fn test_oct_2_dec_invalid_octal() {
        assert!(codcel_oct_2_dec("9".to_string()).is_err());
    }
}
