// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::cmp::Ordering;
use std::error::Error;

/// Excel-compatible `AGGREGATE` that performs one of 19 aggregate calculations on a list.
/// - `function_code`: integer 1–19 selecting which function (1=AVERAGE, 2=COUNT, etc.).
/// - `options`: integer 0–7 controlling how to handle hidden rows, errors, and nested functions.
/// - `values`: the list of numbers to aggregate.
///
/// Returns the aggregated result or an error for invalid codes or insufficient data.
pub fn codcel_aggregate(
    function_code: i32,
    options: i32,
    values: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate function_code
    if !(1..=19).contains(&function_code) {
        return Err("AGGREGATE: Invalid function code. Must be between 1 and 19.".into());
    }

    // Validate options
    if !(0..=7).contains(&options) {
        return Err("AGGREGATE: Invalid options code. Must be between 0 and 7.".into());
    }

    // Option flags
    //let ignore_hidden = options == 1 || options == 3 || options == 5 || options == 7;
    let ignore_errors = options == 2 || options == 3 || options == 6 || options == 7;
    //let ignore_nested = options == 4 || options == 5 || options == 6 || options == 7;

    // In a real Excel implementation:
    // - For ignore_hidden, we would check if the value comes from a hidden row/column
    // - For ignore_nested, we would check if the value is from SUBTOTAL/AGGREGATE functions
    //
    // Since we don't have that context information in this implementation, we'll add
    // placeholder comments to indicate where this logic would go, and just implement
    // the error handling part for now.

    // Filter values based on options
    let filtered_values: Vec<f64> = values
        .into_iter()
        .filter(|&v| {
            // Check for hidden rows/columns (in a real implementation)
            // if ignore_hidden && value_is_from_hidden_cell {
            //    return false;
            // }

            // Check for nested SUBTOTAL/AGGREGATE (in a real implementation)
            // if ignore_nested && value_is_from_subtotal_or_aggregate {
            //    return false;
            // }

            // Handle NaN and error values
            if v.is_infinite() && ignore_errors {
                false // Filter infinite values (could be error values)
            } else if ignore_errors
                && (v == f64::NEG_INFINITY || v.is_sign_negative() && v.abs() > f64::MAX / 2.0)
            {
                // A heuristic to detect Excel error values
                false
            } else {
                true
            }
        })
        .collect();

    // Check if we have enough values for calculations that need data
    if filtered_values.is_empty() && ![2, 3].contains(&function_code) {
        return Err("AGGREGATE: No valid values to calculate result.".into());
    }

    // Implement all 19 functions
    match function_code {
        1 => {
            // AVERAGE
            if filtered_values.is_empty() {
                return Err("AGGREGATE: No values for AVERAGE.".into());
            }
            Ok(filtered_values.iter().sum::<f64>() / filtered_values.len() as f64)
        }
        2 => Ok(filtered_values.len() as f64), // COUNT
        3 => Ok(filtered_values.len() as f64), // COUNTA (treating all numeric values as valid)
        4 => {
            // MAX
            filtered_values
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .copied()
                .ok_or_else(|| "AGGREGATE: No values to find maximum.".into())
        }
        5 => {
            // MIN
            filtered_values
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .copied()
                .ok_or_else(|| "AGGREGATE: No values to find minimum.".into())
        }
        6 => {
            // PRODUCT
            Ok(filtered_values.iter().fold(1.0, |acc, &x| acc * x))
        }
        7 => {
            // STDEV.S (sample standard deviation)
            if filtered_values.len() < 2 {
                return Err("AGGREGATE: At least two values are required for STDEV.S.".into());
            }
            let mean = filtered_values.iter().sum::<f64>() / filtered_values.len() as f64;
            let variance = filtered_values
                .iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / (filtered_values.len() as f64 - 1.0);
            Ok(variance.sqrt())
        }
        8 => {
            // STDEV.P (population standard deviation)
            if filtered_values.is_empty() {
                return Err("AGGREGATE: No values for STDEV.P.".into());
            }
            let n = filtered_values.len() as f64;
            let mean = filtered_values.iter().sum::<f64>() / n;
            let variance = filtered_values
                .iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / n;
            Ok(variance.sqrt())
        }
        9 => Ok(filtered_values.iter().sum()), // SUM
        10 => {
            // VAR.S (sample variance)
            if filtered_values.len() < 2 {
                return Err("AGGREGATE: At least two values are required for VAR.S.".into());
            }
            let mean = filtered_values.iter().sum::<f64>() / filtered_values.len() as f64;
            Ok(filtered_values
                .iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / (filtered_values.len() - 1) as f64)
        }
        11 => {
            // VAR.P (population variance)
            if filtered_values.is_empty() {
                return Err("AGGREGATE: No values for VAR.P.".into());
            }
            let n = filtered_values.len() as f64;
            let mean = filtered_values.iter().sum::<f64>() / n;
            Ok(filtered_values
                .iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / n)
        }
        12 => {
            // MEDIAN
            if filtered_values.is_empty() {
                return Err("AGGREGATE: No values for MEDIAN.".into());
            }

            let mut sorted_values = filtered_values.clone();
            sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            let n = sorted_values.len();
            if n.is_multiple_of(2) {
                // Even number of elements, average the middle two
                Ok((sorted_values[n / 2 - 1] + sorted_values[n / 2]) / 2.0)
            } else {
                // Odd number of elements, return the middle one
                Ok(sorted_values[n / 2])
            }
        }
        13 => {
            // MODE.SNGL (most common value)
            if filtered_values.is_empty() {
                return Err("AGGREGATE: No values for MODE.SNGL.".into());
            }

            let mut value_counts: std::collections::HashMap<String, usize> =
                std::collections::HashMap::new();

            // Count occurrences of each value (using string representation to handle floating-point comparison)
            for value in &filtered_values {
                let key = format!("{value:.15}"); // Use string representation with sufficient precision
                *value_counts.entry(key).or_insert(0) += 1;
            }

            // Find the most frequent value
            let (max_value_str, _) = value_counts
                .iter()
                .max_by_key(|(_, count)| *count)
                .ok_or("AGGREGATE: Could not determine mode.")?;

            // Convert back to f64
            max_value_str
                .parse::<f64>()
                .map_err(|_| "AGGREGATE: Error converting mode value.".into())
        }
        14 => {
            // LARGE - k parameter is the last element in filtered_values
            if filtered_values.len() < 2 {
                return Err("AGGREGATE: Not enough values for LARGE.".into());
            }
            let k = *filtered_values.last().unwrap() as usize;
            let data: Vec<f64> = filtered_values[..filtered_values.len() - 1].to_vec();
            let mut sorted = data.clone();
            sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));
            if k == 0 || k > sorted.len() {
                return Err("AGGREGATE: Invalid k for LARGE.".into());
            }
            Ok(sorted[k - 1])
        }
        15 => {
            // SMALL - k parameter is the last element in filtered_values
            if filtered_values.len() < 2 {
                return Err("AGGREGATE: Not enough values for SMALL.".into());
            }
            let k = *filtered_values.last().unwrap() as usize;
            let data: Vec<f64> = filtered_values[..filtered_values.len() - 1].to_vec();
            let mut sorted = data.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            if k == 0 || k > sorted.len() {
                return Err("AGGREGATE: Invalid k for SMALL.".into());
            }
            Ok(sorted[k - 1])
        }
        16 => {
            // PERCENTILE.INC - k parameter is the last element in filtered_values
            if filtered_values.len() < 2 {
                return Err("AGGREGATE: Not enough values for PERCENTILE.INC.".into());
            }
            let k = *filtered_values.last().unwrap();
            let data: Vec<f64> = filtered_values[..filtered_values.len() - 1].to_vec();

            let mut sorted_values = data.clone();
            sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            let n = sorted_values.len() as f64;
            let position = k * (n - 1.0);
            let lower_index = position.floor() as usize;
            let upper_index = position.ceil() as usize;

            if lower_index == upper_index {
                Ok(sorted_values[lower_index])
            } else {
                let fraction = position - lower_index as f64;
                Ok(sorted_values[lower_index] * (1.0 - fraction)
                    + sorted_values[upper_index] * fraction)
            }
        }
        17 => {
            // QUARTILE.INC - quart parameter is the last element in filtered_values
            if filtered_values.len() < 2 {
                return Err("AGGREGATE: Not enough values for QUARTILE.INC.".into());
            }
            let quart = *filtered_values.last().unwrap() as i32;
            let data: Vec<f64> = filtered_values[..filtered_values.len() - 1].to_vec();

            if !(0..=4).contains(&quart) {
                return Err(
                    "AGGREGATE: Invalid quartile parameter. Must be between 0 and 4.".into(),
                );
            }

            let mut sorted_values = data.clone();
            sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            let k = quart as f64 * 0.25;
            let n = sorted_values.len() as f64;
            let position = k * (n - 1.0);
            let lower_index = position.floor() as usize;
            let upper_index = position.ceil() as usize;

            if lower_index == upper_index {
                Ok(sorted_values[lower_index])
            } else {
                let fraction = position - lower_index as f64;
                Ok(sorted_values[lower_index] * (1.0 - fraction)
                    + sorted_values[upper_index] * fraction)
            }
        }
        18 => {
            // PERCENTILE.EXC - k parameter is the last element in filtered_values
            if filtered_values.len() < 3 {
                return Err("AGGREGATE: Not enough values for PERCENTILE.EXC.".into());
            }
            let k = *filtered_values.last().unwrap();
            let data: Vec<f64> = filtered_values[..filtered_values.len() - 1].to_vec();

            if k <= 0.0 || k >= 1.0 {
                return Err("AGGREGATE: Invalid k parameter for PERCENTILE.EXC. Must be between 0 and 1 exclusive.".into());
            }

            let mut sorted_values = data.clone();
            sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            let n = sorted_values.len() as f64;
            let position = k * (n + 1.0) - 1.0;

            if position < 0.0 || position >= n {
                return Err(
                    "AGGREGATE: The k value results in a position outside array bounds.".into(),
                );
            }

            let lower_index = position.floor() as usize;
            let upper_index = position.ceil() as usize;

            if lower_index == upper_index {
                Ok(sorted_values[lower_index])
            } else {
                let fraction = position - lower_index as f64;
                Ok(sorted_values[lower_index] * (1.0 - fraction)
                    + sorted_values[upper_index] * fraction)
            }
        }
        19 => {
            // QUARTILE.EXC - quart parameter is the last element in filtered_values
            if filtered_values.len() < 4 {
                return Err("AGGREGATE: Not enough values for QUARTILE.EXC.".into());
            }
            let quart = *filtered_values.last().unwrap() as i32;
            let data: Vec<f64> = filtered_values[..filtered_values.len() - 1].to_vec();

            if !(1..=3).contains(&quart) {
                return Err("AGGREGATE: Invalid quartile parameter for QUARTILE.EXC. Must be between 1 and 3.".into());
            }

            let mut sorted_values = data.clone();
            sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

            let n = sorted_values.len() as f64;
            let k = quart as f64 * 0.25;
            let position = k * (n + 1.0) - 1.0;

            if position < 0.0 || position >= n {
                return Err("AGGREGATE: The quartile value results in a position outside array bounds.".into());
            }

            let lower_index = position.floor() as usize;
            let upper_index = position.ceil() as usize;

            if lower_index == upper_index {
                Ok(sorted_values[lower_index])
            } else {
                let fraction = position - lower_index as f64;
                Ok(sorted_values[lower_index] * (1.0 - fraction)
                    + sorted_values[upper_index] * fraction)
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_average() {
        // =AGGREGATE(1, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(1; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(1, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_aggregate_count() {
        // =AGGREGATE(2, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(2; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(2, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_aggregate_counta() {
        // =AGGREGATE(3, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(3; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(3, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_aggregate_max() {
        // =AGGREGATE(4, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(4; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(4, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_aggregate_min() {
        // =AGGREGATE(5, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(5; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(5, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_aggregate_product() {
        // =AGGREGATE(6, 0, 2, 4, 6) in US format
        // =AGGREGATE(6; 0; 2; 4; 6) in German format
        let result = codcel_aggregate(6, 0, vec![2.0, 4.0, 6.0]).unwrap();
        assert_eq!(result, 48.0);
    }

    #[test]
    fn test_aggregate_stdev_s() {
        // =AGGREGATE(7, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(7; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(7, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        let expected = 3.1622776601683795;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_aggregate_stdev_p() {
        // =AGGREGATE(8, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(8; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(8, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        let expected = 2.8284271247461903;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_aggregate_sum() {
        // =AGGREGATE(9, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(9; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(9, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        assert_eq!(result, 30.0);
    }

    #[test]
    fn test_aggregate_var_s() {
        // =AGGREGATE(10, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(10; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(10, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        let expected = 10.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_aggregate_var_p() {
        // =AGGREGATE(11, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(11; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(11, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        let expected = 8.0;
        let epsilon = 1e-14;
        assert!((result - expected).abs() < epsilon);
    }

    #[test]
    fn test_aggregate_median() {
        // =AGGREGATE(12, 0, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(12; 0; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(12, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_aggregate_mode_sngl() {
        // =AGGREGATE(13, 0, 2, 2, 4, 6, 8, 10) in US format
        // =AGGREGATE(13; 0; 2; 2; 4; 6; 8; 10) in German format
        let result = codcel_aggregate(13, 0, vec![2.0, 2.0, 4.0, 6.0, 8.0, 10.0]).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_aggregate_large() {
        // =AGGREGATE(14, 0, {2,4,6,8,10}, 1) - LARGE with k=1 returns largest
        let result = codcel_aggregate(14, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0, 1.0]).unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_aggregate_small() {
        // =AGGREGATE(15, 0, {2,4,6,8,10}, 1) - SMALL with k=1 returns smallest
        let result = codcel_aggregate(15, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0, 1.0]).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_aggregate_percentile_inc() {
        // =AGGREGATE(16, 0, {2,4,6,8,10}, 0.5) - PERCENTILE.INC with k=0.5 returns median
        let result = codcel_aggregate(16, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0, 0.5]).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_aggregate_quartile_inc() {
        // =AGGREGATE(17, 0, {2,4,6,8,10}, 2) - QUARTILE.INC with quart=2 returns median
        let result = codcel_aggregate(17, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0, 2.0]).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_aggregate_percentile_exc() {
        // =AGGREGATE(18, 0, {2,4,6,8,10}, 0.5) - PERCENTILE.EXC with k=0.5 returns median
        let result = codcel_aggregate(18, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0, 0.5]).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_aggregate_quartile_exc() {
        // =AGGREGATE(19, 0, {2,4,6,8,10}, 2) - QUARTILE.EXC with quart=2 returns median
        let result = codcel_aggregate(19, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0, 2.0]).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_aggregate_invalid_function_code() {
        // =AGGREGATE(20, 0, 2, 4, 6, 8, 10) in US format - should return an error
        // =AGGREGATE(20; 0; 2; 4; 6; 8; 10) in German format - should return an error
        let result = codcel_aggregate(20, 0, vec![2.0, 4.0, 6.0, 8.0, 10.0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_aggregate_invalid_options() {
        // =AGGREGATE(1, 8, 2, 4, 6, 8, 10) in US format - should return an error
        // =AGGREGATE(1; 8; 2; 4; 6; 8; 10) in German format - should return an error
        let result = codcel_aggregate(1, 8, vec![2.0, 4.0, 6.0, 8.0, 10.0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_aggregate_ignore_errors() {
        // =AGGREGATE(1, 2, 2, 4, 6, 8, 10, #DIV/0!) in US format
        // =AGGREGATE(1; 2; 2; 4; 6; 8; 10; #DIV/0!) in German format
        // Since we can't directly represent Excel errors in Rust, we'll use f64::INFINITY as a proxy
        let result = codcel_aggregate(1, 2, vec![2.0, 4.0, 6.0, 8.0, 10.0, f64::INFINITY]).unwrap();
        assert_eq!(result, 6.0);
    }
}
