// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::collections::HashMap;
use std::error::Error;

/// Represents a single group produced by GROUPBY.
pub struct GroupEntry {
    /// The grouping key values (one per row_fields column).
    pub key: Vec<Value>,
    /// The value rows belonging to this group (each row has one Value per values column).
    pub values: Vec<Vec<Value>>,
}

/// The result of the core grouping operation, before aggregation.
/// Aggregation requires calling the lambda (async), so it happens in `groupby_value`.
pub struct GroupByData {
    /// The grouped entries, in order.
    pub groups: Vec<GroupEntry>,
    /// All value rows (for computing grand totals).
    pub all_values: Vec<Vec<Value>>,
    /// Sort order: 0=no sort, 1=ascending, -1=descending.
    pub sort_order: Option<i32>,
    /// Total depth: 0=none, 1=grand totals, 2=grand+subtotals, -1=only grand, -2=only subtotals.
    pub total_depth: Option<i32>,
    /// Number of value columns (for assembling output).
    pub num_value_cols: usize,
}

/// Core grouping logic for Excel's GROUPBY function.
///
/// Groups rows from `row_fields` and `values` together. Both arrays must have the same number
/// of rows. The grouping key is formed from the row_fields columns for each row.
///
/// This function performs grouping, filtering, header extraction, and sorting of groups.
/// Aggregation is NOT performed here because it requires calling the lambda (async).
///
/// # Parameters
/// - `row_fields`: 2D array of grouping key columns (rows x key_cols)
/// - `values`: 2D array of value columns to aggregate (rows x val_cols)
/// - `field_headers`: 0=no headers, 1=first row is header, 2=generate headers, 3=first row is header + generate
/// - `total_depth`: Controls grand totals and subtotals
/// - `sort_order`: 0=no sort, 1=ascending, -1=descending
/// - `filter_array`: Optional boolean filter; rows where filter is false are excluded
pub fn codcel_groupby(
    row_fields: Vec<Vec<Value>>,
    values: Vec<Vec<Value>>,
    field_headers: Option<i32>,
    total_depth: Option<i32>,
    sort_order: Option<i32>,
    filter_array: Option<Vec<bool>>,
) -> Result<GroupByData, Box<dyn Error + Send + Sync>> {
    if row_fields.is_empty() || values.is_empty() {
        return Err("GROUPBY: row_fields and values cannot be empty".into());
    }

    if row_fields.len() != values.len() {
        return Err(format!(
            "GROUPBY: row_fields has {} rows but values has {} rows; they must be equal",
            row_fields.len(),
            values.len()
        )
        .into());
    }

    let num_value_cols = values[0].len();

    // When field_headers is None (omitted), auto-detect: if the first row of values
    // contains all strings while subsequent rows contain numbers, treat as having headers.
    // Determine if first row is a header that should be stripped from input data.
    // When field_headers is None (omitted), auto-detect: if the first row of values
    // contains all strings while subsequent rows contain numbers, treat as having headers.
    let field_headers = match field_headers {
        Some(v) => v,
        None => {
            if row_fields.len() > 1 && has_header_row(&values) {
                1 // Auto-detected: first row is header, strip it from data
            } else {
                0 // No headers detected
            }
        }
    };

    // Strip header row from data if field_headers is 1 or 3 (first row contains headers).
    // Note: Headers are NOT included in the output — in the codcel transpiled context,
    // each cell formula references a specific position in the result array, and the
    // transpiler handles header positioning separately.
    let (data_row_fields, data_values) = if field_headers == 1 || field_headers == 3 {
        if row_fields.len() < 2 {
            return Err("GROUPBY: field_headers indicates headers present, but data has only 1 row".into());
        }
        (
            row_fields[1..].to_vec(),
            values[1..].to_vec(),
        )
    } else {
        (row_fields, values)
    };

    // Apply filter_array if provided
    let (filtered_row_fields, filtered_values) = if let Some(filter) = &filter_array {
        let mut rf = Vec::new();
        let mut v = Vec::new();
        for (i, include) in filter.iter().enumerate() {
            // Skip header row offset if headers were extracted
            let data_idx = if (field_headers == 1 || field_headers == 3) && i > 0 {
                i - 1
            } else if field_headers == 1 || field_headers == 3 {
                continue; // skip the header row in the filter
            } else {
                i
            };
            if data_idx < data_row_fields.len() && *include {
                rf.push(data_row_fields[data_idx].clone());
                v.push(data_values[data_idx].clone());
            }
        }
        (rf, v)
    } else {
        (data_row_fields, data_values)
    };

    // Save all values for grand total computation
    let all_values = filtered_values.clone();

    // Group by row_fields — use a Vec to preserve insertion order, with a HashMap for key lookup
    let mut group_map: HashMap<Vec<Value>, usize> = HashMap::new();
    let mut groups: Vec<GroupEntry> = Vec::new();

    for (i, key_row) in filtered_row_fields.iter().enumerate() {
        let key = key_row.clone();
        if let Some(&group_idx) = group_map.get(&key) {
            groups[group_idx].values.push(filtered_values[i].clone());
        } else {
            let group_idx = groups.len();
            group_map.insert(key.clone(), group_idx);
            groups.push(GroupEntry {
                key,
                values: vec![filtered_values[i].clone()],
            });
        }
    }

    // Sort groups if sort_order is specified
    if let Some(order) = sort_order {
        if order != 0 {
            groups.sort_by(|a, b| {
                for (va, vb) in a.key.iter().zip(b.key.iter()) {
                    let cmp = compare_values(va, vb);
                    if cmp != std::cmp::Ordering::Equal {
                        return if order < 0 { cmp.reverse() } else { cmp };
                    }
                }
                std::cmp::Ordering::Equal
            });
        }
    }

    Ok(GroupByData {
        groups,
        all_values,
        sort_order,
        total_depth: Some(total_depth.unwrap_or(0)),
        num_value_cols,
    })
}

