// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::area::{
    process_area_f64_opt_int_opt_bool_to_string, process_area_float_op_int_to_string_value_format,
    process_area_float_string_to_string, process_area_generic, process_area_int_to_string,
    process_area_string_int_int_string_to_string, process_area_string_int_int_to_string,
    process_area_string_int_to_string, process_area_string_multi_to_bool,
    process_area_string_multi_to_float, process_area_string_opt_int_to_string,
    process_area_string_string_string_opt_int_to_string, process_area_string_to_int,
    process_area_string_to_string, process_area_value_opt_bool_to_string,
    process_area_value_to_float, process_area_value_to_string,
};
use crate::text::{
    codcel_array_to_text::codcel_array_to_text, codcel_asc::codcel_asc,
    codcel_bahttext::codcel_bahttext, codcel_char::codcel_char, codcel_dbcs::codcel_dbcs,
    codcel_jis::codcel_jis,
    codcel_clean::codcel_clean, codcel_code::codcel_code, codcel_dollar::codcel_dollar,
    codcel_exact::codcel_exact_vec, codcel_find::codcel_find, codcel_findb::codcel_findb,
    codcel_fixed::codcel_fixed,
    codcel_left::codcel_left, codcel_leftb::codcel_leftb, codcel_lower::codcel_lower,
    codcel_mid::codcel_mid, codcel_midb::codcel_midb,
    codcel_number_value::codcel_number_value_vec, codcel_phonetic::codcel_phonetic,
    codcel_proper::codcel_proper,
    codcel_regexextract::codcel_regexextract,
    codcel_regexreplace::codcel_regexreplace,
    codcel_regextest::codcel_regextest,
    codcel_replace::codcel_replace, codcel_replaceb::codcel_replaceb,
    codcel_rept::codcel_rept, codcel_right::codcel_right, codcel_rightb::codcel_rightb,
    codcel_search::codcel_search, codcel_searchb::codcel_searchb,
    codcel_substitute::codcel_substitute, codcel_t::codcel_t,
    codcel_text::codcel_text, codcel_text_after::codcel_text_after,
    codcel_text_before::codcel_text_before, codcel_text_split::codcel_text_split,
    codcel_trim::codcel_trim, codcel_uni_char::codcel_uni_char, codcel_unicode::codcel_unicode,
    codcel_upper::codcel_upper, codcel_value_to_text::codcel_value_to_text,
};
use crate::value::{area_string, Value};
use crate::value_format::ValueFormat;
use std::error::Error;

/// Returns true if the language is a DBCS (Double-Byte Character Set) locale.
/// DBCS/JIS/ASC functions only perform character width conversion in these locales;
/// in non-DBCS locales they return the input unchanged.
fn is_dbcs_locale(language: &str) -> bool {
    let lang = language.to_lowercase();
    lang.starts_with("ja") || lang.starts_with("zh") || lang.starts_with("ko")
}

/// Excel-compatible `FIND` function.
/// Finds the starting position of a substring within text (case-sensitive).
/// - `substring`: the text to find.
/// - `text`: the text to search within.
/// - `start_position`: optional position to start searching from (1-based, defaults to 1).
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the 1-based position of the first occurrence, or an error if not found.
pub fn find(
    substring: Value,
    text: Value,
    start_position: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let substring = substring.string(value_format)?;
    let start_position = start_position.option_i32(value_format)?;

    let values = text.area_of_value()?;

    // Resultant 2D array for the final output
    let mut result: Vec<Vec<Value>> = Vec::new();

    // Iterate over each row in the 2D array
    for row in values.iter() {
        let mut result_row: Vec<Value> = Vec::new();

        // Iterate over each value (cell) in the row
        for value in row.iter() {
            let value = value
                .string(value_format)
                .expect("FIND: Text must be a string");

            let result = codcel_find(&substring, &value, start_position)?;

            // Add the processed result to the result row
            result_row.push(Value::I32(result));
        }

        // Add the processed row to the final result
        result.push(result_row);
    }

    // Return the 2D array as the final result
    Ok(Value::AreaValue(result))
}

