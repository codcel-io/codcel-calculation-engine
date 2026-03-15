// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use std::error::Error;

/// Returns the number of rows in an array, like Excel's `ROWS`.
pub fn codcel_rows(
    array: Vec<Vec<Value>>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    Ok(Value::I32(array.len() as i32))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i(v: &Value) -> i32 {
        match v {
            Value::I32(n) => *n,
            _ => panic!("expected I32"),
        }
    }

    #[test]
    fn rows_empty() {
        let result = codcel_rows(vec![]).unwrap();
        assert_eq!(i(&result), 0);
    }

    #[test]
    fn rows_1x1() {
        let result = codcel_rows(vec![vec![Value::I32(42)]]).unwrap();
        assert_eq!(i(&result), 1);
    }

    #[test]
    fn rows_3x2() {
        let array = vec![
            vec![Value::I32(1), Value::I32(2)],
            vec![Value::I32(3), Value::I32(4)],
            vec![Value::I32(5), Value::I32(6)],
        ];
        let result = codcel_rows(array).unwrap();
        assert_eq!(i(&result), 3);
    }

    #[test]
    fn rows_single_row() {
        let array = vec![vec![Value::I32(1), Value::I32(2), Value::I32(3)]];
        let result = codcel_rows(array).unwrap();
        assert_eq!(i(&result), 1);
    }

    #[test]
    fn rows_single_column() {
        let array = vec![
            vec![Value::I32(1)],
            vec![Value::I32(2)],
            vec![Value::I32(3)],
        ];
        let result = codcel_rows(array).unwrap();
        assert_eq!(i(&result), 3);
    }
}
