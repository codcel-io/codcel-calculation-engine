// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::excel_error::is_error;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `TYPE` that returns a numeric code indicating the data type of a value.
/// - `value`: the cell value to test.
/// - `_value_format`: unused; retained for signature consistency with other functions.
///
/// Returns:
/// - 1 for a number (including dates and times)
/// - 2 for text
/// - 4 for a logical (boolean) value
/// - 16 for an error value
/// - 64 for an array
pub fn codcel_type(
    value: &Value,
    _value_format: &ValueFormat,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Errors first — covers typed Value::Error, legacy NaN, and string sentinels.
    if is_error(value) {
        return Ok(16.0);
    }
    match value {
        // Logical values (must be checked before numbers since booleans can coerce to numbers)
        Value::Bool(_) => Ok(4.0),
        Value::OptionBool(Some(_)) => Ok(4.0),

        // Numbers (including dates and times, which are stored as numbers in Excel)
        Value::F64(_) => Ok(1.0),
        Value::I32(_) => Ok(1.0),
        Value::OptionF64(Some(_)) => Ok(1.0),
        Value::OptionI32(Some(_)) => Ok(1.0),
        Value::ChronoDateTime(_) => Ok(1.0),
        Value::OptionChronoDateTime(Some(_)) => Ok(1.0),
        Value::Time(_) => Ok(1.0),
        Value::OptionTime(Some(_)) => Ok(1.0),

        // Text
        Value::String(_) => Ok(2.0),
        Value::OptionString(Some(_)) => Ok(2.0),

        // Arrays
        Value::VecValue(_) => Ok(64.0),
        Value::OptionVecValue(Some(_)) => Ok(64.0),
        Value::AreaValue(_) => Ok(64.0),
        Value::OptionAreaValue(Some(_)) => Ok(64.0),

        // None and Option None variants default to number (empty cells are 0 in Excel)
        Value::None
        | Value::OptionF64(None)
        | Value::OptionI32(None)
        | Value::OptionString(None)
        | Value::OptionBool(None)
        | Value::OptionVecValue(None)
        | Value::OptionAreaValue(None)
        | Value::OptionChronoDateTime(None)
        | Value::OptionTime(None) => Ok(1.0),

        // Unreachable: errors handled above.
        Value::Error(_) => Ok(16.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_format::ValueFormat;
    use chrono::{NaiveTime, Utc};

    fn default_format() -> ValueFormat {
        ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: true,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        }
    }

    #[test]
    fn test_type_number_f64() {
        assert_eq!(codcel_type(&Value::F64(42.0), &default_format()).unwrap(), 1.0);
        assert_eq!(codcel_type(&Value::F64(0.0), &default_format()).unwrap(), 1.0);
        assert_eq!(codcel_type(&Value::F64(-3.14), &default_format()).unwrap(), 1.0);
    }

    #[test]
    fn test_type_number_i32() {
        assert_eq!(codcel_type(&Value::I32(42), &default_format()).unwrap(), 1.0);
        assert_eq!(codcel_type(&Value::I32(0), &default_format()).unwrap(), 1.0);
    }

    #[test]
    fn test_type_number_option() {
        assert_eq!(codcel_type(&Value::OptionF64(Some(1.0)), &default_format()).unwrap(), 1.0);
        assert_eq!(codcel_type(&Value::OptionI32(Some(1)), &default_format()).unwrap(), 1.0);
    }

    #[test]
    fn test_type_text() {
        assert_eq!(codcel_type(&Value::String("hello".to_string()), &default_format()).unwrap(), 2.0);
        assert_eq!(codcel_type(&Value::String("".to_string()), &default_format()).unwrap(), 2.0);
    }

    #[test]
    fn test_type_text_option() {
        assert_eq!(codcel_type(&Value::OptionString(Some("hello".to_string())), &default_format()).unwrap(), 2.0);
    }

    #[test]
    fn test_type_logical() {
        assert_eq!(codcel_type(&Value::Bool(true), &default_format()).unwrap(), 4.0);
        assert_eq!(codcel_type(&Value::Bool(false), &default_format()).unwrap(), 4.0);
    }

    #[test]
    fn test_type_logical_option() {
        assert_eq!(codcel_type(&Value::OptionBool(Some(true)), &default_format()).unwrap(), 4.0);
    }

    #[test]
    fn test_type_error() {
        assert_eq!(codcel_type(&Value::F64(f64::NAN), &default_format()).unwrap(), 16.0);
        assert_eq!(codcel_type(&Value::OptionF64(Some(f64::NAN)), &default_format()).unwrap(), 16.0);
    }

    #[test]
    fn test_type_array() {
        assert_eq!(
            codcel_type(&Value::VecValue(vec![Value::F64(1.0)]), &default_format()).unwrap(),
            64.0
        );
        assert_eq!(
            codcel_type(
                &Value::AreaValue(vec![vec![Value::F64(1.0)]]),
                &default_format()
            )
            .unwrap(),
            64.0
        );
    }

    #[test]
    fn test_type_datetime() {
        assert_eq!(codcel_type(&Value::ChronoDateTime(Utc::now()), &default_format()).unwrap(), 1.0);
        assert_eq!(codcel_type(&Value::OptionChronoDateTime(Some(Utc::now())), &default_format()).unwrap(), 1.0);
    }

    #[test]
    fn test_type_time() {
        let time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert_eq!(codcel_type(&Value::Time(time), &default_format()).unwrap(), 1.0);
        assert_eq!(codcel_type(&Value::OptionTime(Some(time)), &default_format()).unwrap(), 1.0);
    }

    #[test]
    fn test_type_none() {
        // Empty cells default to number (0) in Excel
        assert_eq!(codcel_type(&Value::None, &default_format()).unwrap(), 1.0);
    }

    #[test]
    fn test_type_option_none() {
        assert_eq!(codcel_type(&Value::OptionF64(None), &default_format()).unwrap(), 1.0);
        assert_eq!(codcel_type(&Value::OptionI32(None), &default_format()).unwrap(), 1.0);
        assert_eq!(codcel_type(&Value::OptionString(None), &default_format()).unwrap(), 1.0);
        assert_eq!(codcel_type(&Value::OptionBool(None), &default_format()).unwrap(), 1.0);
    }
}
