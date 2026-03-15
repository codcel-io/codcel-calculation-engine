// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `ISOMITTED` that checks whether a value represents an omitted parameter.
/// - `value`: the cell value to test.
/// - `_value_format`: unused; retained for signature consistency with other functions.
///
/// Returns `true` if the value is `Value::None` (representing an omitted optional parameter),
/// `false` otherwise.
pub fn codcel_is_omitted(
    value: &Value,
    _value_format: &ValueFormat,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(matches!(value, Value::None))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_format::ValueFormat;

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
    fn test_is_omitted_with_none() {
        // =ISOMITTED() with no argument (omitted parameter)
        assert!(codcel_is_omitted(&Value::None, &default_format()).unwrap());
    }

    #[test]
    fn test_is_omitted_with_number() {
        // =ISOMITTED(42) should return FALSE
        assert!(!codcel_is_omitted(&Value::F64(42.0), &default_format()).unwrap());
        assert!(!codcel_is_omitted(&Value::I32(42), &default_format()).unwrap());
    }

    #[test]
    fn test_is_omitted_with_string() {
        // =ISOMITTED("test") should return FALSE
        assert!(!codcel_is_omitted(&Value::String("test".to_string()), &default_format()).unwrap());
        // =ISOMITTED("") with empty string should return FALSE (empty string is not omitted)
        assert!(!codcel_is_omitted(&Value::String("".to_string()), &default_format()).unwrap());
    }

    #[test]
    fn test_is_omitted_with_bool() {
        // =ISOMITTED(TRUE) and =ISOMITTED(FALSE) should return FALSE
        assert!(!codcel_is_omitted(&Value::Bool(true), &default_format()).unwrap());
        assert!(!codcel_is_omitted(&Value::Bool(false), &default_format()).unwrap());
    }

    #[test]
    fn test_is_omitted_with_option_types() {
        // Option types with None values are different from Value::None
        assert!(!codcel_is_omitted(&Value::OptionF64(None), &default_format()).unwrap());
        assert!(!codcel_is_omitted(&Value::OptionString(None), &default_format()).unwrap());
        assert!(!codcel_is_omitted(&Value::OptionI32(None), &default_format()).unwrap());
        assert!(!codcel_is_omitted(&Value::OptionBool(None), &default_format()).unwrap());
    }

    #[test]
    fn test_is_omitted_with_option_some_values() {
        // Option types with Some values should return FALSE
        assert!(!codcel_is_omitted(&Value::OptionF64(Some(42.0)), &default_format()).unwrap());
        assert!(!codcel_is_omitted(&Value::OptionString(Some("test".to_string())), &default_format()).unwrap());
    }
}
