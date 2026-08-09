// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Excel-compatible `PI` that returns the value of the mathematical constant π.
///
/// Returns 3.14159265358979... (approximately).
pub fn codcel_pi() -> Result<f64, Box<dyn Error + Send + Sync>> {
    Ok(std::f64::consts::PI)
}

#[cfg(test)]
mod tests {
    // Literals such as 3.14159 and 1.41421 are Excel-visible values under test,
    // not stand-ins for std::f64::consts.
    #![allow(clippy::approx_constant)]
    use super::*;

    #[test]
    fn test_pi() {
        // =PI() in US format
        // =PI() in German format
        let result = codcel_pi().unwrap();
        assert!((result - 3.141592653589793).abs() < 1e-10);
    }

    #[test]
    fn test_pi_calculations() {
        // =PI()*2 in US format
        // =PI()*2 in German format
        let result = codcel_pi().unwrap() * 2.0;
        assert!((result - 6.283185307179586).abs() < 1e-10);
    }

    #[test]
    fn test_pi_equals_std_constant() {
        // Verify that the function returns the standard library's PI constant
        let result = codcel_pi().unwrap();
        assert_eq!(result, std::f64::consts::PI);
    }
}
