// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use regex::Regex;
use std::error::Error;

/// Builds a regex pattern from an Excel wildcard string.
/// - `?` matches any single character (becomes `.` in regex)
/// - `*` matches any sequence of characters (becomes `.*` in regex)
/// - `~?` is a literal `?`, `~*` is a literal `*`, `~~` is a literal `~`
/// - All other regex-special characters are escaped.
pub(crate) fn build_wildcard_pattern(substring: &str) -> String {
    let mut pattern = String::new();
    let chars: Vec<char> = substring.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '~' if i + 1 < chars.len()
                && (chars[i + 1] == '?' || chars[i + 1] == '*' || chars[i + 1] == '~') =>
            {
                // Escaped wildcard or tilde: treat next char as literal
                pattern.push_str(&regex::escape(&chars[i + 1].to_string()));
                i += 2;
            }
            '?' => {
                pattern.push('.'); // `?` -> `.` in regex
                i += 1;
            }
            '*' => {
                pattern.push_str(".*"); // `*` -> `.*` in regex
                i += 1;
            }
            ch => {
                let s = ch.to_string();
                if regex::escape(&s) != s {
                    pattern.push_str(&regex::escape(&s));
                } else {
                    pattern.push(ch);
                }
                i += 1;
            }
        }
    }
    pattern
}

/// Excel-compatible `SEARCH` that locates one text string within another (case-insensitive).
/// - `substring`: the text to find. Supports wildcards: `?` matches any single character,
///   `*` matches any sequence of characters. Use `~` to escape wildcards: `~?` for literal `?`,
///   `~*` for literal `*`, `~~` for literal `~`.
/// - `text`: the text in which to search.
/// - `start_position`: optional position to start searching (default 1, 1-based index).
///   Returns the position of the first occurrence of `substring` within `text` (1-based).
///   Returns an error if `substring` is not found or if `start_position` exceeds the text length.
///   Unlike FIND, SEARCH is case-insensitive and supports wildcards.
pub fn codcel_search(
    substring: &str,
    text: &str,
    start_position: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let start_position = start_position.unwrap_or(1);
    let start_position = start_position as usize;
    let char_count = text.chars().count();

    // Excel: empty substring returns start_position (SEARCH("","") = 1, SEARCH("","abc",2) = 2)
    if substring.is_empty() {
        if start_position > char_count + 1 {
            return Err("Error: SEARCH start position exceeds the text length.".into());
        }
        return Ok(start_position as i32);
    }

    if start_position > char_count {
        return Err("Error: SEARCH start position exceeds the text length.".into());
    }

    let pattern = build_wildcard_pattern(substring);

    // Create regex with case-insensitivity
    let regex_str = format!("(?i){pattern}");
    let regex = Regex::new(&regex_str)?;

    // Adjust the text to start from the specified character position
    let byte_offset = text
        .char_indices()
        .nth(start_position - 1)
        .map_or(text.len(), |(i, _)| i);
    let adjusted_text = &text[byte_offset..];

    // Find the first match and return its character position in the original text
    if let Some(matched) = regex.find(adjusted_text) {
        // Convert byte offset to character offset
        let char_pos = adjusted_text[..matched.start()].chars().count();
        Ok((char_pos + start_position) as i32) // 1-based character index
    } else {
        Err("#VALUE! Substring not found.".into()) // Match not found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_basic() {
        // =SEARCH("e", "abcdefghijk") in US format
        // =SEARCH("e"; "abcdefghijk") in German format
        let result = codcel_search("e", "abcdefghijk", None).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_search_with_start_position() {
        // =SEARCH("e", "abcdefghijek", 6) in US format
        // =SEARCH("e"; "abcdefghijek"; 6) in German format
        let result = codcel_search("e", "abcdefghijek", Some(6)).unwrap();
        println!("{result}");
        assert_eq!(result, 11);
    }

    #[test]
    fn test_search_case_insensitive() {
        // =SEARCH("E", "abcdefghijk") in US format
        // =SEARCH("E"; "abcdefghijk") in German format
        let result = codcel_search("E", "abcdefghijk", None).unwrap();
        println!("{result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_search_with_wildcard_question_mark() {
        // =SEARCH("a?c", "abcdefgabc") in US format
        // =SEARCH("a?c"; "abcdefgabc") in German format
        let result = codcel_search("a?c", "abcdefgabc", None).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_search_with_wildcard_asterisk() {
        // =SEARCH("a*c", "abcdefgabc") in US format
        // =SEARCH("a*c"; "abcdefgabc") in German format
        let result = codcel_search("a*c", "abcdefgabc", None).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_search_not_found() {
        // =SEARCH("z", "abcdefghijk") in US format
        // =SEARCH("z"; "abcdefghijk") in German format
        let result = codcel_search("z", "abcdefghijk", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_search_start_position_exceeds_length() {
        // This should return an error
        let result = codcel_search("e", "abcdefghijk", Some(15));
        assert!(result.is_err());
    }

    #[test]
    fn test_search_empty_substring() {
        // =SEARCH("", "abcdefghijk") in US format
        // =SEARCH(""; "abcdefghijk") in German format
        let result = codcel_search("", "abcdefghijk", None).unwrap();
        println!("{result}");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_search_special_characters() {
        // =SEARCH(".", "abc.def") in US format
        // =SEARCH("."; "abc.def") in German format
        let result = codcel_search(".", "abc.def", None).unwrap();
        println!("{result}");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_search_unicode() {
        // SEARCH("国", "中国香港") should return 2 (2nd character)
        let result = codcel_search("国", "中国香港", None).unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_search_mixed_ascii_unicode() {
        // SEARCH("中", "Hello中国") should return 6 (6th character)
        let result = codcel_search("中", "Hello中国", None).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn test_search_tilde_escape_question_mark() {
        // ~? means literal question mark
        let result = codcel_search("~?", "abc?def", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_search_tilde_escape_asterisk() {
        // ~* means literal asterisk
        let result = codcel_search("~*", "abc*def", None).unwrap();
        assert_eq!(result, 4);
    }

    #[test]
    fn test_search_tilde_escape_tilde() {
        // ~~ means literal tilde
        let result = codcel_search("~~", "abc~def", None).unwrap();
        assert_eq!(result, 4);
    }
}
