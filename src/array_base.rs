// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::information::codcel_cell::codcel_cell;
use crate::lookup_and_reference::codcel_address::codcel_address;
use crate::lookup_and_reference::codcel_areas::codcel_areas;
use crate::lookup_and_reference::codcel_choosecols::codcel_choosecols;
use crate::lookup_and_reference::codcel_column::codcel_column;
use crate::lookup_and_reference::codcel_columns::codcel_columns;
use crate::lookup_and_reference::codcel_row::codcel_row;
use crate::lookup_and_reference::codcel_rows::codcel_rows;
use crate::lookup_and_reference::codcel_chooserows::codcel_chooserows;
use crate::lookup_and_reference::codcel_drop::codcel_drop;
use crate::lookup_and_reference::codcel_expand::codcel_expand;
use crate::lookup_and_reference::codcel_offset::codcel_offset;
use crate::lookup_and_reference::codcel_take::codcel_take;
use crate::lookup_and_reference::codcel_tocol::codcel_tocol;
use crate::lookup_and_reference::codcel_torow::codcel_torow;
use crate::lookup_and_reference::codcel_transpose::codcel_transpose;
use crate::lookup_and_reference::codcel_trimrange::codcel_trimrange;
use crate::lookup_and_reference::codcel_wrapcols::codcel_wrapcols;
use crate::lookup_and_reference::codcel_wraprows::codcel_wraprows;
use crate::codcel_information;
use crate::value::Value;
use crate::value::vec_value_to_vec_i32;
use crate::text::dbcs_utils::dbcs_byte_len;
use crate::value_format::ValueFormat;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

// TODO : CHECK HOW TO FIX THIS FOR WASM
//use rayon::prelude::*;

fn does_column_contain_any_pure_strings(values: &[Vec<Value>], column_index: usize) -> bool {
    // Use `par_iter` for parallel iteration
    values.iter().any(|row| {
        row.get(column_index)
            .is_some_and(|cell| cell.is_single_string())
    })
}

pub fn sort_by(
    area: Value,
    by_area: Vec<(Value, Value)>,
    format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = area.area_of_value()?; // Input matrix

    // Parse sorting keys and orders
    let by_areas: Vec<(Vec<Value>, i32)> = by_area
        .into_iter()
        .map(|(by_array, sort_order)| {
            let key_area = by_array.area_of_value()?; // Convert sorting keys to matrix
            if key_area.len() != 1 {
                return Err("by_area must contain exactly one row of sorting keys.".into());
            }
            let order = sort_order.option_i32(format)?.unwrap_or(1); // Default to ascending
            Ok((key_area[0].clone(), order)) // Extract the single row of keys
        })
        .collect::<Result<Vec<_>, Box<dyn Error + Send + Sync>>>()?;

    // Validate that all rows in `values` have the same length
    if values.len() != 1 {
        return Err("The input area must contain exactly one row.".into());
    }

    let row = &values[0]; // Extract the single row of values to sort

    // Create a vector of (index, value) for sorting
    let mut indexed_values: Vec<(usize, &Value)> = row.iter().enumerate().collect();

    // Perform sorting
    indexed_values.sort_by(|(index_a, _value_a), (index_b, _value_b)| {
        for (key_row, order) in &by_areas {
            let key_a = &key_row[*index_a];
            let key_b = &key_row[*index_b];

            let comparison = if key_a.is_single_string() || key_b.is_single_string() {
                key_a
                    .string(format)
                    .unwrap_or("".to_string())
                    .partial_cmp(&key_b.string(format).unwrap_or("".to_string()))
                    .unwrap_or(Ordering::Equal)
            } else {
                key_a.partial_cmp(key_b).unwrap_or(Ordering::Equal)
            };

            if comparison != Ordering::Equal {
                return if *order == -1 {
                    comparison.reverse()
                } else {
                    comparison
                };
            }
        }
        Ordering::Equal // All keys are equal, maintain original order
    });

    // Extract sorted values based on the sorted indices
    let sorted_values: Vec<Value> = indexed_values
        .into_iter()
        .map(|(_, value)| value.clone())
        .collect();

    // Return the result as a single-row AreaValue
    Ok(Value::AreaValue(vec![sorted_values]))
}

