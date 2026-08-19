// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use crate::excel_error::{err_to_box, ExcelError};
use std::error::Error;

/// Excel-compatible `PERCENTOF` that returns what fraction of a whole a subset represents.
/// - `subset`: the values making up the part.
/// - `all`: the values making up the whole.
///
/// Returns `SUM(subset) / SUM(all)` as a decimal fraction (0.2, not 20%), or `#DIV/0!`
/// when the whole sums to zero. The two arguments are summed independently, so they do
/// not need to be the same length.
pub fn codcel_percentof(
    subset: Vec<f64>,
    all: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let subset_total: f64 = subset.iter().compensated_sum();
    let all_total: f64 = all.iter().compensated_sum();

    // Division by zero check
    if all_total == 0.0 {
        return Err(err_to_box(ExcelError::Div0));
    }

    Ok(subset_total / all_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentof_scalars() {
        // =PERCENTOF(2, 10) in US format
        // =PERCENTOF(2; 10) in German format
        let result = codcel_percentof(vec![2.0], vec![10.0]).unwrap();
        assert!((result - 0.2).abs() < 1e-10);
    }

    #[test]
    fn test_percentof_ranges() {
        // =PERCENTOF(A1:A2, A1:A3) where A1:A3 = 1, 2, 3 -> 3/6
        let result = codcel_percentof(vec![1.0, 2.0], vec![1.0, 2.0, 3.0]).unwrap();
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_percentof_whole_of_itself() {
        // =PERCENTOF(A1:A3, A1:A3) is always 1
        let result = codcel_percentof(vec![4.0, 5.0, 6.0], vec![4.0, 5.0, 6.0]).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_percentof_zero_total_is_div_zero() {
        // =PERCENTOF(5, 0) in US format
        // =PERCENTOF(5; 0) in German format
        let result = codcel_percentof(vec![5.0], vec![0.0]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("#DIV/0!"));
    }

    #[test]
    fn test_percentof_totals_cancelling_to_zero() {
        // The whole sums to zero even though it has non-zero entries.
        let result = codcel_percentof(vec![1.0], vec![5.0, -5.0]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("#DIV/0!"));
    }

    #[test]
    fn test_percentof_empty_subset() {
        // An empty part is 0% of the whole.
        let result = codcel_percentof(vec![], vec![1.0, 2.0, 3.0]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_percentof_empty_total_is_div_zero() {
        // An empty whole sums to zero.
        let result = codcel_percentof(vec![1.0], vec![]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("#DIV/0!"));
    }

    #[test]
    fn test_percentof_negative_values() {
        // =PERCENTOF(-2, 10) in US format
        let result = codcel_percentof(vec![-2.0], vec![10.0]).unwrap();
        assert!((result - -0.2).abs() < 1e-10);

        // A negative whole flips the sign.
        let result = codcel_percentof(vec![2.0], vec![-10.0]).unwrap();
        assert!((result - -0.2).abs() < 1e-10);
    }

    #[test]
    fn test_percentof_subset_larger_than_total() {
        // Nothing constrains the part to be a subset of the whole; over 100% is valid.
        let result = codcel_percentof(vec![15.0], vec![10.0]).unwrap();
        assert!((result - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_percentof_unequal_lengths() {
        // Unlike SUMX2MY2, the two arguments are summed independently.
        let result = codcel_percentof(vec![1.0, 1.0, 1.0, 1.0], vec![8.0]).unwrap();
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_percentof_decimals() {
        // =PERCENTOF(1.1, 4.4) in US format
        // =PERCENTOF(1,1; 4,4) in German format
        let result = codcel_percentof(vec![1.1], vec![4.4]).unwrap();
        assert!((result - 0.25).abs() < 1e-10);
    }
}
