// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::lookup_and_reference::codcel_chooserows::resolve_index;
use crate::value::Value;
use std::error::Error;

/// Returns specified columns from an array, like Excel's `CHOOSECOLS`.
///
/// Indices are 1-based. Positive values count from the left, negative from the right.
/// Supports duplicate indices and reordering.
///
/// # Errors
/// Returns an error when the array is empty, no indices are provided, an index is zero,
/// or an index is out of range.
pub fn codcel_choosecols(
    array: Vec<Vec<Value>>,
    col_indices: Vec<i32>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("CHOOSECOLS: Array cannot be empty".into());
    }
    if col_indices.is_empty() {
        return Err("CHOOSECOLS: At least one column index must be provided".into());
    }

    let total_cols = array[0].len();

    let resolved: Vec<usize> = col_indices
        .iter()
        .map(|&idx| resolve_index(idx, total_cols, "CHOOSECOLS", "column"))
        .collect::<Result<Vec<_>, _>>()?;

    let result: Vec<Vec<Value>> = array
        .iter()
        .map(|row| resolved.iter().map(|&col| row[col].clone()).collect())
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

    fn make_3x3() -> Vec<Vec<Value>> {
        vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::I32(6)],
            vec![Value::I32(7), Value::I32(8), Value::I32(9)],
        ]
    }

    #[test]
    fn choosecols_select_cols_1_and_3() {
        let result = area(codcel_choosecols(make_3x3(), vec![1, 3]).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 3);
        assert_eq!(i(&result[1][0]), 4);
        assert_eq!(i(&result[1][1]), 6);
        assert_eq!(i(&result[2][0]), 7);
        assert_eq!(i(&result[2][1]), 9);
    }

    #[test]
    fn choosecols_single_column() {
        let result = area(codcel_choosecols(make_3x3(), vec![2]).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 2);
        assert_eq!(i(&result[1][0]), 5);
        assert_eq!(i(&result[2][0]), 8);
    }

    #[test]
    fn choosecols_negative_index() {
        // -1 = last column
        let result = area(codcel_choosecols(make_3x3(), vec![-1]).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 3);
        assert_eq!(i(&result[1][0]), 6);
        assert_eq!(i(&result[2][0]), 9);
    }

    #[test]
    fn choosecols_reorder() {
        let result = area(codcel_choosecols(make_3x3(), vec![3, 1, 2]).unwrap());
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 3);
        assert_eq!(i(&result[0][1]), 1);
        assert_eq!(i(&result[0][2]), 2);
    }

    #[test]
    fn choosecols_duplicate() {
        let result = area(codcel_choosecols(make_3x3(), vec![1, 1]).unwrap());
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 1);
    }

    #[test]
    fn choosecols_index_zero_error() {
        let err = codcel_choosecols(make_3x3(), vec![0]).unwrap_err();
        assert!(err.to_string().contains("must not be zero"));
    }

    #[test]
    fn choosecols_out_of_range_error() {
        let err = codcel_choosecols(make_3x3(), vec![4]).unwrap_err();
        assert!(err.to_string().contains("out of range"));
    }

    #[test]
    fn choosecols_negative_out_of_range_error() {
        let err = codcel_choosecols(make_3x3(), vec![-4]).unwrap_err();
        assert!(err.to_string().contains("out of range"));
    }

    #[test]
    fn choosecols_empty_array_error() {
        let err = codcel_choosecols(vec![], vec![1]).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn choosecols_no_indices_error() {
        let err = codcel_choosecols(make_3x3(), vec![]).unwrap_err();
        assert!(err.to_string().contains("At least one"));
    }
}
