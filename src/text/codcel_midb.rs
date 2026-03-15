// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `MIDB` that returns characters from the middle of a text string based on byte positions.
/// - `text`: the text string containing the characters to extract.
/// - `start_position`: the 1-based byte position of the first byte to extract.
/// - `num_bytes`: the number of bytes to extract.
///   Returns the substring that fits within the specified byte range.
///   If `start_position` falls in the middle of a multi-byte UTF-8 character, the start is
///   adjusted forward to the next valid character boundary.
///   If the byte range ends in the middle of a multi-byte character, truncates to the last
///   complete character that fits within the range.
///   Returns an error if `start_position` < 1 or `num_bytes` < 0.
///   Unlike MID which counts characters, MIDB counts bytes. For ASCII text, results are identical.
pub fn codcel_midb<S: AsRef<str>>(
    text: S,
    start_position: i32,
    num_bytes: i32,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref();
    if start_position < 1 || num_bytes < 0 {
        return Err("MIDB: Invalid start position or number of bytes.".into());
    }

    let start_byte = start_position as usize - 1; // Convert to 0-based index
    let num_bytes = num_bytes as usize;

    if start_byte >= text.len() {
        return Ok(String::new());
    }

    // Find the next valid UTF-8 character boundary at or after start_byte
    let mut start = start_byte;
    while start < text.len() && !text.is_char_boundary(start) {
        start += 1;
    }

    if start >= text.len() {
        return Ok(String::new());
    }

    // Calculate the end byte position
    let end_byte = std::cmp::min(start_byte + num_bytes, text.len());

    // Find the last valid UTF-8 character boundary at or before end_byte
    let mut end = end_byte;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }

    if end <= start {
        return Ok(String::new());
    }

    Ok(text[start..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midb_basic_ascii() {
        // =MIDB("Hello World", 7, 5) -> "World"
        let result = codcel_midb("Hello World", 7, 5).unwrap();
        assert_eq!(result, "World");
    }

    #[test]
    fn test_midb_single_byte() {
        // =MIDB("Hello", 2, 1) -> "e"
        let result = codcel_midb("Hello", 2, 1).unwrap();
        assert_eq!(result, "e");
    }

    #[test]
    fn test_midb_zero_bytes() {
        // =MIDB("Hello", 2, 0) -> ""
        let result = codcel_midb("Hello", 2, 0).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_midb_beyond_end() {
        // =MIDB("Hello", 2, 10) -> "ello"
        let result = codcel_midb("Hello", 2, 10).unwrap();
        assert_eq!(result, "ello");
    }

    #[test]
    fn test_midb_start_beyond_end() {
        // =MIDB("Hello", 10, 2) -> ""
        let result = codcel_midb("Hello", 10, 2).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_midb_invalid_start() {
        // =MIDB("Hello", 0, 3) -> error
        let result = codcel_midb("Hello", 0, 3);
        assert!(result.is_err());
    }

    #[test]
    fn test_midb_negative_bytes() {
        // =MIDB("Hello", 2, -1) -> error
        let result = codcel_midb("Hello", 2, -1);
        assert!(result.is_err());
    }

    #[test]
    fn test_midb_cjk_full_character() {
        // "中国香港": 中=3 bytes(1-3), 国=3 bytes(4-6), 香=3 bytes(7-9), 港=3 bytes(10-12)
        // =MIDB("中国香港", 4, 3) -> "国" (bytes 4-6)
        let result = codcel_midb("中国香港", 4, 3).unwrap();
        assert_eq!(result, "国");
    }

    #[test]
    fn test_midb_cjk_two_characters() {
        // =MIDB("中国香港", 4, 6) -> "国香" (bytes 4-9)
        let result = codcel_midb("中国香港", 4, 6).unwrap();
        assert_eq!(result, "国香");
    }

    #[test]
    fn test_midb_cjk_start_mid_character() {
        // =MIDB("中国香港", 2, 5) -> "国" (byte 2 is mid-"中", skips to byte 4; bytes 4-6 = "国")
        let result = codcel_midb("中国香港", 2, 5).unwrap();
        assert_eq!(result, "国");
    }

    #[test]
    fn test_midb_cjk_end_mid_character() {
        // =MIDB("中国香港", 4, 4) -> "国" (bytes 4-7, but byte 7 is start of "香",
        //   only "国" (3 bytes) fits completely)
        let result = codcel_midb("中国香港", 4, 4).unwrap();
        assert_eq!(result, "国");
    }

    #[test]
    fn test_midb_cjk_end_mid_character_5_bytes() {
        // =MIDB("中国香港", 4, 5) -> "国" (bytes 4-8, byte 8 is mid-"香",
        //   only "国" (3 bytes) fits completely)
        let result = codcel_midb("中国香港", 4, 5).unwrap();
        assert_eq!(result, "国");
    }

    #[test]
    fn test_midb_emoji() {
        // "😀😃😄": each emoji is 4 UTF-8 bytes
        // 😀=bytes 1-4, 😃=bytes 5-8, 😄=bytes 9-12
        // =MIDB("😀😃😄", 5, 4) -> "😃" (bytes 5-8)
        let result = codcel_midb("😀😃😄", 5, 4).unwrap();
        assert_eq!(result, "😃");
    }

    #[test]
    fn test_midb_emoji_mid_character_start() {
        // =MIDB("😀😃😄", 3, 6) -> "😃" (byte 3 is mid-"😀", skips to byte 5;
        //   bytes 5-8 = "😃")
        let result = codcel_midb("😀😃😄", 3, 6).unwrap();
        assert_eq!(result, "😃");
    }

    #[test]
    fn test_midb_emoji_mid_character_end() {
        // =MIDB("😀😃😄", 5, 5) -> "😃" (bytes 5-9, byte 9 is start of "😄",
        //   only "😃" fits)
        let result = codcel_midb("😀😃😄", 5, 5).unwrap();
        assert_eq!(result, "😃");
    }

    #[test]
    fn test_midb_mixed_ascii_unicode() {
        // "Hello中国": H=1, e=2, l=3, l=4, o=5, 中=6-8, 国=9-11
        // =MIDB("Hello中国", 6, 3) -> "中" (bytes 6-8)
        let result = codcel_midb("Hello中国", 6, 3).unwrap();
        assert_eq!(result, "中");
    }

    #[test]
    fn test_midb_mixed_ascii_unicode_range() {
        // =MIDB("Hello中国", 4, 5) -> "lo中" (bytes 4-8: l=4, o=5, 中=6-8)
        let result = codcel_midb("Hello中国", 4, 5).unwrap();
        assert_eq!(result, "lo中");
    }

    #[test]
    fn test_midb_empty_string() {
        // =MIDB("", 1, 3) -> ""
        let result = codcel_midb("", 1, 3).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_midb_ascii_same_as_mid() {
        // For pure ASCII, MIDB and MID should give the same results
        let result = codcel_midb("Hello World", 7, 5).unwrap();
        assert_eq!(result, "World");
    }

    #[test]
    fn test_midb_start_at_one() {
        // =MIDB("Hello", 1, 3) -> "Hel"
        let result = codcel_midb("Hello", 1, 3).unwrap();
        assert_eq!(result, "Hel");
    }

    #[test]
    fn test_midb_cjk_all_bytes_mid_character() {
        // =MIDB("中国", 2, 1) -> "" (byte 2 is mid-"中", byte 3 still mid-"中",
        //   no complete character fits in 1 byte from position 2)
        let result = codcel_midb("中国", 2, 1).unwrap();
        assert_eq!(result, "");
    }
}
