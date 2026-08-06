// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::condition::parse_condition;
use crate::match_type_and_compare_macro::compare;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::collections::HashMap;
use std::error::Error;

/// Lowercase a cell value for case-insensitive matching. Non-string values are returned as-is.
fn case_insensitive_cell(value: &Value, value_format: &ValueFormat) -> Value {
    if value.is_string() {
        if let Ok(s) = value.string(value_format) {
            return Value::String(s.to_lowercase());
        }
    }
    value.clone()
}

/// Builds a lowercase header → column-index map from the first row of `database`.
pub(crate) fn build_header_map(
    database: &[Vec<Value>],
    value_format: &ValueFormat,
) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    if let Some(header_row) = database.first() {
        for (idx, header) in header_row.iter().enumerate() {
            if let Ok(name) = header.string(value_format) {
                map.insert(name.trim().to_lowercase(), idx);
            }
        }
    }
    map
}

/// Resolves the D-function `field` argument to a 0-based column index.
/// Accepts a 1-based integer or a header label string (case-insensitive).
pub(crate) fn resolve_field(
    database: &[Vec<Value>],
    field: &Value,
    value_format: &ValueFormat,
) -> Result<usize, Box<dyn Error + Send + Sync>> {
    let header_count = database.first().map(|r| r.len()).unwrap_or(0);

    if let Ok(idx_i32) = field.i32(value_format) {
        if idx_i32 >= 1 && (idx_i32 as usize) <= header_count {
            return Ok((idx_i32 as usize) - 1);
        }
    }

    let label = field.string(value_format)?;
    let headers = build_header_map(database, value_format);
    headers
        .get(&label.trim().to_lowercase())
        .copied()
        .ok_or_else(|| {
            format!(
                "Database function: field '{label}' did not match any database header"
            )
            .into()
        })
}

/// Returns the 0-based indices of data rows (rows 1..n of `database`) that match `criteria`.
///
/// Criteria semantics:
/// - Row 0 of `criteria` is a header row whose labels match `database` headers.
/// - Each subsequent row is an AND of all non-empty cells in that row against the same column.
/// - Multiple criteria rows are OR-combined.
/// - An empty criteria block (only the header row, or all-empty data rows) selects all records.
pub(crate) fn match_db_criteria(
    database: &[Vec<Value>],
    criteria: &[Vec<Value>],
    value_format: &ValueFormat,
) -> Result<Vec<usize>, Box<dyn Error + Send + Sync>> {
    if database.len() < 2 {
        return Ok(Vec::new());
    }
    if criteria.is_empty() {
        return Ok((0..(database.len() - 1)).collect());
    }

    let database_headers = build_header_map(database, value_format);

    // Map criteria column index → database column index, skipping headers we can't match.
    let criteria_header_row = &criteria[0];
    let mut criteria_to_db: Vec<Option<usize>> = Vec::with_capacity(criteria_header_row.len());
    for header in criteria_header_row.iter() {
        if let Ok(name) = header.string(value_format) {
            criteria_to_db.push(
                database_headers
                    .get(&name.trim().to_lowercase())
                    .copied(),
            );
        } else {
            criteria_to_db.push(None);
        }
    }

    let data_rows = &database[1..];

    // No criteria rows beyond the header ⇒ select all.
    if criteria.len() < 2 {
        return Ok((0..data_rows.len()).collect());
    }

    let mut selected: Vec<bool> = vec![false; data_rows.len()];

    for criteria_row in &criteria[1..] {
        // Within one criteria row, all non-empty cells must match (AND).
        let constraints: Vec<(usize, Value)> = criteria_row
            .iter()
            .enumerate()
            .filter_map(|(c_idx, cell)| {
                if cell.is_none() {
                    return None;
                }
                if cell.is_string() {
                    if let Ok(s) = cell.string(value_format) {
                        if s.trim().is_empty() {
                            return None;
                        }
                    }
                }
                let db_col = criteria_to_db.get(c_idx).copied().flatten()?;
                Some((db_col, cell.clone()))
            })
            .collect();

        // An entirely empty criteria row matches everything.
        if constraints.is_empty() {
            return Ok((0..data_rows.len()).collect());
        }

        for (row_idx, row) in data_rows.iter().enumerate() {
            if selected[row_idx] {
                continue;
            }
            let row_matches = constraints.iter().all(|(db_col, criterion)| {
                let cell = match row.get(*db_col) {
                    Some(v) => v.clone(),
                    None => return false,
                };
                let condition_string = match criterion.string(value_format) {
                    Ok(s) => s,
                    Err(_) => return false,
                };
                let (op, rhs_text) = parse_condition(&condition_string);
                let rhs = Value::String(rhs_text.to_lowercase());
                let lhs = case_insensitive_cell(&cell, value_format);
                match compare(&lhs, &rhs, &op, value_format) {
                    Ok(result) => result.bool(value_format).unwrap_or(false),
                    Err(_) => false,
                }
            });
            if row_matches {
                selected[row_idx] = true;
            }
        }
    }

    Ok(selected
        .into_iter()
        .enumerate()
        .filter_map(|(i, sel)| if sel { Some(i) } else { None })
        .collect())
}

