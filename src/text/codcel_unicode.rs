// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `UNICODE` that returns the Unicode code point for the first character.
/// - `text`: the text string from which to get the first character's code point.
///   Returns the Unicode code point (integer value) of the first character in the text.
///   Returns an error if the input string is empty.
///   This is functionally equivalent to CODE but explicitly named for Unicode support.
pub fn codcel_unicode<S: AsRef<str>>(text: S) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref();
    if let Some(character) = text.chars().next() {
        Ok(character as i32)
    } else {
        Err("UNICODE: Input string is empty".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_ascii_letter() {
        // =UNICODE("A") in US format
        // =UNICODE("A") in German format
        let result = codcel_unicode("A").unwrap();
        println!("{result}");
        assert_eq!(result, 65);
    }

    #[test]
    fn test_unicode_ascii_number() {
        // =UNICODE("1") in US format
        // =UNICODE("1") in German format
        let result = codcel_unicode("1").unwrap();
        println!("{result}");
        assert_eq!(result, 49);
    }

    #[test]
    fn test_unicode_special_char() {
        // =UNICODE("!") in US format
        // =UNICODE("!") in German format
        let result = codcel_unicode("!").unwrap();
        println!("{result}");
        assert_eq!(result, 33);
    }

    #[test]
    fn test_unicode_non_ascii() {
        // =UNICODE("é") in US format
        // =UNICODE("é") in German format
        let result = codcel_unicode("é").unwrap();
        println!("{result}");
        assert_eq!(result, 233);
    }

    #[test]
    fn test_unicode_emoji() {
        // =UNICODE("😀") in US format
        // =UNICODE("😀") in German format
        let result = codcel_unicode("😀").unwrap();
        println!("{result}");
        assert_eq!(result, 128512);
    }

    #[test]
    fn test_unicode_first_char_only() {
        // =UNICODE("ABC") in US format
        // =UNICODE("ABC") in German format
        let result = codcel_unicode("ABC").unwrap();
        println!("{result}");
        assert_eq!(result, 65); // Only returns the code for 'A'
    }

    #[test]
    fn test_unicode_empty_string() {
        // =UNICODE("") in US format
        // =UNICODE("") in German format
        let result = codcel_unicode("");
        assert!(result.is_err());
    }
}
