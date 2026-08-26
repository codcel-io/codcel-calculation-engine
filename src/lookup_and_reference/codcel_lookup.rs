// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::error::Error;

/// Implements Excel's `LOOKUP` in both vector and array forms.
///
/// When `result_vector` is provided, the function behaves like `LOOKUP(lookup_value, lookup_vector,
/// result_vector)` and returns the entry from `result_vector` that aligns with the best match in
/// `lookup_vector`. When `result_vector` is `None`, the function uses the simplified array form and
/// returns the best match from `lookup_vector` itself. In approximate mode (the default), the
/// inputs should be sorted in ascending order and the largest value less than or equal to
/// `lookup_value` is returned when no exact match is found.
///
/// # Errors
/// Returns an error when the lookup input is empty, the lookup and result vectors have different
/// lengths, or when no suitable match exists.
pub fn codcel_lookup(
    lookup_value: Value,
    lookup_vector: Vec<Value>,
    result_vector: Option<Vec<Value>>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if lookup_vector.is_empty() {
        return Err("LOOKUP: Lookup vector cannot be empty".into());
    }

    match result_vector {
        Some(result_vec) => vector_lookup(lookup_value, lookup_vector, result_vec),
        None => array_lookup(lookup_value, lookup_vector),
    }
}

/// Array-form helper for `LOOKUP` that accepts a 2D array.
///
/// If the array has more rows than columns, the first column is searched and the last column is
/// returned. Otherwise the first row is searched and the last row is returned, matching Excel's
/// array-form rules.
///
/// # Errors
/// Returns an error when the array is empty, contains empty rows, or when no suitable match is
/// found.
pub fn codcel_lookup_array(
    lookup_value: Value,
    array: Vec<Vec<Value>>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("LOOKUP: Array cannot be empty".into());
    }

    // Determine if array has more rows than columns
    let num_rows = array.len();
    let num_cols = array.first().map(|row| row.len()).unwrap_or(0);

    if num_cols == 0 {
        return Err("LOOKUP: Array cannot have empty rows".into());
    }

    if num_rows >= num_cols {
        // Search down the first column, return from last column
        let lookup_vector: Vec<Value> = array
            .iter()
            .filter_map(|row| row.first().cloned())
            .collect();
        let result_vector: Vec<Value> =
            array.iter().filter_map(|row| row.last().cloned()).collect();
        vector_lookup(lookup_value, lookup_vector, result_vector)
    } else {
        // Search across the first row, return from last row
        let (Some(first_row), Some(last_row)) = (array.first(), array.last()) else {
            return Err("LOOKUP: array must not be empty".into());
        };
        vector_lookup(lookup_value, first_row.clone(), last_row.clone())
    }
}

// Vector form: LOOKUP(lookup_value, lookup_vector, result_vector)
fn vector_lookup(
    lookup_value: Value,
    lookup_vector: Vec<Value>,
    result_vector: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if lookup_vector.len() != result_vector.len() {
        return Err("LOOKUP: Lookup vector and result vector must have the same length".into());
    }

    // LOOKUP assumes data is sorted in ascending order and finds the largest value <= lookup_value
    let mut best_match_index: Option<usize> = None;

    for (i, lookup_val) in lookup_vector.iter().enumerate() {
        match lookup_value.partial_cmp(lookup_val) {
            Some(std::cmp::Ordering::Equal) => {
                return Ok(result_vector[i].clone()); // Exact match
            }
            Some(std::cmp::Ordering::Greater) => {
                best_match_index = Some(i); // Current value is less than lookup
            }
            Some(std::cmp::Ordering::Less) => break, // Values are now too large, stop searching
            None => continue,                        // Incomparable types, skip
        }
    }

    best_match_index
        .map(|i| result_vector[i].clone())
        .ok_or_else(|| "LOOKUP: No suitable match found".into())
}

