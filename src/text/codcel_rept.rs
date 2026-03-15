// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `REPT` that repeats text a specified number of times.
/// - `text`: the text string to repeat.
/// - `number_times`: the number of times to repeat the text.
///   Returns the text repeated the specified number of times.
///   If `number_times` is 0 or negative, returns an empty string.
///   Returns an error if the resulting string would be too large (overflow protection).
pub fn codcel_rept<S: AsRef<str>>(
    text: S,
    number_times: i32,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    if number_times <= 0 {
        return Ok(String::new());
    }

    let text = text.as_ref();

    // Check for potential overflow
    if let Some(total_length) = text.len().checked_mul(number_times as usize) {
        let mut result = String::with_capacity(total_length);
        for _ in 0..number_times {
            result.push_str(text);
        }
        Ok(result)
    } else {
        Err("REPT: The resulting string is too large.".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rept_basic() {
        // =REPT("abc", 3) in US format
        // =REPT("abc"; 3) in German format
        let result = codcel_rept("abc", 3).unwrap();
        println!("{result}");
        assert_eq!(result, "abcabcabc");
    }

    #[test]
    fn test_rept_single_char() {
        // =REPT("x", 5) in US format
        // =REPT("x"; 5) in German format
        let result = codcel_rept("x", 5).unwrap();
        println!("{result}");
        assert_eq!(result, "xxxxx");
    }

    #[test]
    fn test_rept_zero_times() {
        // =REPT("abc", 0) in US format
        // =REPT("abc"; 0) in German format
        let result = codcel_rept("abc", 0).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_rept_negative_times() {
        // =REPT("abc", -3) in US format
        // =REPT("abc"; -3) in German format
        let result = codcel_rept("abc", -3).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_rept_empty_string() {
        // =REPT("", 5) in US format
        // =REPT(""; 5) in German format
        let result = codcel_rept("", 5).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_rept_with_spaces() {
        // =REPT(" ", 3) in US format
        // =REPT(" "; 3) in German format
        let result = codcel_rept(" ", 3).unwrap();
        println!("{result}");
        assert_eq!(result, "   ");
    }

    #[test]
    fn test_rept_with_special_chars() {
        // =REPT("*-*", 2) in US format
        // =REPT("*-*"; 2) in German format
        let result = codcel_rept("*-*", 2).unwrap();
        println!("{result}");
        assert_eq!(result, "*-**-*");
    }
}
