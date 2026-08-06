// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;
use std::f64::consts::PI;

/// Euler-Mascheroni constant
const GAMMA: f64 = 0.5772156649015329;

/// Excel-compatible `BESSELK` that returns the modified Bessel function of the second kind K_n(x).
/// - `x`: the value at which to evaluate the function (must be positive).
/// - `n`: the order of the Bessel function (must be non-negative).
///   Returns K_n(x), or an error when `x` is not positive or `n` is negative.
pub fn codcel_bessel_k(x: f64, n: i32) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if x <= 0.0 {
        return Err("x must be positive".into());
    }
    if n < 0 {
        return Err("n must be non-negative".into());
    }

    let k0 = bessel_k0(x);
    if n == 0 {
        return Ok(k0);
    }

    let k1 = bessel_k1(x);
    if n == 1 {
        return Ok(k1);
    }

    // Forward recurrence: K_{n+1}(x) = (2n/x) * K_n(x) + K_{n-1}(x)
    // This is numerically stable for K_n (which increases with n).
    let mut k_prev = k0;
    let mut k_curr = k1;
    for i in 1..n {
        let k_next = (2.0 * i as f64 / x) * k_curr + k_prev;
        k_prev = k_curr;
        k_curr = k_next;
    }

    Ok(k_curr)
}

/// Compute K_0(x) using series for small x, asymptotic for large x.
fn bessel_k0(x: f64) -> f64 {
    if x <= 8.0 {
        bessel_k0_series(x)
    } else {
        bessel_k_asymptotic(x, 0)
    }
}

/// Compute K_1(x) using series for small x, asymptotic for large x.
fn bessel_k1(x: f64) -> f64 {
    if x <= 8.0 {
        bessel_k1_series(x)
    } else {
        bessel_k_asymptotic(x, 1)
    }
}

/// K_0(x) via series expansion (A&S 9.6.13):
/// K_0(x) = -(ln(x/2) + γ) * I_0(x) + Σ_{k=1}^∞ H_k * (x²/4)^k / (k!)²
/// where H_k = 1 + 1/2 + ... + 1/k (harmonic numbers).
fn bessel_k0_series(x: f64) -> f64 {
    let i0 = bessel_i0(x);
    let x_half = x / 2.0;
    let t = x_half * x_half; // x²/4

    let mut series_sum = 0.0;
    let mut factorial = 1.0;
    let mut h_k = 0.0; // H_k = harmonic number

    for k in 1..50 {
        factorial *= k as f64;
        h_k += 1.0 / k as f64;
        let term = t.powi(k) / (factorial * factorial) * h_k;
        series_sum += term;

        if term.abs() < 1e-16 * series_sum.abs() {
            break;
        }
    }

    -(crate::portable_math::ln(x_half) + GAMMA) * i0 + series_sum
}

/// K_1(x) via series expansion (derived from A&S 9.6.11 with n=1):
/// K_1(x) = 1/x + ln(x/2) * I_1(x) - (x/4) * Σ_{k=0}^∞ [ψ(k+1) + ψ(k+2)] * (x²/4)^k / {k! * (k+1)!}
/// where ψ(k+1) = -γ + H_k, ψ(k+2) = -γ + H_{k+1}.
fn bessel_k1_series(x: f64) -> f64 {
    let i1 = bessel_i1(x);
    let x_half = x / 2.0;
    let t = x_half * x_half; // x²/4

    let mut series = 0.0;
    let mut factor = 1.0; // (x²/4)^k / (k! * (k+1)!)
    let mut h_k = 0.0; // H_k

    for k in 0..50 {
        // ψ(k+1) + ψ(k+2) = -2γ + 2*H_k + 1/(k+1)
        let coeff = -2.0 * GAMMA + 2.0 * h_k + 1.0 / (k as f64 + 1.0);
        let term = coeff * factor;
        series += term;

        if k > 0 && term.abs() < 1e-16 * series.abs() {
            break;
        }

        h_k += 1.0 / (k as f64 + 1.0);
        factor *= t / ((k as f64 + 1.0) * (k as f64 + 2.0));
    }

    1.0 / x + crate::portable_math::ln(x_half) * i1 - (x / 4.0) * series
}

