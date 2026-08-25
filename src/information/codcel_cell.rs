// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `CELL` function that returns information about a cell.
///
/// In the transpiled context, info_types that depend on cell position ("row", "col", "address")
/// are resolved at transpile time. This runtime function handles value-dependent info_types
/// and provides sensible defaults for metadata-dependent ones.
///
/// # Parameters
/// - `info_type`: A string indicating what type of cell information to return.
/// - `value`: The cell value (used for "type" and "contents" info_types).
/// - `_value_format`: Retained for signature consistency.
///
/// # Supported info_types
/// - `"type"` — Returns `"b"` (blank), `"l"` (label/text), or `"v"` (value/number).
/// - `"contents"` — Returns the cell value itself.
/// - `"row"` — Runtime fallback: returns 0 (normally resolved at transpile time).
/// - `"col"` — Runtime fallback: returns 0 (normally resolved at transpile time).
/// - `"address"` — Runtime fallback: returns `""` (normally resolved at transpile time).
/// - `"filename"` / `"sheet"` — Returns `""` (not available in transpiled context).
/// - `"color"` / `"protect"` / `"parentheses"` — Returns 0.
/// - `"width"` — Returns 8 (default column width).
/// - `"prefix"` — Returns `""`.
/// - `"format"` — Returns `"G"` (General format).
pub fn codcel_cell(
    info_type: &str,
    value: &Value,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match info_type.to_lowercase().as_str() {
        "type" => {
            let type_code = match value {
                Value::None
                | Value::OptionF64(None)
                | Value::OptionI32(None)
                | Value::OptionString(None)
                | Value::OptionBool(None)
                | Value::OptionVecValue(None)
                | Value::OptionAreaValue(None)
                | Value::OptionChronoDateTime(None)
                | Value::OptionTime(None) => "b",

                Value::String(s) if s.is_empty() => "b",
                Value::OptionString(Some(s)) if s.is_empty() => "b",

                Value::String(_) | Value::OptionString(Some(_)) => "l",

                _ => "v",
            };
            Ok(Value::String(type_code.to_string()))
        }
        "contents" => Ok(value.clone()),
        // Runtime fallbacks for position-dependent info_types
        // (these should normally be resolved at transpile time)
        "row" => Ok(Value::I32(0)),
        "col" => Ok(Value::I32(0)),
        "address" => Ok(Value::String(String::new())),
        // Metadata-dependent info_types with sensible defaults
        "filename" => Ok(Value::String(String::new())),
        "sheet" => Ok(Value::String(String::new())),
        "color" => Ok(Value::I32(0)),
        "protect" => Ok(Value::I32(0)),
        "parentheses" => Ok(Value::I32(0)),
        "width" => Ok(Value::I32(8)),
        "prefix" => Ok(Value::String(String::new())),
        "format" => Ok(Value::String("G".to_string())),
        _ => Err(format!("CELL: Unknown info_type \"{}\"", info_type).into()),
    }
}

#[cfg(test)]
mod tests {
    // Literals such as 3.14159 and 1.41421 are Excel-visible values under test,
    // not stand-ins for std::f64::consts.
    #![allow(clippy::approx_constant)]
    use super::*;
    use crate::value_format::ValueFormat;

