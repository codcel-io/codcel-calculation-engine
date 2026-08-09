// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

use super::dbcs_utils::{dbcs_byte_len, dbcs_byte_pos_to_char_index_forward, dbcs_char_width};

/// Excel-compatible `REPLACEB` that replaces part of a text string based on DBCS byte positions.
/// - `old_text`: the original text to modify.
/// - `start_position`: the 1-based DBCS byte position where replacement begins.
/// - `num_bytes`: the number of DBCS bytes to replace.
/// - `new_text`: the text that will replace the specified bytes in `old_text`.
///   Returns the modified text with the specified DBCS byte range replaced.
///   If `start_position` falls in the middle of a 2-byte (wide) character, the start is
///   adjusted forward to the next valid character.
///   If the byte range ends in the middle of a wide character, truncates to the last
///   complete character that fits within the range.
///   Returns an error if `start_position` < 1, `num_bytes` < 0, or `start_position` exceeds the text DBCS byte length.
///   Unlike REPLACE which counts characters, REPLACEB counts DBCS bytes. For non-CJK text, results are identical.
pub fn codcel_replaceb<S: AsRef<str>>(
    old_text: S,
    start_position: i32,
    num_bytes: i32,
    new_text: S,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let old_text = old_text.as_ref();
    let new_text = new_text.as_ref();

    if start_position < 1 || num_bytes < 0 {
        return Err("REPLACEB: Invalid start position or number of bytes.".into());
    }

    let dbcs_start = (start_position as usize) - 1; // 0-based
    let num_bytes = num_bytes as usize;
    let total_dbcs_len = dbcs_byte_len(old_text);

    if dbcs_start > total_dbcs_len {
        return Err("REPLACEB: Start position is out of bounds.".into());
    }

    // Find the character index for the start position
    let start_char_idx = dbcs_byte_pos_to_char_index_forward(old_text, dbcs_start);

    // Walk forward from start_char_idx, accumulating DBCS bytes up to num_bytes
    let mut accumulated = 0usize;
    let mut chars_to_replace = 0usize;
    for c in old_text.chars().skip(start_char_idx) {
        let w = dbcs_char_width(c);
        if accumulated + w > num_bytes {
            break;
        }
        accumulated += w;
        chars_to_replace += 1;
    }

    let before: String = old_text.chars().take(start_char_idx).collect();
    let after: String = old_text
        .chars()
        .skip(start_char_idx + chars_to_replace)
        .collect();

    Ok(format!("{before}{new_text}{after}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replaceb_basic_ascii() {
        let result = codcel_replaceb("abcdefghijk", 6, 5, "XYZ").unwrap();
        assert_eq!(result, "abcdeXYZk");
    }

    #[test]
    fn test_replaceb_at_beginning() {
        let result = codcel_replaceb("abcdefghijk", 1, 3, "XYZ").unwrap();
        assert_eq!(result, "XYZdefghijk");
    }

    #[test]
    fn test_replaceb_at_end() {
        let result = codcel_replaceb("abcdefghijk", 9, 3, "XYZ").unwrap();
        assert_eq!(result, "abcdefghXYZ");
    }

    #[test]
    fn test_replaceb_entire_string() {
        let result = codcel_replaceb("abcdefghijk", 1, 11, "XYZ").unwrap();
        assert_eq!(result, "XYZ");
    }

    #[test]
    fn test_replaceb_with_empty_string() {
        let result = codcel_replaceb("abcdefghijk", 4, 3, "").unwrap();
        assert_eq!(result, "abcghijk");
    }

    #[test]
    fn test_replaceb_zero_bytes() {
        let result = codcel_replaceb("abcdefghijk", 4, 0, "XYZ").unwrap();
        assert_eq!(result, "abcXYZdefghijk");
    }

    #[test]
    fn test_replaceb_invalid_start_position() {
        let result = codcel_replaceb("abcdefghijk", 0, 3, "XYZ");
        assert!(result.is_err());
    }

    #[test]
    fn test_replaceb_negative_num_bytes() {
        let result = codcel_replaceb("abcdefghijk", 4, -1, "XYZ");
        assert!(result.is_err());
    }

    #[test]
    fn test_replaceb_start_position_out_of_bounds() {
        let result = codcel_replaceb("abcdefghijk", 15, 3, "XYZ");
        assert!(result.is_err());
    }

    #[test]
    fn test_replaceb_cjk_full_character() {
        // "中国香港": 中=1-2, 国=3-4, 香=5-6, 港=7-8 (DBCS)
        // REPLACEB("中国香港", 3, 2, "X") replaces 国 with "X"
        let result = codcel_replaceb("中国香港", 3, 2, "X").unwrap();
        assert_eq!(result, "中X香港");
    }

    #[test]
    fn test_replaceb_cjk_two_characters() {
        // REPLACEB("中国香港", 3, 4, "X") replaces 国香 with "X"
        let result = codcel_replaceb("中国香港", 3, 4, "X").unwrap();
        assert_eq!(result, "中X港");
    }

    #[test]
    fn test_replaceb_cjk_mid_character_start() {
        // REPLACEB("中国香港", 2, 4, "X")
        // byte 2 is mid-中 (中 spans 1-2), advances to 国 at byte 3
        // From 国: 国=2 bytes, 香 needs 2 more = 4 total, but we only have 4 bytes budget
        // Wait: num_bytes=4, starting from 国: 国=2, 香=2, total=4 -> replaces "国香"
        let result = codcel_replaceb("中国香港", 2, 4, "X").unwrap();
        assert_eq!(result, "中X港");
    }

    #[test]
    fn test_replaceb_cjk_mid_character_end() {
        // REPLACEB("中国香港", 3, 3, "X")
        // start at 国 (byte 3), 国=2 bytes, 香 needs 2 more = 4 > 3, truncate
        // Only 国 is replaced
        let result = codcel_replaceb("中国香港", 3, 3, "X").unwrap();
        assert_eq!(result, "中X香港");
    }

    #[test]
    fn test_replaceb_emoji() {
        // Emoji are 1 DBCS byte each: 😀=1, 😃=2, 😄=3
        // REPLACEB("😀😃😄", 2, 1, "X") replaces 😃 with "X"
        let result = codcel_replaceb("😀😃😄", 2, 1, "X").unwrap();
        assert_eq!(result, "😀X😄");
    }

    #[test]
    fn test_replaceb_mixed_ascii_unicode() {
        // "Hello中国": H=1, e=2, l=3, l=4, o=5, 中=6-7, 国=8-9 (DBCS)
        // REPLACEB("Hello中国", 6, 2, "X") replaces 中 with "X"
        let result = codcel_replaceb("Hello中国", 6, 2, "X").unwrap();
        assert_eq!(result, "HelloX国");
    }

    #[test]
    fn test_replaceb_mixed_ascii_unicode_range() {
        // REPLACEB("Hello中国", 4, 4, "X") replaces "lo中" (l=1, o=1, 中=2 = 4 bytes)
        let result = codcel_replaceb("Hello中国", 4, 4, "X").unwrap();
        assert_eq!(result, "HelX国");
    }

    #[test]
    fn test_replaceb_empty_string() {
        let result = codcel_replaceb("", 1, 0, "X").unwrap();
        assert_eq!(result, "X");
    }

    #[test]
    fn test_replaceb_zero_bytes_cjk() {
        // REPLACEB("中国香港", 3, 0, "X") inserts "X" before 国
        let result = codcel_replaceb("中国香港", 3, 0, "X").unwrap();
        assert_eq!(result, "中X国香港");
    }

    #[test]
    fn test_replaceb_ascii_same_as_replace() {
        let result = codcel_replaceb("Hello World", 7, 5, "XYZ").unwrap();
        assert_eq!(result, "Hello XYZ");
    }

    #[test]
    fn test_replaceb_exceed_length() {
        let result = codcel_replaceb("Hello", 3, 100, "X").unwrap();
        assert_eq!(result, "HeX");
    }
}
