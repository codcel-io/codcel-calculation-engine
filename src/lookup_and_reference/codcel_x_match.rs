// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use std::cmp::Ordering;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum XMatchMode {
    ExactMatch = 0,
    ExactMatchOrNextSmaller = -1,
    ExactMatchOrNextLarger = 1,
    WildcardMatch = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum SearchMode {
    FirstToLast = 1,
    LastToFirst = -1,
    BinarySearchAscending = 2,
    BinarySearchDescending = -2,
}

/// Returns the 1-based position of `lookup_value` in `lookup_array`, mirroring Excel's `XMATCH`.
///
/// `match_mode` defaults to `0` (exact). Use `-1` for exact or next smaller in an ascending list,
/// `1` for exact or next larger in a descending list, and `2` for wildcard pattern matching.
/// `search_mode` defaults to `1` (first to last) but also supports `-1` (last to first),
/// `2` (binary search on ascending data), and `-2` (binary search on descending data). Approximate
/// searches expect the array to be sorted accordingly.
///
/// # Errors
/// Returns an error when the array is empty, an invalid mode is supplied, or no match can be found.
pub fn codcel_x_match(
    lookup_value: Value,
    lookup_array: Vec<Value>,
    match_mode: Option<i32>,
    search_mode: Option<i32>,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    if lookup_array.is_empty() {
        return Err("XMATCH: Array cannot be empty".into());
    }

    let match_mode = match_mode.unwrap_or(0);
    let search_mode = search_mode.unwrap_or(1);

    // Convert i32 to enum for internal use
    let match_mode_enum = match match_mode {
        0 => XMatchMode::ExactMatch,
        -1 => XMatchMode::ExactMatchOrNextSmaller,
        1 => XMatchMode::ExactMatchOrNextLarger,
        2 => XMatchMode::WildcardMatch,
        _ => return Err("XMATCH: Match mode must be 0, 1, -1, or 2".into()),
    };

    let search_mode_enum = match search_mode {
        1 => SearchMode::FirstToLast,
        -1 => SearchMode::LastToFirst,
        2 => SearchMode::BinarySearchAscending,
        -2 => SearchMode::BinarySearchDescending,
        _ => return Err("XMATCH: Search mode must be 1, -1, 2, or -2".into()),
    };

    match match_mode_enum {
        XMatchMode::ExactMatch => exact_match_x_match(lookup_value, lookup_array, search_mode_enum),
        XMatchMode::ExactMatchOrNextSmaller => {
            exact_or_next_smaller_match(lookup_value, lookup_array, search_mode_enum)
        }
        XMatchMode::ExactMatchOrNextLarger => {
            exact_or_next_larger_match(lookup_value, lookup_array, search_mode_enum)
        }
        XMatchMode::WildcardMatch => wildcard_match(lookup_value, lookup_array, search_mode_enum),
    }
}

fn exact_match_x_match(
    lookup_value: Value,
    lookup_array: Vec<Value>,
    search_mode: SearchMode,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    match search_mode {
        SearchMode::FirstToLast => {
            for (i, value) in lookup_array.iter().enumerate() {
                if lookup_value == *value {
                    return Ok(i as i32 + 1);
                }
            }
        }
        SearchMode::LastToFirst => {
            for (i, value) in lookup_array.iter().enumerate().rev() {
                if lookup_value == *value {
                    return Ok(i as i32 + 1);
                }
            }
        }
        SearchMode::BinarySearchAscending | SearchMode::BinarySearchDescending => {
            return binary_search_exact(lookup_value, lookup_array, search_mode);
        }
    }
    Err("XMATCH: Exact match not found".into())
}

