// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `FIND` that locates one text string within another (case-sensitive).
/// - `substring`: the text to find.
/// - `text`: the text in which to search.
/// - `start_position`: optional position to start searching (default 1, 1-based index).
///   Returns the position of the first occurrence of `substring` within `text` (1-based).
///   Returns an error if `substring` is not found or if `start_position` exceeds the text length.
///   Unlike SEARCH, FIND is case-sensitive and does not support wildcards.
pub fn codcel_find(
    substring: &str,
    text: &str,
    start_position: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let start_position = start_position.unwrap_or(1);
    let start_position = start_position as usize;
    let char_count = text.chars().count();

    // Excel: empty substring returns start_position (FIND("","") = 1, FIND("","abc",2) = 2)
    if substring.is_empty() {
        if start_position > char_count + 1 {
            return Err("Error: FIND start position exceeds the text length.".into());
        }
        return Ok(start_position as i32);
    }

    if start_position > char_count {
        return Err("Error: FIND start position exceeds the text length.".into());
    }

    // Adjust the text to start from the specified character position
    let byte_offset = text
        .char_indices()
        .nth(start_position - 1)
        .map_or(text.len(), |(i, _)| i);
    let adjusted_text = &text[byte_offset..];

    // Find the first occurrence of the substring in the adjusted text
    if let Some(byte_pos) = adjusted_text.find(substring) {
        // Convert byte offset to character offset
        let char_pos = adjusted_text[..byte_pos].chars().count();
        Ok((char_pos + start_position) as i32) // 1-based character index
    } else {
        Err("#VALUE! Substring not found.".into()) // Match not found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_basic() {
        // =FIND("l", "Hello") in US format
        // =FIND("l"; "Hello") in German format
        let result = codcel_find("l", "Hello", None).unwrap();
        println!("{result}");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_find_with_start_position() {
        // =FIND("l", "Hello", 4) in US format
        // =FIND("l"; "Hello"; 4) in German format
        let result = codcel_find("l", "Hello", Some(4)).unwrap();
        println!("{result}");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_find_multiple_occurrences() {
        // =FIND("o", "Hello World", 5) in US format
        // =FIND("o"; "Hello World"; 5) in German format
        let result = codcel_find("o", "Hello World", Some(5)).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_find_at_start() {
        // =FIND("H", "Hello") in US format
        // =FIND("H"; "Hello") in German format
        let result = codcel_find("H", "Hello", None).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_at_end() {
        // =FIND("o", "Hello") in US format
        // =FIND("o"; "Hello") in German format
        let result = codcel_find("o", "Hello", None).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_find_substring_not_found() {
        // =FIND("z", "Hello") in US format
        // =FIND("z"; "Hello") in German format
        let result = codcel_find("z", "Hello", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_start_position_exceeds_length() {
        // =FIND("l", "Hello", 10) in US format
        // =FIND("l"; "Hello"; 10) in German format
        let result = codcel_find("l", "Hello", Some(10));
        assert!(result.is_err());
    }

    #[test]
    fn test_find_empty_substring() {
        // =FIND("", "Hello") in US format
        // =FIND(""; "Hello") in German format
        let result = codcel_find("", "Hello", None).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_in_empty_text() {
        // =FIND("a", "") in US format
        // =FIND("a"; "") in German format
        let result = codcel_find("a", "", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_unicode() {
        // FIND("国", "中国香港") should return 2 (2nd character)
        let result = codcel_find("国", "中国香港", None).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_find_unicode_with_start_position() {
        // FIND("港", "中国香港", 3) should return 4 (4th character, starting search from 3rd)
        let result = codcel_find("港", "中国香港", Some(3)).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_find_mixed_ascii_unicode() {
        // FIND("中", "Hello中国") should return 6 (6th character)
        let result = codcel_find("中", "Hello中国", None).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_find_empty_in_empty() {
        // =FIND("", "") returns 1 in Excel
        let result = codcel_find("", "", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_find_empty_with_start_position() {
        // =FIND("", "abc", 2) returns 2 in Excel
        let result = codcel_find("", "abc", Some(2)).unwrap();
        assert_eq!(result, 2);
    }
}
