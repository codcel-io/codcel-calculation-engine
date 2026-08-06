// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `CHISQ.TEST` that returns the chi-squared test for independence.
/// - `observed`: a 2D array of observed values.
/// - `expected`: a 2D array of expected values (must match dimensions of observed).
///
/// Returns the p-value from the chi-squared test, indicating the probability that
/// the observed differences occurred by chance, or an error when inputs are invalid.
pub fn codcel_chisq_test(
    observed: Vec<Vec<f64>>,
    expected: Vec<Vec<f64>>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if observed.is_empty() || expected.is_empty() {
        return Err("CHISQ.TEST: Observed and expected arrays must not be empty.".into());
    }

    if observed.len() != expected.len() {
        return Err(
            "CHISQ.TEST: Observed and expected arrays must have the same dimensions.".into(),
        );
    }

    // Check that all rows have the same length within each array
    let expected_cols = observed[0].len();
    for obs_row in &observed {
        if obs_row.len() != expected_cols {
            return Err("CHISQ.TEST: All rows in observed array must have the same length.".into());
        }
    }

    for exp_row in &expected {
        if exp_row.len() != expected_cols {
            return Err("CHISQ.TEST: All rows in expected array must have the same length.".into());
        }
    }

    // Check that observed and expected have matching row lengths
    for (obs_row, exp_row) in observed.iter().zip(expected.iter()) {
        if obs_row.len() != exp_row.len() {
            return Err(
                "CHISQ.TEST: Rows in observed and expected arrays must have the same length."
                    .into(),
            );
        }
    }

    // Compute the chi-squared statistic
    let mut chi_squared_statistic = 0.0;

    for (obs_row, exp_row) in observed.iter().zip(expected.iter()) {
        for (&obs_value, &exp_value) in obs_row.iter().zip(exp_row.iter()) {
            if exp_value <= 0.0 {
                return Err("CHISQ.TEST: Expected values must be greater than 0.".into());
            }
            chi_squared_statistic += (obs_value - exp_value).powi(2) / exp_value;
        }
    }

    // Degrees of freedom — Excel uses different formulas depending on dimensions:
    //   r > 1 and c > 1  →  df = (r-1)(c-1)
    //   r == 1 and c > 1  →  df = c-1
    //   r > 1 and c == 1  →  df = r-1
    //   r == 1 and c == 1  →  #N/A error
    let num_rows = observed.len();
    let num_cols = observed[0].len();
    let degrees_of_freedom = if num_rows > 1 && num_cols > 1 {
        (num_rows - 1) * (num_cols - 1)
    } else if num_rows == 1 && num_cols > 1 {
        num_cols - 1
    } else if num_rows > 1 && num_cols == 1 {
        num_rows - 1
    } else {
        return Err("CHISQ.TEST: Degrees of freedom must be positive.".into());
    };

    // Compute the p-value using the chi-squared cumulative distribution function
    match statrs::distribution::ChiSquared::new(degrees_of_freedom as f64) {
        Ok(dist) => Ok(1.0 - dist.cdf(chi_squared_statistic)),
        Err(_) => Err("CHISQ.TEST: Error creating chi-squared distribution.".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chisq_test_basic() {
        // =CHISQ.TEST({8,10,12;8,10,12},{10,10,10;10,10,10}) in US format
        // =CHISQ.TEST({8;10;12;8;10;12};{10;10;10;10;10;10}) in German format
        let observed = vec![vec![8.0, 10.0, 12.0], vec![8.0, 10.0, 12.0]];
        let expected = vec![vec![10.0, 10.0, 10.0], vec![10.0, 10.0, 10.0]];
        let result = codcel_chisq_test(observed, expected).unwrap();
        assert!((result - 0.44932896411722145).abs() < 1e-10); // p-value for chi-squared test
    }

    #[test]
    fn test_chisq_test_independence() {
        // =CHISQ.TEST({58,11,10;110,50,31},{54.2,21.8,3;113.8,39.2,38}) in US format
        // =CHISQ.TEST({58;11;10;110;50;31};{54,2;21,8;3;113,8;39,2;38}) in German format
        let observed = vec![vec![58.0, 11.0, 10.0], vec![110.0, 50.0, 31.0]];
        let expected = vec![vec![54.2, 21.8, 3.0], vec![113.8, 39.2, 38.0]];
        let result = codcel_chisq_test(observed, expected).unwrap();
        assert!(result < 0.001); // Just verify it's a very small p-value
    }

    #[test]
    fn test_chisq_test_single_cell() {
        // =CHISQ.TEST({10},{9}) in US format
        // =CHISQ.TEST({10};{9}) in German format
        let observed = vec![vec![10.0]];
        let expected = vec![vec![9.0]];
        let result = codcel_chisq_test(observed, expected);
        assert!(result.is_err()); // Degrees of freedom must be positive
    }

    #[test]
    fn test_chisq_test_single_row() {
        // =CHISQ.TEST({10,20,30},{15,15,30}) in US format
        // Excel uses df = c-1 = 2 for single-row data
        let observed = vec![vec![10.0, 20.0, 30.0]];
        let expected = vec![vec![15.0, 15.0, 30.0]];
        let result = codcel_chisq_test(observed, expected).unwrap();
        // chi-squared stat = (10-15)^2/15 + (20-15)^2/15 + (30-30)^2/30 = 25/15 + 25/15 = 10/3
        // df = 2, p-value from chi-squared distribution
        assert!(result > 0.0 && result < 1.0);
    }

    #[test]
    fn test_chisq_test_single_column() {
        // =CHISQ.TEST({10;20;30},{15;15;30}) in US format
        // Excel uses df = r-1 = 2 for single-column data
        let observed = vec![vec![10.0], vec![20.0], vec![30.0]];
        let expected = vec![vec![15.0], vec![15.0], vec![30.0]];
        let result = codcel_chisq_test(observed, expected).unwrap();
        // Same chi-squared stat as single-row case, same df = 2
        assert!(result > 0.0 && result < 1.0);
    }

    #[test]
    fn test_chisq_test_empty_arrays() {
        // =CHISQ.TEST({},{}) in US format (returns #VALUE! error)
        // =CHISQ.TEST({};{}) in German format (returns #VALUE! error)
        let observed: Vec<Vec<f64>> = vec![];
        let expected: Vec<Vec<f64>> = vec![];
        let result = codcel_chisq_test(observed, expected);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_test_different_dimensions() {
        // =CHISQ.TEST({10,20;30,40},{15,15}) in US format (returns #VALUE! error)
        // =CHISQ.TEST({10;20;30;40};{15;15}) in German format (returns #VALUE! error)
        let observed = vec![vec![10.0, 20.0], vec![30.0, 40.0]];
        let expected = vec![vec![15.0, 15.0]];
        let result = codcel_chisq_test(observed, expected);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_test_different_row_lengths() {
        // =CHISQ.TEST({10,20;30,40,50},{15,15;25,25,25}) in US format (returns #VALUE! error)
        // =CHISQ.TEST({10;20;30;40;50};{15;15;25;25;25}) in German format (returns #VALUE! error)
        let observed = vec![vec![10.0, 20.0], vec![30.0, 40.0, 50.0]];
        let expected = vec![vec![15.0, 15.0], vec![25.0, 25.0, 25.0]];
        let result = codcel_chisq_test(observed, expected);
        println!("{result:?}");
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_test_zero_expected() {
        // =CHISQ.TEST({10,20;30,40},{15,15;25,0}) in US format (returns #DIV/0! error)
        // =CHISQ.TEST({10;20;30;40};{15;15;25;0}) in German format (returns #DIV/0! error)
        let observed = vec![vec![10.0, 20.0], vec![30.0, 40.0]];
        let expected = vec![vec![15.0, 15.0], vec![25.0, 0.0]];
        let result = codcel_chisq_test(observed, expected);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_test_negative_expected() {
        // =CHISQ.TEST({10,20;30,40},{15,15;25,-5}) in US format (returns #NUM! error)
        // =CHISQ.TEST({10;20;30;40};{15;15;25;-5}) in German format (returns #NUM! error)
        let observed = vec![vec![10.0, 20.0], vec![30.0, 40.0]];
        let expected = vec![vec![15.0, 15.0], vec![25.0, -5.0]];
        let result = codcel_chisq_test(observed, expected);
        assert!(result.is_err());
    }

    #[test]
    fn test_chisq_test_perfect_match() {
        // =CHISQ.TEST({10,20;30,40},{10,20;30,40}) in US format
        // =CHISQ.TEST({10;20;30;40};{10;20;30;40}) in German format
        let observed = vec![vec![10.0, 20.0], vec![30.0, 40.0]];
        let expected = vec![vec![10.0, 20.0], vec![30.0, 40.0]];
        let result = codcel_chisq_test(observed, expected).unwrap();
        assert_eq!(result, 1.0); // p-value is 1 when observed equals expected
    }
}
