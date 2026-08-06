// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Performs a vertical lookup against the first column of `table_array`, like Excel's `VLOOKUP`.
///
/// `col_index_num` is 1-based and identifies which column's value to return from the matching row.
/// When `range_lookup` is `None` or `Some(true)`, the first column is treated as ascending and the
/// largest value less than or equal to `lookup_value` is returned; when `range_lookup` is
/// `Some(false)`, an exact match is required.
///
/// # Errors
/// Returns an error when the table is empty, a requested column index is zero or beyond a row's
/// length, or no matching row is found.
pub fn codcel_v_lookup(
    lookup_value: Value,
    table_array: Vec<Vec<Value>>,
    col_index_num: i32,
    range_lookup: Option<bool>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if table_array.is_empty() {
        return Err("VLOOKUP: Table array cannot be empty".into());
    }

    if col_index_num == 0 {
        return Err("VLOOKUP: Column index must be greater than 0".into());
    }

    let col_index_num = col_index_num as usize;

    // Check if all rows have enough columns
    for (i, row) in table_array.iter().enumerate() {
        if row.len() < col_index_num {
            return Err(format!(
                "VLOOKUP: Row {} has only {} columns, but column {} was requested",
                i + 1,
                row.len(),
                col_index_num
            )
            .into());
        }
    }

    let range_lookup = range_lookup.unwrap_or(true);

    if range_lookup {
        approximate_v_lookup(lookup_value, table_array, col_index_num)
    } else {
        exact_v_lookup(lookup_value, table_array, col_index_num)
    }
}

fn exact_v_lookup(
    lookup_value: Value,
    table_array: Vec<Vec<Value>>,
    col_index_num: usize,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    for row in table_array.iter() {
        if let Some(first_col_value) = row.first() {
            if lookup_value == *first_col_value {
                return Ok(row[col_index_num - 1].clone()); // Excel uses 1-based indexing
            }
        }
    }

    Err("VLOOKUP: Exact match not found".into())
}

fn approximate_v_lookup(
    lookup_value: Value,
    table_array: Vec<Vec<Value>>,
    col_index_num: usize,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // For approximate match, the first column should be sorted in ascending order
    let mut best_match_row: Option<usize> = None;

    for (i, row) in table_array.iter().enumerate() {
        if let Some(first_col_value) = row.first() {
            match lookup_value.partial_cmp(first_col_value) {
                Some(std::cmp::Ordering::Equal) => {
                    return Ok(row[col_index_num - 1].clone()); // Exact match
                }
                Some(std::cmp::Ordering::Greater) => {
                    best_match_row = Some(i); // Current value is less than lookup
                }
                Some(std::cmp::Ordering::Less) => break, // Values are now too large, stop searching
                None => continue,                        // Incomparable types, skip
            }
        }
    }

    best_match_row
        .map(|i| table_array[i][col_index_num - 1].clone())
        .ok_or_else(|| "VLOOKUP: Approximate match not found".into())
}
