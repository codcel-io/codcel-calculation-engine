// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::excel_error::{err_to_box, ExcelError};
use std::error::Error;

/// Excel-compatible `PERCENTILE.INC` that returns the k-th percentile (inclusive method).
/// - `array`: an array of numeric values.
/// - `k`: the percentile value (0 to 1, inclusive).
///
/// Returns the value at the k-th percentile using interpolation (inclusive method),
/// or an error when the array is empty or k is out of range.
pub fn codcel_percentile_inc(array: Vec<f64>, k: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("PERCENTILE.INC: Input array must not be empty.".into());
    }

    if !(0.0..=1.0).contains(&k) {
        return Err("PERCENTILE.INC: k must be between 0 and 1 (inclusive).".into());
    }

    let mut sorted_array = array.clone();
    // A NaN in the range is the legacy in-band representation of an Excel error;
    // Excel propagates it rather than sorting around it.
    if sorted_array.iter().any(|v| v.is_nan()) {
        return Err(err_to_box(ExcelError::Na));
    }
    sorted_array.sort_unstable_by(f64::total_cmp);

    let n = sorted_array.len() as f64;
    let pos = k * (n - 1.0);

    let lower_index = pos.floor() as usize;
    let upper_index = pos.ceil() as usize;

    if lower_index == upper_index {
        // Exact match
        return Ok(sorted_array[lower_index]);
    }

    let lower_value = sorted_array[lower_index];
    let upper_value = sorted_array[upper_index];

    let weight = pos - lower_index as f64;
    let result = lower_value + weight * (upper_value - lower_value);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A NaN in the range is the legacy in-band representation of an Excel error.
    /// PERCENTILE.INC used to panic on it inside `partial_cmp(..).unwrap()`; it must return `#N/A`.
    #[test]
    fn test_nan_in_range_returns_na_instead_of_panicking() {
        let error = codcel_percentile_inc(vec![1.0, 2.0, f64::NAN, 4.0], 0.5)
            .expect_err("NaN must not sort");
        assert!(
            error.to_string().contains("#N/A"),
            "expected #N/A, got {error}"
        );
    }

    #[test]
    fn test_percentile_inc_basic() {
        // =PERCENTILE.INC({1,2,3,4,5}, 0.5) in US format
        // =PERCENTILE.INC({1;2;3;4;5}; 0,5) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percentile_inc(array, 0.5).unwrap();
        assert!((result - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_inc_unsorted() {
        // =PERCENTILE.INC({5,3,1,4,2}, 0.5) in US format
        // =PERCENTILE.INC({5;3;1;4;2}; 0,5) in German format
        let array = vec![5.0, 3.0, 1.0, 4.0, 2.0];
        let result = codcel_percentile_inc(array, 0.5).unwrap();
        assert!((result - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_inc_k_zero() {
        // =PERCENTILE.INC({1,2,3,4,5}, 0) in US format
        // =PERCENTILE.INC({1;2;3;4;5}; 0) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percentile_inc(array, 0.0).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_inc_k_one() {
        // =PERCENTILE.INC({1,2,3,4,5}, 1) in US format
        // =PERCENTILE.INC({1;2;3;4;5}; 1) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percentile_inc(array, 1.0).unwrap();
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_inc_interpolation() {
        // =PERCENTILE.INC({1,2,3,4,5}, 0.3) in US format
        // =PERCENTILE.INC({1;2;3;4;5}; 0,3) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percentile_inc(array, 0.3).unwrap();
        assert!((result - 2.2).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_inc_single_value() {
        // =PERCENTILE.INC({42}, 0.5) in US format
        // =PERCENTILE.INC({42}; 0,5) in German format
        let array = vec![42.0];
        let result = codcel_percentile_inc(array, 0.5).unwrap();
        assert!((result - 42.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_inc_empty_array() {
        // Empty array should return an error
        let array: Vec<f64> = vec![];
        let result = codcel_percentile_inc(array, 0.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_percentile_inc_k_out_of_range_low() {
        // k < 0 should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percentile_inc(array, -0.1);
        assert!(result.is_err());
    }

    #[test]
    fn test_percentile_inc_k_out_of_range_high() {
        // k > 1 should return an error
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = codcel_percentile_inc(array, 1.1);
        assert!(result.is_err());
    }
}
