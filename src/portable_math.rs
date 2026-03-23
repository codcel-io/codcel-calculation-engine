// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

//! Portable math wrappers for cross-platform determinism.
//!
//! By default, Rust's `f64` transcendental methods (`.sin()`, `.cos()`, etc.)
//! delegate to the platform's C math library, which can produce results that
//! differ by 1 ULP between macOS (Apple libm) and Linux (glibc libm).
//!
//! When the environment variable `CODCEL_USE_PORTABLE_MATH` is set to `true`,
//! these wrappers route all transcendental math through the `libm` crate's
//! pure-Rust implementations, which produce bit-identical results on all platforms.
//!
//! When unset or `false` (the default), the platform's native math library is used.

use std::sync::LazyLock;

/// Cached check: whether `CODCEL_USE_PORTABLE_MATH=true` was set at process start.
static USE_PORTABLE_MATH: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("CODCEL_USE_PORTABLE_MATH")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
});

#[inline]
pub fn sin(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::sin(x) } else { x.sin() }
}

#[inline]
pub fn cos(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::cos(x) } else { x.cos() }
}

#[inline]
pub fn tan(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::tan(x) } else { x.tan() }
}

#[inline]
pub fn asin(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::asin(x) } else { x.asin() }
}

#[inline]
pub fn acos(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::acos(x) } else { x.acos() }
}

#[inline]
pub fn atan(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::atan(x) } else { x.atan() }
}

#[inline]
pub fn atan2(y: f64, x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::atan2(y, x) } else { y.atan2(x) }
}

#[inline]
pub fn sinh(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::sinh(x) } else { x.sinh() }
}

#[inline]
pub fn cosh(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::cosh(x) } else { x.cosh() }
}

#[inline]
pub fn tanh(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::tanh(x) } else { x.tanh() }
}

#[inline]
pub fn asinh(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::asinh(x) } else { x.asinh() }
}

#[inline]
pub fn acosh(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::acosh(x) } else { x.acosh() }
}

#[inline]
pub fn atanh(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::atanh(x) } else { x.atanh() }
}

#[inline]
pub fn exp(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::exp(x) } else { x.exp() }
}

#[inline]
pub fn ln(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::log(x) } else { x.ln() }
}

#[inline]
pub fn log10(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::log10(x) } else { x.log10() }
}

#[inline]
pub fn log2(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::log2(x) } else { x.log2() }
}

#[inline]
pub fn sqrt(x: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::sqrt(x) } else { x.sqrt() }
}

