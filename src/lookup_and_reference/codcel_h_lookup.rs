// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::excel_error::{err_to_box, ExcelError};
use crate::lookup_and_reference::lookup_match::{excel_equals, LookupMatcher};
use crate::value::Value;
use std::cmp::Ordering;
use std::error::Error;

/// Performs a horizontal lookup against the first row of `table_array`, like Excel's `HLOOKUP`.
///
/// `row_index_num` is 1-based and selects the row to return from the column that matches
/// `lookup_value`. This is the transpose of [`codcel_v_lookup`](super::codcel_v_lookup) and
/// shares its matching rules.
///
/// When `range_lookup` is `Some(false)` an exact match is required, and the lookup value may
/// contain wildcards: `*` matches any sequence of characters, `?` matches any single character,
/// and `~` escapes them (`~*`, `~?`, `~~`). Matching is case-insensitive and `2` matches `2.0`,
/// but numbers never match text and booleans never match numbers. The first matching column wins.
///
/// When `range_lookup` is `None` or `Some(true)` the first row is treated as sorted ascending and
/// a binary search returns the last column whose value is less than or equal to `lookup_value`.
/// Wildcards are not applied in this mode, matching Excel. If the row turns out not to be cleanly
/// ordered — it holds error values or mixed incomparable types — the search degrades to a full
/// linear scan rather than returning a binary-search artefact.
///
/// `table_array` is expected to be rectangular, as an Excel range always is. A row shorter than
/// the first yields a blank for the missing cell instead of invalidating the whole table.
///
/// # Errors
/// - `#VALUE!` when `row_index_num` is less than 1.
/// - `#REF!` when `row_index_num` exceeds the table's height, including when the table is empty.
/// - `#N/A` when no column matches.
pub fn codcel_h_lookup(
    lookup_value: Value,
    table_array: Vec<Vec<Value>>,
    row_index_num: i32,
    range_lookup: Option<bool>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Excel validates the row index before it looks at any data. Checked before the cast,
    // otherwise a negative index wraps to a huge `usize`.
    if row_index_num < 1 {
        return Err(err_to_box(ExcelError::Value));
    }
    let row_index_num = row_index_num as usize;

    // An empty table has height 0, so every row index is out of range and falls into the same
    // `#REF!` branch.
    if row_index_num > table_array.len() {
        return Err(err_to_box(ExcelError::Ref));
    }

    let keys = &table_array[0];
    let result_row = &table_array[row_index_num - 1];

    let column = if range_lookup.unwrap_or(true) {
        approximate_h_lookup(&lookup_value, keys)
    } else {
        exact_h_lookup(&lookup_value, keys)?
    };

    column
        .map(|index| cell_or_blank(result_row, index))
        .ok_or_else(|| err_to_box(ExcelError::Na))
}

/// The index of the first key matching `lookup_value` exactly, honouring wildcards.
fn exact_h_lookup(
    lookup_value: &Value,
    keys: &[Value],
) -> Result<Option<usize>, Box<dyn Error + Send + Sync>> {
    // Built once, not once per column: a wildcard lookup compiles its regex here.
    let matcher = LookupMatcher::new(lookup_value)?;
    Ok(keys.iter().position(|key| matcher.matches(key)))
}

/// The index of the last key less than or equal to `lookup_value`, assuming `keys` ascends.
fn approximate_h_lookup(lookup_value: &Value, keys: &[Value]) -> Option<usize> {
    // Partition point: `low` ends as the number of leading keys that are <= the lookup value,
    // so `low - 1` is the last column of a run of duplicates. That is Excel's tie-break for
    // approximate matches.
    let mut low = 0usize;
    let mut high = keys.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match key_cmp(&keys[mid], lookup_value) {
            Some(Ordering::Greater) => high = mid,
            Some(Ordering::Less | Ordering::Equal) => low = mid + 1,
            // The key is an error value or a type that cannot be ordered against the lookup
            // value. The row is therefore not the ascending ladder a binary search needs, and
            // halving on an unordered probe would silently return an arbitrary column. Restart
            // as a linear scan, which stays correct for a sorted row that merely has a few
            // unusable cells sprinkled through it.
            None => return linear_approximate_h_lookup(lookup_value, keys),
        }
    }

    (low > 0).then(|| low - 1)
}

/// Full scan keeping the last column whose key is less than or equal to the lookup value. Keys
/// that cannot be compared are skipped rather than terminating the scan.
fn linear_approximate_h_lookup(lookup_value: &Value, keys: &[Value]) -> Option<usize> {
    let mut best_match_column: Option<usize> = None;

    for (index, key) in keys.iter().enumerate() {
        if matches!(
            key_cmp(key, lookup_value),
            Some(Ordering::Less | Ordering::Equal)
        ) {
            best_match_column = Some(index);
        }
    }

    best_match_column
}

/// Orders a key against the lookup value.
///
/// Returns `None` when the two values are not comparable, which for `Value` includes any pairing
/// involving `Value::Error` and any mismatched pair of types. `excel_equals` is consulted first
/// so that a case-differing text key, or a `2` against a `2.0`, reports `Equal` rather than the
/// byte-order or type-strict answer.
fn key_cmp(key: &Value, lookup_value: &Value) -> Option<Ordering> {
    if excel_equals(lookup_value, key) {
        return Some(Ordering::Equal);
    }
    key.partial_cmp(lookup_value)
}