/// Collects values of `field_idx` over matched rows, parsed as f64. Non-numeric cells are skipped.
pub(crate) fn collect_numeric_column(
    database: &[Vec<Value>],
    field_idx: usize,
    matched_rows: &[usize],
    value_format: &ValueFormat,
) -> Vec<f64> {
    let data_rows = if database.len() >= 2 { &database[1..] } else { return Vec::new(); };
    matched_rows
        .iter()
        .filter_map(|&i| data_rows.get(i).and_then(|row| row.get(field_idx)))
        .filter_map(|cell| cell.f64(value_format).ok())
        .collect()
}

/// Returns true if the cell is "non-empty" in the Excel COUNTA sense
/// (any value other than None or an empty string).
pub(crate) fn is_non_empty_cell(value: &Value, value_format: &ValueFormat) -> bool {
    if value.is_none() {
        return false;
    }
    if value.is_string() {
        if let Ok(s) = value.string(value_format) {
            return !s.is_empty();
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vf() -> ValueFormat {
        ValueFormat::from_language("en-US")
    }

    fn db() -> Vec<Vec<Value>> {
        // Headers, then 4 data rows.
        vec![
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
            vec![
                Value::String("Dave".into()),
                Value::String("Sales".into()),
                Value::F64(90.0),
            ],
        ]
    }

    #[test]
    fn resolve_field_by_index() {
        let database = db();
        let idx = resolve_field(&database, &Value::I32(3), &vf()).unwrap();
        assert_eq!(idx, 2);
    }

    #[test]
    fn resolve_field_by_header_case_insensitive() {
        let database = db();
        let idx =
            resolve_field(&database, &Value::String("salary".into()), &vf()).unwrap();
        assert_eq!(idx, 2);
    }

    #[test]
    fn resolve_field_invalid_label_errors() {
        let database = db();
        let result =
            resolve_field(&database, &Value::String("Unknown".into()), &vf());
        assert!(result.is_err());
    }

    #[test]
    fn match_db_criteria_equality() {
        let database = db();
        let criteria = vec![
            vec![Value::String("Dept".into())],
            vec![Value::String("Eng".into())],
        ];
        let matched = match_db_criteria(&database, &criteria, &vf()).unwrap();
        assert_eq!(matched, vec![0, 1]);
    }

    #[test]
    fn match_db_criteria_comparison() {
        let database = db();
        let criteria = vec![
            vec![Value::String("Salary".into())],
            vec![Value::String(">=100".into())],
        ];
        let matched = match_db_criteria(&database, &criteria, &vf()).unwrap();
        assert_eq!(matched, vec![0, 2]);
    }

    #[test]
    fn match_db_criteria_and_across_columns() {
        let database = db();
        let criteria = vec![
            vec![
                Value::String("Dept".into()),
                Value::String("Salary".into()),
            ],
            vec![
                Value::String("Eng".into()),
                Value::String(">=100".into()),
            ],
        ];
        let matched = match_db_criteria(&database, &criteria, &vf()).unwrap();
        assert_eq!(matched, vec![0]);
    }

    #[test]
    fn match_db_criteria_or_across_rows() {
        let database = db();
        let criteria = vec![
            vec![Value::String("Dept".into())],
            vec![Value::String("Eng".into())],
            vec![Value::String("Sales".into())],
        ];
        let matched = match_db_criteria(&database, &criteria, &vf()).unwrap();
        assert_eq!(matched, vec![0, 1, 2, 3]);
    }

    #[test]
    fn match_db_criteria_empty_selects_all() {
        let database = db();
        let criteria: Vec<Vec<Value>> = vec![vec![Value::String("Dept".into())]];
        let matched = match_db_criteria(&database, &criteria, &vf()).unwrap();
        assert_eq!(matched, vec![0, 1, 2, 3]);
    }

    #[test]
    fn match_db_criteria_wildcard() {
        let database = db();
        let criteria = vec![
            vec![Value::String("Name".into())],
            vec![Value::String("C*".into())],
        ];
        let matched = match_db_criteria(&database, &criteria, &vf()).unwrap();
        assert_eq!(matched, vec![2]); // Carol
    }

    #[test]
    fn collect_numeric_column_skips_non_numeric() {
        let database = db();
        let numeric = collect_numeric_column(&database, 2, &[0, 1, 2, 3], &vf());
        assert_eq!(numeric, vec![100.0, 80.0, 120.0, 90.0]);
    }

    #[test]
    fn is_non_empty_cell_works() {
        let vf = vf();
        assert!(is_non_empty_cell(&Value::String("a".into()), &vf));
        assert!(!is_non_empty_cell(&Value::String("".into()), &vf));
        assert!(!is_non_empty_cell(&Value::None, &vf));
        assert!(is_non_empty_cell(&Value::F64(1.0), &vf));
    }
}
