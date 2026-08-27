// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Golden output for `TEXT` and `DOLLAR` across locales.
//!
//! These pin the observable end of the locale table. A regeneration that
//! changed a month name, a currency position or a number symbol shows up here
//! as a diff rather than as a surprise in someone's generated project.
//!
//! Format codes are written the way a user of that locale writes them, because
//! that is how Excel reads them: `TEXT` format codes are in the interface
//! language, so a German workbook carries `"#.##0,00"` where an English one
//! carries `"#,##0.00"`. See [`crate::text::codcel_text`].

#[cfg(feature = "locale-data")]
use crate::text::codcel_dollar::codcel_dollar;
use crate::text::codcel_text::codcel_text;
use crate::value_format::ValueFormat;

/// 15 May 2023 (a Monday), 14:30:45, as a 1900-system serial.
const DATE_TIME: f64 = 45061.0 + (14.0 * 3600.0 + 30.0 * 60.0 + 45.0) / 86400.0;

fn text(tag: &str, format: &str) -> String {
    let vf = ValueFormat::from_language(tag);
    codcel_text(DATE_TIME, format, &vf).unwrap_or_else(|e| format!("<error: {e}>"))
}

fn number(tag: &str, value: f64, format: &str) -> String {
    let vf = ValueFormat::from_language(tag);
    codcel_text(value, format, &vf).unwrap_or_else(|e| format!("<error: {e}>"))
}

#[cfg(feature = "locale-data")]
fn dollar(tag: &str) -> String {
    let vf = ValueFormat::from_language(tag);
    codcel_dollar(1234.56, None, &vf).unwrap_or_else(|e| format!("<error: {e}>"))
}

#[cfg(feature = "locale-data")]
#[test]
fn month_and_weekday_names_follow_the_language() {
    let cases = [
        ("en-US", "May 15, 2023", "Monday", "May"),
        ("en-GB", "May 15, 2023", "Monday", "May"),
        ("de-DE", "Mai 15, 2023", "Montag", "Mai"),
        ("fr-FR", "mai 15, 2023", "lundi", "mai"),
        ("pt-BR", "maio 15, 2023", "segunda-feira", "mai."),
        ("ja-JP", "5月 15, 2023", "月曜日", "5月"),
        ("hi-IN", "मई 15, 2023", "सोमवार", "मई"),
        ("ar-EG", "مايو 15, 2023", "الاثنين", "مايو"),
    ];
    for (tag, long, weekday, short) in cases {
        assert_eq!(text(tag, "mmmm d, yyyy"), long, "{tag} mmmm");
        assert_eq!(text(tag, "dddd"), weekday, "{tag} dddd");
        assert_eq!(text(tag, "mmm"), short, "{tag} mmm");
    }
}

/// `mmmmm` is not the first letter of `mmmm`: CLDR gives Japanese a digit and
/// several locales disambiguate months that would otherwise collide.
#[cfg(feature = "locale-data")]
#[test]
fn single_letter_months_are_not_truncated_names() {
    assert_eq!(text("en-US", "mmmmm"), "M");
    assert_eq!(text("ja-JP", "mmmmm"), "5");
    assert_eq!(text("hi-IN", "mmmmm"), "म");
}

/// Excel takes AM/PM from Windows, which upper-cases the Latin markers even in
/// locales CLDR writes in lower case.
#[cfg(feature = "locale-data")]
#[test]
fn am_pm_markers_are_localized_but_latin_stays_upper_case() {
    assert_eq!(text("en-US", "h:mm:ss AM/PM"), "2:30:45 PM");
    assert_eq!(text("en-GB", "h:mm:ss AM/PM"), "2:30:45 PM");
    assert_eq!(text("ja-JP", "h:mm:ss AM/PM"), "2:30:45 午後");
    assert_eq!(text("ar-EG", "h:mm:ss AM/PM"), "2:30:45 م");
}

/// The format code is written in the writer's locale, so each of these is what
/// a user of that locale would actually type.
#[cfg(feature = "locale-data")]
#[test]
fn grouped_decimals_use_the_locale_separators() {
    assert_eq!(number("en-US", 45061.6, "#,##0.00"), "45,061.60");
    assert_eq!(number("de-DE", 45061.6, "#.##0,00"), "45.061,60");
    assert_eq!(number("pt-BR", 45061.6, "#.##0,00"), "45.061,60");
    assert_eq!(number("fr-FR", 45061.6, "# ##0,00"), "45 061,60");
    assert_eq!(number("de-CH", 45061.6, "#'##0.00"), "45'061.60");
}

/// A `[$SYMBOL-LCID]` prefix used to tokenize as a colour code and be dropped
/// on the floor, taking the currency symbol with it.
#[test]
fn currency_format_codes_keep_their_symbol() {
    assert_eq!(number("en-US", 1234.5, "[$€-407]#,##0.00"), "€1,234.50");
    assert_eq!(number("en-US", 1234.5, "[$$-409]#,##0.00"), "$1,234.50");
    // A bare locale id carries no symbol and must not leave one behind.
    assert_eq!(number("en-US", 1234.5, "[$-409]#,##0.00"), "1,234.50");
    // A colour code is still a colour code, and still discarded.
    assert_eq!(number("en-US", 1234.5, "[Red]#,##0.00"), "1,234.50");
}

/// Where the symbol sits, and what separates it from the amount, is the whole
/// reason `DOLLAR` reads a pattern rather than a bare symbol.
#[cfg(feature = "locale-data")]
#[test]
fn dollar_places_the_symbol_the_way_the_locale_does() {
    assert_eq!(dollar("en-US"), "$1,234.56");
    assert_eq!(dollar("en-GB"), "£1,234.56");
    assert_eq!(dollar("de-DE"), "1.234,56\u{a0}€");
    assert_eq!(dollar("fr-FR"), "1 234,56\u{a0}€");
    assert_eq!(dollar("pt-BR"), "R$\u{a0}1.234,56");
    assert_eq!(dollar("de-CH"), "CHF\u{a0}1'234.56");
    assert_eq!(dollar("ja-JP"), "¥1,234.56");
    assert_eq!(dollar("hi-IN"), "₹1,234.56");
}

/// Scientific notation and percentages draw their symbols from the locale.
/// Arabic prefixes both with a left-to-right mark, which is CLDR's doing and
/// is what keeps the sign on the correct side in a right-to-left run.
#[cfg(feature = "locale-data")]
#[test]
fn number_symbols_are_localized() {
    assert_eq!(number("en-US", 45061.6, "0.00E+00"), "4.51E+04");
    assert_eq!(number("de-DE", 45061.6, "0,00E+00"), "4,51E+04");
    assert_eq!(number("ar-EG", 45061.6, "0.00E+00"), "4.51E\u{200e}+04");

    assert_eq!(number("en-US", 0.1234, "0.00%"), "12.34%");
    assert_eq!(number("ar-EG", 0.1234, "0.00%"), "12.34\u{200e}%\u{200e}");
}

/// Elapsed-time codes are locale-independent by construction — they count, they
/// do not name.
#[test]
fn elapsed_time_is_not_localized() {
    assert_eq!(text("en-US", "[h]:mm:ss"), "1081478:30:45");
    assert_eq!(text("de-DE", "[h]:mm:ss"), "1081478:30:45");
}
