// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Excel's `TEXT` and its locale sensitivity.
//!
//! Excel format codes are written in the interface language of the Excel that
//! produced them. A German workbook may carry `"jjjj-mm-tt"` where the
//! canonical code is `"yyyy-mm-dd"`, and a comma-decimal locale writes
//! `"#.##0,00"` where the canonical code is `"#,##0.00"`. The tokenizer in
//! [`crate::text_function`] understands only the canonical form, so both are
//! rewritten here before it sees them.
//!
//! | Meaning | English | Portuguese | German | French | Italian |
//! |---------|---------|------------|--------|--------|---------|
//! | Year    | `yyyy`  | `aaaa`     | `jjjj` | `aaaa` | `aaaa`  |
//! | Month   | `mm`    | `mm`       | `mm`   | `mm`   | `mm`    |
//! | Day     | `dd`    | `dd`       | `tt`   | `jj`   | `gg`    |
//!
//! The per-language pairs live in the generated locale table as
//! [`Locale::date_token_aliases`](crate::locale::Locale::date_token_aliases).
//!
//! Two related pieces of Excel's locale sensitivity are **out of scope here**
//! and always will be:
//!
//! - The argument separator (`,` in English locales, `;` in most European
//!   ones) is a property of the formula grammar, not of a format code.
//! - Localized function names (`DATUM`, `DATA`) likewise. A workbook stores
//!   formula text in canonical English regardless of the authoring language;
//!   Excel localizes only what it displays. Codcel reads files rather than
//!   keystrokes, so nothing localized ever reaches its parser.

use crate::locale::Locale;
use crate::text_function::format_value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Is this character a digit placeholder?
///
/// Used to decide whether a separator sits inside a number pattern, which is
/// the only place it should be rewritten.
fn is_digit_placeholder(c: char) -> bool {
    matches!(c, '0' | '#' | '?')
}

/// Rewrites a format code written in `locale`'s conventions into the canonical
/// form the tokenizer expects.
///
/// Runs one quote-aware pass. Text inside `"…"` and any character escaped with
/// a backslash is copied through untouched, matching how Excel treats literals
/// in a format code.
fn canonicalize(format: &str, locale: &'static Locale, value_format: &ValueFormat) -> String {
    let chars: Vec<char> = format.chars().collect();
    let lower: Vec<char> = format.to_lowercase().chars().collect();
    // Case folding can change length for a few scripts; fall back to the raw
    // form rather than risk indexing the two out of step.
    let lower = if lower.len() == chars.len() {
        lower
    } else {
        chars.clone()
    };

    // Only the separators that actually differ from canonical need rewriting,
    // and only when they are unambiguous single characters.
    let decimal = one_char(&value_format.decimal_separator).filter(|c| *c != '.');
    let thousands = one_char(&value_format.thousands_separator).filter(|c| *c != ',');

    let mut out = String::with_capacity(format.len());
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '"' => {
                out.push('"');
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    out.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    out.push('"');
                    i += 1;
                }
                continue;
            }
            '\\' => {
                out.push('\\');
                i += 1;
                if i < chars.len() {
                    out.push(chars[i]);
                    i += 1;
                }
                continue;
            }
            _ => {}
        }

        // Date-code aliases. The table is ordered longest-first, so the first
        // match is the greedy one: `jjjj` is tried before `jj`.
        if let Some((from, to)) = locale
            .date_token_aliases
            .iter()
            .find(|(from, _)| lower[i..].starts_with(&from.chars().collect::<Vec<_>>()[..]))
        {
            debug_assert!(!from.is_empty());
            out.push_str(to);
            i += from.chars().count();
            continue;
        }

        // Separators, but only where one is standing between digit
        // placeholders. Restricting it that way is what lets a French date code
        // keep its spaces: in `dd mmm yyyy` the space separates letters, while
        // in `# ##0,00` it separates digit placeholders.
        let adjacent_to_digits = |i: usize| {
            let before = i
                .checked_sub(1)
                .and_then(|j| chars.get(j))
                .is_some_and(|c| is_digit_placeholder(*c));
            let after = chars.get(i + 1).is_some_and(|c| is_digit_placeholder(*c));
            before || after
        };

        if Some(chars[i]) == decimal && adjacent_to_digits(i) {
            out.push('.');
            i += 1;
            continue;
        }
        if Some(chars[i]) == thousands && adjacent_to_digits(i) {
            out.push(',');
            i += 1;
            continue;
        }

        out.push(chars[i]);
        i += 1;
    }
    out
}

