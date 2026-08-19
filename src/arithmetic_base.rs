// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::area::{
    process_area_float_multi_to_float, process_area_float_op_float_to_float,
    process_area_float_to_float, process_area_float_to_integer, process_area_int_multi_to_int,
    process_area_int_op_int_to_string, process_area_string_to_float,
};
use crate::information::codcel_is_blank::{codcel_is_blank, codcel_is_blank_or_empty_string};
use crate::information::codcel_na::codcel_na;
use crate::logical::codcel_not::codcel_not;
use crate::maths::codcel_abs::codcel_abs;
use crate::maths::codcel_acos::codcel_acos;
use crate::maths::codcel_acosh::codcel_acosh;
use crate::maths::codcel_acot::codcel_acot;
use crate::maths::codcel_acoth::codcel_acoth;
use crate::maths::codcel_add::codcel_add;
use crate::maths::codcel_aggregate::codcel_aggregate;
use crate::maths::codcel_arabic::codcel_arabic;
use crate::maths::codcel_asin::codcel_asin;
use crate::maths::codcel_asinh::codcel_asinh;
use crate::maths::codcel_atan::codcel_atan;
use crate::maths::codcel_atan2::codcel_atan2;
use crate::maths::codcel_atanh::codcel_atanh;
use crate::maths::codcel_base::codcel_base;
use crate::maths::codcel_ceiling::codcel_ceiling;
use crate::maths::codcel_ceiling_math::codcel_ceiling_math;
use crate::maths::codcel_ceiling_precise::codcel_ceiling_precise;
use crate::maths::codcel_combin::codcel_combin;
use crate::maths::codcel_combina::codcel_combina;
use crate::maths::codcel_cos::codcel_cos;
use crate::maths::codcel_cosh::codcel_cosh;
use crate::maths::codcel_cot::codcel_cot;
use crate::maths::codcel_coth::codcel_coth;
use crate::maths::codcel_csc::codcel_csc;
use crate::maths::codcel_csch::codcel_csch;
use crate::maths::codcel_decimal::codcel_decimal;
use crate::maths::codcel_degrees::codcel_degrees;
use crate::maths::codcel_divide::codcel_divide;
use crate::maths::codcel_even::codcel_even;
use crate::maths::codcel_exp::codcel_exp;
use crate::maths::codcel_fact::codcel_fact;
use crate::maths::codcel_fact_double::codcel_fact_double;
use crate::maths::codcel_floor::codcel_floor;
use crate::maths::codcel_floor_math::codcel_floor_math;
use crate::maths::codcel_floor_precise::codcel_floor_precise;
use crate::maths::codcel_gcd::codcel_gcd;
use crate::maths::codcel_int::codcel_int;
use crate::maths::codcel_iso_ceiling::codcel_iso_ceiling;
use crate::maths::codcel_lcm::codcel_lcm;
use crate::maths::codcel_ln::codcel_ln;
use crate::maths::codcel_log::codcel_log;
use crate::maths::codcel_log10::codcel_log10;
use crate::maths::codcel_m_determ::codcel_m_determ;
use crate::maths::codcel_mround::codcel_mround;
use crate::maths::codcel_multinomial::codcel_multinomial;
use crate::maths::codcel_multiply::codcel_multiply;
use crate::maths::codcel_negative::codcel_negative;
use crate::maths::codcel_odd::codcel_odd;
use crate::maths::codcel_percentof::codcel_percentof;
use crate::maths::codcel_pi::codcel_pi;
use crate::maths::codcel_power::codcel_power_vec;
use crate::maths::codcel_quotient::codcel_quotient;
use crate::maths::codcel_radians::codcel_radians;
use crate::maths::codcel_rand::codcel_rand;
use crate::maths::codcel_rand_array::codcel_rand_array;
use crate::maths::codcel_rand_between::codcel_rand_between;
use crate::maths::codcel_roman::codcel_roman;
use crate::maths::codcel_round::codcel_round;
use crate::maths::codcel_round_down::codcel_round_down;
use crate::maths::codcel_round_up::codcel_round_up;
use crate::maths::codcel_sec::codcel_sec;
use crate::maths::codcel_sech::codcel_sech;
use crate::maths::codcel_sequence::codcel_sequence;
use crate::maths::codcel_series_sum::codcel_series_sum;
use crate::maths::codcel_sign::codcel_sign;
use crate::maths::codcel_sin::codcel_sin;
use crate::maths::codcel_sinh::codcel_sinh;
use crate::maths::codcel_sqrt::codcel_sqrt;
use crate::maths::codcel_sqrt_pi::codcel_sqrt_pi;
use crate::maths::codcel_sub_total::codcel_sub_total;
use crate::maths::codcel_subtract::codcel_subtract;
use crate::maths::codcel_sum_product::codcel_sum_product;
use crate::maths::codcel_sum_sq::codcel_sum_sq;
use crate::maths::codcel_sum_x2my2::codcel_sum_x2my2;
use crate::maths::codcel_sum_x2py2::codcel_sum_x2py2;
use crate::maths::codcel_sum_xmy2::codcel_sum_xmy2;
use crate::maths::codcel_tan::codcel_tan;
use crate::maths::codcel_tanh::codcel_tanh;
use crate::maths::codcel_trunc::codcel_trunc;
use crate::statistical::codcel_permut::codcel_permut_vec;
use crate::statistical::codcel_permutation_a::codcel_permutation_a_vec;
use crate::statistical::codcel_t_dist_2t::codcel_t_dist_2t_vec;
use crate::statistical::codcel_t_inv_2t::codcel_t_inv_2t_vec;
use crate::to_bool::ToBool;
use crate::to_f64::ToF64;
use crate::to_i32::ToI32;
use crate::value::{
    area_f64, flatten_value_to_vec_f64, vec_value_to_vec_f64, vec_value_to_vec_i32, Value,
};
use crate::value_format::ValueFormat;
use nalgebra::{DMatrix, LU};
use std::error::Error;
use std::fmt::{Debug, Display};

fn remove_quotes(input: &str) -> String {
    if input.starts_with('"') && input.ends_with('"') && input.len() > 1 {
        input[1..input.len() - 1].to_string()
    } else {
        input.to_string()
    }
}

// Excel-compatible coercion: an empty/missing cell evaluates to 0 in numeric context.
// Used by the scalar fallback of `add`/`subtract`/`multiply`/`divide` so that Lotus-style
// leading-plus formulas like `=+A1` (parsed as `BinOp(None, Add, A1)`) don't propagate
// "Cannot convert none to number value" errors.
fn value_to_f64_or_zero(
    value: Value,
    value_format: &ValueFormat,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    match value {
        Value::None
        | Value::OptionF64(None)
        | Value::OptionI32(None)
        | Value::OptionString(None)
        | Value::OptionBool(None)
        | Value::OptionChronoDateTime(None)
        | Value::OptionTime(None)
        | Value::OptionVecValue(None)
        | Value::OptionAreaValue(None) => Ok(0.0),
        other => other.f64(value_format),
    }
}

// Converts a String, &str, i32 into a f64, or just returns the f64
/// Converts a generic value to a floating-point number (`f64`).
///
/// This function attempts to convert any value implementing `ToF64` to an `f64`.
/// It handles locale-specific decimal separators by replacing them with standard dots.
///
/// # Parameters
/// - `value`: The value to convert (must implement `ToF64` and `Debug`).
/// - `decimal_separator`: The decimal separator used in string representations (e.g., "." or ",").
///
/// # Returns
/// Returns an `f64` representation of the value.
///
/// # Errors
/// Returns an error if the value cannot be converted to a floating-point number.
pub fn float<T: ToF64 + Debug>(
    value: T,
    decimal_separator: &str,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let string_value = format!("{:?}", value);
    match value.to_f64() {
        Ok(value) => Ok(value),
        Err(_) => {
            let test_string = remove_quotes(&string_value).replace(decimal_separator, ".");
            if let Ok(value) = test_string.to_f64() {
                return Ok(value);
            }

            Err(format!("Cannot convert {:#?} to a number", string_value).into())
        }
    }
}

/// Converts a numeric value to a string with locale-specific decimal separator.
///
/// This function converts any displayable value to a string representation,
/// replacing the standard dot decimal separator with the specified separator.
///
/// # Parameters
/// - `value`: The value to convert (must implement `Display`).
/// - `decimal_separator`: The decimal separator to use in the output string (e.g., "." or ",").
///
/// # Returns
/// Returns a string representation of the value with the specified decimal separator.
///
/// # Errors
/// Returns an error if the conversion fails (though this is unlikely for `Display` types).
pub fn float_to_string<T: Display>(
    value: T,
    decimal_separator: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut num = value.to_string();
    // TODO : CHECK IF THIS NEEDS TO BE SPECIFIC FOR INTEGER?????
    /*
    if !num.contains('.') {
        num = format!("{num}.0");
    }*/
    if "." != decimal_separator {
        num = num.replace('.', decimal_separator);
    }
    Ok(num)
}

fn format_with_thousand_separator(
    number_str: &str,
    decimal_separator: &str,
    thousands_separator: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Split into integer and fractional parts
    let parts: Vec<&str> = number_str.split('.').collect();
    let integer_part = parts[0];

    // Check if the number is negative
    let is_negative = integer_part.starts_with('-');
    let unsigned_integer_part = if is_negative {
        &integer_part[1..]
    } else {
        integer_part
    };

    // Reverse the integer part for easier grouping
    let reversed_chars: Vec<char> = unsigned_integer_part.chars().rev().collect();

    let thousands_separator_char = thousands_separator.chars().next().unwrap_or(' ');

    // Add thousands separators
    let mut formatted_integer = String::new();
    for (i, ch) in reversed_chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            formatted_integer.push(thousands_separator_char);
        }
        formatted_integer.push(*ch);
    }

    // Reverse back to original order
    let formatted_integer: String = formatted_integer.chars().rev().collect();

    // Prepend the negative sign if needed
    let formatted_integer = if is_negative {
        format!("-{formatted_integer}")
    } else {
        formatted_integer
    };

    // Append fractional part if it exists
    if parts.len() > 1 {
        Ok(format!(
            "{formatted_integer}{decimal_separator}{}",
            parts[1]
        ))
    } else {
        Ok(formatted_integer)
    }
}

/// Converts a numeric value to a formatted string with thousands and decimal separators.
///
/// This function converts any displayable value to a string with proper formatting,
/// including thousands separators (e.g., "1,234.56" or "1 234,56").
///
/// # Parameters
/// - `value`: The value to convert (must implement `Display`).
/// - `decimal_separator`: The decimal separator to use (e.g., "." or ",").
/// - `thousands_separator`: The thousands separator to use (e.g., "," or " ").
///
/// # Returns
/// Returns a formatted string representation with both thousands and decimal separators.
///
/// # Errors
/// Returns an error if the formatting fails.
pub fn float_to_formatted_string_display<T: Display>(
    value: T,
    decimal_separator: &str,
    thousands_separator: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let num = value.to_string();
    format_with_thousand_separator(&num, decimal_separator, thousands_separator)
}

/// Converts a generic value to a 32-bit integer (`i32`).
///
/// This function attempts to convert any value implementing `ToI32` to an `i32`.
/// It handles locale-specific decimal separators and truncates fractional parts.
///
/// # Parameters
/// - `value`: The value to convert (must implement `ToI32` and `Debug`).
/// - `decimal_separator`: The decimal separator used in string representations (e.g., "." or ",").
///
/// # Returns
/// Returns an `i32` representation of the value (truncating any fractional part).
///
/// # Errors
/// Returns an error with message "Cannot convert {value} to a whole number" if conversion fails.
pub fn integer<T: ToI32 + Debug>(
    value: T,
    decimal_separator: &str,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let string_value = format!("{:?}", value);
    match value.to_i32() {
        Ok(value) => Ok(value),
        Err(_) => {
            let test_string = remove_quotes(&string_value).replace(decimal_separator, ".");
            if let Ok(value) = test_string.to_i32() {
                return Ok(value);
            }

            Err(format!("Cannot convert {:#?} to a whole number", string_value).into())
        }
    }
}

/// Converts a generic value to a boolean.
///
/// This function attempts to convert any value implementing `ToBool` to a `bool`.
/// It handles locale-specific decimal separators in string representations.
///
/// # Parameters
/// - `value`: The value to convert (must implement `ToBool` and `Debug`).
/// - `decimal_separator`: The decimal separator used in string representations (e.g., "." or ",").
///
/// # Returns
/// Returns a `bool` representation of the value.
///
/// # Errors
/// Returns an error with message "Cannot convert {value} to a boolean" if conversion fails.
pub fn boolean<T: ToBool + Debug>(
    value: T,
    decimal_separator: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let string_value = format!("{:?}", value);
    match value.to_bool() {
        Ok(value) => Ok(value),
        Err(_) => {
            let test_string = remove_quotes(&string_value).replace(decimal_separator, ".");
            if let Ok(value) = test_string.to_bool() {
                return Ok(value);
            }

            Err(format!("Cannot convert {:#?} to a boolean", string_value).into())
        }
    }
}

