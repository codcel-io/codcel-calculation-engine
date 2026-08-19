// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSum;
use std::error::Error;

/// Calculates the variable declining balance depreciation for an asset.
///
/// # Arguments
/// * `cost` - The initial cost of the asset.
/// * `salvage` - The salvage value of the asset at the end of its useful life.
/// * `life` - The useful life of the asset.
/// * `start_period` - The starting period for which to calculate depreciation.
/// * `end_period` - The ending period for which to calculate depreciation.
/// * `factor` - The rate at which the balance declines (optional, defaults to 2.0).
/// * `no_switch` - Whether to switch to straight-line depreciation when it becomes greater than declining balance (optional, defaults to false).
///
/// # Returns
/// The depreciation of the asset for the specified period.
pub fn codcel_vdb(
    cost: f64,
    salvage: f64,
    life: f64,
    start_period: f64,
    end_period: f64,
    factor: Option<f64>,
    no_switch: Option<bool>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let factor = factor.unwrap_or(2.0);
    let no_switch = no_switch.unwrap_or(false);

    // Validate inputs
    if life <= 0.0 {
        return Err("VDB: The useful life (life) must be greater than zero.".into());
    }
    if cost < 0.0 || salvage < 0.0 {
        return Err("VDB: Both cost and salvage must be non-negative.".into());
    }
    if salvage > cost {
        return Err("VDB: Salvage value cannot be greater than the initial cost.".into());
    }
    if start_period < 0.0 || end_period <= start_period {
        return Err("VDB: Start period must be non-negative, and end period must be greater than start period.".into());
    }
    if factor <= 0.0 {
        return Err("VDB: Factor must be greater than zero.".into());
    }

    // Initialize variables
    let mut depreciation = CompensatedSum::new();
    let mut current_value = cost;
    let mut period = 1.0; // Start at the first period

    // Iteratively calculate depreciation over the specified periods
    while period <= end_period {
        // Declining balance depreciation for the current period
        let db_depreciation = (current_value * factor / life).min(current_value - salvage);

        // Check if we should switch to straight-line
        if !no_switch {
            let remaining_life = life - period + 1.0;
            if remaining_life > 0.0 {
                let sl_depreciation = (current_value - salvage) / remaining_life;

                if sl_depreciation >= db_depreciation {
                    // Switch to straight-line for this and all remaining periods
                    while period <= end_period {
                        if period > start_period {
                            depreciation.add(sl_depreciation);
                        }
                        period += 1.0;
                    }
                    return Ok(depreciation.total());
                }
            }
        }

        // Apply declining balance depreciation
        if period > start_period {
            depreciation.add(db_depreciation);
        }

        // Update current value of the asset
        current_value -= db_depreciation;

        period += 1.0;
    }

    Ok(depreciation.total())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vdb_basic() {
        // Test with basic parameters
        let result = codcel_vdb(10000.0, 1000.0, 10.0, 0.0, 1.0, None, None).unwrap();
        assert!(result > 0.0);

        // Test with different factor
        let result = codcel_vdb(10000.0, 1000.0, 10.0, 0.0, 1.0, Some(1.5), None).unwrap();
        assert!(result > 0.0);

        // Test with no_switch = true
        let result = codcel_vdb(10000.0, 1000.0, 10.0, 0.0, 1.0, None, Some(true)).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_vdb_error_cases() {
        // Life must be greater than zero
        assert!(codcel_vdb(10000.0, 1000.0, 0.0, 0.0, 1.0, None, None).is_err());

        // Cost and salvage must be non-negative
        assert!(codcel_vdb(-10000.0, 1000.0, 10.0, 0.0, 1.0, None, None).is_err());
        assert!(codcel_vdb(10000.0, -1000.0, 10.0, 0.0, 1.0, None, None).is_err());

        // Salvage value cannot be greater than the initial cost
        assert!(codcel_vdb(1000.0, 10000.0, 10.0, 0.0, 1.0, None, None).is_err());

        // Start period must be non-negative, and end period must be greater than start period
        assert!(codcel_vdb(10000.0, 1000.0, 10.0, -1.0, 1.0, None, None).is_err());
        assert!(codcel_vdb(10000.0, 1000.0, 10.0, 2.0, 1.0, None, None).is_err());

        // Factor must be greater than zero
        assert!(codcel_vdb(10000.0, 1000.0, 10.0, 0.0, 1.0, Some(0.0), None).is_err());
    }
}
