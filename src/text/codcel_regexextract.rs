// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use regex::Regex;
use std::error::Error;

/// Excel-compatible `REGEXEXTRACT` function (Microsoft 365).
/// Extracts substrings from text that match a regular expression pattern.
///
/// - `text`: the text to extract from.
/// - `pattern`: the regular expression pattern.
/// - `return_mode`: optional (default 0).
///   - 0: return the first match.
///   - 1: return all matches.
///   - 2: return capture groups from the first match.
/// - `case_sensitivity`: optional (default 0).
///   - 0: case-sensitive.
///   - 1: case-insensitive.
///
/// Returns a `Vec<String>` of matched text. Always returns text values.
pub fn codcel_regexextract(
    text: &str,
    pattern: &str,
    return_mode: Option<i32>,
    case_sensitivity: Option<i32>,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let return_mode = return_mode.unwrap_or(0);
    let case_sensitivity = case_sensitivity.unwrap_or(0);

    if !(0..=2).contains(&return_mode) {
        return Err("#VALUE! REGEXEXTRACT: return_mode must be 0, 1, or 2.".into());
    }

    let regex_pattern = if case_sensitivity == 1 {
        format!("(?i){pattern}")
    } else {
        pattern.to_string()
    };

    let regex = Regex::new(&regex_pattern)
        .map_err(|e| format!("#VALUE! REGEXEXTRACT: Invalid regex pattern: {e}"))?;

    match return_mode {
        0 => {
            // Return the first match
            if let Some(m) = regex.find(text) {
                Ok(vec![m.as_str().to_string()])
            } else {
                Err("#N/A REGEXEXTRACT: No match found.".into())
            }
        }
        1 => {
            // Return all matches
            let matches: Vec<String> = regex
                .find_iter(text)
                .map(|m| m.as_str().to_string())
                .collect();
            if matches.is_empty() {
                Err("#N/A REGEXEXTRACT: No match found.".into())
            } else {
                Ok(matches)
            }
        }
        2 => {
            // Return capture groups from the first match
            if let Some(caps) = regex.captures(text) {
                let groups: Vec<String> = caps
                    .iter()
                    .skip(1) // skip the full match (group 0)
                    .map(|m| m.map_or(String::new(), |m| m.as_str().to_string()))
                    .collect();
                if groups.is_empty() {
                    // No capture groups in pattern — return the full match
                    Ok(vec![caps[0].to_string()])
                } else {
                    Ok(groups)
                }
            } else {
                Err("#N/A REGEXEXTRACT: No match found.".into())
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regexextract_basic_first_match() {
        // Extract first sequence of digits
        let result = codcel_regexextract("test123abc", r"\d+", None, None).unwrap();
        assert_eq!(result, vec!["123"]);
    }

    #[test]
    fn test_regexextract_no_match() {
        let result = codcel_regexextract("hello", r"\d+", None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_regexextract_all_matches() {
        // Return all digit sequences
        let result = codcel_regexextract("a1b2c3", r"\d+", Some(1), None).unwrap();
        assert_eq!(result, vec!["1", "2", "3"]);
    }

    #[test]
    fn test_regexextract_all_matches_no_match() {
        let result = codcel_regexextract("abc", r"\d+", Some(1), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_regexextract_capture_groups() {
        // Extract date components
        let result =
            codcel_regexextract("2024-01-15", r"(\d{4})-(\d{2})-(\d{2})", Some(2), None).unwrap();
        assert_eq!(result, vec!["2024", "01", "15"]);
    }

    #[test]
    fn test_regexextract_capture_groups_no_groups_in_pattern() {
        // If no capture groups, return full match
        let result = codcel_regexextract("test123", r"\d+", Some(2), None).unwrap();
        assert_eq!(result, vec!["123"]);
    }

    #[test]
    fn test_regexextract_case_sensitive_default() {
        let result = codcel_regexextract("Hello World", "hello", None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_regexextract_case_insensitive() {
        let result = codcel_regexextract("Hello World", "hello", Some(0), Some(1)).unwrap();
        assert_eq!(result, vec!["Hello"]);
    }

    #[test]
    fn test_regexextract_case_insensitive_all_matches() {
        let result = codcel_regexextract("Hello hello HELLO", "hello", Some(1), Some(1)).unwrap();
        assert_eq!(result, vec!["Hello", "hello", "HELLO"]);
    }

    #[test]
    fn test_regexextract_word_pattern() {
        let result =
            codcel_regexextract("The quick brown fox", r"\b\w{5}\b", Some(1), None).unwrap();
        assert_eq!(result, vec!["quick", "brown"]);
    }

    #[test]
    fn test_regexextract_email() {
        let result = codcel_regexextract(
            "Contact us at info@example.com or support@test.org",
            r"[\w.+-]+@[\w-]+\.[\w.]+",
            Some(1),
            None,
        )
        .unwrap();
        assert_eq!(result, vec!["info@example.com", "support@test.org"]);
    }

    #[test]
    fn test_regexextract_invalid_regex() {
        let result = codcel_regexextract("test", r"[invalid", None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_regexextract_invalid_return_mode() {
        let result = codcel_regexextract("test", r"test", Some(3), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_regexextract_empty_text() {
        let result = codcel_regexextract("", r"\d+", None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_regexextract_empty_pattern_matches_empty() {
        let result = codcel_regexextract("hello", "", None, None).unwrap();
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_regexextract_unicode() {
        let result = codcel_regexextract("中国香港", "国", None, None).unwrap();
        assert_eq!(result, vec!["国"]);
    }

    #[test]
    fn test_regexextract_optional_capture_group() {
        // One capture group matches, the other is optional and doesn't
        let result = codcel_regexextract("abc", r"(a)(z)?", Some(2), None).unwrap();
        assert_eq!(result, vec!["a".to_string(), String::new()]);
    }
}
