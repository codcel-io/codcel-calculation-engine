// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `CLEAN` that removes all non-printable control characters from text.
/// - `text`: the string to clean.
///   Returns the text with control characters (codes 0-31, except tab) removed.
///   Tab characters (code 9) are preserved. Useful for cleaning text imported from
///   other applications that may contain non-printable characters.
pub fn codcel_clean<S: AsRef<str>>(text: S) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text_ref = text.as_ref();
    let cleaned_text: String = text_ref
        .chars()
        .filter(|&c| c >= ' ' || c == '\t')
        .collect();
    Ok(cleaned_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_with_control_characters() {
        // =CLEAN("Test" & CHAR(7)) in US format
        // =CLEAN("Test" & CHAR(7)) in German format
        let text = format!("Test{}", char::from_u32(7).unwrap());
        let result = codcel_clean(text).unwrap();
        println!("{result}");
        assert_eq!(result, "Test");
    }

    #[test]
    fn test_clean_with_multiple_control_characters() {
        // =CLEAN(CHAR(9) & "Test" & CHAR(10) & "Data" & CHAR(13)) in US format
        // =CLEAN(CHAR(9) & "Test" & CHAR(10) & "Data" & CHAR(13)) in German format
        let text = format!(
            "{}Test{}Data{}",
            char::from_u32(9).unwrap(),  // Tab (preserved)
            char::from_u32(10).unwrap(), // Line feed (removed)
            char::from_u32(13).unwrap()  // Carriage return (removed)
        );
        let result = codcel_clean(text).unwrap();
        println!("{result}");
        assert_eq!(result, "\tTestData");
    }

    #[test]
    fn test_clean_with_no_control_characters() {
        // =CLEAN("Regular text") in US format
        // =CLEAN("Regular text") in German format
        let result = codcel_clean("Regular text").unwrap();
        println!("{result}");
        assert_eq!(result, "Regular text");
    }

    #[test]
    fn test_clean_with_only_control_characters() {
        // =CLEAN(CHAR(1) & CHAR(2) & CHAR(3)) in US format
        // =CLEAN(CHAR(1) & CHAR(2) & CHAR(3)) in German format
        let text = format!(
            "{}{}{}",
            char::from_u32(1).unwrap(),
            char::from_u32(2).unwrap(),
            char::from_u32(3).unwrap()
        );
        let result = codcel_clean(text).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_clean_with_tab_character() {
        // =CLEAN("Column1" & CHAR(9) & "Column2") in US format
        // =CLEAN("Column1" & CHAR(9) & "Column2") in German format
        let text = format!("Column1{}Column2", char::from_u32(9).unwrap());
        let result = codcel_clean(text).unwrap();
        println!("{result}");
        assert_eq!(result, "Column1\tColumn2");
    }
}
