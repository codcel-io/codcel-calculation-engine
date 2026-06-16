// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::database::criteria_match::{is_non_empty_cell, match_db_criteria, resolve_field};
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `DCOUNTA(database, field, criteria)`.
/// Counts non-empty cells (any type) in `field` across matched rows. If `field`
/// is omitted, counts matched rows.
pub fn codcel_dcounta(
    database: Value,
    field: Value,
    criteria: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let database = database.area_of_value()?;
    let criteria = criteria.area_of_value()?;
    let matched = match_db_criteria(&database, &criteria, value_format)?;

    let field_omitted = field.is_none()
        || (field.is_string()
            && field.string(value_format).map(|s| s.is_empty()).unwrap_or(false));

    if field_omitted {
        return Ok(Value::I32(matched.len() as i32));
    }

    let field_idx = resolve_field(&database, &field, value_format)?;
    let data_rows = if database.len() >= 2 { &database[1..] } else { &[][..] };
    let count = matched
        .iter()
        .filter_map(|&i| data_rows.get(i).and_then(|row| row.get(field_idx)))
        .filter(|cell| is_non_empty_cell(cell, value_format))
        .count() as i32;

    Ok(Value::I32(count))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vf() -> ValueFormat {
        ValueFormat::from_language("en-US")
    }

    fn db() -> Value {
        Value::AreaValue(vec![
            vec![Value::String("Name".into()), Value::String("Dept".into()), Value::String("Salary".into())],
            vec![Value::String("Alice".into()), Value::String("Eng".into()), Value::F64(100.0)],
            vec![Value::String("Bob".into()), Value::String("Eng".into()), Value::None],
            vec![Value::String("Carol".into()), Value::String("Eng".into()), Value::String("TBD".into())],
        ])
    }

    #[test]
    fn dcounta_counts_text_and_numbers() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("Dept".into())],
            vec![Value::String("Eng".into())],
        ]);
        let result = codcel_dcounta(db(), Value::String("Salary".into()), crit, &vf()).unwrap();
        assert_eq!(result, Value::I32(2));
    }

    #[test]
    fn dcounta_omitted_field() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("Dept".into())],
            vec![Value::String("Eng".into())],
        ]);
        let result = codcel_dcounta(db(), Value::None, crit, &vf()).unwrap();
        assert_eq!(result, Value::I32(3));
    }
}