/// The single `char` of `s`, or `None` if `s` is empty or longer than one.
fn one_char(s: &str) -> Option<char> {
    let mut it = s.chars();
    match (it.next(), it.next()) {
        (Some(c), None) => Some(c),
        _ => None,
    }
}

/// Excel-compatible `TEXT` that converts a value to text with a specified format (with locale support).
/// - `value`: the numeric value to format (for dates, this is the Excel serial date number).
/// - `format_string`: the format pattern (e.g., `"yyyy-mm-dd"`, `"0.00"`, `"#,##0"`).
///   Locale-specific format codes are supported (e.g., German `"jjjj-mm-tt"` for dates).
/// - `value_format`: locale settings for language, decimal separator, etc.
///   Returns the formatted text representation of the value.
///   Returns an error if the format string is unsupported.
pub fn codcel_text_with_locale<S: AsRef<str>>(
    value: f64,
    format_string: S,
    value_format: &ValueFormat,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let canonical = canonicalize(format_string.as_ref(), value_format.locale(), value_format);
    format_value(value, &canonical, value_format)
}

/// Excel-compatible `TEXT` that converts a value to text with a specified format (English locale).
/// - `value`: the numeric value to format (for dates, this is the Excel serial date number).
/// - `format_string`: the format pattern (e.g., `"yyyy-mm-dd"`, `"0.00"`, `"#,##0"`).
///   Returns the formatted text representation of the value using English/US locale settings.
///   This is a convenience wrapper around `codcel_text_with_locale` with default English locale.
pub fn codcel_text<S: AsRef<str>>(
    value: f64,
    format_string: S,
    value_format: &ValueFormat,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    codcel_text_with_locale(value, format_string, value_format)
}

#[cfg(test)]
mod tests {
    use super::*;
    // Used only by the German-locale test, which needs the locale table.
    #[cfg(feature = "locale-data")]
    use crate::{
        date_and_time::codcel_date::codcel_date, date_system::DateSemantics,
        date_time_base::date_time_to_excel,
    };

    // Helper function to create a ValueFormat for testing
    fn create_value_format(language: &str) -> ValueFormat {
        ValueFormat {
            decimal_separator: if language == "de" || language == "fr" || language == "pt" {
                ",".to_string()
            } else {
                ".".to_string()
            },
            currency_symbol: if language == "de" {
                "€".to_string()
            } else {
                "$".to_string()
            },
            thousands_separator: if language == "de" || language == "fr" || language == "pt" {
                ".".to_string()
            } else {
                ",".to_string()
            },
            use_excel_rounding: true,
            language: language.to_string(),
            ..Default::default()
        }
    }

    // Default English ValueFormat
    fn default_value_format() -> ValueFormat {
        create_value_format("en")
    }

    #[test]
    fn test_text_number_basic() {
        // =TEXT(123.45, "0.00") in US format
        // =TEXT(123,45; "0,00") in German format
        // Test with default English locale
        let value_format = default_value_format();
        let result = codcel_text(123.45, "0.00", &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "123.45");

        // Test with explicit English locale
        let result_en = codcel_text_with_locale(123.45, "0.00", &value_format).unwrap();
        println!("English: {result_en}");
        assert_eq!(result_en, "123.45");

        // Test with German format
        let german_format = create_value_format("de");
        let result_de = codcel_text_with_locale(123.45, "0,00", &german_format).unwrap();
        println!("German: {result_de}");
        assert_eq!(result_de, "123,45");
    }

    #[test]
    fn test_text_number_percentage() {
        // =TEXT(0.456, "0.0%") in US format
        // =TEXT(0,456; "0,0%") in German format
        // Test with default English locale
        let value_format = default_value_format();
        let result = codcel_text(0.456, "0.0%", &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "45.6%");

        // Test with explicit English locale
        let result_en = codcel_text_with_locale(0.456, "0.0%", &value_format).unwrap();
        println!("English: {result_en}");
        assert_eq!(result_en, "45.6%");

        // Test with German format
        let german_format = create_value_format("de");
        let result_de = codcel_text_with_locale(0.456, "0,0%", &german_format).unwrap();
        println!("German: {result_de}");
        assert_eq!(result_de, "45,6%");
    }

