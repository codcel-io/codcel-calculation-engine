// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use regex::Regex;
use std::error::Error;

use super::codcel_search::build_wildcard_pattern;
use super::dbcs_utils::{char_index_to_dbcs_byte_pos, dbcs_byte_len, dbcs_byte_pos_to_char_index_forward};

/// Excel-compatible `SEARCHB` that locates one text string within another using DBCS byte positions (case-insensitive).
/// - `substring`: the text to find. Supports wildcards: `?` matches any single character,
///   `*` matches any sequence of characters. Use `~` to escape wildcards: `~?` for literal `?`,
///   `~*` for literal `*`, `~~` for literal `~`.
/// - `text`: the text in which to search.
/// - `start_position`: optional DBCS byte position to start searching (default 1, 1-based index).
///   Returns the DBCS byte position of the first occurrence of `substring` within `text` (1-based).
///   Returns an error if `substring` is not found or if `start_position` exceeds the text DBCS byte length.
///   Unlike SEARCH which counts characters, SEARCHB counts DBCS bytes. For non-CJK text, results are identical.
///   Like SEARCH, SEARCHB is case-insensitive and supports wildcards.
pub fn codcel_searchb(
    substring: &str,
    text: &str,
    start_position: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let start_position = start_position.unwrap_or(1);
    let dbcs_start = start_position as usize;
    let total_dbcs_len = dbcs_byte_len(text);

    // Excel: empty substring returns start_position
    if substring.is_empty() {
        if dbcs_start > total_dbcs_len + 1 {
            return Err("Error: SEARCHB start position exceeds the text byte length.".into());
        }
        return Ok(start_position);
    }

    if dbcs_start > total_dbcs_len {
        return Err("Error: SEARCHB start position exceeds the text byte length.".into());
    }

    // Convert DBCS start position to character index
    let start_char_idx = dbcs_byte_pos_to_char_index_forward(text, dbcs_start - 1);

    // Get the text from start_char_idx onwards as a string
    let adjusted_text: String = text.chars().skip(start_char_idx).collect();

    let pattern = build_wildcard_pattern(substring);
    let regex_str = format!("(?i){pattern}");
    let regex = Regex::new(&regex_str)?;

    if let Some(matched) = regex.find(&adjusted_text) {
        // matched.start() is a UTF-8 byte offset in adjusted_text
        // We need to convert that to a character index, then to a DBCS byte position in the full text
        let match_char_offset = adjusted_text[..matched.start()].chars().count();
        let match_char_idx = start_char_idx + match_char_offset;
        let dbcs_pos = char_index_to_dbcs_byte_pos(text, match_char_idx);
        Ok((dbcs_pos + 1) as i32) // 1-based
    } else {
        Err("#VALUE! Substring not found.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_searchb_basic_ascii() {
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
        // "中国香港": 中=1-2, 国=3-4 (DBCS)
        // SEARCHB("国", "中国香港") -> 3
        let result = codcel_searchb("国", "中国香港", None).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_searchb_mixed_ascii_unicode() {
        // "Hello中国": H=1, e=2, l=3, l=4, o=5, 中=6-7 (DBCS)
        // SEARCHB("中", "Hello中国") -> 6
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
        // "中国香港": 中=1-2, 国=3-4, 香=5-6, 港=7-8 (DBCS)
        // Start at DBCS byte 3 (国), search for "香" -> 5
        let result = codcel_searchb("香", "中国香港", Some(3)).unwrap();
        assert_eq!(result, 5);
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
        let result = codcel_searchb(".", "abc.def", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_searchb_wildcard_with_unicode() {
        // "?" matches any single character (including multi-byte)
        // "?国" should match "中国" starting at DBCS byte 1
        let result = codcel_searchb("?国", "中国香港", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_searchb_tilde_escape_question_mark() {
        let result = codcel_searchb("~?", "abc?def", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_searchb_tilde_escape_asterisk() {
        let result = codcel_searchb("~*", "abc*def", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_searchb_tilde_escape_tilde() {
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
