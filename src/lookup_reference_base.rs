// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::lookup_and_reference::codcel_choose::codcel_choose;
use crate::lookup_and_reference::codcel_h_lookup::codcel_h_lookup;
use crate::lookup_and_reference::codcel_index::codcel_index;
use crate::lookup_and_reference::codcel_lookup::codcel_lookup;
use crate::lookup_and_reference::codcel_match::codcel_match;
use crate::lookup_and_reference::codcel_v_lookup::codcel_v_lookup;
use crate::lookup_and_reference::codcel_x_lookup::codcel_x_lookup;
use crate::lookup_and_reference::codcel_x_match::codcel_x_match;
use crate::value::Value::VecValue;
use crate::value::{vec_value_to_vec_value, Value};
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `FORMULATEXT` function.
/// Returns the formula as a text string, adjusting for locale-specific separators.
/// - `formulas`: the formula value(s) to convert to text.
/// - `strict_type_conversion`: if `true`, enforces strict type conversion rules.
/// - `value_format`: locale settings including the decimal separator.
///
/// Returns the formula text with commas replaced by semicolons when the locale uses comma as decimal separator.
///
/// For single values, returns a `Value::String`; for arrays, returns `Value::VecValue` or `Value::AreaValue`.
pub fn formula_text(
    formulas: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if value_format.decimal_separator == "," {
        let formulas = formulas.area_of_string(strict_type_conversion, value_format)?;

        let formulas: Vec<Vec<Value>> = formulas
            .iter()
            .map(|values| {
                values
                    .iter()
                    .map(|value| Value::String(value.replace(",", ";")))
                    .collect()
            })
            .collect();

        if formulas.len() == 1 && formulas[0].len() == 1 {
            Ok(formulas[0][0].clone())
        } else if formulas.len() == 1 {
            Ok(Value::VecValue(formulas[0].clone()))
        } else {
            Ok(Value::AreaValue(formulas))
        }
    } else {
        Ok(formulas)
    }
}

/// Excel-compatible `CHOOSE` function.
/// Returns a value from a list of values based on an index number.
/// - `indices`: the index (1-based) specifying which value to return.
/// - `values`: a list of values from which to choose (up to 254 values).
/// - `value_format`: locale settings for numeric conversion.
///
/// Returns the value at the specified index position.
///
/// Returns an error if the index is less than 1 or exceeds the number of available values.
pub fn choose(
    indices: Value,
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let indices = indices.to_flatterned_vec_i32(value_format)?;
    let values = vec_value_to_vec_value(values)?;

    Ok(VecValue(codcel_choose(indices, &values)?))
}

/// Excel-compatible `MATCH` function.
/// Returns the relative position of a value within an array.
/// - `lookup_value`: the value to search for in the lookup array.
/// - `lookup_array`: a contiguous range of cells or array containing possible lookup values.
/// - `match_type`: specifies the match behavior:
///   - `1` or omitted: finds the largest value less than or equal to `lookup_value` (array must be sorted ascending).
///   - `0`: finds the first value exactly equal to `lookup_value` (supports wildcards `*` and `?`).
///   - `-1`: finds the smallest value greater than or equal to `lookup_value` (array must be sorted descending).
/// - `value_format`: locale settings for type conversion.
///
/// Returns the 1-based position of the matched value.
///
/// Returns an error if no match is found.
pub fn match_array(
    lookup_value: Value,
    lookup_array: Value,
    match_type: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let lookup_value = lookup_value.to_single_value();
    let lookup_array = lookup_array.to_flatterned_vec_value()?;
    let match_type = match_type.option_i32(value_format)?;

    Ok(Value::I32(codcel_match(
        lookup_value,
        lookup_array,
        match_type,
    )?))
}

