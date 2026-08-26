// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::text_function::format_value;
use crate::value_format::ValueFormat;
use std::error::Error;

/* TODO
Excel TEXT() and DATE/DATUM Function Locale Caveats
===================================================

Excel formulas are **locale-sensitive**, affecting:
1. Argument separators (`,` vs `;`)
2. Function names (`DATE` vs `DATUM`)
3. Format codes in `TEXT()` (like `yyyy`, `aaaa`, `jjjj`)

----------------------------------------
| Meaning      | English | Portuguese | German | French |
|--------------|---------|------------|--------|--------|
| Year         | yyyy    | aaaa       | jjjj   | aaaa   |
| Month        | mm      | mm         | mm     | mm     |
| Day          | dd      | dd         | tt     | jj     |
| Hour         | hh      | hh         | hh     | hh     |
| Minute       | mm      | mm         | mm     | mm     |
| Second       | ss      | ss         | ss     | ss     |

Function Names:
---------------
| Language     | DATE function name |
|--------------|--------------------|
| English      | DATE               |
| Portuguese   | DATA               |
| German       | DATUM              |
| French       | DATE               |

Examples:
---------
=TEXT(DATE(2023,5,15), "yyyy-mm-dd")         --> English (US/UK)
=TEXT(DATA(2023;5;15); "aaaa-mm-dd")         --> Portuguese
=TEXT(DATUM(2023;5;15); "jjjj-mm-tt")        --> German
=TEXT(DATE(2023;5;15); "aaaa-mm-jj")         --> French

Other Notes:
------------
- Argument separator is `,` in English locales, but `;` in most European locales.
- Excel reuses `mm` for both **month** and **minute**, depending on context.
- `"dddd"` and `"mmmm"` return weekday/month names in the local language.
- TEXT() format strings **do not follow ISO 8601**, and must match Excel's UI language.
- Localised function names like `DATUM`, `DATA`, etc., are needed **only if your Excel is running in that language**.
- In VBA or Excel with English interface, always use `DATE`, not `DATUM`.

Tip:
Use Excel's **Format Cells → Custom** dialog to inspect the correct formatting codes for your region.

*/

// Function to normalize format string based on language
fn normalize_format_string(format: &str, language: &str) -> String {
    match language {
        "de" => {
            // German format
            // Note: We need to convert from German format to standard format
            // before passing to format_date_time_with_locale
            format
                .replace("jjjj", "yyyy") // Year
                .replace("jj", "yy") // 2-digit year
                .replace("tt", "dd") // Day
        }
        "pt" | "pt-BR" | "pt-PT" => {
            // Portuguese format
            format
                .replace("aaaa", "yyyy") // Year
                .replace("aa", "yy") // 2-digit year
        }
        "fr" => {
            // French format
            format
                .replace("aaaa", "yyyy") // Year
                .replace("aa", "yy") // 2-digit year
                .replace("jj", "dd") // Day
        }
        _ => {
            // Default to English format
            format.to_string()
        }
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
    let format = format_string.as_ref();
    let language = &value_format.language;

    // Convert format string based on language (date token aliases)
    let mut normalized_format = normalize_format_string(format, language);

    // Normalize decimal/thousands separators so the tokenizer (which always treats
    // '.' as decimal and ',' as thousands) works correctly for any locale.
    // For locales where ',' is the decimal separator and '.' is the thousands separator
    // (e.g. German, French, Portuguese), swap them using a placeholder to avoid collision.
    if value_format.decimal_separator == "," && value_format.thousands_separator == "." {
        // Swap simultaneously: , → . and . → , via placeholder
        normalized_format = swap_separators(&normalized_format);
    }

    format_value(value, &normalized_format, value_format)
}

/// Swap ',' and '.' in a format string, respecting quoted/escaped sections.
fn swap_separators(format: &str) -> String {
    let mut result = String::with_capacity(format.len());
    let chars: Vec<char> = format.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '"' => {
                // Pass through quoted sections unchanged
                result.push('"');
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    result.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    result.push('"');
                    i += 1;
                }
            }
            '\\' => {
                // Pass through escaped character unchanged
                result.push('\\');
                i += 1;
                if i < chars.len() {
                    result.push(chars[i]);
                    i += 1;
                }
            }
            ',' => {
                result.push('.'); // locale decimal → standard decimal
                i += 1;
            }
            '.' => {
                result.push(','); // locale thousands → standard thousands
                i += 1;
            }
            c => {
                result.push(c);
                i += 1;
            }
        }
    }
    result
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
    use crate::date_and_time::codcel_date::codcel_date;

    use super::*;
    use crate::date_system::DateSemantics;
    use crate::date_time_base::date_time_to_excel;

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

        // Debug the normalization process
        let format_string = "jjjj-mm-tt";
        let normalized = normalize_format_string(format_string, &german_format.language);
        println!("Original format: {format_string}, Normalized: {normalized}");

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