/// Excel-compatible multiplication function.
/// Multiplies two values together.
///
/// # Parameters
/// - `lhs`: The left-hand side value (multiplicand).
/// - `rhs`: The right-hand side value (multiplier).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the product of the two values.
///
/// # Errors
/// Returns an error if either value cannot be converted to a number.
pub fn multiply(
    lhs: Value,
    rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Handle element-wise multiplication when lhs is an array
    match (&lhs, &rhs) {
        (Value::VecValue(lhs_vec), Value::VecValue(rhs_vec)) => {
            // Element-wise multiplication of two arrays
            let mut results = Vec::with_capacity(lhs_vec.len());
            for (l, r) in lhs_vec.iter().zip(rhs_vec.iter()) {
                let l_val = l.f64(value_format)?;
                let r_val = r.f64(value_format)?;
                results.push(Value::F64(codcel_multiply(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (Value::VecValue(lhs_vec), _) => {
            // Multiply each element of the array by a scalar
            let r_val = rhs.f64(value_format)?;
            let mut results = Vec::with_capacity(lhs_vec.len());
            for l in lhs_vec {
                let l_val = l.f64(value_format)?;
                results.push(Value::F64(codcel_multiply(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (_, Value::VecValue(rhs_vec)) => {
            // Multiply a scalar by each element of the array
            let l_val = lhs.f64(value_format)?;
            let mut results = Vec::with_capacity(rhs_vec.len());
            for r in rhs_vec {
                let r_val = r.f64(value_format)?;
                results.push(Value::F64(codcel_multiply(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (Value::AreaValue(lhs_rows), Value::AreaValue(rhs_rows)) => {
            // Element-wise multiplication of two 2D arrays
            let mut result_rows = Vec::with_capacity(lhs_rows.len());
            for (lhs_row, rhs_row) in lhs_rows.iter().zip(rhs_rows.iter()) {
                let mut result_row = Vec::with_capacity(lhs_row.len());
                for (l, r) in lhs_row.iter().zip(rhs_row.iter()) {
                    let l_val = l.f64(value_format)?;
                    let r_val = r.f64(value_format)?;
                    result_row.push(Value::F64(codcel_multiply(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        (Value::AreaValue(lhs_rows), _) => {
            // Multiply each element of the 2D array by a scalar
            let r_val = rhs.f64(value_format)?;
            let mut result_rows = Vec::with_capacity(lhs_rows.len());
            for lhs_row in lhs_rows {
                let mut result_row = Vec::with_capacity(lhs_row.len());
                for l in lhs_row {
                    let l_val = l.f64(value_format)?;
                    result_row.push(Value::F64(codcel_multiply(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        (_, Value::AreaValue(rhs_rows)) => {
            // Multiply a scalar by each element of the 2D array
            let l_val = lhs.f64(value_format)?;
            let mut result_rows = Vec::with_capacity(rhs_rows.len());
            for rhs_row in rhs_rows {
                let mut result_row = Vec::with_capacity(rhs_row.len());
                for r in rhs_row {
                    let r_val = r.f64(value_format)?;
                    result_row.push(Value::F64(codcel_multiply(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        _ => {}
    }

    // Single value multiplication
    let lhs = value_to_f64_or_zero(lhs, value_format)?;
    let rhs = value_to_f64_or_zero(rhs, value_format)?;
    Ok(Value::F64(codcel_multiply(lhs, rhs)?))
}

/// Excel-compatible addition function.
/// Adds two values together.
///
/// # Parameters
/// - `lhs`: The left-hand side value (first addend).
/// - `rhs`: The right-hand side value (second addend).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the sum of the two values.
///
/// # Errors
/// Returns an error if either value cannot be converted to a number.
pub fn add(
    lhs: Value,
    rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Handle element-wise addition when operands are arrays
    match (&lhs, &rhs) {
        (Value::VecValue(lhs_vec), Value::VecValue(rhs_vec)) => {
            let mut results = Vec::with_capacity(lhs_vec.len());
            for (l, r) in lhs_vec.iter().zip(rhs_vec.iter()) {
                let l_val = l.f64(value_format)?;
                let r_val = r.f64(value_format)?;
                results.push(Value::F64(codcel_add(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (Value::VecValue(lhs_vec), _) => {
            let r_val = rhs.f64(value_format)?;
            let mut results = Vec::with_capacity(lhs_vec.len());
            for l in lhs_vec {
                let l_val = l.f64(value_format)?;
                results.push(Value::F64(codcel_add(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (_, Value::VecValue(rhs_vec)) => {
            let l_val = lhs.f64(value_format)?;
            let mut results = Vec::with_capacity(rhs_vec.len());
            for r in rhs_vec {
                let r_val = r.f64(value_format)?;
                results.push(Value::F64(codcel_add(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (Value::AreaValue(lhs_rows), Value::AreaValue(rhs_rows)) => {
            let mut result_rows = Vec::with_capacity(lhs_rows.len());
            for (lhs_row, rhs_row) in lhs_rows.iter().zip(rhs_rows.iter()) {
                let mut result_row = Vec::with_capacity(lhs_row.len());
                for (l, r) in lhs_row.iter().zip(rhs_row.iter()) {
                    let l_val = l.f64(value_format)?;
                    let r_val = r.f64(value_format)?;
                    result_row.push(Value::F64(codcel_add(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        (Value::AreaValue(lhs_rows), _) => {
            let r_val = rhs.f64(value_format)?;
            let mut result_rows = Vec::with_capacity(lhs_rows.len());
            for lhs_row in lhs_rows {
                let mut result_row = Vec::with_capacity(lhs_row.len());
                for l in lhs_row {
                    let l_val = l.f64(value_format)?;
                    result_row.push(Value::F64(codcel_add(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        (_, Value::AreaValue(rhs_rows)) => {
            let l_val = lhs.f64(value_format)?;
            let mut result_rows = Vec::with_capacity(rhs_rows.len());
            for rhs_row in rhs_rows {
                let mut result_row = Vec::with_capacity(rhs_row.len());
                for r in rhs_row {
                    let r_val = r.f64(value_format)?;
                    result_row.push(Value::F64(codcel_add(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        _ => {}
    }

    let lhs = value_to_f64_or_zero(lhs, value_format)?;
    let rhs = value_to_f64_or_zero(rhs, value_format)?;
    Ok(Value::F64(codcel_add(lhs, rhs)?))
}

/// Excel-compatible `PI` function.
/// Returns the mathematical constant π (pi), approximately 3.14159265358979.
///
/// # Parameters
/// - `_value_format`: Format settings (unused for this function).
///
/// # Returns
/// Returns a `Value::F64` containing the value of π.
///
/// # Errors
/// This function does not return errors.
pub fn pi(_value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    Ok(Value::F64(codcel_pi()?))
}

/// Excel-compatible `NA` function.
/// Returns the `#N/A` error value (represented as `f64::NAN`).
///
/// # Parameters
/// - `_value_format`: Format settings (unused for this function).
///
/// # Returns
/// Returns a `Value::F64` containing `f64::NAN`.
pub fn na(_value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    Ok(Value::F64(codcel_na()?))
}

/// Excel-compatible division function.
/// Divides one value by another.
///
/// # Parameters
/// - `lhs`: The left-hand side value (dividend/numerator).
/// - `rhs`: The right-hand side value (divisor/denominator).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the quotient of the division.
///
/// # Errors
/// Returns an error if either value cannot be converted to a number, or if division by zero is attempted.
pub fn divide(
    lhs: Value,
    rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Handle element-wise division when operands are arrays
    match (&lhs, &rhs) {
        (Value::VecValue(lhs_vec), Value::VecValue(rhs_vec)) => {
            let mut results = Vec::with_capacity(lhs_vec.len());
            for (l, r) in lhs_vec.iter().zip(rhs_vec.iter()) {
                let l_val = l.f64(value_format)?;
                let r_val = r.f64(value_format)?;
                results.push(Value::F64(codcel_divide(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (Value::VecValue(lhs_vec), _) => {
            let r_val = rhs.f64(value_format)?;
            let mut results = Vec::with_capacity(lhs_vec.len());
            for l in lhs_vec {
                let l_val = l.f64(value_format)?;
                results.push(Value::F64(codcel_divide(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (_, Value::VecValue(rhs_vec)) => {
            let l_val = lhs.f64(value_format)?;
            let mut results = Vec::with_capacity(rhs_vec.len());
            for r in rhs_vec {
                let r_val = r.f64(value_format)?;
                results.push(Value::F64(codcel_divide(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (Value::AreaValue(lhs_rows), Value::AreaValue(rhs_rows)) => {
            let mut result_rows = Vec::with_capacity(lhs_rows.len());
            for (lhs_row, rhs_row) in lhs_rows.iter().zip(rhs_rows.iter()) {
                let mut result_row = Vec::with_capacity(lhs_row.len());
                for (l, r) in lhs_row.iter().zip(rhs_row.iter()) {
                    let l_val = l.f64(value_format)?;
                    let r_val = r.f64(value_format)?;
                    result_row.push(Value::F64(codcel_divide(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        (Value::AreaValue(lhs_rows), _) => {
            let r_val = rhs.f64(value_format)?;
            let mut result_rows = Vec::with_capacity(lhs_rows.len());
            for lhs_row in lhs_rows {
                let mut result_row = Vec::with_capacity(lhs_row.len());
                for l in lhs_row {
                    let l_val = l.f64(value_format)?;
                    result_row.push(Value::F64(codcel_divide(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        (_, Value::AreaValue(rhs_rows)) => {
            let l_val = lhs.f64(value_format)?;
            let mut result_rows = Vec::with_capacity(rhs_rows.len());
            for rhs_row in rhs_rows {
                let mut result_row = Vec::with_capacity(rhs_row.len());
                for r in rhs_row {
                    let r_val = r.f64(value_format)?;
                    result_row.push(Value::F64(codcel_divide(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        _ => {}
    }

    let lhs = value_to_f64_or_zero(lhs, value_format)?;
    let rhs = value_to_f64_or_zero(rhs, value_format)?;
    Ok(Value::F64(codcel_divide(lhs, rhs)?))
}

/// Excel-compatible `ACOT` function.
/// Calculates the arccotangent (inverse cotangent) of a number.
///
/// # Parameters
/// - `area`: The number for which to calculate the arccotangent.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the arccotangent in radians (between 0 and π).
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true).
pub fn acot(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_acot,
        "ACOT",
    )
}

/// Excel-compatible `SIN` function.
/// Calculates the sine of an angle given in radians.
///
/// # Parameters
/// - `area`: The angle in radians.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the sine of the angle (between -1 and 1).
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true).
pub fn sin(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_sin,
        "SIN",
    )
}

/// Excel-compatible `SINH` function.
/// Calculates the hyperbolic sine of a number.
///
/// # Parameters
/// - `area`: The number for which to calculate the hyperbolic sine.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the hyperbolic sine of the number.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true).
pub fn sinh(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_sinh,
        "SINH",
    )
}

/// Excel-compatible `SQRT` function.
/// Calculates the square root of a number.
///
/// # Parameters
/// - `area`: The number for which to calculate the square root (must be non-negative).
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the square root of the number.
///
/// # Errors
/// Returns an error if the value is negative, or cannot be converted to a number (when strict_type_conversion is true).
pub fn sqrt(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_sqrt,
        "SQRT",
    )
}

/// Pass-through function for array anchoring.
/// Returns the input value unchanged, used internally for array handling.
///
/// # Parameters
/// - `area`: The value to pass through.
/// - `_value_format`: Format settings (unused for this function).
///
/// # Returns
/// Returns the input value unchanged.
///
/// # Errors
/// This function does not return errors.
pub fn anchor_array(
    area: Value,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    Ok(area)
}

/// Excel-compatible `LOG10` function.
/// Calculates the base-10 logarithm of a number.
///
/// # Parameters
/// - `area`: The positive number for which to calculate the logarithm.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the base-10 logarithm.
///
/// # Errors
/// Returns an error if the value is zero or negative, or cannot be converted to a number (when strict_type_conversion is true).
pub fn log10(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_log10,
        "LOG10",
    )
}

/// Excel-compatible `TAN` function.
/// Calculates the tangent of an angle given in radians.
///
/// # Parameters
/// - `area`: The angle in radians.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the tangent of the angle.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true).
pub fn tan(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_tan,
        "TAN",
    )
}

/// Excel-compatible `TANH` function.
/// Calculates the hyperbolic tangent of a number.
///
/// # Parameters
/// - `area`: The number for which to calculate the hyperbolic tangent.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the hyperbolic tangent (between -1 and 1).
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true).
pub fn tanh(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_tanh,
        "TANH",
    )
}

/// Excel-compatible `POWER` function.
/// Raises a number to a specified power.
///
/// # Parameters
/// - `area`: The base number.
/// - `second_area`: The exponent.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the result of base^exponent.
///
/// # Errors
/// Returns an error if conversion fails or if the operation is invalid (e.g., negative base with fractional exponent).
pub fn power(
    area: Value,
    second_area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![area, second_area],
        strict_type_conversion,
        value_format,
        "POWER",
        codcel_power_vec,
    )
}

/// Excel-compatible `QUOTIENT` function.
/// Returns the integer portion of a division operation.
///
/// # Parameters
/// - `numerator`: The dividend.
/// - `denominator`: The divisor.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing the integer quotient (truncated toward zero).
///
/// # Errors
/// Returns an error if either value cannot be converted to a number, or if division by zero is attempted.
pub fn quotient(
    numerator: Value,
    denominator: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let numerator = numerator.f64(value_format)?;
    let denominator = denominator.f64(value_format)?;
    Ok(Value::I32(codcel_quotient(numerator, denominator)?))
}

/// Excel-compatible `SQRTPI` function.
/// Calculates the square root of (number × π).
///
/// # Parameters
/// - `value`: The number to multiply by π before taking the square root (must be non-negative).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the square root of (value × π).
///
/// # Errors
/// Returns an error if the value is negative, or cannot be converted to a number.
pub fn sqrt_pi(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_sqrt_pi(value)?))
}

/// Excel-compatible `DECIMAL` function.
/// Converts a text representation of a number in a given base to a decimal number.
///
/// # Parameters
/// - `values`: The text string to convert.
/// - `radix`: The base of the number (between 2 and 36).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing the decimal value.
///
/// # Errors
/// Returns an error if the string is invalid for the given base or if conversion fails.
pub fn decimal(
    values: Value,
    radix: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text_values = values.area_of_value()?;
    let radix_values = radix.area_of_value()?;

    let radix_value = if radix_values.len() == 1 && radix_values[0].len() == 1 {
        Some(radix.i32(value_format)?)
    } else {
        None
    };

    let text_value = if text_values.len() == 1 && text_values[0].len() == 1 {
        Some(
            values
                .string(value_format)
                .map_err(|_| "DECIMAL: Must contain text")?,
        )
    } else {
        None
    };

    let iteration_values = if text_value.is_some() {
        &radix_values
    } else {
        &text_values
    };

    let mut result = Vec::with_capacity(iteration_values.len());

    for (row_pos, row) in iteration_values.iter().enumerate() {
        let mut result_row = Vec::with_capacity(row.len());

        for (column_pos, _value) in row.iter().enumerate() {
            let text = if let Some(text) = &text_value {
                text.clone() // Clone the owned `String`
            } else {
                // Convert to an owned String to prevent temporary lifetimes
                text_values[row_pos][column_pos]
                    .string(value_format)
                    .expect("DECIMAL: Must contain text")
                    .to_owned()
            };

            let radix = if let Some(radix_value) = radix_value {
                radix_value
            } else {
                radix_values[row_pos][column_pos]
                    .i32(value_format)
                    .expect("DECIMAL: Radix must be a number")
            };

            let computed_result = codcel_decimal(&text, radix)?;
            result_row.push(Value::I32(computed_result));
        }

        result.push(result_row);
    }

    Ok(Value::AreaValue(result))
}

/// Excel-compatible `FLOOR` function.
/// Rounds a number down to the nearest multiple of significance.
///
/// # Parameters
/// - `values`: The number to round down.
/// - `significance`: The multiple to which to round.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the number rounded down to the nearest multiple of significance.
///
/// # Errors
/// Returns an error if values cannot be converted to numbers or if significance is zero.
pub fn floor(
    values: Value,
    significance: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number_values = values.area_of_value()?;
    let significance_values = significance.area_of_value()?;

    let significance_value = if significance_values.len() == 1 && significance_values[0].len() == 1
    {
        Some(significance.f64(value_format)?)
    } else {
        None
    };

    let number_value = if number_values.len() == 1 && number_values[0].len() == 1 {
        Some(
            values
                .f64(value_format)
                .map_err(|_| "FLOOR: Must contain a number")?,
        )
    } else {
        None
    };

    let iteration_values = if number_value.is_some() {
        &significance_values
    } else {
        &number_values
    };

    let mut result = Vec::with_capacity(iteration_values.len());

    for (row_pos, row) in iteration_values.iter().enumerate() {
        let mut result_row = Vec::with_capacity(row.len());

        for (column_pos, _value) in row.iter().enumerate() {
            let number = if let Some(number) = &number_value {
                *number // Clone the owned `String`
            } else {
                // Convert to an owned String to prevent temporary lifetimes
                number_values[row_pos][column_pos]
                    .f64(value_format)
                    .expect("FLOOR: Must contain a number")
                    .to_owned()
            };

            let significance = if let Some(significance_value) = significance_value {
                significance_value
            } else {
                significance_values[row_pos][column_pos]
                    .f64(value_format)
                    .expect("FLOOR: Significance must be a number")
            };

            let computed_result = codcel_floor(number, significance)?;
            result_row.push(Value::F64(computed_result));
        }

        result.push(result_row);
    }

    Ok(Value::AreaValue(result))
}

/// Excel-compatible `FLOOR.PRECISE` function.
/// Rounds a number down to the nearest multiple of significance, regardless of sign.
///
/// # Parameters
/// - `values`: The number to round down.
/// - `significance`: The multiple to which to round (defaults to 1 if omitted).
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the number rounded down.
///
/// # Errors
/// Returns an error if values cannot be converted to numbers or if significance is zero.
pub fn floor_precise(
    values: Value,
    significance: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_op_float_to_float(
        values,
        significance,
        strict_type_conversion,
        value_format,
        1.0,
        "FLOOR.PRECISE",
        codcel_floor_precise,
    )
}

/// Excel-compatible `ROMAN` function.
/// Converts an Arabic numeral to Roman numeral text.
///
/// # Parameters
/// - `values`: The Arabic number to convert (between 1 and 3999).
/// - `mode`: The form of Roman numeral (0-4, where 0 is classic and 4 is simplified).
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::String` containing the Roman numeral representation.
///
/// # Errors
/// Returns an error if the number is out of range or conversion fails.
pub fn roman(
    values: Value,
    mode: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_op_int_to_string(
        values,
        mode,
        strict_type_conversion,
        value_format,
        0,
        "ROMAN",
        codcel_roman,
    )
}

/// Excel-compatible `FLOOR.MATH` function.
/// Rounds a number down to the nearest multiple of significance with optional mode.
///
/// # Parameters
/// - `values`: The number to round down.
/// - `significance`: The multiple to which to round (defaults to 1 if omitted).
/// - `mode`: If non-zero, rounds negative numbers toward zero; if zero, rounds away from zero.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the number rounded according to the specified mode.
///
/// # Errors
/// Returns an error if values cannot be converted to numbers.
pub fn floor_math(
    values: Value,
    significance: Value,
    mode: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number_values = values.area_of_value()?;
    let significance_values = significance.option_area_of_value()?;
    let mode = mode.option_i32(value_format)?;

    let significance_value = if let Some(significance_values) = &significance_values {
        if significance_values.len() == 1 && significance_values[0].len() == 1 {
            Some(significance.f64(value_format)?)
        } else {
            None
        }
    } else {
        Some(1.0)
    };

    let number_value = if number_values.len() == 1 && number_values[0].len() == 1 {
        Some(
            values
                .f64(value_format)
                .map_err(|_| "FLOOR.MATH: Must contain a number")?,
        )
    } else {
        None
    };

    if let (Some(num), Some(sig)) = (number_value, significance_value) {
        return Ok(Value::F64(codcel_floor_math(num, Some(sig), mode)?));
    }

    let iteration_values = if number_value.is_some() {
        if let Some(values) = &significance_values {
            values
        } else {
            &number_values
        }
    } else {
        &number_values
    };

    let mut result = Vec::with_capacity(iteration_values.len());

    for (row_pos, row) in iteration_values.iter().enumerate() {
        let mut result_row = Vec::with_capacity(row.len());

        for (column_pos, _value) in row.iter().enumerate() {
            let number = if let Some(number) = &number_value {
                *number
            } else {
                // Convert to an owned String to prevent temporary lifetimes
                number_values[row_pos][column_pos]
                    .f64(value_format)
                    .expect("FLOOR.MATH: Must contain a number")
                    .to_owned()
            };

            let significance = if let Some(significance_value) = significance_value {
                significance_value
            } else if let Some(significance_values) = &significance_values {
                significance_values[row_pos][column_pos]
                    .f64(value_format)
                    .expect("FLOOR.MATH: Significance must be a number")
            } else {
                1.0
            };

            let computed_result = codcel_floor_math(number, Some(significance), mode)?;
            result_row.push(Value::F64(computed_result));
        }

        result.push(result_row);
    }

    Ok(Value::AreaValue(result))
}

/// Excel-compatible `ARABIC` function.
/// Converts a Roman numeral text to an Arabic numeral.
///
/// # Parameters
/// - `area`: The Roman numeral string to convert.
/// - `strict_type_conversion`: If `true`, returns error for invalid Roman numerals; if `false`, may return default values.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the Arabic numeral as a number.
///
/// # Errors
/// Returns an error if the Roman numeral is invalid.
pub fn arabic(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_string_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_arabic,
        "ARABIC",
    )
}

/// Excel-compatible `COT` function.
/// Calculates the cotangent of an angle given in radians.
///
/// # Parameters
/// - `area`: The angle in radians.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the cotangent of the angle.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true), or if the angle is a multiple of π.
pub fn cot(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_cot,
        "COT",
    )
}

/// Excel-compatible `SEC` function.
/// Calculates the secant of an angle given in radians.
///
/// # Parameters
/// - `area`: The angle in radians.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the secant of the angle.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true).
pub fn sec(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_sec,
        "SEC",
    )
}

/// Excel-compatible `SECH` function.
/// Calculates the hyperbolic secant of a number.
///
/// # Parameters
/// - `area`: The number for which to calculate the hyperbolic secant.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the hyperbolic secant (between 0 and 1).
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true).
pub fn sech(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_sech,
        "SECH",
    )
}

/// Excel-compatible `SIGN` function.
/// Returns the sign of a number: 1 if positive, -1 if negative, 0 if zero.
///
/// # Parameters
/// - `area`: The number to check.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing -1, 0, or 1.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true).
pub fn sign(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_integer(
        area,
        strict_type_conversion,
        value_format,
        codcel_sign,
        "SIGN",
    )
}

/// Excel-compatible `FACTDOUBLE` function.
/// Calculates the double factorial of a number (n!! = n × (n-2) × (n-4) × ...).
///
/// # Parameters
/// - `value`: The non-negative integer for which to calculate the double factorial.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing n!! (n double factorial).
///
/// # Errors
/// Returns an error if the value is negative or cannot be converted to an integer.
pub fn fact_double(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.i32(value_format)?;
    Ok(Value::I32(codcel_fact_double(value)?))
}

/// Excel-compatible `EXP` function.
/// Calculates e (Euler's number) raised to the power of a given number.
///
/// # Parameters
/// - `area`: The exponent to raise e to.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing e^value.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true).
pub fn exp(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_exp,
        "EXP",
    )
}

/// Excel-compatible `CSC` function.
/// Calculates the cosecant of an angle given in radians.
///
/// # Parameters
/// - `area`: The angle in radians.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the cosecant of the angle.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true), or if the angle is a multiple of π.
pub fn csc(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_csc,
        "CSC",
    )
}

/// Excel-compatible `CSCH` function.
/// Calculates the hyperbolic cosecant of a number.
///
/// # Parameters
/// - `area`: The number for which to calculate the hyperbolic cosecant (must not be zero).
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the hyperbolic cosecant.
///
/// # Errors
/// Returns an error if the value is zero, or cannot be converted to a number (when strict_type_conversion is true).
pub fn csch(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_csch,
        "CSCH",
    )
}

/// Excel-compatible `COTH` function.
/// Calculates the hyperbolic cotangent of a number.
///
/// # Parameters
/// - `area`: The number for which to calculate the hyperbolic cotangent (must not be zero).
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the hyperbolic cotangent.
///
/// # Errors
/// Returns an error if the value is zero, or cannot be converted to a number (when strict_type_conversion is true).
pub fn coth(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_coth,
        "COTH",
    )
}

/// Excel-compatible `BASE` function.
/// Converts a number to text representation in a given base (radix).
///
/// # Parameters
/// - `number`: The number to convert (must be non-negative integer).
/// - `radix`: The base to convert to (between 2 and 36).
/// - `min_length`: The minimum length of the returned string (pads with leading zeros if needed).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::String` containing the number in the specified base.
///
/// # Errors
/// Returns an error if values are invalid or cannot be converted.
pub fn base(
    number: Value,
    radix: Value,
    min_length: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number = number.i32(value_format)?;
    let radix = radix.i32(value_format)?;
    let min_length = min_length.option_i32(value_format)?;

    Ok(Value::String(codcel_base(number, radix, min_length)?))
}

/// Excel-compatible `COMBINA` function.
/// Calculates the number of combinations with repetitions allowed.
///
/// # Parameters
/// - `number`: The total number of items (n).
/// - `number_chosen`: The number of items to choose (k).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing C(n+k-1,k) = (n+k-1)! / (k! × (n-1)!).
///
/// # Errors
/// Returns an error if values are invalid (negative) or cannot be converted to integers.
pub fn combina(
    number: Value,
    number_chosen: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let n = number.i32(value_format)?;
    let k = number_chosen.i32(value_format)?;
    Ok(Value::I32(codcel_combina(n, k)?))
}

/// Excel-compatible `COMBIN` function.
/// Calculates the number of combinations (ways to choose k items from n items, order doesn't matter).
///
/// # Parameters
/// - `number`: The total number of items (n).
/// - `number_chosen`: The number of items to choose (k).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing C(n,k) = n! / (k! × (n-k)!).
///
/// # Errors
/// Returns an error if values are invalid (negative, k > n) or cannot be converted to integers.
pub fn combin(
    number: Value,
    number_chosen: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let n = number.i32(value_format)?;
    let k = number_chosen.i32(value_format)?;
    Ok(Value::I32(codcel_combin(n, k)?))
}

// Example usage for acoth
/// Excel-compatible `ACOTH` function.
/// Calculates the inverse hyperbolic cotangent of a number.
///
/// # Parameters
/// - `area`: The number for which to calculate the inverse hyperbolic cotangent (must be > 1 or < -1).
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the inverse hyperbolic cotangent.
///
/// # Errors
/// Returns an error if the value is between -1 and 1 (inclusive), or cannot be converted to a number (when strict_type_conversion is true).
pub fn acoth(
    area: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        area,
        strict_type_conversion,
        value_format,
        codcel_acoth,
        "ACOTH",
    )
}

/// Excel-compatible `MDETERM` function.
/// Calculates the determinant of a square matrix.
///
/// # Parameters
/// - `matrix`: The square matrix as a 2D array.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the determinant value.
///
/// # Errors
/// Returns an error if the matrix is not square or if calculation fails.
pub fn m_determ(
    matrix: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let matrix = matrix.area_f64(value_format)?;
    Ok(Value::F64(codcel_m_determ(matrix)?))
}

/// Excel-compatible `MINVERSE` function.
/// Calculates the inverse of a square matrix.
///
/// # Parameters
/// - `matrix`: The square matrix to invert.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::AreaValue` containing the inverted matrix.
///
/// # Errors
/// Returns an error if the matrix is singular (not invertible) or not square.
pub fn m_inverse(
    matrix: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let matrix = matrix.area_f64(value_format)?;

    // Ensure the matrix is non-empty and square
    if matrix.is_empty() || matrix.len() != matrix[0].len() {
        return Err("MINVERSE: Matrix must be square and non-empty".into());
    }

    let size = matrix.len();

    // Flatten the 2D Vec into a 1D slice
    let flat_matrix: Vec<f64> = matrix.iter().flatten().copied().collect();

    // Convert to nalgebra DMatrix
    let dmatrix = DMatrix::from_row_slice(size, size, &flat_matrix);

    // Perform LU decomposition and compute the inverse
    let lu = LU::new(dmatrix);
    if let Some(inverse) = lu.try_inverse() {
        // Convert the DMatrix back into a Vec<Vec<f64>>
        let inverse_vec: Vec<Vec<Value>> = (0..size)
            .map(|i| inverse.row(i).iter().map(|&v| Value::F64(v)).collect())
            .collect();
        Ok(Value::AreaValue(inverse_vec))
    } else {
        Err("MINVERSE: Matrix is singular and cannot be inverted".into())
    }
}

/// Excel-compatible `MROUND` function.
/// Rounds a number to the nearest multiple.
///
/// # Parameters
/// - `number`: The number to round.
/// - `multiple`: The multiple to which to round.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the number rounded to the nearest multiple.
///
/// # Errors
/// Returns an error if values cannot be converted to numbers or if multiple is zero.
pub fn mround(
    number: Value,
    multiple: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number = number.f64(value_format)?;
    let multiple = multiple.f64(value_format)?;
    Ok(Value::F64(codcel_mround(number, multiple)?))
}

/// Excel-compatible `MMULT` function.
/// Multiplies two matrices together.
///
/// # Parameters
/// - `matrix_a`: The first matrix.
/// - `matrix_b`: The second matrix.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::AreaValue` containing the product matrix.
///
/// # Errors
/// Returns an error if matrix dimensions are incompatible (columns of A ≠ rows of B).
pub fn m_mult(
    matrix_a: Value,
    matrix_b: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let a = matrix_a.area_f64(value_format)?;
    let b = matrix_b.area_f64(value_format)?;

    let rows_a = a.len();
    let cols_a = if rows_a > 0 { a[0].len() } else { 0 };

    let rows_b = b.len();
    let cols_b = if rows_b > 0 { b[0].len() } else { 0 };

    // Check if dimensions are compatible
    if cols_a != rows_b {
        return Err("MMULT: Number of columns in the first matrix must equal the number of rows in the second matrix".into());
    }

    // Flatten the matrices into row-major order
    let flat_a: Vec<f64> = a.iter().flatten().copied().collect();
    let flat_b: Vec<f64> = b.iter().flatten().copied().collect();

    // Create nalgebra DMatrix instances
    let matrix_a = DMatrix::from_row_slice(rows_a, cols_a, &flat_a);
    let matrix_b = DMatrix::from_row_slice(rows_b, cols_b, &flat_b);

    // Perform matrix multiplication
    let result_matrix = matrix_a * matrix_b;

    // Convert result back into a Vec<Vec<Value>>
    let result: Vec<Vec<Value>> = (0..result_matrix.nrows())
        .map(|i| {
            result_matrix
                .row(i)
                .iter()
                .map(|&v| Value::F64(v))
                .collect()
        })
        .collect();

    Ok(Value::AreaValue(result))
}

/// Excel-compatible `MUNIT` function.
/// Creates an identity matrix of the specified size.
///
/// # Parameters
/// - `size`: The dimension of the square identity matrix (n×n).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::AreaValue` containing the identity matrix (1s on diagonal, 0s elsewhere).
///
/// # Errors
/// Returns an error if size is invalid or cannot be converted to an integer.
pub fn m_unit(
    size: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Convert the input to usize for matrix size
    let size = size.i32(value_format)? as usize;

    // Create an identity matrix using nalgebra
    let identity_matrix = DMatrix::<f64>::identity(size, size);

    // Convert the identity matrix to Vec<Vec<Value>>
    let result: Vec<Vec<Value>> = (0..identity_matrix.nrows())
        .map(|i| {
            identity_matrix
                .row(i)
                .iter()
                .map(|&v| Value::F64(v))
                .collect()
        })
        .collect();

    // Return the result as a Value::AreaValue
    Ok(Value::AreaValue(result))
}

/// Excel-compatible subtraction function.
/// Subtracts one value from another.
///
/// # Parameters
/// - `lhs`: The left-hand side value (minuend).
/// - `rhs`: The right-hand side value (subtrahend).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the difference (lhs - rhs).
///
/// # Errors
/// Returns an error if either value cannot be converted to a number.
pub fn subtract(
    lhs: Value,
    rhs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Handle element-wise subtraction when operands are arrays
    match (&lhs, &rhs) {
        (Value::VecValue(lhs_vec), Value::VecValue(rhs_vec)) => {
            let mut results = Vec::with_capacity(lhs_vec.len());
            for (l, r) in lhs_vec.iter().zip(rhs_vec.iter()) {
                let l_val = l.f64(value_format)?;
                let r_val = r.f64(value_format)?;
                results.push(Value::F64(codcel_subtract(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (Value::VecValue(lhs_vec), _) => {
            let r_val = rhs.f64(value_format)?;
            let mut results = Vec::with_capacity(lhs_vec.len());
            for l in lhs_vec {
                let l_val = l.f64(value_format)?;
                results.push(Value::F64(codcel_subtract(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (_, Value::VecValue(rhs_vec)) => {
            let l_val = lhs.f64(value_format)?;
            let mut results = Vec::with_capacity(rhs_vec.len());
            for r in rhs_vec {
                let r_val = r.f64(value_format)?;
                results.push(Value::F64(codcel_subtract(l_val, r_val)?));
            }
            return Ok(Value::VecValue(results));
        }
        (Value::AreaValue(lhs_rows), Value::AreaValue(rhs_rows)) => {
            let mut result_rows = Vec::with_capacity(lhs_rows.len());
            for (lhs_row, rhs_row) in lhs_rows.iter().zip(rhs_rows.iter()) {
                let mut result_row = Vec::with_capacity(lhs_row.len());
                for (l, r) in lhs_row.iter().zip(rhs_row.iter()) {
                    let l_val = l.f64(value_format)?;
                    let r_val = r.f64(value_format)?;
                    result_row.push(Value::F64(codcel_subtract(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        (Value::AreaValue(lhs_rows), _) => {
            let r_val = rhs.f64(value_format)?;
            let mut result_rows = Vec::with_capacity(lhs_rows.len());
            for lhs_row in lhs_rows {
                let mut result_row = Vec::with_capacity(lhs_row.len());
                for l in lhs_row {
                    let l_val = l.f64(value_format)?;
                    result_row.push(Value::F64(codcel_subtract(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        (_, Value::AreaValue(rhs_rows)) => {
            let l_val = lhs.f64(value_format)?;
            let mut result_rows = Vec::with_capacity(rhs_rows.len());
            for rhs_row in rhs_rows {
                let mut result_row = Vec::with_capacity(rhs_row.len());
                for r in rhs_row {
                    let r_val = r.f64(value_format)?;
                    result_row.push(Value::F64(codcel_subtract(l_val, r_val)?));
                }
                result_rows.push(result_row);
            }
            return Ok(Value::AreaValue(result_rows));
        }
        _ => {}
    }

    let lhs = value_to_f64_or_zero(lhs, value_format)?;
    let rhs = value_to_f64_or_zero(rhs, value_format)?;
    Ok(Value::F64(codcel_subtract(lhs, rhs)?))
}

/// Excel-compatible `ODD` function.
/// Rounds a number up to the nearest odd integer (away from zero).
///
/// # Parameters
/// - `value`: The number to round.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the nearest odd integer.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number (when strict_type_conversion is true).
pub fn odd(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_integer(
        value,
        strict_type_conversion,
        value_format,
        codcel_odd,
        "ODD: This must be a number.",
    )
}

/// Excel-compatible `SUM` function.
/// Calculates the sum of all numeric values in the provided collection.
///
/// # Parameters
/// - `values`: A vector of values to sum.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the sum of all values. Returns 0 if the collection is empty.
///
/// # Errors
/// Returns an error if conversion fails (when strict_type_conversion is true).
pub fn sum(
    values: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::F64(0.0));
    }

    let result = values.iter().try_fold(
        0.0,
        |acc, value| -> Result<f64, Box<dyn Error + Send + Sync>> {
            if value.is_array() || value.is_area() {
                // Excel SUM ignores non-numeric cells in ranges/arrays
                let area = value.area_of_value()?;
                let mut local_sum = 0.0;
                for row in &area {
                    for cell in row {
                        if let Ok(val) = cell.f64(value_format) {
                            local_sum += val;
                        }
                    }
                }
                Ok(acc + local_sum)
            } else {
                // Try to add the value to the accumulator, stop if error occurs and strict_type_conversion is true
                match value.f64(value_format) {
                    Ok(val) => Ok(acc + val),
                    Err(_) => {
                        if strict_type_conversion {
                            Err("SUM: Input values are not numbers".into()) // Propagate error if strict
                        } else {
                            Ok(acc) // Ignore error and continue with the sum if not strict
                        }
                    }
                }
            }
        },
    )?;

    Ok(Value::F64(result))
}

/// Excel-compatible `DEVSQ` function.
/// Calculates the sum of squares of deviations from the mean.
///
/// # Parameters
/// - `values`: A vector of values for which to calculate the sum of squared deviations.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the sum of squared deviations. Returns 0 if the collection is empty.
///
/// # Errors
/// Returns an error if conversion fails (when strict_type_conversion is true).
pub fn devsq(
    values: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::F64(0.0));
    }

    let mut flat_values = Vec::new();

    // Flatten the input values into a single vector
    for value in &values {
        if value.is_array() || value.is_area() {
            match value.vec_f64(value_format) {
                Ok(vec) => flat_values.extend(vec),
                Err(_) => {
                    if strict_type_conversion {
                        return Err("DEVSQ: Input values are not numbers".into());
                    }
                }
            }
        } else {
            match value.f64(value_format) {
                Ok(val) => flat_values.push(val),
                Err(_) => {
                    if strict_type_conversion {
                        return Err("DEVSQ: Input values are not numbers".into());
                    }
                }
            }
        }
    }

    if flat_values.is_empty() {
        return Ok(Value::F64(0.0));
    }

    // Calculate the mean
    let mean = flat_values.iter().sum::<f64>() / flat_values.len() as f64;

    // Calculate the sum of squared deviations
    let devsq = flat_values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>();

    Ok(Value::F64(devsq))
}

/// Excel-compatible `PRODUCT` function.
/// Multiplies all numeric values in the provided collection.
///
/// # Parameters
/// - `values`: A vector of values to multiply.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 1.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the product of all values. Returns 0 if the collection is empty.
///
/// # Errors
/// Returns an error if conversion fails (when strict_type_conversion is true).
pub fn product(
    values: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::F64(1.0));
    }

    let result = values.iter().try_fold(
        1.0,
        |acc, value| -> Result<f64, Box<dyn Error + Send + Sync>> {
            if value.is_array() {
                // Try to multiply the values in the array with the accumulator
                let vec = value.vec_f64(value_format)?;
                let product_of_array = vec.iter().product::<f64>();
                Ok(acc * product_of_array)
            } else if value.is_area() {
                // Try to multiply the values in the 2D area with the accumulator
                let area = value.area_f64(value_format)?;
                let product_of_area = area.iter().flat_map(|row| row.iter()).product::<f64>();
                Ok(acc * product_of_area)
            } else {
                // Try to multiply the value with the accumulator, stop if error occurs and strict_type_conversion is true
                match value.f64(value_format) {
                    Ok(val) => Ok(acc * val),
                    Err(e) if strict_type_conversion => Err(e), // Convert error and propagate if strict
                    Err(_) => Ok(acc), // Ignore error and continue with the accumulation if not strict
                }
            }
        },
    )?;

    Ok(Value::F64(result))
}

/// Excel-compatible `COUNT` function.
/// Counts the number of numeric values in the provided collection.
///
/// # Parameters
/// - `values`: A vector of values to count.
/// - `_value_format`: Format settings (unused for this function).
///
/// # Returns
/// Returns a `Value::I32` containing the count of numeric values. Returns 0 if the collection is empty or contains no numbers.
///
/// # Errors
/// This function does not return errors.
pub fn count(
    values: Vec<Value>,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Return early with 0 if the values are empty
    if values.is_empty() {
        return Ok(Value::I32(0));
    }

    let count = values
        .iter()
        .filter_map(|value| value.area_of_value().ok()) // Handle potential errors from `area_of_value`
        .flat_map(|areas| areas.into_iter()) // Flatten the outer collection
        .flat_map(|rows| rows.into_iter()) // Flatten inner rows
        .filter(|cols| cols.is_count_number_type()) // Filter only the desired columns
        .count(); // Count the filtered elements

    Ok(Value::I32(count as i32))
}

/// Excel-compatible `COUNTA` function.
/// Counts the number of non-empty values in the provided collection.
///
/// # Parameters
/// - `values`: A vector of values to count.
/// - `_value_format`: Format settings (unused for this function).
///
/// # Returns
/// Returns a `Value::I32` containing the count of non-empty values. Returns 0 if the collection is empty.
///
/// # Errors
/// This function does not return errors.
pub fn counta(
    values: Vec<Value>,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::I32(0));
    }

    let count = values
        .iter()
        .filter_map(|value| value.area_of_value().ok())
        .flat_map(|areas| areas.into_iter())
        .flat_map(|rows| rows.into_iter())
        .filter(|cols| !codcel_is_blank(cols, _value_format).unwrap_or(false))
        .count();

    Ok(Value::I32(count as i32))
}

/// Excel-compatible `COUNTBLANK` function.
/// Counts the number of blank/empty cells in the provided collection.
///
/// # Parameters
/// - `values`: A vector of values to check for blanks.
/// - `value_format`: Format settings for blank detection.
///
/// # Returns
/// Returns a `Value::I32` containing the count of blank cells.
///
/// # Errors
/// Returns an error if blank detection fails for any cell.
pub fn countblank(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::I32(0));
    }

    let count = values
        .iter()
        .filter_map(|value| value.area_of_value().ok())
        .flat_map(|areas| areas.into_iter())
        .flat_map(|rows| rows.into_iter())
        .filter(|cell| codcel_is_blank_or_empty_string(cell, value_format).unwrap_or(false))
        .count();

    Ok(Value::I32(count as i32))
}

/// Excel-compatible `FACT` function.
/// Calculates the factorial of a number.
///
/// # Parameters
/// - `value`: The non-negative integer for which to calculate the factorial.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing n! (n factorial).
///
/// # Errors
/// Returns an error if the value is negative or cannot be converted to an integer.
pub fn fact(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.i32(value_format)?;
    Ok(Value::I32(codcel_fact(value)?))
}

/// Excel-compatible `MAX` function.
/// Returns the largest numeric value in the provided collection.
///
/// # Parameters
/// - `values`: A vector of values to search.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, ignores non-numeric values.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the maximum value. Returns negative infinity if the collection is empty.
///
/// # Errors
/// Returns an error if conversion fails (when strict_type_conversion is true).
pub fn max(
    values: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::F64(0.0));
    }

    let result = values.iter().try_fold(
        f64::MIN,
        |acc, value| -> Result<f64, Box<dyn Error + Send + Sync>> {
            if value.is_array() {
                // Try to find the maximum value in the array and compare it with the accumulator
                let vec = value.vec_f64(value_format)?;
                let max_in_array = vec.iter().fold(f64::MIN, |acc, &val| acc.max(val));
                Ok(acc.max(max_in_array))
            } else if value.is_area() {
                // Excel MAX ignores non-numeric cells in ranges/arrays
                let area = value.area_of_value()?;
                let mut local_max = acc;
                for row in &area {
                    for cell in row {
                        if let Ok(val) = cell.f64(value_format) {
                            local_max = local_max.max(val);
                        }
                    }
                }
                Ok(local_max)
            } else {
                // Excel MAX ignores text and boolean values from cell references
                if value.is_string() || matches!(value, Value::Bool(_)) {
                    return Ok(acc);
                }
                // Try to compare the value with the accumulator, stop if error occurs and strict_type_conversion is true
                match value.f64(value_format) {
                    Ok(val) => Ok(acc.max(val)),
                    Err(e) if strict_type_conversion => Err(e), // Convert error and propagate if strict
                    Err(_) => Ok(acc), // Ignore error and continue with the accumulation if not strict
                }
            }
        },
    )?;

    Ok(Value::F64(result))
}

/// Excel-compatible `INT` function.
/// Rounds a number down to the nearest integer.
///
/// # Parameters
/// - `value`: The number to round down.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing the integer part (rounded down).
///
/// # Errors
/// Returns an error if the value cannot be converted to a number.
pub fn int(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::I32(codcel_int(value)?))
}

/// Excel-compatible `ROUND` function.
/// Rounds a number to a specified number of decimal places.
///
/// # Parameters
/// - `value`: The number to round.
/// - `decimal_places`: The number of decimal places to round to (can be negative to round left of decimal point).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the rounded number.
///
/// # Errors
/// Returns an error if either value cannot be converted to the appropriate type.
pub fn round(
    value: Value,
    decimal_places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    let decimal_places = decimal_places.i32(value_format)?;
    Ok(Value::F64(codcel_round(value, decimal_places)?))
}

/// Excel-compatible `ROUNDDOWN` function.
/// Rounds a number down (toward zero) to a specified number of decimal places.
///
/// # Parameters
/// - `value`: The number to round down.
/// - `decimal_places`: The number of decimal places to round to (can be negative to round left of decimal point).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the number rounded down.
///
/// # Errors
/// Returns an error if either value cannot be converted to the appropriate type.
pub fn round_down(
    value: Value,
    decimal_places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    let decimal_places = decimal_places.i32(value_format)?;
    Ok(Value::F64(codcel_round_down(value, decimal_places)?))
}

/// Excel-compatible `ROUNDUP` function.
/// Rounds a number up (away from zero) to a specified number of decimal places.
///
/// # Parameters
/// - `value`: The number to round up.
/// - `decimal_places`: The number of decimal places to round to (can be negative to round left of decimal point).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the number rounded up.
///
/// # Errors
/// Returns an error if either value cannot be converted to the appropriate type.
pub fn round_up(
    value: Value,
    decimal_places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    let decimal_places = decimal_places.i32(value_format)?;
    Ok(Value::F64(codcel_round_up(value, decimal_places)?))
}

/// Excel-compatible `ABS` function.
/// Returns the absolute value of a number.
///
/// # Parameters
/// - `value`: The number for which to calculate the absolute value.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the absolute value (always non-negative).
///
/// # Errors
/// Returns an error if the value cannot be converted to a number.
pub fn abs(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_abs(value)?))
}

/// Excel-compatible `ACOS` function.
/// Calculates the arccosine (inverse cosine) of a number.
///
/// # Parameters
/// - `value`: The number for which to calculate the arccosine (must be between -1 and 1).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the arccosine in radians (between 0 and π).
///
/// # Errors
/// Returns an error if the value is not between -1 and 1, or cannot be converted to a number.
pub fn acos(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_acos(value)?))
}

/// Excel-compatible `ACOSH` function.
/// Calculates the inverse hyperbolic cosine of a number.
///
/// # Parameters
/// - `value`: The number for which to calculate the inverse hyperbolic cosine (must be >= 1).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the inverse hyperbolic cosine.
///
/// # Errors
/// Returns an error if the value is less than 1, or cannot be converted to a number.
pub fn acosh(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_acosh(value)?))
}

/// Excel-compatible `ASIN` function.
/// Calculates the arcsine (inverse sine) of a number.
///
/// # Parameters
/// - `value`: The number for which to calculate the arcsine (must be between -1 and 1).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the arcsine in radians (between -π/2 and π/2).
///
/// # Errors
/// Returns an error if the value is not between -1 and 1, or cannot be converted to a number.
pub fn asin(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_asin(value)?))
}

/// Excel-compatible `ASINH` function.
/// Calculates the inverse hyperbolic sine of a number.
///
/// # Parameters
/// - `value`: The number for which to calculate the inverse hyperbolic sine.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the inverse hyperbolic sine.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number.
pub fn asinh(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_asinh(value)?))
}

/// Excel-compatible `ATAN` function.
/// Calculates the arctangent (inverse tangent) of a number.
///
/// # Parameters
/// - `value`: The number for which to calculate the arctangent.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the arctangent in radians (between -π/2 and π/2).
///
/// # Errors
/// Returns an error if the value cannot be converted to a number.
pub fn atan(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_atan(value)?))
}

/// Excel-compatible `ATAN2` function.
/// Calculates the arctangent of the quotient of two numbers (x and y coordinates).
///
/// # Parameters
/// - `x`: The x-coordinate.
/// - `y`: The y-coordinate.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the angle in radians (between -π and π).
///
/// # Errors
/// Returns an error if either value cannot be converted to a number, or if both x and y are zero.
pub fn atan2(
    x: Value,
    y: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;
    let y = y.f64(value_format)?;
    Ok(Value::F64(codcel_atan2(x, y)?))
}

/// Excel-compatible `ATANH` function.
/// Calculates the inverse hyperbolic tangent of a number.
///
/// # Parameters
/// - `value`: The number for which to calculate the inverse hyperbolic tangent (must be between -1 and 1, exclusive).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the inverse hyperbolic tangent.
///
/// # Errors
/// Returns an error if the value is not between -1 and 1 (exclusive), or cannot be converted to a number.
pub fn atanh(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_atanh(value)?))
}

/// Excel-compatible `LN` function.
/// Calculates the natural logarithm (base e) of a number.
///
/// # Parameters
/// - `value`: The positive number for which to calculate the natural logarithm.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the natural logarithm.
///
/// # Errors
/// Returns an error if the value is zero or negative, or cannot be converted to a number (when strict_type_conversion is true).
pub fn ln(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_to_float(
        value,
        strict_type_conversion,
        value_format,
        codcel_ln,
        "LN: This must be a number.",
    )
}

/* TODO: Average does not entirely work like Excel's Average will need to be fixed.
/*
The AVERAGE function calculates the mean of numeric values, ignoring text, logical values, and empty cells.
AVERAGEA includes text and logical values, treating text as 0, TRUE as 1, and FALSE as 0, while still ignoring empty cells.
Use AVERAGE for purely numeric data and AVERAGEA when dealing with mixed data types.
*/
 */
/// Excel-compatible `CEILING` function.
/// Rounds a number up to the nearest multiple of significance.
///
/// # Parameters
/// - `number`: The number to round up.
/// - `significance`: The multiple to which to round.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the number rounded up to the nearest multiple of significance.
///
/// # Errors
/// Returns an error if values cannot be converted to numbers or if significance is zero.
pub fn ceiling(
    number: Value,
    significance: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number = number.f64(value_format)?;
    let significance = significance.f64(value_format)?;

    Ok(Value::F64(codcel_ceiling(number, significance)?))
}

/// Excel-compatible `ISO.CEILING` function.
/// Rounds a number up to the nearest multiple of significance, with ISO standard behavior.
///
/// # Parameters
/// - `values`: The number to round up.
/// - `significance`: The multiple to which to round (defaults to 1 if omitted).
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the number rounded up.
///
/// # Errors
/// Returns an error if values cannot be converted to numbers.
pub fn iso_ceiling(
    values: Value,
    significance: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_op_float_to_float(
        values,
        significance,
        strict_type_conversion,
        value_format,
        1.0,
        "ISO.CEILING",
        codcel_iso_ceiling,
    )
}

/// Excel-compatible `LOG` function.
/// Calculates the logarithm of a number to a specified base.
///
/// # Parameters
/// - `values`: The positive number for which to calculate the logarithm.
/// - `base`: The base of the logarithm (must be positive and not equal to 1). Defaults to 10 if not specified.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the logarithm of the value to the specified base.
///
/// # Errors
/// Returns an error if the value or base is invalid, or cannot be converted to a number (when strict_type_conversion is true).
pub fn log(
    values: Value,
    base: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_op_float_to_float(
        values,
        base,
        strict_type_conversion,
        value_format,
        10.0,
        "LOG",
        codcel_log,
    )
}

/// Excel-compatible `CEILING.MATH` function.
/// Rounds a number up to the nearest multiple of significance with optional mode.
///
/// # Parameters
/// - `number`: The number to round up.
/// - `significance`: The multiple to which to round (defaults to 1 if omitted).
/// - `mode`: If non-zero, rounds negative numbers toward zero; if zero, rounds away from zero.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the number rounded according to the specified mode.
///
/// # Errors
/// Returns an error if values cannot be converted to numbers.
pub fn ceiling_math(
    number: Value,
    significance: Value,
    mode: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number = number.f64(value_format)?;
    let significance = significance.option_f64(value_format)?;
    let mode = mode.option_i32(value_format)?;

    Ok(Value::F64(codcel_ceiling_math(number, significance, mode)?))
}

/// Excel-compatible `CEILING.PRECISE` function.
/// Rounds a number up to the nearest multiple of significance, regardless of sign.
///
/// # Parameters
/// - `number`: The number to round up.
/// - `significance`: The multiple to which to round (defaults to 1 if omitted).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the number rounded up.
///
/// # Errors
/// Returns an error if values cannot be converted to numbers.
pub fn ceiling_precise(
    number: Value,
    significance: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number = number.f64(value_format)?;
    let significance = significance.option_f64(value_format)?;

    Ok(Value::F64(codcel_ceiling_precise(number, significance)?))
}

// TODO: MIN SHOULD WORK WITH TEXT AND STRING
/// Excel-compatible `MIN` function.
/// Returns the smallest numeric value in the provided collection.
///
/// # Parameters
/// - `values`: A vector of values to search.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, ignores non-numeric values.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the minimum value. Returns maximum f64 if the collection is empty.
///
/// # Errors
/// Returns an error if conversion fails (when strict_type_conversion is true).
///
/// # Note
/// Contains a TODO about working with text and string like Excel's MIN.
pub fn min(
    values: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::F64(f64::MAX));
    }

    let result = values.iter().try_fold(
        f64::MAX,
        |acc, value| -> Result<f64, Box<dyn Error + Send + Sync>> {
            if value.is_array() {
                // Try to find the minimum value in the array and compare it with the accumulator
                let vec = value.vec_f64(value_format)?;
                let min_in_array = vec.iter().fold(f64::MAX, |acc, &val| acc.min(val));
                Ok(acc.min(min_in_array))
            } else if value.is_area() {
                // Excel MIN ignores non-numeric cells in ranges/arrays
                let area = value.area_of_value()?;
                let mut local_min = acc;
                for row in &area {
                    for cell in row {
                        if let Ok(val) = cell.f64(value_format) {
                            local_min = local_min.min(val);
                        }
                    }
                }
                Ok(local_min)
            } else {
                // Excel MIN ignores text and boolean values from cell references
                if value.is_string() || matches!(value, Value::Bool(_)) {
                    return Ok(acc);
                }
                // Try to compare the value with the accumulator, stop if error occurs and strict_type_conversion is true
                match value.f64(value_format) {
                    Ok(val) => Ok(acc.min(val)),
                    Err(_) => {
                        if strict_type_conversion {
                            Err("MIN: Input values are not numbers".into()) // Convert and propagate error if strict
                        } else {
                            Ok(acc) // Ignore error and continue with the accumulation if not strict
                        }
                    }
                }
            }
        },
    )?;

    Ok(Value::F64(result))
}

/// Excel-compatible `COS` function.
/// Calculates the cosine of an angle given in radians.
///
/// # Parameters
/// - `value`: The angle in radians.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the cosine of the angle (between -1 and 1).
///
/// # Errors
/// Returns an error if the value cannot be converted to a number.
pub fn cos(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_cos(value)?))
}

/// Excel-compatible `COSH` function.
/// Calculates the hyperbolic cosine of a number.
///
/// # Parameters
/// - `value`: The number for which to calculate the hyperbolic cosine.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the hyperbolic cosine (always >= 1).
///
/// # Errors
/// Returns an error if the value cannot be converted to a number.
pub fn cosh(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_cosh(value)?))
}

/// Excel-compatible `RADIANS` function.
/// Converts degrees to radians.
///
/// # Parameters
/// - `degrees`: The angle in degrees.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the angle in radians.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number.
pub fn radians(
    degrees: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = degrees.f64(value_format)?;
    Ok(Value::F64(codcel_radians(value)?))
}

/// Excel-compatible `DEGREES` function.
/// Converts radians to degrees.
///
/// # Parameters
/// - `radians`: The angle in radians.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the angle in degrees.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number.
pub fn degrees(
    radians: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = radians.f64(value_format)?;
    Ok(Value::F64(codcel_degrees(value)?))
}

/// Excel-compatible `RAND` function.
/// Returns a random number between 0 (inclusive) and 1 (exclusive).
///
/// # Returns
/// Returns a `Value::F64` containing a random number in the range [0, 1).
///
/// # Errors
/// This function does not return errors.
pub fn rand() -> Result<Value, Box<dyn Error + Send + Sync>> {
    Ok(Value::F64(codcel_rand()?))
}

/// Excel-compatible `EVEN` function.
/// Rounds a number up to the nearest even integer (away from zero).
///
/// # Parameters
/// - `value`: The number to round.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the nearest even integer.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number.
pub fn even(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    Ok(Value::F64(codcel_even(value)?))
}

/// Excel-compatible `TRUNC` function.
/// Truncates a number to a specified number of decimal places by removing fractional digits.
///
/// # Parameters
/// - `value`: The number to truncate.
/// - `decimals`: The number of decimal places to retain (defaults to 0 if omitted). Can be negative.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the truncated number (as I32 if decimals=0, otherwise F64).
///
/// # Errors
/// Returns an error if values cannot be converted to numbers.
pub fn trunc(
    value: Value,
    decimals: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    let decimals = (decimals.option_i32(value_format)?).unwrap_or_default();

    if decimals == 0 {
        Ok(Value::I32(value.trunc() as i32))
    } else {
        Ok(Value::F64(codcel_trunc(value, Some(decimals))?))
    }
}

/// Negates a numeric value (returns -value).
///
/// # Parameters
/// - `value`: The number to negate.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the negated value.
///
/// # Errors
/// Returns an error if the value cannot be converted to a number.
pub fn negative(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match value {
        Value::F64(value) => Ok(Value::F64(-value)),
        Value::I32(value) => Ok(Value::I32(-value)),
        Value::VecValue(vec) => {
            // Handle element-wise negation for arrays (e.g., --(c>2) pattern)
            let mut results = Vec::with_capacity(vec.len());
            for elem in vec {
                results.push(negative(elem, value_format)?);
            }
            Ok(Value::VecValue(results))
        }
        Value::AreaValue(rows) => {
            // Handle element-wise negation for 2D arrays
            let mut result_rows = Vec::with_capacity(rows.len());
            for row in rows {
                let mut result_row = Vec::with_capacity(row.len());
                for elem in row {
                    result_row.push(negative(elem, value_format)?);
                }
                result_rows.push(result_row);
            }
            Ok(Value::AreaValue(result_rows))
        }
        _ => {
            let value = value.f64(value_format)?;
            Ok(Value::F64(codcel_negative(value)?))
        }
    }
}

/// Excel-compatible `NOT` function.
/// Returns the logical NOT of a value.
///
/// # Parameters
/// - `value`: The value to negate logically.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::Bool` containing the logical negation (true becomes false, false becomes true).
///
/// # Errors
/// Returns an error if the value cannot be converted to a boolean.
pub fn not(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.bool(value_format)?;
    Ok(Value::Bool(codcel_not(value)?))
}

// LOGNORM.DIST
// TODO: THIS DOES NOT WORK
/// Excel-compatible `PERMUT` function.
/// Calculates the number of permutations (ways to arrange k items from n items, order matters).
///
/// # Parameters
/// - `n`: The total number of items.
/// - `k`: The number of items to arrange.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing P(n,k) = n! / (n-k)!.
///
/// # Errors
/// Returns an error if values are invalid (negative, k > n) or cannot be converted.
///
/// # Note
/// Contains a TODO indicating this function may not work correctly.
pub fn permut(
    n: Value,
    k: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_multi_to_int(
        vec![n, k],
        strict_type_conversion,
        value_format,
        "PERMUT",
        codcel_permut_vec,
    )
}

/// Excel-compatible `PERMUTATIONA` function.
/// Calculates the number of permutations with repetitions allowed (n^k).
///
/// # Parameters
/// - `n`: The total number of items.
/// - `k`: The number of positions to fill.
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing n^k (n to the power of k).
///
/// # Errors
/// Returns an error if values cannot be converted.
pub fn permutation_a(
    n: Value,
    k: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_multi_to_int(
        vec![n, k],
        strict_type_conversion,
        value_format,
        "PERMUTATIONA",
        codcel_permutation_a_vec,
    )
}

/// Excel-compatible `T.DIST.2T` function.
/// Calculates the two-tailed Student's t-distribution.
///
/// # Parameters
/// - `x`: The value at which to evaluate the distribution.
/// - `degrees_freedom`: The number of degrees of freedom (must be >= 1).
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the two-tailed probability.
///
/// # Errors
/// Returns an error if values are invalid or cannot be converted.
pub fn t_dist_2t(
    x: Value,
    degrees_freedom: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x, degrees_freedom],
        strict_type_conversion,
        value_format,
        "T.DIST.2T",
        codcel_t_dist_2t_vec,
    )
}

/// Excel-compatible `T.INV.2T` function.
/// Calculates the inverse of the two-tailed Student's t-distribution.
///
/// # Parameters
/// - `probability`: The probability (between 0 and 1).
/// - `degrees_freedom`: The number of degrees of freedom (must be >= 1).
/// - `strict_type_conversion`: If `true`, returns error for non-numeric values; if `false`, treats non-numeric as 0.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value` containing the t-value.
///
/// # Errors
/// Returns an error if probability is not in \[0,1\] or values cannot be converted.
pub fn t_inv_2t(
    probability: Value,
    degrees_freedom: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![probability, degrees_freedom],
        strict_type_conversion,
        value_format,
        "T.INV.2T",
        codcel_t_inv_2t_vec,
    )
}

// TODO: MOVE ENGINEERING FUNCTIONS
/// Excel-compatible `SUBTOTAL` function.
/// Calculates a subtotal using a specified function code.
///
/// # Parameters
/// - `function_code`: The function to use (1-11 or 101-111): AVERAGE=1, COUNT=2, COUNTA=3, MAX=4, MIN=5, PRODUCT=6, STDEV=7, STDEVP=8, SUM=9, VAR=10, VARP=11.
/// - `values`: A vector of values to aggregate.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the result of the specified function.
///
/// # Errors
/// Returns an error if function_code is invalid or conversion fails.
pub fn sub_total(
    function_code: Value,
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let function_code = function_code.i32(value_format)?;
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_sub_total(function_code, values)?))
}

/// Excel-compatible `GCD` function.
/// Calculates the greatest common divisor of a set of integers.
///
/// # Parameters
/// - `numbers`: A vector of integers.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing the GCD of all numbers.
///
/// # Errors
/// Returns an error if values cannot be converted to integers.
pub fn gcd(
    numbers: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let numbers = vec_value_to_vec_i32(numbers, value_format)?;
    Ok(Value::I32(codcel_gcd(numbers)?))
}

/// Excel-compatible `LCM` function.
/// Calculates the least common multiple of a set of integers.
///
/// # Parameters
/// - `numbers`: A vector of integers.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing the LCM of all numbers.
///
/// # Errors
/// Returns an error if values cannot be converted to integers.
pub fn lcm(
    numbers: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let numbers = vec_value_to_vec_f64(numbers, value_format)?;
    Ok(Value::I32(codcel_lcm(numbers)?))
}

/// Excel-compatible `AGGREGATE` function.
/// Calculates an aggregate using a specified function code and options.
///
/// # Parameters
/// - `function_code`: The function to use (1-19): similar to SUBTOTAL but with more functions.
/// - `options`: Options for handling errors and hidden values (0-7).
/// - `values`: A vector of values to aggregate.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the result of the specified function.
///
/// # Errors
/// Returns an error if function_code or options are invalid or conversion fails.
pub fn aggregate(
    function_code: Value,
    options: Value,
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let function_code = function_code.i32(value_format)?;
    let values = vec_value_to_vec_f64(values, value_format)?;
    let options = options.i32(value_format)?;
    Ok(Value::F64(codcel_aggregate(
        function_code,
        options,
        values,
    )?))
}

/// Excel-compatible `MULTINOMIAL` function.
/// Calculates the multinomial coefficient: (sum of all numbers)! / (product of factorials).
///
/// # Parameters
/// - `numbers`: A vector of non-negative integers.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing (n1+n2+...+nk)! / (n1! × n2! × ... × nk!).
///
/// # Errors
/// Returns an error if values are negative or cannot be converted to integers.
pub fn multinomial(
    numbers: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let numbers = vec_value_to_vec_i32(numbers, value_format)?;
    Ok(Value::I32(codcel_multinomial(numbers)?))
}

/// Excel-compatible `RANDARRAY` function.
/// Returns an array of random numbers.
///
/// # Parameters
/// - `rows`: The number of rows in the array.
/// - `columns`: The number of columns in the array.
/// - `min`: The minimum value (defaults to 0).
/// - `max`: The maximum value (defaults to 1).
/// - `whole_number`: If TRUE, returns integers; if FALSE, returns decimals.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::AreaValue` containing a 2D array of random numbers.
///
/// # Errors
/// Returns an error if values cannot be converted or dimensions are invalid.
pub fn rand_array(
    rows: Value,
    columns: Value,
    min: Value,
    max: Value,
    whole_number: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let rows = rows.i32(value_format)?;
    let columns = columns.i32(value_format)?;
    let min = min.option_f64(value_format)?;
    let max = max.option_f64(value_format)?;
    let whole_number = whole_number.option_bool(value_format)?;

    let result = codcel_rand_array(rows, columns, min, max, whole_number)?;

    Ok(area_f64(result))
}

/// Excel-compatible `RANDBETWEEN` function.
/// Returns a random integer between two specified values (inclusive).
///
/// # Parameters
/// - `min`: The smallest integer that can be returned.
/// - `max`: The largest integer that can be returned.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::I32` containing a random integer in the range [min, max].
///
/// # Errors
/// Returns an error if values cannot be converted to integers or if min > max.
pub fn rand_between(
    min: Value,
    max: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let min = min.i32(value_format)?;
    let max = max.i32(value_format)?;

    Ok(Value::I32(codcel_rand_between(min, max)?))
}

/// Excel-compatible `SEQUENCE` function.
/// Generates a sequence of numbers in an array.
///
/// # Parameters
/// - `rows`: The number of rows in the array.
/// - `columns`: The number of columns in the array (defaults to 1).
/// - `start`: The first number in the sequence (defaults to 1).
/// - `step`: The increment between each number (defaults to 1).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::AreaValue` containing a 2D array with the sequence.
///
/// # Errors
/// Returns an error if values cannot be converted or dimensions are invalid.
pub fn sequence(
    rows: Value,
    columns: Value,
    start: Value,
    step: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let rows = rows.i32(value_format)?;
    let columns = columns.option_i32(value_format)?;
    let start = start.option_f64(value_format)?;
    let step = step.option_f64(value_format)?;

    let result = codcel_sequence(rows, columns, start, step)?;

    Ok(area_f64(result))
}

/// Excel-compatible `SERIESSUM` function.
/// Calculates the sum of a power series.
///
/// # Parameters
/// - `x`: The input value to the power series.
/// - `n`: The initial power to which x is raised.
/// - `m`: The step by which to increase n for each term.
/// - `coefficients`: A vector of coefficients for each term.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the sum: coefficient\[0\]*x^n + coefficient\[1\]*x^(n+m) + ...
///
/// # Errors
/// Returns an error if values cannot be converted to numbers.
pub fn series_sum(
    x: Value,
    n: Value,
    m: Value,
    coefficients: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;
    let n = n.f64(value_format)?;
    let m = m.f64(value_format)?;
    let coefficients = vec_value_to_vec_f64(coefficients, value_format)?;
    Ok(Value::F64(codcel_series_sum(x, n, m, coefficients)?))
}

/// Excel-compatible `SUMPRODUCT` function.
/// Multiplies corresponding components in arrays and returns the sum of those products.
///
/// # Parameters
/// - `values`: Arrays or ranges to multiply and sum.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the sum of products.
///
/// # Errors
/// Returns an error if arrays have different dimensions or conversion fails.
pub fn sum_product(
    values: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = values.vec_value()?;

    let mut input: Vec<Vec<f64>> = Vec::new();

    for value in values {
        let value = value.to_flatterned_vec_f64(value_format)?;
        input.push(value);
    }

    Ok(Value::F64(codcel_sum_product(input)?))
}

// TODO: strict type conversion on all Vec<Value>
/// Excel-compatible `SUMSQ` function.
/// Calculates the sum of the squares of all numeric values.
///
/// # Parameters
/// - `values`: A vector of values to square and sum.
/// - `_strict_type_conversion`: Type conversion flag (currently unused).
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the sum of squares.
///
/// # Errors
/// Returns an error if conversion fails.
pub fn sum_sq(
    values: Vec<Value>,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_sum_sq(values)?))
}

/// Excel-compatible `SUMX2MY2` function.
/// Calculates the sum of the difference of squares: Σ(x² - y²).
///
/// # Parameters
/// - `x2`: The first array of values.
/// - `y2`: The second array of values.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing Σ(x² - y²).
///
/// # Errors
/// Returns an error if arrays have different lengths or conversion fails.
pub fn sum_x2my2(
    x2: Value,
    y2: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x2 = flatten_value_to_vec_f64(x2, value_format)?;
    let y2 = flatten_value_to_vec_f64(y2, value_format)?;
    Ok(Value::F64(codcel_sum_x2my2(x2, y2)?))
}

/// Excel-compatible `PERCENTOF` function.
/// Calculates what fraction of a whole a subset represents: SUM(subset) / SUM(all).
///
/// # Parameters
/// - `subset`: The values making up the part. May be a scalar or a range.
/// - `all`: The values making up the whole. May be a scalar or a range.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing the fraction as a decimal (0.2, not 20%).
///
/// # Errors
/// Returns `#DIV/0!` when the whole sums to zero, or an error if conversion fails.
pub fn percentof(
    subset: Value,
    all: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let subset = flatten_value_to_vec_f64(subset, value_format)?;
    let all = flatten_value_to_vec_f64(all, value_format)?;
    Ok(Value::F64(codcel_percentof(subset, all)?))
}

/// Excel-compatible `SUMX2PY2` function.
/// Calculates the sum of the sum of squares: Σ(x² + y²).
///
/// # Parameters
/// - `x2`: The first array of values.
/// - `y2`: The second array of values.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing Σ(x² + y²).
///
/// # Errors
/// Returns an error if arrays have different lengths or conversion fails.
pub fn sum_x2py2(
    x2: Value,
    y2: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x2 = flatten_value_to_vec_f64(x2, value_format)?;
    let y2 = flatten_value_to_vec_f64(y2, value_format)?;
    Ok(Value::F64(codcel_sum_x2py2(x2, y2)?))
}

/// Excel-compatible `SUMXMY2` function.
/// Calculates the sum of squares of differences: Σ(x - y)².
///
/// # Parameters
/// - `x`: The first array of values.
/// - `y2`: The second array of values.
/// - `value_format`: Format settings for locale-specific number parsing.
///
/// # Returns
/// Returns a `Value::F64` containing Σ(x - y)².
///
/// # Errors
/// Returns an error if arrays have different lengths or conversion fails.
pub fn sum_xmy2(
    x: Value,
    y2: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = flatten_value_to_vec_f64(x, value_format)?;
    let y2 = flatten_value_to_vec_f64(y2, value_format)?;
    Ok(Value::F64(codcel_sum_xmy2(x, y2)?))
}

// TODO
/* pub fn h_stack(value: Value, _value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = value.area_of_value()?;
    let result = codcel_h_stack(values)?;
    Ok(Value::VecValue(result))
}

/// Excel-compatible `VSTACK` function (incomplete implementation).
/// Intended to vertically stack arrays to create a combined array.
///
/// # Parameters
/// - `inputs`: A collection of values to stack vertically.
/// - `_value_format`: Format settings (currently unused).
///
/// # Returns
/// Currently returns `Value::None` as the implementation is incomplete.
///
/// # Errors
/// May return an error if value conversion fails.
///
/// # Note
/// This function contains debug output and is not yet fully implemented.
/// The commented-out code suggests it should return a `VecValue`.
pub fn v_stack(inputs: Value, _value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    println!("inputs {:#?}", &inputs);

    let inputs = inputs.vec_value()?;

    let mut values: Vec<Vec<Vec<Value>>> = vec![];

    for value in inputs {
        let area = value.area_of_value()?;
        values.push(area)
    }

    println!("values {:#?}", &values);



    Ok(Value::None)

    /*let values = value.area_of_value()?;
    let result = codcel_v_stack(values)?;
    Ok(Value::VecValue(result))*/
}*/

/// Wraps a computed f64 result into `Value::F64`, returning an error if the
/// value is NaN or infinite (arithmetic overflow / invalid operation).
#[inline]
pub fn codcel_maths(result: f64) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if result.is_nan() || result.is_infinite() {
        Err("Arithmetic overflow or invalid operation".into())
    } else {
        Ok(Value::F64(result))
    }
}

#[cfg(test)]
mod tests {
    // Literals such as 3.14159 and 1.41421 are Excel-visible values under test,
    // not stand-ins for std::f64::consts.
    #![allow(clippy::approx_constant)]
    use super::*;
    use crate::value::{
        area_f64 as value_area_f64, bool as value_bool, f64 as value_f64, i32 as value_i32,
        string as value_string, vec_f64 as value_vec_f64,
    };
    use crate::value_format::ValueFormat;
    // Import moved compatibility functions
    use crate::compatibility_base::{st_dev, st_dev_p, var};
    // Import moved engineering functions
    use crate::engineering_base::{
        bin_2_dec, complex, dec_2_bin, dec_2_hex, dec_2_oct, delta, erf, erfc, hex_2_dec, im_abs,
        oct_2_dec,
    };
    // Import moved statistical functions
    use crate::statistical_base::*;

    fn create_value_format() -> ValueFormat {
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
    fn test_add() {
        let value_format = create_value_format();

        // Test adding two numbers
        let result = add(value_f64(5.0), value_f64(3.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 8.0);

        // Test adding a number and a string
        let result = add(value_f64(5.0), value_string("3".to_string()), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 8.0);

        // Test adding two strings that can be converted to numbers
        let result = add(
            value_string("5".to_string()),
            value_string("3".to_string()),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 8.0);
    }

    #[test]
    fn test_subtract() {
        let value_format = create_value_format();

        // Test subtracting two numbers
        let result = subtract(value_f64(5.0), value_f64(3.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);

        // Test subtracting a string from a number
        let result =
            subtract(value_f64(5.0), value_string("3".to_string()), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);

        // Test subtracting two strings that can be converted to numbers
        let result = subtract(
            value_string("5".to_string()),
            value_string("3".to_string()),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);
    }

    #[test]
    fn test_multiply() {
        let value_format = create_value_format();

        // Test multiplying two numbers
        let result = multiply(value_f64(5.0), value_f64(3.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 15.0);

        // Test multiplying a number and a string
        let result =
            multiply(value_f64(5.0), value_string("3".to_string()), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 15.0);

        // Test multiplying two strings that can be converted to numbers
        let result = multiply(
            value_string("5".to_string()),
            value_string("3".to_string()),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 15.0);
    }

    #[test]
    fn test_divide() {
        let value_format = create_value_format();

        // Test dividing two numbers
        let result = divide(value_f64(15.0), value_f64(3.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.0);

        // Test dividing a number by a string
        let result = divide(
            value_f64(15.0),
            value_string("3".to_string()),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.0);

        // Test dividing two strings that can be converted to numbers
        let result = divide(
            value_string("15".to_string()),
            value_string("3".to_string()),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.0);
    }

    #[test]
    fn test_pi() {
        let value_format = create_value_format();

        let result = pi(&value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), std::f64::consts::PI);
    }

    #[test]
    fn test_sin() {
        let value_format = create_value_format();

        // Test sin of 0
        let result = sin(value_f64(0.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);

        // Test sin of PI/2
        let result = sin(value_f64(std::f64::consts::PI / 2.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cos() {
        let value_format = create_value_format();

        // Test cos of 0
        let result = cos(value_f64(0.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 1.0);

        // Test cos of PI
        let result = cos(value_f64(std::f64::consts::PI), &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() + 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tan() {
        let value_format = create_value_format();

        // Test tan of 0
        let result = tan(value_f64(0.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);

        // Test tan of PI/4
        let result = tan(value_f64(std::f64::consts::PI / 4.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt() {
        let value_format = create_value_format();

        // Test sqrt of 4
        let result = sqrt(value_f64(4.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);

        // Test sqrt of 9
        let result = sqrt(value_f64(9.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 3.0);
    }

    #[test]
    fn test_power() {
        let value_format = create_value_format();

        // Test 2^3
        let result = power(value_f64(2.0), value_f64(3.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 8.0);

        // Test 3^2
        let result = power(value_f64(3.0), value_f64(2.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 9.0);
    }

    #[test]
    fn test_abs() {
        let value_format = create_value_format();

        // Test abs of positive number
        let result = abs(value_f64(5.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.0);

        // Test abs of negative number
        let result = abs(value_f64(-5.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.0);
    }

    #[test]
    fn test_round() {
        let value_format = create_value_format();

        // Test rounding to 0 decimal places
        let result = round(value_f64(5.5), value_i32(0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 6.0);

        // Test rounding to 1 decimal place
        let result = round(value_f64(5.55), value_i32(1), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.6);
    }

    #[test]
    fn test_floor() {
        let value_format = create_value_format();

        // Test floor with positive number
        let result = floor(value_f64(5.7), value_f64(1.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.0);

        // Test floor with negative number
        let result = floor(value_f64(-5.7), value_f64(1.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), -6.0);
    }

    #[test]
    fn test_ceiling() {
        let value_format = create_value_format();

        // Test ceiling with positive number
        let result = ceiling(value_f64(5.2), value_f64(1.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 6.0);

        // Test ceiling with negative number
        let result = ceiling(value_f64(-5.2), value_f64(1.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), -5.0);
    }

    #[test]
    fn test_sum() {
        let value_format = create_value_format();

        // Test sum of numbers
        let result = sum(
            vec![value_f64(1.0), value_f64(2.0), value_f64(3.0)],
            true,
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 6.0);

        // Test sum of mixed types
        let result = sum(
            vec![value_f64(1.0), value_string("2".to_string()), value_i32(3)],
            true,
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 6.0);
    }

    #[test]
    fn test_average() {
        let value_format = create_value_format();

        // Test average of numbers
        let result = average(
            vec![value_f64(1.0), value_f64(2.0), value_f64(3.0)],
            true,
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);

        // Test average of mixed types
        let result = average(
            vec![value_f64(1.0), value_string("2".to_string()), value_i32(3)],
            true,
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);
    }

    #[test]
    fn test_max() {
        let value_format = create_value_format();

        // Test max of numbers
        let result = max(
            vec![value_f64(1.0), value_f64(3.0), value_f64(2.0)],
            true,
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 3.0);

        // Test max ignores string values (Excel behavior for cell references)
        let result = max(
            vec![value_f64(1.0), value_string("3".to_string()), value_i32(2)],
            true,
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);
    }

    #[test]
    fn test_min() {
        let value_format = create_value_format();

        // Test min of numbers
        let result = min(
            vec![value_f64(2.0), value_f64(1.0), value_f64(3.0)],
            true,
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 1.0);

        // Test min ignores string values (Excel behavior for cell references)
        let result = min(
            vec![value_f64(2.0), value_string("1".to_string()), value_i32(3)],
            true,
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);
    }

    #[test]
    fn test_count() {
        let value_format = create_value_format();

        // Test count of numbers
        let result = count(
            vec![value_f64(1.0), value_f64(2.0), value_f64(3.0)],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 3);

        // Test count with non-numeric values
        let result = count(
            vec![
                value_f64(1.0),
                value_string("text".to_string()),
                value_bool(true),
            ],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 1);
    }

    #[test]
    fn test_product() {
        let value_format = create_value_format();

        // Test product of numbers
        let result = product(
            vec![value_f64(2.0), value_f64(3.0), value_f64(4.0)],
            true,
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 24.0);

        // Test product of mixed types
        let result = product(
            vec![value_f64(2.0), value_string("3".to_string()), value_i32(4)],
            true,
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 24.0);
    }

    #[test]
    fn test_fact() {
        let value_format = create_value_format();

        // Test factorial of 5
        let result = fact(value_i32(5), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 120.0);

        // Test factorial of 0
        let result = fact(value_i32(0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 1.0);
    }

    #[test]
    fn test_ln() {
        let value_format = create_value_format();

        // Test natural log of e
        let result = ln(value_f64(std::f64::consts::E), true, &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 1.0).abs() < 1e-10);

        // Test natural log of 1
        let result = ln(value_f64(1.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);
    }

    #[test]
    fn test_log10() {
        let value_format = create_value_format();

        // Test log10 of 100
        let result = log10(value_f64(100.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);

        // Test log10 of 1
        let result = log10(value_f64(1.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);
    }

    #[test]
    fn test_log() {
        let value_format = create_value_format();

        // Test log base 2 of 8
        let result = log(value_f64(8.0), value_f64(2.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 3.0);

        // Test log base 10 of 100
        let result = log(value_f64(100.0), value_f64(10.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);
    }

    #[test]
    fn test_exp() {
        let value_format = create_value_format();

        // Test e^1
        let result = exp(value_f64(1.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - std::f64::consts::E).abs() < 1e-10);

        // Test e^0
        let result = exp(value_f64(0.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 1.0);
    }

    #[test]
    fn test_sign() {
        let value_format = create_value_format();

        // Test sign of positive number
        let result = sign(value_f64(5.7), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 1);

        // Test sign of negative number
        let result = sign(value_f64(-5.7), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), -1);

        // Test sign of zero
        let result = sign(value_f64(0.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 0);
    }

    #[test]
    fn test_rand() {
        // Test random number generation
        let result = rand().unwrap();
        println!("{result:?}");
        let value = result.f64(&create_value_format()).unwrap();
        assert!((0.0..1.0).contains(&value));
    }

    #[test]
    fn test_rand_between() {
        let value_format = create_value_format();

        // Test random number between 1 and 10
        let result = rand_between(value_i32(1), value_i32(10), &value_format).unwrap();
        println!("{result:?}");
        let value = result.i32(&value_format).unwrap();
        assert!((1..=10).contains(&value));
    }

    #[test]
    fn test_int() {
        let value_format = create_value_format();

        // Test int of positive number
        let result = int(value_f64(5.7), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 5);

        // Test int of negative number
        let result = int(value_f64(-5.7), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), -6);
    }

    #[test]
    fn test_trunc() {
        let value_format = create_value_format();

        // Test truncating positive number
        let result = trunc(value_f64(5.7), value_i32(0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.0);

        // Test truncating negative number
        let result = trunc(value_f64(-5.7), value_i32(0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), -5.0);

        // Test truncating to 1 decimal place
        let result = trunc(value_f64(5.78), value_i32(1), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.7);
    }

    #[test]
    fn test_even() {
        let value_format = create_value_format();

        // Test even of 5
        let result = even(value_f64(5.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 6.0);

        // Test even of 6
        let result = even(value_f64(6.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 6.0);

        // Test even of -5
        let result = even(value_f64(-5.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), -6.0);
    }

    #[test]
    fn test_odd() {
        let value_format = create_value_format();

        // Test odd of 5
        let result = odd(value_f64(5.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.0);

        // Test odd of 6
        let result = odd(value_f64(6.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 7.0);

        // Test odd of -5
        let result = odd(value_f64(-5.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), -5.0);
    }

    #[test]
    fn test_gcd() {
        let value_format = create_value_format();

        // Test GCD of 12 and 18
        let result = gcd(vec![value_i32(12), value_i32(18)], &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 6);

        // Test GCD of multiple numbers
        let result = gcd(
            vec![value_i32(12), value_i32(18), value_i32(24)],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 6);
    }

    #[test]
    fn test_lcm() {
        let value_format = create_value_format();

        // Test LCM of 4 and 6
        let result = lcm(vec![value_i32(4), value_i32(6)], &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 12);

        // Test LCM of multiple numbers
        let result = lcm(
            vec![value_i32(2), value_i32(3), value_i32(4)],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 12);
    }

    #[test]
    fn test_asin() {
        let value_format = create_value_format();

        // Test asin of 0
        let result = asin(value_f64(0.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);

        // Test asin of 1
        let result = asin(value_f64(1.0), &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - std::f64::consts::PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_acos() {
        let value_format = create_value_format();

        // Test acos of 1
        let result = acos(value_f64(1.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);

        // Test acos of 0
        let result = acos(value_f64(0.0), &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - std::f64::consts::PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_atan() {
        let value_format = create_value_format();

        // Test atan of 0
        let result = atan(value_f64(0.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);

        // Test atan of 1
        let result = atan(value_f64(1.0), &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - std::f64::consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_atan2() {
        let value_format = create_value_format();

        // Test atan2(1, 1)
        let result = atan2(value_f64(1.0), value_f64(1.0), &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - std::f64::consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_sinh() {
        let value_format = create_value_format();

        // Test sinh of 0
        let result = sinh(value_f64(0.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);

        // Test sinh of 1
        let result = sinh(value_f64(1.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert!(
            (result.f64(&value_format).unwrap()
                - (std::f64::consts::E - 1.0 / std::f64::consts::E) / 2.0)
                .abs()
                < 1e-10
        );
    }

    #[test]
    fn test_cosh() {
        let value_format = create_value_format();

        // Test cosh of 0
        let result = cosh(value_f64(0.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 1.0);

        // Test cosh of 1
        let result = cosh(value_f64(1.0), &value_format).unwrap();
        println!("{result:?}");
        assert!(
            (result.f64(&value_format).unwrap()
                - (std::f64::consts::E + 1.0 / std::f64::consts::E) / 2.0)
                .abs()
                < 1e-10
        );
    }

    #[test]
    fn test_tanh() {
        let value_format = create_value_format();

        // Test tanh of 0
        let result = tanh(value_f64(0.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);

        // Test tanh of a large number (should approach 1)
        let result = tanh(value_f64(10.0), true, &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_degrees_radians() {
        let value_format = create_value_format();

        // Test converting radians to degrees
        let result = degrees(value_f64(std::f64::consts::PI), &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 180.0).abs() < 1e-10);

        // Test converting degrees to radians
        let result = radians(value_f64(180.0), &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_complex() {
        let value_format = create_value_format();

        // Test creating a complex number
        let result = complex(
            value_f64(3.0),
            value_f64(4.0),
            value_string("i".to_string()),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.string(&value_format).unwrap(), "3+4i");
    }

    #[test]
    fn test_im_abs() {
        let value_format = create_value_format();

        // Test absolute value of a complex number
        let result = im_abs(value_string("3+4i".to_string()), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.0);
    }

    #[test]
    fn test_dec_2_bin() {
        let value_format = create_value_format();

        // Test converting decimal to binary
        let result = dec_2_bin(value_i32(10), value_i32(8), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.string(&value_format).unwrap(), "00001010");
    }

    #[test]
    fn test_dec_2_hex() {
        let value_format = create_value_format();

        // Test converting decimal to hexadecimal
        let result = dec_2_hex(value_i32(255), value_i32(4), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.string(&value_format).unwrap(), "00FF");
    }

    #[test]
    fn test_dec_2_oct() {
        let value_format = create_value_format();

        // Test converting decimal to octal
        let result = dec_2_oct(value_i32(8), value_i32(3), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.string(&value_format).unwrap(), "010");
    }

    #[test]
    fn test_hex_2_dec() {
        let value_format = create_value_format();

        // Test converting hexadecimal to decimal
        let result = hex_2_dec(value_string("FF".to_string()), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 255);
    }

    #[test]
    fn test_bin_2_dec() {
        let value_format = create_value_format();

        // Test converting binary to decimal
        let result = bin_2_dec(value_string("1010".to_string()), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 10);
    }

    #[test]
    fn test_oct_2_dec() {
        let value_format = create_value_format();

        // Test converting octal to decimal
        let result = oct_2_dec(value_string("10".to_string()), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 8);
    }

    #[test]
    fn test_delta() {
        let value_format = create_value_format();

        // Test delta function with equal numbers
        let result = delta(value_f64(5.0), value_f64(5.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 1);

        // Test delta function with different numbers
        let result = delta(value_f64(5.0), value_f64(6.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 0);
    }

    #[test]
    fn test_erf() {
        let value_format = create_value_format();

        // Test error function at 0
        let result = erf(value_f64(0.0), value_f64(0.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);
    }

    #[test]
    fn test_erfc() {
        let value_format = create_value_format();

        // Test complementary error function at 0
        let result = erfc(value_f64(0.0), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 1.0);
    }

    #[test]
    fn test_fact_double() {
        let value_format = create_value_format();

        // Test double factorial of 5
        let result = fact_double(value_i32(5), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 15.0); // 5*3*1 = 15

        // Test double factorial of 6
        let result = fact_double(value_i32(6), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 48.0); // 6*4*2 = 48
    }

    #[test]
    fn test_base() {
        let value_format = create_value_format();

        // Test converting to base 2
        let result = base(value_i32(10), value_i32(2), value_i32(8), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.string(&value_format).unwrap(), "00001010");

        // Test converting to base 16
        let result = base(value_i32(255), value_i32(16), value_i32(4), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.string(&value_format).unwrap(), "00FF");
    }

    #[test]
    fn test_roman() {
        let value_format = create_value_format();

        // Test converting to Roman numerals
        let result = roman(value_i32(4), value_i32(0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.string(&value_format).unwrap(), "IV");

        // Test converting to Roman numerals
        let result = roman(value_i32(9), value_i32(0), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.string(&value_format).unwrap(), "IX");
    }

    #[test]
    fn test_arabic() {
        let value_format = create_value_format();

        // Test converting from Roman numerals
        let result = arabic(value_string("IV".to_string()), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 4);

        // Test converting from Roman numerals
        let result = arabic(value_string("IX".to_string()), true, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 9);
    }

    #[test]
    fn test_median() {
        let value_format = create_value_format();

        // Test median of odd number of values
        let result = median(
            vec![value_f64(1.0), value_f64(3.0), value_f64(2.0)],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);

        // Test median of even number of values
        let result = median(
            vec![
                value_f64(1.0),
                value_f64(3.0),
                value_f64(2.0),
                value_f64(4.0),
            ],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.5);
    }

    #[test]
    fn test_var() {
        let value_format = create_value_format();

        // Test variance of a sample
        let result = var(
            vec![
                value_f64(1.0),
                value_f64(2.0),
                value_f64(3.0),
                value_f64(4.0),
                value_f64(5.0),
            ],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 2.5).abs() < 1e-10);
    }

    #[test]
    fn test_var_p() {
        let value_format = create_value_format();

        // Test variance of a population
        let result = var_p(
            vec![
                value_f64(1.0),
                value_f64(2.0),
                value_f64(3.0),
                value_f64(4.0),
                value_f64(5.0),
            ],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_st_dev() {
        let value_format = create_value_format();

        // Test standard deviation of a sample
        let result = st_dev(
            vec![
                value_f64(1.0),
                value_f64(2.0),
                value_f64(3.0),
                value_f64(4.0),
                value_f64(5.0),
            ],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 1.5811388300841898).abs() < 1e-10);
    }

    #[test]
    fn test_st_dev_p() {
        let value_format = create_value_format();

        // Test standard deviation of a population
        let result = st_dev_p(
            vec![
                value_f64(1.0),
                value_f64(2.0),
                value_f64(3.0),
                value_f64(4.0),
                value_f64(5.0),
            ],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 1.4142135623730951).abs() < 1e-10);
    }

    #[test]
    fn test_geo_mean() {
        let value_format = create_value_format();

        // Test geometric mean
        let result = geo_mean(vec![value_f64(2.0), value_f64(8.0)], &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_har_mean() {
        let value_format = create_value_format();

        // Test harmonic mean
        let result = har_mean(
            vec![value_f64(1.0), value_f64(2.0), value_f64(4.0)],
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 1.7142857142857142).abs() < 1e-10);
    }

    #[test]
    fn test_trim_mean() {
        let value_format = create_value_format();

        // Test trimmed mean
        let result = trim_mean(
            value_vec_f64(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]),
            value_f64(0.2),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 5.5).abs() < 1e-10);
    }

    /* TODO Check this
    #[test]
    fn test_rank_avg() {
        let value_format = create_value_format();

        // Test rank average
        let result = rank_avg(value_f64(3.5), value_vec_f64(vec![1.0, 3.5, 5.0, 7.0]), value_bool(false), &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 2.0).abs() < 1e-10);
    }*/

    /* TODO CHECK THIS
    #[test]
    fn test_rank_eq() {
        let value_format = create_value_format();

        // Test rank equal
        let result = rank_eq(value_f64(3.5), value_vec_f64(vec![1.0, 3.5, 5.0, 7.0]), value_bool(false), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.i32(&value_format).unwrap(), 2);
    }*/

    #[test]
    fn test_large() {
        let value_format = create_value_format();

        // Test finding the 2nd largest value
        let result = large(
            value_vec_f64(vec![1.0, 3.0, 5.0, 7.0, 9.0]),
            value_i32(2),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 7.0);
    }

    #[test]
    fn test_small() {
        let value_format = create_value_format();

        // Test finding the 2nd smallest value
        let result = small(
            value_vec_f64(vec![1.0, 3.0, 5.0, 7.0, 9.0]),
            value_i32(2),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 3.0);
    }

    /* TODO CHECK THIS
    #[test]
    fn test_quartile_inc() {
        let value_format = create_value_format();

        // Test first quartile (inclusive)
        let result = quartile_inc(value_vec_f64(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]), value_i32(1), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);


        // Test third quartile (inclusive)
        let result = quartile_inc(value_vec_f64(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]), value_i32(3), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 6.0);

    }*/

    /* TODO Check this
    #[test]
    fn test_quartile_exc() {
        let value_format = create_value_format();

        // Test first quartile (exclusive)
        let result = quartile_exc(value_vec_f64(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]), value_i32(1), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 3.0);

        // Test third quartile (exclusive)
        let result = quartile_exc(value_vec_f64(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]), value_i32(3), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 7.0);
    }*/

    /* TODO CHECK THIS
    #[test]
    fn test_percentile_inc() {
        let value_format = create_value_format();

        // Test 75th percentile (inclusive)
        let result = percentile_inc(value_vec_f64(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]), value_f64(0.75), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 6.0);
    }*/

    /* TODO Check this
    #[test]
    fn test_percentile_exc() {
        let value_format = create_value_format();

        // Test 25th percentile (exclusive)
        let result = percentile_exc(value_vec_f64(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]), value_f64(0.25), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 3.0);

        // Test 75th percentile (exclusive)
        let result = percentile_exc(value_vec_f64(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]), value_f64(0.75), &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 7.0);
    }*/

    #[test]
    fn test_frequency() {
        let value_format = create_value_format();

        // Test frequency distribution
        let result = frequency(
            value_vec_f64(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]),
            value_vec_f64(vec![5.0, 10.0]),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        // Should return [5, 5] (5 values <= 5, 5 values <= 10 but > 5)
        let vec_result = result.vec_i32(&value_format).unwrap();
        assert_eq!(vec_result, vec![5, 5, 0]);
    }

    #[test]
    fn test_forecast() {
        let value_format = create_value_format();

        // Test linear forecast
        let result = forecast(
            value_f64(5.0),
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 5.0);
    }

    #[test]
    fn test_slope() {
        let value_format = create_value_format();

        // Test slope calculation
        let result = slope(
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 1.0);
    }

    #[test]
    fn test_intercept() {
        let value_format = create_value_format();

        // Test intercept calculation
        let result = intercept(
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), 0.0);
    }

    #[test]
    fn test_rsq() {
        let value_format = create_value_format();

        // Test R-squared calculation for perfect correlation
        let result = rsq(
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_correl() {
        let value_format = create_value_format();

        // Test correlation calculation for perfect correlation
        let result = correl(
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_covariance_p() {
        let value_format = create_value_format();

        // Test population covariance
        let result = covariance_p(
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 0.6666666666666666).abs() < 1e-10);
    }

    #[test]
    fn test_covariance_s() {
        let value_format = create_value_format();

        // Test sample covariance
        let result = covariance_s(
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            value_vec_f64(vec![1.0, 2.0, 3.0]),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap() - 1.0).abs() < 1e-10);
    }

    /* TODO CHECK THIS
    #[test]
    fn test_skew() {
        let value_format = create_value_format();

        // Test skewness of a symmetric distribution
        let result = skew(vec![value_f64(1.0), value_f64(2.0), value_f64(3.0), value_f64(2.0), value_f64(1.0)], &value_format).unwrap();
        println!("{result:?}");
        assert!((result.f64(&value_format).unwrap()).abs() < 1e-10);
    }*/

    /* TODO: FIX THIS #[test]
    fn test_kurt() {
        let value_format = create_value_format();

        // Test kurtosis
        let result = kurt(vec![value_f64(1.0), value_f64(2.0), value_f64(3.0), value_f64(4.0), value_f64(5.0)], &value_format).unwrap();
        println!("{result:?}");
        // For a uniform distribution, kurtosis should be close to -1.2
        assert!((result.f64(&value_format).unwrap() + 1.2).abs() < 0.5);
    }*/

    #[test]
    fn test_rand_array() {
        let value_format = create_value_format();

        // Test generating a random array
        let result = rand_array(
            value_i32(2),
            value_i32(3),
            value_f64(1.0),
            value_f64(10.0),
            value_bool(true),
            &value_format,
        )
        .unwrap();
        println!("{result:?}");

        // Check dimensions
        let area = result.area_f64(&value_format).unwrap();
        assert_eq!(area.len(), 2);
        assert_eq!(area[0].len(), 3);

        // Check values are within range
        for row in area {
            for val in row {
                assert!((1.0..=10.0).contains(&val));
                assert_eq!(val.floor(), val); // Check it's a whole number
            }
        }
    }

    #[test]
    fn test_m_determ() {
        let value_format = create_value_format();

        // Test matrix determinant
        let matrix = value_area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let result = m_determ(matrix, &value_format).unwrap();
        println!("{result:?}");
        assert_eq!(result.f64(&value_format).unwrap(), -2.0); // 1*4 - 2*3 = -2
    }

    #[test]
    fn test_m_inverse() {
        let value_format = create_value_format();

        // Test matrix inverse
        let matrix = value_area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let result = m_inverse(matrix, &value_format).unwrap();
        println!("{result:?}");

        // Check a value in the inverse matrix
        let area = result.area_f64(&value_format).unwrap();
        assert!((area[0][0] + 2.0).abs() < 1e-10); // Should be -2.0
    }

    #[test]
    fn test_m_mult() {
        let value_format = create_value_format();

        // Test matrix multiplication
        let matrix_a = value_area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let matrix_b = value_area_f64(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

        let result = m_mult(matrix_a, matrix_b, &value_format).unwrap();
        println!("{result:?}");

        // Check values in the result matrix
        let area = result.area_f64(&value_format).unwrap();
        assert_eq!(area[0][0], 19.0); // 1*5 + 2*7 = 19
        assert_eq!(area[0][1], 22.0); // 1*6 + 2*8 = 22
        assert_eq!(area[1][0], 43.0); // 3*5 + 4*7 = 43
        assert_eq!(area[1][1], 50.0); // 3*6 + 4*8 = 50
    }

    #[test]
    fn test_counta_counts_empty_strings() {
        let value_format = create_value_format();

        // In Excel, COUNTA counts empty strings — they are values, not blank cells
        // COUNTA("","") = 2
        let result = counta(
            vec![value_string("".to_string()), value_string("".to_string())],
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);

        // Mix of empty strings and other values — all are counted
        // COUNTA("","",55,"gamma",TRUE,0) = 6
        let result = counta(
            vec![
                value_string("".to_string()),
                value_string("".to_string()),
                value_f64(55.0),
                value_string("gamma".to_string()),
                value_bool(true),
                value_f64(0.0),
            ],
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 6.0);

        // Non-empty strings should be counted
        let result = counta(
            vec![
                value_string("hello".to_string()),
                value_string("world".to_string()),
            ],
            &value_format,
        )
        .unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);

        // Zeros should be counted (not blank)
        let result = counta(vec![value_f64(0.0), value_i32(0)], &value_format).unwrap();
        assert_eq!(result.f64(&value_format).unwrap(), 2.0);
    }
}