pub fn sort(
    area: Value,
    sort_index: Value,
    sort_order: Value,
    by_col: Value,
    format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = area.area_of_value()?;

    // Determine if the 'by_col' parameter is set to true
    let by_col = by_col.option_bool(format)?.unwrap_or_default();

    // Determine the sorting index (default to 0 if not provided or invalid)
    let sort_index: usize = if let Some(sort) = sort_index.option_i32(format)? {
        sort as usize - 1 // It is necessary to subtract 1 from the index
    } else {
        0
    };

    let descending = sort_order.option_i32(format)?.unwrap_or(1) == -1;

    // Transpose the matrix if by_col is true
    let transposed_values = if by_col { transpose_internal(&values) } else { values };

    // Create a new sorted matrix
    let sorted_values = sort_row(transposed_values, sort_index, descending, format);

    // Transpose back if by_col is true to restore original structure
    let final_values = if by_col {
        transpose_ref_internal(&sorted_values)
    } else {
        sorted_values
    };

    // Return as AreaValue in the same format (rows/columns) as input
    Ok(Value::AreaValue(final_values))
}

// Helper function to transpose a 2D vector of owned values
fn transpose_ref_internal(matrix: &Vec<Vec<Value>>) -> Vec<Vec<Value>> {
    if matrix.is_empty() {
        return Vec::new();
    }
    let row_len = matrix[0].len();
    let mut transposed: Vec<Vec<Value>> = vec![Vec::new(); row_len];

    for row in matrix {
        for (i, value) in row.iter().enumerate() {
            transposed[i].push(value.clone());
        }
    }
    transposed
}
pub fn unique(
    area: Value,
    by_col: Value,
    exactly_once: Value,
    _format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = area.area_of_value()?;

    // Determine if the 'exactly_once' parameter is set to true
    let exactly_once = match exactly_once {
        Value::Bool(b) => b,
        _ => false, // Default to false if exactly_once is not a boolean
    };

    // Determine if the 'by_col' parameter is set to true
    let by_col = match by_col {
        Value::Bool(b) => b,
        _ => false, // Default to false if by_col is not a boolean
    };

    // Transpose the matrix if by_col is true
    let transposed_values = if by_col { transpose_internal(&values) } else { values };

    // HashMap to keep track of occurrences of each value
    let mut occurrences: HashMap<Value, usize> = HashMap::new();

    // Count occurrences of each value
    for row in &transposed_values {
        for value in row {
            let cloned_value = (*value).clone();
            *occurrences.entry(cloned_value).or_insert(0) += 1;
        }
    }

    // Vec to keep the unique values in their original order
    let mut unique_values: Vec<Vec<Value>> = Vec::new();
    let mut seen: HashMap<Value, bool> = HashMap::new(); // To keep track of first appearances

    // Iterate over rows (or columns, if transposed) while checking the occurrences
    for row in transposed_values.iter() {
        let mut unique_row: Vec<Value> = Vec::new();

        for value in row.iter() {
            let cloned_value = (*value).clone();

            if let Some(count) = occurrences.get(&cloned_value) {
                #[allow(clippy::map_entry)]
                if !seen.contains_key(&cloned_value) {
                    // Only consider the first appearance
                    // If `exactly_once` is true, only add if it appears exactly once
                    if (exactly_once && *count == 1) || (!exactly_once && *count >= 1) {
                        unique_row.push(cloned_value.clone());
                    }
                    seen.insert(cloned_value, true); // Mark as seen
                }
            }
        }

        // Only push non-empty rows/columns to the result
        if !unique_row.is_empty() {
            unique_values.push(unique_row);
        }
    }

    // Return as AreaValue in the same format (rows/columns) as input
    Ok(Value::AreaValue(unique_values))
}

pub fn take(
    area: Value,
    rows: Value,
    columns: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let rows = rows.option_i32(value_format)?;
    let columns = columns.option_i32(value_format)?;

    codcel_take(array, rows, columns)
}

pub fn drop(
    area: Value,
    rows: Value,
    columns: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let rows = rows.option_i32(value_format)?;
    let columns = columns.option_i32(value_format)?;

    codcel_drop(array, rows, columns)
}

#[allow(clippy::too_many_arguments)]
pub fn offset(
    area: Value,
    rows: Value,
    cols: Value,
    height: Value,
    width: Value,
    ref_row: Value,
    ref_col: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let rows = rows.i32(value_format)?;
    let cols = cols.i32(value_format)?;
    let height = height.option_i32(value_format)?;
    let width = width.option_i32(value_format)?;
    let ref_row = ref_row.i32(value_format)?;
    let ref_col = ref_col.i32(value_format)?;

    codcel_offset(array, rows, cols, height, width, ref_row, ref_col)
}

