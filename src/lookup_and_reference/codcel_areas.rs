// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Returns the number of areas in a reference, like Excel's `AREAS`.
///
/// When the transpiler encounters AREAS with a MultipleArea parameter,
/// it packages the areas as a `Value::VecValue(vec![...])`.
/// Each element in the vec represents one contiguous area.
pub fn codcel_areas(
    reference: Value,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match reference {
        Value::VecValue(vec) => Ok(Value::I32(vec.len() as i32)),
        _ => Ok(Value::I32(1)),
    }
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
    fn areas_single_area() {
        // AREAS(A1:B2) => 1
        let input = Value::VecValue(vec![
            Value::AreaValue(vec![
                vec![Value::I32(1), Value::I32(2)],
                vec![Value::I32(3), Value::I32(4)],
            ]),
        ]);
        assert_eq!(i(&codcel_areas(input).unwrap()), 1);
    }

    #[test]
    fn areas_two_areas() {
        // AREAS((A1:B2,C3:D4)) => 2
        let input = Value::VecValue(vec![
            Value::AreaValue(vec![
                vec![Value::I32(1), Value::I32(2)],
                vec![Value::I32(3), Value::I32(4)],
            ]),
            Value::AreaValue(vec![
                vec![Value::I32(5), Value::I32(6)],
                vec![Value::I32(7), Value::I32(8)],
            ]),
        ]);
        assert_eq!(i(&codcel_areas(input).unwrap()), 2);
    }

    #[test]
    fn areas_three_areas() {
        // AREAS((A1:B2,C3:D4,E5:F6)) => 3
        let input = Value::VecValue(vec![
            Value::AreaValue(vec![vec![Value::I32(1)]]),
            Value::AreaValue(vec![vec![Value::I32(2)]]),
            Value::AreaValue(vec![vec![Value::I32(3)]]),
        ]);
        assert_eq!(i(&codcel_areas(input).unwrap()), 3);
    }

    #[test]
    fn areas_non_vec_value() {
        // If a single non-VecValue is passed, it counts as 1 area
        let input = Value::AreaValue(vec![vec![Value::I32(42)]]);
        assert_eq!(i(&codcel_areas(input).unwrap()), 1);
    }

    #[test]
    fn areas_single_cell() {
        // Single cell value => 1 area
        let input = Value::I32(42);
        assert_eq!(i(&codcel_areas(input).unwrap()), 1);
    }
}
