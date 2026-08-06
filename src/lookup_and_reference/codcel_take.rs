// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Returns a subset of rows and/or columns from the beginning or end of an array, like Excel's
/// `TAKE`.
///
/// - `rows` is optional. When `None`, all rows are returned. When `Some`, a positive value takes
///   from the top, a negative value takes from the bottom. Zero is invalid.
/// - `columns` is optional. When `None`, all columns are returned. When `Some`, a positive value
///   takes from the left and a negative value takes from the right. Zero is invalid.
///
/// # Errors
/// Returns an error when:
/// - The array is empty.
/// - `rows` or `columns` is zero.
/// - The absolute value of `rows` or `columns` exceeds the array dimensions.
pub fn codcel_take(
    array: Vec<Vec<Value>>,
    rows: Option<i32>,
    columns: Option<i32>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("TAKE: Array cannot be empty".into());
    }

    let total_rows = array.len();
    let total_cols = array[0].len();

    // Validate rows
    if let Some(r) = rows {
        if r == 0 {
            return Err("TAKE: Rows argument must not be zero".into());
        }
        let abs_rows = r.unsigned_abs() as usize;
        if abs_rows > total_rows {
            return Err("TAKE: Rows argument exceeds array dimensions".into());
        }
    }

    // Validate columns
    if let Some(cols) = columns {
        if cols == 0 {
            return Err("TAKE: Columns argument must not be zero".into());
        }
        let abs_cols = cols.unsigned_abs() as usize;
        if abs_cols > total_cols {
            return Err("TAKE: Columns argument exceeds array dimensions".into());
        }
    }

    // Determine row slice
    let row_slice: &[Vec<Value>] = match rows {
        None => &array[..],
        Some(r) => {
            let abs_rows = r.unsigned_abs() as usize;
            if r > 0 {
                &array[..abs_rows]
            } else {
                &array[total_rows - abs_rows..]
            }
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
                    .map(|row| row[..abs_cols].to_vec())
                    .collect()
            } else {
                row_slice
                    .iter()
                    .map(|row| row[total_cols - abs_cols..].to_vec())
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

    // --- positive rows, all columns ---

    #[test]
    fn take_first_2_rows() {
        let result = area(codcel_take(make_3x3(), Some(2), None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 4);
        assert_eq!(result[0].len(), 3);
    }

    #[test]
    fn take_first_1_row() {
        let result = area(codcel_take(make_3x3(), Some(1), None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][2]), 3);
    }

    // --- negative rows, all columns ---

    #[test]
    fn take_last_2_rows() {
        let result = area(codcel_take(make_3x3(), Some(-2), None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 4);
        assert_eq!(i(&result[1][0]), 7);
    }

    #[test]
    fn take_last_1_row() {
        let result = area(codcel_take(make_3x3(), Some(-1), None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(i(&result[0][0]), 7);
        assert_eq!(i(&result[0][2]), 9);
    }

    // --- positive rows + positive columns ---

    #[test]
    fn take_2_rows_2_cols() {
        let result = area(codcel_take(make_3x3(), Some(2), Some(2)).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[1][0]), 4);
        assert_eq!(i(&result[1][1]), 5);
    }

    // --- negative rows + negative columns ---

    #[test]
    fn take_last_1_row_last_1_col() {
        let result = area(codcel_take(make_3x3(), Some(-1), Some(-1)).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 9);
    }

    #[test]
    fn take_last_2_rows_last_2_cols() {
        let result = area(codcel_take(make_3x3(), Some(-2), Some(-2)).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 5);
        assert_eq!(i(&result[0][1]), 6);
        assert_eq!(i(&result[1][0]), 8);
        assert_eq!(i(&result[1][1]), 9);
    }

    // --- mixed signs ---

    #[test]
    fn take_first_2_rows_last_1_col() {
        let result = area(codcel_take(make_3x3(), Some(2), Some(-1)).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 3);
        assert_eq!(i(&result[1][0]), 6);
    }

    #[test]
    fn take_last_1_row_first_2_cols() {
        let result = area(codcel_take(make_3x3(), Some(-1), Some(2)).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 7);
        assert_eq!(i(&result[0][1]), 8);
    }

    // --- take all (count equals dimension) ---

    #[test]
    fn take_all_rows() {
        let result = area(codcel_take(make_3x3(), Some(3), None).unwrap());
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn take_all_rows_all_cols() {
        let result = area(codcel_take(make_3x3(), Some(3), Some(3)).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[2][2]), 9);
    }

    // --- error: rows = 0 ---

    #[test]
    fn take_rows_zero_error() {
        let err = codcel_take(make_3x3(), Some(0), None).unwrap_err();
        assert!(err.to_string().contains("must not be zero"));
    }

    // --- error: columns = 0 ---

    #[test]
    fn take_cols_zero_error() {
        let err = codcel_take(make_3x3(), Some(1), Some(0)).unwrap_err();
        assert!(err.to_string().contains("must not be zero"));
    }

    // --- error: rows exceeds array ---

    #[test]
    fn take_rows_exceeds_error() {
        let err = codcel_take(make_3x3(), Some(4), None).unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }

    #[test]
    fn take_negative_rows_exceeds_error() {
        let err = codcel_take(make_3x3(), Some(-4), None).unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }

    // --- error: columns exceeds array ---

    #[test]
    fn take_cols_exceeds_error() {
        let err = codcel_take(make_3x3(), Some(1), Some(4)).unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }

    #[test]
    fn take_negative_cols_exceeds_error() {
        let err = codcel_take(make_3x3(), Some(1), Some(-4)).unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }

    // --- error: empty array ---

    #[test]
    fn take_empty_array_error() {
        let err = codcel_take(vec![], Some(1), None).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    // --- single row/column arrays ---

    #[test]
    fn take_single_row_array() {
        let array = vec![vec![Value::I32(10), Value::I32(20), Value::I32(30)]];
        let result = area(codcel_take(array, Some(1), Some(2)).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 10);
        assert_eq!(i(&result[0][1]), 20);
    }

    #[test]
    fn take_single_col_array() {
        let array = vec![
            vec![Value::I32(10)],
            vec![Value::I32(20)],
            vec![Value::I32(30)],
        ];
        let result = area(codcel_take(array, Some(-2), None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 20);
        assert_eq!(i(&result[1][0]), 30);
    }
}
