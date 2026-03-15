// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `LEFTB` that returns characters from the start of a text string based on byte count.
/// - `text`: the text string from which to extract characters.
/// - `num_bytes`: optional number of bytes to extract (default 1).
///   Returns the leftmost characters from the text that fit within the specified byte count.
///   If `num_bytes` exceeds the text byte length, returns the entire text.
///   When `num_bytes` falls in the middle of a multi-byte UTF-8 character, truncates to the
///   last complete character that fits within the byte count.
///   Returns an error if `num_bytes` is negative.
///   Unlike LEFT which counts characters, LEFTB counts bytes. For ASCII text, results are identical.
pub fn codcel_leftb<S: AsRef<str>>(
    text: S,
    num_bytes: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text_ref = text.as_ref();
    let bytes_to_take = num_bytes.unwrap_or(1);

    if bytes_to_take < 0 {
        return Err("LEFTB: The number of bytes must be non-negative.".into());
    }

    let bytes_to_take = bytes_to_take as usize;

    if bytes_to_take >= text_ref.len() {
        return Ok(text_ref.to_string());
    }

    // Find the last valid UTF-8 character boundary at or before bytes_to_take
    let mut end = bytes_to_take;
    while end > 0 && !text_ref.is_char_boundary(end) {
        end -= 1;
    }

    Ok(text_ref[..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leftb_default() {
        // =LEFTB("Hello") -> "H" (1 byte default)
        let result = codcel_leftb("Hello", None).unwrap();
        assert_eq!(result, "H");
    }

    #[test]
    fn test_leftb_ascii() {
        // =LEFTB("Hello", 3) -> "Hel"
        let result = codcel_leftb("Hello", Some(3)).unwrap();
        assert_eq!(result, "Hel");
    }

    #[test]
    fn test_leftb_zero_bytes() {
        // =LEFTB("Hello", 0) -> ""
        let result = codcel_leftb("Hello", Some(0)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_leftb_exceeds_length() {
        // =LEFTB("Hello", 10) -> "Hello" (5 bytes total)
        let result = codcel_leftb("Hello", Some(10)).unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_leftb_empty_string() {
        // =LEFTB("", 3) -> ""
        let result = codcel_leftb("", Some(3)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_leftb_negative_bytes() {
        // =LEFTB("Hello", -1) -> #VALUE! error
        let result = codcel_leftb("Hello", Some(-1));
        assert!(result.is_err());
    }

    #[test]
    fn test_leftb_cjk_full_character() {
        // "中国香港": 中=3 bytes, 国=3 bytes, 香=3 bytes, 港=3 bytes (12 bytes total)
        // =LEFTB("中国香港", 6) -> "中国" (exactly 6 bytes = 2 CJK chars)
        let result = codcel_leftb("中国香港", Some(6)).unwrap();
        assert_eq!(result, "中国");
    }

    #[test]
    fn test_leftb_cjk_mid_character_truncation() {
        // =LEFTB("中国香港", 4) -> "中" (4 bytes requested, but "中国" = 6 bytes,
        //   so only "中" = 3 bytes fits completely)
        let result = codcel_leftb("中国香港", Some(4)).unwrap();
        assert_eq!(result, "中");
    }

    #[test]
    fn test_leftb_cjk_mid_character_truncation_5_bytes() {
        // =LEFTB("中国香港", 5) -> "中" (5 bytes requested, "中国" = 6 bytes,
        //   only "中" = 3 bytes fits)
        let result = codcel_leftb("中国香港", Some(5)).unwrap();
        assert_eq!(result, "中");
    }

    #[test]
    fn test_leftb_emoji() {
        // "😀😃😄": each emoji is 4 UTF-8 bytes (12 bytes total, 3 chars)
        // =LEFTB("😀😃😄", 4) -> "😀" (exactly 4 bytes = 1 emoji)
        let result = codcel_leftb("😀😃😄", Some(4)).unwrap();
        assert_eq!(result, "😀");
    }

    #[test]
    fn test_leftb_emoji_mid_character() {
        // =LEFTB("😀😃😄", 5) -> "😀" (5 bytes requested, next emoji needs 4 more,
        //   only 1 byte left so truncate to "😀")
        let result = codcel_leftb("😀😃😄", Some(5)).unwrap();
        assert_eq!(result, "😀");
    }

    #[test]
    fn test_leftb_emoji_two_chars() {
        // =LEFTB("😀😃😄", 8) -> "😀😃" (exactly 8 bytes = 2 emojis)
        let result = codcel_leftb("😀😃😄", Some(8)).unwrap();
        assert_eq!(result, "😀😃");
    }

    #[test]
    fn test_leftb_mixed_ascii_unicode() {
        // "Hello中国": "Hello" = 5 bytes, "中" = bytes 5-7, "国" = bytes 8-10 (11 bytes total)
        // =LEFTB("Hello中国", 7) -> "Hello" (byte 7 is mid-"中", boundary at 5)
        let result = codcel_leftb("Hello中国", Some(7)).unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_leftb_mixed_ascii_unicode_exact() {
        // =LEFTB("Hello中国", 8) -> "Hello中" (5 + 3 = 8 bytes exactly)
        let result = codcel_leftb("Hello中国", Some(8)).unwrap();
        assert_eq!(result, "Hello中");
    }

    #[test]
    fn test_leftb_one_byte_cjk() {
        // =LEFTB("中国", 1) -> "" (1 byte is mid-"中", no complete char fits)
        let result = codcel_leftb("中国", Some(1)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_leftb_two_bytes_cjk() {
        // =LEFTB("中国", 2) -> "" (2 bytes is still mid-"中", boundary at 0)
        let result = codcel_leftb("中国", Some(2)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_leftb_three_bytes_cjk() {
        // =LEFTB("中国", 3) -> "中" (exactly 3 bytes = 1 CJK char)
        let result = codcel_leftb("中国", Some(3)).unwrap();
        assert_eq!(result, "中");
    }
}