/// Asymptotic expansion for K_ν(x), valid for large x:
/// K_ν(x) ≈ sqrt(π/(2x)) * exp(-x) * Σ_{k=0}^N a_k(ν) / x^k
/// where a_k(ν) = [(4ν²-1²)(4ν²-3²)...(4ν²-(2k-1)²)] / (k! * 8^k)
fn bessel_k_asymptotic(x: f64, n: i32) -> f64 {
    let pi_factor = crate::portable_math::sqrt(PI / (2.0 * x));
    let exp_factor = crate::portable_math::exp(-x);
    let nu = n as f64;
    let four_nu_sq = 4.0 * nu * nu;

    let mut sum: f64 = 1.0;
    let mut term: f64 = 1.0;
    let mut prev_abs = f64::MAX;

    for k in 1..50 {
        let kf = k as f64;
        let odd = 2.0 * kf - 1.0;
        term *= (four_nu_sq - odd * odd) / (kf * 8.0 * x);
        let abs_term = term.abs();

        // Optimal truncation: stop when terms start growing
        if abs_term >= prev_abs {
            break;
        }
        if abs_term < 1e-16 * sum.abs() {
            sum += term;
            break;
        }
        sum += term;
        prev_abs = abs_term;
    }

    pi_factor * exp_factor * sum
}

/// Modified Bessel function I_0(x) via series:
/// I_0(x) = Σ_{k=0}^∞ (x²/4)^k / (k!)²
fn bessel_i0(x: f64) -> f64 {
    let mut sum = 1.0;
    let mut term = 1.0;
    let x_half = x / 2.0;

    for k in 1..50 {
        term *= (x_half * x_half) / (k as f64 * k as f64);
        sum += term;

        if term.abs() < 1e-16 * sum.abs() {
            break;
        }
    }

    sum
}

/// Modified Bessel function I_1(x) via series:
/// I_1(x) = (x/2) * Σ_{k=0}^∞ (x²/4)^k / {k! * (k+1)!}
fn bessel_i1(x: f64) -> f64 {
    let x_half = x / 2.0;
    let t = x_half * x_half;
    let mut sum = 1.0;
    let mut term = 1.0;

    for k in 1..50 {
        term *= t / (k as f64 * (k as f64 + 1.0));
        sum += term;

        if term.abs() < 1e-16 * sum.abs() {
            break;
        }
    }

    x_half * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bessel_k0_at_1() {
        let result = codcel_bessel_k(1.0, 0).unwrap();
        assert!((result - 0.4210244382).abs() < 1e-8);
    }

    #[test]
    fn test_bessel_k1_at_1() {
        let result = codcel_bessel_k(1.0, 1).unwrap();
        assert!((result - 0.6019072302).abs() < 1e-8);
    }

    #[test]
    fn test_bessel_k2_at_1() {
        let result = codcel_bessel_k(1.0, 2).unwrap();
        assert!((result - 1.6248388986).abs() < 1e-6);
    }

    #[test]
    fn test_bessel_k1_at_1_5() {
        let result = codcel_bessel_k(1.5, 1).unwrap();
        assert!((result - 0.2773878005).abs() < 1e-6);
    }

    #[test]
    fn test_bessel_k2_at_2_5() {
        let result = codcel_bessel_k(2.5, 2).unwrap();
        assert!((result - 0.1214602062).abs() < 1e-6);
    }

    #[test]
    fn test_bessel_k0_at_5() {
        let result = codcel_bessel_k(5.0, 0).unwrap();
        assert!((result - 0.003691098).abs() < 1e-6);
    }

    #[test]
    fn test_bessel_k_negative_x() {
        assert!(codcel_bessel_k(-1.0, 0).is_err());
    }

    #[test]
    fn test_bessel_k_negative_order() {
        assert!(codcel_bessel_k(1.0, -1).is_err());
    }
}