/// Excel-compatible `FINDB` function.
/// Finds the starting byte position of a substring within text (case-sensitive).
/// Unlike FIND which counts characters, FINDB counts bytes. For ASCII text, results are identical.
/// - `substring`: the text to find.
/// - `text`: the text to search within.
/// - `start_position`: optional byte position to start searching from (1-based, defaults to 1).
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the 1-based byte position of the first occurrence, or an error if not found.
pub fn findb(
    substring: Value,
    text: Value,
    start_position: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let substring = substring.string(value_format)?;
    let start_position = start_position.option_i32(value_format)?;

    let values = text.area_of_value()?;

    // Resultant 2D array for the final output
    let mut result: Vec<Vec<Value>> = Vec::new();

    // Iterate over each row in the 2D array
    for row in values.iter() {
        let mut result_row: Vec<Value> = Vec::new();

        // Iterate over each value (cell) in the row
        for value in row.iter() {
            let value = value
                .string(value_format)
                .expect("FINDB: Text must be a string");

            let result = codcel_findb(&substring, &value, start_position)?;

            // Add the processed result to the result row
            result_row.push(Value::I32(result));
        }

        // Add the processed row to the final result
        result.push(result_row);
    }

    // Return the 2D array as the final result
    Ok(Value::AreaValue(result))
}

/// Excel-compatible `SEARCH` function.
/// Finds the starting position of a substring within text (case-insensitive).
/// Supports wildcard characters: `?` matches any single character, `*` matches any sequence.
/// - `substring`: the text to find (may contain wildcards).
/// - `text`: the text to search within.
/// - `start_position`: optional position to start searching from (1-based, defaults to 1).
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the 1-based position of the first occurrence, or an error if not found.
pub fn search(
    substring: Value,
    text: Value,
    start_position: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let substring = substring.string(value_format)?;
    let start_position = start_position.option_i32(value_format)?;

    let values = text.area_of_value()?;

    // Resultant 2D array for the final output
    let mut result: Vec<Vec<Value>> = Vec::new();

    // Iterate over each row in the 2D array
    for row in values.iter() {
        let mut result_row: Vec<Value> = Vec::new();

        // Iterate over each value (cell) in the row
        for value in row.iter() {
            let value = value
                .string(value_format)
                .expect("SEARCH: Text must be a string");

            let result = codcel_search(&substring, &value, start_position)?;

            // Add the processed result to the result row
            result_row.push(Value::I32(result));
        }

        // Add the processed row to the final result
        result.push(result_row);
    }

    // Return the 2D array as the final result
    Ok(Value::AreaValue(result))
}

/// Excel-compatible `SEARCHB` function.
/// Finds the starting byte position of a substring within text (case-insensitive).
/// Supports wildcard characters: `?` matches any single character, `*` matches any sequence.
/// Unlike SEARCH which counts characters, SEARCHB counts bytes. For ASCII text, results are identical.
/// - `substring`: the text to find (may contain wildcards).
/// - `text`: the text to search within.
/// - `start_position`: optional byte position to start searching from (1-based, defaults to 1).
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the 1-based byte position of the first occurrence, or an error if not found.
pub fn searchb(
    substring: Value,
    text: Value,
    start_position: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let substring = substring.string(value_format)?;
    let start_position = start_position.option_i32(value_format)?;

    let values = text.area_of_value()?;

    // Resultant 2D array for the final output
    let mut result: Vec<Vec<Value>> = Vec::new();

    // Iterate over each row in the 2D array
    for row in values.iter() {
        let mut result_row: Vec<Value> = Vec::new();

        // Iterate over each value (cell) in the row
        for value in row.iter() {
            let value = value
                .string(value_format)
                .expect("SEARCHB: Text must be a string");

            let result = codcel_searchb(&substring, &value, start_position)?;

            // Add the processed result to the result row
            result_row.push(Value::I32(result));
        }

        // Add the processed row to the final result
        result.push(result_row);
    }

    // Return the 2D array as the final result
    Ok(Value::AreaValue(result))
}

/// Excel-compatible `REGEXEXTRACT` function (Microsoft 365).
/// Extracts substrings from text that match a regular expression pattern.
/// - `text`: the text to extract from.
/// - `pattern`: the regular expression pattern.
/// - `return_mode`: optional; 0=first match (default), 1=all matches, 2=capture groups.
/// - `case_sensitivity`: optional; 0=case-sensitive (default), 1=case-insensitive.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns a single string (mode 0) or an array of strings (modes 1, 2).
pub fn regexextract(
    text: Value,
    pattern: Value,
    return_mode: Value,
    case_sensitivity: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let pattern = pattern.string(value_format)?;
    let return_mode = return_mode.option_i32(value_format)?;
    let case_sensitivity = case_sensitivity.option_i32(value_format)?;

    let matches = codcel_regexextract(&text, &pattern, return_mode, case_sensitivity)?;

    Ok(area_string(vec![matches]))
}

