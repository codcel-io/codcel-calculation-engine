// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `REPLACEB` that replaces part of a text string based on byte positions.
/// - `old_text`: the original text to modify.
/// - `start_position`: the 1-based byte position where replacement begins.
/// - `num_bytes`: the number of bytes to replace.
/// - `new_text`: the text that will replace the specified bytes in `old_text`.
///   Returns the modified text with the specified byte range replaced.
///   If `start_position` falls in the middle of a multi-byte UTF-8 character, the start is
///   adjusted forward to the next valid character boundary.
///   If the byte range ends in the middle of a multi-byte character, truncates to the last
///   complete character that fits within the range.
///   Returns an error if `start_position` < 1, `num_bytes` < 0, or `start_position` exceeds the text byte length.
///   Unlike REPLACE which counts characters, REPLACEB counts bytes. For ASCII text, results are identical.
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

    let start_byte = start_position as usize - 1; // Convert to 0-based index
    let num_bytes = num_bytes as usize;

    if start_byte > old_text.len() {
        return Err("REPLACEB: Start position is out of bounds.".into());
    }

    // Find the next valid UTF-8 character boundary at or after start_byte
    let mut start = start_byte;
    while start < old_text.len() && !old_text.is_char_boundary(start) {
        start += 1;
    }

    // Calculate the end byte position
    let end_byte = std::cmp::min(start_byte + num_bytes, old_text.len());

    // Find the last valid UTF-8 character boundary at or before end_byte
    let mut end = end_byte;
    while end > 0 && !old_text.is_char_boundary(end) {
        end -= 1;
    }

    // If end moved before start, use start as the end (replace nothing)
    if end < start {
        end = start;
    }

    let result = format!("{}{}{}", &old_text[..start], new_text, &old_text[end..]);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replaceb_basic_ascii() {
        // =REPLACEB("abcdefghijk", 6, 5, "XYZ") -> "abcdeXYZk"
        let result = codcel_replaceb("abcdefghijk", 6, 5, "XYZ").unwrap();
        assert_eq!(result, "abcdeXYZk");
    }

    #[test]
    fn test_replaceb_at_beginning() {
        // =REPLACEB("abcdefghijk", 1, 3, "XYZ") -> "XYZdefghijk"
        let result = codcel_replaceb("abcdefghijk", 1, 3, "XYZ").unwrap();
        assert_eq!(result, "XYZdefghijk");
    }

    #[test]
    fn test_replaceb_at_end() {
        // =REPLACEB("abcdefghijk", 9, 3, "XYZ") -> "abcdefghXYZ"
        let result = codcel_replaceb("abcdefghijk", 9, 3, "XYZ").unwrap();
        assert_eq!(result, "abcdefghXYZ");
    }

    #[test]
    fn test_replaceb_entire_string() {
        // =REPLACEB("abcdefghijk", 1, 11, "XYZ") -> "XYZ"
        let result = codcel_replaceb("abcdefghijk", 1, 11, "XYZ").unwrap();
        assert_eq!(result, "XYZ");
    }

    #[test]
    fn test_replaceb_with_empty_string() {
        // =REPLACEB("abcdefghijk", 4, 3, "") -> "abcghijk"
        let result = codcel_replaceb("abcdefghijk", 4, 3, "").unwrap();
        assert_eq!(result, "abcghijk");
    }

    #[test]
    fn test_replaceb_zero_bytes() {
        // =REPLACEB("abcdefghijk", 4, 0, "XYZ") -> "abcXYZdefghijk"
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
        // "中国香港": 中=bytes 1-3, 国=bytes 4-6, 香=bytes 7-9, 港=bytes 10-12
        // =REPLACEB("中国香港", 4, 3, "X") replaces bytes 4-6 ("国") with "X"
        let result = codcel_replaceb("中国香港", 4, 3, "X").unwrap();
        assert_eq!(result, "中X香港");
    }

    #[test]
    fn test_replaceb_cjk_two_characters() {
        // =REPLACEB("中国香港", 4, 6, "X") replaces bytes 4-9 ("国香") with "X"
        let result = codcel_replaceb("中国香港", 4, 6, "X").unwrap();
        assert_eq!(result, "中X港");
    }

    #[test]
    fn test_replaceb_cjk_mid_character_start() {
        // =REPLACEB("中国香港", 2, 5, "X")
        // byte 2 is mid-"中", adjusts start forward to byte 4 (start of "国")
        // end = byte 2+5-1 = byte 6 (end of "国")
        // replaces "国" with "X"
        let result = codcel_replaceb("中国香港", 2, 5, "X").unwrap();
        assert_eq!(result, "中X香港");
    }

    #[test]
    fn test_replaceb_cjk_mid_character_end() {
        // =REPLACEB("中国香港", 4, 4, "X")
        // start = byte 4 (start of "国"), end = byte 7+1 = byte 8 (mid-"香")
        // end adjusts backward to byte 7 (start of "香"), so only "国" is replaced
        let result = codcel_replaceb("中国香港", 4, 4, "X").unwrap();
        assert_eq!(result, "中X香港");
    }

    #[test]
    fn test_replaceb_cjk_mid_character_end_5_bytes() {
        // =REPLACEB("中国香港", 4, 5, "X")
        // start = byte 4 (start of "国"), end = byte 8 (mid-"香")
        // end adjusts backward to byte 7 (start of "香"), so only "国" is replaced
        let result = codcel_replaceb("中国香港", 4, 5, "X").unwrap();
        assert_eq!(result, "中X香港");
    }

    #[test]
    fn test_replaceb_emoji() {
        // "😀😃😄": each emoji is 4 UTF-8 bytes
        // 😀=bytes 1-4, 😃=bytes 5-8, 😄=bytes 9-12
        // =REPLACEB("😀😃😄", 5, 4, "X") replaces "😃" with "X"
        let result = codcel_replaceb("😀😃😄", 5, 4, "X").unwrap();
        assert_eq!(result, "😀X😄");
    }

    #[test]
    fn test_replaceb_emoji_mid_character_start() {
        // =REPLACEB("😀😃😄", 3, 6, "X")
        // byte 3 is mid-"😀", adjusts start forward to byte 5 (start of "😃")
        // end = byte 3+6-1 = byte 8 (end of "😃")
        // replaces "😃" with "X"
        let result = codcel_replaceb("😀😃😄", 3, 6, "X").unwrap();
        assert_eq!(result, "😀X😄");
    }

    #[test]
    fn test_replaceb_emoji_mid_character_end() {
        // =REPLACEB("😀😃😄", 5, 5, "X")
        // start = byte 5 (start of "😃"), end = byte 9 (start of "😄")
        // end is on a boundary, so replaces "😃" with "X"
        let result = codcel_replaceb("😀😃😄", 5, 5, "X").unwrap();
        assert_eq!(result, "😀X😄");
    }

    #[test]
    fn test_replaceb_emoji_mid_character_end_partial() {
        // =REPLACEB("😀😃😄", 5, 6, "X")
        // start = byte 5 (start of "😃"), end = byte 10 (mid-"😄")
        // end adjusts backward to byte 9 (start of "😄"), so only "😃" is replaced
        let result = codcel_replaceb("😀😃😄", 5, 6, "X").unwrap();
        assert_eq!(result, "😀X😄");
    }

    #[test]
    fn test_replaceb_mixed_ascii_unicode() {
        // "Hello中国": H=1, e=2, l=3, l=4, o=5, 中=6-8, 国=9-11
        // =REPLACEB("Hello中国", 6, 3, "X") replaces "中" with "X"
        let result = codcel_replaceb("Hello中国", 6, 3, "X").unwrap();
        assert_eq!(result, "HelloX国");
    }

    #[test]
    fn test_replaceb_mixed_ascii_unicode_range() {
        // =REPLACEB("Hello中国", 4, 5, "X") replaces "lo中" (bytes 4-8) with "X"
        let result = codcel_replaceb("Hello中国", 4, 5, "X").unwrap();
        assert_eq!(result, "HelX国");
    }

    #[test]
    fn test_replaceb_empty_string() {
        // =REPLACEB("", 1, 0, "X") -> "X"
        let result = codcel_replaceb("", 1, 0, "X").unwrap();
        assert_eq!(result, "X");
    }

    #[test]
    fn test_replaceb_zero_bytes_cjk() {
        // =REPLACEB("中国香港", 4, 0, "X") inserts "X" before "国"
        let result = codcel_replaceb("中国香港", 4, 0, "X").unwrap();
        assert_eq!(result, "中X国香港");
    }

    #[test]
    fn test_replaceb_ascii_same_as_replace() {
        // For pure ASCII, REPLACEB and REPLACE should give the same results
        let result = codcel_replaceb("Hello World", 7, 5, "XYZ").unwrap();
        assert_eq!(result, "Hello XYZ");
    }

    #[test]
    fn test_replaceb_exceed_length() {
        // =REPLACEB("Hello", 3, 100, "X") replaces from byte 3 to end
        let result = codcel_replaceb("Hello", 3, 100, "X").unwrap();
        assert_eq!(result, "HeX");
    }
}
