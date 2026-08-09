// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `SUBTOTAL` that returns a subtotal using a specified function.
/// - `function_code`: integer 1–11 or 101–111 selecting the aggregation function.
/// - `values`: the list of numbers to process.
///
/// Returns the subtotal or an error for invalid function codes or insufficient data.
pub fn codcel_sub_total(
    function_code: i32,
    values: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Check if function_code is valid
    if !(1..=11).contains(&function_code) && !(101..=111).contains(&function_code) {
        return Err("SUBTOTAL: Invalid function code. Must be between 1-11 or 101-111.".into());
    }

    // Function codes 101-111 ignore hidden values, but since we don't have visibility information
    // in our simple implementation, we'll treat them the same as 1-11
    let actual_code = if function_code > 100 {
        function_code - 100
    } else {
        function_code
    };

    // Filter out any NaN values as Excel would ignore them
    let filtered_values: Vec<f64> = values.into_iter().filter(|v| !v.is_nan()).collect();

    // Return error if no valid values for functions that need values
    if filtered_values.is_empty() && actual_code != 2 && actual_code != 3 {
        return Err("SUBTOTAL: No valid values to calculate result.".into());
    }

    match actual_code {
        1 => {
            let sum: f64 = filtered_values.iter().sum();
            let count = filtered_values.len() as f64;
            if count == 0.0 {
                Err("SUBTOTAL: Cannot compute average of zero elements.".into())
            } else {
                Ok(sum / count)
            }
        }
        2 => Ok(filtered_values.len() as f64), // COUNT
        3 => {
            // COUNTA: Count values that are not empty
            // In our case, we're counting all numeric values provided
            Ok(filtered_values.len() as f64)
        }
        4 => Ok(*filtered_values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or("SUBTOTAL: No values to find maximum.")?), // MAX
        5 => Ok(*filtered_values
            .iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or("SUBTOTAL: No values to find minimum.")?), // MIN
        6 => {
            // PRODUCT: Multiply all values
            Ok(filtered_values.iter().fold(1.0, |acc, &x| acc * x))
        }
        7 => {
            if filtered_values.len() < 2 {
                return Err("SUBTOTAL: At least two values are required for STDEV.S.".into());
            }
            let n = filtered_values.len() as f64;
            let mean = filtered_values.iter().sum::<f64>() / n;
            let variance = filtered_values
                .iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / (n - 1.0);
            Ok(crate::portable_math::sqrt(variance))
        }
        8 => {
            // STDEV.P (population standard deviation)
            let n = filtered_values.len() as f64;
            let mean = filtered_values.iter().sum::<f64>() / n;
            let variance = filtered_values
                .iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / n;
            Ok(crate::portable_math::sqrt(variance))
        }
        9 => Ok(filtered_values.iter().sum()), // SUM (same as function_code 1 in Excel)
        10 => {
            // VAR.S (sample variance)
            if filtered_values.len() < 2 {
                return Err(
                    "SUBTOTAL: At least two values are required for sample variance.".into(),
                );
            }
            let n = filtered_values.len() as f64;
            let mean = filtered_values.iter().sum::<f64>() / n;
            Ok(filtered_values
                .iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / (n - 1.0))
        }
        11 => {
            // VAR.P (population variance)
            let n = filtered_values.len() as f64;
            let mean = filtered_values.iter().sum::<f64>() / n;
            Ok(filtered_values
                .iter()
                .map(|&x| (x - mean).powi(2))
                .sum::<f64>()
                / n)
        }
        _ => Err("SUBTOTAL: Unimplemented function code.".into()),
    }
}

#[cfg(test)]
mod tests {
    // Literals such as 3.14159 and 1.41421 are Excel-visible values under test,
    // not stand-ins for std::f64::consts.
    #![allow(clippy::approx_constant)]
    use super::*;

    #[test]
    fn test_subtotal_average() {
        // =SUBTOTAL(1,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(1;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(1, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_subtotal_count() {
        // =SUBTOTAL(2,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(2;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(2, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_subtotal_counta() {
        // =SUBTOTAL(3,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(3;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(3, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_subtotal_max() {
        // =SUBTOTAL(4,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(4;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(4, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_subtotal_min() {
        // =SUBTOTAL(5,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(5;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(5, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_subtotal_product() {
        // =SUBTOTAL(6,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(6;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(6, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 120.0);
    }

    #[test]
    fn test_subtotal_stdev_s() {
        // =SUBTOTAL(7,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(7;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(7, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert!((result - 1.58113883).abs() < 1e-10);
    }

    #[test]
    fn test_subtotal_stdev_p() {
        // =SUBTOTAL(8,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(8;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(8, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert!((result - 1.4142135623730951).abs() < 1e-10);
    }

    #[test]
    fn test_subtotal_sum() {
        // =SUBTOTAL(9,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(9;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(9, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_subtotal_var_s() {
        // =SUBTOTAL(10,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(10;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(10, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 2.5);
    }

    #[test]
    fn test_subtotal_var_p() {
        // =SUBTOTAL(11,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(11;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(11, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_subtotal_with_101_code() {
        // =SUBTOTAL(101,A1:A5) where A1:A5 contains [1, 2, 3, 4, 5] in US format
        // =SUBTOTAL(101;A1:A5) where A1:A5 contains [1; 2; 3; 4; 5] in German format
        let result = codcel_sub_total(101, vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result, 3.0); // Same as function code 1 (AVERAGE)
    }

    #[test]
    fn test_subtotal_invalid_function_code() {
        // =SUBTOTAL(0,A1:A5) in US format (returns #VALUE! error)
        // =SUBTOTAL(0;A1:A5) in German format (returns #VALUE! error)
        let result = codcel_sub_total(0, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert!(result.is_err());
    }

    #[test]
    fn test_subtotal_empty_values() {
        // =SUBTOTAL(9,) in US format (returns #VALUE! error)
        // =SUBTOTAL(9;) in German format (returns #VALUE! error)
        let result = codcel_sub_total(9, vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn test_subtotal_empty_values_count() {
        // =SUBTOTAL(2,) in US format (returns 0)
        // =SUBTOTAL(2;) in German format (returns 0)
        let result = codcel_sub_total(2, vec![]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_subtotal_with_nan_values() {
        // =SUBTOTAL(9,A1:A3) where A1:A3 contains [1, #N/A, 3] in US format
        // =SUBTOTAL(9;A1:A3) where A1:A3 contains [1; #N/A; 3] in German format
        let result = codcel_sub_total(9, vec![1.0, f64::NAN, 3.0]).unwrap();
        assert_eq!(result, 4.0); // NaN values are ignored
    }
}
