// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::compatibility::codcel_chi_dist::codcel_chi_dist;
use std::error::Error;

/// Excel-compatible `CHITEST`/`CHI.TEST` function.
/// Calculates the chi-squared test p-value comparing observed and expected frequencies.
/// - `observed`: array of observed frequency values.
/// - `expected`: array of expected frequency values (all must be positive).
///
/// Arrays must be the same length and non-empty.
///
/// Returns the right-tailed probability from the chi-squared statistic with `len - 1` degrees of freedom.
pub fn codcel_chi_test(
    observed: Vec<f64>,
    expected: Vec<f64>,
    rows: usize,
    cols: usize,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if observed.len() != expected.len() {
        return Err("CHITEST: Observed and expected arrays must have the same length".into());
    }

    if observed.is_empty() {
        return Err("CHITEST: Observed and expected arrays cannot be empty".into());
    }

    let mut chi_squared = 0.0;

    for (o, e) in observed.iter().zip(expected.iter()) {
        if *e <= 0.0 {
            return Err("CHITEST: Expected values must be greater than 0".into());
        }
        chi_squared += (*o - *e).powi(2) / *e;
    }

    // For 2D contingency tables (both dims > 1): df = (rows-1) * (cols-1)
    // For 1D arrays (either dim is 1): df = total_elements - 1
    let df = if rows > 1 && cols > 1 {
        ((rows - 1) * (cols - 1)) as f64
    } else {
        (observed.len() - 1) as f64
    };

    codcel_chi_dist(chi_squared, df)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chi_test_basic() {
        // =CHITEST({8,10,12,10},{9,11,10,10}) in US format
        // =CHITEST({8;10;12;10};{9;11;10;10}) in German format
        let observed = vec![8.0, 10.0, 12.0, 10.0];
        let expected = vec![9.0, 11.0, 10.0, 10.0];
        let result = codcel_chi_test(observed, expected, 1, 4).unwrap();
        println!("{result}");
        assert!((result - 0.8959697388878254).abs() < 0.0001);
    }

    #[test]
    fn test_chi_test_identical() {
        // =CHITEST({10,10,10,10},{10,10,10,10}) in US format
        // =CHITEST({10;10;10;10};{10;10;10;10}) in German format
        let observed = vec![10.0, 10.0, 10.0, 10.0];
        let expected = vec![10.0, 10.0, 10.0, 10.0];
        let result = codcel_chi_test(observed, expected, 1, 4).unwrap();
        println!("{result}");
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_chi_test_large_difference() {
        // =CHITEST({5,15,20,10},{10,10,10,10}) in US format
        // =CHITEST({5;15;20;10};{10;10;10;10}) in German format
        let observed = vec![5.0, 15.0, 20.0, 10.0];
        let expected = vec![10.0, 10.0, 10.0, 10.0];
        let result = codcel_chi_test(observed, expected, 1, 4).unwrap();
        println!("{result}");
        assert!((result - 0.0018166489665722985).abs() < 0.0001);
    }

    #[test]
    fn test_chi_test_small_values() {
        // =CHITEST({1,2,3,4},{2,2,2,2}) in US format
        // =CHITEST({1;2;3;4};{2;2;2;2}) in German format
        let observed = vec![1.0, 2.0, 3.0, 4.0];
        let expected = vec![2.0, 2.0, 2.0, 2.0];
        let result = codcel_chi_test(observed, expected, 1, 4).unwrap();
        println!("{result}");
        assert!((result - 0.39162517627109017).abs() < 0.0001);
    }

    #[test]
    fn test_chi_test_large_values() {
        // =CHITEST({100,120,130,110},{115,115,115,115}) in US format
        // =CHITEST({100;120;130;110};{115;115;115;115}) in German format
        let observed = vec![100.0, 120.0, 130.0, 110.0];
        let expected = vec![115.0, 115.0, 115.0, 115.0];
        let result = codcel_chi_test(observed, expected, 1, 4).unwrap();
        println!("{result}");
        assert!((result - 0.22627215200744).abs() < 0.0001);
    }

    #[test]
    fn test_chi_test_different_lengths() {
        // Different lengths should return an error
        let observed = vec![8.0, 10.0, 12.0];
        let expected = vec![9.0, 11.0, 10.0, 10.0];
        let result = codcel_chi_test(observed, expected, 1, 3);
        assert!(result.is_err());
    }

    #[test]
    fn test_chi_test_empty_arrays() {
        // Empty arrays should return an error
        let observed: Vec<f64> = vec![];
        let expected: Vec<f64> = vec![];
        let result = codcel_chi_test(observed, expected, 0, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_chi_test_negative_expected() {
        // Negative expected values should return an error
        let observed = vec![8.0, 10.0, 12.0, 10.0];
        let expected = vec![9.0, -11.0, 10.0, 10.0];
        let result = codcel_chi_test(observed, expected, 1, 4);
        assert!(result.is_err());
    }

    #[test]
    fn test_chi_test_zero_expected() {
        // Zero expected values should return an error
        let observed = vec![8.0, 10.0, 12.0, 10.0];
        let expected = vec![9.0, 0.0, 10.0, 10.0];
        let result = codcel_chi_test(observed, expected, 1, 4);
        assert!(result.is_err());
    }
}
