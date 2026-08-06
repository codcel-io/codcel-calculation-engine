// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::lookup_and_reference::codcel_tocol::should_include;
use crate::value::Value;
use std::error::Error;

/// Converts a 2D array into a single row, like Excel's `TOROW`.
///
/// - `ignore`: 0=keep all (default), 1=ignore blanks, 2=ignore errors, 3=ignore both.
/// - `scan_by_column`: `false`=scan row-by-row (default), `true`=scan column-by-column.
///
/// # Errors
/// Returns an error when the array is empty or all values were filtered out.
pub fn codcel_torow(
    array: Vec<Vec<Value>>,
    ignore: Option<i32>,
    scan_by_column: Option<bool>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("TOROW: Array cannot be empty".into());
    }

    let ignore = ignore.unwrap_or(0);
    let scan_by_col = scan_by_column.unwrap_or(false);

    let mut result: Vec<Value> = Vec::new();

    if scan_by_col {
        let col_count = array[0].len();
        for col in 0..col_count {
            for row in &array {
                if col < row.len() && should_include(&row[col], ignore) {
                    result.push(row[col].clone());
                }
            }
        }
    } else {
        for row in &array {
            for value in row {
                if should_include(value, ignore) {
                    result.push(value.clone());
                }
            }
        }
    }

    if result.is_empty() {
        return Err("TOROW: All values were filtered out".into());
    }

    // Return as single row (all elements in one row)
    Ok(Value::AreaValue(vec![result]))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn area(v: Value) -> Vec<Vec<Value>> {
        match v {
            Value::AreaValue(a) => a,
            _ => panic!("expected AreaValue"),
        }
    }

    fn i(v: &Value) -> i32 {
        match v {
            Value::I32(n) => *n,
            _ => panic!("expected I32"),
        }
    }

    fn make_2x3() -> Vec<Vec<Value>> {
        vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::I32(6)],
        ]
    }

    #[test]
    fn torow_row_scan() {
        // Row-by-row: [1,2,3,4,5,6]
        let result = area(codcel_torow(make_2x3(), None, None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 6);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[0][2]), 3);
        assert_eq!(i(&result[0][3]), 4);
        assert_eq!(i(&result[0][4]), 5);
        assert_eq!(i(&result[0][5]), 6);
    }

    #[test]
    fn torow_column_scan() {
        // Column-by-column: [1,4,2,5,3,6]
        let result = area(codcel_torow(make_2x3(), None, Some(true)).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 6);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 4);
        assert_eq!(i(&result[0][2]), 2);
        assert_eq!(i(&result[0][3]), 5);
        assert_eq!(i(&result[0][4]), 3);
        assert_eq!(i(&result[0][5]), 6);
    }

    #[test]
    fn torow_single_row_no_change() {
        let array = vec![vec![Value::I32(1), Value::I32(2), Value::I32(3)]];
        let result = area(codcel_torow(array, None, None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[0][2]), 3);
    }

    #[test]
    fn torow_single_column_to_row() {
        let array = vec![
            vec![Value::I32(1)],
            vec![Value::I32(2)],
            vec![Value::I32(3)],
        ];
        let result = area(codcel_torow(array, None, None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[0][2]), 3);
    }

    #[test]
    fn torow_ignore_blanks() {
        let array = vec![
            vec![Value::I32(1), Value::String("".to_string()), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::String("".to_string())],
        ];
        let result = area(codcel_torow(array, Some(1), None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 4);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 3);
        assert_eq!(i(&result[0][2]), 4);
        assert_eq!(i(&result[0][3]), 5);
    }

    #[test]
    fn torow_empty_error() {
        let err = codcel_torow(vec![], None, None).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }
}
