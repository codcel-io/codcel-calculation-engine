// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! The character set Excel's `CODE` and `CHAR` number characters in.
//!
//! Excel does not use Unicode for these two functions but the platform's legacy character set:
//! Windows code page 1252 (`CODE("€")` is 128) or Mac Roman (`CODE("€")` is 219). `UNICODE` and
//! `UNICHAR` are the Unicode versions. All three agree for ASCII (0 to 127).

use crate::text::codcel_char::codcel_char;
use crate::text::codcel_code::codcel_code;
use std::error::Error;

/// Which numbering `CODE` and `CHAR` use, chosen by [`crate::value_format::ValueFormat::character_set`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterSet {
    /// Windows code page 1252, as Excel for Windows (the default).
    Windows,
    /// Mac Roman, as Excel for Mac.
    Mac,
    /// Unicode code points, the same as `UNICODE` and `UNICHAR`.
    Unicode,
}

impl CharacterSet {
    /// `windows`, `mac` or `unicode`, in any case. Anything else is Windows, the default.
    pub fn parse(name: &str) -> CharacterSet {
        match name.trim().to_ascii_lowercase().as_str() {
            "mac" | "macintosh" | "mac-roman" | "macroman" => CharacterSet::Mac,
            "unicode" => CharacterSet::Unicode,
            _ => CharacterSet::Windows,
        }
    }

    fn high_half(self) -> Option<&'static [char; 128]> {
        match self {
            CharacterSet::Windows => Some(&WINDOWS_1252_HIGH),
            CharacterSet::Mac => Some(&MAC_ROMAN_HIGH),
            CharacterSet::Unicode => None,
        }
    }

    /// Excel's `CODE`: the number of the first character of `text`. A character the set does
    /// not have is 63 (`?`), as Excel gives.
    pub fn code(self, text: &str) -> Result<i32, Box<dyn Error + Send + Sync>> {
        let Some(high) = self.high_half() else {
            return codcel_code(text);
        };
        let Some(c) = text.chars().next() else {
            return Err("CODE: Input string is empty".into());
        };
        if (c as u32) < 0x80 {
            return Ok(c as i32);
        }
        Ok(high.iter().position(|&h| h == c).map_or(63, |i| 0x80 + i as i32))
    }

    /// Excel's `CHAR`: the character numbered `code`, 1 to 255 (`#VALUE!` otherwise).
    pub fn char(self, code: i32) -> Result<String, Box<dyn Error + Send + Sync>> {
        let Some(high) = self.high_half() else {
            return codcel_char(code);
        };
        match code {
            1..=0x7F => Ok(char::from(code as u8).to_string()),
            0x80..=0xFF => Ok(high[(code - 0x80) as usize].to_string()),
            _ => Err(crate::excel_error::err_to_box(crate::excel_error::ExcelError::Value)),
        }
    }
}

/// Code page 1252 from 0x80. 0xA0 to 0xFF are Latin-1. The five numbers 1252 leaves unassigned
/// (0x81, 0x8D, 0x8F, 0x90, 0x9D) are the control characters of the same number, as Windows maps them.
const WINDOWS_1252_HIGH: [char; 128] = {
    const LOW: [char; 32] = [
        '\u{20AC}', '\u{0081}', '\u{201A}', '\u{0192}', '\u{201E}', '\u{2026}', '\u{2020}', '\u{2021}',
        '\u{02C6}', '\u{2030}', '\u{0160}', '\u{2039}', '\u{0152}', '\u{008D}', '\u{017D}', '\u{008F}',
        '\u{0090}', '\u{2018}', '\u{2019}', '\u{201C}', '\u{201D}', '\u{2022}', '\u{2013}', '\u{2014}',
        '\u{02DC}', '\u{2122}', '\u{0161}', '\u{203A}', '\u{0153}', '\u{009D}', '\u{017E}', '\u{0178}',
    ];
    let mut table = ['\0'; 128];
    let mut i = 0;
    while i < 128 {
        table[i] = if i < 32 {
            LOW[i]
        } else {
            match char::from_u32(0x80 + i as u32) {
                Some(c) => c,
                None => '\0',
            }
        };
        i += 1;
    }
    table
};

