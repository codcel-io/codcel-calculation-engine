// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `LEFT` that returns the specified number of characters from the start of a text string.
/// - `text`: the text string from which to extract characters.
/// - `num_chars`: optional number of characters to extract (default 1).
///   Returns the leftmost characters from the text. If `num_chars` exceeds the text length,
///   returns the entire text. Returns an error if `num_chars` is negative.
///   Handles Unicode characters correctly.
pub fn codcel_left<S: AsRef<str>>(
    text: S,
    num_chars: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text_ref = text.as_ref();
    let chars_to_take = num_chars.unwrap_or(1); // Default to 1 if `num_chars` is None

    if chars_to_take < 0 {
        return Err("LEFT: The number of characters must be non-negative.".into());
    }

    let result = if chars_to_take as usize >= text_ref.chars().count() {
        text_ref.to_string() // Return the entire string if `chars_to_take` is greater than char count
    } else {
        text_ref.chars().take(chars_to_take as usize).collect() // Collect only the first `chars_to_take` characters
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_default() {
        // =LEFT("Hello") in US format
        // =LEFT("Hello") in German format
        let result = codcel_left("Hello", None).unwrap();
        println!("{result}");
        assert_eq!(result, "H");
    }

    #[test]
    fn test_left_with_num_chars() {
        // =LEFT("Hello", 3) in US format
        // =LEFT("Hello"; 3) in German format
        let result = codcel_left("Hello", Some(3)).unwrap();
        println!("{result}");
        assert_eq!(result, "Hel");
    }

    #[test]
    fn test_left_with_zero_chars() {
        // =LEFT("Hello", 0) in US format
        // =LEFT("Hello"; 0) in German format
        let result = codcel_left("Hello", Some(0)).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_left_with_more_chars_than_string() {
        // =LEFT("Hello", 10) in US format
        // =LEFT("Hello"; 10) in German format
        let result = codcel_left("Hello", Some(10)).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_left_with_empty_string() {
        // =LEFT("", 3) in US format
        // =LEFT(""; 3) in German format
        let result = codcel_left("", Some(3)).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_left_with_unicode_chars() {
        // =LEFT("😀😃😄", 2) in US format
        // =LEFT("😀😃😄"; 2) in German format
        let result = codcel_left("😀😃😄", Some(2)).unwrap();
        println!("{result}");
        assert_eq!(result, "😀😃");
    }

    #[test]
    fn test_left_with_negative_chars() {
        // =LEFT("Hello", -1) -> #VALUE! error
        let result = codcel_left("Hello", Some(-1));
        assert!(result.is_err());
    }
}
