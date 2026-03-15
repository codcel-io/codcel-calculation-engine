// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use std::error::Error;

/// Returns a range that is offset from a reference position within the array,
/// like Excel's `OFFSET`.
///
/// The array represents the full data context (e.g., the entire sheet).
/// `ref_row` and `ref_col` indicate the 0-based position of the reference
/// cell within that array. The offsets are applied relative to this position.
///
/// - `rows` is the number of rows to offset from the reference position.
///   Positive moves down, negative moves up.
/// - `cols` is the number of columns to offset from the reference position.
///   Positive moves right, negative moves left.
/// - `height` is optional. When `None`, defaults to 1 (single cell).
///   Must be greater than zero if provided.
/// - `width` is optional. When `None`, defaults to 1 (single cell).
///   Must be greater than zero if provided.
/// - `ref_row` is the 0-based row index of the reference cell within the array.
/// - `ref_col` is the 0-based column index of the reference cell within the array.
///
/// # Errors
/// Returns an error when:
/// - The array is empty.
/// - The computed start position is before the beginning of the array.
/// - The result range exceeds the array dimensions.
/// - Height or width is zero or negative.
pub fn codcel_offset(
    array: Vec<Vec<Value>>,
    rows: i32,
    cols: i32,
    height: Option<i32>,
    width: Option<i32>,
    ref_row: i32,
    ref_col: i32,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("OFFSET: Array cannot be empty".into());
    }

    let total_rows = array.len() as i32;
    let total_cols = array[0].len() as i32;

    let result_height = match height {
        Some(h) => {
            if h <= 0 {
                return Err("OFFSET: Height must be greater than zero".into());
            }
            h
        }
        None => 1,
    };

    let result_width = match width {
        Some(w) => {
            if w <= 0 {
                return Err("OFFSET: Width must be greater than zero".into());
            }
            w
        }
        None => 1,
    };

    let start_row = ref_row + rows;
    let start_col = ref_col + cols;

    if start_row < 0 {
        return Err("OFFSET: Row offset would move before the start of the array".into());
    }
    if start_col < 0 {
        return Err("OFFSET: Column offset would move before the start of the array".into());
    }
    if start_row + result_height > total_rows {
        return Err("OFFSET: Result range exceeds array row dimensions".into());
    }
    if start_col + result_width > total_cols {
        return Err("OFFSET: Result range exceeds array column dimensions".into());
    }

    let sr = start_row as usize;
    let sc = start_col as usize;
    let rh = result_height as usize;
    let rw = result_width as usize;

    let result: Vec<Vec<Value>> = array[sr..sr + rh]
        .iter()
        .map(|row| row[sc..sc + rw].to_vec())
        .collect();

    Ok(Value::AreaValue(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_4x4() -> Vec<Vec<Value>> {
        vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3), Value::I32(4)],
            vec![Value::I32(5), Value::I32(6), Value::I32(7), Value::I32(8)],
            vec![Value::I32(9), Value::I32(10), Value::I32(11), Value::I32(12)],
            vec![Value::I32(13), Value::I32(14), Value::I32(15), Value::I32(16)],
        ]
    }

    fn make_3x3() -> Vec<Vec<Value>> {
        vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::I32(6)],
            vec![Value::I32(7), Value::I32(8), Value::I32(9)],
        ]
    }

    fn make_5x3() -> Vec<Vec<Value>> {
        vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::I32(6)],
            vec![Value::I32(7), Value::I32(8), Value::I32(9)],
            vec![Value::I32(10), Value::I32(11), Value::I32(12)],
            vec![Value::I32(13), Value::I32(14), Value::I32(15)],
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

    // --- Basic offset with no resize (ref at origin) ---

    #[test]
    fn offset_no_resize_from_origin() {
        // OFFSET(origin, 0, 0) with ref at (0,0), default height/width = 1
        let result = area(codcel_offset(make_3x3(), 0, 0, Some(3), Some(3), 0, 0).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[2][2]), 9);
    }

    // --- Offset with resize ---

    #[test]
    fn offset_1_row_1_col_with_resize() {
        // OFFSET(4x4, 1, 1, 2, 2) => {6,7;10,11}
        let result = area(codcel_offset(make_4x4(), 1, 1, Some(2), Some(2), 0, 0).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 6);
        assert_eq!(i(&result[0][1]), 7);
        assert_eq!(i(&result[1][0]), 10);
        assert_eq!(i(&result[1][1]), 11);
    }

    #[test]
    fn offset_2_rows_0_cols_with_height() {
        // OFFSET(4x4, 2, 0, 2, 4) => {9,10,11,12;13,14,15,16}
        let result = area(codcel_offset(make_4x4(), 2, 0, Some(2), Some(4), 0, 0).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 4);
        assert_eq!(i(&result[0][0]), 9);
        assert_eq!(i(&result[0][3]), 12);
        assert_eq!(i(&result[1][0]), 13);
        assert_eq!(i(&result[1][3]), 16);
    }

    #[test]
    fn offset_single_cell() {
        // OFFSET(4x4, 2, 3, 1, 1) => {12}
        let result = area(codcel_offset(make_4x4(), 2, 3, Some(1), Some(1), 0, 0).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 12);
    }

    #[test]
    fn offset_with_only_height_specified() {
        // OFFSET(4x4, 0, 0, 2, None) => first 2 rows, 1 col (default width=1)
        let result = area(codcel_offset(make_4x4(), 0, 0, Some(2), None, 0, 0).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 5);
    }

    #[test]
    fn offset_with_only_width_specified() {
        // OFFSET(4x4, 0, 0, None, 2) => 1 row (default height=1), first 2 cols
        let result = area(codcel_offset(make_4x4(), 0, 0, None, Some(2), 0, 0).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
    }

    // --- Default height/width = 1 (single cell) ---

    #[test]
    fn offset_default_single_cell() {
        // OFFSET(3x3, 1, 2) with no height/width => single cell at (1,2)
        let result = area(codcel_offset(make_3x3(), 1, 2, None, None, 0, 0).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 6);
    }

    // --- Negative offsets with ref_row/ref_col ---

    #[test]
    fn offset_negative_row_with_ref_position() {
        // OFFSET(A5, -2, 0) where A5 is at ref_row=4 in a 5x3 array
        // start_row = 4 + (-2) = 2, default height=1, width=1
        // Should return the value at row 2, col 0 = 7
        let result = area(codcel_offset(make_5x3(), -2, 0, None, None, 4, 0).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 7);
    }

    #[test]
    fn offset_negative_col_with_ref_position() {
        // OFFSET(C1, 0, -2) where C1 is at ref_col=2 in a 5x3 array
        // start_col = 2 + (-2) = 0, default height=1, width=1
        // Should return the value at row 0, col 0 = 1
        let result = area(codcel_offset(make_5x3(), 0, -2, None, None, 0, 2).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 1);
    }

    #[test]
    fn offset_negative_both_with_ref_position() {
        // OFFSET(C5, -3, -1) where C5 is at ref_row=4, ref_col=2 in 5x3
        // start_row = 4-3 = 1, start_col = 2-1 = 1
        // Should return value at (1,1) = 5
        let result = area(codcel_offset(make_5x3(), -3, -1, None, None, 4, 2).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 5);
    }

    #[test]
    fn offset_with_ref_and_height_width() {
        // OFFSET(A1, 1, 0, 3, 2) with ref at (0,0) in a 5x3 array
        // start_row = 0+1 = 1, height=3, width=2
        // Returns rows 1..4, cols 0..2 => {4,5;7,8;10,11}
        let result = area(codcel_offset(make_5x3(), 1, 0, Some(3), Some(2), 0, 0).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 4);
        assert_eq!(i(&result[0][1]), 5);
        assert_eq!(i(&result[1][0]), 7);
        assert_eq!(i(&result[2][1]), 11);
    }

    // --- Error cases ---

    #[test]
    fn offset_empty_array_error() {
        let err = codcel_offset(vec![], 0, 0, None, None, 0, 0).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn offset_negative_row_exceeds_error() {
        // ref_row=0, rows=-1 => start_row = -1 < 0
        let err = codcel_offset(make_3x3(), -1, 0, None, None, 0, 0).unwrap_err();
        assert!(err.to_string().contains("before the start"));
    }

    #[test]
    fn offset_negative_col_exceeds_error() {
        // ref_col=0, cols=-1 => start_col = -1 < 0
        let err = codcel_offset(make_3x3(), 0, -1, None, None, 0, 0).unwrap_err();
        assert!(err.to_string().contains("before the start"));
    }

    #[test]
    fn offset_exceeds_rows_error() {
        // ref_row=0, rows=2, default height=1 => start_row(2)+height(1) = 3 = total_rows => OK
        // But rows=3, height=1 => 3+1=4 > 3
        let err = codcel_offset(make_3x3(), 3, 0, Some(1), None, 0, 0).unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }

    #[test]
    fn offset_exceeds_cols_error() {
        let err = codcel_offset(make_3x3(), 0, 3, None, Some(1), 0, 0).unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }

    #[test]
    fn offset_zero_height_error() {
        let err = codcel_offset(make_3x3(), 0, 0, Some(0), None, 0, 0).unwrap_err();
        assert!(err.to_string().contains("greater than zero"));
    }

    #[test]
    fn offset_negative_height_error() {
        let err = codcel_offset(make_3x3(), 0, 0, Some(-1), None, 0, 0).unwrap_err();
        assert!(err.to_string().contains("greater than zero"));
    }

    #[test]
    fn offset_zero_width_error() {
        let err = codcel_offset(make_3x3(), 0, 0, None, Some(0), 0, 0).unwrap_err();
        assert!(err.to_string().contains("greater than zero"));
    }

    #[test]
    fn offset_negative_width_error() {
        let err = codcel_offset(make_3x3(), 0, 0, None, Some(-1), 0, 0).unwrap_err();
        assert!(err.to_string().contains("greater than zero"));
    }

    #[test]
    fn offset_height_exceeds_bounds_error() {
        let err = codcel_offset(make_4x4(), 1, 0, Some(4), None, 0, 0).unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }

    #[test]
    fn offset_width_exceeds_bounds_error() {
        let err = codcel_offset(make_4x4(), 0, 1, None, Some(4), 0, 0).unwrap_err();
        assert!(err.to_string().contains("exceeds"));
    }

    // --- Edge cases ---

    #[test]
    fn offset_single_row_array() {
        let array = vec![vec![Value::I32(10), Value::I32(20), Value::I32(30)]];
        let result = area(codcel_offset(array, 0, 1, Some(1), Some(2), 0, 0).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 20);
        assert_eq!(i(&result[0][1]), 30);
    }

    #[test]
    fn offset_single_column_array() {
        let array = vec![
            vec![Value::I32(10)],
            vec![Value::I32(20)],
            vec![Value::I32(30)],
        ];
        let result = area(codcel_offset(array, 1, 0, Some(2), Some(1), 0, 0).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 20);
        assert_eq!(i(&result[1][0]), 30);
    }

    #[test]
    fn offset_full_extent_explicit() {
        let result = area(codcel_offset(make_3x3(), 0, 0, Some(3), Some(3), 0, 0).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[2][2]), 9);
    }

    #[test]
    fn offset_last_cell() {
        // Get the very last cell of a 4x4 array
        let result = area(codcel_offset(make_4x4(), 3, 3, Some(1), Some(1), 0, 0).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 16);
    }

    #[test]
    fn offset_with_nonzero_ref_no_offset() {
        // ref at (2,1) in 4x4, offset (0,0), height=2, width=2
        // Returns rows 2..4, cols 1..3 => {10,11;14,15}
        let result = area(codcel_offset(make_4x4(), 0, 0, Some(2), Some(2), 2, 1).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 10);
        assert_eq!(i(&result[0][1]), 11);
        assert_eq!(i(&result[1][0]), 14);
        assert_eq!(i(&result[1][1]), 15);
    }
}
