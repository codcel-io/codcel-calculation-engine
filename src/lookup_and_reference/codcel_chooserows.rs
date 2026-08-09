// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Resolves a 1-based index (positive or negative) to a 0-based index.
///
/// - Positive indices: 1 = first element, 2 = second, etc.
/// - Negative indices: -1 = last element, -2 = second-to-last, etc.
/// - Zero is invalid.
///
/// Shared by CHOOSEROWS and CHOOSECOLS.
pub(crate) fn resolve_index(
    idx: i32,
    total: usize,
    func_name: &str,
    dimension: &str,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    if idx == 0 {
        return Err(format!("{func_name}: {dimension} index must not be zero").into());
    }
    let resolved = if idx > 0 {
        (idx - 1) as usize
    } else {
        let abs_idx = idx.unsigned_abs() as usize;
        if abs_idx > total {
            return Err(format!("{func_name}: {dimension} index {idx} is out of range").into());
        }
        total - abs_idx
    };
    if resolved >= total {
        return Err(format!("{func_name}: {dimension} index {idx} is out of range").into());
    }
    Ok(resolved)
}

/// Returns specified rows from an array, like Excel's `CHOOSEROWS`.
///
/// Indices are 1-based. Positive values count from the top, negative from the bottom.
/// Supports duplicate indices and reordering.
///
/// # Errors
/// Returns an error when the array is empty, no indices are provided, an index is zero,
/// or an index is out of range.
pub fn codcel_chooserows(
    array: Vec<Vec<Value>>,
    row_indices: Vec<i32>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("CHOOSEROWS: Array cannot be empty".into());
    }
    if row_indices.is_empty() {
        return Err("CHOOSEROWS: At least one row index must be provided".into());
    }

    let total_rows = array.len();

    let resolved: Vec<usize> = row_indices
        .iter()
        .map(|&idx| resolve_index(idx, total_rows, "CHOOSEROWS", "row"))
        .collect::<Result<Vec<_>, _>>()?;

    let result: Vec<Vec<Value>> = resolved
        .iter()
        .map(|&row_idx| array[row_idx].clone())
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
    fn chooserows_select_rows_1_and_3() {
        let result = area(codcel_chooserows(make_3x3(), vec![1, 3]).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[0][2]), 3);
        assert_eq!(i(&result[1][0]), 7);
        assert_eq!(i(&result[1][1]), 8);
        assert_eq!(i(&result[1][2]), 9);
    }

    #[test]
    fn chooserows_negative_index() {
        // -1 = last row
        let result = area(codcel_chooserows(make_3x3(), vec![-1]).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(i(&result[0][0]), 7);
        assert_eq!(i(&result[0][1]), 8);
        assert_eq!(i(&result[0][2]), 9);
    }

    #[test]
    fn chooserows_reorder() {
        let result = area(codcel_chooserows(make_3x3(), vec![3, 1]).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 7);
        assert_eq!(i(&result[1][0]), 1);
    }

    #[test]
    fn chooserows_duplicate() {
        let result = area(codcel_chooserows(make_3x3(), vec![1, 1]).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 1);
    }

    #[test]
    fn chooserows_negative_2() {
        // -2 = second-to-last row
        let result = area(codcel_chooserows(make_3x3(), vec![-2]).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(i(&result[0][0]), 4);
    }

    #[test]
    fn chooserows_mixed_positive_negative() {
        let result = area(codcel_chooserows(make_3x3(), vec![1, -1]).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 7);
    }

    #[test]
    fn chooserows_index_zero_error() {
        let err = codcel_chooserows(make_3x3(), vec![0]).unwrap_err();
        assert!(err.to_string().contains("must not be zero"));
    }

    #[test]
    fn chooserows_out_of_range_error() {
        let err = codcel_chooserows(make_3x3(), vec![4]).unwrap_err();
        assert!(err.to_string().contains("out of range"));
    }

    #[test]
    fn chooserows_negative_out_of_range_error() {
        let err = codcel_chooserows(make_3x3(), vec![-4]).unwrap_err();
        assert!(err.to_string().contains("out of range"));
    }

    #[test]
    fn chooserows_empty_array_error() {
        let err = codcel_chooserows(vec![], vec![1]).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn chooserows_no_indices_error() {
        let err = codcel_chooserows(make_3x3(), vec![]).unwrap_err();
        assert!(err.to_string().contains("At least one"));
    }

    #[test]
    fn chooserows_single_row_array() {
        let array = vec![vec![Value::I32(10), Value::I32(20)]];
        let result = area(codcel_chooserows(array, vec![1]).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(i(&result[0][0]), 10);
        assert_eq!(i(&result[0][1]), 20);
    }
}
