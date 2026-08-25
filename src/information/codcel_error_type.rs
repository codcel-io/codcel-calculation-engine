// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::excel_error::error_type;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `ERROR.TYPE` that returns a numeric code for the error kind.
///
/// Returns:
/// - 1 for `#NULL!`
/// - 2 for `#DIV/0!`
/// - 3 for `#VALUE!`
/// - 4 for `#REF!`
/// - 5 for `#NAME?`
/// - 6 for `#NUM!`
/// - 7 for `#N/A` (this also covers legacy `f64::NAN` error values whose
///   type can't be distinguished further)
///
/// For non-error inputs, Excel itself returns `#N/A`. This implementation returns
/// `f64::NAN` so the wrapping `process_area_value_to_float` produces a numeric
/// `Value::F64(NaN)` (the engine's legacy `#N/A` representation), which keeps
/// `IFERROR(ERROR.TYPE(x), …)` working naturally.
pub fn codcel_error_type(
    value: &Value,
    _value_format: &ValueFormat,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(match error_type(value) {
        Some(e) => e.to_type_code(),
        None => f64::NAN,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::excel_error::ExcelError;

    fn default_format() -> ValueFormat {
        ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        }
    }

    #[test]
    fn test_error_type_per_variant() {
        let cases = [
            (ExcelError::Null, 1.0),
            (ExcelError::Div0, 2.0),
            (ExcelError::Value, 3.0),
            (ExcelError::Ref, 4.0),
            (ExcelError::Name, 5.0),
            (ExcelError::Num, 6.0),
            (ExcelError::Na, 7.0),
        ];
        for (variant, expected) in cases {
            let result = codcel_error_type(&Value::Error(variant), &default_format()).unwrap();
            assert_eq!(
                result, expected,
                "ERROR.TYPE({variant:?}) should be {expected}"
            );
        }
    }

    #[test]
    fn test_error_type_legacy_nan_returns_na() {
        let result = codcel_error_type(&Value::F64(f64::NAN), &default_format()).unwrap();
        assert_eq!(result, 7.0);
        let result =
            codcel_error_type(&Value::OptionF64(Some(f64::NAN)), &default_format()).unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_error_type_legacy_string_errors() {
        let result =
            codcel_error_type(&Value::String("#NUM!".to_string()), &default_format()).unwrap();
        assert_eq!(result, 6.0);
        let result =
            codcel_error_type(&Value::String("#DIV/0!".to_string()), &default_format()).unwrap();
        assert_eq!(result, 2.0);
        let result =
            codcel_error_type(&Value::String("#VALUE!".to_string()), &default_format()).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_error_type_non_error_returns_nan() {
        let result = codcel_error_type(&Value::F64(42.0), &default_format()).unwrap();
        assert!(
            result.is_nan(),
            "ERROR.TYPE of a number must be #N/A (NaN), got {result}"
        );
        let result =
            codcel_error_type(&Value::String("hello".to_string()), &default_format()).unwrap();
        assert!(result.is_nan());
        let result = codcel_error_type(&Value::Bool(true), &default_format()).unwrap();
        assert!(result.is_nan());
        let result = codcel_error_type(&Value::None, &default_format()).unwrap();
        assert!(result.is_nan());
    }
}