fn exact_or_next_smaller_match(
    lookup_value: Value,
    lookup_array: Vec<Value>,
    search_mode: SearchMode,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    match search_mode {
        SearchMode::BinarySearchAscending => {
            binary_search_next_smaller(lookup_value, lookup_array, true)
        }
        SearchMode::BinarySearchDescending => {
            binary_search_next_smaller(lookup_value, lookup_array, false)
        }
        _ => {
            let mut best_match: Option<usize> = None;
            let indices: Box<dyn Iterator<Item = usize>> = match search_mode {
                SearchMode::FirstToLast => Box::new(0..lookup_array.len()),
                SearchMode::LastToFirst => Box::new((0..lookup_array.len()).rev()),
                _ => Box::new(0..lookup_array.len()),
            };

            for i in indices {
                let value = &lookup_array[i];
                match lookup_value.partial_cmp(value) {
                    Some(Ordering::Equal) => return Ok(i as i32 + 1),
                    Some(Ordering::Greater) => {
                        best_match = Some(i);
                        if matches!(search_mode, SearchMode::FirstToLast) {
                            // Continue to find the largest smaller value
                        } else {
                            // For LastToFirst, take the first (last in original order) match
                            break;
                        }
                    }
                    _ => continue,
                }
            }

            best_match
                .map(|i| i as i32 + 1)
                .ok_or_else(|| "XMATCH: No smaller or equal match found".into())
        }
    }
}

fn exact_or_next_larger_match(
    lookup_value: Value,
    lookup_array: Vec<Value>,
    search_mode: SearchMode,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    match search_mode {
        SearchMode::BinarySearchAscending => {
            binary_search_next_larger(lookup_value, lookup_array, true)
        }
        SearchMode::BinarySearchDescending => {
            binary_search_next_larger(lookup_value, lookup_array, false)
        }
        _ => {
            let mut best_match: Option<usize> = None;
            let indices: Box<dyn Iterator<Item = usize>> = match search_mode {
                SearchMode::FirstToLast => Box::new(0..lookup_array.len()),
                SearchMode::LastToFirst => Box::new((0..lookup_array.len()).rev()),
                _ => Box::new(0..lookup_array.len()),
            };

            for i in indices {
                let value = &lookup_array[i];
                match lookup_value.partial_cmp(value) {
                    Some(Ordering::Equal) => return Ok(i as i32 + 1),
                    Some(Ordering::Less) => {
                        let is_better = match best_match {
                            None => true,
                            Some(current) => {
                                (matches!(search_mode, SearchMode::FirstToLast) && i < current)
                                    || (matches!(search_mode, SearchMode::LastToFirst)
                                        && i > current)
                            }
                        };
                        if is_better {
                            best_match = Some(i);
                        }
                        if matches!(search_mode, SearchMode::LastToFirst) {
                            break;
                        }
                    }
                    _ => continue,
                }
            }

            best_match
                .map(|i| i as i32 + 1)
                .ok_or_else(|| "XMATCH: No larger or equal match found".into())
        }
    }
}

fn wildcard_match(
    lookup_value: Value,
    lookup_array: Vec<Value>,
    search_mode: SearchMode,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let pattern = lookup_value.to_string_for_wildcard();

    let indices: Box<dyn Iterator<Item = usize>> = match search_mode {
        SearchMode::FirstToLast => Box::new(0..lookup_array.len()),
        SearchMode::LastToFirst => Box::new((0..lookup_array.len()).rev()),
        _ => Box::new(0..lookup_array.len()), // Binary search not applicable for wildcards
    };

    for i in indices {
        let value_str = lookup_array[i].to_string_for_wildcard();
        if matches_wildcard_pattern(&value_str, &pattern) {
            return Ok(i as i32 + 1);
        }
    }

    Err("XMATCH: No wildcard match found".into())
}