// Array form: LOOKUP(lookup_value, array) - simplified version for single vector
fn array_lookup(
    lookup_value: Value,
    lookup_vector: Vec<Value>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // In array form with a single vector, we search for the value and return it
    // This is essentially finding the largest value <= lookup_value
    let mut best_match_index: Option<usize> = None;

    for (i, lookup_val) in lookup_vector.iter().enumerate() {
        match lookup_value.partial_cmp(lookup_val) {
            Some(std::cmp::Ordering::Equal) => {
                return Ok(lookup_vector[i].clone()); // Exact match
            }
            Some(std::cmp::Ordering::Greater) => {
                best_match_index = Some(i); // Current value is less than lookup
            }
            Some(std::cmp::Ordering::Less) => break, // Values are now too large, stop searching
            None => continue,                        // Incomparable types, skip
        }
    }

    best_match_index
        .map(|i| lookup_vector[i].clone())
        .ok_or_else(|| "LOOKUP: No suitable match found".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_vector_lookup_exact_match() {
        let lookup_vector = vec![Value::I32(1), Value::I32(2), Value::I32(3), Value::I32(4)];
        let result_vector = vec![
            Value::String("One".to_string()),
            Value::String("Two".to_string()),
            Value::String("Three".to_string()),
            Value::String("Four".to_string()),
        ];

        let result = codcel_lookup(Value::I32(3), lookup_vector, Some(result_vector));
        assert_eq!(result.unwrap(), Value::String("Three".to_string()));
    }

    #[test]
    fn test_vector_lookup_approximate_match() {
        let lookup_vector = vec![Value::I32(1), Value::I32(3), Value::I32(5), Value::I32(7)];
        let result_vector = vec![
            Value::String("A".to_string()),
            Value::String("B".to_string()),
            Value::String("C".to_string()),
            Value::String("D".to_string()),
        ];

        // Looking for 4, should return "B" (corresponding to 3, the largest value <= 4)
        let result = codcel_lookup(Value::I32(4), lookup_vector, Some(result_vector));
        assert_eq!(result.unwrap(), Value::String("B".to_string()));
    }

    #[test]
    fn test_array_lookup_2d_more_rows() {
        let array = vec![
            vec![Value::I32(1), Value::String("Alpha".to_string())],
            vec![Value::I32(2), Value::String("Beta".to_string())],
            vec![Value::I32(3), Value::String("Gamma".to_string())],
            vec![Value::I32(4), Value::String("Delta".to_string())],
        ];

        let result = codcel_lookup_array(Value::I32(3), array);
        assert_eq!(result.unwrap(), Value::String("Gamma".to_string()));
    }

    #[test]
    fn test_array_lookup_2d_more_cols() {
        let array = vec![
            vec![Value::I32(1), Value::I32(2), Value::I32(3), Value::I32(4)],
            vec![
                Value::String("A".to_string()),
                Value::String("B".to_string()),
                Value::String("C".to_string()),
                Value::String("D".to_string()),
            ],
        ];

        let result = codcel_lookup_array(Value::I32(3), array);
        assert_eq!(result.unwrap(), Value::String("C".to_string()));
    }

    #[test]
    fn test_array_lookup_single_vector() {
        let lookup_vector = vec![
            Value::I32(10),
            Value::I32(20),
            Value::I32(30),
            Value::I32(40),
        ];

        // Looking for 25, should return 20 (largest value <= 25)
        let result = codcel_lookup(Value::I32(25), lookup_vector, None);
        assert_eq!(result.unwrap(), Value::I32(20));
    }

    #[test]
    fn test_lookup_with_strings() {
        let lookup_vector = vec![
            Value::String("Apple".to_string()),
            Value::String("Banana".to_string()),
            Value::String("Cherry".to_string()),
            Value::String("Date".to_string()),
        ];
        let result_vector = vec![Value::I32(1), Value::I32(2), Value::I32(3), Value::I32(4)];

        let result = codcel_lookup(
            Value::String("Banana".to_string()),
            lookup_vector,
            Some(result_vector),
        );
        assert_eq!(result.unwrap(), Value::I32(2));
    }

    #[test]
    fn test_lookup_with_floats() {
        let lookup_vector = vec![
            Value::F64(1.1),
            Value::F64(2.2),
            Value::F64(3.3),
            Value::F64(4.4),
        ];
        let result_vector = vec![
            Value::String("Low".to_string()),
            Value::String("Medium".to_string()),
            Value::String("High".to_string()),
            Value::String("Very High".to_string()),
        ];

        // Looking for 2.5, should return "Medium" (corresponding to 2.2)
        let result = codcel_lookup(Value::F64(2.5), lookup_vector, Some(result_vector));
        assert_eq!(result.unwrap(), Value::String("Medium".to_string()));
    }

    #[test]
    fn test_lookup_errors() {
        // Empty lookup vector
        let result = codcel_lookup(Value::I32(1), vec![], Some(vec![Value::I32(1)]));
        assert!(result.is_err());

        // Mismatched vector lengths
        let result = codcel_lookup(
            Value::I32(1),
            vec![Value::I32(1)],
            Some(vec![Value::I32(1), Value::I32(2)]),
        );
        assert!(result.is_err());

        // No suitable match (lookup value is smaller than all values)
        let result = codcel_lookup(
            Value::I32(0),
            vec![Value::I32(1), Value::I32(2), Value::I32(3)],
            Some(vec![
                Value::String("A".to_string()),
                Value::String("B".to_string()),
                Value::String("C".to_string()),
            ]),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_lookup_with_bools() {
        let lookup_vector = vec![Value::Bool(false), Value::Bool(true)];
        let result_vector = vec![
            Value::String("No".to_string()),
            Value::String("Yes".to_string()),
        ];

        let result = codcel_lookup(Value::Bool(true), lookup_vector, Some(result_vector));
        assert_eq!(result.unwrap(), Value::String("Yes".to_string()));
    }

    #[test]
    fn test_lookup_with_options() {
        let lookup_vector = vec![
            Value::OptionI32(Some(1)),
            Value::OptionI32(Some(2)),
            Value::OptionI32(None),
        ];
        let result_vector = vec![
            Value::String("First".to_string()),
            Value::String("Second".to_string()),
            Value::String("None".to_string()),
        ];

        let result = codcel_lookup(
            Value::OptionI32(Some(2)),
            lookup_vector,
            Some(result_vector),
        );
        assert_eq!(result.unwrap(), Value::String("Second".to_string()));
    }

    #[test]
    fn test_lookup_with_datetime() {
        use chrono::{TimeZone, Utc};

        let date1 = Utc
            .with_ymd_and_hms(2023, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let date2 = Utc
            .with_ymd_and_hms(2023, 1, 2, 0, 0, 0)
            .single()
            .expect("valid test date");
        let date3 = Utc
            .with_ymd_and_hms(2023, 1, 3, 0, 0, 0)
            .single()
            .expect("valid test date");

        let lookup_vector = vec![
            Value::ChronoDateTime(date1),
            Value::ChronoDateTime(date2),
            Value::ChronoDateTime(date3),
        ];
        let result_vector = vec![
            Value::String("Day 1".to_string()),
            Value::String("Day 2".to_string()),
            Value::String("Day 3".to_string()),
        ];

        let result = codcel_lookup(
            Value::ChronoDateTime(date2),
            lookup_vector,
            Some(result_vector),
        );
        assert_eq!(result.unwrap(), Value::String("Day 2".to_string()));
    }
}
