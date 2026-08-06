// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `TEXTBEFORE` that returns the text that occurs before a given delimiter.
/// - `text`: the text to search in.
/// - `delimiter`: the text to search for.
/// - `instance_number`: optional instance of the delimiter to find (default 1). Must be > 0.
/// - `match_mode`: optional case sensitivity flag (default `true` = case-sensitive,
///   falls back to case-insensitive if not found with case-sensitive search).
/// - `not_found`: optional text to return if the delimiter is not found (default empty string).
///   Returns the text before the specified instance of the delimiter.
///   Returns an error if `instance_number` <= 0 or if `delimiter` is empty.
pub fn codcel_text_before<S: AsRef<str>>(
    text: S,
    delimiter: S,
    instance_number: Option<i32>,
    match_mode: Option<bool>,
    not_found: Option<S>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref();
    let delimiter = delimiter.as_ref();

    // Validate input and defaults
    let instance_number = instance_number.unwrap_or(1);
    if instance_number <= 0 {
        return Err("TEXTBEFORE: Instance number must be greater than 0.".into());
    }
    let match_mode = match_mode.unwrap_or(true);
    let not_found = match not_found {
        None => "".to_string(),
        Some(s) => s.as_ref().to_string(),
    };

    // Handle empty delimiter case
    if delimiter.is_empty() {
        return Err("TEXTBEFORE: Delimiter cannot be empty.".into());
    }

    // Store the original text for the result
    let original_text = text.to_string();

    // First try a case-sensitive search
    let mut delimiter_indices = text.match_indices(delimiter);
    let mut current_instance = 0;
    let mut found = false;
    let mut result_index = 0;

    for (index, _) in delimiter_indices {
        current_instance += 1;
        if current_instance == instance_number {
            // Found the delimiter with case-sensitive search
            found = true;
            result_index = index;
            break;
        }
    }

    // If not found with case-sensitive search and match_mode is true, try case-insensitive search
    if !found && match_mode {
        let text_lower = text.to_lowercase();
        let delimiter_lower = delimiter.to_lowercase();

        delimiter_indices = text_lower.match_indices(&delimiter_lower);
        current_instance = 0;

        for (index, _) in delimiter_indices {
            current_instance += 1;
            if current_instance == instance_number {
                // Found the delimiter with case-insensitive search
                found = true;
                result_index = index;
                break;
            }
        }
    }

    if found {
        // Return the original text before the delimiter
        return Ok(original_text[..result_index].to_string());
    }

    // If the specified instance of the delimiter wasn't found, return `not_found`
    if current_instance < instance_number {
        return Ok(not_found.to_string());
    }

    Ok(String::new()) // Fallback
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_before_basic() {
        // =TEXTBEFORE("Hello World", " ") in US format
        // =TEXTBEFORE("Hello World"; " ") in German format
        let result = codcel_text_before("Hello World", " ", None, None, None).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_text_before_multiple_delimiters() {
        // =TEXTBEFORE("apple,banana,cherry", ",", 2) in US format
        // =TEXTBEFORE("apple;banana;cherry"; ";"; 2) in German format
        let result = codcel_text_before("apple,banana,cherry", ",", Some(2), None, None).unwrap();
        println!("{result}");
        assert_eq!(result, "apple,banana");
    }

    #[test]
    fn test_text_before_case_sensitive() {
        // =TEXTBEFORE("Hello World hello world", "world", 1, TRUE) in US format
        // =TEXTBEFORE("Hello World hello world"; "world"; 1; TRUE) in German format
        let result = codcel_text_before(
            "Hello World hello world",
            "world",
            Some(1),
            Some(true),
            None,
        )
        .unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World hello ");
    }

    #[test]
    fn test_text_before_case_insensitive() {
        // =TEXTBEFORE("Hello World hello world", "WORLD", 1, TRUE) in US format
        // =TEXTBEFORE("Hello World hello world"; "WORLD"; 1; TRUE) in German format
        let result = codcel_text_before(
            "Hello World hello world",
            "WORLD",
            Some(1),
            Some(true),
            None,
        )
        .unwrap();
        println!("{result}");
        assert_eq!(result, "Hello ");
    }

    #[test]
    fn test_text_before_not_found_default() {
        // =TEXTBEFORE("Hello World", "xyz") in US format
        // =TEXTBEFORE("Hello World"; "xyz") in German format
        let result = codcel_text_before("Hello World", "xyz", None, None, None).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_text_before_not_found_custom() {
        // =TEXTBEFORE("Hello World", "xyz", 1, TRUE, "Not found") in US format
        // =TEXTBEFORE("Hello World"; "xyz"; 1; TRUE; "Not found") in German format
        let result =
            codcel_text_before("Hello World", "xyz", Some(1), Some(true), Some("Not found"))
                .unwrap();
        println!("{result}");
        assert_eq!(result, "Not found");
    }

    #[test]
    fn test_text_before_instance_out_of_range() {
        // =TEXTBEFORE("apple,banana,cherry", ",", 5, TRUE, "Not found") in US format
        // =TEXTBEFORE("apple;banana;cherry"; ";"; 5; TRUE; "Not found") in German format
        let result = codcel_text_before(
            "apple,banana,cherry",
            ",",
            Some(5),
            Some(true),
            Some("Not found"),
        )
        .unwrap();
        println!("{result}");
        assert_eq!(result, "Not found");
    }

    #[test]
    fn test_text_before_empty_text() {
        // =TEXTBEFORE("", ",") in US format
        // =TEXTBEFORE(""; ";") in German format
        let result = codcel_text_before("", ",", None, None, None).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_text_before_invalid_instance_number() {
        // =TEXTBEFORE("Hello World", " ", -1) in US format - this should error
        // =TEXTBEFORE("Hello World"; " "; -1) in German format - this should error
        let result = codcel_text_before("Hello World", " ", Some(-1), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_text_before_empty_delimiter() {
        // =TEXTBEFORE("Hello World", "") in US format - this should error
        // =TEXTBEFORE("Hello World"; "") in German format - this should error
        let result = codcel_text_before("Hello World", "", None, None, None);
        assert!(result.is_err());
    }
}
