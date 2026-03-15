// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::Value;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum MatchMode {
    ExactMatch = 0,
    ExactMatchOrNextSmaller = -1,
    ExactMatchOrNextLarger = 1,
    WildcardMatch = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum SearchMode {
    First = 1,
    Last = -1,
    Binary = 2,
    BinaryReverse = -2,
}

/// Implements Excel's `XLOOKUP`, returning the value from `return_array` that aligns with the match.
///
/// `match_mode` uses Excel's semantics: `0` (default) exact, `-1` exact or next smaller, `1` exact
/// or next larger, and `2` wildcard string matching. `search_mode` controls search direction:
/// `1` (default) first to last, `-1` last to first, `2` binary search on ascending data, and `-2`
/// binary search on descending data. When no match is found, `if_not_found` is returned instead of
/// an error when provided.
///
/// # Errors
/// Returns an error when the lookup or return arrays are empty, lengths differ, modes are invalid,
/// or no match is found and no `if_not_found` value is supplied.
pub fn codcel_x_lookup(
    lookup_value: Value,
    lookup_array: Vec<Value>,
    return_array: Vec<Value>,
    if_not_found: Option<Value>,
    match_mode: Option<i32>,
    search_mode: Option<i32>,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if lookup_array.is_empty() {
        return Err("XLOOKUP: Lookup array cannot be empty".into());
    }

    if return_array.is_empty() {
        return Err("XLOOKUP: Return array cannot be empty".into());
    }

    if lookup_array.len() != return_array.len() {
        return Err("XLOOKUP: Lookup array and return array must have the same length".into());
    }

    let match_mode = parse_match_mode(match_mode.unwrap_or(0))?;
    let search_mode = parse_search_mode(search_mode.unwrap_or(1))?;

    let result = match search_mode {
        SearchMode::First => search_first(&lookup_value, &lookup_array, &return_array, match_mode),
        SearchMode::Last => search_last(&lookup_value, &lookup_array, &return_array, match_mode),
        SearchMode::Binary => search_binary(
            &lookup_value,
            &lookup_array,
            &return_array,
            match_mode,
            false,
        ),
        SearchMode::BinaryReverse => search_binary(
            &lookup_value,
            &lookup_array,
            &return_array,
            match_mode,
            true,
        ),
    };

    match result {
        Ok(value) => Ok(value),
        Err(_) => {
            if let Some(not_found_value) = if_not_found {
                Ok(not_found_value)
            } else {
                Err("XLOOKUP: No match found".into())
            }
        }
    }
}

fn parse_match_mode(mode: i32) -> Result<MatchMode, Box<dyn Error + Send + Sync>> {
    match mode {
        0 => Ok(MatchMode::ExactMatch),
        -1 => Ok(MatchMode::ExactMatchOrNextSmaller),
        1 => Ok(MatchMode::ExactMatchOrNextLarger),
        2 => Ok(MatchMode::WildcardMatch),
        _ => {
            Err(format!("XLOOKUP: Invalid match_mode {mode}. Valid values are 0, -1, 1, 2").into())
        }
    }
}

fn parse_search_mode(mode: i32) -> Result<SearchMode, Box<dyn Error + Send + Sync>> {
    match mode {
        1 => Ok(SearchMode::First),
        -1 => Ok(SearchMode::Last),
        2 => Ok(SearchMode::Binary),
        -2 => Ok(SearchMode::BinaryReverse),
        _ => Err(
            format!("XLOOKUP: Invalid search_mode {mode}. Valid values are 1, -1, 2, -2").into(),
        ),
    }
}

fn search_first(
    lookup_value: &Value,
    lookup_array: &[Value],
    return_array: &[Value],
    match_mode: MatchMode,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    for (i, array_value) in lookup_array.iter().enumerate() {
        if is_match(lookup_value, array_value, match_mode)? {
            return Ok(return_array[i].clone());
        }
    }
    Err("No match found".into())
}

fn search_last(
    lookup_value: &Value,
    lookup_array: &[Value],
    return_array: &[Value],
    match_mode: MatchMode,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    for (i, array_value) in lookup_array.iter().enumerate().rev() {
        if is_match(lookup_value, array_value, match_mode)? {
            return Ok(return_array[i].clone());
        }
    }
    Err("No match found".into())
}

fn search_binary(
    lookup_value: &Value,
    lookup_array: &[Value],
    return_array: &[Value],
    match_mode: MatchMode,
    reverse_order: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match match_mode {
        MatchMode::ExactMatch => {
            binary_search_exact(lookup_value, lookup_array, return_array, reverse_order)
        }
        MatchMode::ExactMatchOrNextSmaller => binary_search_approximate(
            lookup_value,
            lookup_array,
            return_array,
            reverse_order,
            true,
        ),
        MatchMode::ExactMatchOrNextLarger => binary_search_approximate(
            lookup_value,
            lookup_array,
            return_array,
            reverse_order,
            false,
        ),
        MatchMode::WildcardMatch => {
            // Binary search doesn't make sense for wildcard matching, fall back to linear search
            search_first(lookup_value, lookup_array, return_array, match_mode)
        }
    }
}