/// Mac Roman from 0x80.
const MAC_ROMAN_HIGH: [char; 128] = [
    'Ä', 'Å', 'Ç', 'É', 'Ñ', 'Ö', 'Ü', 'á', 'à', 'â', 'ä', 'ã', 'å', 'ç', 'é', 'è', //
    'ê', 'ë', 'í', 'ì', 'î', 'ï', 'ñ', 'ó', 'ò', 'ô', 'ö', 'õ', 'ú', 'ù', 'û', 'ü', //
    '†', '°', '¢', '£', '§', '•', '¶', 'ß', '®', '©', '™', '´', '¨', '≠', 'Æ', 'Ø', //
    '∞', '±', '≤', '≥', '¥', 'µ', '∂', '∑', '∏', 'π', '∫', 'ª', 'º', 'Ω', 'æ', 'ø', //
    '¿', '¡', '¬', '√', 'ƒ', '≈', '∆', '«', '»', '…', '\u{00A0}', 'À', 'Ã', 'Õ', 'Œ', 'œ', //
    '–', '—', '“', '”', '‘', '’', '÷', '◊', 'ÿ', 'Ÿ', '⁄', '€', '‹', '›', 'ﬁ', 'ﬂ', //
    '‡', '·', '‚', '„', '‰', 'Â', 'Ê', 'Á', 'Ë', 'È', 'Í', 'Î', 'Ï', 'Ì', 'Ó', 'Ô', //
    '\u{F8FF}', 'Ò', 'Ú', 'Û', 'Ù', 'ı', 'ˆ', '˜', '¯', '˘', '˙', '˚', '¸', '˝', '˛', 'ˇ', //
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euro_in_each_set() {
        assert_eq!(CharacterSet::Windows.code("€").unwrap(), 128);
        assert_eq!(CharacterSet::Mac.code("€").unwrap(), 219);
        assert_eq!(CharacterSet::Unicode.code("€").unwrap(), 8364);
        assert_eq!(CharacterSet::Windows.char(128).unwrap(), "€");
        assert_eq!(CharacterSet::Mac.char(219).unwrap(), "€");
        assert_eq!(CharacterSet::Unicode.char(8364).unwrap(), "€");
    }

    #[test]
    fn ascii_is_the_same_in_every_set() {
        for set in [CharacterSet::Windows, CharacterSet::Mac, CharacterSet::Unicode] {
            assert_eq!(set.code("A").unwrap(), 65);
            assert_eq!(set.code("m").unwrap(), 109);
            assert_eq!(set.char(45).unwrap(), "-");
            assert_eq!(set.char(10).unwrap(), "\n");
        }
    }

    #[test]
    fn latin_1_and_mac_letters() {
        assert_eq!(CharacterSet::Windows.code("é").unwrap(), 233);
        assert_eq!(CharacterSet::Mac.code("é").unwrap(), 142);
        assert_eq!(CharacterSet::Windows.char(233).unwrap(), "é");
        assert_eq!(CharacterSet::Mac.char(142).unwrap(), "é");
        assert_eq!(CharacterSet::Windows.char(129).unwrap(), "\u{0081}");
        assert_eq!(CharacterSet::Windows.code("\u{0081}").unwrap(), 129);
    }

    #[test]
    fn a_character_outside_the_set_is_a_question_mark() {
        assert_eq!(CharacterSet::Windows.code("π").unwrap(), 63);
        assert_eq!(CharacterSet::Mac.code("π").unwrap(), 185);
        assert_eq!(CharacterSet::Mac.code("Š").unwrap(), 63);
    }

    #[test]
    fn char_outside_1_to_255_is_an_error_except_in_unicode() {
        assert!(CharacterSet::Windows.char(0).is_err());
        assert!(CharacterSet::Windows.char(256).is_err());
        assert!(CharacterSet::Mac.char(256).is_err());
        assert_eq!(CharacterSet::Unicode.char(256).unwrap(), "Ā");
        assert!(CharacterSet::Windows.code("").is_err());
    }

    #[test]
    fn every_byte_round_trips() {
        for set in [CharacterSet::Windows, CharacterSet::Mac] {
            for code in 1..=255 {
                assert_eq!(set.code(&set.char(code).unwrap()).unwrap(), code, "{set:?} {code}");
            }
        }
    }

    #[test]
    fn parse_names() {
        assert_eq!(CharacterSet::parse("MAC"), CharacterSet::Mac);
        assert_eq!(CharacterSet::parse("unicode"), CharacterSet::Unicode);
        assert_eq!(CharacterSet::parse("windows"), CharacterSet::Windows);
        assert_eq!(CharacterSet::parse(""), CharacterSet::Windows);
    }
}
