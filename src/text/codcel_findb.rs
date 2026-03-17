// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

use super::dbcs_utils::{char_index_to_dbcs_byte_pos, dbcs_byte_len, dbcs_byte_pos_to_char_index_forward};

/// Excel-compatible `FINDB` that locates one text string within another using DBCS byte positions (case-sensitive).
/// - `substring`: the text to find.
/// - `text`: the text in which to search.
/// - `start_position`: optional DBCS byte position to start searching (default 1, 1-based index).
///   Returns the DBCS byte position of the first occurrence of `substring` within `text` (1-based).
///   Returns an error if `substring` is not found or if `start_position` exceeds the text DBCS byte length.
///   Unlike FIND which counts characters, FINDB counts DBCS bytes. For non-CJK text, results are identical.
///   Like FIND, FINDB is case-sensitive and does not support wildcards.
pub fn codcel_findb(
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
            return Err("Error: FINDB start position exceeds the text byte length.".into());
        }
        return Ok(start_position);
    }

    if dbcs_start > total_dbcs_len {
        return Err("Error: FINDB start position exceeds the text byte length.".into());
    }

    // Convert DBCS start position to character index
    let start_char_idx = dbcs_byte_pos_to_char_index_forward(text, dbcs_start - 1);

    let text_chars: Vec<char> = text.chars().collect();
    let needle_chars: Vec<char> = substring.chars().collect();

    for i in start_char_idx..text_chars.len() {
        if text_chars[i..].starts_with(&needle_chars) {
            let dbcs_pos = char_index_to_dbcs_byte_pos(text, i);
            return Ok((dbcs_pos + 1) as i32); // 1-based
        }
    }

    Err("#VALUE! Substring not found.".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_findb_basic() {
        let result = codcel_findb("l", "Hello", None).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_findb_with_start_position() {
        let result = codcel_findb("l", "Hello", Some(4)).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_findb_at_start() {
        let result = codcel_findb("H", "Hello", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_findb_at_end() {
        let result = codcel_findb("o", "Hello", None).unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_findb_substring_not_found() {
        let result = codcel_findb("z", "Hello", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_findb_start_position_exceeds_length() {
        let result = codcel_findb("l", "Hello", Some(10));
        assert!(result.is_err());
    }

    #[test]
    fn test_findb_empty_substring() {
        let result = codcel_findb("", "Hello", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_findb_in_empty_text() {
        let result = codcel_findb("a", "", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_findb_unicode() {
        // "中国香港": 中=bytes 1-2, 国=bytes 3-4 (DBCS)
        // FINDB("国", "中国香港") -> 3
        let result = codcel_findb("国", "中国香港", None).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_findb_mixed_ascii_unicode() {
        // "Hello中国": H=1, e=2, l=3, l=4, o=5, 中=6-7 (DBCS)
        // FINDB("中", "Hello中国") -> 6
        let result = codcel_findb("中", "Hello中国", None).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_findb_unicode_with_start_position() {
        // "中国香港": 中=1-2, 国=3-4, 香=5-6, 港=7-8 (DBCS)
        // FINDB("港", "中国香港", 5) -> 7
        let result = codcel_findb("港", "中国香港", Some(5)).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn test_findb_case_sensitive() {
        let result = codcel_findb("h", "Hello", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_findb_empty_in_empty() {
        let result = codcel_findb("", "", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_findb_empty_with_start_position() {
        let result = codcel_findb("", "abc", Some(2)).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_findb_accented_latin() {
        // Accented chars are 1 DBCS byte each
        // FINDB("ö", "Héllo Wörld") -> 8
        let result = codcel_findb("ö", "Héllo Wörld", None).unwrap();
        assert_eq!(result, 8);
    }
}