fn binary_search_exact(
    lookup_value: &Value,
    lookup_array: &[Value],
    return_array: &[Value],
    reverse_order: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let mut left = 0;
    let mut right = lookup_array.len();

    while left < right {
        let mid = left + (right - left) / 2;
        let cmp = lookup_value.partial_cmp(&lookup_array[mid]);

        match cmp {
            Some(std::cmp::Ordering::Equal) => {
                return Ok(return_array[mid].clone());
            }
            Some(std::cmp::Ordering::Less) => {
                if reverse_order {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            Some(std::cmp::Ordering::Greater) => {
                if reverse_order {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            None => return Err("Incomparable values in binary search".into()),
        }
    }

    Err("Exact match not found in binary search".into())
}

fn binary_search_approximate(
    lookup_value: &Value,
    lookup_array: &[Value],
    return_array: &[Value],
    reverse_order: bool,
    find_smaller: bool,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let mut best_match: Option<usize> = None;
    let mut left = 0;
    let mut right = lookup_array.len();

    while left < right {
        let mid = left + (right - left) / 2;
        let cmp = lookup_value.partial_cmp(&lookup_array[mid]);

        match cmp {
            Some(std::cmp::Ordering::Equal) => {
                return Ok(return_array[mid].clone());
            }
            Some(std::cmp::Ordering::Less) => {
                if !find_smaller {
                    best_match = Some(mid);
                }
                if reverse_order {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            Some(std::cmp::Ordering::Greater) => {
                if find_smaller {
                    best_match = Some(mid);
                }
                if reverse_order {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            None => return Err("Incomparable values in binary search".into()),
        }
    }

    best_match
        .map(|i| return_array[i].clone())
        .ok_or_else(|| "Approximate match not found in binary search".into())
}

fn is_match(
    lookup_value: &Value,
    array_value: &Value,
    match_mode: MatchMode,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    match match_mode {
        MatchMode::ExactMatch => Ok(lookup_value == array_value),
        MatchMode::ExactMatchOrNextSmaller => {
            if lookup_value == array_value {
                Ok(true)
            } else {
                match lookup_value.partial_cmp(array_value) {
                    Some(std::cmp::Ordering::Greater) => Ok(true),
                    _ => Ok(false),
                }
            }
        }
        MatchMode::ExactMatchOrNextLarger => {
            if lookup_value == array_value {
                Ok(true)
            } else {
                match lookup_value.partial_cmp(array_value) {
                    Some(std::cmp::Ordering::Less) => Ok(true),
                    _ => Ok(false),
                }
            }
        }
        MatchMode::WildcardMatch => {
            // Simplified wildcard matching - you may want to implement more sophisticated pattern matching
            match (lookup_value, array_value) {
                (Value::String(pattern), Value::String(text)) => Ok(wildcard_match(pattern, text)),
                _ => Ok(lookup_value == array_value),
            }
        }
    }
}

fn wildcard_match(pattern: &str, text: &str) -> bool {
    wildcard_match_recursive(pattern, text)
}

fn wildcard_match_recursive(pattern: &str, text: &str) -> bool {
    let mut pattern_chars = pattern.chars();
    let mut text_chars = text.chars();

    loop {
        match pattern_chars.next() {
            Some('*') => {
                // Collect remaining pattern after *
                let remaining_pattern: String = pattern_chars.collect();

                if remaining_pattern.is_empty() {
                    return true; // Pattern ends with *, matches everything remaining
                }

                // Try to match the rest of the pattern with remaining text
                let remaining_text: String = text_chars.collect();
                for i in 0..=remaining_text.len() {
                    if wildcard_match_recursive(&remaining_pattern, &remaining_text[i..]) {
                        return true;
                    }
                }
                return false;
            }
            Some('?') => {
                if text_chars.next().is_none() {
                    return false; // Pattern expects a character but text is exhausted
                }
            }
            Some(pattern_char) => match text_chars.next() {
                Some(text_char) if pattern_char == text_char => continue,
                _ => return false,
            },
            None => {
                // Pattern is exhausted, check if text is also exhausted
                return text_chars.next().is_none();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_xlookup_invalid_modes() {
        let lookup_array = vec![Value::I32(1)];
        let return_array = vec![Value::String("One".to_string())];

        // Invalid match_mode
        let result = codcel_x_lookup(
            Value::I32(1),
            lookup_array.clone(),
            return_array.clone(),
            None,
            Some(99), // Invalid match_mode
            Some(1),
        );
        assert!(result.is_err());

        // Invalid search_mode
        let result = codcel_x_lookup(
            Value::I32(1),
            lookup_array,
            return_array,
            None,
            Some(0),
            Some(99), // Invalid search_mode
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_xlookup_exact_match() {
        let lookup_array = vec![
            Value::String("Apple".to_string()),
            Value::String("Banana".to_string()),
            Value::String("Cherry".to_string()),
        ];
        let return_array = vec![Value::I32(10), Value::I32(20), Value::I32(30)];

        let result = codcel_x_lookup(
            Value::String("Banana".to_string()),
            lookup_array,
            return_array,
            None,
            Some(0), // ExactMatch
            Some(1), // First
        );

        assert_eq!(result.unwrap(), Value::I32(20));
    }

    #[test]
    fn test_xlookup_not_found_with_default() {
        let lookup_array = vec![
            Value::String("Apple".to_string()),
            Value::String("Banana".to_string()),
        ];
        let return_array = vec![Value::I32(10), Value::I32(20)];

        let result = codcel_x_lookup(
            Value::String("Orange".to_string()),
            lookup_array,
            return_array,
            Some(Value::String("Not Found".to_string())),
            Some(0), // ExactMatch
            Some(1), // First
        );

        assert_eq!(result.unwrap(), Value::String("Not Found".to_string()));
    }

    #[test]
    fn test_xlookup_approximate_match_larger() {
        let lookup_array = vec![Value::I32(1), Value::I32(3), Value::I32(5), Value::I32(7)];
        let return_array = vec![
            Value::String("One".to_string()),
            Value::String("Three".to_string()),
            Value::String("Five".to_string()),
            Value::String("Seven".to_string()),
        ];

        let result = codcel_x_lookup(
            Value::I32(4),
            lookup_array,
            return_array,
            None,
            Some(1), // ExactMatchOrNextLarger
            Some(1), // First
        );

        assert_eq!(result.unwrap(), Value::String("Five".to_string()));
    }

    #[test]
    fn test_xlookup_search_last() {
        let lookup_array = vec![
            Value::String("Apple".to_string()),
            Value::String("Banana".to_string()),
            Value::String("Apple".to_string()), // Duplicate
        ];
        let return_array = vec![Value::I32(10), Value::I32(20), Value::I32(30)];

        let result = codcel_x_lookup(
            Value::String("Apple".to_string()),
            lookup_array,
            return_array,
            None,
            Some(0),  // ExactMatch
            Some(-1), // Last
        );

        assert_eq!(result.unwrap(), Value::I32(30)); // Should find the last occurrence
    }

    #[test]
    fn test_xlookup_binary_search() {
        let lookup_array = vec![
            Value::I32(1),
            Value::I32(3),
            Value::I32(5),
            Value::I32(7),
            Value::I32(9),
        ];
        let return_array = vec![
            Value::String("One".to_string()),
            Value::String("Three".to_string()),
            Value::String("Five".to_string()),
            Value::String("Seven".to_string()),
            Value::String("Nine".to_string()),
        ];

        let result = codcel_x_lookup(
            Value::I32(5),
            lookup_array,
            return_array,
            None,
            Some(0), // ExactMatch
            Some(2), // Binary
        );

        assert_eq!(result.unwrap(), Value::String("Five".to_string()));
    }

    #[test]
    fn test_xlookup_errors() {
        let empty_lookup: Vec<Value> = vec![];
        let return_array = vec![Value::I32(1)];

        let result = codcel_x_lookup(Value::I32(1), empty_lookup, return_array, None, None, None);
        assert!(result.is_err());

        let lookup_array = vec![Value::I32(1)];
        let empty_return: Vec<Value> = vec![];

        let result = codcel_x_lookup(Value::I32(1), lookup_array, empty_return, None, None, None);
        assert!(result.is_err());

        let lookup_array = vec![Value::I32(1), Value::I32(2)];
        let return_array = vec![Value::I32(10)]; // Different lengths

        let result = codcel_x_lookup(Value::I32(1), lookup_array, return_array, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_wildcard_matching() {
        assert!(wildcard_match("A*", "Apple"));
        assert!(wildcard_match("A*", "A"));
        assert!(!wildcard_match("A*", "Banana"));

        assert!(wildcard_match("B?", "Ba"));
        assert!(!wildcard_match("B?", "Banana"));
        assert!(!wildcard_match("B?", "B"));

        assert!(wildcard_match("*ple", "Apple"));
        assert!(wildcard_match("*ple", "Simple"));
        assert!(!wildcard_match("*ple", "Banana"));

        assert!(wildcard_match("A*e", "Apple"));
        assert!(wildcard_match("A*e", "Awesome"));
        assert!(!wildcard_match("A*e", "Amazing"));
    }
}
