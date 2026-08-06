// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use regex::Regex;
use std::error::Error;

/// Excel-compatible `REGEXTEST` function (Microsoft 365).
/// Tests whether text matches a regular expression pattern.
///
/// - `text`: the text to test.
/// - `pattern`: the regular expression pattern.
/// - `case_sensitivity`: optional (default 0).
///   - 0: case-sensitive.
///   - 1: case-insensitive.
///
/// Returns `true` if the pattern matches anywhere in the text, `false` otherwise.
pub fn codcel_regextest(
    text: &str,
    pattern: &str,
    case_sensitivity: Option<i32>,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let case_sensitivity = case_sensitivity.unwrap_or(0);

    let regex_pattern = if case_sensitivity == 1 {
        format!("(?i){pattern}")
    } else {
        pattern.to_string()
    };

    let regex = Regex::new(&regex_pattern)
        .map_err(|e| format!("#VALUE! REGEXTEST: Invalid regex pattern: {e}"))?;

    Ok(regex.is_match(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regextest_basic_match() {
        let result = codcel_regextest("test123abc", r"\d+", None).unwrap();
        assert!(result);
    }

    #[test]
    fn test_regextest_no_match() {
        let result = codcel_regextest("hello", r"\d+", None).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_regextest_case_sensitive_default() {
        let result = codcel_regextest("Hello World", "hello", None).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_regextest_case_insensitive() {
        let result = codcel_regextest("Hello World", "hello", Some(1)).unwrap();
        assert!(result);
    }

    #[test]
    fn test_regextest_case_sensitive_explicit() {
        let result = codcel_regextest("Hello World", "hello", Some(0)).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_regextest_full_match() {
        let result = codcel_regextest("abc", "^abc$", None).unwrap();
        assert!(result);
    }

    #[test]
    fn test_regextest_partial_match() {
        let result = codcel_regextest("abc123", r"\d+", None).unwrap();
        assert!(result);
    }

    #[test]
    fn test_regextest_empty_text() {
        let result = codcel_regextest("", r"\d+", None).unwrap();
        assert!(!result);
    }

    #[test]
    fn test_regextest_empty_pattern_matches() {
        let result = codcel_regextest("hello", "", None).unwrap();
        assert!(result);
    }

    #[test]
    fn test_regextest_invalid_regex() {
        let result = codcel_regextest("test", r"[invalid", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_regextest_unicode() {
        let result = codcel_regextest("中国香港", "国", None).unwrap();
        assert!(result);
    }

    #[test]
    fn test_regextest_email_pattern() {
        let result = codcel_regextest(
            "Contact us at info@example.com",
            r"[\w.+-]+@[\w-]+\.[\w.]+",
            None,
        )
        .unwrap();
        assert!(result);
    }

    #[test]
    fn test_regextest_word_boundary() {
        let result = codcel_regextest("The quick brown fox", r"\bquick\b", None).unwrap();
        assert!(result);
    }

    #[test]
    fn test_regextest_no_email_match() {
        let result = codcel_regextest("no email here", r"[\w.+-]+@[\w-]+\.[\w.]+", None).unwrap();
        assert!(!result);
    }
}
