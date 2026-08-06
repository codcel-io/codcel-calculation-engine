// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Performs a horizontal lookup against the first row of `table_array`, like Excel's `HLOOKUP`.
///
/// `row_index_num` is 1-based and selects the row to return from the column that matches
/// `lookup_value`. When `range_lookup` is `None` or `Some(true)`, the first row is treated as
/// ascending and the largest value less than or equal to `lookup_value` is returned; when
/// `range_lookup` is `Some(false)`, an exact match is required.
///
/// # Errors
/// Returns an error when the table is empty, the requested row index is zero or out of bounds,
/// rows have inconsistent column counts, or no match is found.
pub fn codcel_h_lookup(
    lookup_value: Value,
    table_array: Vec<Vec<Value>>,
    row_index_num: i32,
    range_lookup: Option<bool>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if table_array.is_empty() {
        return Err("HLOOKUP: Table array cannot be empty".into());
    }

    if row_index_num == 0 {
        return Err("HLOOKUP: Row index must be greater than 0".into());
    }

    let row_index_num = row_index_num as usize;

    // Check if we have enough rows
    if table_array.len() < row_index_num {
        return Err(format!(
            "HLOOKUP: Table has only {} rows, but row {} was requested",
            table_array.len(),
            row_index_num
        )
        .into());
    }

    // Check if the first row exists (this is where we search)
    if table_array[0].is_empty() {
        return Err("HLOOKUP: First row cannot be empty".into());
    }

    // Check if all rows have the same number of columns as the first row
    let expected_cols = table_array[0].len();
    for (i, row) in table_array.iter().enumerate() {
        if row.len() != expected_cols {
            return Err(format!(
                "HLOOKUP: Row {} has {} columns, but expected {} columns",
                i + 1,
                row.len(),
                expected_cols
            )
            .into());
        }
    }

    let range_lookup = range_lookup.unwrap_or(true);

    if range_lookup {
        approximate_h_lookup(lookup_value, table_array, row_index_num)
    } else {
        exact_h_lookup(lookup_value, table_array, row_index_num)
    }
}

fn exact_h_lookup(
    lookup_value: Value,
    table_array: Vec<Vec<Value>>,
    row_index_num: usize,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let first_row = &table_array[0];

    for (col_index, first_row_value) in first_row.iter().enumerate() {
        if lookup_value == *first_row_value {
            return Ok(table_array[row_index_num - 1][col_index].clone()); // Excel uses 1-based indexing
        }
    }

    Err("HLOOKUP: Exact match not found".into())
}

fn approximate_h_lookup(
    lookup_value: Value,
    table_array: Vec<Vec<Value>>,
    row_index_num: usize,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // For approximate match, the first row should be sorted in ascending order
    let first_row = &table_array[0];
    let mut best_match_col: Option<usize> = None;

    for (col_index, first_row_value) in first_row.iter().enumerate() {
        match lookup_value.partial_cmp(first_row_value) {
            Some(std::cmp::Ordering::Equal) => {
                return Ok(table_array[row_index_num - 1][col_index].clone()); // Exact match
            }
            Some(std::cmp::Ordering::Greater) => {
                best_match_col = Some(col_index); // Current value is less than lookup
            }
            Some(std::cmp::Ordering::Less) => break, // Values are now too large, stop searching
            None => continue,                        // Incomparable types, skip
        }
    }

    best_match_col
        .map(|col| table_array[row_index_num - 1][col].clone())
        .ok_or_else(|| "HLOOKUP: Approximate match not found".into())
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
        assert!(result.is_err());

        let table = vec![vec![Value::I32(1), Value::I32(2)]];
        let result = codcel_h_lookup(Value::I32(1), table, 0, None);
        assert!(result.is_err());

        let result = codcel_h_lookup(
            Value::I32(1),
            vec![vec![Value::I32(1), Value::I32(2)]],
            3,
            None,
        );
        assert!(result.is_err());

        // Test inconsistent column count
        let inconsistent_table = vec![
            vec![Value::I32(1), Value::I32(2)],
            vec![Value::I32(10)], // This row has fewer columns
        ];
        let result = codcel_h_lookup(Value::I32(1), inconsistent_table, 2, None);
        assert!(result.is_err());
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
