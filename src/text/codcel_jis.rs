// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

use super::codcel_dbcs::codcel_dbcs;

/// Excel-compatible `JIS` that converts half-width (single-byte) characters to full-width (double-byte) characters.
/// - `text`: the string containing half-width characters to convert.
///   Returns the text with half-width ASCII characters (U+0021 to U+007E) converted to their
///   full-width equivalents (U+FF01 to U+FF5E). Characters outside this range are left unchanged.
///   JIS is functionally identical to DBCS — it is the Japanese locale name for the same operation.
pub fn codcel_jis<S: AsRef<str>>(text: S) -> Result<String, Box<dyn Error + Send + Sync>> {
    codcel_dbcs(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jis_with_half_width_characters() {
        // =JIS("ABC") converts half-width to full-width
        let result = codcel_jis("ABC").unwrap();
        assert_eq!(result, "ＡＢＣ");
    }

    #[test]
    fn test_jis_with_full_width_characters() {
        // =JIS("ＡＢＣ") - already full-width, no change
        let result = codcel_jis("ＡＢＣ").unwrap();
        assert_eq!(result, "ＡＢＣ");
    }

    #[test]
    fn test_jis_with_mixed_characters() {
        // =JIS("ABCＤＥＦ")
        let result = codcel_jis("ABCＤＥＦ").unwrap();
        assert_eq!(result, "ＡＢＣＤＥＦ");
    }

    #[test]
    fn test_jis_with_numbers() {
        // =JIS("123")
        let result = codcel_jis("123").unwrap();
        assert_eq!(result, "１２３");
    }

    #[test]
    fn test_jis_with_symbols() {
        // =JIS("!@#")
        let result = codcel_jis("!@#").unwrap();
        assert_eq!(result, "！＠＃");
    }

    #[test]
    fn test_jis_roundtrip_with_asc() {
        // JIS is the inverse of ASC: ASC(JIS(text)) == text for ASCII
        use crate::text::codcel_asc::codcel_asc;
        let original = "Hello, World! 123";
        let full_width = codcel_jis(original).unwrap();
        let back = codcel_asc(&full_width).unwrap();
        assert_eq!(back, original);
    }

    #[test]
    fn test_jis_with_space() {
        // Space (U+0020) is outside the conversion range, should stay as-is
        let result = codcel_jis("A B").unwrap();
        assert_eq!(result, "Ａ Ｂ");
    }

    #[test]
    fn test_jis_matches_dbcs() {
        // JIS and DBCS should produce identical results
        use crate::text::codcel_dbcs::codcel_dbcs;
        let input = "Hello 123 !@#";
        assert_eq!(codcel_jis(input).unwrap(), codcel_dbcs(input).unwrap());
    }
}