/// The requested cell, or a blank when the result row is shorter than the first row. Ragged input
/// cannot come from a real spreadsheet range; treating the missing cell as blank keeps the error
/// surface to the three Excel errors `HLOOKUP` can actually raise.
fn cell_or_blank(row: &[Value], column_index: usize) -> Value {
    row.get(column_index).cloned().unwrap_or(Value::None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_exact_hlookup_string() {
        let table = vec![
            vec![
                Value::String("Apple".to_string()),
                Value::String("Banana".to_string()),
                Value::String("Carrot".to_string()),
            ],
            vec![
                Value::String("Fruit".to_string()),
                Value::String("Fruit".to_string()),
                Value::String("Vegetable".to_string()),
            ],
            vec![
                Value::String("Red".to_string()),
                Value::String("Yellow".to_string()),
                Value::String("Orange".to_string()),
            ],
        ];

        let result = codcel_h_lookup(Value::String("Banana".to_string()), table, 2, Some(false));
        assert_eq!(result.unwrap(), Value::String("Fruit".to_string()));
    }

    #[test]
    fn test_approximate_hlookup_numeric() {
        let table = vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3), Value::I32(4)],
            vec![
                Value::I32(10),
                Value::I32(20),
                Value::I32(30),
                Value::I32(40),
            ],
            vec![
                Value::I32(100),
                Value::I32(200),
                Value::I32(300),
                Value::I32(400),
            ],
        ];

        let result = codcel_h_lookup(Value::I32(2), table, 3, Some(true));
        assert_eq!(result.unwrap(), Value::I32(200));

        // Test approximate match (should find largest value <= lookup_value)
        let table2 = vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(4), Value::I32(5)],
            vec![
                Value::I32(10),
                Value::I32(20),
                Value::I32(40),
                Value::I32(50),
            ],
            vec![
                Value::I32(100),
                Value::I32(200),
                Value::I32(400),
                Value::I32(500),
            ],
        ];

        let result2 = codcel_h_lookup(Value::I32(3), table2, 2, Some(true));
        assert_eq!(result2.unwrap(), Value::I32(20)); // Should match column with key=2
    }

    #[test]
    fn test_hlookup_errors() {
        let empty_table: Vec<Vec<Value>> = vec![];
        let result = codcel_h_lookup(Value::I32(1), empty_table, 1, None);
        assert!(result.unwrap_err().to_string().contains("#REF!"));

        let table = vec![vec![Value::I32(1), Value::I32(2)]];
        let result = codcel_h_lookup(Value::I32(1), table, 0, None);
        assert!(result.unwrap_err().to_string().contains("#VALUE!"));

        // A row index past the end of the table is #REF!.
        let result = codcel_h_lookup(
            Value::I32(1),
            vec![vec![Value::I32(1), Value::I32(2)]],
            3,
            None,
        );
        assert!(result.unwrap_err().to_string().contains("#REF!"));

        // A negative row index used to wrap to a huge `usize`.
        let result = codcel_h_lookup(
            Value::I32(1),
            vec![vec![Value::I32(1), Value::I32(2)]],
            -1,
            None,
        );
        assert!(result.unwrap_err().to_string().contains("#VALUE!"));

        // No matching column is #N/A.
        let result = codcel_h_lookup(
            Value::I32(9),
            vec![vec![Value::I32(1), Value::I32(2)]],
            1,
            Some(false),
        );
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_hlookup_ragged_table_is_not_rejected() {
        // A short row no longer invalidates the whole table; it was previously a hard error.
        let table = vec![
            vec![Value::I32(1), Value::I32(2)],
            vec![Value::I32(10)], // This row has fewer columns
        ];

        // Column 0 exists in the short row, so the lookup resolves normally.
        let result = codcel_h_lookup(Value::I32(1), table.clone(), 2, None);
        assert_eq!(result.unwrap(), Value::I32(10));

        // Column 1 is missing from the short row, so the result is a blank.
        let result = codcel_h_lookup(Value::I32(2), table, 2, None);
        assert_eq!(result.unwrap(), Value::None);
    }

    #[test]
    fn test_hlookup_exact_is_case_insensitive() {
        // =HLOOKUP("BANANA", A1:C2, 2, FALSE) in US format
        // =HLOOKUP("BANANA"; A1:C2; 2; FALSE) in German format
        let table = vec![
            vec![
                Value::String("Apple".to_string()),
                Value::String("Banana".to_string()),
            ],
            vec![
                Value::String("Red".to_string()),
                Value::String("Yellow".to_string()),
            ],
        ];
        let result = codcel_h_lookup(Value::String("BANANA".to_string()), table, 2, Some(false));
        assert_eq!(result.unwrap(), Value::String("Yellow".to_string()));
    }

    #[test]
    fn test_hlookup_exact_matches_across_numeric_types() {
        let table = vec![
            vec![Value::F64(2.0), Value::I32(3)],
            vec![
                Value::String("two".to_string()),
                Value::String("three".to_string()),
            ],
        ];
        let result = codcel_h_lookup(Value::I32(2), table, 2, Some(false));
        assert_eq!(result.unwrap(), Value::String("two".to_string()));
    }

    #[test]
    fn test_hlookup_exact_wildcards() {
        // =HLOOKUP("App*", A1:C2, 2, FALSE) in US format
        // =HLOOKUP("App*"; A1:C2; 2; FALSE) in German format
        let table = vec![
            vec![
                Value::String("Apple".to_string()),
                Value::String("Banana".to_string()),
            ],
            vec![
                Value::String("Red".to_string()),
                Value::String("Yellow".to_string()),
            ],
        ];
        let result = codcel_h_lookup(
            Value::String("App*".to_string()),
            table.clone(),
            2,
            Some(false),
        );
        assert_eq!(result.unwrap(), Value::String("Red".to_string()));

        let result = codcel_h_lookup(
            Value::String("B?nana".to_string()),
            table.clone(),
            2,
            Some(false),
        );
        assert_eq!(result.unwrap(), Value::String("Yellow".to_string()));

        // `~*` is a literal asterisk, which no key contains.
        let result = codcel_h_lookup(Value::String("~*".to_string()), table, 2, Some(false));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_hlookup_approximate_duplicates_return_last_column() {
        // Excel's approximate match lands on the last column of a run of duplicate keys.
        let table = vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(2), Value::I32(3)],
            vec![
                Value::String("a".to_string()),
                Value::String("b".to_string()),
                Value::String("c".to_string()),
                Value::String("d".to_string()),
            ],
        ];
        let result = codcel_h_lookup(Value::I32(2), table, 2, Some(true));
        assert_eq!(result.unwrap(), Value::String("c".to_string()));
    }

    #[test]
    fn test_hlookup_approximate_below_first_key_is_not_available() {
        let table = vec![
            vec![Value::I32(10), Value::I32(20)],
            vec![Value::I32(1), Value::I32(2)],
        ];
        let result = codcel_h_lookup(Value::I32(5), table, 2, Some(true));
        assert!(result.unwrap_err().to_string().contains("#N/A"));
    }

    #[test]
    fn test_hlookup_approximate_error_in_key_row_falls_back_to_scan() {
        let table = vec![
            vec![Value::I32(1), Value::Error(ExcelError::Na), Value::I32(3)],
            vec![
                Value::String("one".to_string()),
                Value::String("error".to_string()),
                Value::String("three".to_string()),
            ],
        ];
        let result = codcel_h_lookup(Value::I32(3), table.clone(), 2, Some(true));
        assert_eq!(result.unwrap(), Value::String("three".to_string()));

        let result = codcel_h_lookup(Value::I32(2), table, 2, Some(true));
        assert_eq!(result.unwrap(), Value::String("one".to_string()));
    }

    #[test]
    fn test_hlookup_bool() {
        let table = vec![
            vec![Value::Bool(true), Value::Bool(false)],
            vec![
                Value::String("Yes".to_string()),
                Value::String("No".to_string()),
            ],
        ];

        let result = codcel_h_lookup(Value::Bool(true), table, 2, Some(false));
        assert_eq!(result.unwrap(), Value::String("Yes".to_string()));
    }

    #[test]
    fn test_hlookup_datetime() {
        use chrono::{TimeZone, Utc};

        let date1 = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let date2 = Utc.with_ymd_and_hms(2023, 1, 2, 0, 0, 0).unwrap();

        let table = vec![
            vec![Value::ChronoDateTime(date1), Value::ChronoDateTime(date2)],
            vec![
                Value::String("Day 1".to_string()),
                Value::String("Day 2".to_string()),
            ],
        ];

        let result = codcel_h_lookup(Value::ChronoDateTime(date1), table, 2, Some(false));
        assert_eq!(result.unwrap(), Value::String("Day 1".to_string()));
    }

    #[test]
    fn test_hlookup_single_row() {
        let table = vec![vec![Value::I32(1), Value::I32(2), Value::I32(3)]];

        let result = codcel_h_lookup(Value::I32(2), table, 1, Some(false));
        assert_eq!(result.unwrap(), Value::I32(2));
    }

    #[test]
    fn test_hlookup_large_table() {
        let table = vec![
            vec![
                Value::I32(1),
                Value::I32(2),
                Value::I32(3),
                Value::I32(4),
                Value::I32(5),
            ],
            vec![
                Value::I32(10),
                Value::I32(20),
                Value::I32(30),
                Value::I32(40),
                Value::I32(50),
            ],
            vec![
                Value::I32(100),
                Value::I32(200),
                Value::I32(300),
                Value::I32(400),
                Value::I32(500),
            ],
            vec![
                Value::I32(1000),
                Value::I32(2000),
                Value::I32(3000),
                Value::I32(4000),
                Value::I32(5000),
            ],
        ];

        let result = codcel_h_lookup(Value::I32(3), table, 4, Some(false));
        assert_eq!(result.unwrap(), Value::I32(3000));
    }
}
