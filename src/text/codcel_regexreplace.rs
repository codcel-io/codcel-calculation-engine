// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use regex::Regex;
use std::error::Error;

/// Excel-compatible `REGEXREPLACE` function (Microsoft 365).
/// Replaces text matched by a regular expression with replacement text.
///
/// - `text`: the text to search within.
/// - `pattern`: the regular expression pattern.
/// - `replacement`: the replacement text (supports `$1`, `$2` backreferences).
/// - `instance_num`: optional (default 0).
///   - 0: replace all matches.
///   - N > 0: replace only the Nth match.
/// - `case_sensitivity`: optional (default 0).
///   - 0: case-sensitive.
///   - 1: case-insensitive.
///
/// Returns the modified text string.
pub fn codcel_regexreplace(
    text: &str,
    pattern: &str,
    replacement: &str,
    instance_num: Option<i32>,
    case_sensitivity: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let instance_num = instance_num.unwrap_or(0);
    let case_sensitivity = case_sensitivity.unwrap_or(0);

    if instance_num < 0 {
        return Err("#VALUE! REGEXREPLACE: instance_num must be 0 or positive.".into());
    }

    let regex_pattern = if case_sensitivity == 1 {
        format!("(?i){pattern}")
    } else {
        pattern.to_string()
    };

    let regex = Regex::new(&regex_pattern)
        .map_err(|e| format!("#VALUE! REGEXREPLACE: Invalid regex pattern: {e}"))?;

    if instance_num == 0 {
        // Replace all matches
        Ok(regex.replace_all(text, replacement).to_string())
    } else {
        // Replace only the Nth match
        let mut result = String::new();
        let mut count = 0;

        for caps in regex.captures_iter(text) {
            count += 1;
            if count == instance_num {
                let Some(whole_match) = caps.get(0) else {
                    continue;
                };
                result.push_str(&text[..whole_match.start()]);
                caps.expand(replacement, &mut result);
                result.push_str(&text[whole_match.end()..]);
                return Ok(result);
            }
        }

        // Nth instance not found, return original text
        Ok(text.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regexreplace_all_occurrences() {
        let result = codcel_regexreplace("abc123def456", r"\d+", "NUM", None, None).unwrap();
        assert_eq!(result, "abcNUMdefNUM");
    }

    #[test]
    fn test_regexreplace_all_explicit_zero() {
        let result = codcel_regexreplace("abc123def456", r"\d+", "NUM", Some(0), None).unwrap();
        assert_eq!(result, "abcNUMdefNUM");
    }

    #[test]
    fn test_regexreplace_first_instance() {
        let result = codcel_regexreplace("abc123def456", r"\d+", "NUM", Some(1), None).unwrap();
        assert_eq!(result, "abcNUMdef456");
    }

    #[test]
    fn test_regexreplace_second_instance() {
        let result = codcel_regexreplace("abc123def456", r"\d+", "NUM", Some(2), None).unwrap();
        assert_eq!(result, "abc123defNUM");
    }

    #[test]
    fn test_regexreplace_instance_not_found() {
        let result = codcel_regexreplace("abc123def456", r"\d+", "NUM", Some(3), None).unwrap();
        assert_eq!(result, "abc123def456");
    }

    #[test]
    fn test_regexreplace_no_match() {
        let result = codcel_regexreplace("hello world", r"\d+", "NUM", None, None).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_regexreplace_case_sensitive_default() {
        let result = codcel_regexreplace("Hello hello HELLO", "hello", "HI", None, None).unwrap();
        assert_eq!(result, "Hello HI HELLO");
    }

    #[test]
    fn test_regexreplace_case_insensitive() {
        let result =
            codcel_regexreplace("Hello hello HELLO", "hello", "HI", None, Some(1)).unwrap();
        assert_eq!(result, "HI HI HI");
    }

    #[test]
    fn test_regexreplace_case_insensitive_first_instance() {
        let result =
            codcel_regexreplace("Hello hello HELLO", "hello", "HI", Some(1), Some(1)).unwrap();
        assert_eq!(result, "HI hello HELLO");
    }

    #[test]
    fn test_regexreplace_backreference() {
        let result = codcel_regexreplace(
            "2024-01-15",
            r"(\d{4})-(\d{2})-(\d{2})",
            "$2/$3/$1",
            None,
            None,
        )
        .unwrap();
        assert_eq!(result, "01/15/2024");
    }

    #[test]
    fn test_regexreplace_backreference_nth_instance() {
        let result =
            codcel_regexreplace("foo:bar baz:qux", r"(\w+):(\w+)", "$2:$1", Some(2), None).unwrap();
        assert_eq!(result, "foo:bar qux:baz");
    }

    #[test]
    fn test_regexreplace_empty_replacement() {
        let result = codcel_regexreplace("abc123def", r"\d+", "", None, None).unwrap();
        assert_eq!(result, "abcdef");
    }

    #[test]
    fn test_regexreplace_invalid_regex() {
        let result = codcel_regexreplace("test", r"[invalid", "x", None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_regexreplace_negative_instance_num() {
        let result = codcel_regexreplace("test", r"t", "x", Some(-1), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_regexreplace_empty_text() {
        let result = codcel_regexreplace("", r"\d+", "NUM", None, None).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_regexreplace_unicode() {
        let result = codcel_regexreplace("中国香港", "国", "國", None, None).unwrap();
        assert_eq!(result, "中國香港");
    }

    #[test]
    fn test_regexreplace_word_pattern() {
        let result =
            codcel_regexreplace("The quick brown fox", r"\b\w{5}\b", "WORD", None, None).unwrap();
        assert_eq!(result, "The WORD WORD fox");
    }

    #[test]
    fn test_regexreplace_literal_dollar_in_replacement() {
        let result = codcel_regexreplace("price 100", r"\d+", "$$200", None, None).unwrap();
        assert_eq!(result, "price $200");
    }
}
