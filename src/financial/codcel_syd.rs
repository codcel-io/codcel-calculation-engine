// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Calculate the sum-of-years' digits depreciation of an asset for a specified period.
///
/// # Arguments
/// * `cost` - The initial cost of the asset.
/// * `salvage` - The salvage value at the end of the depreciation.
/// * `life` - The number of periods over which the asset is depreciated.
/// * `period` - The period for which you want to calculate the depreciation.
///
/// # Returns
/// The depreciation allowance for the specified period.
pub fn codcel_syd(
    cost: f64,
    salvage: f64,
    life: f64,
    period: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if life <= 0.0 {
        return Err("SYD: The useful life (life) must be greater than zero.".into());
    }
    if period <= 0.0 || period > life {
        return Err(
            "SYD: The period must be greater than zero and less than or equal to the useful life."
                .into(),
        );
    }
    if cost < 0.0 || salvage < 0.0 {
        return Err("SYD: Both cost and salvage must be non-negative.".into());
    }
    if salvage > cost {
        return Err("SYD: Salvage value cannot be greater than the initial cost.".into());
    }

    // Sum of years formula
    let sum_of_years = (life * (life + 1.0)) / 2.0;

    // SYD formula: SYD = ((cost - salvage) * (life - period + 1)) / sum_of_years
    let syd = ((cost - salvage) * (life - period + 1.0)) / sum_of_years;

    Ok(syd)
}

/// Vector version of codcel_syd for compatibility with array inputs.
///
/// # Arguments
/// * `inputs` - A vector containing [cost, salvage, life, period].
///
/// # Returns
/// The depreciation allowance for the specified period.
pub fn codcel_syd_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 4 {
        return Err("SYD: Must have 4 parameters.".into());
    }

    codcel_syd(inputs[0], inputs[1], inputs[2], inputs[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syd_basic() {
        // Test with cost = 10000, salvage = 1000, life = 10, period = 1
        // This should give a depreciation of 1636.36 for the first period
        let result = codcel_syd(10000.0, 1000.0, 10.0, 1.0).unwrap();
        assert!((result - 1636.36).abs() < 0.01);

        // Test with period = 10 (last period)
        // This should give a depreciation of 163.64 for the last period
        let result = codcel_syd(10000.0, 1000.0, 10.0, 10.0).unwrap();
        assert!((result - 163.64).abs() < 0.01);
    }

    #[test]
    fn test_syd_vec() {
        // Test vector version
        let result = codcel_syd_vec(vec![10000.0, 1000.0, 10.0, 1.0]).unwrap();
        assert!((result - 1636.36).abs() < 0.01);
    }

    #[test]
    fn test_syd_error_cases() {
        // Test with negative life
        let result = codcel_syd(10000.0, 1000.0, -10.0, 1.0);
        assert!(result.is_err());

        // Test with period > life
        let result = codcel_syd(10000.0, 1000.0, 10.0, 11.0);
        assert!(result.is_err());

        // Test with period <= 0
        let result = codcel_syd(10000.0, 1000.0, 10.0, 0.0);
        assert!(result.is_err());

        // Test with negative cost
        let result = codcel_syd(-10000.0, 1000.0, 10.0, 1.0);
        assert!(result.is_err());

        // Test with negative salvage
        let result = codcel_syd(10000.0, -1000.0, 10.0, 1.0);
        assert!(result.is_err());

        // Test with salvage > cost
        let result = codcel_syd(1000.0, 10000.0, 10.0, 1.0);
        assert!(result.is_err());

        // Test vector with wrong number of parameters
        let result = codcel_syd_vec(vec![10000.0, 1000.0, 10.0]);
        assert!(result.is_err());
    }
}
