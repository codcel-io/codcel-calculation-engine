// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use std::error::Error;

/// Expands an array to specified dimensions, padding with a fill value, like Excel's `EXPAND`.
///
/// - `rows`: target number of rows (must be >= current rows).
/// - `columns`: optional target number of columns (must be >= current columns; defaults to current).
/// - `pad_with`: optional fill value for new cells (defaults to empty string).
///
/// # Errors
/// Returns an error when the array is empty, or target dimensions are smaller than current.
pub fn codcel_expand(
    array: Vec<Vec<Value>>,
    rows: i32,
    columns: Option<i32>,
    pad_with: Option<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("EXPAND: Array cannot be empty".into());
    }

    let current_rows = array.len();
    let current_cols = array[0].len();

    let target_rows = rows as usize;
    let target_cols = columns.map(|c| c as usize).unwrap_or(current_cols);

    if target_rows < current_rows {
        return Err("EXPAND: Target rows must be >= current rows".into());
    }
    if target_cols < current_cols {
        return Err("EXPAND: Target columns must be >= current columns".into());
    }

    let pad = pad_with.unwrap_or(Value::String("".to_string()));

    let mut result: Vec<Vec<Value>> = Vec::with_capacity(target_rows);

    for row_idx in 0..target_rows {
        let mut row = Vec::with_capacity(target_cols);
        if let Some(source_row) = array.get(row_idx) {
            for col_idx in 0..target_cols {
                if let Some(val) = source_row.get(col_idx) {
                    row.push(val.clone());
                } else {
                    row.push(pad.clone());
                }
            }
        } else {
            row.resize(target_cols, pad.clone());
        }
        result.push(row);
    }

    Ok(Value::AreaValue(result))
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

    fn s(v: &Value) -> &str {
        match v {
            Value::String(s) => s.as_str(),
            _ => panic!("expected String"),
        }
    }

    fn f(v: &Value) -> f64 {
        match v {
            Value::F64(n) => *n,
            _ => panic!("expected F64"),
        }
    }

    #[test]
    fn expand_both_dimensions() {
        // [[1,2],[3,4]] expand to 3x4
        let array = vec![
            vec![Value::I32(1), Value::I32(2)],
            vec![Value::I32(3), Value::I32(4)],
        ];
        let result = area(codcel_expand(array, 3, Some(4), None).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 4);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(s(&result[0][2]), "");
        assert_eq!(s(&result[0][3]), "");
        assert_eq!(i(&result[1][0]), 3);
        assert_eq!(i(&result[1][1]), 4);
        assert_eq!(s(&result[2][0]), "");
        assert_eq!(s(&result[2][3]), "");
    }

    #[test]
    fn expand_rows_only() {
        // [[1,2],[3,4]] expand to 4 rows, columns stay 2
        let array = vec![
            vec![Value::I32(1), Value::I32(2)],
            vec![Value::I32(3), Value::I32(4)],
        ];
        let result = area(codcel_expand(array, 4, None, None).unwrap());
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 3);
        assert_eq!(s(&result[2][0]), "");
        assert_eq!(s(&result[3][0]), "");
    }

    #[test]
    fn expand_with_custom_pad() {
        let array = vec![vec![Value::I32(1)]];
        let result = area(codcel_expand(array, 2, Some(3), Some(Value::F64(0.0))).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(f(&result[0][1]), 0.0);
        assert_eq!(f(&result[0][2]), 0.0);
        assert_eq!(f(&result[1][0]), 0.0);
    }

    #[test]
    fn expand_same_dimensions_no_op() {
        let array = vec![
            vec![Value::I32(1), Value::I32(2)],
            vec![Value::I32(3), Value::I32(4)],
        ];
        let result = area(codcel_expand(array, 2, Some(2), None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][1]), 4);
    }

    #[test]
    fn expand_target_rows_smaller_error() {
        let array = vec![
            vec![Value::I32(1)],
            vec![Value::I32(2)],
            vec![Value::I32(3)],
        ];
        let err = codcel_expand(array, 2, None, None).unwrap_err();
        assert!(err.to_string().contains("Target rows"));
    }

    #[test]
    fn expand_target_cols_smaller_error() {
        let array = vec![vec![Value::I32(1), Value::I32(2), Value::I32(3)]];
        let err = codcel_expand(array, 1, Some(2), None).unwrap_err();
        assert!(err.to_string().contains("Target columns"));
    }

    #[test]
    fn expand_empty_error() {
        let err = codcel_expand(vec![], 3, Some(3), None).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }
}
