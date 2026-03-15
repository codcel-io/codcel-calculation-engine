// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `PHONETIC` that returns the phonetic (furigana) characters from a text string.
/// Since Codcel operates on plain string data without embedded furigana metadata,
/// this returns the input text unchanged — matching Excel's behavior for non-annotated text.
/// - `text`: the text to extract phonetic characters from.
///   Returns the text unchanged.
pub fn codcel_phonetic<S: AsRef<str>>(text: S) -> Result<String, Box<dyn Error + Send + Sync>> {
    Ok(text.as_ref().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phonetic_plain_text() {
        // =PHONETIC("hello") returns "hello" for non-annotated text
        let result = codcel_phonetic("hello").unwrap();
        println!("{result}");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_phonetic_mixed_case() {
        // =PHONETIC("Hello World") returns "Hello World"
        let result = codcel_phonetic("Hello World").unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_phonetic_with_numbers() {
        // =PHONETIC("abc123") returns "abc123"
        let result = codcel_phonetic("abc123").unwrap();
        println!("{result}");
        assert_eq!(result, "abc123");
    }

    #[test]
    fn test_phonetic_empty_string() {
        // =PHONETIC("") returns ""
        let result = codcel_phonetic("").unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_phonetic_special_chars() {
        // =PHONETIC("Hello, World!") returns "Hello, World!"
        let result = codcel_phonetic("Hello, World!").unwrap();
        println!("{result}");
        assert_eq!(result, "Hello, World!");
    }
}
