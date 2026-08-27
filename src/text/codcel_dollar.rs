// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::locale::Locale;
use crate::text_function::format_value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Builds an Excel format code for `decimals` decimal places, laid out the way
/// `locale` places its currency.
///
/// CLDR expresses currency layout as a pattern with `¤` standing in for the
/// symbol — `¤#,##0.00` in the United States, `#,##0.00 ¤` in Germany,
/// `¤ #,##0.00;¤-#,##0.00` in Switzerland. Position, spacing and the shape of
/// the negative form all live in that pattern, and none of them can be
/// recovered from a bare symbol, which is why `DOLLAR` needs the pattern.
///
/// The symbol is substituted quoted, because several are letters the format
/// tokenizer would otherwise read as date codes: unquoted `CHF` tokenizes as an
/// hour followed by two literals.
///
/// One known limitation: locales that group by lakh rather than uniform
/// thousands (CLDR gives Hindi `¤#,##,##0.00`) are rendered with uniform
/// three-digit groups, because `add_thousands_sep` in the tokenizer groups
/// uniformly.
fn currency_format_code(locale: &'static Locale, symbol: &str, decimals: usize) -> String {
    let mut digits = String::from("#,##0");
    if decimals > 0 {
        digits.push('.');
        for _ in 0..decimals {
            digits.push('0');
        }
    }

    let pattern = locale.currency.standard;
    let quoted = format!("\"{}\"", symbol.replace('"', ""));

    // The numeric run is everything from the first digit placeholder to the
    // last; `,` and `.` in between belong to it.
    let chars: Vec<char> = pattern.chars().collect();
    let first = chars.iter().position(|c| matches!(c, '#' | '0'));
    let last = chars.iter().rposition(|c| matches!(c, '#' | '0'));

    let with_digits = match (first, last) {
        (Some(first), Some(last)) => {
            let head: String = chars[..first].iter().collect();
            let tail: String = chars[last + 1..].iter().collect();
            format!("{head}{digits}{tail}")
        }
        // A pattern with no digit placeholders is not one we can lay out;
        // fall back to the canonical prefix form.
        _ => format!("\u{a4}{digits}"),
    };

    with_digits.replace('\u{a4}', &quoted)
}

/// Excel-compatible `DOLLAR` that converts a number to text in currency format.
/// - `number`: the number to format as currency.
/// - `decimals`: optional number of decimal places (default 2). Negative values round
///   to the left of the decimal point.
/// - `value_format`: locale settings for currency symbol, decimal separator, and thousands separator.
///
/// Returns the number formatted as currency text with thousands separators,
/// with the symbol placed the way the locale places it — `$1,234.56` in the
/// United States and `1.234,56 €` in Germany. The symbol itself comes from
/// [`ValueFormat::currency_symbol`], which a caller can override; only its
/// position comes from the locale.
pub fn codcel_dollar(
    number: f64,
    decimals: Option<i32>,
    value_format: &ValueFormat,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let decimals = decimals.unwrap_or(2); // Default to 2 decimal places if not provided
    let factor = 10f64.powi(decimals.abs());

    let rounded = if decimals >= 0 {
        (number * factor).round() / factor
    } else {
        // Negative `decimals` rounds to the left of the decimal point, and the
        // result is then shown with no decimal places at all.
        (number / factor).round() * factor
    };

    let places = decimals.max(0) as usize;
    let code = currency_format_code(value_format.locale(), &value_format.currency_symbol, places);
    format_value(rounded, &code, value_format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dollar_positive_number_us_format() {
        // =DOLLAR(1234.56) in US format
        // =DOLLAR(1234,56) in German format
        let value_format = ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        };

        let result = codcel_dollar(1234.56, None, &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "$1,234.56");
    }

    #[test]
    fn test_dollar_negative_number_us_format() {
        // =DOLLAR(-1234.56) in US format
        // =DOLLAR(-1234,56) in German format
        let value_format = ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        };

        let result = codcel_dollar(-1234.56, None, &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "-$1,234.56");
    }

    #[test]
    fn test_dollar_with_custom_decimals_us_format() {
        // =DOLLAR(1234.56, 3) in US format
        // =DOLLAR(1234,56; 3) in German format
        let value_format = ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        };

        let result = codcel_dollar(1234.56, Some(3), &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "$1,234.560");
    }

    #[test]
    fn test_dollar_with_zero_decimals_us_format() {
        // =DOLLAR(1234.56, 0) in US format
        // =DOLLAR(1234,56; 0) in German format
        let value_format = ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        };

        let result = codcel_dollar(1234.56, Some(0), &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "$1,235");
    }

    #[test]
    fn test_dollar_with_negative_decimals_us_format() {
        // =DOLLAR(1234.56, -1) in US format
        // =DOLLAR(1234,56; -1) in German format
        let value_format = ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        };

        let result = codcel_dollar(1234.56, Some(-1), &value_format).unwrap();
        println!("{result}");
        assert_eq!(result, "$1,230");
    }

    // Asserts a non-English locale, which only exists with `locale-data` on.
    #[cfg(feature = "locale-data")]
    #[test]
    /// German puts the currency symbol *after* the amount, separated by a
    /// non-breaking space: `=DOLLAR(1234,56)` in a German Excel gives
    /// `1.234,56 €`, not `€1.234,56`.
    fn test_dollar_german_format() {
        let value_format = ValueFormat {
            decimal_separator: ",".to_string(),
            currency_symbol: "€".to_string(),
            thousands_separator: ".".to_string(),
            use_excel_rounding: true,
            language: "de".to_string(),
            ..Default::default()
        };

        let result = codcel_dollar(1234.56, None, &value_format).unwrap();
        assert_eq!(result, "1.234,56\u{a0}€");
    }

    /// A negative amount follows the locale's negative form, which is not
    /// always a leading minus.
    // Asserts a non-English locale, which only exists with `locale-data` on.
    #[cfg(feature = "locale-data")]
    #[test]
    fn negative_amounts_follow_the_locale_negative_form() {
        let de = ValueFormat {
            decimal_separator: ",".to_string(),
            currency_symbol: "€".to_string(),
            thousands_separator: ".".to_string(),
            language: "de".to_string(),
            ..Default::default()
        };
        assert_eq!(
            codcel_dollar(-1234.56, None, &de).unwrap(),
            "-1.234,56\u{a0}€"
        );
    }

    /// A symbol made of letters has to survive the format tokenizer, which
    /// would otherwise read `CHF` as an hour code followed by two literals.
    #[test]
    fn a_letter_currency_symbol_is_not_read_as_a_date_code() {
        let ch = ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "CHF".to_string(),
            thousands_separator: "'".to_string(),
            language: "de".to_string(),
            region: "CH".to_string(),
            ..Default::default()
        };
        let result = codcel_dollar(1234.56, None, &ch).unwrap();
        assert!(result.contains("CHF"), "got {result:?}");
        assert!(result.contains("1'234.56"), "got {result:?}");
    }
}
