// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Calculate the straight-line depreciation of an asset for a single period.
///
/// # Arguments
/// * `cost` - The initial cost of the asset.
/// * `salvage` - The salvage value at the end of the depreciation.
/// * `life` - The number of periods over which the asset is depreciated.
///
/// # Returns
/// The depreciation allowance for a single period.
pub fn codcel_sln(cost: f64, salvage: f64, life: f64) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if life <= 0.0 {
        return Err("SLN: The useful life (life) must be greater than zero.".into());
    }
    if cost < 0.0 || salvage < 0.0 {
        return Err("SLN: Both cost and salvage must be non-negative.".into());
    }
    if salvage > cost {
        return Err("SLN: Salvage value cannot be greater than the initial cost.".into());
    }

    // Straight-line depreciation formula: SLN = (cost - salvage) / life
    let sln = (cost - salvage) / life;

    Ok(sln)
}

/// Vector version of codcel_sln for compatibility with array inputs.
///
/// # Arguments
/// * `inputs` - A vector containing [cost, salvage, life].
///
/// # Returns
/// The depreciation allowance for a single period.
pub fn codcel_sln_vec(inputs: Vec<f64>) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if inputs.len() != 3 {
        return Err("SLN: Must have 3 parameters".into());
    }

    codcel_sln(inputs[0], inputs[1], inputs[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sln_basic() {
        // Test with cost = 10000, salvage = 1000, life = 10
        // This should give a depreciation of 900 per period
        let result = codcel_sln(10000.0, 1000.0, 10.0).unwrap();
        assert_eq!(result, 900.0);
    }

    #[test]
    fn test_sln_vec() {
        // Test vector version
        let result = codcel_sln_vec(vec![10000.0, 1000.0, 10.0]).unwrap();
        assert_eq!(result, 900.0);
    }

    #[test]
    fn test_sln_error_cases() {
        // Test with negative life
        let result = codcel_sln(10000.0, 1000.0, -10.0);
        assert!(result.is_err());

        // Test with negative cost
        let result = codcel_sln(-10000.0, 1000.0, 10.0);
        assert!(result.is_err());

        // Test with negative salvage
        let result = codcel_sln(10000.0, -1000.0, 10.0);
        assert!(result.is_err());

        // Test with salvage > cost
        let result = codcel_sln(1000.0, 10000.0, 10.0);
        assert!(result.is_err());

        // Test vector with wrong number of parameters
        let result = codcel_sln_vec(vec![10000.0, 1000.0]);
        assert!(result.is_err());
    }
}