pub fn transpose(
    area: Value,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    codcel_transpose(array)
}

pub fn column(
    area: Value,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    codcel_column(array)
}

pub fn columns(
    area: Value,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    codcel_columns(array)
}

pub fn row(
    area: Value,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    codcel_row(array)
}

pub fn rows(
    area: Value,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    codcel_rows(array)
}

pub fn areas(
    reference: Value,
    _value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_areas(reference)
}

pub fn address(
    row_num: Value,
    col_num: Value,
    abs_num: Value,
    a1: Value,
    sheet_text: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let row_num = row_num.i32(value_format)?;
    let col_num = col_num.i32(value_format)?;
    let abs_num = abs_num.option_i32(value_format)?;
    let a1 = a1.option_bool(value_format)?;
    let sheet_text = sheet_text.option_string(value_format)?;
    codcel_address(row_num, col_num, abs_num, a1, sheet_text)
}

pub fn cell(
    info_type: Value,
    reference: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let info_type_str = info_type.string(value_format)?;
    codcel_cell(&info_type_str, &reference, value_format)
}

pub fn tocol(
    area: Value,
    ignore: Value,
    scan_by_column: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let ignore = ignore.option_i32(value_format)?;
    let scan_by_column = scan_by_column.option_bool(value_format)?;
    codcel_tocol(array, ignore, scan_by_column)
}

pub fn torow(
    area: Value,
    ignore: Value,
    scan_by_column: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let ignore = ignore.option_i32(value_format)?;
    let scan_by_column = scan_by_column.option_bool(value_format)?;
    codcel_torow(array, ignore, scan_by_column)
}

pub fn chooserows(
    area: Value,
    indices: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let row_indices = vec_value_to_vec_i32(indices, value_format)?;
    codcel_chooserows(array, row_indices)
}

pub fn choosecols(
    area: Value,
    indices: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let col_indices = vec_value_to_vec_i32(indices, value_format)?;
    codcel_choosecols(array, col_indices)
}

pub fn wraprows(
    area: Value,
    wrap_count: Value,
    pad_with: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let wrap_count = wrap_count.i32(value_format)?;
    let pad_with = if pad_with.is_none() { None } else { Some(pad_with) };
    codcel_wraprows(array, wrap_count, pad_with)
}

pub fn wrapcols(
    area: Value,
    wrap_count: Value,
    pad_with: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let wrap_count = wrap_count.i32(value_format)?;
    let pad_with = if pad_with.is_none() { None } else { Some(pad_with) };
    codcel_wrapcols(array, wrap_count, pad_with)
}

pub fn expand(
    area: Value,
    rows: Value,
    columns: Value,
    pad_with: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let target_rows = if rows.is_none() {
        array.len() as i32
    } else {
        rows.i32(value_format)?
    };
    let columns = columns.option_i32(value_format)?;
    let pad_with = if pad_with.is_none() { None } else { Some(pad_with) };
    codcel_expand(array, target_rows, columns, pad_with)
}

pub fn trimrange(
    area: Value,
    trim_rows: Value,
    trim_columns: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = area.area_of_value()?;
    let trim_rows = trim_rows.option_i32(value_format)?;
    let trim_columns = trim_columns.option_i32(value_format)?;
    codcel_trimrange(array, trim_rows, trim_columns)
}

/*pub fn left(area: Value, num_chars: Value, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = area.area_of_value()?;

    // Extract the number of characters to take (default to 1 if not provided)
    let num_chars_option: Option<i32> = num_chars.option_i32(value_format).expect("LEFT: num_chars value is not a number");
    let chars_to_take = num_chars_option.unwrap_or(1);

    // Resultant 2D array for the final output
    let mut result: Vec<Vec<Value>> = Vec::new();

    // Iterate over each row in the 2D array
    for row in values.iter() {
        let mut result_row: Vec<Value> = Vec::new();

        // Iterate over each value (cell) in the row
        for value in row.iter() {
            let text_string = value.string(value_format).expect("LEFT: Text value is not a string");

            // Apply the `LEFT` logic for each string
            let left_value = if chars_to_take > text_string.len() as i32 {
                Value::String(text_string)  // Return the entire string if `chars_to_take` exceeds string length
            } else {
                Value::String(text_string.chars().take(chars_to_take as usize).collect())  // Take the first `chars_to_take` characters
            };

            // Add the processed string to the result row
            result_row.push(left_value);
        }

        // Add the processed row to the final result
        result.push(result_row);
    }

    // Return the 2D array as the final result
    Ok(Value::AreaValue(result))
}*/

