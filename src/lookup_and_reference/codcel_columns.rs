// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Returns the number of columns in an array, like Excel's `COLUMNS`.
///
/// # Errors
/// None — returns 0 for an empty array.
pub fn codcel_columns(array: Vec<Vec<Value>>) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Ok(Value::I32(0));
    }
    Ok(Value::I32(array[0].len() as i32))
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
    fn columns_empty() {
        let result = codcel_columns(vec![]).unwrap();
        assert_eq!(i(&result), 0);
    }

    #[test]
    fn columns_1x1() {
        let result = codcel_columns(vec![vec![Value::I32(42)]]).unwrap();
        assert_eq!(i(&result), 1);
    }

    #[test]
    fn columns_3x2() {
        let array = vec![
            vec![Value::I32(1), Value::I32(2)],
            vec![Value::I32(3), Value::I32(4)],
            vec![Value::I32(5), Value::I32(6)],
        ];
        let result = codcel_columns(array).unwrap();
        assert_eq!(i(&result), 2);
    }

    #[test]
    fn columns_single_row() {
        let array = vec![vec![Value::I32(1), Value::I32(2), Value::I32(3)]];
        let result = codcel_columns(array).unwrap();
        assert_eq!(i(&result), 3);
    }

    #[test]
    fn columns_single_column() {
        let array = vec![
            vec![Value::I32(1)],
            vec![Value::I32(2)],
            vec![Value::I32(3)],
        ];
        let result = codcel_columns(array).unwrap();
        assert_eq!(i(&result), 1);
    }
}
