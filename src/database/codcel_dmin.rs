// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::database::criteria_match::{collect_numeric_column, match_db_criteria, resolve_field};
use crate::statistical::codcel_min::codcel_min;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `DMIN(database, field, criteria)`.
/// Minimum of `field` over matched rows. Empty match ⇒ 0.0 (matches Excel).
pub fn codcel_dmin(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let database = database.area_of_value()?;
    let criteria = criteria.area_of_value()?;
    let field_idx = resolve_field(&database, &field, value_format)?;
    let matched = match_db_criteria(&database, &criteria, value_format)?;
    let values = collect_numeric_column(&database, field_idx, &matched, value_format);
    if values.is_empty() {
        return Ok(Value::F64(0.0));
    }
    Ok(Value::F64(codcel_min(values)?))
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
        ])
    }

    #[test]
    fn dmin_basic() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("Dept".into())],
            vec![Value::String("Eng".into())],
        ]);
        let result = codcel_dmin(db(), Value::String("Salary".into()), crit, &vf()).unwrap();
        assert_eq!(result, Value::F64(80.0));
    }

    #[test]
    fn dmin_no_match_returns_zero() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("Dept".into())],
            vec![Value::String("HR".into())],
        ]);
        let result = codcel_dmin(db(), Value::String("Salary".into()), crit, &vf()).unwrap();
        assert_eq!(result, Value::F64(0.0));
    }
}
