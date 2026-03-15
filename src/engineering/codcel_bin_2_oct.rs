// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `BIN2OCT` that converts a binary number to octal.
/// - `binary`: a string of up to 10 binary digits (0s and 1s); 10-digit values starting with `1` use two's complement for negatives.
/// - `places`: optional number of characters for the result (1–10); pads with leading zeros.
///   Returns the octal string, or an error when the input is invalid or exceeds limits.
pub fn codcel_bin_2_oct<S: AsRef<str>>(
    binary: S,
    places: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let binary_str = binary.as_ref().trim();

    // Validate input length (max 10 digits in Excel)
    if binary_str.len() > 10 {
        return Err("BIN2OCT: Binary number is too large".into());
    }

    // Validate binary string contains only 0s and 1s
    if !binary_str.chars().all(|c| c == '0' || c == '1') {
        return Err("BIN2OCT: Invalid binary number".into());
    }

    // Handle empty string
    if binary_str.is_empty() {
        return Err("BIN2OCT: Empty input string".into());
    }

    // Validate places parameter
    if let Some(p) = places {
        if !(1..=10).contains(&p) {
            return Err("BIN2OCT: Places must be between 1 and 10".into());
        }
    }

    // Parse binary and apply 10-bit two's complement
    let raw_value = u64::from_str_radix(binary_str, 2)
        .map_err(|_| "BIN2OCT: Failed to convert binary to decimal")?;

    let signed_value = if binary_str.len() == 10 && binary_str.starts_with('1') {
        raw_value as i64 - 1024
    } else {
        raw_value as i64
    };

    // Convert to 30-bit two's complement octal
    let oct = if signed_value < 0 {
        format!("{:o}", (signed_value + (1i64 << 30)) as u64)
    } else {
        format!("{:o}", signed_value)
    };

    // Apply padding
    if let Some(p) = places {
        let p = p as usize;
        if oct.len() > p && signed_value >= 0 {
            return Err("BIN2OCT: Number exceeds specified places length".into());
        }
        Ok(format!("{:0>width$}", oct, width = p.max(oct.len())))
    } else {
        Ok(oct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bin_2_oct_basic() {
        // =BIN2OCT("1010") in US format
        // =BIN2OCT("1010") in German format
        let result = codcel_bin_2_oct("1010", None).unwrap();
        println!("{result}");
        assert_eq!(result, "12");
    }

    #[test]
    fn test_bin_2_oct_with_places() {
        // =BIN2OCT("1010", 4) in US format
        // =BIN2OCT("1010"; 4) in German format
        let result = codcel_bin_2_oct("1010", Some(4)).unwrap();
        println!("{result}");
        assert_eq!(result, "0012");
    }

    #[test]
    fn test_bin_2_oct_zero() {
        // =BIN2OCT("0") in US format
        // =BIN2OCT("0") in German format
        let result = codcel_bin_2_oct("0", None).unwrap();
        println!("{result}");
        assert_eq!(result, "0");
    }

    #[test]
    fn test_bin_2_oct_negative() {
        // =BIN2OCT("1111111111") in US format
        // =BIN2OCT("1111111111") in German format
        let result = codcel_bin_2_oct("1111111111", None).unwrap();
        println!("{result}");
        assert_eq!(result, "7777777777");
    }

    #[test]
    fn test_bin_2_oct_negative_minus_2() {
        // =BIN2OCT("1111111110") in US format
        // =BIN2OCT("1111111110") in German format
        let result = codcel_bin_2_oct("1111111110", None).unwrap();
        println!("{result}");
        assert_eq!(result, "7777777776");
    }

    #[test]
    fn test_bin_2_oct_with_places_negative() {
        // =BIN2OCT("1111111110", 6) in US format
        // Excel ignores places for negative results, returns full 10-digit octal
        let result = codcel_bin_2_oct("1111111110", Some(6)).unwrap();
        println!("{result}");
        assert_eq!(result, "7777777776");
    }

    #[test]
    fn test_bin_2_oct_invalid_chars() {
        // =BIN2OCT("10102") in US format
        // =BIN2OCT("10102") in German format
        let result = codcel_bin_2_oct("10102", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_bin_2_oct_empty_string() {
        // =BIN2OCT("") in US format
        // =BIN2OCT("") in German format
        let result = codcel_bin_2_oct("", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_bin_2_oct_invalid_places() {
        // =BIN2OCT("1010", 11) in US format
        // =BIN2OCT("1010"; 11) in German format
        let result = codcel_bin_2_oct("1010", Some(11));
        assert!(result.is_err());
    }
}
