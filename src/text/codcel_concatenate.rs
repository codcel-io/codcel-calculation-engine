// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `CONCATENATE` that joins multiple text strings into one string.
/// - `values`: a vector of text strings to concatenate.
///   Returns all the text strings joined together without any delimiter.
///   Note: CONCATENATE is maintained for backward compatibility; CONCAT is preferred.
pub fn codcel_concatenate<S: AsRef<str>>(
    values: Vec<S>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let concatenated_string = values.iter().map(|s| s.as_ref()).collect::<String>();
    Ok(concatenated_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concatenate_strings() {
        // =CONCATENATE("Hello", " ", "World") in US format
        // =CONCATENATE("Hello"; " "; "World") in German format
        let values = vec!["Hello", " ", "World"];
        let result = codcel_concatenate(values).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_concatenate_numbers_as_strings() {
        // =CONCATENATE("Value: ", "123") in US format
        // =CONCATENATE("Value: "; "123") in German format
        let values = vec!["Value: ", "123"];
        let result = codcel_concatenate(values).unwrap();
        println!("{result}");
        assert_eq!(result, "Value: 123");
    }

    #[test]
    fn test_concatenate_empty_strings() {
        // =CONCATENATE("", "", "") in US format
        // =CONCATENATE(""; ""; "") in German format
        let values = vec!["", "", ""];
        let result = codcel_concatenate(values).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_concatenate_single_string() {
        // =CONCATENATE("Single") in US format
        // =CONCATENATE("Single") in German format
        let values = vec!["Single"];
        let result = codcel_concatenate(values).unwrap();
        println!("{result}");
        assert_eq!(result, "Single");
    }

    #[test]
    fn test_concatenate_special_characters() {
        // =CONCATENATE("Special", "!", "@", "#") in US format
        // =CONCATENATE("Special"; "!"; "@"; "#") in German format
        let values = vec!["Special", "!", "@", "#"];
        let result = codcel_concatenate(values).unwrap();
        println!("{result}");
        assert_eq!(result, "Special!@#");
    }
}
