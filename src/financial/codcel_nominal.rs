// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Calculates the nominal annual interest rate, given the effective rate and the number of compounding periods per year.
///
/// # Arguments
/// * `effect_rate` - The effective interest rate.
/// * `npery` - The number of compounding periods per year.
///
/// # Returns
/// The nominal annual interest rate.
pub fn codcel_nominal(effect_rate: f64, npery: i32) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if effect_rate <= 0.0 {
        return Err("NOMINAL: Effective rate must be greater than zero".into());
    }
    if npery <= 0 {
        return Err(
            "NOMINAL: Number of compounding periods per year must be greater than zero".into(),
        );
    }
    Ok(npery as f64 * (crate::portable_math::powf(1.0 + effect_rate, 1.0 / npery as f64) - 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal_error_cases() {
        // Effective rate must be greater than zero
        assert!(codcel_nominal(0.0, 4).is_err());
        assert!(codcel_nominal(-0.05, 4).is_err());

        // Number of compounding periods must be greater than zero
        assert!(codcel_nominal(0.05, 0).is_err());
        assert!(codcel_nominal(0.05, -4).is_err());
    }
}
