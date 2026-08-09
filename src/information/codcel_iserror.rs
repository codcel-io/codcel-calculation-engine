// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::excel_error::is_error;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `ISERROR` that checks whether a value is any error
/// (`#N/A`, `#VALUE!`, `#REF!`, `#DIV/0!`, `#NUM!`, `#NAME?`, `#NULL!`).
/// - `value`: the cell value to test.
/// - `_value_format`: unused; retained for signature consistency with other functions.
pub fn codcel_iserror(
    value: &Value,
    _value_format: &ValueFormat,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(is_error(value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_format::ValueFormat;
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
    fn test_iserror_with_nan_f64() {
        assert!(codcel_iserror(&Value::F64(f64::NAN), &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_nan_option_f64() {
        assert!(codcel_iserror(&Value::OptionF64(Some(f64::NAN)), &default_format()).unwrap());
    }

    // --- FALSE cases ---

    #[test]
    fn test_iserror_with_number() {
        assert!(!codcel_iserror(&Value::F64(42.0), &default_format()).unwrap());
        assert!(!codcel_iserror(&Value::F64(0.0), &default_format()).unwrap());
        assert!(!codcel_iserror(&Value::F64(-1.5), &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_infinity() {
        assert!(!codcel_iserror(&Value::F64(f64::INFINITY), &default_format()).unwrap());
        assert!(!codcel_iserror(&Value::F64(f64::NEG_INFINITY), &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_i32() {
        assert!(!codcel_iserror(&Value::I32(42), &default_format()).unwrap());
        assert!(!codcel_iserror(&Value::I32(0), &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_string() {
        assert!(!codcel_iserror(&Value::String("hello".to_string()), &default_format()).unwrap());
        assert!(!codcel_iserror(&Value::String("".to_string()), &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_bool() {
        assert!(!codcel_iserror(&Value::Bool(true), &default_format()).unwrap());
        assert!(!codcel_iserror(&Value::Bool(false), &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_none() {
        assert!(!codcel_iserror(&Value::None, &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_option_none() {
        assert!(!codcel_iserror(&Value::OptionF64(None), &default_format()).unwrap());
        assert!(!codcel_iserror(&Value::OptionI32(None), &default_format()).unwrap());
        assert!(!codcel_iserror(&Value::OptionString(None), &default_format()).unwrap());
        assert!(!codcel_iserror(&Value::OptionBool(None), &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_option_some_values() {
        assert!(!codcel_iserror(&Value::OptionF64(Some(42.0)), &default_format()).unwrap());
        assert!(!codcel_iserror(&Value::OptionI32(Some(42)), &default_format()).unwrap());
        assert!(!codcel_iserror(
            &Value::OptionString(Some("test".to_string())),
            &default_format()
        )
        .unwrap());
        assert!(!codcel_iserror(&Value::OptionBool(Some(true)), &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_datetime() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();
        assert!(!codcel_iserror(&Value::ChronoDateTime(dt), &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_time() {
        let time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        assert!(!codcel_iserror(&Value::Time(time), &default_format()).unwrap());
    }

    #[test]
    fn test_iserror_with_vec_value() {
        assert!(!codcel_iserror(&Value::VecValue(vec![]), &default_format()).unwrap());
        assert!(!codcel_iserror(
            &Value::VecValue(vec![Value::F64(f64::NAN)]),
            &default_format()
        )
        .unwrap());
    }
}