/// Excel-compatible `REGEXREPLACE` function (Microsoft 365).
/// Replaces text matched by a regular expression with replacement text.
/// - `text`: the text to search within.
/// - `pattern`: the regular expression pattern.
/// - `replacement`: the replacement text (supports `$1`, `$2` backreferences).
/// - `instance_num`: optional; 0=replace all (default), N=replace Nth match.
/// - `case_sensitivity`: optional; 0=case-sensitive (default), 1=case-insensitive.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the modified text.
pub fn regexreplace(
    text: Value,
    pattern: Value,
    replacement: Value,
    instance_num: Value,
    case_sensitivity: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let pattern = pattern.string(value_format)?;
    let replacement = replacement.string(value_format)?;
    let instance_num = instance_num.option_i32(value_format)?;
    let case_sensitivity = case_sensitivity.option_i32(value_format)?;

    let result = codcel_regexreplace(&text, &pattern, &replacement, instance_num, case_sensitivity)?;

    Ok(Value::String(result))
}

/// Excel-compatible `REGEXTEST` function (Microsoft 365).
/// Tests whether text matches a regular expression pattern.
/// - `text`: the text to test.
/// - `pattern`: the regular expression pattern.
/// - `case_sensitivity`: optional; 0=case-sensitive (default), 1=case-insensitive.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns `true` if the pattern matches, `false` otherwise.
pub fn regextest(
    text: Value,
    pattern: Value,
    case_sensitivity: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let pattern = pattern.string(value_format)?;
    let case_sensitivity = case_sensitivity.option_i32(value_format)?;

    let result = codcel_regextest(&text, &pattern, case_sensitivity)?;

    Ok(Value::Bool(result))
}

/// Excel-compatible `CHAR` function.
/// Returns the character specified by a code number from the current character set.
/// - `area`: a number between 1 and 255.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the corresponding character, or an error if the number is out of range.
pub fn char(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_to_string(
        area,
        strict_type_conversion,
        value_format,
        codcel_char,
        "CHAR",
    )
}

/// Excel-compatible `ASC` function.
/// Converts full-width (double-byte) characters to half-width (single-byte) characters.
/// Primarily used for East Asian language support.
/// - `area`: text containing full-width characters to convert.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text with full-width characters converted to half-width equivalents.
pub fn asc(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if is_dbcs_locale(&value_format.language) {
        process_area_string_to_string(area, strict_type_conversion, value_format, codcel_asc, "ASC")
    } else {
        process_area_string_to_string(area, strict_type_conversion, value_format, Ok, "ASC")
    }
}

/// Excel-compatible `DBCS` function.
/// Converts half-width (single-byte) characters to full-width (double-byte) characters.
/// Only performs conversion in DBCS locales (Japanese, Chinese, Korean).
/// In non-DBCS locales, returns the input unchanged.
pub fn dbcs(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if is_dbcs_locale(&value_format.language) {
        process_area_string_to_string(area, strict_type_conversion, value_format, codcel_dbcs, "DBCS")
    } else {
        process_area_string_to_string(area, strict_type_conversion, value_format, Ok, "DBCS")
    }
}

/// Excel-compatible `JIS` function.
/// Converts half-width (single-byte) characters to full-width (double-byte) characters.
/// Functionally identical to DBCS — JIS is the Japanese locale name for the same operation.
/// Only performs conversion in DBCS locales (Japanese, Chinese, Korean).
/// In non-DBCS locales, returns the input unchanged.
pub fn jis(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if is_dbcs_locale(&value_format.language) {
        process_area_string_to_string(area, strict_type_conversion, value_format, codcel_jis, "JIS")
    } else {
        process_area_string_to_string(area, strict_type_conversion, value_format, Ok, "JIS")
    }
}

