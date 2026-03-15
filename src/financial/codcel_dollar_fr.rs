// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

/// Converts a dollar price expressed as a decimal number into a dollar price expressed as a fraction.
///
/// # Arguments
/// * `decimal_dollar` - A dollar price expressed as a decimal number.
/// * `fraction` - The denominator of the fraction to use.
///
/// # Returns
/// A dollar price expressed as a fraction.
pub fn codcel_dollar_fr(
    decimal_dollar: f64,
    fraction: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if fraction <= 0 {
        return Err("DOLLARFR: Fraction must be greater than 0".into());
    }

    let sign = if decimal_dollar < 0.0 { -1.0 } else { 1.0 };
    let decimal_dollar = decimal_dollar.abs();

    let integer_part = decimal_dollar.trunc();
    let decimal_part = decimal_dollar.fract();

    // The divisor is 10^ceil(log10(fraction)).
    // This places the fractional numerator into the correct decimal position.
    // e.g. fraction=8 → divisor=10, fraction=16 → divisor=100, fraction=1000 → divisor=1000
    let divisor = if fraction == 1 {
        1.0_f64
    } else {
        let digits = (fraction as f64).log10().ceil() as u32;
        10_f64.powi(digits as i32)
    };

    let numerator = decimal_part * fraction as f64;
    let result = (integer_part + numerator / divisor) * sign;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dollar_fr_basic() {
        // DOLLARFR(1.125, 16) → 1.02  (0.125*16=2, divisor=100 → 2/100=0.02)
        let result = codcel_dollar_fr(1.125, 16).unwrap();
        assert!((result - 1.02).abs() < 0.000001);
    }

    #[test]
    fn test_dollar_fr_eighths() {
        // DOLLARFR(1.125, 8) → 1.1  (0.125*8=1, divisor=10 → 1/10=0.1)
        let result = codcel_dollar_fr(1.125, 8).unwrap();
        assert!((result - 1.1).abs() < 0.000001);
    }

    #[test]
    fn test_dollar_fr_tenths() {
        // DOLLARFR(10.5, 10) → 10.5  (0.5*10=5, divisor=10 → 5/10=0.5)
        let result = codcel_dollar_fr(10.5, 10).unwrap();
        assert!((result - 10.5).abs() < 0.000001);
    }

    #[test]
    fn test_dollar_fr_large_fraction() {
        // DOLLARFR(1.0625, 32) → 1.02  (0.0625*32=2, divisor=100 → 2/100=0.02)
        let result = codcel_dollar_fr(1.0625, 32).unwrap();
        assert!((result - 1.02).abs() < 0.000001);
    }

    #[test]
    fn test_dollar_fr_thousands() {
        // DOLLARFR(1.125, 1000) → 1.125  (0.125*1000=125, divisor=1000 → 125/1000=0.125)
        let result = codcel_dollar_fr(1.125, 1000).unwrap();
        assert!((result - 1.125).abs() < 0.000001);
    }

    #[test]
    fn test_dollar_fr_precision() {
        // DOLLARFR(1.00625, 16) → 1.001  (0.00625*16=0.1, divisor=100 → 0.1/100=0.001)
        let result = codcel_dollar_fr(1.00625, 16).unwrap();
        assert!((result - 1.001).abs() < 0.000001);
    }

    #[test]
    fn test_dollar_fr_negative() {
        // DOLLARFR(-1.125, 16) → -1.02
        let result = codcel_dollar_fr(-1.125, 16).unwrap();
        assert!((result - (-1.02)).abs() < 0.000001);
    }

    #[test]
    fn test_dollar_fr_fraction_one() {
        // DOLLARFR(10, 1) → 10  (no fractional part)
        let result = codcel_dollar_fr(10.0, 1).unwrap();
        assert!((result - 10.0).abs() < 0.000001);
    }

    #[test]
    fn test_dollar_fr_large_frac_case() {
        // DOLLARFR(3.05, 1000) → 3.05  (0.05*1000=50, divisor=1000 → 50/1000=0.05)
        let result = codcel_dollar_fr(3.05, 1000).unwrap();
        assert!((result - 3.05).abs() < 0.000001);
    }
}
