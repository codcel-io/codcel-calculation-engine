// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Stacks arrays horizontally (side by side), like Excel's `HSTACK`.
///
/// Row `r` of the result is row `r` of the first array followed by row `r` of
/// the second array, and so on. Every array must have the same number of rows.
///
/// # Divergence from Excel
/// Excel pads shorter arrays with `#N/A`. Codcel rejects ragged input instead:
/// an `#N/A` cell cannot be converted back into a numeric spill without
/// panicking, so a clear error is preferable to an unrepresentable value. A
/// scalar argument is a 1x1 array, so `HSTACK(1, A1:A3)` is ragged and errors.
///
/// # Errors
/// Returns an error when no arrays are supplied, when any array is empty or
/// non-rectangular, or when the arrays do not all have the same number of rows.
pub fn codcel_h_stack(arrays: Vec<Vec<Vec<Value>>>) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if arrays.is_empty() {
        return Err("HSTACK: At least one array is required".into());
    }

    for array in &arrays {
        if array.is_empty() || array[0].is_empty() {
            return Err("HSTACK: Array cannot be empty".into());
        }
        let columns = array[0].len();
        if array.iter().any(|row| row.len() != columns) {
            return Err("HSTACK: Array rows must all be the same length".into());
        }
    }

    let rows = arrays[0].len();
    if arrays.iter().any(|array| array.len() != rows) {
        return Err("HSTACK: All arrays must have the same number of rows".into());
    }

    let width: usize = arrays.iter().map(|array| array[0].len()).sum();

    let mut result: Vec<Vec<Value>> = Vec::with_capacity(rows);
    for row_index in 0..rows {
        let mut row: Vec<Value> = Vec::with_capacity(width);
        for array in &arrays {
            row.extend(array[row_index].iter().cloned());
        }
        result.push(row);
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

    fn column(values: &[i32]) -> Vec<Vec<Value>> {
        values.iter().map(|n| vec![Value::I32(*n)]).collect()
    }

    fn text(value: &str) -> Value {
        Value::String(value.to_string())
    }

    #[test]
    fn h_stack_two_columns() {
        let result = area(codcel_h_stack(vec![column(&[1, 2, 3]), column(&[4, 5, 6])]).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 2);
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(i(&result[0][1]), 4);
        assert_eq!(i(&result[2][0]), 3);
        assert_eq!(i(&result[2][1]), 6);
    }

    #[test]
    fn h_stack_two_matrices() {
        let left = vec![
            vec![Value::I32(1), Value::I32(2)],
            vec![Value::I32(3), Value::I32(4)],
        ];
        let right = vec![
            vec![Value::I32(5), Value::I32(6), Value::I32(7)],
            vec![Value::I32(8), Value::I32(9), Value::I32(10)],
        ];
        let result = area(codcel_h_stack(vec![left, right]).unwrap());
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].len(), 5);
        assert_eq!(i(&result[1][2]), 8);
        assert_eq!(i(&result[1][4]), 10);
    }

    #[test]
    fn h_stack_three_arguments() {
        let result = area(
            codcel_h_stack(vec![
                vec![vec![Value::I32(1), Value::I32(2)]],
                vec![vec![Value::I32(3), Value::I32(4)]],
                vec![vec![Value::I32(5), Value::I32(6)]],
            ])
            .unwrap(),
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 6);
        assert_eq!(i(&result[0][5]), 6);
    }

    #[test]
    fn h_stack_single_argument_is_identity() {
        let result = area(codcel_h_stack(vec![column(&[1, 2, 3])]).unwrap());
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].len(), 1);
        assert_eq!(i(&result[1][0]), 2);
    }

    #[test]
    fn h_stack_scalars_of_equal_height() {
        // =HSTACK("Apple", "Banana", "Cherry")
        let result = area(
            codcel_h_stack(vec![
                vec![vec![text("Apple")]],
                vec![vec![text("Banana")]],
                vec![vec![text("Cherry")]],
            ])
            .unwrap(),
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].len(), 3);
        assert_eq!(s(&result[0][0]), "Apple");
        assert_eq!(s(&result[0][2]), "Cherry");
    }

    #[test]
    fn h_stack_mixed_types_are_preserved() {
        let left = vec![vec![Value::I32(1)], vec![Value::I32(2)]];
        let right = vec![vec![text("a")], vec![text("b")]];
        let result = area(codcel_h_stack(vec![left, right]).unwrap());
        assert_eq!(i(&result[0][0]), 1);
        assert_eq!(s(&result[0][1]), "a");
        assert_eq!(s(&result[1][1]), "b");
    }

    #[test]
    fn h_stack_blank_cells_pass_through() {
        let left = vec![vec![Value::None], vec![Value::I32(2)]];
        let right = vec![vec![Value::I32(3)], vec![Value::None]];
        let result = area(codcel_h_stack(vec![left, right]).unwrap());
        assert!(matches!(result[0][0], Value::None));
        assert!(matches!(result[1][1], Value::None));
    }

    #[test]
    fn h_stack_error_values_pass_through_unchanged() {
        let left = vec![vec![Value::Error(ExcelError::Na)]];
        let right = vec![vec![Value::I32(1)]];
        let result = area(codcel_h_stack(vec![left, right]).unwrap());
        assert!(matches!(result[0][0], Value::Error(ExcelError::Na)));
        assert_eq!(i(&result[0][1]), 1);
    }

    #[test]
    fn h_stack_ragged_rows_error() {
        let err = codcel_h_stack(vec![column(&[1, 2, 3]), column(&[4, 5])]).unwrap_err();
        assert!(err.to_string().contains("same number of rows"));
    }

    #[test]
    fn h_stack_scalar_with_column_is_ragged_error() {
        // Excel returns 1 / #N/A / #N/A here. Codcel deliberately errors.
        let err = codcel_h_stack(vec![vec![vec![Value::I32(1)]], column(&[1, 2, 3])]).unwrap_err();
        assert!(err.to_string().contains("same number of rows"));
    }

    #[test]
    fn h_stack_no_arguments_error() {
        let err = codcel_h_stack(vec![]).unwrap_err();
        assert!(err.to_string().contains("At least one array"));
    }

    #[test]
    fn h_stack_empty_array_error() {
        let err = codcel_h_stack(vec![vec![]]).unwrap_err();
        assert!(err.to_string().contains("empty"));
    }

    #[test]
    fn h_stack_non_rectangular_argument_error() {
        let ragged = vec![vec![Value::I32(1), Value::I32(2)], vec![Value::I32(3)]];
        let err = codcel_h_stack(vec![ragged]).unwrap_err();
        assert!(err.to_string().contains("same length"));
    }
}