/// Excel-compatible `BAHTTEXT` function.
/// Converts a number to Thai text representing Thai Baht currency.
/// - `area`: the number to convert.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the Thai Baht text representation of the number.
pub fn bahttext(
    area: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_generic(
        area,
        value_format,
        codcel_bahttext,
        "BAHTTEXT",
        |value, vf| value.f64(vf),
        Value::String,
    )
}

/// Excel-compatible `CODE` function.
/// Returns the numeric code for the first character in a text string.
/// - `area`: text whose first character's code is returned.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the character code (1-255 for standard characters), or an error for empty text.
pub fn code(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_to_int(
        area,
        strict_type_conversion,
        value_format,
        codcel_code,
        "CODE",
    )
}

/// Excel-compatible `ARRAYTOTEXT` function.
/// Converts an array of values to a text representation.
/// - `array`: the array or range of values to convert.
/// - `format`: optional format mode (`false` or omitted for concise, `true` for strict format with quotes and braces).
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns a text string representing the array contents.
pub fn array_to_text(
    array: Value,
    format: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = array.area_of_string(strict_type_conversion, value_format)?;
    let format = format.option_bool(value_format)?;
    Ok(Value::String(codcel_array_to_text(array, format)?))
}

/// Excel-compatible `CLEAN` function.
/// Removes all non-printable characters (ASCII codes 0-31) from text.
/// - `area`: text to clean.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text with non-printable characters removed.
pub fn clean(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_to_string(
        area,
        strict_type_conversion,
        value_format,
        codcel_clean,
        "CLEAN",
    )
}

/// Excel-compatible `TRIM` function.
/// Removes extra spaces from text, leaving only single spaces between words.
/// Also removes leading and trailing spaces.
/// - `area`: text to trim.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text with extra spaces removed.
pub fn trim(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_to_string(
        area,
        strict_type_conversion,
        value_format,
        codcel_trim,
        "TRIM",
    )
}

/// Excel-compatible `DOLLAR` function.
/// Converts a number to text using currency format with rounded decimals.
/// - `area`: the number to format.
/// - `decimals`: optional number of decimal places (defaults to 2).
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings (determines currency symbol and separators).
///
/// Returns the number formatted as currency text (e.g., "$1,234.57").
pub fn dollar(
    area: Value,
    decimals: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_op_int_to_string_value_format(
        area,
        decimals,
        strict_type_conversion,
        value_format,
        2,
        "DOLLAR",
        codcel_dollar,
    )
}

/// Excel-compatible `UNICHAR` function.
/// Returns the Unicode character referenced by the given code point.
/// - `area`: a Unicode code point (positive integer).
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the corresponding Unicode character, or an error for invalid code points.
pub fn uni_char(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_to_string(
        area,
        strict_type_conversion,
        value_format,
        codcel_uni_char,
        "UNICHAR",
    )
}

/// Excel-compatible `UNICODE` function.
/// Returns the Unicode code point for the first character of a text string.
/// - `area`: text whose first character's Unicode value is returned.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the Unicode code point as an integer, or an error for empty text.
pub fn unicode(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_to_int(
        area,
        strict_type_conversion,
        value_format,
        codcel_unicode,
        "UNICODE",
    )
}

/// Excel-compatible `EXACT` function.
/// Compares two text strings and returns `true` if they are identical (case-sensitive).
/// - `area`: the first text string to compare.
/// - `text2`: the second text string to compare.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns `true` if the strings are exactly equal, `false` otherwise.
pub fn exact(
    area: Value,
    text2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_multi_to_bool(
        vec![area, text2],
        strict_type_conversion,
        value_format,
        "EXACT",
        codcel_exact_vec,
    )
}

/// Excel-compatible `LOWER` function.
/// Converts all uppercase letters in a text string to lowercase.
/// - `area`: text to convert.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text with all letters converted to lowercase.
pub fn lower(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_to_string(
        area,
        strict_type_conversion,
        value_format,
        codcel_lower,
        "LOWER",
    )
}

/// Excel-compatible `UPPER` function.
/// Converts all lowercase letters in a text string to uppercase.
/// - `area`: text to convert.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text with all letters converted to uppercase.
pub fn upper(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_to_string(
        area,
        strict_type_conversion,
        value_format,
        codcel_upper,
        "UPPER",
    )
}

