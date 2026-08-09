// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

use super::dbcs_utils::{dbcs_byte_len, dbcs_byte_pos_to_char_index_forward, dbcs_char_width};

/// Excel-compatible `MIDB` that returns characters from the middle of a text string based on DBCS byte positions.
/// - `text`: the text string containing the characters to extract.
/// - `start_position`: the 1-based DBCS byte position of the first byte to extract.
/// - `num_bytes`: the number of DBCS bytes to extract.
///   Returns the substring that fits within the specified DBCS byte range.
///   If `start_position` falls in the middle of a 2-byte (wide) character, the start is
///   adjusted forward to the next complete character.
///   If the byte range ends in the middle of a wide character, truncates to the last
///   complete character that fits within the range.
///   Returns an error if `start_position` < 1 or `num_bytes` < 0.
///   Unlike MID which counts characters, MIDB counts DBCS bytes. For non-CJK text, results are identical.
pub fn codcel_midb<S: AsRef<str>>(
    text: S,
    start_position: i32,
    num_bytes: i32,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref();
    if start_position < 1 || num_bytes < 0 {
        return Err("MIDB: Invalid start position or number of bytes.".into());
    }

    let dbcs_start = (start_position as usize) - 1; // Convert to 0-based
    let num_bytes = num_bytes as usize;
    let total_dbcs_len = dbcs_byte_len(text);

    if dbcs_start >= total_dbcs_len {
        return Ok(String::new());
    }

    // Find the character index for the start position (advances forward if mid-character)
    let start_char_idx = dbcs_byte_pos_to_char_index_forward(text, dbcs_start);
    let total_chars = text.chars().count();

    if start_char_idx >= total_chars {
        return Ok(String::new());
    }

    // Walk forward from start_char_idx, accumulating DBCS bytes up to num_bytes
    let mut accumulated = 0usize;
    let mut end_char_idx = start_char_idx;
    for c in text.chars().skip(start_char_idx) {
        let w = dbcs_char_width(c);
        if accumulated + w > num_bytes {
            break;
        }
        accumulated += w;
        end_char_idx += 1;
    }

    if end_char_idx <= start_char_idx {
        return Ok(String::new());
    }

    Ok(text
        .chars()
        .skip(start_char_idx)
        .take(end_char_idx - start_char_idx)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midb_basic_ascii() {
        let result = codcel_midb("Hello World", 7, 5).unwrap();
        assert_eq!(result, "World");
    }

    #[test]
    fn test_midb_single_byte() {
        let result = codcel_midb("Hello", 2, 1).unwrap();
        assert_eq!(result, "e");
    }

    #[test]
    fn test_midb_zero_bytes() {
        let result = codcel_midb("Hello", 2, 0).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_midb_beyond_end() {
        let result = codcel_midb("Hello", 2, 10).unwrap();
        assert_eq!(result, "ello");
    }

    #[test]
    fn test_midb_start_beyond_end() {
        let result = codcel_midb("Hello", 10, 2).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_midb_invalid_start() {
        let result = codcel_midb("Hello", 0, 3);
        assert!(result.is_err());
    }

    #[test]
    fn test_midb_negative_bytes() {
        let result = codcel_midb("Hello", 2, -1);
        assert!(result.is_err());
    }

    #[test]
    fn test_midb_cjk_full_character() {
        // "中国香港": 中=bytes 1-2, 国=bytes 3-4, 香=bytes 5-6, 港=bytes 7-8 (DBCS)
        // MIDB("中国香港", 3, 2) -> "国"
        let result = codcel_midb("中国香港", 3, 2).unwrap();
        assert_eq!(result, "国");
    }

    #[test]
    fn test_midb_cjk_two_characters() {
        // MIDB("中国香港", 3, 4) -> "国香" (bytes 3-6)
        let result = codcel_midb("中国香港", 3, 4).unwrap();
        assert_eq!(result, "国香");
    }

    #[test]
    fn test_midb_cjk_start_mid_character() {
        // MIDB("中国香港", 2, 4) -> byte 2 is mid-中 (中 spans bytes 1-2), advances to 国 at byte 3
        // From 国: 国=2 bytes, 香=2 bytes -> total 4 bytes -> "国香"
        let result = codcel_midb("中国香港", 2, 4).unwrap();
        assert_eq!(result, "国香");
    }

    #[test]
    fn test_midb_cjk_end_mid_character() {
        // MIDB("中国香港", 3, 3) -> start at 国 (byte 3), 国=2 bytes, 香 needs 2 more = 4, truncate
        // Result: "国"
        let result = codcel_midb("中国香港", 3, 3).unwrap();
        assert_eq!(result, "国");
    }

    #[test]
    fn test_midb_emoji() {
        // Emoji are 1 DBCS byte each: 😀=byte 1, 😃=byte 2, 😄=byte 3
        // MIDB("😀😃😄", 2, 1) -> "😃"
        let result = codcel_midb("😀😃😄", 2, 1).unwrap();
        assert_eq!(result, "😃");
    }

    #[test]
    fn test_midb_mixed_ascii_unicode() {
        // "Hello中国": H=1, e=2, l=3, l=4, o=5, 中=6-7, 国=8-9 (DBCS)
        // MIDB("Hello中国", 6, 2) -> "中"
        let result = codcel_midb("Hello中国", 6, 2).unwrap();
        assert_eq!(result, "中");
    }

    #[test]
    fn test_midb_mixed_ascii_unicode_range() {
        // MIDB("Hello中国", 4, 4) -> "lo中" (l=1, o=1, 中=2 = 4 bytes)
        let result = codcel_midb("Hello中国", 4, 4).unwrap();
        assert_eq!(result, "lo中");
    }

    #[test]
    fn test_midb_empty_string() {
        let result = codcel_midb("", 1, 3).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_midb_start_at_one() {
        let result = codcel_midb("Hello", 1, 3).unwrap();
        assert_eq!(result, "Hel");
    }

    #[test]
    fn test_midb_cjk_all_bytes_mid_character() {
        // MIDB("中国", 2, 1) -> byte 2 is mid-中, advances to 国 at byte 3, but only 1 byte available
        // 国 needs 2 bytes, doesn't fit -> ""
        let result = codcel_midb("中国", 2, 1).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_midb_accented_latin() {
        // Accented chars are 1 DBCS byte each
        // MIDB("Héllo", 2, 3) -> "éll"
        let result = codcel_midb("Héllo", 2, 3).unwrap();
        assert_eq!(result, "éll");
    }
}
