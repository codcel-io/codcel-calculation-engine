// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use std::error::Error;

/// Returns the value at the given 1-based row and column in `array`, like Excel's `INDEX`.
///
/// When `column_num` is `None`, the array is treated as a single row and `row_num` selects the
/// column from that row. When `column_num` is provided, both coordinates must reference existing
/// entries in the 2D array.
///
/// # Errors
/// Returns an error when the array is empty, `row_num`/`column_num` is zero or out of range, or
/// when the requested coordinates do not exist.
pub fn codcel_index(
    array: Vec<Vec<Value>>,
    row_num: i32,
    column_num: Option<i32>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("INDEX: Array cannot be empty".into());
    }

    if row_num == 0 {
        return Err("INDEX: Row number must be greater than 0".into());
    }

    let row_num = (row_num - 1) as usize;

    if let Some(column_num) = column_num {
        if column_num == 0 {
            return Err("INDEX: Column number must be greater than 0".into());
        }

        let column_num = (column_num - 1) as usize;

        if let Some(value) = array.get(row_num).and_then(|row| row.get(column_num)) {
            Ok(value.clone())
        } else {
            Err("INDEX: Row number and column number combination are out of range".into())
        }
    } else if array.len() == 1 {
        // Single-row array: row_num selects the column
        if let Some(value) = array[0].get(row_num) {
            Ok(value.clone())
        } else {
            Err("INDEX: Row number is out of range".into())
        }
    } else if array[0].len() == 1 {
        // Single-column array: row_num selects the row
        if let Some(row) = array.get(row_num) {
            Ok(row[0].clone())
        } else {
            Err("INDEX: Row number is out of range".into())
        }
    } else {
        // Multi-row, multi-column array: row_num selects the entire row
        if let Some(row) = array.get(row_num) {
            Ok(Value::AreaValue(vec![row.clone()]))
        } else {
            Err("INDEX: Row number is out of range".into())
        }
    }
}