/// Excel-compatible `VALUE` function.
/// Converts a text string that represents a number to a numeric value.
/// Handles currency symbols, percentage signs, and locale-specific formatting.
/// - `area`: text representing a number.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the numeric value, or an error if the text cannot be parsed as a number.
pub fn value(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_float(area, strict_type_conversion, value_format, xlsx_value)
}

pub(crate) fn xlsx_value(
    value: &Value,
    value_format: &ValueFormat,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if let Ok(val) = value.f64(value_format) {
        return Ok(val);
    }

    let value_string = value.string(value_format)?;

    // Remove the first character if it is not a "-" or a number
    // as it could be a currency symbol
    let value_string = if let Some(first_char) = value_string.chars().next() {
        if !first_char.is_numeric() && first_char != '-' {
            // Safely skip the first character
            value_string[first_char.len_utf8()..].trim().to_string()
        } else {
            value_string
        }
    } else {
        value_string
    };

    let value_string = value_string.replace(&value_format.thousands_separator, "");

    let val = Value::String(value_string);

    val.f64(value_format)
}

/// Excel-compatible `CONCATENATE` function.
/// Joins multiple text strings into one string.
/// - `values`: one or more text values or ranges to concatenate.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns a single string containing all inputs joined together.
///
/// Note: Use `CONCAT` or `TEXTJOIN` for newer Excel-style behavior.
pub fn concatenate(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let mut result_string = String::new();

    for value in values {
        let strings_in_array = value
            .vec_string(value_format)
            .expect("CONCATENATE: Input values are not strings");
        for s in strings_in_array {
            result_string.push_str(&s);
        }
    }

    Ok(Value::String(result_string))
}

/// Excel-compatible `CONCAT` function.
/// Joins multiple text strings into one string.
/// Unlike `CONCATENATE`, this function accepts ranges and arrays directly.
/// - `values`: one or more text values, ranges, or arrays to concatenate.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns a single string containing all inputs joined together.
pub fn concat(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    concatenate(values, value_format)
}

/// Excel-compatible `TEXTJOIN` function.
/// Joins text from multiple ranges/strings with a delimiter.
/// - `delimiter`: the text to insert between each joined element.
/// - `ignore_empty`: `true` to skip empty cells, `false` to include them.
/// - `values`: one or more text values, ranges, or arrays to join.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns a single string with all values joined by the delimiter.
pub fn text_join(
    delimiter: Value,
    ignore_empty: Value,
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let delimiter = delimiter.string(value_format)?;
    let ignore_empty = ignore_empty.bool(value_format)?;
    let mut result_string = String::new();
    let mut first_value_added = false;

    for value in values {
        let mut value_strings: Vec<String> = Vec::new();

        if value.is_array() {
            // Get the strings in the array
            value_strings.extend(
                value
                    .vec_string(value_format)
                    .expect("TEXTJOIN: Input values are not strings"),
            );
        } else {
            // Get the single string value
            value_strings.push(
                value
                    .string(value_format)
                    .expect("TEXTJOIN: Input value is not a string"),
            );
        }

        for s in value_strings {
            if !ignore_empty || !s.is_empty() {
                if first_value_added {
                    result_string.push_str(&delimiter);
                }
                result_string.push_str(&s);
                first_value_added = true;
            }
        }
    }

    Ok(Value::String(result_string))
}

/// Excel-compatible `MID` function.
/// Extracts a substring from the middle of a text string.
/// - `text`: the text string to extract from.
/// - `start_num`: the 1-based position of the first character to extract.
/// - `num_chars`: the number of characters to extract.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the extracted substring, or an error if start_num is less than 1.
pub fn mid(
    text: Value,
    start_num: Value,
    num_chars: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_int_int_to_string(
        text,
        start_num,
        num_chars,
        strict_type_conversion,
        value_format,
        "MID",
        codcel_mid,
    )
}

/// Excel-compatible `MIDB` function.
/// Returns characters from the middle of a text string based on byte positions.
/// Unlike MID which counts characters, MIDB counts bytes. For ASCII text, results are identical.
/// - `text`: the text string containing the characters to extract.
/// - `start_num`: the 1-based byte position of the first byte to extract.
/// - `num_bytes`: the number of bytes to extract.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the extracted substring, or an error if start_num is less than 1.
pub fn midb(
    text: Value,
    start_num: Value,
    num_bytes: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_int_int_to_string(
        text,
        start_num,
        num_bytes,
        strict_type_conversion,
        value_format,
        "MIDB",
        codcel_midb,
    )
}

