// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Checks whether a value is blank (empty string, None, or Option variants with None).
pub(crate) fn is_blank(value: &Value) -> bool {
    match value {
        Value::None => true,
        Value::String(s) => s.is_empty(),
        Value::OptionString(None) => true,
        Value::OptionF64(None) => true,
        Value::OptionI32(None) => true,
        Value::OptionBool(None) => true,
        _ => false,
    }
}

/// Determines whether a value should be included based on the `ignore` parameter.
///
/// - 0: keep all values (default)
/// - 1: ignore blanks
/// - 2: ignore errors (not applicable in our system, treated as keep all)
/// - 3: ignore blanks and errors
pub(crate) fn should_include(value: &Value, ignore: i32) -> bool {
    match ignore {
        1 | 3 => !is_blank(value),
        _ => true,
    }
}

/// Converts a 2D array into a single column, like Excel's `TOCOL`.
///
/// - `ignore`: 0=keep all (default), 1=ignore blanks, 2=ignore errors, 3=ignore both.
/// - `scan_by_column`: `false`=scan row-by-row (default), `true`=scan column-by-column.
///
/// # Errors
/// Returns an error when the array is empty or all values were filtered out.
pub fn codcel_tocol(
    array: Vec<Vec<Value>>,
    ignore: Option<i32>,
    scan_by_column: Option<bool>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("TOCOL: Array cannot be empty".into());
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
        return Err("TOCOL: All values were filtered out".into());
    }

    // Return as single column (each element is its own row)
    let column: Vec<Vec<Value>> = result.into_iter().map(|v| vec![v]).collect();
    Ok(Value::AreaValue(column))
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
    fn tocol_row_scan() {
        // Row-by-row: 1,2,3,4,5,6
        let result = area(codcel_tocol(make_2x3(), None, None).unwrap());
        assert_eq!(result.len(), 6);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 2);
        assert_eq!(i(&result[2][0]), 3);
        assert_eq!(i(&result[3][0]), 4);
        assert_eq!(i(&result[4][0]), 5);
        assert_eq!(i(&result[5][0]), 6);
    }

    #[test]
    fn tocol_column_scan() {
        // Column-by-column: 1,4,2,5,3,6
        let result = area(codcel_tocol(make_2x3(), None, Some(true)).unwrap());
        assert_eq!(result.len(), 6);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 4);
        assert_eq!(i(&result[2][0]), 2);
        assert_eq!(i(&result[3][0]), 5);
        assert_eq!(i(&result[4][0]), 3);
        assert_eq!(i(&result[5][0]), 6);
    }

    #[test]
    fn tocol_single_row() {
        let array = vec![vec![Value::I32(1), Value::I32(2), Value::I32(3)]];
        let result = area(codcel_tocol(array, None, None).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 2);
        assert_eq!(i(&result[2][0]), 3);
    }

    #[test]
    fn tocol_single_column() {
        let array = vec![
            vec![Value::I32(1)],
            vec![Value::I32(2)],
            vec![Value::I32(3)],
        ];
        let result = area(codcel_tocol(array, None, None).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 2);
        assert_eq!(i(&result[2][0]), 3);
    }

    #[test]
    fn tocol_ignore_blanks() {
        let array = vec![
            vec![Value::I32(1), Value::String("".to_string()), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::String("".to_string())],
        ];
        let result = area(codcel_tocol(array, Some(1), None).unwrap());
        assert_eq!(result.len(), 4);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 3);
        assert_eq!(i(&result[2][0]), 4);
        assert_eq!(i(&result[3][0]), 5);
    }

    #[test]
    fn tocol_empty_error() {
        let err = codcel_tocol(vec![], None, None).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn tocol_1x1() {
        let result = area(codcel_tocol(vec![vec![Value::I32(42)]], None, None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(i(&result[0][0]), 42);
    }
}
