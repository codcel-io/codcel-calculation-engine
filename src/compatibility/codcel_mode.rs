// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::collections::HashMap;
use std::error::Error;

/// Excel-compatible `MODE` function.
/// Returns the most frequently occurring value in a data set.
/// - `values`: array of numeric values.
///
/// Returns an error when no mode exists (all values unique) or when the input is empty.
pub fn codcel_mode(values: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if the input vector is empty
    if values.is_empty() {
        return Err("MODE: No values provided".into());
    }

    let mut frequency_map: HashMap<String, usize> = HashMap::new();
    let mut max_frequency = 0;
    let mut mode_value: Option<f64> = None;

    // Count frequencies of each value
    // Convert f64 to string to handle floating-point comparison issues
    for value in &values {
        let value_str = value.to_string();
        let count = frequency_map.entry(value_str).or_insert(0);
        *count += 1;

        // Track the highest frequency and its value
        if *count > max_frequency {
            max_frequency = *count;
            mode_value = Some(*value);
        }
    }

    // If no value appears more than once, Excel returns #N/A
    if max_frequency <= 1 {
        return Err("MODE: No mode found (no value appears more than once)".into());
    }

    // `mode_value` is always set once `max_frequency` exceeds 1, which the check above enforces.
    mode_value.ok_or_else(|| "MODE: No mode found (no value appears more than once)".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_basic() {
        // =MODE(1,2,3,4,3,5) in US format
        // =MODE(1;2;3;4;3;5) in German format
        let result = codcel_mode(vec![1.0, 2.0, 3.0, 4.0, 3.0, 5.0]).unwrap();
        println!("{result}");
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_mode_multiple_modes() {
        // =MODE(1,2,3,3,4,4,5) in US format
        // =MODE(1;2;3;3;4;4;5) in German format
        // Excel returns the first mode encountered
        let result = codcel_mode(vec![1.0, 2.0, 3.0, 3.0, 4.0, 4.0, 5.0]).unwrap();
        println!("{result}");
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_mode_decimal_values() {
        // =MODE(1.5,2.5,1.5,3.5,4.5) in US format
        // =MODE(1,5;2,5;1,5;3,5;4,5) in German format
        let result = codcel_mode(vec![1.5, 2.5, 1.5, 3.5, 4.5]).unwrap();
        println!("{result}");
        assert_eq!(result, 1.5);
    }

    #[test]
    fn test_mode_empty_input() {
        // =MODE() in US format
        // =MODE() in German format
        let result = codcel_mode(vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_mode_no_mode() {
        // =MODE(1,2,3,4,5) in US format
        // =MODE(1;2;3;4;5) in German format
        let result = codcel_mode(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!(result.is_err());
    }
}
