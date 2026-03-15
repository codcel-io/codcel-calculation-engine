// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use regex::Regex;
use std::error::Error;

use super::codcel_search::build_wildcard_pattern;

/// Excel-compatible `SEARCHB` that locates one text string within another using byte positions (case-insensitive).
/// - `substring`: the text to find. Supports wildcards: `?` matches any single character,
///   `*` matches any sequence of characters. Use `~` to escape wildcards: `~?` for literal `?`,
///   `~*` for literal `*`, `~~` for literal `~`.
/// - `text`: the text in which to search.
/// - `start_position`: optional byte position to start searching (default 1, 1-based index).
///   Returns the byte position of the first occurrence of `substring` within `text` (1-based).
///   Returns an error if `substring` is not found or if `start_position` exceeds the text byte length.
///   Unlike SEARCH which counts characters, SEARCHB counts bytes. For ASCII text, results are identical.
///   Like SEARCH, SEARCHB is case-insensitive and supports wildcards.
pub fn codcel_searchb(
    substring: &str,
    text: &str,
    start_position: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let start_position = start_position.unwrap_or(1);
    let start_position = start_position as usize;

    // Excel: empty substring returns start_position (SEARCHB("","") = 1, SEARCHB("","a",2) = 2)
    if substring.is_empty() {
        if start_position > text.len() + 1 {
            return Err("Error: SEARCHB start position exceeds the text byte length.".into());
        }
        return Ok(start_position as i32);
    }

    if start_position > text.len() {
        return Err("Error: SEARCHB start position exceeds the text byte length.".into());
    }

    if !text.is_char_boundary(start_position - 1) {
        return Err("#VALUE! SEARCHB start position is not on a character boundary.".into());
    }

    let pattern = build_wildcard_pattern(substring);

    // Create regex with case-insensitivity
    let regex_str = format!("(?i){pattern}");
    let regex = Regex::new(&regex_str)?;

    // Slice text at byte offset (like FINDB)
    let adjusted_text = &text[start_position - 1..];

    // Find the first match and return its byte position
    if let Some(matched) = regex.find(adjusted_text) {
        Ok((matched.start() + start_position) as i32) // 1-based byte index
    } else {
        Err("#VALUE! Substring not found.".into()) // Match not found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_searchb_basic_ascii() {
        // For ASCII text, SEARCHB and SEARCH return the same result
        let result = codcel_searchb("e", "abcdefghijk", None).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_searchb_case_insensitive() {
        let result = codcel_searchb("E", "abcdefghijk", None).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_searchb_wildcard_question_mark() {
        let result = codcel_searchb("a?c", "abcdefgabc", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_searchb_wildcard_asterisk() {
        let result = codcel_searchb("a*c", "abcdefgabc", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_searchb_unicode_byte_position() {
        // SEARCHB("国", "中国香港") — "中" is 3 UTF-8 bytes, so "国" starts at byte 4
        // Compare: SEARCH returns 2 (character position)
        let result = codcel_searchb("国", "中国香港", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_searchb_mixed_ascii_unicode() {
        // SEARCHB("中", "Hello中国") — "Hello" is 5 bytes, so "中" starts at byte 6
        let result = codcel_searchb("中", "Hello中国", None).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_searchb_unicode_case_insensitive() {
        let result = codcel_searchb("HELLO", "hello中国", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_searchb_with_start_position_byte_based() {
        // Start at byte 4 (past "中" which is 3 bytes), search for "香"
        // "中国香港": 中=bytes 1-3, 国=bytes 4-6, 香=bytes 7-9, 港=bytes 10-12
        let result = codcel_searchb("香", "中国香港", Some(4)).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn test_searchb_not_found() {
        let result = codcel_searchb("z", "abcdefghijk", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_searchb_start_position_exceeds_byte_length() {
        let result = codcel_searchb("e", "abcdefghijk", Some(15));
        assert!(result.is_err());
    }

    #[test]
    fn test_searchb_empty_substring() {
        let result = codcel_searchb("", "abcdefghijk", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_searchb_special_regex_characters() {
        // "." should be treated literally, not as regex wildcard
        let result = codcel_searchb(".", "abc.def", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_searchb_wildcard_with_unicode() {
        // "?" should match any single CHARACTER (including multi-byte)
        // "?国" should match "中国" starting at byte 1
        let result = codcel_searchb("?国", "中国香港", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_searchb_tilde_escape_question_mark() {
        // ~? means literal question mark
        let result = codcel_searchb("~?", "abc?def", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_searchb_tilde_escape_asterisk() {
        // ~* means literal asterisk
        let result = codcel_searchb("~*", "abc*def", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_searchb_tilde_escape_tilde() {
        // ~~ means literal tilde
        let result = codcel_searchb("~~", "abc~def", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_searchb_with_start_position_ascii() {
        let result = codcel_searchb("e", "abcdefghijek", Some(6)).unwrap();
        assert_eq!(result, 11);
    }

    #[test]
    fn test_searchb_in_empty_text() {
        let result = codcel_searchb("a", "", None);
        assert!(result.is_err());
    }
}
