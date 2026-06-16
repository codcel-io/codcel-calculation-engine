// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::database::criteria_match::{collect_numeric_column, match_db_criteria, resolve_field};
use crate::maths::codcel_sum::codcel_sum;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `DSUM(database, field, criteria)`.
/// Sums the values in `field` column over records matching `criteria`.
/// Returns 0.0 when no records match.
pub fn codcel_dsum(
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
    Ok(Value::F64(codcel_sum(values)?))
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
            vec![Value::String("Alice".into()), Value::String("Eng".into()), Value::F64(100.0)],
            vec![Value::String("Bob".into()), Value::String("Eng".into()), Value::F64(80.0)],
            vec![Value::String("Carol".into()), Value::String("Sales".into()), Value::F64(120.0)],
            vec![Value::String("Dave".into()), Value::String("Sales".into()), Value::F64(90.0)],
        ])
    }

    fn criteria(dept: &str) -> Value {
        Value::AreaValue(vec![
            vec![Value::String("Dept".into())],
            vec![Value::String(dept.into())],
        ])
    }

    #[test]
    fn dsum_basic() {
        // =DSUM(db, "Salary", {"Dept"; "Eng"}) → 100 + 80 = 180
        let result = codcel_dsum(db(), Value::String("Salary".into()), criteria("Eng"), &vf()).unwrap();
        assert_eq!(result, Value::F64(180.0));
    }

    #[test]
    fn dsum_no_match_returns_zero() {
        let result = codcel_dsum(db(), Value::String("Salary".into()), criteria("HR"), &vf()).unwrap();
        assert_eq!(result, Value::F64(0.0));
    }

    #[test]
    fn dsum_by_index() {
        let result = codcel_dsum(db(), Value::I32(3), criteria("Sales"), &vf()).unwrap();
        assert_eq!(result, Value::F64(210.0));
    }

    #[test]
    fn dsum_with_comparison() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("Salary".into())],
            vec![Value::String(">=100".into())],
        ]);
        let result = codcel_dsum(db(), Value::String("Salary".into()), crit, &vf()).unwrap();
        assert_eq!(result, Value::F64(220.0));
    }
}