/// Excel-compatible `REPLACE` function.
/// Replaces part of a text string with a different text string, based on position.
/// - `text`: the original text string.
/// - `start_num`: the 1-based position where replacement begins.
/// - `num_chars`: the number of characters to replace.
/// - `replace`: the new text to insert.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text with the specified portion replaced.
pub fn replace(
    text: Value,
    start_num: Value,
    num_chars: Value,
    replace: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_int_int_string_to_string(
        text,
        start_num,
        num_chars,
        replace,
        strict_type_conversion,
        value_format,
        "REPLACE",
        codcel_replace,
    )
}
/// Excel-compatible `REPLACEB` function.
/// Replaces part of a text string based on byte positions.
/// Unlike REPLACE which counts characters, REPLACEB counts bytes.
pub fn replaceb(
    text: Value,
    start_num: Value,
    num_bytes: Value,
    replace: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_int_int_string_to_string(
        text,
        start_num,
        num_bytes,
        replace,
        strict_type_conversion,
        value_format,
        "REPLACEB",
        codcel_replaceb,
    )
}
/// Excel-compatible `SUBSTITUTE` function.
/// Replaces occurrences of a substring within text with new text.
/// - `text`: the original text string.
/// - `old_text`: the text to find and replace.
/// - `new_text`: the replacement text.
/// - `instant_num`: optional occurrence number to replace (if omitted, replaces all occurrences).
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text with substitutions made.
pub fn substitute(
    text: Value,
    old_text: Value,
    new_text: Value,
    instant_num: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_string_string_opt_int_to_string(
        text,
        old_text,
        new_text,
        instant_num,
        strict_type_conversion,
        value_format,
        "SUBSTITUTE",
        codcel_substitute,
    )
}

/// Excel-compatible `TEXT` function.
/// Formats a number as text using a specified format string.
/// - `value`: the number to format.
/// - `format_string`: the format pattern (e.g., "0.00", "#,##0", "mm/dd/yyyy").
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the formatted number as text.
pub fn text(
    value: Value,
    format_string: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Excel: TEXT(TRUE, "@") returns "TRUE", TEXT(FALSE, "@") returns "FALSE".
    // The generic processor converts Bool→f64 first, losing boolean identity.
    // Handle boolean + "@" format specially.
    if let Value::Bool(b) = &value {
        if let Ok(fmt) = format_string.string(value_format) {
            if fmt == "@" {
                let text = if *b { "TRUE" } else { "FALSE" };
                return Ok(Value::String(text.to_string()));
            }
        }
    }
    process_area_float_string_to_string(
        value,
        format_string,
        strict_type_conversion,
        value_format,
        "TEXT",
        |v, s| codcel_text(v, s, value_format),
    )
}

/// Excel-compatible `NUMBERVALUE` function.
/// Converts text to a number, independent of locale.
/// - `text`: the text representing a number.
/// - `decimal_separator`: optional character used as decimal point (defaults to locale setting).
/// - `group_separator`: optional character used as thousands separator (defaults to locale setting).
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the numeric value, or an error if conversion fails.
pub fn number_value(
    text: Value,
    decimal_separator: Value,
    group_separator: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let decimal_separator = decimal_separator.option_area_of_value()?;
    let decimal_separator = if let Some(value) = decimal_separator {
        value
    } else {
        vec![vec![Value::String(
            value_format.decimal_separator.to_string(),
        )]]
    };

    let group_separator = group_separator.option_area_of_value()?;
    let group_separator = if let Some(value) = group_separator {
        value
    } else {
        vec![vec![Value::String(
            value_format.thousands_separator.to_string(),
        )]]
    };

    process_area_string_multi_to_float(
        vec![
            text,
            Value::AreaValue(decimal_separator),
            Value::AreaValue(group_separator),
        ],
        strict_type_conversion,
        value_format,
        "NUMBERVALUE",
        codcel_number_value_vec,
    )
}

