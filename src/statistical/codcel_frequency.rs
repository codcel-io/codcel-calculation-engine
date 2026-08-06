// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `FREQUENCY` that calculates how often values occur within a range of values.
/// - `data_array`: an array of values for which to count frequencies.
/// - `bins_array`: an array of intervals (bins) into which to group the values.
///
/// Returns a vertical array of frequencies, with one more element than the bins array
/// (the last element counts values greater than the highest bin),
/// or an error when either array is empty.
pub fn codcel_frequency(
    data_array: Vec<f64>,
    bins_array: Vec<f64>,
) -> Result<Vec<i32>, Box<dyn Error + Send + Sync>> {
    if data_array.is_empty() {
        return Err("FREQUENCY: data_array must not be empty.".into());
    }
    if bins_array.is_empty() {
        return Err("FREQUENCY: bins_array must not be empty.".into());
    }

    let mut frequency = vec![0; bins_array.len() + 1]; // +1 for values above the highest bin

    for &value in &data_array {
        let mut added = false;
        for (i, &bin) in bins_array.iter().enumerate() {
            if value <= bin {
                frequency[i] += 1;
                added = true;
                break;
            }
        }
        if !added {
            // Value falls above the highest bin
            frequency[bins_array.len()] += 1;
        }
    }

    Ok(frequency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_basic() {
        // =FREQUENCY({1,2,3,4,5,6,7,8,9,10},{5,10}) in US format
        // =FREQUENCY({1;2;3;4;5;6;7;8;9;10};{5;10}) in German format
        let data_array = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let bins_array = vec![5.0, 10.0];
        let result = codcel_frequency(data_array, bins_array).unwrap();
        println!("{result:?}");
        assert_eq!(result, vec![5, 5, 0]); // 5 values <= 5, 5 values <= 10 but > 5, 0 values > 10
    }

    #[test]
    fn test_frequency_with_values_above_highest_bin() {
        // =FREQUENCY({1,2,3,4,5,6,7,8,9,10},{3,6}) in US format
        // =FREQUENCY({1;2;3;4;5;6;7;8;9;10};{3;6}) in German format
        let data_array = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let bins_array = vec![3.0, 6.0];
        let result = codcel_frequency(data_array, bins_array).unwrap();
        println!("{result:?}");
        assert_eq!(result, vec![3, 3, 4]); // 3 values <= 3, 3 values <= 6 but > 3, 4 values > 6
    }

    #[test]
    fn test_frequency_with_duplicate_values() {
        // =FREQUENCY({1,2,2,3,3,3,4,4,4,4},{2,4}) in US format
        // =FREQUENCY({1;2;2;3;3;3;4;4;4;4};{2;4}) in German format
        let data_array = vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0];
        let bins_array = vec![2.0, 4.0];
        let result = codcel_frequency(data_array, bins_array).unwrap();
        println!("{result:?}");
        assert_eq!(result, vec![3, 7, 0]); // 3 values <= 2, 3 values <= 4 but > 2, 4 values > 4
    }

    #[test]
    fn test_frequency_with_negative_values() {
        // =FREQUENCY({-5,-4,-3,-2,-1,0,1,2,3,4,5},{-3,0,3}) in US format
        // =FREQUENCY({-5;-4;-3;-2;-1;0;1;2;3;4;5};{-3;0;3}) in German format
        let data_array = vec![-5.0, -4.0, -3.0, -2.0, -1.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let bins_array = vec![-3.0, 0.0, 3.0];
        let result = codcel_frequency(data_array, bins_array).unwrap();
        println!("{result:?}");
        assert_eq!(result, vec![3, 3, 3, 2]); // 3 values <= -3, 3 values <= 0 but > -3, 3 values <= 3 but > 0, 2 values > 3
    }

    #[test]
    fn test_frequency_empty_data_array() {
        // Empty data_array should return an error
        let data_array: Vec<f64> = vec![];
        let bins_array = vec![5.0, 10.0];
        let result = codcel_frequency(data_array, bins_array);
        assert!(result.is_err());
    }

    #[test]
    fn test_frequency_empty_bins_array() {
        // Empty bins_array should return an error
        let data_array = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let bins_array: Vec<f64> = vec![];
        let result = codcel_frequency(data_array, bins_array);
        assert!(result.is_err());
    }

    #[test]
    fn test_frequency_single_bin() {
        // =FREQUENCY({1,2,3,4,5,6,7,8,9,10},{5}) in US format
        // =FREQUENCY({1;2;3;4;5;6;7;8;9;10};{5}) in German format
        let data_array = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let bins_array = vec![5.0];
        let result = codcel_frequency(data_array, bins_array).unwrap();
        println!("{result:?}");
        assert_eq!(result, vec![5, 5]); // 5 values <= 5, 5 values > 5
    }
}
