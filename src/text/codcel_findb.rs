// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `FINDB` that locates one text string within another using byte positions (case-sensitive).
/// - `substring`: the text to find.
/// - `text`: the text in which to search.
/// - `start_position`: optional byte position to start searching (default 1, 1-based index).
///   Returns the byte position of the first occurrence of `substring` within `text` (1-based).
///   Returns an error if `substring` is not found or if `start_position` exceeds the text byte length.
///   Unlike FIND which counts characters, FINDB counts bytes. For ASCII text, results are identical.
///   Like FIND, FINDB is case-sensitive and does not support wildcards.
pub fn codcel_findb(
    substring: &str,
    text: &str,
    start_position: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let start_position = start_position.unwrap_or(1);
    let start_position = start_position as usize;

    // Excel: empty substring returns start_position (FINDB("","") = 1, FINDB("","a",2) = 2)
    if substring.is_empty() {
        if start_position > text.len() + 1 {
            return Err("Error: FINDB start position exceeds the text byte length.".into());
        }
        return Ok(start_position as i32);
    }

    if start_position > text.len() {
        return Err("Error: FINDB start position exceeds the text byte length.".into());
    }

    // Adjust the text to start from the specified byte position
    let adjusted_text = &text[start_position - 1..];

    // Find the first occurrence of the substring in the adjusted text (byte offset)
    if let Some(pos) = adjusted_text.find(substring) {
        Ok((pos + start_position) as i32) // 1-based byte index
    } else {
        Err("#VALUE! Substring not found.".into()) // Match not found
    }
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
        // FINDB("国", "中国香港") — "中" is 3 UTF-8 bytes, so "国" starts at byte 4
        let result = codcel_findb("国", "中国香港", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_findb_mixed_ascii_unicode() {
        // FINDB("中", "Hello中国") — "Hello" is 5 bytes, so "中" starts at byte 6
        let result = codcel_findb("中", "Hello中国", None).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_findb_unicode_with_start_position() {
        // FINDB("港", "中国香港", 7) — "中"=3, "国"=3, "香"=3, "港" starts at byte 10
        let result = codcel_findb("港", "中国香港", Some(7)).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_findb_case_sensitive() {
        let result = codcel_findb("h", "Hello", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_findb_empty_in_empty() {
        // =FINDB("", "") returns 1 in Excel
        let result = codcel_findb("", "", None).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_findb_empty_with_start_position() {
        // =FINDB("", "abc", 2) returns 2 in Excel
        let result = codcel_findb("", "abc", Some(2)).unwrap();
        assert_eq!(result, 2);
    }
}
