// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

use super::dbcs_utils::{dbcs_byte_len, dbcs_chars_fitting_in_bytes};

/// Excel-compatible `LEFTB` that returns characters from the start of a text string based on DBCS byte count.
/// - `text`: the text string from which to extract characters.
/// - `num_bytes`: optional number of DBCS bytes to extract (default 1).
///   Returns the leftmost characters from the text that fit within the specified DBCS byte count.
///   If `num_bytes` exceeds the text DBCS byte length, returns the entire text.
///   When `num_bytes` falls in the middle of a 2-byte (wide) character, truncates to the
///   last complete character that fits within the byte count.
///   Returns an error if `num_bytes` is negative.
///   Unlike LEFT which counts characters, LEFTB counts DBCS bytes. For non-CJK text, results are identical.
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

    if bytes_to_take >= dbcs_byte_len(text_ref) {
        return Ok(text_ref.to_string());
    }

    let char_count = dbcs_chars_fitting_in_bytes(text_ref, bytes_to_take);
    Ok(text_ref.chars().take(char_count).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leftb_default() {
        let result = codcel_leftb("Hello", None).unwrap();
        assert_eq!(result, "H");
    }

    #[test]
    fn test_leftb_ascii() {
        let result = codcel_leftb("Hello", Some(3)).unwrap();
        assert_eq!(result, "Hel");
    }

    #[test]
    fn test_leftb_zero_bytes() {
        let result = codcel_leftb("Hello", Some(0)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_leftb_exceeds_length() {
        let result = codcel_leftb("Hello", Some(10)).unwrap();
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_leftb_empty_string() {
        let result = codcel_leftb("", Some(3)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_leftb_negative_bytes() {
        let result = codcel_leftb("Hello", Some(-1));
        assert!(result.is_err());
    }

    #[test]
    fn test_leftb_cjk_full_character() {
        // "中国香港": each CJK char = 2 DBCS bytes, total = 8 DBCS bytes
        // LEFTB("中国香港", 4) -> "中国" (2 chars x 2 bytes = 4)
        let result = codcel_leftb("中国香港", Some(4)).unwrap();
        assert_eq!(result, "中国");
    }

    #[test]
    fn test_leftb_cjk_mid_character_truncation() {
        // LEFTB("中国香港", 3) -> "中" (3 bytes requested, 中=2 bytes fits, 国 needs 2 more = 4, truncate)
        let result = codcel_leftb("中国香港", Some(3)).unwrap();
        assert_eq!(result, "中");
    }

    #[test]
    fn test_leftb_cjk_mid_character_truncation_5_bytes() {
        // LEFTB("中国香港", 5) -> "中国" (5 bytes requested, 中国=4 bytes, 香 needs 2 more = 6, truncate)
        let result = codcel_leftb("中国香港", Some(5)).unwrap();
        assert_eq!(result, "中国");
    }

    #[test]
    fn test_leftb_emoji() {
        // Emoji are 1 DBCS byte each
        // LEFTB("😀😃😄", 2) -> "😀😃"
        let result = codcel_leftb("😀😃😄", Some(2)).unwrap();
        assert_eq!(result, "😀😃");
    }

    #[test]
    fn test_leftb_accented_latin() {
        // Accented Latin chars are 1 DBCS byte each
        // LEFTB("Héllo", 3) -> "Hél"
        let result = codcel_leftb("Héllo", Some(3)).unwrap();
        assert_eq!(result, "Hél");
    }

    #[test]
    fn test_leftb_mixed_ascii_unicode() {
        // "Hello中国": Hello=5 bytes, 中=2 bytes, 国=2 bytes, total=9 DBCS bytes
        // LEFTB("Hello中国", 7) -> "Hello中" (5 + 2 = 7)
        let result = codcel_leftb("Hello中国", Some(7)).unwrap();
        assert_eq!(result, "Hello中");
    }

    #[test]
    fn test_leftb_one_byte_cjk() {
        // LEFTB("中国", 1) -> "" (1 byte, but 中 needs 2)
        let result = codcel_leftb("中国", Some(1)).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_leftb_two_bytes_cjk() {
        // LEFTB("中国", 2) -> "中"
        let result = codcel_leftb("中国", Some(2)).unwrap();
        assert_eq!(result, "中");
    }

    #[test]
    fn test_leftb_three_bytes_cjk() {
        // LEFTB("中国", 3) -> "中" (3 bytes, 中=2, 国 needs 2 more = 4, truncate)
        let result = codcel_leftb("中国", Some(3)).unwrap();
        assert_eq!(result, "中");
    }
}
