// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::database::criteria_match::{collect_numeric_column, match_db_criteria, resolve_field};
use crate::statistical::codcel_st_dev_s::codcel_st_dev_s;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `DSTDEV(database, field, criteria)`.
/// Sample standard deviation of `field` over matched rows (n-1 divisor).
/// Fewer than 2 numeric values ⇒ `#DIV/0!`.
pub fn codcel_dstdev(
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
    if values.len() < 2 {
        return Ok(Value::String("#DIV/0!".into()));
    }
    Ok(Value::F64(codcel_st_dev_s(values)?))
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
            vec![Value::String("X".into()), Value::F64(2.0)],
            vec![Value::String("X".into()), Value::F64(4.0)],
            vec![Value::String("X".into()), Value::F64(4.0)],
            vec![Value::String("X".into()), Value::F64(4.0)],
            vec![Value::String("X".into()), Value::F64(5.0)],
            vec![Value::String("X".into()), Value::F64(5.0)],
            vec![Value::String("X".into()), Value::F64(7.0)],
            vec![Value::String("X".into()), Value::F64(9.0)],
        ])
    }

    #[test]
    fn dstdev_basic() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("G".into())],
            vec![Value::String("X".into())],
        ]);
        let result = codcel_dstdev(db(), Value::String("V".into()), crit, &vf()).unwrap();
        if let Value::F64(v) = result {
            // STDEV.S of {2,4,4,4,5,5,7,9} ≈ 2.138089935299395
            assert!((v - 2.138089935299395).abs() < 1e-9);
        } else {
            panic!("expected F64");
        }
    }

    #[test]
    fn dstdev_single_returns_div_zero() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("V".into())],
            vec![Value::String("=2".into())],
        ]);
        let result = codcel_dstdev(db(), Value::String("V".into()), crit, &vf()).unwrap();
        assert_eq!(result, Value::String("#DIV/0!".into()));
    }
}
