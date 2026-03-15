// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `PERCENTILE`/`PERCENTILE.INC` function.
/// Returns the k-th percentile using Excel's inclusive percentile interpolation.
/// - `array`: array of numeric values.
/// - `k`: percentile value in `(0, 1)`.
///
/// Returns an error when the array is empty or `k` is outside `(0, 1)`.
pub fn codcel_percentile(array: Vec<f64>, k: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if array.is_empty() {
        return Err("PERCENTILE: Input array must not be empty.".into());
    }

    if !(0.0..=1.0).contains(&k) {
        return Err("PERCENTILE: k must be between 0 and 1 (inclusive).".into());
    }

    let mut sorted_array = array.clone();
    sorted_array.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted_array.len();

    if n == 1 {
        return Ok(sorted_array[0]);
    }

    // Excel PERCENTILE.INC formula: position = k * (n - 1)
    let position = k * (n as f64 - 1.0);
    let lower_index = position.floor() as usize;
    let upper_index = lower_index.min(n - 2) + 1;
    let fraction = position - lower_index as f64;

    Ok(sorted_array[lower_index] + fraction * (sorted_array[upper_index] - sorted_array[lower_index]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentile_basic() {
        // =PERCENTILE({1,2,3,4,5}, 0.5) in US format
        // =PERCENTILE({1;2;3;4;5}; 0,5) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let k = 0.5;
        let n = array.len() as f64;
        let pos = k * (n - 1.0) + 1.0;
        println!("n: {n}, k: {k}, pos: {pos}");
        println!(
            "lower_index: {}, upper_index: {}",
            (pos - 1.0).floor() as usize,
            (pos - 1.0).ceil() as usize
        );
        println!(
            "lower_value: {}, upper_value: {}",
            array[(pos - 1.0).floor() as usize],
            array[(pos - 1.0).ceil() as usize]
        );
        println!("weight: {}", pos - 1.0 - (pos - 1.0).floor());

        let result = codcel_percentile(array, k).unwrap();
        println!("result: {result}");
        assert!((result - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_interpolation() {
        // =PERCENTILE({1,2,3,4,5}, 0.3) in US format
        // =PERCENTILE({1;2;3;4;5}; 0,3) in German format
        let array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let k = 0.3;
        let n = array.len() as f64;
        let pos = k * (n - 1.0) + 1.0;
        println!("n: {n}, k: {k}, pos: {pos}");
        println!(
            "lower_index: {}, upper_index: {}",
            (pos - 1.0).floor() as usize,
            (pos - 1.0).ceil() as usize
        );
        println!(
            "lower_value: {}, upper_value: {}",
            array[(pos - 1.0).floor() as usize],
            array[(pos - 1.0).ceil() as usize]
        );
        println!("weight: {}", pos - 1.0 - (pos - 1.0).floor());

        let result = codcel_percentile(array, k).unwrap();
        println!("result: {result}");
        assert!((result - 2.2).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_larger_array() {
        // =PERCENTILE({10,20,30,40,50,60,70,80,90,100}, 0.4) in US format
        // =PERCENTILE({10;20;30;40;50;60;70;80;90;100}; 0,4) in German format
        let array = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];
        let k = 0.4;
        let n = array.len() as f64;
        let pos_check = k * (n + 1.0);
        let pos = k * (n - 1.0) + 1.0;
        println!("n: {n}, k: {k}, pos_check: {pos_check}, pos: {pos}");
        println!(
            "integer_part: {}, fractional_part: {}",
            pos.floor(),
            pos - pos.floor()
        );
        println!(
            "lower_index: {}, upper_index: {}",
            pos.floor() as usize - 1,
            pos.ceil() as usize - 1
        );
        println!(
            "lower_value: {}, upper_value: {}",
            array[pos.floor() as usize - 1],
            array[pos.ceil() as usize - 1]
        );

        let result = codcel_percentile(array, k).unwrap();
        println!("result: {result}");
        assert!((result - 46.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_unsorted_array() {
        // =PERCENTILE({5,3,1,4,2}, 0.5) in US format
        // =PERCENTILE({5;3;1;4;2}; 0,5) in German format
        let result = codcel_percentile(vec![5.0, 3.0, 1.0, 4.0, 2.0], 0.5).unwrap();
        println!("{result}");
        assert!((result - 3.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_empty_array() {
        // =PERCENTILE({}, 0.5) in US format
        // =PERCENTILE({}; 0,5) in German format
        let result = codcel_percentile(vec![], 0.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_percentile_k_zero() {
        // =PERCENTILE({1,2,3,4,5}, 0) in US format
        // =PERCENTILE({1;2;3;4;5}; 0) in German format
        let result = codcel_percentile(vec![1.0, 2.0, 3.0, 4.0, 5.0], 0.0).unwrap();
        assert!((result - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_k_one() {
        // =PERCENTILE({1,2,3,4,5}, 1) in US format
        // =PERCENTILE({1;2;3;4;5}; 1) in German format
        let result = codcel_percentile(vec![1.0, 2.0, 3.0, 4.0, 5.0], 1.0).unwrap();
        assert!((result - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_k_0_1() {
        // =PERCENTILE({1,2,3,4,5}, 0.1) in US format
        let result = codcel_percentile(vec![1.0, 2.0, 3.0, 4.0, 5.0], 0.1).unwrap();
        assert!((result - 1.4).abs() < 0.0001);
    }

    #[test]
    fn test_percentile_k_0_9() {
        // =PERCENTILE({1,2,3,4,5}, 0.9) in US format
        let result = codcel_percentile(vec![1.0, 2.0, 3.0, 4.0, 5.0], 0.9).unwrap();
        assert!((result - 4.6).abs() < 0.0001);
    }
}
