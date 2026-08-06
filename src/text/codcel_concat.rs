// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::text::codcel_concatenate::codcel_concatenate;
use std::error::Error;

/// Excel-compatible `CONCAT` that joins multiple text strings into one string.
/// - `values`: a vector of text strings to concatenate.
///   Returns all the text strings joined together without any delimiter.
///   This is the modern replacement for CONCATENATE with improved range support.
pub fn codcel_concat<S: AsRef<str>>(
    values: Vec<S>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    codcel_concatenate(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_strings() {
        // =CONCAT("Hello", " ", "World") in US format
        // =CONCAT("Hello"; " "; "World") in German format
        let values = vec!["Hello", " ", "World"];
        let result = codcel_concat(values).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_concat_numbers_as_strings() {
        // =CONCAT("Value: ", "123") in US format
        // =CONCAT("Value: "; "123") in German format
        let values = vec!["Value: ", "123"];
        let result = codcel_concat(values).unwrap();
        println!("{result}");
        assert_eq!(result, "Value: 123");
    }

    #[test]
    fn test_concat_empty_strings() {
        // =CONCAT("", "", "") in US format
        // =CONCAT(""; ""; "") in German format
        let values = vec!["", "", ""];
        let result = codcel_concat(values).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_concat_single_string() {
        // =CONCAT("Single") in US format
        // =CONCAT("Single") in German format
        let values = vec!["Single"];
        let result = codcel_concat(values).unwrap();
        println!("{result}");
        assert_eq!(result, "Single");
    }

    #[test]
    fn test_concat_special_characters() {
        // =CONCAT("Special", "!", "@", "#") in US format
        // =CONCAT("Special"; "!"; "@"; "#") in German format
        let values = vec!["Special", "!", "@", "#"];
        let result = codcel_concat(values).unwrap();
        println!("{result}");
        assert_eq!(result, "Special!@#");
    }
}
