// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::excel_error::{error_type, ExcelError};
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `ISNA` that checks whether a value is the `#N/A` error specifically.
/// - `value`: the cell value to test.
/// - `_value_format`: unused; retained for signature consistency with other functions.
pub fn codcel_isna(
    value: &Value,
    _value_format: &ValueFormat,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(error_type(value) == Some(ExcelError::Na))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveTime, TimeZone, Utc};

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

    // --- TRUE cases ---

    #[test]
    fn test_isna_with_typed_na() {
        assert!(codcel_isna(&Value::Error(ExcelError::Na), &default_format()).unwrap());
    }

    #[test]
    fn test_isna_with_legacy_nan_f64() {
        assert!(codcel_isna(&Value::F64(f64::NAN), &default_format()).unwrap());
        assert!(codcel_isna(&Value::OptionF64(Some(f64::NAN)), &default_format()).unwrap());
    }

    #[test]
    fn test_isna_with_legacy_string() {
        assert!(codcel_isna(&Value::String("#N/A".to_string()), &default_format()).unwrap());
    }

    // --- FALSE cases (other errors and non-errors) ---

    #[test]
    fn test_isna_with_other_typed_errors() {
        for e in [
            ExcelError::Null,
            ExcelError::Div0,
            ExcelError::Value,
            ExcelError::Ref,
            ExcelError::Name,
            ExcelError::Num,
        ] {
            assert!(!codcel_isna(&Value::Error(e), &default_format()).unwrap());
        }
    }

    #[test]
    fn test_isna_with_legacy_non_na_string_errors() {
        assert!(!codcel_isna(&Value::String("#NUM!".to_string()), &default_format()).unwrap());
        assert!(!codcel_isna(&Value::String("#DIV/0!".to_string()), &default_format()).unwrap());
    }

    #[test]
    fn test_isna_with_number() {
        assert!(!codcel_isna(&Value::F64(42.0), &default_format()).unwrap());
        assert!(!codcel_isna(&Value::I32(0), &default_format()).unwrap());
    }

    #[test]
    fn test_isna_with_string() {
        assert!(!codcel_isna(&Value::String("hello".to_string()), &default_format()).unwrap());
    }

    #[test]
    fn test_isna_with_bool() {
        assert!(!codcel_isna(&Value::Bool(true), &default_format()).unwrap());
    }

    #[test]
    fn test_isna_with_none() {
        assert!(!codcel_isna(&Value::None, &default_format()).unwrap());
    }

    #[test]
    fn test_isna_with_datetime() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        assert!(!codcel_isna(&Value::ChronoDateTime(dt), &default_format()).unwrap());
    }

    #[test]
    fn test_isna_with_time() {
        let time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert!(!codcel_isna(&Value::Time(time), &default_format()).unwrap());
    }
}
