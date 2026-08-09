// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::area::{process_area_value_to_bool, process_area_value_to_float};
use crate::information::codcel_error_type::codcel_error_type;
use crate::information::codcel_is_blank::codcel_is_blank;
use crate::information::codcel_is_even::codcel_is_even;
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
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

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
