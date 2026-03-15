// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;

/// Returns unique values from an input array, mirroring Excel's `UNIQUE`.
///
/// - `values`: The array of values to filter for unique entries.
/// - `by_col`: Reserved for future use (column-based uniqueness for 2D arrays).
/// - `exactly_once`: When `Some(true)`, returns only values appearing exactly once.
///   When `Some(false)` or `None`, returns all distinct values.
///
/// # Errors
/// This function currently does not return errors, but uses `Result` for consistency
/// with other Excel-compatible functions.
pub fn codcel_unique<S>(
    values: Vec<S>,
    _by_col: Option<bool>,
    exactly_once: Option<bool>,
) -> Result<Vec<S>, Box<dyn Error + Send + Sync>>
where
    S: AsRef<str> + Eq + Hash + Clone,
{
    let mut occurrences = HashMap::new();

    for item in values {
        *occurrences.entry(item.clone()).or_insert(0) += 1;
    }

    let result = occurrences
        .into_iter()
        .filter_map(|(item, count)| match exactly_once {
            Some(true) if count == 1 => Some(item),
            Some(false) if count > 1 => None,
            None | Some(false) => Some(item),
            _ => None,
        })
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_basic() {
        let values = vec!["apple", "banana", "apple", "cherry"];
        let result = codcel_unique(values, None, None).unwrap();
        assert_eq!(result.len(), 3);
        assert!(result.contains(&"apple"));
        assert!(result.contains(&"banana"));
        assert!(result.contains(&"cherry"));
    }

    #[test]
    fn test_unique_no_duplicates() {
        let values = vec!["apple", "banana", "cherry"];
        let result = codcel_unique(values, None, None).unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_unique_all_same() {
        let values = vec!["apple", "apple", "apple"];
        let result = codcel_unique(values, None, None).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result.contains(&"apple"));
    }

    #[test]
    fn test_unique_exactly_once_true() {
        // When exactly_once is true, only return values that appear exactly once
        let values = vec!["apple", "banana", "apple", "cherry"];
        let result = codcel_unique(values, None, Some(true)).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"banana"));
        assert!(result.contains(&"cherry"));
        assert!(!result.contains(&"apple")); // apple appears twice
    }

    #[test]
    fn test_unique_exactly_once_false() {
        // When exactly_once is Some(false), items appearing more than once are excluded
        // This differs from None - only items with count == 1 are included
        let values = vec!["apple", "banana", "apple", "cherry"];
        let result = codcel_unique(values, None, Some(false)).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"banana"));
        assert!(result.contains(&"cherry"));
        assert!(!result.contains(&"apple")); // apple appears twice, excluded
    }

    #[test]
    fn test_unique_empty_input() {
        let values: Vec<&str> = vec![];
        let result = codcel_unique(values, None, None).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_unique_with_strings() {
        let values = vec![
            "hello".to_string(),
            "world".to_string(),
            "hello".to_string(),
        ];
        let result = codcel_unique(values, None, None).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"hello".to_string()));
        assert!(result.contains(&"world".to_string()));
    }

    #[test]
    fn test_unique_exactly_once_all_duplicates() {
        // When all values are duplicates and exactly_once is true, return empty
        let values = vec!["apple", "apple", "banana", "banana"];
        let result = codcel_unique(values, None, Some(true)).unwrap();
        assert!(result.is_empty());
    }
}
