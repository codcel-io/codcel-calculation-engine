// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `RANK.AVG` that returns the rank of a value, averaging ties.
/// - `value`: the value for which to find the rank.
/// - `values`: an array of numeric values to rank against.
/// - `order`: if `false` or omitted, ranks in descending order; if `true`, ranks in ascending order.
///
/// Returns the average rank when there are ties,
/// or an error when the array is empty.
pub fn codcel_rank_avg(
    value: f64,
    values: Vec<f64>,
    order: Option<bool>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("RANK.AVG: The values list cannot be empty.".into());
    }

    // Default to descending order if `order` is None
    let sort_ascending = order.unwrap_or(false);

    // Without NaN, the tied values take the ranks just after those ranked above them: counted
    // in one pass, rather than by sorting a copy on every call.
    if !values.iter().any(|v| v.is_nan()) {
        let (mut above, mut tied) = (0u64, 0u64);
        for &v in &values {
            if v == value {
                tied += 1;
            } else if (sort_ascending && v < value) || (!sort_ascending && v > value) {
                above += 1;
            }
        }
        if tied == 0 {
            return Err("RANK.AVG: The value is not found in the list.".into());
        }
        // The sum of the ranks above+1 ..= above+tied, exact as the sum of the ranks was.
        let rank_sum = tied * above + tied * (tied + 1) / 2;
        return Ok(rank_sum as f64 / tied as f64);
    }

    // Sort the values in ascending or descending order
    let mut sorted_values = values;
    if sort_ascending {
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        sorted_values.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    }

    // Calculate the rank(s) of the value
    let mut rank_sum = 0.0;
    let mut count = 0;

    for (i, &v) in sorted_values.iter().enumerate() {
        if v == value {
            rank_sum += (i + 1) as f64; // 1-based rank
            count += 1;
        }
    }

    if count == 0 {
        return Err("RANK.AVG: The value is not found in the list.".into());
    }

    // Average the ranks for tied values
    let avg_rank = rank_sum / count as f64;
    Ok(avg_rank)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ranks as the sorting implementation computed them, to check the counting one against.
    fn sorted_ranks(value: f64, values: &[f64], ascending: bool) -> Option<(i32, f64)> {
        let mut sorted = values.to_vec();
        if ascending {
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        } else {
            sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        }
        let positions: Vec<usize> = (0..sorted.len()).filter(|&i| sorted[i] == value).collect();
        let first = *positions.first()? as i32 + 1;
        let sum: f64 = positions.iter().map(|&i| (i + 1) as f64).sum();
        Some((first, sum / positions.len() as f64))
    }

    #[test]
    fn counted_ranks_match_sorted_ranks() {
        let mut seed: u64 = 0x2545_f491_4f6c_dd1d;
        let mut next = || {
            seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            seed >> 33
        };
        for _ in 0..2_000 {
            let len = 1 + (next() % 40) as usize;
            // Few distinct values, so there are many ties; includes -0.0 and 0.0.
            let values: Vec<f64> = (0..len)
                .map(|_| match next() % 9 {
                    0 => -0.0,
                    1 => 0.0,
                    n => (n as f64 - 4.0) * 1.5,
                })
                .collect();
            let value = values[(next() as usize) % len];
            let missing = 1234.5;
            for ascending in [false, true] {
                let order = Some(ascending);
                let (eq, avg) = sorted_ranks(value, &values, ascending).unwrap();
                assert_eq!(
                    crate::statistical::codcel_rank_eq::codcel_rank_eq(
                        value,
                        values.clone(),
                        order
                    )
                    .unwrap(),
                    eq
                );
                assert_eq!(
                    crate::compatibility::codcel_rank::codcel_rank(value, values.clone(), order)
                        .unwrap(),
                    eq
                );
                assert_eq!(
                    codcel_rank_avg(value, values.clone(), order)
                        .unwrap()
                        .to_bits(),
                    avg.to_bits()
                );
                assert!(codcel_rank_avg(missing, values.clone(), order).is_err());
                assert!(crate::statistical::codcel_rank_eq::codcel_rank_eq(
                    missing,
                    values.clone(),
                    order
                )
                .is_err());
            }
        }
    }

    #[test]
    fn test_rank_avg_descending() {
        // =RANK.AVG(3.5, {7,3.5,3.5,1,2}, FALSE) in US format
        // =RANK.AVG(3,5; {7;3,5;3,5;1;2}; FALSE) in German format
        let value = 3.5;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, Some(false)).unwrap();
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_rank_avg_ascending() {
        // =RANK.AVG(3.5, {7,3.5,3.5,1,2}, TRUE) in US format
        // =RANK.AVG(3,5; {7;3,5;3,5;1;2}; TRUE) in German format
        let value = 3.5;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, Some(true)).unwrap();
        assert!((result - 3.5).abs() < 0.0001);
    }

    #[test]
    fn test_rank_avg_default_order() {
        // =RANK.AVG(3.5, {7,3.5,3.5,1,2}) in US format
        // =RANK.AVG(3,5; {7;3,5;3,5;1;2}) in German format
        let value = 3.5;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, None).unwrap();
        assert!((result - 2.5).abs() < 0.0001);
    }

    #[test]
    fn test_rank_avg_single_occurrence() {
        // =RANK.AVG(7, {7,3.5,3.5,1,2}, FALSE) in US format
        // =RANK.AVG(7; {7;3,5;3,5;1;2}; FALSE) in German format
        let value = 7.0;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, Some(false)).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_rank_avg_multiple_ties() {
        // =RANK.AVG(3, {7,3,3,3,1,2}, FALSE) in US format
        // =RANK.AVG(3; {7;3;3;3;1;2}; FALSE) in German format
        let value = 3.0;
        let values = vec![7.0, 3.0, 3.0, 3.0, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, Some(false)).unwrap();
        assert!((result - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_rank_avg_empty_array() {
        // Empty array should return an error
        let value = 3.5;
        let values: Vec<f64> = vec![];
        let result = codcel_rank_avg(value, values, Some(false));
        assert!(result.is_err());
    }

    #[test]
    fn test_rank_avg_value_not_found() {
        // Value not in the list should return an error
        let value = 10.0;
        let values = vec![7.0, 3.5, 3.5, 1.0, 2.0];
        let result = codcel_rank_avg(value, values, Some(false));
        assert!(result.is_err());
    }
}
