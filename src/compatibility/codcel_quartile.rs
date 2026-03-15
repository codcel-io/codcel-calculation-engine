// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

/// Excel-compatible `QUARTILE`/`QUARTILE.INC` function.
/// Returns the requested quartile of an array using Excel's inclusive percentile logic.
/// - `array`: array of numeric values.
/// - `quart`: quartile to return (0=min, 1=25th percentile, 2=median, 3=75th percentile, 4=max).
///
/// Returns an error when the array is empty or `quart` is outside 0–4.
pub fn codcel_quartile(
    array: Vec<f64>,
    quart: i32,
) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    if array.is_empty() {
        return Err("QUARTILE: Array cannot be empty".into());
    }
    if !(0..=4).contains(&quart) {
        return Err("QUARTILE: Quart must be between 0 (minimum) and 4 (maximum)".into());
    }

    let mut sorted_array = array.clone();
    sorted_array.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    match quart {
        0 => Ok(sorted_array[0]),                      // Minimum
        1 => calculate_quartile(&sorted_array, 0.25),  // 1st Quartile
        2 => calculate_quartile(&sorted_array, 0.5),   // Median (2nd Quartile)
        3 => calculate_quartile(&sorted_array, 0.75),  // 3rd Quartile
        4 => Ok(sorted_array[sorted_array.len() - 1]), // Maximum
        _ => unreachable!(),                           // Should never occur due to previous check
    }
}

// Helper function to calculate quartiles
fn calculate_quartile(
    sorted_array: &[f64],
    k: f64,
) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    let n = sorted_array.len() as f64;

    // Excel uses (n-1)*k + 1 for the position
    let position = (n - 1.0) * k + 1.0;

    let integer_part = position.floor();
    let fractional_part = position - integer_part;

    let lower_index = integer_part as usize - 1;
    let upper_index = if lower_index + 1 < sorted_array.len() {
        lower_index + 1
    } else {
        lower_index
    };

    let lower_value = sorted_array[lower_index];
    let upper_value = sorted_array[upper_index];

    // Linear interpolation
    let result = lower_value + fractional_part * (upper_value - lower_value);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quartile_minimum() {
        // =QUARTILE({1,2,3,4,5,6,7,8,9,10}, 0) in US format
        // =QUARTILE({1;2;3;4;5;6;7;8;9;10}; 0) in German format
        let result =
            codcel_quartile(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], 0).unwrap();
        println!("{result}");
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_quartile_first() {
        // =QUARTILE({1,2,3,4,5,6,7,8,9,10}, 1) in US format
        // =QUARTILE({1;2;3;4;5;6;7;8;9;10}; 1) in German format
        let result =
            codcel_quartile(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], 1).unwrap();
        println!("{result}");
        assert!((result - 3.25).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_median() {
        // =QUARTILE({1,2,3,4,5,6,7,8,9,10}, 2) in US format
        // =QUARTILE({1;2;3;4;5;6;7;8;9;10}; 2) in German format
        let result =
            codcel_quartile(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], 2).unwrap();
        println!("{result}");
        assert!((result - 5.5).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_third() {
        // =QUARTILE({1,2,3,4,5,6,7,8,9,10}, 3) in US format
        // =QUARTILE({1;2;3;4;5;6;7;8;9;10}; 3) in German format
        let result =
            codcel_quartile(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], 3).unwrap();
        println!("{result}");
        assert!((result - 7.75).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_maximum() {
        // =QUARTILE({1,2,3,4,5,6,7,8,9,10}, 4) in US format
        // =QUARTILE({1;2;3;4;5;6;7;8;9;10}; 4) in German format
        let result =
            codcel_quartile(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], 4).unwrap();
        println!("{result}");
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_quartile_unsorted_array() {
        // =QUARTILE({10,5,1,7,3,9,8,2,6,4}, 2) in US format
        // =QUARTILE({10;5;1;7;3;9;8;2;6;4}; 2) in German format
        let result =
            codcel_quartile(vec![10.0, 5.0, 1.0, 7.0, 3.0, 9.0, 8.0, 2.0, 6.0, 4.0], 2).unwrap();
        println!("{result}");
        assert!((result - 5.5).abs() < 0.0001);
    }

    #[test]
    fn test_quartile_empty_array() {
        // =QUARTILE({}, 2) in US format
        // =QUARTILE({}; 2) in German format
        let result = codcel_quartile(vec![], 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_quartile_invalid_quart() {
        // =QUARTILE({1,2,3,4,5}, 5) in US format
        // =QUARTILE({1;2;3;4;5}; 5) in German format
        let result = codcel_quartile(vec![1.0, 2.0, 3.0, 4.0, 5.0], 5);
        assert!(result.is_err());
    }
}