/// Compare two Values for sorting purposes.
fn compare_values(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::I32(a), Value::I32(b)) => a.cmp(b),
        (Value::F64(a), Value::F64(b)) => a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal),
        (Value::I32(a), Value::F64(b)) => {
            (*a as f64).partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
        }
        (Value::F64(a), Value::I32(b)) => {
            a.partial_cmp(&(*b as f64)).unwrap_or(std::cmp::Ordering::Equal)
        }
        (Value::String(a), Value::String(b)) => a.to_lowercase().cmp(&b.to_lowercase()),
        (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
        // For mixed types, use a type ordering: None < Bool < Number < String
        _ => type_order(a).cmp(&type_order(b)),
    }
}

/// Auto-detect if the first row of values looks like a header row.
/// Returns true if the first row contains all strings and at least one subsequent row
/// contains a numeric value in the same column position.
fn has_header_row(values: &[Vec<Value>]) -> bool {
    if values.len() < 2 {
        return false;
    }
    let first_row = &values[0];
    // Check: first row must have all strings
    let first_row_all_strings = first_row.iter().all(|v| matches!(v, Value::String(_)));
    if !first_row_all_strings {
        return false;
    }
    // Check: at least one subsequent row has a numeric value in some column
    for row in &values[1..] {
        for v in row {
            if matches!(v, Value::I32(_) | Value::F64(_)) {
                return true;
            }
        }
    }
    false
}

