// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::database::criteria_match::{match_db_criteria, resolve_field};
use crate::excel_error::{err_to_box, ExcelError};
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `DGET(database, field, criteria)`.
/// Returns the single value at `(matched_row, field)`.
/// As in Excel, fails with `#VALUE!` if no row matches and `#NUM!` if more than one does.
pub fn codcel_dget(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let database = database.into_area_of_value()?;
    let criteria = criteria.into_area_of_value()?;
    let field_idx = resolve_field(&database, &field, value_format)?;
    let matched = match_db_criteria(&database, &criteria, value_format)?;

    match matched.len() {
        0 => Err(err_to_box(ExcelError::Value)),
        1 => {
            let row_idx = matched[0];
            let data_rows = &database[1..];
            data_rows
                .get(row_idx)
                .and_then(|row| row.get(field_idx))
                .cloned()
                .ok_or_else(|| "DGET: matched row missing field column".into())
        }
        _ => Err(err_to_box(ExcelError::Num)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vf() -> ValueFormat {
        ValueFormat::from_language("en-US")
    }

    fn db() -> Value {
        Value::AreaValue(vec![
            vec![
                Value::String("Name".into()),
                Value::String("Dept".into()),
                Value::String("Salary".into()),
            ],
            vec![
                Value::String("Alice".into()),
                Value::String("Eng".into()),
                Value::F64(100.0),
            ],
            vec![
                Value::String("Bob".into()),
                Value::String("Eng".into()),
                Value::F64(80.0),
            ],
            vec![
                Value::String("Carol".into()),
                Value::String("Sales".into()),
                Value::F64(120.0),
            ],
        ])
    }

    #[test]
    fn dget_unique_match() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("Name".into())],
            vec![Value::String("Alice".into())],
        ]);
        let result = codcel_dget(db(), Value::String("Salary".into()), crit, &vf()).unwrap();
        assert_eq!(result, Value::F64(100.0));
    }

    #[test]
    fn dget_multiple_matches_is_a_num_error() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("Dept".into())],
            vec![Value::String("Eng".into())],
        ]);
        let error = codcel_dget(db(), Value::String("Salary".into()), crit, &vf()).unwrap_err();
        assert_eq!(ExcelError::from_err(error.as_ref()), ExcelError::Num);
    }

    #[test]
    fn dget_no_match_is_a_value_error() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("Dept".into())],
            vec![Value::String("HR".into())],
        ]);
        let error = codcel_dget(db(), Value::String("Salary".into()), crit, &vf()).unwrap_err();
        assert_eq!(ExcelError::from_err(error.as_ref()), ExcelError::Value);
    }
}
