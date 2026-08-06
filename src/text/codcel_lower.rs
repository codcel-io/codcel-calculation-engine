// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `LOWER` that converts all uppercase letters in a text string to lowercase.
/// - `text`: the text to convert to lowercase.
///   Returns the text with all letters converted to lowercase.
///   Non-alphabetic characters remain unchanged.
pub fn codcel_lower<S: AsRef<str>>(text: S) -> Result<String, Box<dyn Error + Send + Sync>> {
    Ok(text.as_ref().to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_uppercase() {
        // =LOWER("HELLO") in US format
        // =LOWER("HELLO") in German format
        let result = codcel_lower("HELLO").unwrap();
        println!("{result}");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_lower_mixed_case() {
        // =LOWER("Hello World") in US format
        // =LOWER("Hello World") in German format
        let result = codcel_lower("Hello World").unwrap();
        println!("{result}");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_lower_with_numbers() {
        // =LOWER("ABC123") in US format
        // =LOWER("ABC123") in German format
        let result = codcel_lower("ABC123").unwrap();
        println!("{result}");
        assert_eq!(result, "abc123");
    }

    #[test]
    fn test_lower_with_special_chars() {
        // =LOWER("HELLO, WORLD!") in US format
        // =LOWER("HELLO, WORLD!") in German format
        let result = codcel_lower("HELLO, WORLD!").unwrap();
        println!("{result}");
        assert_eq!(result, "hello, world!");
    }

    #[test]
    fn test_lower_already_lowercase() {
        // =LOWER("already lowercase") in US format
        // =LOWER("already lowercase") in German format
        let result = codcel_lower("already lowercase").unwrap();
        println!("{result}");
        assert_eq!(result, "already lowercase");
    }
}
