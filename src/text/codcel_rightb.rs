// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

use super::dbcs_utils::{dbcs_byte_len, dbcs_chars_fitting_in_bytes_from_right};

/// Excel-compatible `RIGHTB` that returns characters from the end of a text string based on DBCS byte count.
/// - `text`: the text string from which to extract characters.
/// - `num_bytes`: optional number of DBCS bytes to extract (default 1).
///   Returns the rightmost characters from the text that fit within the specified DBCS byte count.
///   If `num_bytes` exceeds the text DBCS byte length, returns the entire text.
///   When `num_bytes` falls in the middle of a 2-byte (wide) character, truncates to the
///   next complete character that fits within the byte count (from the right).
///   Returns an error if `num_bytes` is negative.
///   Unlike RIGHT which counts characters, RIGHTB counts DBCS bytes. For non-CJK text, results are identical.
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

    if bytes_to_take >= dbcs_byte_len(text_ref) {
        return Ok(text_ref.to_string());
    }

    let char_count = dbcs_chars_fitting_in_bytes_from_right(text_ref, bytes_to_take);
    let total_chars = text_ref.chars().count();
    Ok(text_ref.chars().skip(total_chars - char_count).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rightb_default() {
        let result = codcel_rightb("Hello", None).unwrap();
        assert_eq!(result, "o");
    }

    #[test]
    fn test_rightb_ascii() {
        let result = codcel_rightb("Hello", Some(3)).unwrap();
        assert_eq!(result, "llo");
    }

    #[test]
    fn test_rightb_zero_bytes() {
        let result = codcel_rightb("Hello", Some(0)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_rightb_exceeds_length() {
        let result = codcel_rightb("Hello", Some(10)).unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_rightb_empty_string() {
        let result = codcel_rightb("", Some(3)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_rightb_negative_bytes() {
        let result = codcel_rightb("Hello", Some(-1));
        assert!(result.is_err());
    }

    #[test]
    fn test_rightb_cjk_full_character() {
        // "中国香港": each CJK = 2 DBCS bytes, total = 8
        // RIGHTB("中国香港", 4) -> "香港" (2 chars x 2 bytes = 4)
        let result = codcel_rightb("中国香港", Some(4)).unwrap();
        assert_eq!(result, "香港");
    }

    #[test]
    fn test_rightb_cjk_mid_character_truncation() {
        // RIGHTB("中国香港", 3) -> "港" (3 bytes from right, 港=2 fits, 香 needs 2 more = 4)
        let result = codcel_rightb("中国香港", Some(3)).unwrap();
        assert_eq!(result, "港");
    }

    #[test]
    fn test_rightb_cjk_mid_character_truncation_5_bytes() {
        // RIGHTB("中国香港", 5) -> "香港" (5 bytes, 港=2 + 香=2 = 4 fits, 国 needs 2 more = 6)
        let result = codcel_rightb("中国香港", Some(5)).unwrap();
        assert_eq!(result, "香港");
    }

    #[test]
    fn test_rightb_emoji() {
        // Emoji are 1 DBCS byte each
        // RIGHTB("😀😃😄", 2) -> "😃😄"
        let result = codcel_rightb("😀😃😄", Some(2)).unwrap();
        assert_eq!(result, "😃😄");
    }

    #[test]
    fn test_rightb_accented_latin() {
        // Accented Latin chars are 1 DBCS byte each
        // RIGHTB("Héllo Wörld", 5) -> "Wörld"
        let result = codcel_rightb("Héllo Wörld", Some(5)).unwrap();
        assert_eq!(result, "Wörld");
    }

    #[test]
    fn test_rightb_mixed_ascii_unicode() {
        // "Hello中国": Hello=5, 中=2, 国=2, total=9 DBCS bytes
        // RIGHTB("Hello中国", 4) -> "中国" (2 + 2 = 4)
        let result = codcel_rightb("Hello中国", Some(4)).unwrap();
        assert_eq!(result, "中国");
    }

    #[test]
    fn test_rightb_mixed_ascii_unicode_7() {
        // RIGHTB("Hello中国", 7) -> "lo中国" (l=1, o=1, 中=2, 国=2 -> but that's 6, not 7)
        // Actually: 国=2, 中=2, o=1, l=1, l=1 = 7
        let result = codcel_rightb("Hello中国", Some(7)).unwrap();
        assert_eq!(result, "llo中国");
    }

    #[test]
    fn test_rightb_one_byte_cjk() {
        // RIGHTB("中国", 1) -> "" (1 byte, but 国 needs 2)
        let result = codcel_rightb("中国", Some(1)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_rightb_two_bytes_cjk() {
        // RIGHTB("中国", 2) -> "国"
        let result = codcel_rightb("中国", Some(2)).unwrap();
        assert_eq!(result, "国");
    }

    #[test]
    fn test_rightb_three_bytes_cjk() {
        // RIGHTB("中国", 3) -> "国" (3 bytes, 国=2, 中 needs 2 more = 4)
        let result = codcel_rightb("中国", Some(3)).unwrap();
        assert_eq!(result, "国");
    }
}
