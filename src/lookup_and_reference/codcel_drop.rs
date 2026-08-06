// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Removes a specified number of rows and/or columns from the beginning or end of an array,
/// like Excel's `DROP`.
///
/// - `rows` is optional. When `None`, all rows are kept. When `Some`, a positive value drops from
///   the top, a negative value drops from the bottom. Zero is invalid (unless `columns` is provided
///   and non-zero).
/// - `columns` is optional. When `None`, all columns are returned. When `Some`, a positive value
///   drops from the left and a negative value drops from the right. Zero is invalid.
///
/// # Errors
/// Returns an error when:
/// - The array is empty.
/// - Both `rows` is zero and `columns` is `None` or zero.
/// - The absolute value of `rows` or `columns` is greater than or equal to the array dimensions
///   (dropping all rows/columns would produce an empty result).
pub fn codcel_drop(
    array: Vec<Vec<Value>>,
    rows: Option<i32>,
    columns: Option<i32>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("DROP: Array cannot be empty".into());
    }

    let total_rows = array.len();
    let total_cols = array[0].len();

    // Validate rows
    if let Some(r) = rows {
        if r == 0 && columns.is_none() {
            return Err("DROP: Rows argument must not be zero when columns is not provided".into());
        }
        let abs_rows = r.unsigned_abs() as usize;
        if r != 0 && abs_rows >= total_rows {
            return Err("DROP: Rows argument would remove all rows".into());
        }
    }

    // Validate columns
    if let Some(cols) = columns {
        if cols == 0 {
            return Err("DROP: Columns argument must not be zero".into());
        }
        let abs_cols = cols.unsigned_abs() as usize;
        if abs_cols >= total_cols {
            return Err("DROP: Columns argument would remove all columns".into());
        }
    }

    // Determine row slice (inverse of TAKE)
    let row_slice: &[Vec<Value>] = match rows {
        None => &array[..],
        Some(0) => &array[..],
        Some(r) if r > 0 => {
            let abs_rows = r.unsigned_abs() as usize;
            &array[abs_rows..]
        }
        Some(r) => {
            let abs_rows = r.unsigned_abs() as usize;
            &array[..total_rows - abs_rows]
        }
    };

    // Determine column range and slice each row
    let result: Vec<Vec<Value>> = match columns {
        None => row_slice.to_vec(),
        Some(cols) => {
            let abs_cols = cols.unsigned_abs() as usize;
            if cols > 0 {
                row_slice
                    .iter()
                    .map(|row| row[abs_cols..].to_vec())
                    .collect()
            } else {
                row_slice
                    .iter()
                    .map(|row| row[..total_cols - abs_cols].to_vec())
                    .collect()
            }
        }
    };

    Ok(Value::AreaValue(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: build a 3x3 array of i32 values {1,2,3;4,5,6;7,8,9}.
    fn make_3x3() -> Vec<Vec<Value>> {
        vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::I32(6)],
            vec![Value::I32(7), Value::I32(8), Value::I32(9)],
        ]
    }

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

    // --- positive rows, all columns (drop from top) ---

    #[test]
    fn drop_first_2_rows() {
        let result = area(codcel_drop(make_3x3(), Some(2), None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 7);
        assert_eq!(i(&result[0][1]), 8);
        assert_eq!(i(&result[0][2]), 9);
    }

    #[test]
    fn drop_first_1_row() {
        let result = area(codcel_drop(make_3x3(), Some(1), None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 4);
        assert_eq!(i(&result[1][0]), 7);
    }

    // --- negative rows, all columns (drop from bottom) ---

    #[test]
    fn drop_last_2_rows() {
        let result = area(codcel_drop(make_3x3(), Some(-2), None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[0][2]), 3);
    }

    #[test]
    fn drop_last_1_row() {
        let result = area(codcel_drop(make_3x3(), Some(-1), None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 4);
    }

    // --- positive rows + positive columns ---

    #[test]
    fn drop_1_row_1_col() {
        // Drop first row and first column: {5,6;8,9}
        let result = area(codcel_drop(make_3x3(), Some(1), Some(1)).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 5);
        assert_eq!(i(&result[0][1]), 6);
        assert_eq!(i(&result[1][0]), 8);
        assert_eq!(i(&result[1][1]), 9);
    }

    // --- negative rows + negative columns ---

    #[test]
    fn drop_last_1_row_last_1_col() {
        // Drop last row and last column: {1,2;4,5}
        let result = area(codcel_drop(make_3x3(), Some(-1), Some(-1)).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[1][0]), 4);
        assert_eq!(i(&result[1][1]), 5);
    }

    #[test]
    fn drop_last_2_rows_last_2_cols() {
        // Drop last 2 rows and last 2 columns: {1}
        let result = area(codcel_drop(make_3x3(), Some(-2), Some(-2)).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 1);
    }

    // --- mixed signs ---

    #[test]
    fn drop_first_1_row_last_1_col() {
        // Drop first row and last column: {4,5;7,8}
        let result = area(codcel_drop(make_3x3(), Some(1), Some(-1)).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 4);
        assert_eq!(i(&result[0][1]), 5);
        assert_eq!(i(&result[1][0]), 7);
        assert_eq!(i(&result[1][1]), 8);
    }

    #[test]
    fn drop_last_1_row_first_1_col() {
        // Drop last row and first column: {2,3;5,6}
        let result = area(codcel_drop(make_3x3(), Some(-1), Some(1)).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 2);
        assert_eq!(i(&result[0][1]), 3);
        assert_eq!(i(&result[1][0]), 5);
        assert_eq!(i(&result[1][1]), 6);
    }

    // --- columns only (rows = 0 with columns provided) ---

    #[test]
    fn drop_0_rows_1_col() {
        // Drop only columns, keep all rows: {2,3;5,6;8,9}
        let result = area(codcel_drop(make_3x3(), Some(0), Some(1)).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 2);
        assert_eq!(i(&result[0][1]), 3);
        assert_eq!(i(&result[2][0]), 8);
        assert_eq!(i(&result[2][1]), 9);
    }

    #[test]
    fn drop_0_rows_negative_1_col() {
        // Drop only last column, keep all rows: {1,2;4,5;7,8}
        let result = area(codcel_drop(make_3x3(), Some(0), Some(-1)).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[2][0]), 7);
        assert_eq!(i(&result[2][1]), 8);
    }

    // --- error: rows = 0 without columns ---

    #[test]
    fn drop_rows_zero_no_columns_error() {
        let err = codcel_drop(make_3x3(), Some(0), None).unwrap_err();
        assert!(err.to_string().contains("must not be zero"));
    }

    // --- error: columns = 0 ---

    #[test]
    fn drop_cols_zero_error() {
        let err = codcel_drop(make_3x3(), Some(1), Some(0)).unwrap_err();
        assert!(err.to_string().contains("must not be zero"));
    }

    // --- error: rows would remove all rows ---

    #[test]
    fn drop_all_rows_error() {
        let err = codcel_drop(make_3x3(), Some(3), None).unwrap_err();
        assert!(err.to_string().contains("remove all rows"));
    }

    #[test]
    fn drop_negative_all_rows_error() {
        let err = codcel_drop(make_3x3(), Some(-3), None).unwrap_err();
        assert!(err.to_string().contains("remove all rows"));
    }

    #[test]
    fn drop_exceeds_rows_error() {
        let err = codcel_drop(make_3x3(), Some(4), None).unwrap_err();
        assert!(err.to_string().contains("remove all rows"));
    }

    // --- error: columns would remove all columns ---

    #[test]
    fn drop_all_cols_error() {
        let err = codcel_drop(make_3x3(), Some(1), Some(3)).unwrap_err();
        assert!(err.to_string().contains("remove all columns"));
    }

    #[test]
    fn drop_negative_all_cols_error() {
        let err = codcel_drop(make_3x3(), Some(1), Some(-3)).unwrap_err();
        assert!(err.to_string().contains("remove all columns"));
    }

    // --- error: empty array ---

    #[test]
    fn drop_empty_array_error() {
        let err = codcel_drop(vec![], Some(1), None).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    // --- single row/column arrays ---

    #[test]
    fn drop_single_row_array_col() {
        let array = vec![vec![Value::I32(10), Value::I32(20), Value::I32(30)]];
        let result = area(codcel_drop(array, Some(0), Some(1)).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 20);
        assert_eq!(i(&result[0][1]), 30);
    }

    #[test]
    fn drop_single_col_array_row() {
        let array = vec![
            vec![Value::I32(10)],
            vec![Value::I32(20)],
            vec![Value::I32(30)],
        ];
        let result = area(codcel_drop(array, Some(1), None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 20);
        assert_eq!(i(&result[1][0]), 30);
    }

    // --- drop all-but-one edge case ---

    #[test]
    fn drop_all_but_one_row() {
        let result = area(codcel_drop(make_3x3(), Some(2), None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(i(&result[0][0]), 7);
    }

    #[test]
    fn drop_all_but_one_col() {
        let result = area(codcel_drop(make_3x3(), Some(0), Some(2)).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 3);
        assert_eq!(i(&result[1][0]), 6);
        assert_eq!(i(&result[2][0]), 9);
    }
}
