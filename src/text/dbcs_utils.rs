// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

/// Returns true if the character is a DBCS "wide" character (counts as 2 bytes in Excel's *B functions).
/// In Excel's DBCS encoding, CJK and related East Asian characters are 2 bytes; everything else is 1 byte.
pub fn is_dbcs_wide(c: char) -> bool {
    matches!(c,
        '\u{2E80}'..='\u{2EFF}'   // CJK Radicals Supplement
        | '\u{2F00}'..='\u{2FDF}' // Kangxi Radicals
        | '\u{3040}'..='\u{309F}' // Hiragana
        | '\u{30A0}'..='\u{30FF}' // Katakana
        | '\u{3100}'..='\u{312F}' // Bopomofo
        | '\u{3400}'..='\u{4DBF}' // CJK Extension A
        | '\u{4E00}'..='\u{9FFF}' // CJK Unified Ideographs
        | '\u{AC00}'..='\u{D7AF}' // Hangul Syllables
        | '\u{F900}'..='\u{FAFF}' // CJK Compatibility Ideographs
        | '\u{FF01}'..='\u{FF60}' // Fullwidth Forms (punctuation, letters, digits)
        | '\u{FFE0}'..='\u{FFE6}' // Fullwidth Signs (cent, pound, yen, etc.)
    )
}

/// Returns the DBCS byte width of a single character (1 or 2).
pub fn dbcs_char_width(c: char) -> usize {
    if is_dbcs_wide(c) { 2 } else { 1 }
}

/// Returns the total DBCS byte length of a string.
pub fn dbcs_byte_len(s: &str) -> usize {
    s.chars().map(dbcs_char_width).sum()
}

/// Returns the number of characters from the start that fit within `dbcs_byte_count` DBCS bytes.
/// If a wide character would exceed the byte count, it is not included (truncates to last complete char).
pub fn dbcs_chars_fitting_in_bytes(s: &str, dbcs_byte_count: usize) -> usize {
    let mut accumulated = 0usize;
    let mut char_count = 0usize;
    for c in s.chars() {
        let w = dbcs_char_width(c);
        if accumulated + w > dbcs_byte_count {
            break;
        }
        accumulated += w;
        char_count += 1;
    }
    char_count
}

/// Returns the number of characters from the end that fit within `dbcs_byte_count` DBCS bytes.
pub fn dbcs_chars_fitting_in_bytes_from_right(s: &str, dbcs_byte_count: usize) -> usize {
    let chars: Vec<char> = s.chars().collect();
    let mut accumulated = 0usize;
    let mut count = 0usize;
    for &c in chars.iter().rev() {
        let w = dbcs_char_width(c);
        if accumulated + w > dbcs_byte_count {
            break;
        }
        accumulated += w;
        count += 1;
    }
    count
}

/// Converts a 0-based DBCS byte position to a 0-based character index.
/// If the position falls in the middle of a 2-byte (wide) character, advances to the next character.
/// Returns the total character count if the position is at or beyond the end.
pub fn dbcs_byte_pos_to_char_index_forward(s: &str, dbcs_pos: usize) -> usize {
    let mut accumulated = 0usize;
    for (i, c) in s.chars().enumerate() {
        if accumulated >= dbcs_pos {
            return i;
        }
        accumulated += dbcs_char_width(c);
    }
    s.chars().count()
}

/// Returns the DBCS byte position (0-based) of the character at the given character index (0-based).
pub fn char_index_to_dbcs_byte_pos(s: &str, char_idx: usize) -> usize {
    s.chars().take(char_idx).map(dbcs_char_width).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_dbcs_wide() {
        assert!(is_dbcs_wide('中'));
        assert!(is_dbcs_wide('国'));
        assert!(is_dbcs_wide('あ')); // Hiragana
        assert!(is_dbcs_wide('ア')); // Katakana
        assert!(is_dbcs_wide('한')); // Hangul
        assert!(!is_dbcs_wide('A'));
        assert!(!is_dbcs_wide('z'));
        assert!(!is_dbcs_wide('é'));
        assert!(!is_dbcs_wide('ö'));
        assert!(!is_dbcs_wide('😀'));
    }

    #[test]
    fn test_dbcs_byte_len() {
        assert_eq!(dbcs_byte_len("Hello"), 5);
        assert_eq!(dbcs_byte_len("中国"), 4);
        assert_eq!(dbcs_byte_len("中国香港"), 8);
        assert_eq!(dbcs_byte_len("Héllo Wörld"), 11);
        assert_eq!(dbcs_byte_len("Hello中国"), 9);
        assert_eq!(dbcs_byte_len("😀😃😄"), 3);
        assert_eq!(dbcs_byte_len(""), 0);
    }

    #[test]
    fn test_dbcs_chars_fitting_in_bytes() {
        // ASCII: 1 byte each
        assert_eq!(dbcs_chars_fitting_in_bytes("Hello", 3), 3);
        // CJK: 2 bytes each
        assert_eq!(dbcs_chars_fitting_in_bytes("中国香港", 4), 2);
        assert_eq!(dbcs_chars_fitting_in_bytes("中国香港", 5), 2); // 3rd char would need 6 bytes
        assert_eq!(dbcs_chars_fitting_in_bytes("中国香港", 6), 3);
        // Mixed
        assert_eq!(dbcs_chars_fitting_in_bytes("Hello中国", 7), 6); // 5 ASCII + 1 CJK = 7
        // Emoji: 1 byte each in DBCS
        assert_eq!(dbcs_chars_fitting_in_bytes("😀😃😄", 2), 2);
        // Accented: 1 byte each in DBCS
        assert_eq!(dbcs_chars_fitting_in_bytes("Héllo", 3), 3);
    }

    #[test]
    fn test_dbcs_chars_fitting_in_bytes_from_right() {
        assert_eq!(dbcs_chars_fitting_in_bytes_from_right("Hello", 3), 3);
        assert_eq!(dbcs_chars_fitting_in_bytes_from_right("中国香港", 4), 2);
        assert_eq!(dbcs_chars_fitting_in_bytes_from_right("中国香港", 5), 2);
        assert_eq!(dbcs_chars_fitting_in_bytes_from_right("Héllo Wörld", 5), 5);
    }

    #[test]
    fn test_dbcs_byte_pos_to_char_index_forward() {
        // "中国香港": 中=0-1, 国=2-3, 香=4-5, 港=6-7
        assert_eq!(dbcs_byte_pos_to_char_index_forward("中国香港", 0), 0);
        assert_eq!(dbcs_byte_pos_to_char_index_forward("中国香港", 2), 1);
        assert_eq!(dbcs_byte_pos_to_char_index_forward("中国香港", 4), 2);
        // Mid-character: position 1 is inside 中 (width 2), advances to 国 at char index 1
        assert_eq!(dbcs_byte_pos_to_char_index_forward("中国香港", 1), 1);
        // Beyond end
        assert_eq!(dbcs_byte_pos_to_char_index_forward("中国香港", 8), 4);
        assert_eq!(dbcs_byte_pos_to_char_index_forward("中国香港", 10), 4);
    }

    #[test]
    fn test_char_index_to_dbcs_byte_pos() {
        assert_eq!(char_index_to_dbcs_byte_pos("中国香港", 0), 0);
        assert_eq!(char_index_to_dbcs_byte_pos("中国香港", 1), 2);
        assert_eq!(char_index_to_dbcs_byte_pos("中国香港", 2), 4);
        assert_eq!(char_index_to_dbcs_byte_pos("Hello中国", 5), 5);
        assert_eq!(char_index_to_dbcs_byte_pos("Hello中国", 6), 7);
    }
}
