// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::database::criteria_match::{collect_numeric_column, match_db_criteria, resolve_field};
use crate::statistical::codcel_var_dot_p::codcel_var_dot_p;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `DVARP(database, field, criteria)`.
/// Population variance of `field` over matched rows (n divisor).
/// Zero numeric values ⇒ `#NUM!`.
pub fn codcel_dvarp(
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
        return Ok(Value::String("#NUM!".into()));
    }
    Ok(Value::F64(codcel_var_dot_p(values)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vf() -> ValueFormat {
        ValueFormat::from_language("en-US")
    }

    fn db() -> Value {
        Value::AreaValue(vec![
            vec![Value::String("G".into()), Value::String("V".into())],
            vec![Value::String("X".into()), Value::F64(1.0)],
            vec![Value::String("X".into()), Value::F64(2.0)],
            vec![Value::String("X".into()), Value::F64(3.0)],
            vec![Value::String("X".into()), Value::F64(4.0)],
            vec![Value::String("X".into()), Value::F64(5.0)],
        ])
    }

    #[test]
    fn dvarp_basic() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("G".into())],
            vec![Value::String("X".into())],
        ]);
        let result = codcel_dvarp(db(), Value::String("V".into()), crit, &vf()).unwrap();
        if let Value::F64(v) = result {
            // VAR.P of 1..5 = 2.0
            assert!((v - 2.0).abs() < 1e-9);
        } else {
            panic!("expected F64");
        }
    }

    #[test]
    fn dvarp_no_match_returns_num_error() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("G".into())],
            vec![Value::String("Z".into())],
        ]);
        let result = codcel_dvarp(db(), Value::String("V".into()), crit, &vf()).unwrap();
        assert_eq!(result, Value::String("#NUM!".into()));
    }
}
