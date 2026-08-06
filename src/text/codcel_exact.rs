// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `EXACT` that compares two text strings for exact equality.
/// - `text1`: the first text string to compare.
/// - `text2`: the second text string to compare.
///   Returns `true` if the strings are exactly the same (case-sensitive), `false` otherwise.
pub fn codcel_exact<S: AsRef<str>>(
    text1: S,
    text2: S,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(text1.as_ref() == text2.as_ref())
}

/// Vector variant of `EXACT` that accepts a vector of exactly 2 text strings.
/// - `inputs`: a vector containing exactly 2 text strings to compare.
///   Returns `true` if the strings are exactly the same, or an error if not exactly 2 inputs.
pub fn codcel_exact_vec<S: AsRef<str>>(
    inputs: Vec<S>,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 2 {
        return Err("EXACT: Must contain exactly 2 parameters.".into());
    }
    codcel_exact(inputs[0].as_ref(), inputs[1].as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_identical_strings() {
        // =EXACT("Hello", "Hello") in US format
        // =EXACT("Hello"; "Hello") in German format
        let result = codcel_exact("Hello", "Hello").unwrap();
        println!("{result}");
        assert!(result);
    }

    #[test]
    fn test_exact_different_strings() {
        // =EXACT("Hello", "World") in US format
        // =EXACT("Hello"; "World") in German format
        let result = codcel_exact("Hello", "World").unwrap();
        println!("{result}");
        assert!(!result);
    }

    #[test]
    fn test_exact_case_sensitive() {
        // =EXACT("Hello", "hello") in US format
        // =EXACT("Hello"; "hello") in German format
        let result = codcel_exact("Hello", "hello").unwrap();
        println!("{result}");
        assert!(!result);
    }

    #[test]
    fn test_exact_with_spaces() {
        // =EXACT("Hello World", "Hello World") in US format
        // =EXACT("Hello World"; "Hello World") in German format
        let result = codcel_exact("Hello World", "Hello World").unwrap();
        println!("{result}");
        assert!(result);
    }

    #[test]
    fn test_exact_with_numbers() {
        // =EXACT("123", "123") in US format
        // =EXACT("123"; "123") in German format
        let result = codcel_exact("123", "123").unwrap();
        println!("{result}");
        assert!(result);
    }

    #[test]
    fn test_exact_vec_valid() {
        // =EXACT("Hello", "Hello") in US format
        // =EXACT("Hello"; "Hello") in German format
        let inputs = vec!["Hello", "Hello"];
        let result = codcel_exact_vec(inputs).unwrap();
        println!("{result}");
        assert!(result);
    }

    #[test]
    fn test_exact_vec_invalid_params() {
        // This test checks that the function returns an error when given too few or too many parameters
        let inputs = vec!["Hello"];
        let result = codcel_exact_vec(inputs);
        assert!(result.is_err());

        let inputs = vec!["Hello", "World", "Extra"];
        let result = codcel_exact_vec(inputs);
        assert!(result.is_err());
    }
}