fn binary_search_exact(
    lookup_value: Value,
    lookup_array: Vec<Value>,
    search_mode: SearchMode,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let ascending = matches!(search_mode, SearchMode::BinarySearchAscending);
    let mut left = 0;
    let mut right = lookup_array.len();

    while left < right {
        let mid = left + (right - left) / 2;
        let mid_value = &lookup_array[mid];

        match lookup_value.partial_cmp(mid_value) {
            Some(Ordering::Equal) => return Ok(mid as i32 + 1),
            Some(Ordering::Less) => {
                if ascending {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            Some(Ordering::Greater) => {
                if ascending {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            None => return Err("XMATCH: Incomparable values in binary search".into()),
        }
    }

    Err("XMATCH: Binary search exact match not found".into())
}

fn binary_search_next_smaller(
    lookup_value: Value,
    lookup_array: Vec<Value>,
    ascending: bool,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let mut left = 0;
    let mut right = lookup_array.len();
    let mut result: Option<usize> = None;

    while left < right {
        let mid = left + (right - left) / 2;
        let mid_value = &lookup_array[mid];

        match lookup_value.partial_cmp(mid_value) {
            Some(Ordering::Equal) => return Ok(mid as i32 + 1),
            Some(Ordering::Greater) => {
                result = Some(mid);
                if ascending {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            Some(Ordering::Less) => {
                if ascending {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            None => return Err("XMATCH: Incomparable values in binary search".into()),
        }
    }

    result
        .map(|i| i as i32 + 1)
        .ok_or_else(|| "XMATCH: No smaller value found in binary search".into())
}

fn binary_search_next_larger(
    lookup_value: Value,
    lookup_array: Vec<Value>,
    ascending: bool,
) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let mut left = 0;
    let mut right = lookup_array.len();
    let mut result: Option<usize> = None;

    while left < right {
        let mid = left + (right - left) / 2;
        let mid_value = &lookup_array[mid];

        match lookup_value.partial_cmp(mid_value) {
            Some(Ordering::Equal) => return Ok(mid as i32 + 1),
            Some(Ordering::Less) => {
                result = Some(mid);
                if ascending {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            Some(Ordering::Greater) => {
                if ascending {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            None => return Err("XMATCH: Incomparable values in binary search".into()),
        }
    }

    result
        .map(|i| i as i32 + 1)
        .ok_or_else(|| "XMATCH: No larger value found in binary search".into())
}

// Simple wildcard pattern matching (? for single char, * for multiple chars)
fn matches_wildcard_pattern(text: &str, pattern: &str) -> bool {
    wildcard_match_simple(text, pattern)
}

fn wildcard_match_simple(text: &str, pattern: &str) -> bool {
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    wildcard_match_recursive(&text_chars, &pattern_chars, 0, 0)
}

fn wildcard_match_recursive(text: &[char], pattern: &[char], t_idx: usize, p_idx: usize) -> bool {
    if p_idx == pattern.len() {
        return t_idx == text.len();
    }

    if t_idx == text.len() {
        return pattern[p_idx..].iter().all(|&c| c == '*');
    }

    match pattern[p_idx] {
        '*' => {
            // Try matching zero characters
            if wildcard_match_recursive(text, pattern, t_idx, p_idx + 1) {
                return true;
            }
            // Try matching one or more characters
            wildcard_match_recursive(text, pattern, t_idx + 1, p_idx)
        }
        '?' => wildcard_match_recursive(text, pattern, t_idx + 1, p_idx + 1),
        c => {
            if text[t_idx] == c {
                wildcard_match_recursive(text, pattern, t_idx + 1, p_idx + 1)
            } else {
                false
            }
        }
    }
}

// Test cases
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmatch_exact_match() {
        let lookup_array = vec![Value::I32(1), Value::I32(2), Value::I32(3)];
        let result = codcel_x_match(Value::I32(2), lookup_array, Some(0), None);
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_xmatch_exact_match_strings() {
        let lookup_array = vec![
            Value::String("apple".to_string()),
            Value::String("banana".to_string()),
            Value::String("cherry".to_string()),
        ];
        let result = codcel_x_match(
            Value::String("banana".to_string()),
            lookup_array,
            Some(0),
            None,
        );
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_xmatch_next_smaller() {
        let lookup_array = vec![
            Value::I32(1),
            Value::I32(3),
            Value::I32(5),
            Value::I32(7),
            Value::I32(9),
        ];
        let result = codcel_x_match(Value::I32(6), lookup_array, Some(-1), Some(1));
        assert_eq!(result.unwrap(), 3); // Should find 5 at position 3
    }

    #[test]
    fn test_xmatch_next_larger() {
        let lookup_array = vec![
            Value::I32(1),
            Value::I32(3),
            Value::I32(5),
            Value::I32(7),
            Value::I32(9),
        ];
        let result = codcel_x_match(Value::I32(6), lookup_array, Some(1), Some(1));
        assert_eq!(result.unwrap(), 4); // Should find 7 at position 4
    }

    #[test]
    fn test_xmatch_binary_search_ascending() {
        let lookup_array = vec![
            Value::I32(1),
            Value::I32(3),
            Value::I32(5),
            Value::I32(7),
            Value::I32(9),
            Value::I32(11),
            Value::I32(13),
        ];
        let result = codcel_x_match(Value::I32(7), lookup_array, Some(0), Some(2));
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn test_xmatch_binary_search_descending() {
        let lookup_array = vec![
            Value::I32(13),
            Value::I32(11),
            Value::I32(9),
            Value::I32(7),
            Value::I32(5),
            Value::I32(3),
            Value::I32(1),
        ];
        let result = codcel_x_match(Value::I32(7), lookup_array, Some(0), Some(-2));
        assert_eq!(result.unwrap(), 4);
    }

    #[test]
    fn test_xmatch_wildcard_asterisk() {
        let lookup_array = vec![
            Value::String("apple".to_string()),
            Value::String("banana".to_string()),
            Value::String("cherry".to_string()),
            Value::String("apricot".to_string()),
        ];
        let result = codcel_x_match(
            Value::String("ap*".to_string()),
            lookup_array,
            Some(2),
            None,
        );
        assert_eq!(result.unwrap(), 1); // Should match "apple"
    }

    #[test]
    fn test_xmatch_wildcard_question() {
        let lookup_array = vec![
            Value::String("cat".to_string()),
            Value::String("bat".to_string()),
            Value::String("rat".to_string()),
            Value::String("hat".to_string()),
        ];
        let result = codcel_x_match(
            Value::String("?at".to_string()),
            lookup_array,
            Some(2),
            None,
        );
        assert_eq!(result.unwrap(), 1); // Should match "cat"
    }

    #[test]
    fn test_xmatch_last_to_first_search() {
        let lookup_array = vec![
            Value::I32(1),
            Value::I32(2),
            Value::I32(3),
            Value::I32(2),
            Value::I32(1),
        ];
        let result = codcel_x_match(Value::I32(2), lookup_array, Some(0), Some(-1));
        assert_eq!(result.unwrap(), 4); // Should find the last occurrence of 2
    }

    #[test]
    fn test_xmatch_not_found() {
        let lookup_array = vec![Value::I32(1), Value::I32(2), Value::I32(3)];
        let result = codcel_x_match(Value::I32(5), lookup_array, Some(0), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_xmatch_empty_array() {
        let lookup_array: Vec<Value> = vec![];
        let result = codcel_x_match(Value::I32(1), lookup_array, None, None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Array cannot be empty"));
    }

    #[test]
    fn test_xmatch_default_modes() {
        let lookup_array = vec![
            Value::I32(1),
            Value::I32(2),
            Value::I32(3),
            Value::I32(4),
            Value::I32(5),
        ];
        let result = codcel_x_match(Value::I32(3), lookup_array, None, None);
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn test_xmatch_invalid_match_mode() {
        let lookup_array = vec![Value::I32(1), Value::I32(2), Value::I32(3)];
        let result = codcel_x_match(Value::I32(2), lookup_array, Some(5), None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Match mode must be 0, 1, -1, or 2"));
    }

    #[test]
    fn test_xmatch_invalid_search_mode() {
        let lookup_array = vec![Value::I32(1), Value::I32(2), Value::I32(3)];
        let result = codcel_x_match(Value::I32(2), lookup_array, Some(0), Some(5));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Search mode must be 1, -1, 2, or -2"));
    }
}
