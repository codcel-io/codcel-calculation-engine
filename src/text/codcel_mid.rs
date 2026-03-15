// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `MID` that returns a specific number of characters from a text string.
/// - `text`: the text string containing the characters to extract.
/// - `start_position`: the position of the first character to extract (1-based index).
/// - `num_chars`: the number of characters to extract.
///   Returns the specified substring. Returns an error if `start_position` < 1 or `num_chars` < 0.
///   If the extraction extends beyond the text end, returns characters up to the end.
///   Handles Unicode characters correctly.
pub fn codcel_mid<S: AsRef<str>>(
    text: S,
    start_position: i32,
    num_chars: i32,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref();
    if start_position < 1 || num_chars < 0 {
        return Err("MID: Invalid start position or number of characters.".into());
    }

    let start_position = start_position as usize - 1; // Convert to 0-based index
    let num_chars = num_chars as usize;

    // Handle Unicode characters properly by using character iterators
    let chars: Vec<char> = text.chars().collect();

    if start_position >= chars.len() {
        return Ok(String::new());
    }

    let end_position = std::cmp::min(start_position + num_chars, chars.len());
    Ok(chars[start_position..end_position].iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mid_basic() {
        // =MID("Hello World", 7, 5) in US format
        // =MID("Hello World"; 7; 5) in German format
        let result = codcel_mid("Hello World", 7, 5).unwrap();
        println!("{result}");
        assert_eq!(result, "World");
    }

    #[test]
    fn test_mid_single_char() {
        // =MID("Hello", 2, 1) in US format
        // =MID("Hello"; 2; 1) in German format
        let result = codcel_mid("Hello", 2, 1).unwrap();
        println!("{result}");
        assert_eq!(result, "e");
    }

    #[test]
    fn test_mid_zero_chars() {
        // =MID("Hello", 2, 0) in US format
        // =MID("Hello"; 2; 0) in German format
        let result = codcel_mid("Hello", 2, 0).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_mid_beyond_end() {
        // =MID("Hello", 2, 10) in US format
        // =MID("Hello"; 2; 10) in German format
        let result = codcel_mid("Hello", 2, 10).unwrap();
        println!("{result}");
        assert_eq!(result, "ello");
    }

    #[test]
    fn test_mid_start_beyond_end() {
        // =MID("Hello", 10, 2) in US format
        // =MID("Hello"; 10; 2) in German format
        let result = codcel_mid("Hello", 10, 2).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_mid_with_unicode() {
        // =MID("😀😃😄😁😆", 2, 3) in US format
        // =MID("😀😃😄😁😆"; 2; 3) in German format
        let result = codcel_mid("😀😃😄😁😆", 2, 3).unwrap();
        println!("{result}");
        assert_eq!(result, "😃😄😁");
    }

    #[test]
    fn test_mid_invalid_start() {
        // =MID("Hello", 0, 3) in US format - this should error
        // =MID("Hello"; 0; 3) in German format - this should error
        let result = codcel_mid("Hello", 0, 3);
        assert!(result.is_err());
    }

    #[test]
    fn test_mid_negative_chars() {
        // =MID("Hello", 2, -1) in US format - this should error
        // =MID("Hello"; 2; -1) in German format - this should error
        let result = codcel_mid("Hello", 2, -1);
        assert!(result.is_err());
    }
}
