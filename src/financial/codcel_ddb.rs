// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Calculates the depreciation of an asset using the double-declining balance method.
///
/// # Arguments
/// * `cost` - The initial cost of the asset.
/// * `salvage` - The salvage value of the asset at the end of its useful life.
/// * `life` - The useful life of the asset in years.
/// * `period` - The period for which to calculate the depreciation.
/// * `factor` - Optional. The rate at which the balance declines. If omitted, 2 is used (double-declining).
///
/// # Returns
/// The depreciation of the asset for the specified period.
pub fn codcel_ddb(
    cost: f64,
    salvage: f64,
    life: f64,
    period: f64,
    factor: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Input validation
    if cost <= 0.0 || salvage < 0.0 || life <= 0.0 || period <= 0.0 {
        return Err("DDB: Invalid input parameters".into());
    }
    if period > life {
        return Ok(0.0);
    }

    // Default factor is 2 (double declining)
    let factor = factor.unwrap_or(2.0);

    // Calculate the depreciation rate
    let rate = factor / life;

    // Handle the case where calculated depreciation would push book value below salvage
    let mut book_value = cost;

    for current_period in 1..=period.ceil() as i32 {
        let depreciation = book_value * rate;

        // If this is our target period
        if current_period as f64 == period {
            // For partial periods, prorate the depreciation
            let partial = period.fract();
            if partial > 0.0 {
                let full_period_depreciation = depreciation;
                return Ok(full_period_depreciation * partial);
            }
            // Check if this depreciation would push us below salvage value
            if (book_value - depreciation) < salvage {
                return Ok(book_value - salvage);
            }
            return Ok(depreciation);
        }

        // For all periods before our target
        if (book_value - depreciation) < salvage {
            book_value = salvage;
            break;
        }

        book_value -= depreciation;
    }

    // If we've already hit salvage value, return 0
    if book_value <= salvage {
        return Ok(0.0);
    }

    Ok(book_value * rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ddb_basic() {
        // Example: $10,000 asset with $1,000 salvage value over 5 years
        let cost = 10000.0;
        let salvage = 1000.0;
        let life = 5.0;

        // First year depreciation (40% of cost)
        let result1 = codcel_ddb(cost, salvage, life, 1.0, None).unwrap();
        assert!((result1 - 4000.0).abs() < 0.01);

        // Second year depreciation (40% of remaining book value)
        let result2 = codcel_ddb(cost, salvage, life, 2.0, None).unwrap();
        assert!((result2 - 2400.0).abs() < 0.01);
    }

    #[test]
    fn test_ddb_custom_factor() {
        // Test with custom factor (1.5 instead of 2)
        let result = codcel_ddb(10000.0, 1000.0, 5.0, 1.0, Some(1.5)).unwrap();
        assert!((result - 3000.0).abs() < 0.01); // 30% of cost
    }
}
