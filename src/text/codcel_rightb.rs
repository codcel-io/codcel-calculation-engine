// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `RIGHTB` that returns characters from the end of a text string based on byte count.
/// - `text`: the text string from which to extract characters.
/// - `num_bytes`: optional number of bytes to extract (default 1).
///   Returns the rightmost characters from the text that fit within the specified byte count.
///   If `num_bytes` exceeds the text byte length, returns the entire text.
///   When `num_bytes` falls in the middle of a multi-byte UTF-8 character, truncates to the
///   next complete character that fits within the byte count (from the right).
///   Returns an error if `num_bytes` is negative.
///   Unlike RIGHT which counts characters, RIGHTB counts bytes. For ASCII text, results are identical.
pub fn codcel_rightb<S: AsRef<str>>(
    text: S,
    num_bytes: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text_ref = text.as_ref();
    let bytes_to_take = num_bytes.unwrap_or(1);

    if bytes_to_take < 0 {
        return Err("RIGHTB: The number of bytes must be non-negative.".into());
    }

    let bytes_to_take = bytes_to_take as usize;

    if bytes_to_take >= text_ref.len() {
        return Ok(text_ref.to_string());
    }

    // Find the first valid UTF-8 character boundary at or after (len - bytes_to_take)
    let mut start = text_ref.len() - bytes_to_take;
    while start < text_ref.len() && !text_ref.is_char_boundary(start) {
        start += 1;
    }

    Ok(text_ref[start..].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rightb_default() {
        // =RIGHTB("Hello") -> "o" (1 byte default)
        let result = codcel_rightb("Hello", None).unwrap();
        assert_eq!(result, "o");
    }

    #[test]
    fn test_rightb_ascii() {
        // =RIGHTB("Hello", 3) -> "llo"
        let result = codcel_rightb("Hello", Some(3)).unwrap();
        assert_eq!(result, "llo");
    }

    #[test]
    fn test_rightb_zero_bytes() {
        // =RIGHTB("Hello", 0) -> ""
        let result = codcel_rightb("Hello", Some(0)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_rightb_exceeds_length() {
        // =RIGHTB("Hello", 10) -> "Hello" (5 bytes total)
        let result = codcel_rightb("Hello", Some(10)).unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_rightb_empty_string() {
        // =RIGHTB("", 3) -> ""
        let result = codcel_rightb("", Some(3)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_rightb_negative_bytes() {
        // =RIGHTB("Hello", -1) -> #VALUE! error
        let result = codcel_rightb("Hello", Some(-1));
        assert!(result.is_err());
    }

    #[test]
    fn test_rightb_cjk_full_character() {
        // "中国香港": 中=3 bytes, 国=3 bytes, 香=3 bytes, 港=3 bytes (12 bytes total)
        // =RIGHTB("中国香港", 6) -> "香港" (exactly 6 bytes = 2 CJK chars from right)
        let result = codcel_rightb("中国香港", Some(6)).unwrap();
        assert_eq!(result, "香港");
    }

    #[test]
    fn test_rightb_cjk_mid_character_truncation() {
        // =RIGHTB("中国香港", 4) -> "港" (4 bytes requested from right, but start lands
        //   mid-character, so advance to next boundary → only "港" = 3 bytes)
        let result = codcel_rightb("中国香港", Some(4)).unwrap();
        assert_eq!(result, "港");
    }

    #[test]
    fn test_rightb_cjk_mid_character_truncation_5_bytes() {
        // =RIGHTB("中国香港", 5) -> "港" (5 bytes from right, start at byte 7 which is
        //   mid-"香", advance to byte 9 → only "港")
        let result = codcel_rightb("中国香港", Some(5)).unwrap();
        assert_eq!(result, "港");
    }

    #[test]
    fn test_rightb_emoji() {
        // "😀😃😄": each emoji is 4 UTF-8 bytes (12 bytes total, 3 chars)
        // =RIGHTB("😀😃😄", 4) -> "😄" (exactly 4 bytes = 1 emoji from right)
        let result = codcel_rightb("😀😃😄", Some(4)).unwrap();
        assert_eq!(result, "😄");
    }

    #[test]
    fn test_rightb_emoji_mid_character() {
        // =RIGHTB("😀😃😄", 5) -> "😄" (5 bytes from right, start at byte 7 which is
        //   mid-"😃", advance to byte 8 → only "😄")
        let result = codcel_rightb("😀😃😄", Some(5)).unwrap();
        assert_eq!(result, "😄");
    }

    #[test]
    fn test_rightb_emoji_two_chars() {
        // =RIGHTB("😀😃😄", 8) -> "😃😄" (exactly 8 bytes = 2 emojis from right)
        let result = codcel_rightb("😀😃😄", Some(8)).unwrap();
        assert_eq!(result, "😃😄");
    }

    #[test]
    fn test_rightb_mixed_ascii_unicode() {
        // "Hello中国": "Hello" = 5 bytes, "中" = 3 bytes, "国" = 3 bytes (11 bytes total)
        // =RIGHTB("Hello中国", 7) -> "o中国" would be 7 bytes if boundary aligned
        // start = 11 - 7 = 4, byte 4 is 'o' boundary → "o中国"
        let result = codcel_rightb("Hello中国", Some(7)).unwrap();
        assert_eq!(result, "o中国");
    }

    #[test]
    fn test_rightb_mixed_ascii_unicode_exact() {
        // =RIGHTB("Hello中国", 6) -> "中国" (3 + 3 = 6 bytes from right)
        let result = codcel_rightb("Hello中国", Some(6)).unwrap();
        assert_eq!(result, "中国");
    }

    #[test]
    fn test_rightb_one_byte_cjk() {
        // =RIGHTB("中国", 1) -> "" (1 byte from right, start at byte 5 which is mid-"国",
        //   advance to byte 6 which is end → empty string)
        let result = codcel_rightb("中国", Some(1)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_rightb_two_bytes_cjk() {
        // =RIGHTB("中国", 2) -> "" (2 bytes from right, start at byte 4 which is mid-"国",
        //   advance to byte 6 → empty)
        let result = codcel_rightb("中国", Some(2)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_rightb_three_bytes_cjk() {
        // =RIGHTB("中国", 3) -> "国" (exactly 3 bytes = 1 CJK char from right)
        let result = codcel_rightb("中国", Some(3)).unwrap();
        assert_eq!(result, "国");
    }
}
