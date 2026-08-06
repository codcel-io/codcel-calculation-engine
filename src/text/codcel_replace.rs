// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `REPLACE` that replaces part of a text string with a different text string.
/// - `old_text`: the original text to modify.
/// - `start_position`: the position in `old_text` where replacement begins (1-based character index).
/// - `num_chars`: the number of characters to replace.
/// - `new_text`: the text that will replace characters in `old_text`.
///   Returns the modified text with the specified portion replaced.
///   Returns an error if `start_position` < 1, `num_chars` < 0, or `start_position` exceeds the text length.
///   Handles Unicode characters correctly by counting characters, not bytes.
pub fn codcel_replace<S: AsRef<str>>(
    old_text: S,
    start_position: i32,
    num_chars: i32,
    new_text: S,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let old_text = old_text.as_ref();
    let new_text = new_text.as_ref();

    if start_position < 1 || num_chars < 0 {
        return Err("REPLACE: Invalid start position or number of characters.".into());
    }

    let start_position = start_position as usize - 1; // Convert to 0-based index

    // Use character-based indexing for proper Unicode support
    let chars: Vec<char> = old_text.chars().collect();

    if start_position > chars.len() {
        return Err("REPLACE: Start position is out of bounds.".into());
    }

    let end_position = std::cmp::min(start_position + num_chars as usize, chars.len());

    let before: String = chars[..start_position].iter().collect();
    let after: String = chars[end_position..].iter().collect();
    let result = format!("{}{}{}", before, new_text, after);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_basic() {
        // =REPLACE("abcdefghijk", 6, 5, "XYZ") in US format
        // =REPLACE("abcdefghijk"; 6; 5; "XYZ") in German format
        let result = codcel_replace("abcdefghijk", 6, 5, "XYZ").unwrap();
        println!("{result}");
        assert_eq!(result, "abcdeXYZk");
    }

    #[test]
    fn test_replace_at_beginning() {
        // =REPLACE("abcdefghijk", 1, 3, "XYZ") in US format
        // =REPLACE("abcdefghijk"; 1; 3; "XYZ") in German format
        let result = codcel_replace("abcdefghijk", 1, 3, "XYZ").unwrap();
        println!("{result}");
        assert_eq!(result, "XYZdefghijk");
    }

    #[test]
    fn test_replace_at_end() {
        // =REPLACE("abcdefghijk", 9, 3, "XYZ") in US format
        // =REPLACE("abcdefghijk"; 9; 3; "XYZ") in German format
        let result = codcel_replace("abcdefghijk", 9, 3, "XYZ").unwrap();
        println!("{result}");
        assert_eq!(result, "abcdefghXYZ");
    }

    #[test]
    fn test_replace_entire_string() {
        // =REPLACE("abcdefghijk", 1, 11, "XYZ") in US format
        // =REPLACE("abcdefghijk"; 1; 11; "XYZ") in German format
        let result = codcel_replace("abcdefghijk", 1, 11, "XYZ").unwrap();
        println!("{result}");
        assert_eq!(result, "XYZ");
    }

    #[test]
    fn test_replace_with_empty_string() {
        // =REPLACE("abcdefghijk", 4, 3, "") in US format
        // =REPLACE("abcdefghijk"; 4; 3; "") in German format
        let result = codcel_replace("abcdefghijk", 4, 3, "").unwrap();
        println!("{result}");
        assert_eq!(result, "abcghijk");
    }

    #[test]
    fn test_replace_zero_chars() {
        // =REPLACE("abcdefghijk", 4, 0, "XYZ") in US format
        // =REPLACE("abcdefghijk"; 4; 0; "XYZ") in German format
        let result = codcel_replace("abcdefghijk", 4, 0, "XYZ").unwrap();
        println!("{result}");
        assert_eq!(result, "abcXYZdefghijk");
    }

    #[test]
    fn test_replace_invalid_start_position() {
        // This should return an error
        let result = codcel_replace("abcdefghijk", 0, 3, "XYZ");
        assert!(result.is_err());
    }

    #[test]
    fn test_replace_negative_num_chars() {
        // This should return an error
        let result = codcel_replace("abcdefghijk", 4, -1, "XYZ");
        assert!(result.is_err());
    }

    #[test]
    fn test_replace_start_position_out_of_bounds() {
        // This should return an error
        let result = codcel_replace("abcdefghijk", 15, 3, "XYZ");
        assert!(result.is_err());
    }

    #[test]
    fn test_replace_cjk() {
        // =REPLACE("中国香港", 2, 1, "X") -> "中X香港"
        let result = codcel_replace("中国香港", 2, 1, "X").unwrap();
        assert_eq!(result, "中X香港");
    }

    #[test]
    fn test_replace_cjk_multiple() {
        // =REPLACE("中国香港", 2, 2, "XX") -> "中XX港"
        let result = codcel_replace("中国香港", 2, 2, "XX").unwrap();
        assert_eq!(result, "中XX港");
    }

    #[test]
    fn test_replace_cjk_at_end() {
        // =REPLACE("中国香港", 3, 2, "XY") -> "中国XY"
        let result = codcel_replace("中国香港", 3, 2, "XY").unwrap();
        assert_eq!(result, "中国XY");
    }

    #[test]
    fn test_replace_emoji() {
        // =REPLACE("😀😃😄", 2, 1, "X") -> "😀X😄"
        let result = codcel_replace("😀😃😄", 2, 1, "X").unwrap();
        assert_eq!(result, "😀X😄");
    }

    #[test]
    fn test_replace_mixed_ascii_unicode() {
        // =REPLACE("Hello中国", 6, 1, "X") -> "HelloX国"
        let result = codcel_replace("Hello中国", 6, 1, "X").unwrap();
        assert_eq!(result, "HelloX国");
    }

    #[test]
    fn test_replace_zero_chars_cjk() {
        // =REPLACE("中国香港", 3, 0, "X") -> "中国X香港"
        let result = codcel_replace("中国香港", 3, 0, "X").unwrap();
        assert_eq!(result, "中国X香港");
    }
}
