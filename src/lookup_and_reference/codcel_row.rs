// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use std::error::Error;

/// Returns the row number of a reference, like Excel's `ROW`.
///
/// Most ROW calls are resolved at transpile time to literal integers.
/// This runtime fallback handles edge cases with non-reference arguments.
pub fn codcel_row(
    _array: Vec<Vec<Value>>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    Ok(Value::I32(0))
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
    fn row_empty() {
        let result = codcel_row(vec![]).unwrap();
        assert_eq!(i(&result), 0);
    }

    #[test]
    fn row_1x1() {
        let result = codcel_row(vec![vec![Value::I32(42)]]).unwrap();
        assert_eq!(i(&result), 0);
    }
}
