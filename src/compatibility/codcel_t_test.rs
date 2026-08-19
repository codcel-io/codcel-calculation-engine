// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSumExt;
use statrs::distribution::ContinuousCDF;
use std::error::Error;

/// Excel-compatible `TTEST`/`T.TEST` function.
/// Computes the probability associated with a Student's t-test.
/// - `array1`: first array of sample values.
/// - `array2`: second array of sample values.
/// - `tails`: 1 for one-tailed, 2 for two-tailed test.
/// - `t_type`: 1 for paired, 2 for two-sample equal variance, 3 for two-sample unequal variance.
///
/// Returns an error on invalid arguments or empty inputs.
pub fn codcel_t_test(
    array1: Vec<f64>,
    array2: Vec<f64>,
    tails: i32,
    t_type: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    use statrs::distribution::StudentsT;

    if array1.is_empty() || array2.is_empty() {
        return Err("TTEST: Both arrays must contain at least one value.".into());
    }
    if !(tails == 1 || tails == 2) {
        return Err("TTEST: Number of tails must be either 1 or 2.".into());
    }
    if !(t_type == 1 || t_type == 2 || t_type == 3) {
        return Err("TTEST: Type must be 1 (paired), 2 (two-sample equal variance), or 3 (two-sample unequal variance).".into());
    }

    let mean1 = array1.iter().compensated_sum() / array1.len() as f64;
    let mean2 = array2.iter().compensated_sum() / array2.len() as f64;

    let variance1 = array1
        .iter()
        .map(|&x| (x - mean1).powi(2))
        .compensated_sum()
        / (array1.len() - 1) as f64;
    let variance2 = array2
        .iter()
        .map(|&x| (x - mean2).powi(2))
        .compensated_sum()
        / (array2.len() - 1) as f64;

    let (t_stat, degrees_freedom) = match t_type {
        1 => {
            // Paired t-test
            if array1.len() != array2.len() {
                return Err(
                    "TTEST: For paired tests, both arrays must have the same length.".into(),
                );
            }
            let differences: Vec<f64> = array1
                .iter()
                .zip(array2.iter())
                .map(|(&a, &b)| a - b)
                .collect();
            let mean_diff = differences.iter().compensated_sum() / differences.len() as f64;
            let variance_diff = differences
                .iter()
                .map(|&x| (x - mean_diff).powi(2))
                .compensated_sum()
                / (differences.len() - 1) as f64;

            let t_stat = mean_diff
                / (crate::portable_math::sqrt(variance_diff)
                    / crate::portable_math::sqrt(differences.len() as f64));
            let degrees_freedom = (differences.len() - 1) as f64;
            (t_stat, degrees_freedom)
        }
        2 => {
            // Two-sample t-test with equal variance
            let pooled_variance = (((array1.len() - 1) as f64 * variance1)
                + ((array2.len() - 1) as f64 * variance2))
                / ((array1.len() + array2.len() - 2) as f64);
            let t_stat = (mean1 - mean2)
                / crate::portable_math::sqrt(
                    pooled_variance * (1.0 / array1.len() as f64 + 1.0 / array2.len() as f64),
                );
            let degrees_freedom = (array1.len() + array2.len() - 2) as f64;
            (t_stat, degrees_freedom)
        }
        3 => {
            // Two-sample t-test with unequal variance
            let t_stat = (mean1 - mean2)
                / crate::portable_math::sqrt(
                    (variance1 / array1.len() as f64) + (variance2 / array2.len() as f64),
                );
            let numerator =
                ((variance1 / array1.len() as f64) + (variance2 / array2.len() as f64)).powi(2);
            let denominator = ((variance1 / array1.len() as f64).powi(2)
                / (array1.len() - 1) as f64)
                + ((variance2 / array2.len() as f64).powi(2) / (array2.len() - 1) as f64);
            let degrees_freedom = numerator / denominator;
            (t_stat, degrees_freedom)
        }
        _ => unreachable!(),
    };

    let t_distribution = StudentsT::new(0.0, 1.0, degrees_freedom)?;
    let p_value = match tails {
        1 => 1.0 - t_distribution.cdf(t_stat.abs()),
        2 => (1.0 - t_distribution.cdf(t_stat.abs())) * 2.0,
        _ => unreachable!(),
    };

    Ok(p_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_test_paired_one_tail() {
        // =TTEST({5,7,9,8,6}, {4,6,7,8,5}, 1, 1) in US format
        // =TTEST({5;7;9;8;6}; {4;6;7;8;5}; 1; 1) in German format
        let result = codcel_t_test(
            vec![5.0, 7.0, 9.0, 8.0, 6.0],
            vec![4.0, 6.0, 7.0, 8.0, 5.0],
            1,
            1,
        )
        .unwrap();
        println!("{result}");
        assert!((result - 0.017054711583704862).abs() < 0.01);
    }

    #[test]
    fn test_t_test_paired_two_tail() {
        // =TTEST({5,7,9,8,6}, {4,6,7,8,5}, 2, 1) in US format
        // =TTEST({5;7;9;8;6}; {4;6;7;8;5}; 2; 1) in German format
        let result = codcel_t_test(
            vec![5.0, 7.0, 9.0, 8.0, 6.0],
            vec![4.0, 6.0, 7.0, 8.0, 5.0],
            2,
            1,
        )
        .unwrap();
        println!("{result}");
        assert!((result - 0.034109423167409725).abs() < 0.01);
    }

    /* TODO: NOT WORKING #[test]
    fn test_t_test_equal_variance() {
        // =TTEST({5,7,9,8,6}, {4,6,7,8,5,6}, 2, 2) in US format
        // =TTEST({5;7;9;8;6}; {4;6;7;8;5;6}; 2; 2) in German format
        let result = codcel_t_test(
            vec![5.0, 7.0, 9.0, 8.0, 6.0],
            vec![4.0, 6.0, 7.0, 8.0, 5.0, 6.0],
            2,
            2
        ).unwrap();
        println!("{}", result);
        assert!((result - 0.2776).abs() < 0.01);
    }*/

    #[test]
    fn test_t_test_unequal_variance() {
        // =TTEST({5,7,9,8,6}, {4,6,7,8,5,6}, 2, 3) in US format
        // =TTEST({5;7;9;8;6}; {4;6;7;8;5;6}; 2; 3) in German format
        let result = codcel_t_test(
            vec![5.0, 7.0, 9.0, 8.0, 6.0],
            vec![4.0, 6.0, 7.0, 8.0, 5.0, 6.0],
            2,
            3,
        )
        .unwrap();
        println!("{result}");
        assert!((result - 0.3044622454687975).abs() < 0.01);
    }

    #[test]
    fn test_t_test_empty_array() {
        // =TTEST({}, {4,6,7,8,5}, 2, 1) in US format
        // =TTEST({}; {4;6;7;8;5}; 2; 1) in German format
        let result = codcel_t_test(vec![], vec![4.0, 6.0, 7.0, 8.0, 5.0], 2, 1);
        assert!(result.is_err());
    }

    #[test]
    fn test_t_test_invalid_tails() {
        // =TTEST({5,7,9,8,6}, {4,6,7,8,5}, 3, 1) in US format
        // =TTEST({5;7;9;8;6}; {4;6;7;8;5}; 3; 1) in German format
        let result = codcel_t_test(
            vec![5.0, 7.0, 9.0, 8.0, 6.0],
            vec![4.0, 6.0, 7.0, 8.0, 5.0],
            3,
            1,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_t_test_invalid_type() {
        // =TTEST({5,7,9,8,6}, {4,6,7,8,5}, 2, 4) in US format
        // =TTEST({5;7;9;8;6}; {4;6;7;8;5}; 2; 4) in German format
        let result = codcel_t_test(
            vec![5.0, 7.0, 9.0, 8.0, 6.0],
            vec![4.0, 6.0, 7.0, 8.0, 5.0],
            2,
            4,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_t_test_paired_different_lengths() {
        // =TTEST({5,7,9,8,6}, {4,6,7,8}, 2, 1) in US format
        // =TTEST({5;7;9;8;6}; {4;6;7;8}; 2; 1) in German format
        let result = codcel_t_test(
            vec![5.0, 7.0, 9.0, 8.0, 6.0],
            vec![4.0, 6.0, 7.0, 8.0],
            2,
            1,
        );
        assert!(result.is_err());
    }
}
