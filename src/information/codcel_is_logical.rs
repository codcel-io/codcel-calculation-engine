// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `ISLOGICAL` that checks whether a value is a logical (boolean) value.
/// - `value`: the cell value to test.
/// - `_value_format`: unused; retained for signature consistency with other functions.
///
/// Returns `true` only for `TRUE`/`FALSE`. Numbers, text (including `"TRUE"`),
/// dates, errors and empty cells all return `false`, matching Excel.
pub fn codcel_is_logical(
    value: &Value,
    _value_format: &ValueFormat,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(value.is_excel_single_bool())
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
    fn test_is_logical_with_true() {
        // =ISLOGICAL(TRUE)
        assert!(codcel_is_logical(&Value::Bool(true), &default_format()).unwrap());
    }

    #[test]
    fn test_is_logical_with_false() {
        // =ISLOGICAL(FALSE)
        assert!(codcel_is_logical(&Value::Bool(false), &default_format()).unwrap());
    }

    #[test]
    fn test_is_logical_with_option_bool_some() {
        assert!(codcel_is_logical(&Value::OptionBool(Some(true)), &default_format()).unwrap());
        assert!(codcel_is_logical(&Value::OptionBool(Some(false)), &default_format()).unwrap());
    }

    // --- FALSE cases ---

    #[test]
    fn test_is_logical_with_number() {
        // =ISLOGICAL(1) — numbers are not logical values, even though TRUE coerces to 1
        assert!(!codcel_is_logical(&Value::F64(1.0), &default_format()).unwrap());
        assert!(!codcel_is_logical(&Value::F64(0.0), &default_format()).unwrap());
        assert!(!codcel_is_logical(&Value::I32(1), &default_format()).unwrap());
        assert!(!codcel_is_logical(&Value::OptionF64(Some(1.0)), &default_format()).unwrap());
        assert!(!codcel_is_logical(&Value::OptionI32(Some(0)), &default_format()).unwrap());
    }

    #[test]
    fn test_is_logical_with_text() {
        // =ISLOGICAL("TRUE") — the text "TRUE" is not a logical value
        assert!(!codcel_is_logical(&Value::String("TRUE".to_string()), &default_format()).unwrap());
        assert!(!codcel_is_logical(&Value::String("".to_string()), &default_format()).unwrap());
        assert!(!codcel_is_logical(
            &Value::OptionString(Some("FALSE".to_string())),
            &default_format()
        )
        .unwrap());
    }

    #[test]
    fn test_is_logical_with_empty() {
        // An empty cell is not a logical value.
        assert!(!codcel_is_logical(&Value::None, &default_format()).unwrap());
        assert!(!codcel_is_logical(&Value::OptionBool(None), &default_format()).unwrap());
        assert!(!codcel_is_logical(&Value::OptionString(None), &default_format()).unwrap());
    }

    #[test]
    fn test_is_logical_with_date_and_time() {
        let date_time = Utc
            .with_ymd_and_hms(2026, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        assert!(!codcel_is_logical(&Value::ChronoDateTime(date_time), &default_format()).unwrap());

        let time = NaiveTime::from_hms_opt(12, 30, 0).unwrap();
        assert!(!codcel_is_logical(&Value::Time(time), &default_format()).unwrap());
    }

    #[test]
    fn test_is_logical_with_array() {
        // A whole array is not a single logical value; broadcasting happens in the
        // `is_logical` wrapper, which visits each element individually.
        let area = Value::AreaValue(vec![vec![Value::Bool(true), Value::Bool(false)]]);
        assert!(!codcel_is_logical(&area, &default_format()).unwrap());

        let vec_value = Value::VecValue(vec![Value::Bool(true)]);
        assert!(!codcel_is_logical(&vec_value, &default_format()).unwrap());
    }
}
