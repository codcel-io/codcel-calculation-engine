// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `DECIMAL` that converts a text representation in a given base to a decimal number.
/// - `text`: a text string representing the number in the specified base.
/// - `radix`: the base of the number (2–36).
///
/// Returns the decimal integer or an error for invalid radix or unparseable text.
pub fn codcel_decimal(text: &str, radix: i32) -> Result<i32, Box<dyn Error + Send + Sync>> {
    // Ensure the radix is within the valid range
    if !(2..=36).contains(&radix) {
        return Err(format!("DECIMAL: Radix must be between 2 and 36, got {radix}").into());
    }

    // Check if the number is negative
    let is_negative = text.starts_with('-');
    let number_str = if is_negative { &text[1..] } else { text };

    // Attempt to parse the input string as a number in the specified base
    let value = i32::from_str_radix(number_str, radix as u32)
        .map_err(|e| format!("DECIMAL: Error parsing '{text}': {e}"))?;

    // Adjust for negativity if applicable
    Ok(if is_negative { -value } else { value })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_binary() {
        // =DECIMAL("1010", 2) in US format
        // =DECIMAL("1010"; 2) in German format
        let result = codcel_decimal("1010", 2).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_decimal_octal() {
        // =DECIMAL("144", 8) in US format
        // =DECIMAL("144"; 8) in German format
        let result = codcel_decimal("144", 8).unwrap();
        assert_eq!(result, 100);
    }

    #[test]
    fn test_decimal_hexadecimal() {
        // =DECIMAL("FF", 16) in US format
        // =DECIMAL("FF"; 16) in German format
        let result = codcel_decimal("FF", 16).unwrap();
        assert_eq!(result, 255);
    }

    #[test]
    fn test_decimal_base36() {
        // =DECIMAL("YA", 36) in US format
        // =DECIMAL("YA"; 36) in German format
        let result = codcel_decimal("YA", 36).unwrap();
        assert_eq!(result, 1234);
    }

    #[test]
    fn test_decimal_negative() {
        // =DECIMAL("-1010", 2) in US format
        // =DECIMAL("-1010"; 2) in German format
        let result = codcel_decimal("-1010", 2).unwrap();
        assert_eq!(result, -10);
    }

    #[test]
    fn test_decimal_zero() {
        // =DECIMAL("0", 2) in US format
        // =DECIMAL("0"; 2) in German format
        let result = codcel_decimal("0", 2).unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_decimal_case_insensitivity() {
        // =DECIMAL("ff", 16) in US format
        // =DECIMAL("ff"; 16) in German format
        let result = codcel_decimal("ff", 16).unwrap();
        assert_eq!(result, 255);

        // =DECIMAL("FF", 16) in US format
        // =DECIMAL("FF"; 16) in German format
        let result = codcel_decimal("FF", 16).unwrap();
        assert_eq!(result, 255);
    }

    #[test]
    fn test_decimal_invalid_radix() {
        // =DECIMAL("1010", 1) in US format - should return an error
        // =DECIMAL("1010"; 1) in German format - should return an error
        let result = codcel_decimal("1010", 1);
        assert!(result.is_err());

        // =DECIMAL("1010", 37) in US format - should return an error
        // =DECIMAL("1010"; 37) in German format - should return an error
        let result = codcel_decimal("1010", 37);
        assert!(result.is_err());
    }

    #[test]
    fn test_decimal_invalid_digit() {
        // =DECIMAL("12A", 10) in US format - should return an error
        // =DECIMAL("12A"; 10) in German format - should return an error
        let result = codcel_decimal("12A", 10);
        assert!(result.is_err());
    }
}