fn type_order(v: &Value) -> u8 {
    match v {
        Value::None => 0,
        Value::Bool(_) => 1,
        Value::I32(_) | Value::F64(_) => 2,
        Value::String(_) => 3,
        _ => 4,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i(v: i32) -> Value {
        Value::I32(v)
    }

    fn s(v: &str) -> Value {
        Value::String(v.to_string())
    }

    fn f(v: f64) -> Value {
        Value::F64(v)
    }

    /// Helper: build a simple dataset
    /// Fruit | Region | Sales
    /// Apple | East   | 10
    /// Banana| West   | 20
    /// Apple | West   | 30
    /// Banana| East   | 40
    /// Apple | East   | 50
    fn make_test_data() -> (Vec<Vec<Value>>, Vec<Vec<Value>>) {
        let row_fields = vec![
            vec![s("Apple")],
            vec![s("Banana")],
            vec![s("Apple")],
            vec![s("Banana")],
            vec![s("Apple")],
        ];
        let values = vec![
            vec![i(10)],
            vec![i(20)],
            vec![i(30)],
            vec![i(40)],
            vec![i(50)],
        ];
        (row_fields, values)
    }

    fn make_test_data_with_headers() -> (Vec<Vec<Value>>, Vec<Vec<Value>>) {
        let row_fields = vec![
            vec![s("Fruit")],  // header
            vec![s("Apple")],
            vec![s("Banana")],
            vec![s("Apple")],
            vec![s("Banana")],
            vec![s("Apple")],
        ];
        let values = vec![
            vec![s("Sales")],  // header
            vec![i(10)],
            vec![i(20)],
            vec![i(30)],
            vec![i(40)],
            vec![i(50)],
        ];
        (row_fields, values)
    }

    #[test]
    fn basic_grouping() {
        let (rf, v) = make_test_data();
        let result = codcel_groupby(rf, v, None, None, None, None).unwrap();

        assert_eq!(result.groups.len(), 2);
        // Apple group: rows 0, 2, 4
        assert_eq!(result.groups[0].key, vec![s("Apple")]);
        assert_eq!(result.groups[0].values.len(), 3);
        assert_eq!(result.groups[0].values[0], vec![i(10)]);
        assert_eq!(result.groups[0].values[1], vec![i(30)]);
        assert_eq!(result.groups[0].values[2], vec![i(50)]);
        // Banana group: rows 1, 3
        assert_eq!(result.groups[1].key, vec![s("Banana")]);
        assert_eq!(result.groups[1].values.len(), 2);
    }

    #[test]
    fn grouping_with_headers() {
        let (rf, v) = make_test_data_with_headers();
        let result = codcel_groupby(rf, v, Some(1), None, None, None).unwrap();

        // Headers are stripped from input but not included in output
        assert_eq!(result.groups.len(), 2);
        // Data should exclude the header row
        assert_eq!(result.groups[0].key, vec![s("Apple")]);
        assert_eq!(result.groups[0].values.len(), 3);
    }

    #[test]
    fn sort_ascending() {
        let (rf, v) = make_test_data();
        let result = codcel_groupby(rf, v, None, None, Some(1), None).unwrap();

        // Apple < Banana alphabetically
        assert_eq!(result.groups[0].key, vec![s("Apple")]);
        assert_eq!(result.groups[1].key, vec![s("Banana")]);
    }

    #[test]
    fn sort_descending() {
        let (rf, v) = make_test_data();
        let result = codcel_groupby(rf, v, None, None, Some(-1), None).unwrap();

        // Descending: Banana first
        assert_eq!(result.groups[0].key, vec![s("Banana")]);
        assert_eq!(result.groups[1].key, vec![s("Apple")]);
    }

    #[test]
    fn filter_array() {
        let (rf, v) = make_test_data();
        // Only include rows 0, 2, 4 (Apple rows)
        let filter = vec![true, false, true, false, true];
        let result = codcel_groupby(rf, v, None, None, None, Some(filter)).unwrap();

        assert_eq!(result.groups.len(), 1);
        assert_eq!(result.groups[0].key, vec![s("Apple")]);
        assert_eq!(result.groups[0].values.len(), 3);
    }

    #[test]
    fn multi_column_key() {
        let row_fields = vec![
            vec![s("Apple"), s("East")],
            vec![s("Apple"), s("West")],
            vec![s("Apple"), s("East")],
            vec![s("Banana"), s("East")],
        ];
        let values = vec![
            vec![i(10)],
            vec![i(20)],
            vec![i(30)],
            vec![i(40)],
        ];
        let result = codcel_groupby(row_fields, values, None, None, None, None).unwrap();

        assert_eq!(result.groups.len(), 3);
        assert_eq!(result.groups[0].key, vec![s("Apple"), s("East")]);
        assert_eq!(result.groups[0].values.len(), 2); // rows 0, 2
        assert_eq!(result.groups[1].key, vec![s("Apple"), s("West")]);
        assert_eq!(result.groups[1].values.len(), 1);
        assert_eq!(result.groups[2].key, vec![s("Banana"), s("East")]);
        assert_eq!(result.groups[2].values.len(), 1);
    }

    #[test]
    fn multi_column_values() {
        let row_fields = vec![
            vec![s("Apple")],
            vec![s("Banana")],
            vec![s("Apple")],
        ];
        let values = vec![
            vec![i(10), f(1.5)],
            vec![i(20), f(2.5)],
            vec![i(30), f(3.5)],
        ];
        let result = codcel_groupby(row_fields, values, None, None, None, None).unwrap();

        assert_eq!(result.num_value_cols, 2);
        assert_eq!(result.groups[0].values[0], vec![i(10), f(1.5)]);
        assert_eq!(result.groups[0].values[1], vec![i(30), f(3.5)]);
    }

    #[test]
    fn empty_input_errors() {
        let result = codcel_groupby(vec![], vec![], None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn mismatched_rows_errors() {
        let rf = vec![vec![s("A")], vec![s("B")]];
        let v = vec![vec![i(1)]];
        let result = codcel_groupby(rf, v, None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn single_group() {
        let rf = vec![vec![s("A")], vec![s("A")], vec![s("A")]];
        let v = vec![vec![i(1)], vec![i(2)], vec![i(3)]];
        let result = codcel_groupby(rf, v, None, None, None, None).unwrap();

        assert_eq!(result.groups.len(), 1);
        assert_eq!(result.groups[0].values.len(), 3);
    }

    #[test]
    fn all_unique_groups() {
        let rf = vec![vec![s("A")], vec![s("B")], vec![s("C")]];
        let v = vec![vec![i(1)], vec![i(2)], vec![i(3)]];
        let result = codcel_groupby(rf, v, None, None, None, None).unwrap();

        assert_eq!(result.groups.len(), 3);
        for group in &result.groups {
            assert_eq!(group.values.len(), 1);
        }
    }

    #[test]
    fn total_depth_preserved() {
        let (rf, v) = make_test_data();
        let result = codcel_groupby(rf, v, None, Some(1), None, None).unwrap();
        assert_eq!(result.total_depth, Some(1));
        assert_eq!(result.all_values.len(), 5); // all data rows available for grand total
    }

    #[test]
    fn numeric_key_sorting() {
        let rf = vec![vec![i(3)], vec![i(1)], vec![i(2)]];
        let v = vec![vec![s("c")], vec![s("a")], vec![s("b")]];
        let result = codcel_groupby(rf, v, None, None, Some(1), None).unwrap();

        assert_eq!(result.groups[0].key, vec![i(1)]);
        assert_eq!(result.groups[1].key, vec![i(2)]);
        assert_eq!(result.groups[2].key, vec![i(3)]);
    }

    #[test]
    fn auto_detect_headers() {
        // When field_headers is None, auto-detect based on first row being strings
        // and subsequent rows having numbers
        let (rf, v) = make_test_data_with_headers();
        let result = codcel_groupby(rf, v, None, None, None, None).unwrap();

        // Should auto-detect headers and strip the header row from data
        assert_eq!(result.groups.len(), 2);
        assert_eq!(result.groups[0].key, vec![s("Apple")]);
        assert_eq!(result.groups[0].values.len(), 3);
    }

    #[test]
    fn no_auto_detect_when_no_string_header() {
        // When values first row is numeric, no auto-detect
        let (rf, v) = make_test_data();
        let result = codcel_groupby(rf, v, None, None, None, None).unwrap();

        assert_eq!(result.groups.len(), 2);
    }
}