    #[test]
    fn test_text_number_with_commas() {
        // =TEXT(1234567.89, "#,##0.00") in US format
        // =TEXT(1234567,89; "#.##0,00") in German format
        // Test with default English locale
        let value_format = default_value_format();
        let result = codcel_text(1234567.89, "#,##0.00", &value_format).unwrap();
        println!("{result}");
        // Note: The actual implementation might not handle thousand separators correctly
        // This test might need adjustment based on the actual behavior

        // Test with explicit English locale
        let result_en = codcel_text_with_locale(1234567.89, "#,##0.00", &value_format).unwrap();
        println!("English: {result_en}");

        // Test with German format
        let german_format = create_value_format("de");
        let result_de = codcel_text_with_locale(1234567.89, "#.##0,00", &german_format).unwrap();
        println!("German: {result_de}");
    }

    // Asserts a non-English locale, which only exists with `locale-data` on.
    #[cfg(feature = "locale-data")]
    #[test]
    fn test_text_date_yyyy_mm_dd() {
        // =TEXT(DATE(2023,5,15), "yyyy-mm-dd") in US format
        // =TEXT(DATE(2023;5;15); "jjjj-mm-tt") in German format
        // May 15, 2023 is 44696 days after Dec 30, 1899
        let days_since_base = date_time_to_excel(
            &codcel_date(2023, 5, 15).unwrap(),
            DateSemantics::EXCEL_1900,
        )
        .unwrap();

        // Test with default English locale
        let value_format = default_value_format();
        let result = codcel_text(days_since_base, "yyyy-mm-dd", &value_format).unwrap();
        println!("Default: {result}");
        assert_eq!(result, "2023-05-15");

        // Test with explicit English locale
        let result_en =
            codcel_text_with_locale(days_since_base, "yyyy-mm-dd", &value_format).unwrap();
        println!("English: {result_en}");
        assert_eq!(result_en, "2023-05-15");

        // Test with German format
        let german_format = create_value_format("de");
        println!("German language: {}", german_format.language);

        let normalized = canonicalize("jjjj-mm-tt", german_format.locale(), &german_format);
        assert_eq!(normalized, "yyyy-mm-dd");

        let result_de =
            codcel_text_with_locale(days_since_base, "jjjj-mm-tt", &german_format).unwrap();
        println!("German: {result_de}");
        assert_eq!(result_de, "2023-05-15");
    }

    /* TODO: FIX CODCEL TEXT #[test]
    fn test_text_date_dd_mmm_yyyy() {
        // =TEXT(DATE(2023,5,15), "dd-mmm-yyyy") in US format
        // =TEXT(DATE(2023;5;15); "dd-mmm-yyyy") in German format
        // May 15, 2023 is 44696 days after Dec 30, 1899
        let days_since_base = 44696.0;
        let result = codcel_text(days_since_base, "dd-mmm-yyyy").unwrap();
        println!("{result}");
        assert_eq!(result, "15-May-2023");
    }*/

    /* TODO: Fix this #[test]
    fn test_text_time() {
        // =TEXT(TIME(14,30,0), "hh:mm") in US format
        // =TEXT(TIME(14;30;0); "hh:mm") in German format
        // 14:30:00 is 0.6041666... of a day
        let time_fraction = time_to_excel(&codcel_time(14,30,0).unwrap()).unwrap();
        let result = codcel_text(time_fraction, "hh:mm").unwrap();
        println!("{result}");
        assert_eq!(result, "14:30");
    }*/

    /* TODO: Fix this text
    #[test]
    fn test_text_date_time() {
        // =TEXT(DATE(2023,5,15)+TIME(14,30,0), "yyyy-mm-dd hh:mm:ss") in US format
        // =TEXT(DATE(2023;5;15)+TIME(14;30;0); "yyyy-mm-dd hh:mm:ss") in German format
        // May 15, 2023 14:30:00 is 44696.6041666... days after Dec 30, 1899
        let days_since_base = date_time_to_excel(&codcel_date(2023,5,15).unwrap()).unwrap() + time_to_excel(&codcel_time(14,30,0).unwrap()).unwrap();
        let result = codcel_text(days_since_base, "yyyy-mm-dd hh:mm:ss").unwrap();
        println!("{result}");
        assert_eq!(result, "2023-05-15 14:30:00");
    }*/

    #[test]
    fn test_text_arbitrary_format() {
        // Excel's TEXT function never errors on format strings — it interprets
        // recognized tokens (d, s, m, h, etc.) and passes through literals.
        // "unsupported" contains 'd' (day) and 's' (second), so it's treated as date/time.
        let value_format = default_value_format();
        let result = codcel_text(123.45, "unsupported", &value_format);
        assert!(result.is_ok());
    }
}
