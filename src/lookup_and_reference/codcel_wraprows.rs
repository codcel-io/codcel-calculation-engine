// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Wraps a flattened array into rows of a specified width, like Excel's `WRAPROWS`.
///
/// The 2D input is first flattened row-by-row, then chunked into rows of `wrap_count` elements.
/// The last row is padded with `pad_with` (default: empty string) if it has fewer elements.
///
/// # Errors
/// Returns an error when the array is empty or `wrap_count` is not positive.
pub fn codcel_wraprows(
    array: Vec<Vec<Value>>,
    wrap_count: i32,
    pad_with: Option<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("WRAPROWS: Array cannot be empty".into());
    }
    if wrap_count <= 0 {
        return Err("WRAPROWS: wrap_count must be a positive integer".into());
    }

    let flat: Vec<Value> = array.into_iter().flat_map(|row| row.into_iter()).collect();
    let wrap = wrap_count as usize;
    let pad = pad_with.unwrap_or(Value::String("".to_string()));

    let mut result: Vec<Vec<Value>> = Vec::new();

    for chunk in flat.chunks(wrap) {
        let mut row = chunk.to_vec();
        while row.len() < wrap {
            row.push(pad.clone());
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

    #[test]
    fn wraprows_exact_fit() {
        // [1,2,3,4,5,6] wrap=3 -> [[1,2,3],[4,5,6]]
        let array = vec![vec![
            Value::I32(1),
            Value::I32(2),
            Value::I32(3),
            Value::I32(4),
            Value::I32(5),
            Value::I32(6),
        ]];
        let result = area(codcel_wraprows(array, 3, None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[0][2]), 3);
        assert_eq!(i(&result[1][0]), 4);
        assert_eq!(i(&result[1][1]), 5);
        assert_eq!(i(&result[1][2]), 6);
    }

    #[test]
    fn wraprows_needs_padding() {
        // [1,2,3,4,5] wrap=3 -> [[1,2,3],[4,5,""]]
        let array = vec![vec![
            Value::I32(1),
            Value::I32(2),
            Value::I32(3),
            Value::I32(4),
            Value::I32(5),
        ]];
        let result = area(codcel_wraprows(array, 3, None).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[1].len(), 3);
        assert_eq!(i(&result[1][0]), 4);
        assert_eq!(i(&result[1][1]), 5);
        assert_eq!(s(&result[1][2]), "");
    }

    #[test]
    fn wraprows_custom_pad() {
        // [1,2,3,4,5] wrap=3, pad=0 -> [[1,2,3],[4,5,0]]
        let array = vec![vec![
            Value::I32(1),
            Value::I32(2),
            Value::I32(3),
            Value::I32(4),
            Value::I32(5),
        ]];
        let result = area(codcel_wraprows(array, 3, Some(Value::I32(0))).unwrap());
        assert_eq!(result[1][2], Value::I32(0));
    }

    #[test]
    fn wraprows_wrap_count_1() {
        // Each element is its own row
        let array = vec![vec![Value::I32(1), Value::I32(2), Value::I32(3)]];
        let result = area(codcel_wraprows(array, 1, None).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 2);
        assert_eq!(i(&result[2][0]), 3);
    }

    #[test]
    fn wraprows_wrap_count_exceeds_total() {
        // Single row, padded
        let array = vec![vec![Value::I32(1), Value::I32(2)]];
        let result = area(codcel_wraprows(array, 5, None).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 5);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(s(&result[0][2]), "");
    }

    #[test]
    fn wraprows_2d_input() {
        // 2x3 input is flattened first: [1,2,3,4,5,6], then wrapped
        let array = vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::I32(6)],
        ];
        let result = area(codcel_wraprows(array, 2, None).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[1][0]), 3);
        assert_eq!(i(&result[1][1]), 4);
        assert_eq!(i(&result[2][0]), 5);
        assert_eq!(i(&result[2][1]), 6);
    }

    #[test]
    fn wraprows_zero_error() {
        let err = codcel_wraprows(vec![vec![Value::I32(1)]], 0, None).unwrap_err();
        assert!(err.to_string().contains("positive"));
    }

    #[test]
    fn wraprows_negative_error() {
        let err = codcel_wraprows(vec![vec![Value::I32(1)]], -1, None).unwrap_err();
        assert!(err.to_string().contains("positive"));
    }

    #[test]
    fn wraprows_empty_error() {
        let err = codcel_wraprows(vec![], 3, None).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }
}
