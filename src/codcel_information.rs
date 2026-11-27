// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::area::{process_area_value_to_bool, process_area_value_to_float};
use crate::information::codcel_error_type::codcel_error_type;
use crate::information::codcel_is_blank::codcel_is_blank;
use crate::information::codcel_is_even::codcel_is_even;
use crate::information::codcel_is_logical::codcel_is_logical;
use crate::information::codcel_is_non_text::codcel_is_non_text;
use crate::information::codcel_is_number::codcel_is_number;
use crate::information::codcel_is_odd::codcel_is_odd;
use crate::information::codcel_is_omitted::codcel_is_omitted;
use crate::information::codcel_is_text::codcel_is_text;
use crate::information::codcel_iserr::codcel_iserr;
use crate::information::codcel_iserror::codcel_iserror;
use crate::information::codcel_isna::codcel_isna;
use crate::information::codcel_n::codcel_n;
use crate::information::codcel_type::codcel_type;
use crate::excel_error::{err_to_box, ExcelError};
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Raises the Excel error whose `ERROR.TYPE` number is `code` (1 `#NULL!`, 2 `#DIV/0!`,
/// 3 `#VALUE!`, 4 `#REF!`, 5 `#NAME?`, 6 `#NUM!`, 7 `#N/A`, 9 `#SPILL!`, 14 `#CALC!`).
///
/// Not an Excel function: it is the transpiler's `CODCEL.ERROR`, standing in for an error value
/// typed into a cell (`#NAME?`), which a formula cannot otherwise write. An unknown code is
/// `#VALUE!`.
pub fn codcel_error(code: Value, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let code = code.f64(value_format)?;
    Err(err_to_box(ExcelError::from_type_code(code).unwrap_or(ExcelError::Value)))
}

pub fn n(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_float(value, strict_type_conversion, value_format, codcel_n)
}

pub fn is_odd(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::Bool(codcel_is_odd(value)?))
}

pub fn is_even(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::Bool(codcel_is_even(value)?))
}

pub fn is_number(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_bool(
        value,
        strict_type_conversion,
        value_format,
        codcel_is_number,
    )
}

pub fn is_logical(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_bool(
        value,
        strict_type_conversion,
        value_format,
        codcel_is_logical,
    )
}

pub fn is_text(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_bool(value, strict_type_conversion, value_format, codcel_is_text)
}

pub fn is_non_text(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_bool(
        value,
        strict_type_conversion,
        value_format,
        codcel_is_non_text,
    )
}

pub fn is_blank(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_bool(value, strict_type_conversion, value_format, codcel_is_blank)
}

pub fn iserr(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_bool(value, strict_type_conversion, value_format, codcel_iserr)
}

pub fn iserror(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_bool(value, strict_type_conversion, value_format, codcel_iserror)
}

pub fn is_omitted(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_bool(
        value,
        strict_type_conversion,
        value_format,
        codcel_is_omitted,
    )
}

pub fn excel_type(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_float(value, strict_type_conversion, value_format, codcel_type)
}

pub fn isna(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_bool(value, strict_type_conversion, value_format, codcel_isna)
}

pub fn error_type(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_value_to_float(
        value,
        strict_type_conversion,
        value_format,
        codcel_error_type,
    )
}

#[cfg(test)]
mod codcel_error_tests {
    use super::*;
    use crate::value_format::ValueFormat;

    #[test]
    fn codcel_error_raises_the_error_of_its_type_code() {
        let format = ValueFormat::default();
        for (code, expected) in [
            (1.0, ExcelError::Null),
            (5.0, ExcelError::Name),
            (7.0, ExcelError::Na),
            (9.0, ExcelError::Spill),
            (14.0, ExcelError::Calc),
        ] {
            let error = codcel_error(Value::F64(code), &format).unwrap_err();
            assert_eq!(ExcelError::from_err(error.as_ref()), expected);
        }
        let unknown = codcel_error(Value::F64(42.0), &format).unwrap_err();
        assert_eq!(ExcelError::from_err(unknown.as_ref()), ExcelError::Value);
    }
}
