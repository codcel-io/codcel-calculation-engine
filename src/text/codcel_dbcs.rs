// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `DBCS` that converts half-width (single-byte) characters to full-width (double-byte) characters.
/// - `text`: the string containing half-width characters to convert.
///   Returns the text with half-width ASCII characters (U+0021 to U+007E) converted to their
///   full-width equivalents (U+FF01 to U+FF5E). Characters outside this range are left unchanged.
///   Commonly used for Japanese text processing.
pub fn codcel_dbcs<S: AsRef<str>>(text: S) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut result = String::new();
    for c in text.as_ref().chars() {
        // Convert half-width ASCII characters (U+0021 to U+007E) to full-width
        if ('\u{0021}'..='\u{007E}').contains(&c) {
            result.push(char::from_u32(c as u32 + 0xFEE0).unwrap_or(c));
        }
        // Leave other characters (spaces, control chars, non-ASCII) as they are
        else {
            result.push(c);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbcs_with_half_width_characters() {
        // =DBCS("ABC") in US format
        let result = codcel_dbcs("ABC").unwrap();
        println!("{result}");
        assert_eq!(result, "ＡＢＣ");
    }

    #[test]
    fn test_dbcs_with_full_width_characters() {
        // =DBCS("ＡＢＣ") - already full-width, no change
        let result = codcel_dbcs("ＡＢＣ").unwrap();
        println!("{result}");
        assert_eq!(result, "ＡＢＣ");
    }

    #[test]
    fn test_dbcs_with_mixed_characters() {
        // =DBCS("ABCＤＥＦ") in US format
        let result = codcel_dbcs("ABCＤＥＦ").unwrap();
        println!("{result}");
        assert_eq!(result, "ＡＢＣＤＥＦ");
    }

    #[test]
    fn test_dbcs_with_numbers() {
        // =DBCS("123") in US format
        let result = codcel_dbcs("123").unwrap();
        println!("{result}");
        assert_eq!(result, "１２３");
    }

    #[test]
    fn test_dbcs_with_symbols() {
        // =DBCS("!@#") in US format
        let result = codcel_dbcs("!@#").unwrap();
        println!("{result}");
        assert_eq!(result, "！＠＃");
    }

    #[test]
    fn test_dbcs_roundtrip_with_asc() {
        // DBCS is the inverse of ASC: ASC(DBCS(text)) == text for ASCII
        use crate::text::codcel_asc::codcel_asc;
        let original = "Hello, World! 123";
        let full_width = codcel_dbcs(original).unwrap();
        let back = codcel_asc(&full_width).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn test_dbcs_with_space() {
        // Space (U+0020) is outside the conversion range, should stay as-is
        let result = codcel_dbcs("A B").unwrap();
        println!("{result}");
        assert_eq!(result, "Ａ Ｂ");
    }
}
