// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `ASC` that converts full-width (double-byte) characters to half-width (single-byte) characters.
/// - `text`: the string containing full-width characters to convert.
///   Returns the text with full-width characters (U+FF01 to U+FF5E) converted to their
///   half-width ASCII equivalents. Characters outside this range are left unchanged.
///   Commonly used for Japanese text processing.
pub fn codcel_asc<S: AsRef<str>>(text: S) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut result = String::new();
    for c in text.as_ref().chars() {
        // Convert full-width characters (U+FF01 to U+FF5E) to half-width
        if ('\u{FF01}'..='\u{FF5E}').contains(&c) {
            result.push((c as u32 - 0xFEE0) as u8 as char);
        }
        // Leave other characters as they are
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
    fn test_asc_with_full_width_characters() {
        // =ASC("ＡＢＣ") in US format
        // =ASC("ＡＢＣ") in German format
        let result = codcel_asc("ＡＢＣ").unwrap();
        println!("{result}");
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_asc_with_regular_characters() {
        // =ASC("ABC") in US format
        // =ASC("ABC") in German format
        let result = codcel_asc("ABC").unwrap();
        println!("{result}");
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_asc_with_mixed_characters() {
        // =ASC("ABCＤＥＦ") in US format
        // =ASC("ABCＤＥＦ") in German format
        let result = codcel_asc("ABCＤＥＦ").unwrap();
        println!("{result}");
        assert_eq!(result, "ABCDEF");
    }

    #[test]
    fn test_asc_with_numbers() {
        // =ASC("１２３") in US format
        // =ASC("１２３") in German format
        let result = codcel_asc("１２３").unwrap();
        println!("{result}");
        assert_eq!(result, "123");
    }

    #[test]
    fn test_asc_with_symbols() {
        // =ASC("！＠＃") in US format
        // =ASC("！＠＃") in German format
        let result = codcel_asc("！＠＃").unwrap();
        println!("{result}");
        assert_eq!(result, "!@#");
    }
}
