// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `UPPER` that converts all lowercase letters in a text string to uppercase.
/// - `text`: the text to convert to uppercase.
///   Returns the text with all letters converted to uppercase.
///   Non-alphabetic characters remain unchanged.
pub fn codcel_upper<S: AsRef<str>>(text: S) -> Result<String, Box<dyn Error + Send + Sync>> {
    Ok(text.as_ref().to_uppercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upper_lowercase() {
        // =UPPER("hello") in US format
        // =UPPER("hello") in German format
        let result = codcel_upper("hello").unwrap();
        println!("{result}");
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_upper_mixed_case() {
        // =UPPER("Hello World") in US format
        // =UPPER("Hello World") in German format
        let result = codcel_upper("Hello World").unwrap();
        println!("{result}");
        assert_eq!(result, "HELLO WORLD");
    }

    #[test]
    fn test_upper_with_numbers() {
        // =UPPER("abc123") in US format
        // =UPPER("abc123") in German format
        let result = codcel_upper("abc123").unwrap();
        println!("{result}");
        assert_eq!(result, "ABC123");
    }

    #[test]
    fn test_upper_with_special_chars() {
        // =UPPER("Hello, World!") in US format
        // =UPPER("Hello, World!") in German format
        let result = codcel_upper("Hello, World!").unwrap();
        println!("{result}");
        assert_eq!(result, "HELLO, WORLD!");
    }

    #[test]
    fn test_upper_already_uppercase() {
        // =UPPER("ALREADY UPPERCASE") in US format
        // =UPPER("ALREADY UPPERCASE") in German format
        let result = codcel_upper("ALREADY UPPERCASE").unwrap();
        println!("{result}");
        assert_eq!(result, "ALREADY UPPERCASE");
    }
}
