// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `UNICHAR` that returns the character for a given Unicode code point.
/// - `code`: the Unicode code point (integer value) of the character to return.
///   Returns the character corresponding to the given code point.
///   Returns an error if the code point is invalid (e.g., out of Unicode range or a surrogate).
pub fn codcel_uni_char(code: i32) -> Result<String, Box<dyn Error + Send + Sync>> {
    if let Some(character) = char::from_u32(code as u32) {
        Ok(character.to_string())
    } else {
        Err(format!("UNICHAR: Invalid Unicode code point: {code}").into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uni_char_ascii_letter() {
        // =UNICHAR(65) in US format
        // =UNICHAR(65) in German format
        let result = codcel_uni_char(65).unwrap();
        println!("{result}");
        assert_eq!(result, "A");
    }

    #[test]
    fn test_uni_char_ascii_number() {
        // =UNICHAR(49) in US format
        // =UNICHAR(49) in German format
        let result = codcel_uni_char(49).unwrap();
        println!("{result}");
        assert_eq!(result, "1");
    }

    #[test]
    fn test_uni_char_special_char() {
        // =UNICHAR(33) in US format
        // =UNICHAR(33) in German format
        let result = codcel_uni_char(33).unwrap();
        println!("{result}");
        assert_eq!(result, "!");
    }

    #[test]
    fn test_uni_char_non_ascii() {
        // =UNICHAR(233) in US format
        // =UNICHAR(233) in German format
        let result = codcel_uni_char(233).unwrap();
        println!("{result}");
        assert_eq!(result, "é");
    }

    #[test]
    fn test_uni_char_emoji() {
        // =UNICHAR(128512) in US format
        // =UNICHAR(128512) in German format
        let result = codcel_uni_char(128512).unwrap();
        println!("{result}");
        assert_eq!(result, "😀");
    }

    #[test]
    fn test_uni_char_space() {
        // =UNICHAR(32) in US format
        // =UNICHAR(32) in German format
        let result = codcel_uni_char(32).unwrap();
        println!("{result}");
        assert_eq!(result, " ");
    }

    #[test]
    fn test_uni_char_invalid_code() {
        // =UNICHAR(1114112) in US format - this is an invalid Unicode code point
        // =UNICHAR(1114112) in German format - this is an invalid Unicode code point
        let result = codcel_uni_char(1114112);
        assert!(result.is_err());
    }
}
