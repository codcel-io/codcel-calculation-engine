// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `RIGHT` that returns the specified number of characters from the end of a text string.
/// - `text`: the text string from which to extract characters.
/// - `num_chars`: optional number of characters to extract (default 1).
///   Returns the rightmost characters from the text. If `num_chars` exceeds the text length,
///   returns the entire text. Returns an error if `num_chars` is negative.
///   Handles Unicode characters correctly.
pub fn codcel_right<S: AsRef<str>>(
    text: S,
    num_chars: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text_ref = text.as_ref();
    let chars_to_take = num_chars.unwrap_or(1); // Default to 1 if `num_chars` is None

    if chars_to_take < 0 {
        return Err("RIGHT: The number of characters must be non-negative.".into());
    }

    let result = if chars_to_take > text_ref.len() as i32 {
        text_ref.to_string() // Return the entire string if `chars_to_take` exceeds string length
    } else {
        text_ref
            .chars()
            .rev() // Reverse the characters
            .take(chars_to_take as usize) // Take the last `chars_to_take` characters
            .collect::<Vec<char>>()
            .into_iter()
            .rev() // Reverse again to maintain the original order
            .collect()
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_default() {
        // =RIGHT("Hello") in US format
        // =RIGHT("Hello") in German format
        let result = codcel_right("Hello", None).unwrap();
        println!("{result}");
        assert_eq!(result, "o");
    }

    #[test]
    fn test_right_with_num_chars() {
        // =RIGHT("Hello World", 5) in US format
        // =RIGHT("Hello World"; 5) in German format
        let result = codcel_right("Hello World", Some(5)).unwrap();
        println!("{result}");
        assert_eq!(result, "World");
    }

    #[test]
    fn test_right_with_zero_chars() {
        // =RIGHT("Hello", 0) in US format
        // =RIGHT("Hello"; 0) in German format
        let result = codcel_right("Hello", Some(0)).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_right_with_more_chars_than_string() {
        // =RIGHT("Hello", 10) in US format
        // =RIGHT("Hello"; 10) in German format
        let result = codcel_right("Hello", Some(10)).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_right_with_empty_string() {
        // =RIGHT("", 3) in US format
        // =RIGHT(""; 3) in German format
        let result = codcel_right("", Some(3)).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_right_with_unicode_chars() {
        // =RIGHT("😀😃😄", 2) in US format
        // =RIGHT("😀😃😄"; 2) in German format
        let result = codcel_right("😀😃😄", Some(2)).unwrap();
        println!("{result}");
        assert_eq!(result, "😃😄");
    }

    #[test]
    fn test_right_with_negative_chars() {
        // =RIGHT("Hello", -1) in US format - this should error
        // =RIGHT("Hello"; -1) in German format - this should error
        let result = codcel_right("Hello", Some(-1));
        assert!(result.is_err());
    }
}
