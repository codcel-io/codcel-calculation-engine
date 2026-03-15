// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `CODE` that returns the numeric code for the first character in a text string.
/// - `text`: the text string from which to extract the first character's code.
///   Returns the Unicode code point of the first character.
///   Returns an error if the input string is empty.
pub fn codcel_code<S: AsRef<str>>(text: S) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref();
    if let Some(character) = text.chars().next() {
        Ok(character as i32)
    } else {
        Err("CODE: Input string is empty".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_ascii_letter() {
        // =CODE("A") in US format
        // =CODE("A") in German format
        let result = codcel_code("A").unwrap();
        println!("{result}");
        assert_eq!(result, 65);
    }

    #[test]
    fn test_code_ascii_number() {
        // =CODE("1") in US format
        // =CODE("1") in German format
        let result = codcel_code("1").unwrap();
        println!("{result}");
        assert_eq!(result, 49);
    }

    #[test]
    fn test_code_ascii_symbol() {
        // =CODE("!") in US format
        // =CODE("!") in German format
        let result = codcel_code("!").unwrap();
        println!("{result}");
        assert_eq!(result, 33);
    }

    #[test]
    fn test_code_unicode_symbol() {
        // =CODE("★") in US format
        // =CODE("★") in German format
        let result = codcel_code("★").unwrap();
        println!("{result}");
        assert_eq!(result, 9733);
    }

    #[test]
    fn test_code_first_character_only() {
        // =CODE("ABC") in US format
        // =CODE("ABC") in German format
        let result = codcel_code("ABC").unwrap();
        println!("{result}");
        assert_eq!(result, 65); // Only returns code for 'A'
    }

    #[test]
    fn test_code_empty_string() {
        // =CODE("") in US format
        // =CODE("") in German format
        let result = codcel_code("");
        assert!(result.is_err());
    }
}