/// Excel-compatible `VLOOKUP` function.
/// Searches for a value in the first column of a table and returns a value in the same row from a specified column.
/// - `lookup_value`: the value to search for in the first column of the table.
/// - `lookup_array`: a 2D array (table) containing the data to search.
/// - `col_index_num`: the column number (1-based) in the table from which to return a value.
/// - `range_lookup`: determines the match type:
///   - `true` or omitted: approximate match (first column must be sorted ascending).
///   - `false`: exact match required.
/// - `value_format`: locale settings for type conversion.
///
/// Returns the value from the specified column in the matching row.
///
/// Returns an error if no match is found or if `col_index_num` is out of range.
pub fn v_lookup_array(
    lookup_value: Value,
    lookup_array: Value,
    col_index_num: Value,
    range_lookup: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let lookup_value = lookup_value.to_single_value();
    let lookup_array = lookup_array.area_of_value()?;
    let col_index_num = col_index_num.i32(value_format)?;
    let range_lookup = range_lookup.option_bool(value_format)?;

    codcel_v_lookup(lookup_value, lookup_array, col_index_num, range_lookup)
}

/// Excel-compatible `INDEX` function.
/// Returns the value at a specified row and column intersection within an array.
/// - `array`: the range or array from which to return a value.
/// - `row_num`: the row number (1-based) in the array. Use `0` to return the entire column.
/// - `column_num`: the column number (1-based) in the array. Use `0` or omit to return the entire row.
/// - `value_format`: locale settings for type conversion.
///
/// Returns the value at the intersection of the specified row and column.
///
/// Returns an error if row or column numbers are out of range.
pub fn index_array(
    array: Value,
    row_num: Value,
    column_num: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = array.area_of_value()?;
    let row_num = row_num.i32(value_format)?;
    let column_num = column_num.option_i32(value_format)?;

    codcel_index(array, row_num, column_num)
}

/// Excel-compatible `LOOKUP` function.
/// Searches for a value in a vector and returns a value from the same position in a second vector.
/// - `lookup_value`: the value to search for in the lookup array.
/// - `lookup_array`: a one-row or one-column range to search (must be sorted in ascending order).
/// - `result_vector`: a one-row or one-column range from which to return the result. If omitted, the lookup array is used.
/// - `_value_format`: locale settings (unused in this function).
///
/// Returns the value from `result_vector` at the position where the match was found.
///
/// The function finds the largest value less than or equal to `lookup_value`.
///
/// Returns an error if no suitable match is found.
pub fn lookup_array(
    lookup_value: Value,
    lookup_array: Value,
    result_vector: Value,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let lookup_value = lookup_value.to_single_value();
    let lookup_array = lookup_array.to_flatterned_vec_value()?;
    let result_vector = result_vector.option_vec_of_value()?;
    codcel_lookup(lookup_value, lookup_array, result_vector)
}

/// Excel-compatible `HLOOKUP` function.
/// Searches for a value in the first row of a table and returns a value in the same column from a specified row.
/// - `lookup_value`: the value to search for in the first row of the table.
/// - `lookup_array`: a 2D array (table) containing the data to search.
/// - `col_index_num`: the row number (1-based) in the table from which to return a value.
/// - `range_lookup`: determines the match type:
///   - `true` or omitted: approximate match (first row must be sorted ascending).
///   - `false`: exact match required.
/// - `value_format`: locale settings for type conversion.
///
/// Returns the value from the specified row in the matching column.
///
/// Returns an error if no match is found or if `col_index_num` is out of range.
pub fn h_lookup_array(
    lookup_value: Value,
    lookup_array: Value,
    col_index_num: Value,
    range_lookup: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let lookup_value = lookup_value.to_single_value();
    let lookup_array = lookup_array.area_of_value()?;
    let col_index_num = col_index_num.i32(value_format)?;
    let range_lookup = range_lookup.option_bool(value_format)?;

    codcel_h_lookup(lookup_value, lookup_array, col_index_num, range_lookup)
}

