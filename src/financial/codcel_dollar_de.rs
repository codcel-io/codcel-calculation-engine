// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Converts a dollar price expressed as a fraction into a dollar price expressed as a decimal number.
///
/// # Arguments
/// * `fractional_dollar` - A dollar price expressed as a fraction.
/// * `fraction` - The denominator of the fraction.
///
/// # Returns
/// A dollar price expressed as a decimal number.
pub fn codcel_dollar_de(
    fractional_dollar: f64,
    fraction: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if fraction <= 0 {
        return Err("DOLLARDE: Fraction must be greater than 0".into());
    }

    let sign = if fractional_dollar < 0.0 { -1.0 } else { 1.0 };
    let fractional_dollar = fractional_dollar.abs();

    let integer_part = fractional_dollar.trunc();
    let fractional_part = fractional_dollar.fract();

    // Reduce fraction by dividing by 10 until it's <= 10
    let mut reduced_fraction = fraction as f64;
    while reduced_fraction > 10.0 {
        reduced_fraction /= 10.0;
    }

    // Calculate decimal part using the Excel formula:
    // fractional_part * 10 / reduced_fraction
    let decimal_part = fractional_part * 10.0 / reduced_fraction;

    let result = (integer_part + decimal_part) * sign;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dollar_de_large_fraction() {
        // Test with larger fraction (32)
        let result = codcel_dollar_de(1.02, 32).unwrap();
        assert!((result - 1.0625).abs() < 0.0001); // 1.02 in thirty-seconds is 1.0625 in decimal
    }
}
