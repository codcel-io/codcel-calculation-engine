// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use crate::value_format::ValueFormat;
use chrono::{DateTime, Utc};
use std::error::Error;

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
fn process_area<T, U, F>(
    inputs: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    converter: fn(&Value, bool, &ValueFormat) -> Result<Vec<Vec<T>>, Box<dyn Error + Send + Sync>>,
    operation: F,
    result_mapper: fn(U) -> Value,
    default: T,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    T: Clone,
    U: 'static,
    F: Fn(Vec<T>) -> Result<U, Box<dyn Error + Send + Sync>>,
{
    // Convert all inputs to areas of T
    let areas: Vec<Vec<Vec<T>>> = inputs
        .iter()
        .map(|input| converter(input, strict_type_conversion, value_format))
        .collect::<Result<_, _>>()?;

    // Determine the maximum dimensions
    let max_rows = areas.iter().map(|area| area.len()).max().unwrap_or(0);
    let max_cols = areas
        .iter()
        .map(|area| area.first().map_or(0, |row| row.len()))
        .max()
        .unwrap_or(0);

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    // Gather the values for the current cell from all inputs
                    let values_at_cell: Vec<T> = areas
                        .iter()
                        .map(|area| {
                            let value = area
                                .get(i % area.len())
                                .and_then(|row| row.get(j % row.len()));
                            if std::mem::size_of::<T>() == 0 {
                                value.cloned().unwrap_or_else(|| default.clone())
                            } else {
                                value.map_or(default.clone(), |v| v.clone())
                            }
                        })
                        .collect();

                    // Apply the operation
                    operation(values_at_cell)
                        .map(result_mapper)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect();

    Ok(Value::AreaValue(result?))
}

/// Specialized function for float operations
pub(crate) fn process_area_float_multi_to_float<F>(
    inputs: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    process_area(
        inputs,
        strict_type_conversion,
        value_format,
        function_name,
        Value::area_of_f64,
        operation,
        Value::F64,
        0.0,
    )
}

/// Specialized function for integer operations
pub(crate) fn process_area_int_multi_to_int<F>(
    inputs: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Vec<i32>) -> Result<i32, Box<dyn Error + Send + Sync>>,
{
    process_area(
        inputs,
        strict_type_conversion,
        value_format,
        function_name,
        Value::area_of_i32,
        operation,
        Value::I32,
        0,
    )
}

/// Specialized function for integer operations
pub(crate) fn process_area_int_multi_to_float<F>(
    inputs: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Vec<i32>) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    process_area(
        inputs,
        strict_type_conversion,
        value_format,
        function_name,
        Value::area_of_i32,
        operation,
        Value::F64,
        0,
    )
}

/// Specialized function for string operations
pub(crate) fn process_area_string_multi_to_bool<F>(
    inputs: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Vec<String>) -> Result<bool, Box<dyn Error + Send + Sync>>,
{
    process_area(
        inputs,
        strict_type_conversion,
        value_format,
        function_name,
        Value::area_of_string,
        operation,
        Value::Bool,
        "".to_string(),
    )
}

