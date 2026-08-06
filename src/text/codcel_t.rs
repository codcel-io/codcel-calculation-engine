// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Excel-compatible `T` that returns the text referred to by a value.
/// - `value`: the value to test and potentially convert.
///   Returns the text if the value is a text string; otherwise returns an empty string.
///   Useful for ensuring a value is treated as text and for compatibility with other
///   spreadsheet applications.
pub fn codcel_t(value: Value) -> Result<String, Box<dyn Error + Send + Sync>> {
    match value {
        Value::String(value) => Ok(value),
        Value::OptionString(Some(val)) => Ok(val),
        _ => Ok("".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::{bool, f64, i32, some_string, string};

    #[test]
    fn test_t_with_string() {
        // =T("Hello") in US format
        // =T("Hello") in German format
        let result = codcel_t(string("Hello".to_string())).unwrap();
        println!("{result}");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_t_with_option_string() {
        // =T("World") in US format
        // =T("World") in German format
        let result = codcel_t(some_string("World".to_string())).unwrap();
        println!("{result}");
        assert_eq!(result, "World");
    }

    #[test]
    fn test_t_with_number() {
        // =T(123) in US format
        // =T(123) in German format
        let result = codcel_t(i32(123)).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_t_with_decimal() {
        // =T(123.45) in US format
        // =T(123,45) in German format
        let result = codcel_t(f64(123.45)).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_t_with_boolean() {
        // =T(TRUE) in US format
        // =T(TRUE) in German format
        let result = codcel_t(bool(true)).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }

    #[test]
    fn test_t_with_empty_string() {
        // =T("") in US format
        // =T("") in German format
        let result = codcel_t(string("".to_string())).unwrap();
        println!("{result}");
        assert_eq!(result, "");
    }
}