/// Excel-compatible `PROPER` function.
/// Capitalizes the first letter of each word in a text string.
/// Converts other letters to lowercase.
/// - `area`: text to convert.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text with proper capitalization (e.g., "hello world" becomes "Hello World").
pub fn proper(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_to_string(
        area,
        strict_type_conversion,
        value_format,
        codcel_proper,
        "PROPER",
    )
}

/// Excel-compatible `PHONETIC` function.
/// Returns the phonetic (furigana) characters from a text string.
/// For non-annotated text, returns the input unchanged.
/// - `area`: text to extract phonetic characters from.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text unchanged (furigana metadata is not available in plain strings).
pub fn phonetic(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_to_string(
        area,
        strict_type_conversion,
        value_format,
        codcel_phonetic,
        "PHONETIC",
    )
}

/// Excel-compatible `REPT` function.
/// Repeats text a specified number of times.
/// - `text`: the text to repeat.
/// - `number_times`: the number of times to repeat (must be non-negative).
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the repeated text string, or an empty string if number_times is 0.
pub fn rept(
    text: Value,
    number_times: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_int_to_string(
        text,
        number_times,
        strict_type_conversion,
        value_format,
        "REPT",
        codcel_rept,
    )
}

/// Excel-compatible `FIXED` function.
/// Formats a number as text with a fixed number of decimal places.
/// - `number`: the number to format.
/// - `decimal_places`: optional number of decimal places (defaults to 2).
/// - `no_commas`: optional; `true` to omit thousands separators, `false` (default) to include them.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the number formatted as text with the specified decimal places.
pub fn fixed(
    number: Value,
    decimal_places: Value,
    no_commas: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_f64_opt_int_opt_bool_to_string(
        number,
        decimal_places,
        no_commas,
        strict_type_conversion,
        value_format,
        "FIXED",
        codcel_fixed,
    )
}

/// Excel-compatible `LEFT` function.
/// Extracts a specified number of characters from the beginning of a text string.
/// - `text`: the text string to extract from.
/// - `num_chars`: optional number of characters to extract (defaults to 1).
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the leftmost characters from the text string.
pub fn left(
    text: Value,
    num_chars: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_opt_int_to_string(
        text,
        num_chars,
        strict_type_conversion,
        value_format,
        "LEFT",
        codcel_left,
    )
}

/// Excel-compatible `LEFTB` function.
/// Extracts characters from the beginning of a text string based on byte count.
/// Unlike LEFT which counts characters, LEFTB counts bytes. For ASCII text, results are identical.
/// When the byte count falls in the middle of a multi-byte character, truncates to the last
/// complete character that fits.
/// - `text`: the text string to extract from.
/// - `num_bytes`: optional number of bytes to extract (defaults to 1).
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the leftmost characters from the text string that fit within the byte count.
pub fn leftb(
    text: Value,
    num_bytes: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_opt_int_to_string(
        text,
        num_bytes,
        strict_type_conversion,
        value_format,
        "LEFTB",
        codcel_leftb,
    )
}

/// Excel-compatible `RIGHT` function.
/// Extracts a specified number of characters from the end of a text string.
/// - `text`: the text string to extract from.
/// - `num_chars`: optional number of characters to extract (defaults to 1).
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the rightmost characters from the text string.
pub fn right(
    text: Value,
    num_chars: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_opt_int_to_string(
        text,
        num_chars,
        strict_type_conversion,
        value_format,
        "RIGHT",
        codcel_right,
    )
}

/// Excel-compatible `RIGHTB` function.
/// Extracts characters from the end of a text string based on byte count.
/// Unlike RIGHT which counts characters, RIGHTB counts bytes. For ASCII text, results are identical.
/// When the byte count falls in the middle of a multi-byte character, truncates to the next
/// complete character that fits.
/// - `text`: the text string to extract from.
/// - `num_bytes`: optional number of bytes to extract (defaults to 1).
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the rightmost characters from the text string that fit within the byte count.
pub fn rightb(
    text: Value,
    num_bytes: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_opt_int_to_string(
        text,
        num_bytes,
        strict_type_conversion,
        value_format,
        "RIGHTB",
        codcel_rightb,
    )
}