/// Excel-compatible `XLOOKUP` function.
/// Searches for a value in a range and returns a corresponding value from another range.
/// - `lookup_value`: the value to search for.
/// - `lookup_array`: the array or range to search.
/// - `return_array`: the array or range from which to return values.
/// - `if_not_found`: the value to return if no match is found. If omitted, returns an error on no match.
/// - `match_mode`: specifies the match type:
///   - `0` or omitted: exact match.
///   - `-1`: exact match or next smaller item.
///   - `1`: exact match or next larger item.
///   - `2`: wildcard match (`*`, `?`, `~`).
/// - `search_mode`: specifies the search direction:
///   - `1` or omitted: search first to last.
///   - `-1`: search last to first.
///   - `2`: binary search ascending (lookup array must be sorted ascending).
///   - `-2`: binary search descending (lookup array must be sorted descending).
/// - `value_format`: locale settings for type conversion.
///
/// Returns the corresponding value from `return_array` at the matched position.
pub fn x_lookup_array(
    lookup_value: Value,
    lookup_array: Value,
    return_array: Value,
    if_not_found: Value,
    match_mode: Value,
    search_mode: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let lookup_value = lookup_value.to_single_value();
    let lookup_array = lookup_array.to_flatterned_vec_value()?;
    let return_array = return_array.to_flatterned_vec_value()?;
    let if_not_found = if_not_found.option_value()?;
    let match_mode = match_mode.option_i32(value_format)?;
    let search_mode = search_mode.option_i32(value_format)?;

    codcel_x_lookup(
        lookup_value,
        lookup_array,
        return_array,
        if_not_found,
        match_mode,
        search_mode,
    )
}

/// Excel-compatible `XMATCH` function.
/// Returns the relative position of a value in an array with flexible matching options.
/// - `lookup_value`: the value to search for.
/// - `lookup_array`: the array or range to search.
/// - `match_mode`: specifies the match type:
///   - `0` or omitted: exact match.
///   - `-1`: exact match or next smaller item.
///   - `1`: exact match or next larger item.
///   - `2`: wildcard match (`*`, `?`, `~`).
/// - `search_mode`: specifies the search direction:
///   - `1` or omitted: search first to last.
///   - `-1`: search last to first.
///   - `2`: binary search ascending (lookup array must be sorted ascending).
///   - `-2`: binary search descending (lookup array must be sorted descending).
/// - `value_format`: locale settings for type conversion.
///
/// Returns the 1-based position of the matched value.
///
/// Returns an error if no match is found.
pub fn x_match_array(
    lookup_value: Value,
    lookup_array: Value,
    match_mode: Value,
    search_mode: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let lookup_value = lookup_value.to_single_value();
    let lookup_array = lookup_array.to_flatterned_vec_value()?;
    let match_mode = match_mode.option_i32(value_format)?;
    let search_mode = search_mode.option_i32(value_format)?;

    Ok(Value::I32(codcel_x_match(
        lookup_value,
        lookup_array,
        match_mode,
        search_mode,
    )?))
}

/// Excel-compatible `FILTER` function for array values.
/// Filters an array based on a boolean include mask.
/// Used when FILTER is called inside a lambda (BYROW, MAP, etc.) with lambda parameters.
///
/// - `array`: the array to filter
/// - `include`: boolean array (same size) - true keeps element, false excludes
/// - `if_empty`: value to return if all elements are filtered out
/// - `value_format`: locale settings for type conversion
///
/// Returns filtered array containing only elements where include is truthy.
pub fn filter_array(
    array: Value,
    include: Value,
    if_empty: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array_elements = array.to_flatterned_vec_value()?;
    let include_elements = include.to_flatterned_vec_value()?;

    let mut results: Vec<Value> = Vec::new();
    for (elem, inc) in array_elements.into_iter().zip(include_elements) {
        // Check if include value is truthy (non-zero number, true boolean, non-empty string)
        let is_truthy: bool = inc.bool(value_format).unwrap_or_default();
        if is_truthy {
            results.push(elem);
        }
    }

    if results.is_empty() {
        Ok(if_empty)
    } else {
        Ok(Value::VecValue(results))
    }
}