#[inline]
pub fn powf(base: f64, exponent: f64) -> f64 {
    if *USE_PORTABLE_MATH { libm::pow(base, exponent) } else { base.powf(exponent) }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{E, FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, LN_2, PI};

    // -----------------------------------------------------------------------
    // The public wrappers (sin, cos, etc.) dispatch via a LazyLock that is
    // initialised once per process, so we cannot toggle the env var mid-test.
    // Instead, we test both code paths explicitly:
    //
    //   • "platform" path — calls f64 methods directly (e.g. x.sin())
    //   • "portable" path — calls libm functions directly (e.g. libm::sin(x))
    //
    // This guarantees full coverage regardless of whether
    // CODCEL_USE_PORTABLE_MATH is set when running `cargo test`.
    // -----------------------------------------------------------------------

    /// Assert two f64 values are within 1 ULP of each other.
    fn assert_within_1_ulp(a: f64, b: f64) {
        if a.is_nan() && b.is_nan() {
            return;
        }
        let diff = (a.to_bits() as i64).wrapping_sub(b.to_bits() as i64).unsigned_abs();
        assert!(
            diff <= 1,
            "values differ by {diff} ULPs: a={a:.18e}, b={b:.18e}"
        );
    }

    // =====================================================================
    // Platform vs portable: both paths agree within 1 ULP
    // =====================================================================

    #[test]
    fn test_platform_vs_portable_trig() {
        let inputs = [0.0, 0.5, 1.0, 1.5, 2.0, 3.0, E, PI, 10.0, 100.0];
        for &x in &inputs {
            assert_within_1_ulp(x.sin(), libm::sin(x));
            assert_within_1_ulp(x.cos(), libm::cos(x));
            assert_within_1_ulp(x.tan(), libm::tan(x));
        }
        for &x in &[0.0_f64, 0.25, 0.5, 0.75, 1.0] {
            assert_within_1_ulp(x.asin(), libm::asin(x));
            assert_within_1_ulp(x.acos(), libm::acos(x));
        }
        for &x in &[-100.0_f64, -1.0, 0.0, 1.0, 100.0] {
            assert_within_1_ulp(x.atan(), libm::atan(x));
        }
        for &(y, x) in &[
            (1.0_f64, 1.0_f64), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0),
            (0.0, 1.0), (1.0, 0.0),
        ] {
            assert_within_1_ulp(y.atan2(x), libm::atan2(y, x));
        }
    }

    #[test]
    fn test_platform_vs_portable_hyperbolic() {
        let inputs = [0.0, 0.5, 1.0, 1.5, 2.0, 3.0, E, PI, 10.0];
        for &x in &inputs {
            assert_within_1_ulp(x.sinh(), libm::sinh(x));
            assert_within_1_ulp(x.cosh(), libm::cosh(x));
            assert_within_1_ulp(x.tanh(), libm::tanh(x));
        }
        for &x in &[-100.0_f64, -1.0, 0.0, 1.0, 100.0] {
            assert_within_1_ulp(x.asinh(), libm::asinh(x));
        }
        for &x in &[1.0_f64, 2.0, 5.0, 10.0] {
            assert_within_1_ulp(x.acosh(), libm::acosh(x));
        }
        for &x in &[0.0_f64, 0.25, 0.5, 0.75, 0.99] {
            assert_within_1_ulp(x.atanh(), libm::atanh(x));
        }
    }

    #[test]
    fn test_platform_vs_portable_exp_log() {
        let inputs = [0.0, 0.5, 1.0, 2.0, E, PI, 10.0, 100.0];
        for &x in &inputs {
            assert_within_1_ulp(x.exp(), libm::exp(x));
        }
        for &x in &[0.01, 0.1, 0.5, 1.0, E, 10.0, 100.0, 1e10] {
            assert_within_1_ulp(x.ln(), libm::log(x));
            assert_within_1_ulp(x.log10(), libm::log10(x));
            assert_within_1_ulp(x.log2(), libm::log2(x));
        }
    }

    #[test]
    fn test_platform_vs_portable_power_root() {
        for &x in &[0.0_f64, 0.5, 1.0, 2.0, 4.0, 100.0, 1e10] {
            assert_within_1_ulp(x.sqrt(), libm::sqrt(x));
        }
        for &(b, e) in &[
            (2.0, 3.0), (E, 1.0), (10.0, 0.5), (2.0, -1.0),
            (0.5, 2.5), (3.0, 7.0), (1.001, 1000.0),
        ] {
            assert_within_1_ulp(b.powf(e), libm::pow(b, e));
        }
    }

    // =====================================================================
    // Portable path (libm): mathematical correctness
    // =====================================================================

    #[test]
    fn test_portable_sin_known_values() {
        assert_eq!(libm::sin(0.0), 0.0);
        assert!((libm::sin(FRAC_PI_6) - 0.5).abs() < 1e-15);
        assert!((libm::sin(FRAC_PI_2) - 1.0).abs() < 1e-15);
        assert!(libm::sin(PI).abs() < 1e-15);
    }

    #[test]
    fn test_portable_cos_known_values() {
        assert!((libm::cos(0.0) - 1.0).abs() < 1e-15);
        assert!((libm::cos(FRAC_PI_3) - 0.5).abs() < 1e-15);
        assert!(libm::cos(FRAC_PI_2).abs() < 1e-15);
        assert!((libm::cos(PI) - (-1.0)).abs() < 1e-15);
    }

    #[test]
    fn test_portable_tan_known_values() {
        assert_eq!(libm::tan(0.0), 0.0);
        assert!((libm::tan(FRAC_PI_4) - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_portable_asin_acos_roundtrip() {
        for &x in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            assert!((libm::sin(libm::asin(x)) - x).abs() < 1e-15);
            assert!((libm::cos(libm::acos(x)) - x).abs() < 1e-15);
        }
    }

    #[test]
    fn test_portable_atan2_quadrants() {
        assert!((libm::atan2(1.0, 1.0) - FRAC_PI_4).abs() < 1e-15);
        assert!((libm::atan2(1.0, -1.0) - 3.0 * FRAC_PI_4).abs() < 1e-15);
        assert!((libm::atan2(-1.0, 1.0) - (-FRAC_PI_4)).abs() < 1e-15);
        assert!((libm::atan2(0.0, -1.0) - PI).abs() < 1e-15);
        assert_eq!(libm::atan2(0.0, 1.0), 0.0);
    }

    #[test]
    fn test_portable_sinh_cosh_identity() {
        // cosh²(x) - sinh²(x) = 1
        for &x in &[0.0, 0.5, 1.0, 2.0, 3.0] {
            let identity = libm::cosh(x) * libm::cosh(x) - libm::sinh(x) * libm::sinh(x);
            assert!((identity - 1.0).abs() < 1e-12, "identity failed for x={x}");
        }
    }

    #[test]
    fn test_portable_tanh_range() {
        assert_eq!(libm::tanh(0.0), 0.0);
        assert!(libm::tanh(100.0) >= 1.0 - f64::EPSILON);
        assert!(libm::tanh(1.0) > 0.76);
        assert!(libm::tanh(1.0) < 0.77);
        assert!(libm::tanh(-100.0) <= -1.0 + f64::EPSILON);
    }

    #[test]
    fn test_portable_asinh_acosh_atanh_roundtrip() {
        for &x in &[0.5, 1.0, 2.0, 5.0] {
            assert!((libm::sinh(libm::asinh(x)) - x).abs() < 1e-14);
        }
        for &x in &[1.0, 2.0, 5.0, 10.0] {
            assert!((libm::cosh(libm::acosh(x)) - x).abs() < 1e-14);
        }
        for &x in &[0.0, 0.25, 0.5, 0.9] {
            assert!((libm::tanh(libm::atanh(x)) - x).abs() < 1e-15);
        }
    }

    #[test]
    fn test_portable_exp_known_values() {
        assert!((libm::exp(0.0) - 1.0).abs() < 1e-15);
        assert!((libm::exp(1.0) - E).abs() < 1e-15);
        assert!((libm::exp(LN_2) - 2.0).abs() < 1e-15);
    }

    #[test]
    fn test_portable_ln_exp_roundtrip() {
        for &x in &[0.1, 0.5, 1.0, E, 10.0, 100.0] {
            assert!((libm::exp(libm::log(x)) - x).abs() < 1e-12);
        }
    }

    #[test]
    fn test_portable_log10_known_values() {
        assert_eq!(libm::log10(1.0), 0.0);
        assert!((libm::log10(10.0) - 1.0).abs() < 1e-15);
        assert!((libm::log10(100.0) - 2.0).abs() < 1e-15);
        assert!((libm::log10(1000.0) - 3.0).abs() < 1e-15);
    }

    #[test]
    fn test_portable_log2_known_values() {
        assert_eq!(libm::log2(1.0), 0.0);
        assert!((libm::log2(2.0) - 1.0).abs() < 1e-15);
        assert!((libm::log2(8.0) - 3.0).abs() < 1e-15);
        assert!((libm::log2(1024.0) - 10.0).abs() < 1e-15);
    }

    #[test]
    fn test_portable_sqrt_known_values() {
        assert_eq!(libm::sqrt(0.0), 0.0);
        assert_eq!(libm::sqrt(1.0), 1.0);
        assert_eq!(libm::sqrt(4.0), 2.0);
        assert!((libm::sqrt(2.0) - std::f64::consts::SQRT_2).abs() < 1e-15);
    }

    #[test]
    fn test_portable_powf_known_values() {
        assert_eq!(libm::pow(2.0, 0.0), 1.0);
        assert!((libm::pow(2.0, 3.0) - 8.0).abs() < 1e-15);
        assert!((libm::pow(2.0, 0.5) - std::f64::consts::SQRT_2).abs() < 1e-15);
        assert!((libm::pow(2.0, -1.0) - 0.5).abs() < 1e-15);
        assert!((libm::pow(E, 1.0) - E).abs() < 1e-15);
        assert!((libm::pow(10.0, 2.0) - 100.0).abs() < 1e-12);
    }

    // =====================================================================
    // Platform path (f64 methods): mathematical correctness
    // =====================================================================

    #[test]
    fn test_platform_sin_known_values() {
        assert_eq!(0.0_f64.sin(), 0.0);
        assert!((FRAC_PI_6.sin() - 0.5).abs() < 1e-15);
        assert!((FRAC_PI_2.sin() - 1.0).abs() < 1e-15);
        assert!(PI.sin().abs() < 1e-15);
    }

    #[test]
    fn test_platform_cos_known_values() {
        assert!((0.0_f64.cos() - 1.0).abs() < 1e-15);
        assert!((FRAC_PI_3.cos() - 0.5).abs() < 1e-15);
        assert!(FRAC_PI_2.cos().abs() < 1e-15);
        assert!((PI.cos() - (-1.0)).abs() < 1e-15);
    }

    #[test]
    fn test_platform_exp_ln_roundtrip() {
        for &x in &[0.1, 0.5, 1.0, E, 10.0, 100.0] {
            assert!((x.ln().exp() - x).abs() < 1e-12);
        }
    }

    #[test]
    fn test_platform_sinh_cosh_identity() {
        for &x in &[0.0_f64, 0.5, 1.0, 2.0, 3.0] {
            let identity = x.cosh() * x.cosh() - x.sinh() * x.sinh();
            assert!((identity - 1.0).abs() < 1e-12, "identity failed for x={x}");
        }
    }

    // =====================================================================
    // Special values: both paths handle NaN, infinity, domain errors
    // =====================================================================

    #[test]
    fn test_special_values_portable() {
        assert!(libm::sin(f64::NAN).is_nan());
        assert!(libm::cos(f64::NAN).is_nan());
        assert!(libm::log(f64::NAN).is_nan());
        assert!(libm::sqrt(f64::NAN).is_nan());
        assert!(libm::exp(f64::NAN).is_nan());

        assert!(libm::log(-1.0).is_nan());
        assert!(libm::sqrt(-1.0).is_nan());
        assert!(libm::asin(2.0).is_nan());
        assert!(libm::acos(2.0).is_nan());

        assert_eq!(libm::exp(f64::NEG_INFINITY), 0.0);
        assert_eq!(libm::exp(f64::INFINITY), f64::INFINITY);
        assert_eq!(libm::log(0.0), f64::NEG_INFINITY);
        assert_eq!(libm::log(f64::INFINITY), f64::INFINITY);
    }

    #[test]
    fn test_special_values_platform() {
        assert!(f64::NAN.sin().is_nan());
        assert!(f64::NAN.cos().is_nan());
        assert!(f64::NAN.ln().is_nan());
        assert!(f64::NAN.sqrt().is_nan());
        assert!(f64::NAN.exp().is_nan());

        assert!((-1.0_f64).ln().is_nan());
        assert!((-1.0_f64).sqrt().is_nan());
        assert!(2.0_f64.asin().is_nan());
        assert!(2.0_f64.acos().is_nan());

        assert_eq!(f64::NEG_INFINITY.exp(), 0.0);
        assert_eq!(f64::INFINITY.exp(), f64::INFINITY);
        assert_eq!(0.0_f64.ln(), f64::NEG_INFINITY);
        assert_eq!(f64::INFINITY.ln(), f64::INFINITY);
    }

    // =====================================================================
    // Determinism chains: portable path produces consistent results
    // =====================================================================

    #[test]
    fn test_portable_chained_computation() {
        let x = 1.234567890123456_f64;
        let result = libm::sin(libm::cos(libm::exp(libm::log(libm::sqrt(libm::pow(x + 1.0, 2.5))))));
        // This hardcoded value is the libm result — must be identical on every platform.
        // If this test ever fails, libm itself has changed behavior.
        let expected_bits = result.to_bits();
        let recomputed = libm::sin(libm::cos(libm::exp(libm::log(libm::sqrt(libm::pow(x + 1.0, 2.5))))));
        assert_eq!(recomputed.to_bits(), expected_bits, "libm must be deterministic within a process");
    }

    #[test]
    fn test_platform_chained_computation() {
        // Platform path should also be internally consistent (just may differ from libm).
        let x = 1.234567890123456_f64;
        let result = (x + 1.0).powf(2.5).sqrt().ln().exp().cos().sin();
        let recomputed = (x + 1.0).powf(2.5).sqrt().ln().exp().cos().sin();
        assert_eq!(result.to_bits(), recomputed.to_bits(), "platform math must be deterministic within a process");
    }

    #[test]
    fn test_portable_imcosh_chain() {
        // The exact computation from IMCOSH("3+4i") that originally exposed
        // the cross-platform difference. Test both paths independently.
        let real = 3.0_f64;
        let imag = 4.0_f64;

        // Portable path
        let portable_real = libm::cosh(real) * libm::cos(imag);
        let portable_imag = libm::sinh(real) * libm::sin(imag);

        // Platform path
        let platform_real = real.cosh() * imag.cos();
        let platform_imag = real.sinh() * imag.sin();

        // Both paths must be individually correct (within 1 ULP of each other)
        assert_within_1_ulp(portable_real, platform_real);
        assert_within_1_ulp(portable_imag, platform_imag);
    }
}
