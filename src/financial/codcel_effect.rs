// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Calculates the effective annual interest rate from a nominal annual interest rate and the number of compounding periods per year.
///
/// # Arguments
/// * `nominal_rate` - The nominal annual interest rate.
/// * `npery` - The number of compounding periods per year.
///
/// # Returns
/// The effective annual interest rate.
pub fn codcel_effect(nominal_rate: f64, npery: i32) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if nominal_rate <= 0.0 {
        return Err("EFFECT: Nominal rate must be greater than 0".into());
    }
    if npery <= 0 {
        return Err("EFFECT: Number of periods (npery) must be greater than 0".into());
    }

    let effective_rate = crate::portable_math::powf(1.0 + nominal_rate / npery as f64, npery as f64) - 1.0;

    Ok(effective_rate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effect_basic() {
        // Example: 5% nominal rate compounded quarterly
        let result = codcel_effect(0.05, 4).unwrap();
        assert!((result - 0.05095).abs() < 0.0001); // Should be approximately 5.095%
    }

    #[test]
    fn test_effect_error_cases() {
        // Nominal rate must be greater than 0
        assert!(codcel_effect(0.0, 4).is_err());
        assert!(codcel_effect(-0.05, 4).is_err());

        // Number of periods must be greater than 0
        assert!(codcel_effect(0.05, 0).is_err());
        assert!(codcel_effect(0.05, -4).is_err());
    }
}
