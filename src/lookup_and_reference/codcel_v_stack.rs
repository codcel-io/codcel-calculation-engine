// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Stacks arrays vertically (one on top of another), like Excel's `VSTACK`.
///
/// Every row of every array is appended in order. All arrays must have the same
/// number of columns.
///
/// # Divergence from Excel
/// Excel right-pads narrower rows with `#N/A`. Codcel rejects mismatched widths
/// instead: an `#N/A` cell cannot be converted back into a numeric spill without
/// panicking, so a clear error is preferable to an unrepresentable value. A
/// scalar argument is a 1x1 array, so `VSTACK(A1:C1, "x")` is ragged and errors.
///
/// # Errors
/// Returns an error when no arrays are supplied, when any array is empty, or
/// when the arrays do not all have the same number of columns.
pub fn codcel_v_stack(arrays: Vec<Vec<Vec<Value>>>) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if arrays.is_empty() {
        return Err("VSTACK: At least one array is required".into());
    }

    for array in &arrays {
        if array.is_empty() || array[0].is_empty() {
            return Err("VSTACK: Array cannot be empty".into());
        }
    }

    // A single width check covers both a non-rectangular argument and a
    // mismatch between arguments.
    let columns = arrays[0][0].len();
    for array in &arrays {
        if array.iter().any(|row| row.len() != columns) {
            return Err("VSTACK: All arrays must have the same number of columns".into());
        }
    }

    let height: usize = arrays.iter().map(|array| array.len()).sum();

    let mut result: Vec<Vec<Value>> = Vec::with_capacity(height);
    for array in arrays {
        for row in array {
            result.push(row);
        }
    }

    Ok(Value::AreaValue(result))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::excel_error::ExcelError;

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

    fn row(values: &[i32]) -> Vec<Vec<Value>> {
        vec![values.iter().map(|n| Value::I32(*n)).collect()]
    }

    fn text(value: &str) -> Value {
        Value::String(value.to_string())
    }

    #[test]
    fn v_stack_two_rows() {
        let result = area(codcel_v_stack(vec![row(&[1, 2, 3]), row(&[4, 5, 6])]).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[1][0]), 4);
        assert_eq!(i(&result[1][2]), 6);
    }

    #[test]
    fn v_stack_two_matrices() {
        let top = vec![
            vec![Value::I32(1), Value::I32(2)],
            vec![Value::I32(3), Value::I32(4)],
        ];
        let bottom = vec![
            vec![Value::I32(5), Value::I32(6)],
            vec![Value::I32(7), Value::I32(8)],
            vec![Value::I32(9), Value::I32(10)],
        ];
        let result = area(codcel_v_stack(vec![top, bottom]).unwrap());
        assert_eq!(result.len(), 5);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[2][0]), 5);
        assert_eq!(i(&result[4][1]), 10);
    }

    #[test]
    fn v_stack_three_arguments() {
        let result = area(codcel_v_stack(vec![row(&[1, 2]), row(&[3, 4]), row(&[5, 6])]).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[2][1]), 6);
    }

    #[test]
    fn v_stack_single_argument_is_identity() {
        let result = area(codcel_v_stack(vec![row(&[1, 2, 3])]).unwrap());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(i(&result[0][1]), 2);
    }

    #[test]
    fn v_stack_scalars_of_equal_width() {
        // =VSTACK("Apple", "Banana", "Cherry")
        let result = area(
            codcel_v_stack(vec![
                vec![vec![text("Apple")]],
                vec![vec![text("Banana")]],
                vec![vec![text("Cherry")]],
            ])
            .unwrap(),
        );
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1);
        assert_eq!(s(&result[0][0]), "Apple");
        assert_eq!(s(&result[2][0]), "Cherry");
    }

    #[test]
    fn v_stack_mixed_types_are_preserved() {
        let top = vec![vec![Value::I32(1), Value::I32(2)]];
        let bottom = vec![vec![text("a"), text("b")]];
        let result = area(codcel_v_stack(vec![top, bottom]).unwrap());
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(s(&result[1][0]), "a");
        assert_eq!(s(&result[1][1]), "b");
    }

    #[test]
    fn v_stack_blank_cells_pass_through() {
        let top = vec![vec![Value::None, Value::I32(2)]];
        let bottom = vec![vec![Value::I32(3), Value::None]];
        let result = area(codcel_v_stack(vec![top, bottom]).unwrap());
        assert!(matches!(result[0][0], Value::None));
        assert!(matches!(result[1][1], Value::None));
    }

    #[test]
    fn v_stack_error_values_pass_through_unchanged() {
        let top = vec![vec![Value::Error(ExcelError::Na)]];
        let bottom = vec![vec![Value::I32(1)]];
        let result = area(codcel_v_stack(vec![top, bottom]).unwrap());
        assert!(matches!(result[0][0], Value::Error(ExcelError::Na)));
        assert_eq!(i(&result[1][0]), 1);
    }

    #[test]
    fn v_stack_mismatched_columns_error() {
        let err = codcel_v_stack(vec![row(&[1, 2, 3]), row(&[4, 5])]).unwrap_err();
        assert!(err.to_string().contains("same number of columns"));
    }

    #[test]
    fn v_stack_scalar_with_row_is_ragged_error() {
        // Excel returns a padded row here. Codcel deliberately errors.
        let err = codcel_v_stack(vec![row(&[1, 2, 3]), vec![vec![text("x")]]]).unwrap_err();
        assert!(err.to_string().contains("same number of columns"));
    }

    #[test]
    fn v_stack_no_arguments_error() {
        let err = codcel_v_stack(vec![]).unwrap_err();
        assert!(err.to_string().contains("At least one array"));
    }

    #[test]
    fn v_stack_empty_array_error() {
        let err = codcel_v_stack(vec![vec![]]).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn v_stack_non_rectangular_argument_error() {
        let ragged = vec![vec![Value::I32(1), Value::I32(2)], vec![Value::I32(3)]];
        let err = codcel_v_stack(vec![ragged]).unwrap_err();
        assert!(err.to_string().contains("same number of columns"));
    }
}
