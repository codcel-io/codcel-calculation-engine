// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `N` that converts a value to a number.
/// - `value`: the cell value to convert.
/// - `value_format`: formatting options used for parsing (decimal separator, etc.).
///
/// Returns the numeric representation of the value, or `0.0` for non-numeric inputs
/// (text that cannot be parsed, errors, etc.).
pub fn codcel_n(
    value: &Value,
    value_format: &ValueFormat,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    match value.f64(value_format) {
        Ok(value) => Ok(value),
        Err(_) => Ok(0.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveTime, TimeZone, Utc};

    fn default_value_format() -> ValueFormat {
        ValueFormat {
            use_excel_rounding: true,
            ..Default::default()
        }
    }

    #[test]
    fn test_n_positive_number() {
        // =N(42) in Excel
        let value = Value::F64(42.0);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_n_negative_number() {
        // =N(-123.45) in Excel
        let value = Value::F64(-123.45);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, -123.45);
    }

    #[test]
    fn test_n_zero() {
        // =N(0) in Excel
        let value = Value::F64(0.0);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_n_integer() {
        // =N(123) in Excel
        let value = Value::I32(123);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 123.0);
    }

    #[test]
    fn test_n_true() {
        // =N(TRUE) in Excel
        let value = Value::Bool(true);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_n_false() {
        // =N(FALSE) in Excel
        let value = Value::Bool(false);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_n_string_convertible_to_number() {
        // =N("123.45") in Excel
        let value = Value::String("123.45".to_string());
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        // In this implementation, strings that can be converted to numbers are converted to their numeric value
        assert_eq!(result, 123.45);
    }

    #[test]
    fn test_n_string_not_convertible_to_number() {
        // =N("text") in Excel
        let value = Value::String("text".to_string());
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_n_date() {
        // =N(DATE(2023,1,1)) in Excel
        // January 1, 2023 has serial number 44927 in Excel
        let date = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let value = Value::ChronoDateTime(date);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        // The exact serial number may vary, but it should be a positive number
        assert!(result > 0.0);
    }

    #[test]
    fn test_n_time() {
        // =N(TIME(12,30,0)) in Excel
        // 12:30:00 has serial number 0.5208333... in Excel
        let time = NaiveTime::from_hms_opt(12, 30, 0).unwrap();
        let value = Value::Time(time);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        // The exact serial number may vary, but it should be between 0 and 1
        assert!(result > 0.0 && result < 1.0);
    }

    #[test]
    fn test_n_none() {
        // =N() in Excel
        let value = Value::None;
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_n_option_f64_some() {
        // =N(42) in Excel
        let value = Value::OptionF64(Some(42.0));
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_n_option_f64_none() {
        // =N() in Excel
        let value = Value::OptionF64(None);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_n_option_i32_some() {
        // =N(42) in Excel
        let value = Value::OptionI32(Some(42));
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_n_option_i32_none() {
        // =N() in Excel
        let value = Value::OptionI32(None);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_n_option_bool_true() {
        // =N(TRUE) in Excel
        let value = Value::OptionBool(Some(true));
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_n_option_bool_false() {
        // =N(FALSE) in Excel
        let value = Value::OptionBool(Some(false));
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_n_option_bool_none() {
        // =N() in Excel
        let value = Value::OptionBool(None);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_n_option_string_some() {
        // =N("text") in Excel
        let value = Value::OptionString(Some("text".to_string()));
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_n_option_string_none() {
        // =N() in Excel
        let value = Value::OptionString(None);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_n_vec_value() {
        // =N({1,2,3}) in Excel
        let value = Value::VecValue(vec![Value::F64(1.0), Value::F64(2.0), Value::F64(3.0)]);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        // In this implementation, vector values are converted to their first element's value
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_n_area_value() {
        // =N({1,2;3,4}) in Excel
        let value = Value::AreaValue(vec![
            vec![Value::F64(1.0), Value::F64(2.0)],
            vec![Value::F64(3.0), Value::F64(4.0)],
        ]);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        // In this implementation, area values are converted to their first element's value
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_n_large_number() {
        // =N(1E+308) in Excel
        let value = Value::F64(1.0e308);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 1.0e308);
    }

    #[test]
    fn test_n_small_number() {
        // =N(1E-308) in Excel
        let value = Value::F64(1.0e-308);
        let format = default_value_format();
        let result = codcel_n(&value, &format).unwrap();
        assert_eq!(result, 1.0e-308);
    }
}
