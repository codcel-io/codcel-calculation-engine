// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::cmp::Ordering;
use std::error::Error;

/// Returns the 1-based position of `lookup_value` in `lookup_array`, mirroring Excel's `MATCH`.
///
/// The optional `match_type` follows Excel's rules: `0` for exact match, `1` (default) for the
/// largest value less than or equal to `lookup_value` in an ascending list, and `-1` for the
/// smallest value greater than or equal to `lookup_value` in a descending list. The input array
/// should be sorted appropriately for approximate searches.
///
/// # Errors
/// Returns an error when the array is empty, when `match_type` is not one of `-1`, `0`, or `1`, or
/// when no suitable match exists.
pub fn codcel_match<T>(
    lookup_value: T,
    lookup_array: Vec<T>,
    match_type: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>>
where
    T: PartialEq + PartialOrd + Clone,
{
    if lookup_array.is_empty() {
        return Err("#N/A MATCH: Array cannot be empty".into());
    }

    let match_type = match_type.unwrap_or(1);

    match match_type {
        0 => exact_match(lookup_value, lookup_array),
        1 => largest_less_or_equal_match(lookup_value, lookup_array),
        -1 => smallest_greater_or_equal_match(lookup_value, lookup_array),
        _ => Err("MATCH: Match type must be 0, 1 or -1".into()),
    }
}

fn exact_match<T>(
    lookup_value: T,
    lookup_array: Vec<T>,
) -> Result<i32, Box<dyn Error + Send + Sync>>
where
    T: PartialEq,
{
    for (i, value) in lookup_array.iter().enumerate() {
        if lookup_value == *value {
            return Ok(i as i32 + 1); // Excel uses 1-based indexing
        }
    }
    Err("#N/A MATCH: Exact match not found.".into())
}

fn largest_less_or_equal_match<T>(
    lookup_value: T,
    lookup_array: Vec<T>,
) -> Result<i32, Box<dyn Error + Send + Sync>>
where
    T: PartialOrd,
{
    // For this match type, the array should be sorted in ascending order
    let mut best_match: Option<usize> = None;

    for (i, value) in lookup_array.iter().enumerate() {
        match lookup_value.partial_cmp(value) {
            Some(Ordering::Equal) => return Ok(i as i32 + 1), // Exact match
            Some(Ordering::Greater) => best_match = Some(i),  // Current value is less than lookup
            Some(Ordering::Less) => break, // Values are now too large, stop searching
            None => continue,              // Incomparable types, skip
        }
    }

    best_match
        .map(|i| i as i32 + 1)
        .ok_or_else(|| "#N/A MATCH: Largest less or equal match not found.".into())
}

fn smallest_greater_or_equal_match<T>(
    lookup_value: T,
    lookup_array: Vec<T>,
) -> Result<i32, Box<dyn Error + Send + Sync>>
where
    T: PartialOrd,
{
    // For this match type, the array should be sorted in descending order
    for (i, value) in lookup_array.iter().enumerate() {
        match lookup_value.partial_cmp(value) {
            Some(Ordering::Equal) => return Ok(i as i32 + 1), // Exact match
            Some(Ordering::Less) => return Ok(i as i32 + 1),  // Found smallest value >= lookup
            Some(Ordering::Greater) => continue,              // Keep looking
            None => continue,                                 // Incomparable types, skip
        }
    }

    Err("#N/A MATCH: Smallest greater or equal match not found.".into())
}

#[test]
fn test_exact_match_i32() {
    let lookup_array = vec![1, 2, 3, 4, 5];
    let result = codcel_match(3, lookup_array, Some(0));
    assert_eq!(result.unwrap(), 3);
}

#[test]
fn test_exact_match_f64() {
    let lookup_array = vec![1.0, 2.5, 3.7, 4.2];
    let result = codcel_match(2.5, lookup_array, Some(0));
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_exact_match_string() {
    let lookup_array = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];
    let result = codcel_match("banana".to_string(), lookup_array, Some(0));
    assert_eq!(result.unwrap(), 2);
}

#[test]
fn test_largest_less_or_equal() {
    let lookup_array = vec![1, 3, 5, 7, 9];
    let result = codcel_match(6, lookup_array, Some(1));
    assert_eq!(result.unwrap(), 3); // Should find 5 at position 3
}

#[test]
fn test_not_found() {
    let lookup_array = vec![1, 2, 3];
    let result = codcel_match(5, lookup_array, Some(0));
    assert!(result.is_err());
}

#[test]
fn test_empty_array() {
    let lookup_array: Vec<i32> = vec![];
    let result = codcel_match(1, lookup_array, Some(0));
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Array cannot be empty"));
}

#[test]
fn test_invalid_match_type() {
    let lookup_array = vec![1, 2, 3];
    let result = codcel_match(2, lookup_array, Some(5));
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Match type must be 0, 1 or -1"));
}

#[test]
fn test_default_match_type() {
    let lookup_array = vec![1, 3, 5, 7];
    let result = codcel_match(4, lookup_array, None); // Should default to 1
    assert_eq!(result.unwrap(), 2); // Should find 3 at position 2
}

#[cfg(test)]
mod by_reference_tests {
    use super::codcel_match;
    use crate::value::Value;

    /// Matching borrowed cells gives the positions and errors of matching owned ones.
    #[test]
    fn references_match_as_values_do() {
        let s = |t: &str| Value::String(t.to_string());
        let arrays = vec![
            vec![
                Value::F64(1.0),
                Value::F64(2.0),
                Value::F64(2.0),
                Value::F64(5.0),
            ],
            vec![Value::F64(5.0), Value::I32(3), Value::F64(1.0)],
            vec![s("apple"), s("Banana"), s("cherry"), Value::None],
            vec![
                Value::Bool(true),
                s("x"),
                Value::F64(0.0),
                Value::None,
                Value::I32(7),
            ],
            vec![],
        ];
        let lookups = [
            Value::F64(2.0),
            Value::I32(3),
            Value::F64(4.5),
            s("banana"),
            s("b*"),
            Value::Bool(true),
            Value::None,
            Value::F64(-1.0),
        ];
        for array in &arrays {
            for lookup in &lookups {
                for match_type in [None, Some(0), Some(1), Some(-1), Some(2)] {
                    let owned = codcel_match(lookup.clone(), array.clone(), match_type)
                        .map_err(|e| e.to_string());
                    let borrowed = codcel_match(lookup, array.iter().collect(), match_type)
                        .map_err(|e| e.to_string());
                    assert_eq!(owned, borrowed, "{lookup:?} in {array:?}, {match_type:?}");
                }
            }
        }
    }
}
