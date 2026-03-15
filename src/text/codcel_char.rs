// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `CHAR` that returns the character specified by a code number.
/// - `code`: a number between 1 and 1114111 representing a Unicode code point.
///   Returns the character corresponding to the given code point.
///   Returns an error if the code is outside the valid Unicode range or cannot be
///   converted to a valid character.
pub fn codcel_char(code: i32) -> Result<String, Box<dyn Error + Send + Sync>> {
    if !(1..=1114111).contains(&code) {
        return Err("CHAR: Code is out of valid Unicode range (1-1114111).".into());
    }
    if let Some(value) = std::char::from_u32(code as u32) {
        Ok(format!("{value}"))
    } else {
        Err(format!("CHAR: Code {code:} can't be converted to a Unicode char.").into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_ascii_letter() {
        // =CHAR(65) in US format
        // =CHAR(65) in German format
        let result = codcel_char(65).unwrap();
        println!("{result}");
        assert_eq!(result, "A");
    }

    #[test]
    fn test_char_ascii_number() {
        // =CHAR(49) in US format
        // =CHAR(49) in German format
        let result = codcel_char(49).unwrap();
        println!("{result}");
        assert_eq!(result, "1");
    }

    #[test]
    fn test_char_ascii_symbol() {
        // =CHAR(33) in US format
        // =CHAR(33) in German format
        let result = codcel_char(33).unwrap();
        println!("{result}");
        assert_eq!(result, "!");
    }

    #[test]
    fn test_char_unicode_symbol() {
        // =CHAR(9733) in US format
        // =CHAR(9733) in German format
        let result = codcel_char(9733).unwrap();
        println!("{result}");
        assert_eq!(result, "★");
    }

    #[test]
    fn test_char_out_of_range_low() {
        // =CHAR(0) in US format
        // =CHAR(0) in German format
        let result = codcel_char(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_char_out_of_range_high() {
        // =CHAR(1114112) in US format
        // =CHAR(1114112) in German format
        let result = codcel_char(1114112);
        assert!(result.is_err());
    }
}
