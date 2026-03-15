// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use std::error::Error;

/// Transposes rows and columns of a 2D array, like Excel's `TRANSPOSE`.
///
/// Rows become columns and columns become rows.
///
/// # Errors
/// Returns an error when the array is empty.
pub fn codcel_transpose(
    array: Vec<Vec<Value>>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("TRANSPOSE: Array cannot be empty".into());
    }

    let col_count = array[0].len();
    let mut transposed: Vec<Vec<Value>> = vec![Vec::with_capacity(array.len()); col_count];

    for row in &array {
        for (col_idx, value) in row.iter().enumerate() {
            transposed[col_idx].push(value.clone());
        }
    }

    Ok(Value::AreaValue(transposed))
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

    fn make_3x3() -> Vec<Vec<Value>> {
        vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::I32(6)],
            vec![Value::I32(7), Value::I32(8), Value::I32(9)],
        ]
    }

    #[test]
    fn transpose_1x1() {
        let result = area(codcel_transpose(vec![vec![Value::I32(42)]]).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 42);
    }

    #[test]
    fn transpose_3x3() {
        let result = area(codcel_transpose(make_3x3()).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 3);
        // First column of input becomes first row of output
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 4);
        assert_eq!(i(&result[0][2]), 7);
        // Second column of input becomes second row of output
        assert_eq!(i(&result[1][0]), 2);
        assert_eq!(i(&result[1][1]), 5);
        assert_eq!(i(&result[1][2]), 8);
        // Third column of input becomes third row of output
        assert_eq!(i(&result[2][0]), 3);
        assert_eq!(i(&result[2][1]), 6);
        assert_eq!(i(&result[2][2]), 9);
    }

    #[test]
    fn transpose_2x3_non_square() {
        // 2 rows x 3 cols -> 3 rows x 2 cols
        let array = vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3)],
            vec![Value::I32(4), Value::I32(5), Value::I32(6)],
        ];
        let result = area(codcel_transpose(array).unwrap());
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
    fn transpose_row_to_column() {
        // 1x3 row -> 3x1 column
        let array = vec![vec![Value::I32(1), Value::I32(2), Value::I32(3)]];
        let result = area(codcel_transpose(array).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 2);
        assert_eq!(i(&result[2][0]), 3);
    }

    #[test]
    fn transpose_column_to_row() {
        // 3x1 column -> 1x3 row
        let array = vec![
            vec![Value::I32(1)],
            vec![Value::I32(2)],
            vec![Value::I32(3)],
        ];
        let result = area(codcel_transpose(array).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 2);
        assert_eq!(i(&result[0][2]), 3);
    }

    #[test]
    fn transpose_mixed_types() {
        let array = vec![
            vec![Value::String("a".to_string()), Value::I32(1)],
            vec![Value::String("b".to_string()), Value::I32(2)],
        ];
        let result = area(codcel_transpose(array).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 2);
        assert_eq!(s(&result[0][0]), "a");
        assert_eq!(s(&result[0][1]), "b");
        assert_eq!(i(&result[1][0]), 1);
        assert_eq!(i(&result[1][1]), 2);
    }

    #[test]
    fn transpose_empty_error() {
        let err = codcel_transpose(vec![]).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }
}