pub(crate) fn process_area_float_float_int_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, f64, i32) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_i32(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1.len().max(area_2.len()).max(area_3.len());

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    // Apply the operation
                    operation(value_1, value_2, value_3)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_string_multi_to_float<F>(
    inputs: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Vec<String>) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert all inputs to areas of strings
    let areas: Vec<Vec<Vec<String>>> = inputs
        .into_iter()
        .map(|input| input.area_of_string(strict_type_conversion, value_format))
        .collect::<Result<_, _>>()?;

    // Determine the maximum dimensions
    let max_rows = areas.iter().map(|area| area.len()).max().unwrap_or(0);
    let max_cols = areas
        .iter()
        .map(|area| area.first().map_or(0, |row| row.len()))
        .max()
        .unwrap_or(0);

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    // Gather the string values for the current cell from all inputs
                    let values_at_cell: Vec<String> = areas
                        .iter()
                        .map(|area| {
                            area.get(i % area.len())
                                .and_then(|row| row.get(j % row.len()))
                                .cloned()
                                .unwrap_or_default()
                        })
                        .collect();

                    // Apply the operation
                    operation(values_at_cell)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

/*pub(crate) fn process_area_string_float_to_string<F>(
    string_values: Value,
    f64_values: Value,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(String, f64) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let string_area = string_values.area_of_string(strict_type_conversion, value_format)?;
    let f64_area = f64_values.area_of_f64(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = string_area.len().max(f64_area.len());

    let max_cols = string_area
        .first()
        .map_or(0, |row| row.len())
        .max(f64_area.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    // Retrieve the string value
                    let string_value = string_area
                        .get(i % string_area.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_default();  // TODO: CHECK THIS CODE

                    // Retrieve the f64 value
                    let f64_value = f64_area
                        .get(i % f64_area.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();  // TODO: CHECK THIS CODE

                    // Apply the operation
                    operation(string_value, f64_value)
                        .map(Value::String)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}*/

pub(crate) fn process_area_float_string_to_string<F>(
    f64_values: Value,
    string_values: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, String) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let f64_area = f64_values.area_of_f64(strict_type_conversion, value_format)?;
    let string_area = string_values.area_of_string(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = f64_area.len().max(string_area.len());

    let max_cols = f64_area
        .first()
        .map_or(0, |row| row.len())
        .max(string_area.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    // Retrieve the f64 value
                    let f64_value = f64_area
                        .get(i % f64_area.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default(); // TODO: CHECK THIS

                    // Retrieve the string value
                    let string_value = string_area
                        .get(i % string_area.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_default(); // TODO: CHECK THIS

                    // Apply the operation
                    operation(f64_value, string_value)
                        .map(Value::String)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_string_int_int_to_string<F>(
    string_values: Value,
    int_values_1: Value,
    int_values_2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(String, i32, i32) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let string_area = string_values.area_of_string(strict_type_conversion, value_format)?;
    let int_area_1 = int_values_1.area_of_i32(strict_type_conversion, value_format)?;
    let int_area_2 = int_values_2.area_of_i32(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = string_area
        .len()
        .max(int_area_1.len())
        .max(int_area_2.len());
    let max_cols = string_area
        .first()
        .map_or(0, |row| row.len())
        .max(int_area_1.first().map_or(0, |row| row.len()))
        .max(int_area_2.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let string_value = string_area
                        .get(i % string_area.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_default();

                    let int_value_1 = int_area_1
                        .get(i % int_area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(0);

                    let int_value_2 = int_area_2
                        .get(i % int_area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(0);

                    // Apply the operation
                    operation(string_value, int_value_1, int_value_2)
                        .map(Value::String)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_string_int_to_string<F>(
    string_values: Value,
    int_values: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(String, i32) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let string_area = string_values.area_of_string(strict_type_conversion, value_format)?;
    let int_area = int_values.area_of_i32(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = string_area.len().max(int_area.len());
    let max_cols = string_area
        .first()
        .map_or(0, |row| row.len())
        .max(int_area.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let string_value = string_area
                        .get(i % string_area.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_default();

                    let int_value = int_area
                        .get(i % int_area.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(0);

                    // Apply the operation
                    operation(string_value, int_value)
                        .map(Value::String)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_string_opt_int_to_string<F>(
    string_values: Value,
    optional_int_values: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(String, Option<i32>) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    let area_strings = string_values.area_of_string(strict_type_conversion, value_format)?;
    let area_opt_int =
        optional_int_values.option_area_of_i32(strict_type_conversion, value_format)?;

    let max_rows = area_strings
        .len()
        .max(area_opt_int.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_strings.first().map_or(0, |row| row.len()).max(
        area_opt_int
            .as_ref()
            .and_then(|area| area.first().map(|row| row.len()))
            .unwrap_or(0),
    );

    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let string_value = area_strings
                        .get(i % area_strings.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_default();

                    let opt_int = area_opt_int.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    operation(string_value, opt_int)
                        .map(Value::String)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>();

    Ok(Value::AreaValue(result?))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_string_string_string_opt_int_to_string<F>(
    string_values_1: Value,
    string_values_2: Value,
    string_values_3: Value,
    optional_int_values: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(String, String, String, Option<i32>) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = string_values_1.area_of_string(strict_type_conversion, value_format)?;
    let area_2 = string_values_2.area_of_string(strict_type_conversion, value_format)?;
    let area_3 = string_values_3.area_of_string(strict_type_conversion, value_format)?;
    let area_opt_int =
        optional_int_values.option_area_of_i32(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_opt_int.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(
            area_opt_int
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_default();

                    let opt_int = area_opt_int.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(value_1, value_2, value_3, opt_int)
                        .map(Value::String)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_f64_opt_int_opt_bool_to_string<F>(
    number_values: Value,
    optional_int_values: Value,
    optional_bool_values: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(
        f64,
        Option<i32>,
        Option<bool>,
        &str,
        &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    let area_f64 = number_values.area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_int =
        optional_int_values.option_area_of_i32(strict_type_conversion, value_format)?;
    let area_opt_bool =
        optional_bool_values.option_area_of_bool(strict_type_conversion, value_format)?;

    let max_rows = area_f64
        .len()
        .max(area_opt_int.as_ref().map_or(0, |a| a.len()))
        .max(area_opt_bool.as_ref().map_or(0, |a| a.len()));

    let max_cols = area_f64
        .first()
        .map_or(0, |r| r.len())
        .max(
            area_opt_int
                .as_ref()
                .and_then(|a| a.first().map(|r| r.len()))
                .unwrap_or(0),
        )
        .max(
            area_opt_bool
                .as_ref()
                .and_then(|a| a.first().map(|r| r.len()))
                .unwrap_or(0),
        );

    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let num_val = area_f64
                        .get(i % area_f64.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(0.0);

                    let opt_int = area_opt_int.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    let opt_bool = area_opt_bool.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    operation(
                        num_val,
                        opt_int,
                        opt_bool,
                        &value_format.thousands_separator,
                        &value_format.decimal_separator,
                    )
                    .map(Value::String)
                    .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>();

    Ok(Value::AreaValue(result?))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_string_int_int_string_to_string<F>(
    string_values: Value,
    int_values_1: Value,
    int_values_2: Value,
    additional_string_values: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(String, i32, i32, String) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let string_area = string_values.area_of_string(strict_type_conversion, value_format)?;
    let int_area_1 = int_values_1.area_of_i32(strict_type_conversion, value_format)?;
    let int_area_2 = int_values_2.area_of_i32(strict_type_conversion, value_format)?;
    let additional_string_area =
        additional_string_values.area_of_string(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = string_area
        .len()
        .max(int_area_1.len())
        .max(int_area_2.len())
        .max(additional_string_area.len());
    let max_cols = string_area
        .first()
        .map_or(0, |row| row.len())
        .max(int_area_1.first().map_or(0, |row| row.len()))
        .max(int_area_2.first().map_or(0, |row| row.len()))
        .max(additional_string_area.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let string_value = string_area
                        .get(i % string_area.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_default();

                    let int_value_1 = int_area_1
                        .get(i % int_area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(0);

                    let int_value_2 = int_area_2
                        .get(i % int_area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(0);

                    let additional_string_value = additional_string_area
                        .get(i % additional_string_area.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_default();

                    // Apply the operation
                    operation(
                        string_value,
                        int_value_1,
                        int_value_2,
                        additional_string_value,
                    )
                    .map(Value::String)
                    .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

/// Process an area of float values with an optional second area, applying an operation to each pair
/// and returning a new area of float values.
///
/// # Arguments
/// * `values` - The primary area of float values
/// * `optional_values` - The optional secondary area of float values
/// * `value_format` - Format information for value conversion
/// * `default_optional_value` - Default value to use when optional value is missing
/// * `function_name` - Name of the function for error reporting
/// * `operation` - Function to apply to each pair of values
///
/// # Returns
/// A new area containing the results of applying the operation to each pair of values
pub(crate) fn process_area_float_op_float_to_float<F>(
    values: Value,
    optional_values: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    default_optional_value: f64,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, Option<f64>) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Extract the primary and optional values
    let values = values.area_of_f64(strict_type_conversion, value_format)?;
    let optional_values =
        optional_values.option_area_of_f64(strict_type_conversion, value_format)?;

    // Determine if we should use the default value
    let default_value = if let Some(op_values) = &optional_values {
        if op_values.is_empty() {
            Some(default_optional_value)
        } else {
            None
        }
    } else {
        Some(default_optional_value)
    };

    // Create the secondary values array, using either the provided values or a default
    let secondary_values: Vec<Vec<f64>> = if let Some(value) = default_value {
        vec![vec![value]]
    } else {
        optional_values.unwrap()
    };

    // Calculate dimensions for the result
    let rows_a = values.len();
    let cols_a = values.first().map_or(0, |row| row.len());
    let rows_b = secondary_values.len();
    let cols_b = secondary_values.first().map_or(0, |row| row.len());

    let max_rows = rows_a.max(rows_b);
    let max_cols = cols_a.max(cols_b);

    // Process each cell in the result area
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    // Get the primary value, using modulo to handle different sized areas
                    let a_value = values
                        .get(i % rows_a)
                        .and_then(|row| row.get(j % cols_a))
                        .copied()
                        .unwrap_or(default_optional_value);

                    // Get the secondary value, using modulo to handle different sized areas
                    let b_value = secondary_values
                        .get(i % rows_b)
                        .and_then(|row| row.get(j % cols_b))
                        .copied()
                        .unwrap_or(default_optional_value);

                    // Apply the operation and convert the result to a Value
                    operation(a_value, Some(b_value))
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>();

    // Unwrap the result and return as an AreaValue
    result.map(Value::AreaValue)
}

/// Process an area of integer values with an optional second area, applying an operation to each pair
/// and returning a new area of string values.
///
/// # Arguments
/// * `values` - The primary area of integer values
/// * `optional_values` - The optional secondary area of integer values
/// * `value_format` - Format information for value conversion
/// * `default_optional_value` - Default value to use when optional value is missing
/// * `function_name` - Name of the function for error reporting
/// * `operation` - Function to apply to each pair of values
///
/// # Returns
/// A new area containing the string results of applying the operation to each pair of values
pub(crate) fn process_area_int_op_int_to_string<F>(
    values: Value,
    optional_values: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    default_optional_value: i32,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(i32, Option<i32>) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    // Extract the primary and optional values
    let values = values.area_of_i32(strict_type_conversion, value_format)?;
    let optional_values =
        optional_values.option_area_of_i32(strict_type_conversion, value_format)?;

    // Determine if we should use the default value
    let default_value = if let Some(op_values) = &optional_values {
        if op_values.is_empty() {
            Some(default_optional_value)
        } else {
            None
        }
    } else {
        Some(default_optional_value)
    };

    // Create the secondary values array, using either the provided values or a default
    let secondary_values: Vec<Vec<i32>> = if let Some(value) = default_value {
        vec![vec![value]]
    } else {
        optional_values.unwrap()
    };

    // Calculate dimensions for the result
    let rows_a = values.len();
    let cols_a = values.first().map_or(0, |row| row.len());
    let rows_b = secondary_values.len();
    let cols_b = secondary_values.first().map_or(0, |row| row.len());

    let max_rows = rows_a.max(rows_b);
    let max_cols = cols_a.max(cols_b);

    // Process each cell in the result area
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    // Get the primary value, using modulo to handle different sized areas
                    let a_value = values
                        .get(i % rows_a)
                        .and_then(|row| row.get(j % cols_a))
                        .copied()
                        .unwrap_or(default_optional_value);

                    // Get the secondary value, using modulo to handle different sized areas
                    let b_value = secondary_values
                        .get(i % rows_b)
                        .and_then(|row| row.get(j % cols_b))
                        .copied()
                        .unwrap_or(default_optional_value);

                    // Apply the operation and convert the result to a Value
                    operation(a_value, Some(b_value))
                        .map(Value::String)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>();

    // Return the result as an AreaValue
    result.map(Value::AreaValue)
}

pub(crate) fn process_area_float_op_int_to_string_value_format<F>(
    values: Value,
    optional_values: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    default_optional_value: i32,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, Option<i32>, &ValueFormat) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    let values = values.area_of_f64(strict_type_conversion, value_format)?;
    let optional_values =
        optional_values.option_area_of_i32(strict_type_conversion, value_format)?;

    let default_value = if let Some(op_values) = &optional_values {
        if op_values.is_empty() {
            Some(default_optional_value)
        } else {
            None
        }
    } else {
        Some(default_optional_value)
    };

    let secondary_values: Vec<Vec<i32>> = if let Some(value) = default_value {
        vec![vec![value]]
    } else {
        optional_values.unwrap()
    };

    let rows_a = values.len();
    let cols_a = values.first().map_or(0, |row| row.len());
    let rows_b = secondary_values.len();
    let cols_b = secondary_values.first().map_or(0, |row| row.len());

    let max_rows = rows_a.max(rows_b);
    let max_cols = cols_a.max(cols_b);

    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let a_value = values
                        .get(i % rows_a)
                        .and_then(|row| row.get(j % cols_a))
                        .copied()
                        .unwrap_or(0.0);
                    let b_value = secondary_values
                        .get(i % rows_b)
                        .and_then(|row| row.get(j % cols_b))
                        .copied()
                        .unwrap_or(default_optional_value);
                    operation(a_value, Some(b_value), value_format)
                        .map(Value::String)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_float_float_float_opt_float_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    values_4: Value,
    optional_values: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, f64, f64, f64, Option<f64>) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_4 = values_4.area_of_f64(strict_type_conversion, value_format)?;
    let area_opt = optional_values.option_area_of_f64(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_4.len())
        .max(area_opt.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(area_4.first().map_or(0, |row| row.len()))
        .max(
            area_opt
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_4 = area_4
                        .get(i % area_4.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt = area_opt.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(value_1, value_2, value_3, value_4, opt)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_float_float_opt_float_opt_int_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    optional_values_1: Value,
    optional_values_2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, f64, f64, Option<f64>, Option<i32>) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_1 = optional_values_1.option_area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_2 = optional_values_2.option_area_of_i32(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_opt_1.as_ref().map_or(0, |area| area.len()))
        .max(area_opt_2.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(
            area_opt_1
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        )
        .max(
            area_opt_2
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt_1 = area_opt_1.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    let opt_2 = area_opt_2.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(value_1, value_2, value_3, opt_1, opt_2)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_int_float_float_opt_float_opt_int_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    values_4: Value,
    optional_values_1: Value,
    optional_values_2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(
        f64,
        i32,
        f64,
        f64,
        Option<f64>,
        Option<i32>,
    ) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_i32(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_4 = values_4.area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_1 = optional_values_1.option_area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_2 = optional_values_2.option_area_of_i32(strict_type_conversion, value_format)?;

    // Determine maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_4.len())
        .max(area_opt_1.as_ref().map_or(0, |area| area.len()))
        .max(area_opt_2.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(area_4.first().map_or(0, |row| row.len()))
        .max(
            area_opt_1
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        )
        .max(
            area_opt_2
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_4 = area_4
                        .get(i % area_4.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt_1 = area_opt_1.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    let opt_2 = area_opt_2.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(value_1, value_2, value_3, value_4, opt_1, opt_2)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_float_float_float_float_opt_float_opt_bool_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    values_4: Value,
    values_5: Value,
    optional_values_1: Value,
    optional_values_2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(
        f64,
        f64,
        f64,
        f64,
        f64,
        Option<f64>,
        Option<bool>,
    ) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_4 = values_4.area_of_f64(strict_type_conversion, value_format)?;
    let area_5 = values_5.area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_1 = optional_values_1.option_area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_2 = optional_values_2.option_area_of_bool(strict_type_conversion, value_format)?;

    // Determine maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_4.len())
        .max(area_5.len())
        .max(area_opt_1.as_ref().map_or(0, |area| area.len()))
        .max(area_opt_2.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(area_4.first().map_or(0, |row| row.len()))
        .max(area_5.first().map_or(0, |row| row.len()))
        .max(
            area_opt_1
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        )
        .max(
            area_opt_2
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_4 = area_4
                        .get(i % area_4.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_5 = area_5
                        .get(i % area_5.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt_1 = area_opt_1.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    let opt_2 = area_opt_2.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(value_1, value_2, value_3, value_4, value_5, opt_1, opt_2)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_float_float_opt_float_opt_int_opt_float_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    optional_values_1: Value,
    optional_values_2: Value,
    optional_values_3: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(
        f64,
        f64,
        f64,
        Option<f64>,
        Option<i32>,
        Option<f64>,
    ) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_1 = optional_values_1.option_area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_2 = optional_values_2.option_area_of_i32(strict_type_conversion, value_format)?;
    let area_opt_3 = optional_values_3.option_area_of_f64(strict_type_conversion, value_format)?;

    // Determine maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_opt_1.as_ref().map_or(0, |area| area.len()))
        .max(area_opt_2.as_ref().map_or(0, |area| area.len()))
        .max(area_opt_3.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(
            area_opt_1
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        )
        .max(
            area_opt_2
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        )
        .max(
            area_opt_3
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt_1 = area_opt_1.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    let opt_2 = area_opt_2.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    let opt_3 = area_opt_3.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(value_1, value_2, value_3, opt_1, opt_2, opt_3)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_int_float_int_opt_int<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    optional_values_1: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(i32, f64, i32, Option<i32>) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert inputs into appropriate matrices
    let area_1 = values_1.area_of_i32(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_i32(strict_type_conversion, value_format)?;
    let area_opt_1 = optional_values_1.option_area_of_i32(strict_type_conversion, value_format)?;

    // Determine maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_opt_1.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(
            area_opt_1
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt_1 = area_opt_1.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(value_1, value_2, value_3, opt_1)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_float_float_opt_float_opt_float_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    optional_values_1: Value,
    optional_values_2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, f64, f64, Option<f64>, Option<f64>) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_1 = optional_values_1.option_area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_2 = optional_values_2.option_area_of_f64(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_opt_1.as_ref().map_or(0, |area| area.len()))
        .max(area_opt_2.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(
            area_opt_1
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        )
        .max(
            area_opt_2
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt_1 = area_opt_1.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    let opt_2 = area_opt_2.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(value_1, value_2, value_3, opt_1, opt_2)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_float_float_float_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    values_4: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, f64, f64, f64) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_4 = values_4.area_of_f64(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_4.len());

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(area_4.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_4 = area_4
                        .get(i % area_4.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    // Apply the operation
                    operation(value_1, value_2, value_3, value_4)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_int_int_float_opt_float_opt_int_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    values_4: Value,
    optional_values_1: Value,
    optional_values_2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(
        f64,
        i32,
        i32,
        f64,
        Option<f64>,
        Option<i32>,
    ) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_i32(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_i32(strict_type_conversion, value_format)?;
    let area_4 = values_4.area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_1 = optional_values_1.option_area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_2 = optional_values_2.option_area_of_i32(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_4.len())
        .max(area_opt_1.as_ref().map_or(0, |area| area.len()))
        .max(area_opt_2.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(area_4.first().map_or(0, |row| row.len()))
        .max(
            area_opt_1
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        )
        .max(
            area_opt_2
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_4 = area_4
                        .get(i % area_4.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt_1 = area_opt_1.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    let opt_2 = area_opt_2.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(value_1, value_2, value_3, value_4, opt_1, opt_2)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_float_float_int_opt_int_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    values_4: Value,
    optional_values_2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, f64, f64, i32, Option<i32>) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_4 = values_4.area_of_i32(strict_type_conversion, value_format)?;
    let area_opt_2 = optional_values_2.option_area_of_i32(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_4.len())
        .max(area_opt_2.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(area_4.first().map_or(0, |row| row.len()))
        .max(
            area_opt_2
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt_1 = area_4
                        .get(i % area_4.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(0); // Default to 0 if missing

                    let opt_2 = area_opt_2.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(value_1, value_2, value_3, opt_1, opt_2)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_datetime_datetime_float_int_float_opt_int_to_int<F>(
    value_1: Value,
    datetime_1: Value,
    datetime_2: Value,
    value_2: Value,
    int_value: Value,
    value_3: Value,
    optional_int: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(
        f64,
        DateTime<Utc>,
        DateTime<Utc>,
        f64,
        i32,
        f64,
        Option<i32>,
    ) -> Result<i32, Box<dyn Error + Send + Sync>>,
{
    let area_1 = value_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_dt_1 = datetime_1.area_of_date_time(strict_type_conversion, value_format)?;
    let area_dt_2 = datetime_2.area_of_date_time(strict_type_conversion, value_format)?;
    let area_2 = value_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_int = int_value.area_of_i32(strict_type_conversion, value_format)?;
    let area_3 = value_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_int = optional_int.option_area_of_i32(strict_type_conversion, value_format)?;

    let max_rows = *[
        area_1.len(),
        area_dt_1.len(),
        area_dt_2.len(),
        area_2.len(),
        area_int.len(),
        area_3.len(),
        area_opt_int.as_ref().map_or(0, |a| a.len()),
    ]
    .iter()
    .max()
    .unwrap_or(&0);

    let max_cols = *[
        area_1.first().map_or(0, |r| r.len()),
        area_dt_1.first().map_or(0, |r| r.len()),
        area_dt_2.first().map_or(0, |r| r.len()),
        area_2.first().map_or(0, |r| r.len()),
        area_int.first().map_or(0, |r| r.len()),
        area_3.first().map_or(0, |r| r.len()),
        area_opt_int
            .as_ref()
            .and_then(|a| a.first().map(|r| r.len()))
            .unwrap_or(0),
    ]
    .iter()
    .max()
    .unwrap_or(&0);

    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let v1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let dt1 = area_dt_1
                        .get(i % area_dt_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);

                    let dt2 = area_dt_2
                        .get(i % area_dt_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);

                    let v2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let int_val = area_int
                        .get(i % area_int.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(0);

                    let v3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt_int = area_opt_int.as_ref().and_then(|a| {
                        a.get(i % a.len())
                            .and_then(|row| row.get(j % row.len()).copied())
                    });

                    operation(v1, dt1, dt2, v2, int_val, v3, opt_int)
                        .map(Value::I32)
                        .map_err(|e| -> Box<dyn Error + Send + Sync> {
                            format!("{function_name}: {e}").into()
                        })
                })
                .collect::<Result<Vec<_>, Box<dyn Error + Send + Sync>>>()
        })
        .collect();

    Ok(Value::AreaValue(result?))
}

pub(crate) fn process_area_datetime_datetime_to_int<F>(
    datetime_1: Value,
    datetime_2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(DateTime<Utc>, DateTime<Utc>) -> Result<i32, Box<dyn Error + Send + Sync>>,
{
    let area_dt_1 = datetime_1.area_of_date_time(strict_type_conversion, value_format)?;
    let area_dt_2 = datetime_2.area_of_date_time(strict_type_conversion, value_format)?;

    let max_rows = *[area_dt_1.len(), area_dt_2.len()]
        .iter()
        .max()
        .unwrap_or(&0);

    let max_cols = *[
        area_dt_1.first().map_or(0, |r| r.len()),
        area_dt_2.first().map_or(0, |r| r.len()),
    ]
    .iter()
    .max()
    .unwrap_or(&0);

    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let dt1 = area_dt_1
                        .get(i % area_dt_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);

                    let dt2 = area_dt_2
                        .get(i % area_dt_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);

                    operation(dt1, dt2).map(Value::I32).map_err(
                        |e| -> Box<dyn Error + Send + Sync> {
                            format!("{function_name}: {e}").into()
                        },
                    )
                })
                .collect::<Result<Vec<_>, Box<dyn Error + Send + Sync>>>()
        })
        .collect();

    Ok(Value::AreaValue(result?))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_datetime_datetime_opt_bool_to_int<F>(
    datetime_1: Value,
    datetime_2: Value,
    optional_bool: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(DateTime<Utc>, DateTime<Utc>, Option<bool>) -> Result<i32, Box<dyn Error + Send + Sync>>,
{
    let area_dt_1 = datetime_1.area_of_date_time(strict_type_conversion, value_format)?;
    let area_dt_2 = datetime_2.area_of_date_time(strict_type_conversion, value_format)?;
    let area_opt_bool = optional_bool.option_area_of_bool(strict_type_conversion, value_format)?;

    let max_rows = *[
        area_dt_1.len(),
        area_dt_2.len(),
        area_opt_bool.as_ref().map_or(0, |a| a.len()),
    ]
    .iter()
    .max()
    .unwrap_or(&0);

    let max_cols = *[
        area_dt_1.first().map_or(0, |r| r.len()),
        area_dt_2.first().map_or(0, |r| r.len()),
        area_opt_bool
            .as_ref()
            .and_then(|a| a.first().map(|r| r.len()))
            .unwrap_or(0),
    ]
    .iter()
    .max()
    .unwrap_or(&0);

    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let dt1 = area_dt_1
                        .get(i % area_dt_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);

                    let dt2 = area_dt_2
                        .get(i % area_dt_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);

                    let opt_bool = area_opt_bool.as_ref().and_then(|a| {
                        a.get(i % a.len())
                            .and_then(|row| row.get(j % row.len()).copied())
                    });

                    operation(dt1, dt2, opt_bool).map(Value::I32).map_err(
                        |e| -> Box<dyn Error + Send + Sync> {
                            format!("{function_name}: {e}").into()
                        },
                    )
                })
                .collect::<Result<Vec<_>, Box<dyn Error + Send + Sync>>>()
        })
        .collect();

    Ok(Value::AreaValue(result?))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_datetime_datetime_float_float_opt_int_to_float<F>(
    datetime_1: Value,
    datetime_2: Value,
    values_1: Value,
    values_2: Value,
    optional_int: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(
        DateTime<Utc>,
        DateTime<Utc>,
        f64,
        f64,
        Option<i32>,
    ) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    let area_dt_1 = datetime_1.area_of_date_time(strict_type_conversion, value_format)?;
    let area_dt_2 = datetime_2.area_of_date_time(strict_type_conversion, value_format)?;
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_opt_int = optional_int.option_area_of_i32(strict_type_conversion, value_format)?;

    let max_rows = *[
        area_dt_1.len(),
        area_dt_2.len(),
        area_1.len(),
        area_2.len(),
        area_opt_int.as_ref().map_or(0, |a| a.len()),
    ]
    .iter()
    .max()
    .unwrap_or(&0);

    let max_cols = *[
        area_dt_1.first().map_or(0, |r| r.len()),
        area_dt_2.first().map_or(0, |r| r.len()),
        area_1.first().map_or(0, |r| r.len()),
        area_2.first().map_or(0, |r| r.len()),
        area_opt_int
            .as_ref()
            .and_then(|a| a.first().map(|r| r.len()))
            .unwrap_or(0),
    ]
    .iter()
    .max()
    .unwrap_or(&0);

    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let dt1 = area_dt_1
                        .get(i % area_dt_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);

                    let dt2 = area_dt_2
                        .get(i % area_dt_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);

                    let v1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let v2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let opt_int = area_opt_int.as_ref().and_then(|a| {
                        a.get(i % a.len())
                            .and_then(|row| row.get(j % row.len()).copied())
                    });

                    operation(dt1, dt2, v1, v2, opt_int)
                        .map(Value::F64)
                        .map_err(|e| -> Box<dyn Error + Send + Sync> {
                            format!("{function_name}: {e}").into()
                        })
                })
                .collect::<Result<Vec<_>, Box<dyn Error + Send + Sync>>>()
        })
        .collect();

    Ok(Value::AreaValue(result?))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_datetime_datetime_datetime_float_float_float_int_opt_int_to_float<F>(
    datetime_1: Value,
    datetime_2: Value,
    datetime_3: Value,
    values_1: Value,
    values_2: Value,
    values_3: Value,
    int_value: Value,
    optional_int: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(
        DateTime<Utc>,
        DateTime<Utc>,
        DateTime<Utc>,
        f64,
        f64,
        f64,
        i32,
        Option<i32>,
    ) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    let area_dt_1 = datetime_1.area_of_date_time(strict_type_conversion, value_format)?;
    let area_dt_2 = datetime_2.area_of_date_time(strict_type_conversion, value_format)?;
    let area_dt_3 = datetime_3.area_of_date_time(strict_type_conversion, value_format)?;
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_int = int_value.area_of_i32(strict_type_conversion, value_format)?;
    let area_opt_int = optional_int.option_area_of_i32(strict_type_conversion, value_format)?;

    let max_rows = area_dt_1
        .len()
        .max(area_dt_2.len())
        .max(area_dt_3.len())
        .max(area_1.len())
        .max(area_2.len())
        .max(area_3.len())
        .max(area_int.len())
        .max(area_opt_int.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_dt_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_dt_2.first().map_or(0, |row| row.len()))
        .max(area_dt_3.first().map_or(0, |row| row.len()))
        .max(area_1.first().map_or(0, |row| row.len()))
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(area_int.first().map_or(0, |row| row.len()))
        .max(
            area_opt_int
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let dt_1 = area_dt_1
                        .get(i % area_dt_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);
                    let dt_2 = area_dt_2
                        .get(i % area_dt_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);
                    let dt_3 = area_dt_3
                        .get(i % area_dt_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);

                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();
                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();
                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let int_value = area_int
                        .get(i % area_int.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(0);

                    let opt_int = area_opt_int.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    operation(
                        dt_1, dt_2, dt_3, value_1, value_2, value_3, int_value, opt_int,
                    )
                    .map(Value::F64)
                    .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>();

    let result = result?;
    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_datetime_datetime_datetime_float_float_int_opt_int_opt_bool_to_float<F>(
    datetime_1: Value,
    datetime_2: Value,
    datetime_3: Value,
    values_1: Value,
    values_2: Value,
    int_value: Value,
    optional_int: Value,
    optional_bool: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(
        DateTime<Utc>,
        DateTime<Utc>,
        DateTime<Utc>,
        f64,
        f64,
        i32,
        Option<i32>,
        Option<bool>,
    ) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_dt_1 = datetime_1.area_of_date_time(strict_type_conversion, value_format)?;
    let area_dt_2 = datetime_2.area_of_date_time(strict_type_conversion, value_format)?;
    let area_dt_3 = datetime_3.area_of_date_time(strict_type_conversion, value_format)?;
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_int = int_value.area_of_i32(strict_type_conversion, value_format)?;
    let area_opt_int = optional_int.option_area_of_i32(strict_type_conversion, value_format)?;
    let area_opt_bool = optional_bool.option_area_of_bool(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_dt_1
        .len()
        .max(area_dt_2.len())
        .max(area_dt_3.len())
        .max(area_1.len())
        .max(area_2.len())
        .max(area_int.len())
        .max(area_opt_int.as_ref().map_or(0, |area| area.len()))
        .max(area_opt_bool.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_dt_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_dt_2.first().map_or(0, |row| row.len()))
        .max(area_dt_3.first().map_or(0, |row| row.len()))
        .max(area_1.first().map_or(0, |row| row.len()))
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_int.first().map_or(0, |row| row.len()))
        .max(
            area_opt_int
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        )
        .max(
            area_opt_bool
                .as_ref()
                .and_then(|area| area.first().map(|row| row.len()))
                .unwrap_or(0),
        );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let dt_1 = area_dt_1
                        .get(i % area_dt_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);
                    let dt_2 = area_dt_2
                        .get(i % area_dt_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);
                    let dt_3 = area_dt_3
                        .get(i % area_dt_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or_else(Utc::now);

                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();
                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let int_value = area_int
                        .get(i % area_int.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(0);

                    let opt_int = area_opt_int.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    let opt_bool = area_opt_bool.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(
                        dt_1, dt_2, dt_3, value_1, value_2, int_value, opt_int, opt_bool,
                    )
                    .map(Value::F64)
                    .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_value_to_string<F>(
    string_value: Value,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Value) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    // Convert the input into an appropriate matrix
    let area_value = string_value.area_of_value()?;

    let max_rows = area_value.len();
    let max_cols = area_value.first().map_or(0, |row| row.len());

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let val = area_value
                        .get(i)
                        .and_then(|row| row.get(j))
                        .cloned()
                        .unwrap_or(Value::None);

                    operation(val)
                        .map(Value::String)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>();

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_value_opt_bool_to_string<F>(
    string_value: Value,
    optional_bool: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Value, Option<bool>, &ValueFormat) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_value = string_value.area_of_value()?;
    let area_opt_bool = optional_bool.option_area_of_bool(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_value
        .len()
        .max(area_opt_bool.as_ref().map_or(0, |area| area.len()));

    let max_cols = area_value.first().map_or(0, |row| row.len()).max(
        area_opt_bool
            .as_ref()
            .and_then(|area| area.first().map(|row| row.len()))
            .unwrap_or(0),
    );

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let str_val = area_value
                        .get(i % area_value.len())
                        .and_then(|row| row.get(j % row.len()))
                        .cloned()
                        .unwrap_or(Value::None);

                    let opt_bool = area_opt_bool.as_ref().and_then(|area| {
                        area.get(i % area.len())
                            .and_then(|row| row.get(j % row.len()))
                            .copied()
                    });

                    // Apply the operation
                    operation(str_val, opt_bool, value_format)
                        .map(Value::String)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>();

    let result = result?;

    Ok(Value::AreaValue(result))
}

/// Generic function to process a single area with a single input type and single output type
///
/// This function handles the common pattern of:
/// 1. Extracting a 2D array of values from the input
/// 2. Converting each value to the input type
/// 3. Applying an operation to each value
/// 4. Converting the result to a Value
/// 5. Returning the result as an AreaValue
pub(crate) fn process_area_generic<F, T, U, ExtractFn, ConvertFn>(
    area: Value,
    value_format: &ValueFormat,
    operation: F,
    function_name: &str,
    extract_fn: ExtractFn,
    convert_fn: ConvertFn,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(T) -> Result<U, Box<dyn Error + Send + Sync>>,
    ExtractFn: Fn(&Value, &ValueFormat) -> Result<T, Box<dyn Error + Send + Sync>>,
    ConvertFn: Fn(U) -> Value,
    T: Copy,
{
    process_area_impl(
        area,
        value_format,
        operation,
        function_name,
        extract_fn,
        convert_fn,
    )
}

/// Implementation of process_area_generic without the Copy constraint
/// This allows handling of non-Copy types like String
fn process_area_impl<F, T, U, ExtractFn, ConvertFn>(
    area: Value,
    value_format: &ValueFormat,
    operation: F,
    function_name: &str,
    extract_fn: ExtractFn,
    convert_fn: ConvertFn,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(T) -> Result<U, Box<dyn Error + Send + Sync>>,
    ExtractFn: Fn(&Value, &ValueFormat) -> Result<T, Box<dyn Error + Send + Sync>>,
    ConvertFn: Fn(U) -> Value,
{
    // Extract the 2D array of values
    let values = area.area_of_value()?;

    // Use iterators to transform rows
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = values
        .iter()
        .map(|row| {
            row.iter()
                .map(|value| {
                    // Process each value in the row
                    let input = extract_fn(value, value_format)
                        .map_err(|e| format!("{function_name}: {e}"))?;

                    let computed_result = operation(input)?;

                    Ok(convert_fn(computed_result))
                })
                .collect() // Collect processed row into a Vec<Value>
        })
        .collect(); // Collect rows into a Vec<Vec<Value>>

    // Map the result into the final `Value` type
    result.map(Value::AreaValue)
}

/// Generic function for string inputs (non-Copy types)
pub(crate) fn process_area_generic_string<F, U, ExtractFn, ConvertFn>(
    area: Value,
    value_format: &ValueFormat,
    operation: F,
    function_name: &str,
    extract_fn: ExtractFn,
    convert_fn: ConvertFn,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(String) -> Result<U, Box<dyn Error + Send + Sync>>,
    ExtractFn: Fn(&Value, &ValueFormat) -> Result<String, Box<dyn Error + Send + Sync>>,
    ConvertFn: Fn(U) -> Value,
{
    process_area_impl(
        area,
        value_format,
        operation,
        function_name,
        extract_fn,
        convert_fn,
    )
}

pub(crate) fn process_area_float_to_float<F>(
    area: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
    operation: F,
    function_name: &str,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    process_area_generic(
        area,
        value_format,
        operation,
        function_name,
        |value, value_format| value.f64(value_format),
        Value::F64,
    )
}

pub(crate) fn process_area_float_to_integer<F>(
    area: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
    operation: F,
    function_name: &str,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64) -> Result<i32, Box<dyn Error + Send + Sync>>,
{
    process_area_generic(
        area,
        value_format,
        operation,
        function_name,
        |value, value_format| value.f64(value_format),
        Value::I32,
    )
}

pub(crate) fn process_area_string_to_float<F>(
    area: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
    operation: F,
    function_name: &str,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(&str) -> Result<f64, Box<dyn Error + Send + Sync>> + Copy,
{
    // Create a wrapper that takes ownership of the String and passes a reference to the original function
    let operation_wrapper = move |s: String| operation(&s);

    process_area_generic_string(
        area,
        value_format,
        operation_wrapper,
        function_name,
        |value, value_format| value.string(value_format),
        Value::F64,
    )
}

pub(crate) fn process_area_int_to_string<F>(
    area: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
    operation: F,
    function_name: &str,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(i32) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    process_area_generic(
        area,
        value_format,
        operation,
        function_name,
        |value, value_format| value.i32(value_format),
        Value::String,
    )
}

pub(crate) fn process_area_string_to_int<F>(
    area: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
    operation: F,
    function_name: &str,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(String) -> Result<i32, Box<dyn Error + Send + Sync>>,
{
    process_area_generic_string(
        area,
        value_format,
        operation,
        function_name,
        |value, value_format| value.string(value_format),
        Value::I32,
    )
}

pub(crate) fn process_area_value_to_float<F>(
    area: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(&Value, &ValueFormat) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Extract the 2D array of values
    let values = area.area_of_value()?;

    // Use iterators to transform rows
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = values
        .iter()
        .map(|row| {
            row.iter()
                .map(|value| {
                    // Process each value in the row
                    let computed_result = operation(value, value_format)?;
                    Ok(Value::F64(computed_result))
                })
                .collect() // Collect processed row into a Vec<Value>
        })
        .collect(); // Collect rows into a Vec<Vec<Value>>

    // Map the result into the final `Value` type
    result.map(Value::AreaValue)
}

pub(crate) fn process_area_value_to_bool<F>(
    area: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(&Value, &ValueFormat) -> Result<bool, Box<dyn Error + Send + Sync>>,
{
    // Extract the 2D array of values
    let values = area.area_of_value()?;

    // Use iterators to transform rows
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = values
        .iter()
        .map(|row| {
            row.iter()
                .map(|value| {
                    // Process each value in the row
                    let computed_result = operation(value, value_format)?;
                    Ok(Value::Bool(computed_result))
                })
                .collect() // Collect processed row into a Vec<Value>
        })
        .collect(); // Collect rows into a Vec<Vec<Value>>

    // Map the result into the final `Value` type
    result.map(Value::AreaValue)
}

pub(crate) fn process_area_string_to_string<F>(
    area: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
    operation: F,
    function_name: &str,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(String) -> Result<String, Box<dyn Error + Send + Sync>>,
{
    process_area_generic_string(
        area,
        value_format,
        operation,
        function_name,
        |value, value_format| value.string(value_format),
        Value::String,
    )
}

pub(crate) fn process_area_int_float_float_to_int<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(i32, f64, f64) -> Result<i32, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_i32(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1.len().max(area_2.len()).max(area_3.len());

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    // Apply the operation
                    operation(value_1, value_2, value_3)
                        .map(Value::I32)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_float_float_bool_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    values_4: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, f64, f64, bool) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_4 = values_4.area_of_bool(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_4.len());

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(area_4.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_4 = area_4
                        .get(i % area_4.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(false);

                    // Apply the operation
                    operation(value_1, value_2, value_3, value_4)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_float_float_bool_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, f64, bool) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_bool(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1.len().max(area_2.len()).max(area_3.len());

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(false);

                    // Apply the operation
                    operation(value_1, value_2, value_3)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_float_float_float_float_bool_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    values_4: Value,
    values_5: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, f64, f64, f64, bool) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_4 = values_4.area_of_f64(strict_type_conversion, value_format)?;
    let area_5 = values_5.area_of_bool(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_4.len())
        .max(area_5.len());

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(area_4.first().map_or(0, |row| row.len()))
        .max(area_5.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_4 = area_4
                        .get(i % area_4.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_5 = area_5
                        .get(i % area_5.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(false);

                    // Apply the operation
                    operation(value_1, value_2, value_3, value_4, value_5)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn process_area_int_int_float_bool_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    values_4: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(i32, i32, f64, bool) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_i32(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_i32(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;
    let area_4 = values_4.area_of_bool(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1
        .len()
        .max(area_2.len())
        .max(area_3.len())
        .max(area_4.len());

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()))
        .max(area_4.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_4 = area_4
                        .get(i % area_4.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(false);

                    // Apply the operation
                    operation(value_1, value_2, value_3, value_4)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_int_float_bool_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(i32, f64, bool) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_i32(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_f64(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_bool(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1.len().max(area_2.len()).max(area_3.len());

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(false);

                    // Apply the operation
                    operation(value_1, value_2, value_3)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_int_int_float_to_float<F>(
    values_1: Value,
    values_2: Value,
    values_3: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(i32, i32, f64) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_i32(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_i32(strict_type_conversion, value_format)?;
    let area_3 = values_3.area_of_f64(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1.len().max(area_2.len()).max(area_3.len());

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()))
        .max(area_3.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_3 = area_3
                        .get(i % area_3.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    // Apply the operation
                    operation(value_1, value_2, value_3)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}

pub(crate) fn process_area_float_bool_to_float<F>(
    values_1: Value,
    values_2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
    function_name: &str,
    operation: F,
) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64, bool) -> Result<f64, Box<dyn Error + Send + Sync>>,
{
    // Convert the inputs into appropriate matrices
    let area_1 = values_1.area_of_f64(strict_type_conversion, value_format)?;
    let area_2 = values_2.area_of_bool(strict_type_conversion, value_format)?;

    // Determine the maximum dimensions across inputs
    let max_rows = area_1.len().max(area_2.len());

    let max_cols = area_1
        .first()
        .map_or(0, |row| row.len())
        .max(area_2.first().map_or(0, |row| row.len()));

    // Compute the result matrix
    let result: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = (0..max_rows)
        .map(|i| {
            (0..max_cols)
                .map(|j| {
                    let value_1 = area_1
                        .get(i % area_1.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or_default();

                    let value_2 = area_2
                        .get(i % area_2.len())
                        .and_then(|row| row.get(j % row.len()))
                        .copied()
                        .unwrap_or(false);

                    // Apply the operation
                    operation(value_1, value_2)
                        .map(Value::F64)
                        .map_err(|e| format!("{function_name}: {e}").into())
                })
                .collect::<Result<Vec<_>, _>>() // Specify the inner result type for the row
        })
        .collect::<Result<Vec<_>, _>>(); // Specify the outer result type for the matrix

    let result = result?;

    Ok(Value::AreaValue(result))
}
