// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;
use unicode_segmentation::UnicodeSegmentation;

/// Excel-compatible `TRIM` that removes extra spaces from text.
/// - `text`: the text from which to remove extra spaces.
///   Returns the text with leading and trailing spaces removed, and multiple consecutive
///   spaces between words reduced to single spaces. Handles tabs and newlines as spaces.
///   Uses Unicode grapheme cluster awareness for proper text handling.
pub fn codcel_trim<S: AsRef<str>>(text: S) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref().trim();
    let mut result = String::new();
    let mut in_space = false;

    for c in text.graphemes(true) {
        if c.trim().is_empty() {
            if !in_space {
                result.push(' ');
                in_space = true;
            }
        } else {
            result.push_str(c);
            in_space = false;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_leading_trailing_spaces() {
        // =TRIM("   Hello World   ") in US format
        // =TRIM("   Hello World   ") in German format
        let result = codcel_trim("   Hello World   ").unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_trim_multiple_spaces_between_words() {
        // =TRIM("Hello     World") in US format
        // =TRIM("Hello     World") in German format
        let result = codcel_trim("Hello     World").unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_trim_multiple_spaces_everywhere() {
        // =TRIM("   Hello     World   ") in US format
        // =TRIM("   Hello     World   ") in German format
        let result = codcel_trim("   Hello     World   ").unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_trim_tabs_and_newlines() {
        // =TRIM("Hello\t\tWorld\n") in US format
        // =TRIM("Hello\t\tWorld\n") in German format
        let result = codcel_trim("Hello\t\tWorld\n").unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_trim_no_extra_spaces() {
        // =TRIM("Hello World") in US format
        // =TRIM("Hello World") in German format
        let result = codcel_trim("Hello World").unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_trim_only_spaces() {
        // =TRIM("     ") in US format
        // =TRIM("     ") in German format
        let result = codcel_trim("     ").unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_trim_empty_string() {
        // =TRIM("") in US format
        // =TRIM("") in German format
        let result = codcel_trim("").unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }
}
