// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::database::criteria_match::{collect_numeric_column, match_db_criteria, resolve_field};
use crate::maths::codcel_product::codcel_product;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `DPRODUCT(database, field, criteria)`.
/// Product of numeric values in `field` across matched rows.
/// Empty match ⇒ 0.0 (Excel returns 0 for DPRODUCT with no matches,
/// distinct from PRODUCT([]) which returns 1).
pub fn codcel_dproduct(
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
    Ok(Value::F64(codcel_product(values)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vf() -> ValueFormat {
        ValueFormat::from_language("en-US")
    }

    fn db() -> Value {
        Value::AreaValue(vec![
            vec![Value::String("Item".into()), Value::String("Qty".into())],
            vec![Value::String("A".into()), Value::F64(2.0)],
            vec![Value::String("A".into()), Value::F64(3.0)],
            vec![Value::String("A".into()), Value::F64(4.0)],
            vec![Value::String("B".into()), Value::F64(5.0)],
        ])
    }

    #[test]
    fn dproduct_basic() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("Item".into())],
            vec![Value::String("A".into())],
        ]);
        let result = codcel_dproduct(db(), Value::String("Qty".into()), crit, &vf()).unwrap();
        assert_eq!(result, Value::F64(24.0));
    }

    #[test]
    fn dproduct_no_match_returns_zero() {
        let crit = Value::AreaValue(vec![
            vec![Value::String("Item".into())],
            vec![Value::String("Z".into())],
        ]);
        let result = codcel_dproduct(db(), Value::String("Qty".into()), crit, &vf()).unwrap();
        assert_eq!(result, Value::F64(0.0));
    }
}
