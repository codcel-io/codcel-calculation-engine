// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use std::error::Error;

/// Wraps a flattened array into columns of a specified height, like Excel's `WRAPCOLS`.
///
/// The 2D input is first flattened row-by-row, then filled into columns of `wrap_count` height.
/// Element at position `i` goes to row `i % wrap_count`, column `i / wrap_count`.
/// The last column is padded with `pad_with` (default: empty string) if it has fewer elements.
///
/// # Errors
/// Returns an error when the array is empty or `wrap_count` is not positive.
pub fn codcel_wrapcols(
    array: Vec<Vec<Value>>,
    wrap_count: i32,
    pad_with: Option<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("WRAPCOLS: Array cannot be empty".into());
    }
    if wrap_count <= 0 {
        return Err("WRAPCOLS: wrap_count must be a positive integer".into());
    }

    let flat: Vec<Value> = array.into_iter().flat_map(|row| row.into_iter()).collect();
    let wrap = wrap_count as usize;
    let pad = pad_with.unwrap_or(Value::String("".to_string()));
    let total = flat.len();
    let num_cols = total.div_ceil(wrap); // ceiling division

    // Initialize result: wrap rows x num_cols columns
    let mut result: Vec<Vec<Value>> = vec![Vec::with_capacity(num_cols); wrap];

    for (idx, value) in flat.into_iter().enumerate() {
        let row = idx % wrap;
        result[row].push(value);
    }

    // Pad incomplete last column
    for row in &mut result {
        while row.len() < num_cols {
            row.push(pad.clone());
        }
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

    #[test]
    fn wrapcols_exact_fit() {
        // [1,2,3,4,5,6] wrap=3 -> [[1,4],[2,5],[3,6]]
        let array = vec![vec![
            Value::I32(1),
            Value::I32(2),
            Value::I32(3),
            Value::I32(4),
            Value::I32(5),
            Value::I32(6),
        ]];
        let result = area(codcel_wrapcols(array, 3, None).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 4);
        assert_eq!(i(&result[1][0]), 2);
        assert_eq!(i(&result[1][1]), 5);
        assert_eq!(i(&result[2][0]), 3);
        assert_eq!(i(&result[2][1]), 6);
    }

    #[test]
    fn wrapcols_needs_padding() {
        // [1,2,3,4,5] wrap=3 -> [[1,4],[2,5],[3,""]]
        let array = vec![vec![
            Value::I32(1),
            Value::I32(2),
            Value::I32(3),
            Value::I32(4),
            Value::I32(5),
        ]];
        let result = area(codcel_wrapcols(array, 3, None).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 4);
        assert_eq!(i(&result[1][0]), 2);
        assert_eq!(i(&result[1][1]), 5);
        assert_eq!(i(&result[2][0]), 3);
        assert_eq!(s(&result[2][1]), "");
    }

    #[test]
    fn wrapcols_custom_pad() {
        // [1,2,3,4,5] wrap=3, pad=0 -> [[1,4],[2,5],[3,0]]
        let array = vec![vec![
            Value::I32(1),
            Value::I32(2),
            Value::I32(3),
            Value::I32(4),
            Value::I32(5),
        ]];
        let result = area(codcel_wrapcols(array, 3, Some(Value::I32(0))).unwrap());
        assert_eq!(result[2][1], Value::I32(0));
    }

    #[test]
    fn wrapcols_wrap_count_1() {
        // [1,2,3] wrap=1 -> [[1,2,3]] (single row, each element is a column)
        let array = vec![vec![Value::I32(1), Value::I32(2), Value::I32(3)]];
        let result = area(codcel_wrapcols(array, 1, None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[0][2]), 3);
    }

    #[test]
    fn wrapcols_wrap_count_exceeds_total() {
        // [1,2] wrap=5 -> [[1],[2],[""],[""],[""]]] (single column, padded)
        let array = vec![vec![Value::I32(1), Value::I32(2)]];
        let result = area(codcel_wrapcols(array, 5, None).unwrap());
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 2);
        assert_eq!(s(&result[2][0]), "");
        assert_eq!(s(&result[3][0]), "");
        assert_eq!(s(&result[4][0]), "");
    }

    #[test]
    fn wrapcols_2d_input() {
        // 2x3 flattened first: [1,2,3,4,5,6], wrap=2 -> [[1,3,5],[2,4,6]]
        let array = vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::I32(6)],
        ];
        let result = area(codcel_wrapcols(array, 2, None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 3);
        assert_eq!(i(&result[0][2]), 5);
        assert_eq!(i(&result[1][0]), 2);
        assert_eq!(i(&result[1][1]), 4);
        assert_eq!(i(&result[1][2]), 6);
    }

    #[test]
    fn wrapcols_zero_error() {
        let err = codcel_wrapcols(vec![vec![Value::I32(1)]], 0, None).unwrap_err();
        assert!(err.to_string().contains("positive"));
    }

    #[test]
    fn wrapcols_negative_error() {
        let err = codcel_wrapcols(vec![vec![Value::I32(1)]], -1, None).unwrap_err();
        assert!(err.to_string().contains("positive"));
    }

    #[test]
    fn wrapcols_empty_error() {
        let err = codcel_wrapcols(vec![], 3, None).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }
}
