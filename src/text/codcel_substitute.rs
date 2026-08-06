// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `SUBSTITUTE` that substitutes new text for old text in a text string.
/// - `text`: the text in which to substitute characters.
/// - `old_text`: the text to replace.
/// - `new_text`: the text to replace `old_text` with.
/// - `instance_num`: optional instance number to replace. If omitted, all instances are replaced.
///   Returns the text with the specified substitutions made.
///   Returns an error if `old_text` is empty. The search is case-sensitive.
pub fn codcel_substitute<S: AsRef<str>>(
    text: S,
    old_text: S,
    new_text: S,
    instance_num: Option<i32>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref();
    let old_text = old_text.as_ref();
    let new_text = new_text.as_ref();

    if old_text.is_empty() {
        return Err("SUBSTITUTE: The `old_text` argument cannot be empty.".into());
    }

    if let Some(instance) = instance_num {
        let mut result = String::new();
        let mut count = 0;
        let mut start = 0;

        while let Some(pos) = text[start..].find(old_text) {
            count += 1;
            let current_pos = start + pos;
            result.push_str(&text[start..current_pos]);

            if count == instance {
                result.push_str(new_text);
                start = current_pos + old_text.len();
                result.push_str(&text[start..]);
                return Ok(result);
            }

            result.push_str(old_text);
            start = current_pos + old_text.len();
        }

        // If the specified instance is not found, return the original text
        Ok(text.to_string())
    } else {
        // Replace all occurrences of old_text with new_text
        let result = text.replace(old_text, new_text);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_substitute_all_occurrences() {
        // =SUBSTITUTE("Hello Hello World", "Hello", "Hi") in US format
        // =SUBSTITUTE("Hello Hello World"; "Hello"; "Hi") in German format
        let result = codcel_substitute("Hello Hello World", "Hello", "Hi", None).unwrap();
        println!("{result}");
        assert_eq!(result, "Hi Hi World");
    }

    #[test]
    fn test_substitute_specific_occurrence() {
        // =SUBSTITUTE("Hello Hello World", "Hello", "Hi", 2) in US format
        // =SUBSTITUTE("Hello Hello World"; "Hello"; "Hi"; 2) in German format
        let result = codcel_substitute("Hello Hello World", "Hello", "Hi", Some(2)).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello Hi World");
    }

    #[test]
    fn test_substitute_occurrence_not_found() {
        // =SUBSTITUTE("Hello World", "Hello", "Hi", 2) in US format
        // =SUBSTITUTE("Hello World"; "Hello"; "Hi"; 2) in German format
        let result = codcel_substitute("Hello World", "Hello", "Hi", Some(2)).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World"); // Original text returned when occurrence not found
    }

    #[test]
    fn test_substitute_with_empty_new_text() {
        // =SUBSTITUTE("Hello World", "Hello", "") in US format
        // =SUBSTITUTE("Hello World"; "Hello"; "") in German format
        let result = codcel_substitute("Hello World", "Hello", "", None).unwrap();
        println!("{result}");
        assert_eq!(result, " World"); // "Hello" is removed
    }

    #[test]
    fn test_substitute_with_text_not_found() {
        // =SUBSTITUTE("Hello World", "Goodbye", "Hi") in US format
        // =SUBSTITUTE("Hello World"; "Goodbye"; "Hi") in German format
        let result = codcel_substitute("Hello World", "Goodbye", "Hi", None).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World"); // Original text returned when old_text not found
    }

    #[test]
    fn test_substitute_case_sensitive() {
        // =SUBSTITUTE("Hello hello World", "hello", "Hi") in US format
        // =SUBSTITUTE("Hello hello World"; "hello"; "Hi") in German format
        let result = codcel_substitute("Hello hello World", "hello", "Hi", None).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello Hi World"); // Only lowercase "hello" is replaced
    }

    #[test]
    fn test_substitute_empty_old_text() {
        // =SUBSTITUTE("Hello World", "", "Hi") in US format - this should error
        // =SUBSTITUTE("Hello World"; ""; "Hi") in German format - this should error
        let result = codcel_substitute("Hello World", "", "Hi", None);
        assert!(result.is_err());
    }
}
