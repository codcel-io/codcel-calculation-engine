// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `TEXTAFTER` that returns the text that occurs after a given delimiter.
/// - `text`: the text to search in.
/// - `delimiter`: the text to search for.
/// - `instance_number`: optional instance of the delimiter to find (default 1). Must be > 0.
/// - `match_mode`: optional case sensitivity flag (default `true` = case-sensitive).
/// - `not_found`: optional text to return if the delimiter is not found (default empty string).
///   Returns the text after the specified instance of the delimiter.
///   Returns an error if `instance_number` <= 0 or if `delimiter` is empty.
pub fn codcel_text_after<S: AsRef<str>>(
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
        return Err("TEXTAFTER: Instance number must be greater than 0.".into());
    }
    let match_mode = match_mode.unwrap_or(true);
    let not_found = match not_found {
        None => "".to_string(),
        Some(s) => s.as_ref().to_string(),
    };

    // Handle empty delimiter case
    if delimiter.is_empty() {
        return Err("TEXTAFTER: Delimiter cannot be empty.".into());
    }

    // Apply case-insensitive search if match_mode is set to false
    let (text, delimiter) = if match_mode {
        (text.to_string(), delimiter.to_string())
    } else {
        (text.to_lowercase(), delimiter.to_lowercase())
    };

    // Find instances of the delimiter
    let delimiter_indices = text.match_indices(&delimiter);
    let mut current_instance = 0;

    for (index, _) in delimiter_indices {
        current_instance += 1;
        if current_instance == instance_number {
            // Return the text after the delimiter
            return Ok(text[(index + delimiter.len())..].to_string());
        }
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
    fn test_text_after_basic() {
        // =TEXTAFTER("Hello World", " ") in US format
        // =TEXTAFTER("Hello World"; " ") in German format
        let result = codcel_text_after("Hello World", " ", None, None, None).unwrap();
        println!("{result}");
        assert_eq!(result, "World");
    }

    #[test]
    fn test_text_after_multiple_delimiters() {
        // =TEXTAFTER("apple,banana,cherry", ",", 2) in US format
        // =TEXTAFTER("apple;banana;cherry"; ";"; 2) in German format
        let result = codcel_text_after("apple,banana,cherry", ",", Some(2), None, None).unwrap();
        println!("{result}");
        assert_eq!(result, "cherry");
    }

    #[test]
    fn test_text_after_case_sensitive() {
        // =TEXTAFTER("Hello World hello world", "hello", 1, TRUE) in US format
        // =TEXTAFTER("Hello World hello world"; "hello"; 1; TRUE) in German format
        let result = codcel_text_after(
            "Hello World hello world",
            "hello",
            Some(1),
            Some(true),
            None,
        )
        .unwrap();
        println!("{result}");
        assert_eq!(result, " world");
    }

    #[test]
    fn test_text_after_case_insensitive() {
        // =TEXTAFTER("Hello World hello world", "hello", 1, FALSE) in US format
        // =TEXTAFTER("Hello World hello world"; "hello"; 1; FALSE) in German format
        let result = codcel_text_after(
            "Hello World hello world",
            "hello",
            Some(1),
            Some(false),
            None,
        )
        .unwrap();
        println!("{result}");
        assert_eq!(result, " world hello world");
    }

    #[test]
    fn test_text_after_not_found_default() {
        // =TEXTAFTER("Hello World", "xyz") in US format
        // =TEXTAFTER("Hello World"; "xyz") in German format
        let result = codcel_text_after("Hello World", "xyz", None, None, None).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_text_after_not_found_custom() {
        // =TEXTAFTER("Hello World", "xyz", 1, TRUE, "Not found") in US format
        // =TEXTAFTER("Hello World"; "xyz"; 1; TRUE; "Not found") in German format
        let result =
            codcel_text_after("Hello World", "xyz", Some(1), Some(true), Some("Not found"))
                .unwrap();
        println!("{result}");
        assert_eq!(result, "Not found");
    }

    #[test]
    fn test_text_after_instance_out_of_range() {
        // =TEXTAFTER("apple,banana,cherry", ",", 5, TRUE, "Not found") in US format
        // =TEXTAFTER("apple;banana;cherry"; ";"; 5; TRUE; "Not found") in German format
        let result = codcel_text_after(
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
    fn test_text_after_empty_text() {
        // =TEXTAFTER("", ",") in US format
        // =TEXTAFTER(""; ";") in German format
        let result = codcel_text_after("", ",", None, None, None).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_text_after_invalid_instance_number() {
        // =TEXTAFTER("Hello World", " ", -1) in US format - this should error
        // =TEXTAFTER("Hello World"; " "; -1) in German format - this should error
        let result = codcel_text_after("Hello World", " ", Some(-1), None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_text_after_empty_delimiter() {
        // =TEXTAFTER("Hello World", "") in US format - this should error
        // =TEXTAFTER("Hello World"; "") in German format - this should error
        let result = codcel_text_after("Hello World", "", None, None, None);
        assert!(result.is_err());
    }
}