pub fn excel_mod(
    area: Value,
    divisor: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let divisor = divisor.f64(value_format)?;
    if divisor == 0.0 {
        return Err("MOD: Division by zero error".into());
    }

    let values = area.area_of_value()?;

    // Resultant 2D array for the final output
    let mut result: Vec<Vec<Value>> = Vec::new();

    // Iterate over each row in the 2D array
    for row in values.iter() {
        let mut result_row: Vec<Value> = Vec::new();

        // Iterate over each value (cell) in the row
        for value in row.iter() {
            let number = value.f64(value_format).expect("MOD: It must be a number");

            // Excel MOD: n - d * INT(n/d) where INT = floor
            let result = number - divisor * (number / divisor).floor();

            // Add the processed result to the result row
            result_row.push(Value::F64(result));
        }

        // Add the processed row to the final result
        result.push(result_row);
    }

    // Return the 2D array as the final result
    Ok(Value::AreaValue(result))
}

// Helper function to transpose a 2D vector
fn transpose_internal(area: &Vec<Vec<Value>>) -> Vec<Vec<Value>> {
    if area.is_empty() {
        return Vec::new();
    }

    let row_count = area.len();
    let col_count = area[0].len();
    let mut transposed: Vec<Vec<Value>> = vec![Vec::with_capacity(row_count); col_count];

    for row in area {
        for (col_idx, value) in row.iter().enumerate() {
            transposed[col_idx].push(value.clone());
        }
    }

    transposed
}

pub fn iserr(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_information::iserr(value, strict_type_conversion, value_format)
}

pub fn iserror(
    value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    codcel_information::iserror(value, strict_type_conversion, value_format)
}

pub fn len(area: Value, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = area.area_of_value()?;

    let mut result: Vec<Vec<Value>> = Vec::new();

    for row in values.iter() {
        let mut result_row: Vec<Value> = Vec::new();

        for value in row.iter() {
            let text_string = value.string(value_format)?;
            let len_value = Value::I32(text_string.chars().count() as i32);
            result_row.push(len_value);
        }

        result.push(result_row);
    }

    Ok(Value::AreaValue(result))
}

pub fn lenb(area: Value, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = area.area_of_value()?;

    let mut result: Vec<Vec<Value>> = Vec::new();

    for row in values.iter() {
        let mut result_row: Vec<Value> = Vec::new();

        for value in row.iter() {
            let text_string = value.string(value_format)?;
            let len_value = Value::I32(dbcs_byte_len(&text_string) as i32);
            result_row.push(len_value);
        }

        result.push(result_row);
    }

    Ok(Value::AreaValue(result))
}

// Function to create a sorted copy of a vector of values based on sort_index
fn sort_row(
    area: Vec<Vec<Value>>,
    sort_index: usize,
    descending: bool,
    format: &ValueFormat,
) -> Vec<Vec<Value>> {
    let (mut valid_rows, invalid_rows): (Vec<_>, Vec<_>) =
        area.into_iter().partition(|row| row.len() > sort_index);

    let string_sort = does_column_contain_any_pure_strings(&valid_rows, sort_index);

    if string_sort {
        if descending {
            valid_rows.sort_by(|a, b| {
                b[sort_index]
                    .string(format)
                    .unwrap_or_default()
                    .cmp(&a[sort_index].string(format).unwrap_or_default())
            });
        } else {
            valid_rows.sort_by(|a, b| {
                a[sort_index]
                    .string(format)
                    .unwrap_or_default()
                    .cmp(&b[sort_index].string(format).unwrap_or_default())
            });
        }
    } else if descending {
        valid_rows.sort_by(|a, b| {
            b[sort_index]
                .partial_cmp(&a[sort_index])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    } else {
        valid_rows.sort_by(|a, b| {
            a[sort_index]
                .partial_cmp(&b[sort_index])
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    // Combine sorted valid rows with unsorted invalid rows
    valid_rows.extend(invalid_rows);
    valid_rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_does_column_contain_any_pure_strings() {
        let val1 = Value::String("Test1".to_string());
        let val2 = Value::String("2".to_string());
        let row1 = vec![vec![val1], vec![val2]];
        assert!(does_column_contain_any_pure_strings(&row1, 0));

        let val1 = Value::String("1".to_string());
        let val2 = Value::String("2".to_string());
        let row1 = vec![vec![val1], vec![val2]];
        assert!(!does_column_contain_any_pure_strings(&row1, 0));
    }
}