/// Excel-compatible `TEXTAFTER` function.
/// Returns text that occurs after a specified delimiter.
/// - `text`: the text to search within.
/// - `delimiter`: the text that marks the end of the portion to skip.
/// - `instance_number`: optional; which occurrence of delimiter to use (defaults to 1, negative counts from end).
/// - `match_mode`: optional; `true` for case-insensitive matching, `false` (default) for case-sensitive.
/// - `not_found`: optional; value to return if delimiter is not found (defaults to error).
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text after the specified delimiter occurrence.
pub fn text_after(
    text: Value,
    delimiter: Value,
    instance_number: Value,
    match_mode: Value,
    not_found: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let delimiter = delimiter.string(value_format)?;
    let instance_number = instance_number.option_i32(value_format)?;
    let match_mode = match_mode.option_bool(value_format)?;
    let not_found = not_found.option_string(value_format)?;

    Ok(Value::String(codcel_text_after(
        text,
        delimiter,
        instance_number,
        match_mode,
        not_found,
    )?))
}

/// Excel-compatible `TEXTBEFORE` function.
/// Returns text that occurs before a specified delimiter.
/// - `text`: the text to search within.
/// - `delimiter`: the text that marks the beginning of the portion to skip.
/// - `instance_number`: optional; which occurrence of delimiter to use (defaults to 1, negative counts from end).
/// - `match_mode`: optional; `true` for case-insensitive matching, `false` (default) for case-sensitive.
/// - `not_found`: optional; value to return if delimiter is not found (defaults to error).
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the text before the specified delimiter occurrence.
pub fn text_before(
    text: Value,
    delimiter: Value,
    instance_number: Value,
    match_mode: Value,
    not_found: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let delimiter = delimiter.string(value_format)?;
    let instance_number = instance_number.option_i32(value_format)?;
    let match_mode = match_mode.option_bool(value_format)?;
    let not_found = not_found.option_string(value_format)?;

    Ok(Value::String(codcel_text_before(
        text,
        delimiter,
        instance_number,
        match_mode,
        not_found,
    )?))
}

/// Excel-compatible `TEXTSPLIT` function.
/// Splits text into columns and/or rows using specified delimiters.
/// - `text`: the text to split.
/// - `col_delimiter`: the delimiter for splitting into columns.
/// - `row_delimiter`: optional delimiter for splitting into rows.
/// - `ignore_empty`: optional; `true` to skip empty values, `false` (default) to include them.
/// - `match_mode`: optional; `0` for case-sensitive (default), `1` for case-insensitive.
/// - `pad_with`: optional value to pad shorter rows (defaults to #N/A).
/// - `value_format`: locale-specific formatting settings.
///
/// Returns a 2D array of text split by the specified delimiters.
pub fn text_split(
    text: Value,
    col_delimiter: Value,
    row_delimiter: Value,
    ignore_empty: Value,
    match_mode: Value,
    pad_with: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let col_delimiter = col_delimiter.string(value_format)?;
    let row_delimiter = row_delimiter.option_string(value_format)?;
    let ignore_empty = ignore_empty.option_bool(value_format)?;
    let match_mode = match_mode.option_i32(value_format)?;
    let pad_with = pad_with.option_string(value_format)?;

    Ok(area_string(codcel_text_split(
        text,
        col_delimiter,
        row_delimiter,
        ignore_empty,
        match_mode,
        pad_with,
    )?))
}

/// Excel-compatible `VALUETOTEXT` function.
/// Converts a value to text in a specific format.
/// - `value`: the value to convert to text.
/// - `format`: optional; `false` (default) for concise format, `true` for strict format with type indicators.
/// - `strict_type_conversion`: whether to enforce strict type conversion.
/// - `value_format`: locale-specific formatting settings.
///
/// Returns the value represented as text.
pub fn value_to_text(
    value: Value,
    format: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_opt_bool_to_string(
        value,
        format,
        strict_type_conversion,
        value_format,
        "VALUETOTEXT",
        codcel_value_to_text,
    )
}

/// Excel-compatible `T` function.
/// Returns the text if the value is text, otherwise returns an empty string.
/// Useful for ensuring a value is treated as text.
/// - `value`: the value to test and return.
/// - `_strict_type_conversion`: (unused) whether to enforce strict type conversion.
/// - `_value_format`: (unused) locale-specific formatting settings.
///
/// Returns the text value if input is text, otherwise returns an empty string.
pub fn t(
    value: Value,
    _strict_type_conversion: bool,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_string(value, "T", codcel_t)
}
