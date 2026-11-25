// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;
use unicode_segmentation::UnicodeSegmentation;

/// Excel-compatible `PROPER` that capitalizes the first letter of each word in a text string.
/// - `text`: the text string to convert to proper case (title case).
///   Returns the text with the first letter of each word capitalized and
///   all other letters in lowercase. Words are delimited by any character that is not a
///   letter (spaces, punctuation and digits). Handles Unicode grapheme clusters correctly.
pub fn codcel_proper<S: AsRef<str>>(text: S) -> Result<String, Box<dyn Error + Send + Sync>> {
    let text = text.as_ref();
    let mut result = String::with_capacity(text.len());
    let mut capitalize_next = true;

    // Handle empty string case
    if text.is_empty() {
        return Ok(String::new());
    }

    // Process text using grapheme clusters instead of chars to handle complex Unicode correctly
    for grapheme in text.graphemes(true) {
        let mut chars = grapheme.chars();
        // A grapheme cluster always contains at least one char.
        let Some(first_char) = chars.next() else {
            continue;
        };

        // As in Excel, a letter after anything that is not a letter (a space, punctuation or a
        // digit: "123main" gives "123Main") starts a word.
        if first_char.is_alphabetic() {
            if capitalize_next {
                result.extend(first_char.to_uppercase());
            } else {
                result.extend(first_char.to_lowercase());
            }
            result.extend(chars.flat_map(|c| c.to_lowercase()));
            capitalize_next = false;
        } else {
            result.push(first_char);
            result.extend(chars);
            capitalize_next = true;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_basic() {
        // =PROPER("this is a test") in US format
        // =PROPER("this is a test") in German format
        let result = codcel_proper("this is a test").unwrap();
        println!("{result}");
        assert_eq!(result, "This Is A Test");
    }

    #[test]
    fn test_proper_mixed_case() {
        // =PROPER("tHiS iS a TeSt") in US format
        // =PROPER("tHiS iS a TeSt") in German format
        let result = codcel_proper("tHiS iS a TeSt").unwrap();
        println!("{result}");
        assert_eq!(result, "This Is A Test");
    }

    #[test]
    fn test_proper_with_numbers() {
        // =PROPER("1st thing 2nd thing") in US format
        // =PROPER("1st thing 2nd thing") in German format
        let result = codcel_proper("1st thing 2nd thing").unwrap();
        println!("{result}");
        // Excel starts a word at a letter after a digit too.
        assert_eq!(result, "1St Thing 2Nd Thing");
        assert_eq!(codcel_proper("123main street").unwrap(), "123Main Street");
    }

    #[test]
    fn test_proper_with_punctuation() {
        // =PROPER("hello, world! how are you?") in US format
        // =PROPER("hello, world! how are you?") in German format
        let result = codcel_proper("hello, world! how are you?").unwrap();
        println!("{result}");
        assert_eq!(result, "Hello, World! How Are You?");
    }

    #[test]
    fn test_proper_empty_string() {
        // =PROPER("") in US format
        // =PROPER("") in German format
        let result = codcel_proper("").unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_proper_with_special_chars() {
        // =PROPER("a-b-c/d") in US format
        // =PROPER("a-b-c/d") in German format
        let result = codcel_proper("a-b-c/d").unwrap();
        println!("{result}");
        assert_eq!(result, "A-B-C/D");
    }
}
