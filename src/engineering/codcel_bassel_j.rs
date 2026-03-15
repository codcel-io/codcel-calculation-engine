// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::engineering::factorial::factorial;
use std::error::Error;

/// Excel-compatible `BESSELJ` that returns the Bessel function of the first kind J_n(x).
/// - `x`: the value at which to evaluate the function.
/// - `n`: the order of the Bessel function (negative orders use `(-1)^n` symmetry).
///   Returns J_n(x), or an error when input is NaN or computation overflows.
pub fn codcel_bessel_j(x: f64, n: i32) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Handle invalid input
    if x.is_nan() {
        return Err("Input x is NaN".into());
    }

    // Special case for x = 0, Jn(x) = 0 for n > 0, and J0(0) = 1
    if x == 0.0 {
        return if n == 0 { Ok(1.0) } else { Ok(0.0) };
    }

    // For negative orders, use the relation J_{-n}(x) = (-1)^n * J_n(x)
    let n_abs = n.abs();
    let sign_correction = if n < 0 && n % 2 != 0 { -1.0 } else { 1.0 };

    // Initialize variables for series calculation
    let mut sum: f64 = 0.0;
    let mut term: f64 = 1.0;
    let mut k: u32 = 0;
    let mut factorial_k: f64 = 1.0;
    let mut factorial_n_plus_k: f64 = factorial(n_abs as u64)?;
    let x_half: f64 = x / 2.0;
    let x_half_pow_n: f64 = x_half.powi(n_abs);

    // Calculate using series expansion
    // J_n(x) = (x/2)^n * sum_{k=0}^∞ (-1)^k * (x^2/4)^k / (k! * (n+k)!)
    while k < 50 && term.abs() > 1e-15 {
        if k > 0 {
            factorial_k *= k as f64;
            factorial_n_plus_k *= (n_abs as u64 + k as u64) as f64;
        }

        term = x_half_pow_n * (x_half * x_half).powi(k as i32) / (factorial_k * factorial_n_plus_k);

        // Alternate signs for Jn's series expansion
        if !k.is_multiple_of(2) {
            term = -term;
        }

        sum += term;
        k += 1;

        // Check for overflow
        if sum.is_infinite() {
            return Err("BESSELJ: Computation overflow".into());
        }
    }

    if sum.is_nan() {
        return Err("BESSELJ: Computation resulted in NaN".into());
    }

    Ok(sign_correction * sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bessel_j_basic() {
        // =BESSELJ(1, 0) in US format
        // =BESSELJ(1; 0) in German format
        let result = codcel_bessel_j(1.0, 0).unwrap();
        println!("{result}");
        assert!((result - 0.7651976865579666).abs() < 0.0001);
    }

    #[test]
    fn test_bessel_j_order_1() {
        // =BESSELJ(1, 1) in US format
        // =BESSELJ(1; 1) in German format
        let result = codcel_bessel_j(1.0, 1).unwrap();
        println!("{result}");
        assert!((result - 0.44005058574493355).abs() < 0.0001);
    }

    #[test]
    fn test_bessel_j_order_2() {
        // =BESSELJ(1, 2) in US format
        // =BESSELJ(1; 2) in German format
        let result = codcel_bessel_j(1.0, 2).unwrap();
        println!("{result}");
        assert!((result - 0.11490348493190048).abs() < 0.0001);
    }

    #[test]
    fn test_bessel_j_negative_order() {
        // =BESSELJ(1, -1) in US format
        // =BESSELJ(1; -1) in German format
        let result = codcel_bessel_j(1.0, -1).unwrap();
        println!("{result}");
        assert!((result - (-0.44005058574493355)).abs() < 0.0001);
    }

    #[test]
    fn test_bessel_j_zero_x() {
        // =BESSELJ(0, 0) in US format
        // =BESSELJ(0; 0) in German format
        let result = codcel_bessel_j(0.0, 0).unwrap();
        println!("{result}");
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_bessel_j_zero_x_nonzero_order() {
        // =BESSELJ(0, 1) in US format
        // =BESSELJ(0; 1) in German format
        let result = codcel_bessel_j(0.0, 1).unwrap();
        println!("{result}");
        assert_eq!(result, 0.0);
    }
}
