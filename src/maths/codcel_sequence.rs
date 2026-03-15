// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::maths::check_values::check_value_f64;
use std::error::Error;

/// Excel-compatible `SEQUENCE` that generates an array of sequential numbers.
/// - `rows`: the number of rows to return.
/// - `columns`: optional number of columns (defaults to 1).
/// - `start`: optional starting value (defaults to 1).
/// - `step`: optional increment value (defaults to 1).
///
/// Returns a 2D array of sequential values or an error for invalid inputs.
pub fn codcel_sequence(
    rows: i32,
    columns: Option<i32>,
    start: Option<f64>,
    step: Option<f64>,
) -> Result<Vec<Vec<f64>>, Box<dyn Error + Send + Sync>> {
    if rows <= 0 {
        return Err("SEQUENCE: Rows must be greater than 0".into());
    }

    let columns = columns.unwrap_or(1);
    if columns <= 0 {
        return Err("SEQUENCE: Columns must be greater than 0".into());
    }

    let mut current = start.unwrap_or(1.0);
    let step = step.unwrap_or(1.0);

    check_value_f64("SEQUENCE: start", current)?;
    check_value_f64("SEQUENCE: step", step)?;

    let mut result: Vec<Vec<f64>> = Vec::with_capacity(rows as usize);

    for _ in 0..rows {
        let mut row: Vec<f64> = Vec::with_capacity(columns as usize);
        for _ in 0..columns {
            row.push(current);
            current += step;
        }
        result.push(row);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_defaults_single_column() {
        // =SEQUENCE(4) in US format
        // =SEQUENCE(4) in German format
        let result = codcel_sequence(4, None, None, None).unwrap();
        assert_eq!(result, vec![vec![1.0], vec![2.0], vec![3.0], vec![4.0]]);
    }

    #[test]
    fn test_sequence_multiple_columns_and_step() {
        // =SEQUENCE(2,3,5,2) in US format
        // =SEQUENCE(2;3;5;2) in German format
        let result = codcel_sequence(2, Some(3), Some(5.0), Some(2.0)).unwrap();
        assert_eq!(result, vec![vec![5.0, 7.0, 9.0], vec![11.0, 13.0, 15.0]]);
    }

    #[test]
    fn test_sequence_negative_step() {
        // =SEQUENCE(3,1,10,-2.5) in US format
        // =SEQUENCE(3;1;10;-2,5) in German format
        let result = codcel_sequence(3, Some(1), Some(10.0), Some(-2.5)).unwrap();
        assert_eq!(result, vec![vec![10.0], vec![7.5], vec![5.0]]);
    }

    #[test]
    fn test_sequence_zero_rows_error() {
        // =SEQUENCE(0) in US format (returns #VALUE! error)
        // =SEQUENCE(0) in German format (returns #VALUE! error)
        let result = codcel_sequence(0, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_sequence_zero_columns_error() {
        // =SEQUENCE(3,0) in US format (returns #VALUE! error)
        // =SEQUENCE(3;0) in German format (returns #VALUE! error)
        let result = codcel_sequence(3, Some(0), None, None);
        assert!(result.is_err());
    }
}
