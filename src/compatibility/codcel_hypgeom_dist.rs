// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

/// Excel-compatible `HYPGEOMDIST` function.
/// Returns the hypergeometric probability of drawing exactly `sample_successes` successes.
/// - `sample_successes`: number of successes in the sample (non-negative, <= min of sample_size and total_successes).
/// - `sample_size`: number of draws from the population (0 to population_size).
/// - `total_successes`: total successes in the population (0 to population_size).
/// - `population_size`: total population size (must be greater than 0).
///
/// Returns an error when counts are outside their valid ranges.
pub fn codcel_hypgeom_dist(
    sample_successes: i32,
    sample_size: i32,
    total_successes: i32,
    population_size: i32,
) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    if population_size <= 0 {
        return Err("HYPGEOMDIST: Population size must be greater than 0".into());
    }
    if sample_size < 0 || sample_size > population_size {
        return Err("HYPGEOMDIST: Sample size must be between 0 and the population size".into());
    }
    if total_successes < 0 || total_successes > population_size {
        return Err(
            "HYPGEOMDIST: Total successes must be between 0 and the population size".into(),
        );
    }
    if sample_successes < 0 || sample_successes > sample_size || sample_successes > total_successes
    {
        return Err("HYPGEOMDIST: Sample successes must be between 0 and the minimum of sample size and total successes".into());
    }

    fn combination(n: i32, k: i32) -> f64 {
        if k > n || k < 0 {
            return 0.0;
        }
        (1..=k).fold(1.0, |acc, i| acc * (n - i + 1) as f64 / i as f64)
    }

    let success_comb = combination(total_successes, sample_successes);
    let failure_comb = combination(
        population_size - total_successes,
        sample_size - sample_successes,
    );
    let total_comb = combination(population_size, sample_size);

    let probability = (success_comb * failure_comb) / total_comb;

    Ok(probability)
}

/// Convenience wrapper for `HYPGEOMDIST` that accepts four parameters in order:
/// `[sample_successes, sample_size, total_successes, population_size]`.
/// Errors if the vector does not contain exactly four values.
pub fn codcel_hypgeom_dist_vec(
    inputs: Vec<i32>,
) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    if inputs.len() != 4 {
        return Err("HYPGEOMDIST: Must have 4 parameters.".into());
    }

    codcel_hypgeom_dist(inputs[0], inputs[1], inputs[2], inputs[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hypgeom_dist_basic() {
        // =HYPGEOMDIST(1, 4, 8, 20)
        let result = codcel_hypgeom_dist(1, 4, 8, 20).unwrap();
        assert!((result - 0.3632610939112487).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dist_two_successes() {
        // =HYPGEOMDIST(2, 5, 10, 20)
        let result = codcel_hypgeom_dist(2, 5, 10, 20).unwrap();
        assert!((result - 0.3482972136222912).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dist_zero_sample_successes() {
        // =HYPGEOMDIST(0, 4, 8, 20)
        let result = codcel_hypgeom_dist(0, 4, 8, 20).unwrap();
        assert!((result - 0.10216718266253864).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dist_all_successes_in_sample() {
        // =HYPGEOMDIST(4, 4, 8, 20)
        let result = codcel_hypgeom_dist(4, 4, 8, 20).unwrap();
        assert!((result - 0.014447884416924652).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dist_coin_flip() {
        // =HYPGEOMDIST(1, 1, 10, 20)
        let result = codcel_hypgeom_dist(1, 1, 10, 20).unwrap();
        assert!((result - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dist_large_population() {
        // =HYPGEOMDIST(5, 10, 50, 100)
        let result = codcel_hypgeom_dist(5, 10, 50, 100).unwrap();
        assert!((result - 0.25933354622553517).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dist_num_error_sample_gt_pop() {
        // =HYPGEOMDIST(2, 10, 4, 5) gives #NUM! in Excel (sample_size=10 > population_size=5)
        let result = codcel_hypgeom_dist(2, 10, 4, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dist_negative_sample_successes() {
        let result = codcel_hypgeom_dist(-1, 4, 8, 20);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dist_sample_successes_too_large() {
        // sample_successes=5 > sample_size=4
        let result = codcel_hypgeom_dist(5, 4, 8, 20);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dist_zero_population() {
        let result = codcel_hypgeom_dist(0, 0, 0, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dist_negative_population() {
        let result = codcel_hypgeom_dist(0, 4, 2, -10);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dist_negative_total_successes() {
        let result = codcel_hypgeom_dist(0, 4, -2, 20);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dist_total_successes_too_large() {
        // total_successes=25 > population_size=20
        let result = codcel_hypgeom_dist(0, 4, 25, 20);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dist_negative_sample_size() {
        let result = codcel_hypgeom_dist(0, -4, 8, 20);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dist_sample_size_too_large() {
        // sample_size=25 > population_size=20
        let result = codcel_hypgeom_dist(0, 25, 8, 20);
        assert!(result.is_err());
    }

    #[test]
    fn test_hypgeom_dist_vec_basic() {
        // =HYPGEOMDIST(1, 4, 8, 20)
        let result = codcel_hypgeom_dist_vec(vec![1, 4, 8, 20]).unwrap();
        assert!((result - 0.3632610939112487).abs() < 1e-10);
    }

    #[test]
    fn test_hypgeom_dist_vec_wrong_params() {
        let result = codcel_hypgeom_dist_vec(vec![2, 10, 4]);
        assert!(result.is_err());
    }
}
