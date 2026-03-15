// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::lookup_and_reference::codcel_tocol::is_blank;
use crate::value::Value;
use std::error::Error;

fn is_row_empty(row: &[Value]) -> bool {
    row.iter().all(is_blank)
}

fn is_col_empty(array: &[Vec<Value>], col: usize) -> bool {
    array
        .iter()
        .all(|row| col >= row.len() || is_blank(&row[col]))
}

/// Trims empty rows and columns from the edges of a 2D array, like Excel's `TRIMRANGE`.
///
/// - `trim_rows`: controls row trimming:
///   0 = no row trimming
///   1 = trim leading blank rows only
///   2 = trim trailing blank rows only
///   3 = trim both leading and trailing blank rows (default)
///
/// - `trim_columns`: controls column trimming:
///   0 = no column trimming
///   1 = trim leading blank columns only
///   2 = trim trailing blank columns only
///   3 = trim both leading and trailing blank columns (default)
///
/// # Errors
/// Returns an error when the array is empty.
pub fn codcel_trimrange(
    array: Vec<Vec<Value>>,
    trim_rows: Option<i32>,
    trim_columns: Option<i32>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("TRIMRANGE: Array cannot be empty".into());
    }

    let trim_rows = trim_rows.unwrap_or(3);
    let trim_columns = trim_columns.unwrap_or(3);

    let total_rows = array.len();
    let total_cols = array[0].len();

    // Determine row bounds based on trim_rows
    let trim_leading_rows = trim_rows == 1 || trim_rows == 3;
    let trim_trailing_rows = trim_rows == 2 || trim_rows == 3;

    let first_row = if trim_leading_rows {
        match (0..total_rows).find(|&r| !is_row_empty(&array[r])) {
            Some(r) => r,
            None => return Ok(Value::AreaValue(vec![vec![Value::String("".to_string())]])),
        }
    } else {
        0
    };

    let last_row = if trim_trailing_rows {
        match (0..total_rows).rev().find(|&r| !is_row_empty(&array[r])) {
            Some(r) => r,
            None => return Ok(Value::AreaValue(vec![vec![Value::String("".to_string())]])),
        }
    } else {
        total_rows - 1
    };

    // Determine column bounds based on trim_columns
    let trim_leading_cols = trim_columns == 1 || trim_columns == 3;
    let trim_trailing_cols = trim_columns == 2 || trim_columns == 3;

    let first_col = if trim_leading_cols {
        (0..total_cols)
            .find(|&c| !is_col_empty(&array[first_row..=last_row], c))
            .unwrap_or(0)
    } else {
        0
    };

    let last_col = if trim_trailing_cols {
        (0..total_cols)
            .rev()
            .find(|&c| !is_col_empty(&array[first_row..=last_row], c))
            .unwrap_or(total_cols - 1)
    } else {
        total_cols - 1
    };

    let result: Vec<Vec<Value>> = array[first_row..=last_row]
        .iter()
        .map(|row| row[first_col..=last_col].to_vec())
        .collect();

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

    fn e() -> Value {
        Value::String("".to_string())
    }

    // Default: trim_rows=3, trim_columns=3 (trim both leading and trailing for rows and cols)
    #[test]
    fn trimrange_default_trims_all_edges() {
        let array = vec![
            vec![e(), e(), e()],
            vec![e(), Value::I32(1), Value::I32(2)],
            vec![e(), e(), e()],
        ];
        let result = area(codcel_trimrange(array, None, None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
    }

    #[test]
    fn trimrange_empty_cols_left_right() {
        let array = vec![
            vec![e(), Value::I32(1), e()],
            vec![e(), Value::I32(2), e()],
        ];
        let result = area(codcel_trimrange(array, None, None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 2);
    }

    #[test]
    fn trimrange_both() {
        let array = vec![
            vec![e(), e(), e()],
            vec![e(), Value::I32(1), e()],
            vec![e(), e(), e()],
        ];
        let result = area(codcel_trimrange(array, None, None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 1);
    }

    #[test]
    fn trimrange_no_trim_needed() {
        let array = vec![
            vec![Value::I32(1), Value::I32(2)],
            vec![Value::I32(3), Value::I32(4)],
        ];
        let result = area(codcel_trimrange(array, None, None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][1]), 4);
    }

    #[test]
    fn trimrange_all_empty() {
        let array = vec![vec![e(), e()], vec![e(), e()]];
        let result = area(codcel_trimrange(array, None, None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(s(&result[0][0]), "");
    }

    #[test]
    fn trimrange_single_cell() {
        let result = area(codcel_trimrange(vec![vec![Value::I32(42)]], None, None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 42);
    }

    #[test]
    fn trimrange_empty_array_error() {
        let err = codcel_trimrange(vec![], None, None).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn trimrange_preserves_inner_blanks() {
        let array = vec![
            vec![Value::I32(1), e(), Value::I32(3)],
            vec![e(), e(), e()],
            vec![Value::I32(7), e(), Value::I32(9)],
        ];
        let result = area(codcel_trimrange(array, None, None).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(s(&result[1][1]), "");
        assert_eq!(i(&result[2][2]), 9);
    }

    // trim_rows=3 (both), trim_columns=0 (no column trimming)
    #[test]
    fn trimrange_trim_rows_only() {
        let array = vec![
            vec![e(), e(), e()],
            vec![e(), Value::I32(1), e()],
            vec![e(), e(), e()],
        ];
        let result = area(codcel_trimrange(array, Some(3), Some(0)).unwrap());
        assert_eq!(result.len(), 1); // only row 1 remains
        assert_eq!(result[0].len(), 3); // all 3 columns kept
        assert_eq!(s(&result[0][0]), "");
        assert_eq!(i(&result[0][1]), 1);
        assert_eq!(s(&result[0][2]), "");
    }

    // trim_rows=0 (no row trimming), trim_columns=3 (both)
    #[test]
    fn trimrange_trim_cols_only() {
        let array = vec![
            vec![e(), e(), e()],
            vec![e(), Value::I32(1), e()],
            vec![e(), e(), e()],
        ];
        let result = area(codcel_trimrange(array, Some(0), Some(3)).unwrap());
        assert_eq!(result.len(), 3); // all 3 rows kept
        assert_eq!(result[0].len(), 1); // only col 1 remains
        assert_eq!(s(&result[0][0]), "");
        assert_eq!(i(&result[1][0]), 1);
        assert_eq!(s(&result[2][0]), "");
    }

    // trim_rows=0, trim_columns=0: no trimming at all
    #[test]
    fn trimrange_no_trim() {
        let array = vec![
            vec![e(), e(), e()],
            vec![e(), Value::I32(1), e()],
            vec![e(), e(), e()],
        ];
        let result = area(codcel_trimrange(array, Some(0), Some(0)).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[1][1]), 1);
    }

    // trim_rows=1 (leading only), trim_columns=3 (both)
    #[test]
    fn trimrange_trim_leading_rows_only() {
        let array = vec![
            vec![e(), e(), e()],
            vec![e(), Value::I32(1), e()],
            vec![e(), e(), e()],
        ];
        let result = area(codcel_trimrange(array, Some(1), Some(3)).unwrap());
        assert_eq!(result.len(), 2); // rows 1 and 2 kept (trailing empty not trimmed)
        assert_eq!(result[0].len(), 1); // only col 1
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(s(&result[1][0]), "");
    }

    // trim_rows=2 (trailing only), trim_columns=3 (both)
    #[test]
    fn trimrange_trim_trailing_rows_only() {
        let array = vec![
            vec![e(), e(), e()],
            vec![e(), Value::I32(1), e()],
            vec![e(), e(), e()],
        ];
        let result = area(codcel_trimrange(array, Some(2), Some(3)).unwrap());
        assert_eq!(result.len(), 2); // rows 0 and 1 kept (leading empty not trimmed)
        assert_eq!(result[0].len(), 1); // only col 1
        assert_eq!(s(&result[0][0]), "");
        assert_eq!(i(&result[1][0]), 1);
    }

    // trim_rows=3, trim_columns=1 (leading cols only)
    #[test]
    fn trimrange_trim_leading_cols_only() {
        let array = vec![
            vec![e(), Value::I32(1), e()],
            vec![e(), Value::I32(2), e()],
        ];
        let result = area(codcel_trimrange(array, Some(3), Some(1)).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2); // cols 1 and 2 (trailing empty not trimmed)
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(s(&result[0][1]), "");
    }

    // trim_rows=3, trim_columns=2 (trailing cols only)
    #[test]
    fn trimrange_trim_trailing_cols_only() {
        let array = vec![
            vec![e(), Value::I32(1), e()],
            vec![e(), Value::I32(2), e()],
        ];
        let result = area(codcel_trimrange(array, Some(3), Some(2)).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2); // cols 0 and 1 (leading empty not trimmed)
        assert_eq!(s(&result[0][0]), "");
        assert_eq!(i(&result[0][1]), 1);
    }
}
