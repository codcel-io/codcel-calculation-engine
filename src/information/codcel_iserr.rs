// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::excel_error::{error_type, ExcelError};
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `ISERR` that checks whether a value is an error other than `#N/A`.
/// - `value`: the cell value to test.
/// - `_value_format`: unused; retained for signature consistency with other functions.
pub fn codcel_iserr(
    value: &Value,
    _value_format: &ValueFormat,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(match error_type(value) {
        Some(ExcelError::Na) => false,
        Some(_) => true,
        None => false,
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

    // --- TRUE cases (typed errors other than #N/A) ---

    #[test]
    fn test_iserr_with_typed_error_variants() {
        for e in [
            ExcelError::Null,
            ExcelError::Div0,
            ExcelError::Value,
            ExcelError::Ref,
            ExcelError::Name,
            ExcelError::Num,
        ] {
            assert!(codcel_iserr(&Value::Error(e), &default_format()).unwrap());
        }
    }

    #[test]
    fn test_iserr_with_legacy_string_errors() {
        // Database functions historically return error strings; these are not #N/A.
        assert!(codcel_iserr(&Value::String("#NUM!".to_string()), &default_format()).unwrap());
        assert!(codcel_iserr(&Value::String("#DIV/0!".to_string()), &default_format()).unwrap());
        assert!(codcel_iserr(&Value::String("#VALUE!".to_string()), &default_format()).unwrap());
    }

    // --- FALSE cases ---

    #[test]
    fn test_iserr_with_na_typed_is_false() {
        assert!(!codcel_iserr(&Value::Error(ExcelError::Na), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_nan_f64_is_false() {
        // Legacy NaN errors are treated as #N/A, which ISERR excludes.
        assert!(!codcel_iserr(&Value::F64(f64::NAN), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::OptionF64(Some(f64::NAN)), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_legacy_na_string_is_false() {
        assert!(!codcel_iserr(&Value::String("#N/A".to_string()), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_number() {
        assert!(!codcel_iserr(&Value::F64(42.0), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::F64(0.0), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::F64(-1.5), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_infinity() {
        assert!(!codcel_iserr(&Value::F64(f64::INFINITY), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::F64(f64::NEG_INFINITY), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_i32() {
        assert!(!codcel_iserr(&Value::I32(42), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::I32(0), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_string() {
        assert!(!codcel_iserr(&Value::String("hello".to_string()), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::String("".to_string()), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_bool() {
        assert!(!codcel_iserr(&Value::Bool(true), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::Bool(false), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_none() {
        assert!(!codcel_iserr(&Value::None, &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_option_none() {
        assert!(!codcel_iserr(&Value::OptionF64(None), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::OptionI32(None), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::OptionString(None), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::OptionBool(None), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_option_some_values() {
        assert!(!codcel_iserr(&Value::OptionF64(Some(42.0)), &default_format()).unwrap());
        assert!(!codcel_iserr(&Value::OptionI32(Some(42)), &default_format()).unwrap());
        assert!(!codcel_iserr(
            &Value::OptionString(Some("test".to_string())),
            &default_format()
        )
        .unwrap());
        assert!(!codcel_iserr(&Value::OptionBool(Some(true)), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_datetime() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        assert!(!codcel_iserr(&Value::ChronoDateTime(dt), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_time() {
        let time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert!(!codcel_iserr(&Value::Time(time), &default_format()).unwrap());
    }

    #[test]
    fn test_iserr_with_vec_value() {
        assert!(!codcel_iserr(&Value::VecValue(vec![]), &default_format()).unwrap());
        assert!(!codcel_iserr(
            &Value::VecValue(vec![Value::F64(f64::NAN)]),
            &default_format()
        )
        .unwrap());
    }
}
