// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Excel-compatible `PROB` that returns the probability that values are between two limits.
/// - `values`: an array of numeric values.
/// - `probabilities`: an array of probabilities associated with each value (must sum to 1).
/// - `lower_limit`: the lower bound of the range.
/// - `upper_limit`: optional upper bound (defaults to `lower_limit` for exact probability).
///
/// Returns the sum of probabilities for values within the specified range,
/// or an error when arrays have different lengths or probabilities are invalid.
pub fn codcel_prob(
    values: Vec<f64>,
    probabilities: Vec<f64>,
    lower_limit: f64,
    upper_limit: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Ensure the lengths of values and probabilities are the same
    if values.len() != probabilities.len() {
        return Err("PROB: Values and probabilities vectors must have the same length.".into());
    }

    // Ensure probabilities sum to 1
    let prob_sum: f64 = probabilities.iter().sum();
    if (prob_sum - 1.0).abs() > f64::EPSILON {
        return Err("PROB: Probabilities must sum to 1.".into());
    }

    // Calculate the probability within the range
    let mut total_probability = 0.0;
    for (value, &probability) in values.iter().zip(probabilities.iter()) {
        if upper_limit.is_none() {
            // If upper_limit is None, only consider values exactly equal to lower_limit
            if *value == lower_limit {
                total_probability += probability;
            }
        } else {
            // Otherwise, consider values within the range [lower_limit, upper_limit]
            if *value >= lower_limit {
                if let Some(upper) = upper_limit {
                    if *value > upper {
                        continue;
                    }
                }
                total_probability += probability;
            }
        }
    }

    Ok(total_probability)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prob_basic() {
        // =PROB({0,1,2,3}, {0.2,0.3,0.1,0.4}, 1, 3) in US format
        // =PROB({0;1;2;3}; {0,2;0,3;0,1;0,4}; 1; 3) in German format
        let values = vec![0.0, 1.0, 2.0, 3.0];
        let probabilities = vec![0.2, 0.3, 0.1, 0.4];
        let result = codcel_prob(values, probabilities, 1.0, Some(3.0)).unwrap();
        assert!((result - 0.8).abs() < 0.0001);
    }

    #[test]
    fn test_prob_single_value() {
        // =PROB({0,1,2,3}, {0.2,0.3,0.1,0.4}, 2) in US format
        // =PROB({0;1;2;3}; {0,2;0,3;0,1;0,4}; 2) in German format
        let values = vec![0.0, 1.0, 2.0, 3.0];
        let probabilities = vec![0.2, 0.3, 0.1, 0.4];
        let result = codcel_prob(values, probabilities, 2.0, None).unwrap();
        assert!((result - 0.1).abs() < 0.0001);
    }

    #[test]
    fn test_prob_range() {
        // =PROB({0,1,2,3}, {0.2,0.3,0.1,0.4}, 0, 2) in US format
        // =PROB({0;1;2;3}; {0,2;0,3;0,1;0,4}; 0; 2) in German format
        let values = vec![0.0, 1.0, 2.0, 3.0];
        let probabilities = vec![0.2, 0.3, 0.1, 0.4];
        let result = codcel_prob(values, probabilities, 0.0, Some(2.0)).unwrap();
        assert!((result - 0.6).abs() < 0.0001);
    }

    #[test]
    fn test_prob_no_values_in_range() {
        // =PROB({0,1,2,3}, {0.2,0.3,0.1,0.4}, 4, 5) in US format
        // =PROB({0;1;2;3}; {0,2;0,3;0,1;0,4}; 4; 5) in German format
        let values = vec![0.0, 1.0, 2.0, 3.0];
        let probabilities = vec![0.2, 0.3, 0.1, 0.4];
        let result = codcel_prob(values, probabilities, 4.0, Some(5.0)).unwrap();
        assert!((result - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_prob_different_length_vectors() {
        // Different length vectors should return an error
        let values = vec![0.0, 1.0, 2.0];
        let probabilities = vec![0.2, 0.3, 0.1, 0.4];
        let result = codcel_prob(values, probabilities, 1.0, Some(3.0));
        assert!(result.is_err());
    }

    #[test]
    fn test_prob_probabilities_not_sum_to_one() {
        // Probabilities not summing to 1 should return an error
        let values = vec![0.0, 1.0, 2.0, 3.0];
        let probabilities = vec![0.2, 0.3, 0.1, 0.3]; // Sum is 0.9
        let result = codcel_prob(values, probabilities, 1.0, Some(3.0));
        assert!(result.is_err());
    }
}