    fn default_format() -> ValueFormat {
        ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        }
    }

    fn string_val(v: &Value) -> &str {
        match v {
            Value::String(s) => s,
            _ => panic!("expected String, got {:?}", v),
        }
    }

    fn i32_val(v: &Value) -> i32 {
        match v {
            Value::I32(n) => *n,
            _ => panic!("expected I32, got {:?}", v),
        }
    }

    // --- "type" info_type ---

    #[test]
    fn type_blank_none() {
        let result = codcel_cell("type", &Value::None, &default_format()).unwrap();
        assert_eq!(string_val(&result), "b");
    }

    #[test]
    fn type_blank_empty_string() {
        let result = codcel_cell("type", &Value::String(String::new()), &default_format()).unwrap();
        assert_eq!(string_val(&result), "b");
    }

    #[test]
    fn type_label_text() {
        let result = codcel_cell(
            "type",
            &Value::String("hello".to_string()),
            &default_format(),
        )
        .unwrap();
        assert_eq!(string_val(&result), "l");
    }

    #[test]
    fn type_value_number() {
        let result = codcel_cell("type", &Value::F64(42.0), &default_format()).unwrap();
        assert_eq!(string_val(&result), "v");
    }

    #[test]
    fn type_value_integer() {
        let result = codcel_cell("type", &Value::I32(10), &default_format()).unwrap();
        assert_eq!(string_val(&result), "v");
    }

    #[test]
    fn type_value_bool() {
        let result = codcel_cell("type", &Value::Bool(true), &default_format()).unwrap();
        assert_eq!(string_val(&result), "v");
    }

    #[test]
    fn type_blank_option_none() {
        let result = codcel_cell("type", &Value::OptionF64(None), &default_format()).unwrap();
        assert_eq!(string_val(&result), "b");
    }

    #[test]
    fn type_value_option_some() {
        let result = codcel_cell("type", &Value::OptionF64(Some(3.14)), &default_format()).unwrap();
        assert_eq!(string_val(&result), "v");
    }

    // --- "contents" info_type ---

    #[test]
    fn contents_number() {
        let result = codcel_cell("contents", &Value::F64(42.5), &default_format()).unwrap();
        match result {
            Value::F64(v) => assert_eq!(v, 42.5),
            _ => panic!("expected F64"),
        }
    }

    #[test]
    fn contents_string() {
        let result = codcel_cell(
            "contents",
            &Value::String("hello".to_string()),
            &default_format(),
        )
        .unwrap();
        assert_eq!(string_val(&result), "hello");
    }

    #[test]
    fn contents_none() {
        let result = codcel_cell("contents", &Value::None, &default_format()).unwrap();
        assert!(matches!(result, Value::None));
    }

    // --- Runtime fallbacks for position info ---

    #[test]
    fn row_runtime_fallback() {
        let result = codcel_cell("row", &Value::None, &default_format()).unwrap();
        assert_eq!(i32_val(&result), 0);
    }

    #[test]
    fn col_runtime_fallback() {
        let result = codcel_cell("col", &Value::None, &default_format()).unwrap();
        assert_eq!(i32_val(&result), 0);
    }

    #[test]
    fn address_runtime_fallback() {
        let result = codcel_cell("address", &Value::None, &default_format()).unwrap();
        assert_eq!(string_val(&result), "");
    }

    // --- Metadata defaults ---

    #[test]
    fn filename_default() {
        let result = codcel_cell("filename", &Value::None, &default_format()).unwrap();
        assert_eq!(string_val(&result), "");
    }

    #[test]
    fn format_default() {
        let result = codcel_cell("format", &Value::None, &default_format()).unwrap();
        assert_eq!(string_val(&result), "G");
    }

    #[test]
    fn width_default() {
        let result = codcel_cell("width", &Value::None, &default_format()).unwrap();
        assert_eq!(i32_val(&result), 8);
    }

    #[test]
    fn color_default() {
        let result = codcel_cell("color", &Value::None, &default_format()).unwrap();
        assert_eq!(i32_val(&result), 0);
    }

    #[test]
    fn protect_default() {
        let result = codcel_cell("protect", &Value::None, &default_format()).unwrap();
        assert_eq!(i32_val(&result), 0);
    }

    // --- Case insensitivity ---

    #[test]
    fn type_case_insensitive() {
        let result = codcel_cell("TYPE", &Value::F64(1.0), &default_format()).unwrap();
        assert_eq!(string_val(&result), "v");
    }

    #[test]
    fn contents_case_insensitive() {
        let result = codcel_cell("CONTENTS", &Value::I32(5), &default_format()).unwrap();
        assert_eq!(i32_val(&result), 5);
    }

    // --- Error case ---

    #[test]
    fn unknown_info_type() {
        let result = codcel_cell("unknown", &Value::None, &default_format());
        assert!(result.is_err());
    }
}
