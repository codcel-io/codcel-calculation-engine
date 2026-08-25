// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `ISBLANK` that checks whether a value represents an empty/blank cell.
/// - `value`: the cell value to test.
/// - `_value_format`: unused; retained for signature consistency with other functions.
///
/// Excel-compatible `ISBLANK` — returns `true` only for truly empty/blank cells.
///
/// In Excel, `ISBLANK("")` returns `FALSE`: an empty string is a value, not blank.
/// Blank values include: `Value::None`, and all `Option*(None)` variants.
pub fn codcel_is_blank(
    value: &Value,
    _value_format: &ValueFormat,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(matches!(
        value,
        Value::None
            | Value::OptionF64(None)
            | Value::OptionI32(None)
            | Value::OptionString(None)
            | Value::OptionBool(None)
            | Value::OptionChronoDateTime(None)
            | Value::OptionTime(None)
            | Value::OptionVecValue(None)
            | Value::OptionAreaValue(None)
    ))
}

/// Returns `true` if the value is blank OR is an empty string.
/// Used by `COUNTBLANK`, which counts both empty cells and cells containing `""`.
pub fn codcel_is_blank_or_empty_string(
    value: &Value,
    value_format: &ValueFormat,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    if codcel_is_blank(value, value_format)? {
        return Ok(true);
    }
    Ok(match value {
        Value::String(s) if s.is_empty() => true,
        Value::OptionString(Some(s)) if s.is_empty() => true,
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_format::ValueFormat;
    use chrono::{NaiveTime, TimeZone, Utc};

    fn default_format() -> ValueFormat {
        ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        }
    }

    // --- TRUE cases ---

    #[test]
    fn test_is_blank_with_none() {
        assert!(codcel_is_blank(&Value::None, &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_option_f64_none() {
        assert!(codcel_is_blank(&Value::OptionF64(None), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_option_i32_none() {
        assert!(codcel_is_blank(&Value::OptionI32(None), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_option_string_none() {
        assert!(codcel_is_blank(&Value::OptionString(None), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_option_bool_none() {
        assert!(codcel_is_blank(&Value::OptionBool(None), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_option_chrono_date_time_none() {
        assert!(codcel_is_blank(&Value::OptionChronoDateTime(None), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_option_time_none() {
        assert!(codcel_is_blank(&Value::OptionTime(None), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_option_vec_value_none() {
        assert!(codcel_is_blank(&Value::OptionVecValue(None), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_option_area_value_none() {
        assert!(codcel_is_blank(&Value::OptionAreaValue(None), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_empty_string() {
        // In Excel, ISBLANK("") returns FALSE — an empty string is a value, not blank
        assert!(!codcel_is_blank(&Value::String("".to_string()), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_option_string_some_empty() {
        // In Excel, ISBLANK("") returns FALSE
        assert!(!codcel_is_blank(
            &Value::OptionString(Some("".to_string())),
            &default_format()
        )
        .unwrap());
    }

    // --- codcel_is_blank_or_empty_string tests (used by COUNTBLANK) ---

    #[test]
    fn test_is_blank_or_empty_string_with_empty_string() {
        assert!(
            codcel_is_blank_or_empty_string(&Value::String("".to_string()), &default_format())
                .unwrap()
        );
    }

    #[test]
    fn test_is_blank_or_empty_string_with_option_string_some_empty() {
        assert!(codcel_is_blank_or_empty_string(
            &Value::OptionString(Some("".to_string())),
            &default_format()
        )
        .unwrap());
    }

    #[test]
    fn test_is_blank_or_empty_string_with_none() {
        assert!(codcel_is_blank_or_empty_string(&Value::None, &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_or_empty_string_with_non_empty_string() {
        assert!(!codcel_is_blank_or_empty_string(
            &Value::String("test".to_string()),
            &default_format()
        )
        .unwrap());
    }

    // --- FALSE cases ---

    #[test]
    fn test_is_blank_with_number() {
        assert!(!codcel_is_blank(&Value::F64(42.0), &default_format()).unwrap());
        assert!(!codcel_is_blank(&Value::F64(0.0), &default_format()).unwrap());
        assert!(!codcel_is_blank(&Value::I32(0), &default_format()).unwrap());
        assert!(!codcel_is_blank(&Value::I32(42), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_non_empty_string() {
        assert!(!codcel_is_blank(&Value::String("test".to_string()), &default_format()).unwrap());
        assert!(!codcel_is_blank(&Value::String(" ".to_string()), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_bool() {
        assert!(!codcel_is_blank(&Value::Bool(true), &default_format()).unwrap());
        assert!(!codcel_is_blank(&Value::Bool(false), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_option_some_values() {
        assert!(!codcel_is_blank(&Value::OptionF64(Some(42.0)), &default_format()).unwrap());
        assert!(!codcel_is_blank(&Value::OptionI32(Some(42)), &default_format()).unwrap());
        assert!(!codcel_is_blank(
            &Value::OptionString(Some("test".to_string())),
            &default_format()
        )
        .unwrap());
        assert!(!codcel_is_blank(&Value::OptionBool(Some(true)), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_datetime() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        assert!(!codcel_is_blank(&Value::ChronoDateTime(dt), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_time() {
        let time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert!(!codcel_is_blank(&Value::Time(time), &default_format()).unwrap());
    }

    #[test]
    fn test_is_blank_with_vec_value() {
        assert!(!codcel_is_blank(&Value::VecValue(vec![]), &default_format()).unwrap());
        assert!(!codcel_is_blank(&Value::VecValue(vec![Value::None]), &default_format()).unwrap());
    }
}
