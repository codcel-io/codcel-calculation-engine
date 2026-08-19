// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! The one standard normal distribution used by every normal-family Excel function.
//!
//! `NORM.S.DIST`, `NORMSDIST`, `NORM.DIST`, `NORMDIST`, `NORM.S.INV`, `NORMSINV`, `NORM.INV`,
//! `NORMINV`, `PHI`, `GAUSS`, `LOGNORM.DIST`, `LOGNORMDIST`, `LOGNORM.INV`, `LOGINV`,
//! `CONFIDENCE`, `CONFIDENCE.NORM`, `Z.TEST` and `ZTEST` all route through here, so they agree
//! with each other to the last bit. Excel treats them as one distribution and so must we.
//!
//! Three deliberate choices, each measured against 60-digit references rather than assumed:
//!
//! - **`libm`, not `statrs`, for the error function.** `statrs::function::erf` carries a relative
//!   error around `1e-11` — it gets `Φ(1)` as `0.8413447460549428` where the true value is
//!   `0.8413447460685429`. `libm`'s FreeBSD msun port lands within a couple of ULP. `libm` is
//!   also pure Rust and calls only its own `exp`, so the whole cumulative branch is bit-identical
//!   across platforms regardless of `CODCEL_USE_PORTABLE_MATH`.
//! - **`erfc`, not `1 + erf`.** `erf` saturates at `-1.0` long before the CDF underflows, so
//!   `0.5 * (1 + erf(z / sqrt(2)))` cancels the entire result away in the left tail: it returns
//!   exactly `0.0` at `z = -10`, where the true value is `7.6e-24`. `0.5 * erfc(-z / sqrt(2))`
//!   holds full relative accuracy out to the edge of `f64`.
//! - **A Newton step on the inverse.** `statrs`'s `erfc_inv` is good to a few ULP, but refining it
//!   once against the CDF above makes [`std_normal_inv`] the true inverse of [`std_normal_cdf`],
//!   which is the coherence Excel users actually observe.
//!
//! The density stays on [`crate::portable_math`]: it is a closed form with nothing to gain from a
//! special function, so every caller's PDF branch keeps the existing determinism guarantee.

use std::f64::consts::SQRT_2;

/// The square root of 2π, to the last representable bit of an `f64`.
pub(crate) const SQRT_2PI: f64 = 2.5066282746310002;

/// The standard normal probability density φ(z).
pub(crate) fn std_normal_pdf(z: f64) -> f64 {
    crate::portable_math::exp(-z * z / 2.0) / SQRT_2PI
}

/// The standard normal cumulative distribution Φ(z) = ½·erfc(−z/√2).
pub(crate) fn std_normal_cdf(z: f64) -> f64 {
    0.5 * libm::erfc(-z / SQRT_2)
}

/// Φ(z) − ½ = ½·erf(z/√2), the quantity `GAUSS` returns.
///
/// Computed directly rather than by subtracting ½ from [`std_normal_cdf`], which would cancel
/// away the leading digits for small `z`.
pub(crate) fn std_normal_cdf_minus_half(z: f64) -> f64 {
    0.5 * libm::erf(z / SQRT_2)
}

/// The inverse standard normal cumulative distribution Φ⁻¹(p).
///
/// Callers must validate `0.0 < p < 1.0` first; `p` outside that range yields ±∞ rather than an
/// error, because Excel's `#NUM!` message differs per function and belongs at the call site.
pub(crate) fn std_normal_inv(p: f64) -> f64 {
    // Always solve on the lower tail, where `Φ(x0) - q` is a difference of like-sized quantities
    // rather than a catastrophic cancellation between two numbers near 1. For `p` in `[0.5, 1]`
    // Sterbenz's lemma makes `1.0 - p` exact, so the reflection costs nothing.
    if p > 0.5 {
        -std_normal_inv_lower_tail(1.0 - p)
    } else {
        std_normal_inv_lower_tail(p)
    }
}

/// Φ⁻¹(q) for `0 < q <= 0.5`, refined to be the exact inverse of [`std_normal_cdf`].
fn std_normal_inv_lower_tail(q: f64) -> f64 {
    let estimate = -SQRT_2 * statrs::function::erf::erfc_inv(2.0 * q);

    // One Newton step: x - (Φ(x) - q) / φ(x). Quadratic convergence takes the few-ULP seed to
    // full precision. The density underflows for subnormal `q`, where the seed is already all
    // the accuracy available, so fall back to it rather than dividing by zero.
    let correction = (std_normal_cdf(estimate) - q) / std_normal_pdf(estimate);
    if correction.is_finite() {
        estimate - correction
    } else {
        estimate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every expected value below was computed with mpmath at 60 decimal digits and rounded to
    // the nearest f64.

    #[test]
    fn test_sqrt_2pi_is_the_nearest_f64() {
        assert_eq!(SQRT_2PI, (2.0 * std::f64::consts::PI).sqrt());
    }

    #[test]
    fn test_std_normal_cdf_at_zero() {
        assert_eq!(std_normal_cdf(0.0), 0.5);
    }

    #[test]
    fn test_std_normal_cdf_matches_reference() {
        for (z, expected) in [
            (-3.0, 0.0013498980316300946),
            (-1.0, 0.15865525393145705),
            (-0.5, 0.3085375387259869),
            (0.5, 0.6914624612740131),
            (1.0, 0.8413447460685429),
            (1.5, 0.9331927987311419),
            (2.0, 0.9772498680518208),
            (3.0, 0.9986501019683699),
        ] {
            let result = std_normal_cdf(z);
            assert!(
                ((result - expected) / expected).abs() < 1e-15,
                "Phi({z}) = {result}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_std_normal_cdf_keeps_the_far_left_tail() {
        // The `0.5 * (1 + erf(z / sqrt(2)))` form returns exactly 0.0 for the last three of these.
        for (z, expected) in [
            (-6.0, 9.86587645037698e-10),
            (-8.0, 6.220960574271784e-16),
            (-10.0, 7.619853024160525e-24),
            (-15.0, 3.670966199312751e-51),
        ] {
            let result = std_normal_cdf(z);
            assert!(
                ((result - expected) / expected).abs() < 1e-13,
                "Phi({z}) = {result}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_std_normal_cdf_is_symmetric() {
        for z in [0.25, 0.5, 1.0, 2.0, 4.0] {
            assert!((std_normal_cdf(z) + std_normal_cdf(-z) - 1.0).abs() < 1e-15);
        }
    }

    #[test]
    fn test_std_normal_pdf_matches_reference() {
        for (z, expected) in [
            (0.0, 0.3989422804014327),
            (0.5, 0.35206532676429947),
            (1.0, 0.24197072451914334),
            (1.5, 0.12951759566589172),
            (3.0, 0.0044318484119380075),
        ] {
            let result = std_normal_pdf(z);
            assert!(
                ((result - expected) / expected).abs() < 1e-15,
                "phi({z}) = {result}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_std_normal_inv_matches_reference() {
        for (p, expected) in [
            (1e-8, -5.612001244174789),
            (0.001, -3.0902323061678136),
            (0.01, -2.326347874040841),
            (0.025, -1.9599639845400543),
            (0.1, -1.2815515655446004),
            (0.3, -0.5244005127080408),
            (0.9, 1.2815515655446004),
            (0.975, 1.9599639845400543),
            (0.99, 2.326347874040841),
            (0.999, 3.0902323061678136),
        ] {
            let result = std_normal_inv(p);
            assert!(
                ((result - expected) / expected).abs() < 1e-15,
                "Phi^-1({p}) = {result}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_std_normal_inv_at_half() {
        assert_eq!(std_normal_inv(0.5), 0.0);
    }

    #[test]
    fn test_std_normal_inv_inverts_the_cdf() {
        // Round-tripping through the lower tail is lossless: Phi(z) is a small number carrying
        // full relative precision, so the inverse recovers z to within a couple of ULP.
        for z in [-5.0, -3.0, -2.0, -1.0, -0.5, 0.0] {
            let round_tripped = std_normal_inv(std_normal_cdf(z));
            assert!(
                (round_tripped - z).abs() < 1e-14 * z.abs().max(1.0),
                "round trip of {z} gave {round_tripped}"
            );
        }
    }

    #[test]
    fn test_std_normal_inv_round_trip_degrades_in_the_upper_tail() {
        // Not an implementation limit: Phi(5) = 0.9999997133484281 sits where consecutive f64
        // values are 1.1e-16 apart, so the probability itself only pins z to about 1e-11. Excel
        // is bounded by exactly the same representation. Documented here so nobody "fixes" it.
        for z in [0.5, 1.0, 2.0, 5.0] {
            let round_tripped = std_normal_inv(std_normal_cdf(z));
            assert!(
                (round_tripped - z).abs() < 1e-10 * z.abs().max(1.0),
                "round trip of {z} gave {round_tripped}"
            );
        }
    }

    #[test]
    fn test_std_normal_inv_survives_the_subnormal_tail() {
        // The Newton step's density underflows here; the seed must be returned rather than NaN.
        let result = std_normal_inv(1e-320);
        assert!(result.is_finite() && result < -38.0, "got {result}");
    }

    #[test]
    fn test_std_normal_cdf_minus_half_matches_reference() {
        for (z, expected) in [
            (1e-8, 3.989422804014327e-09),
            (0.5, 0.1914624612740131),
            (1.0, 0.3413447460685429),
            (2.0, 0.4772498680518208),
        ] {
            let result = std_normal_cdf_minus_half(z);
            assert!(
                ((result - expected) / expected).abs() < 1e-15,
                "Phi({z}) - 0.5 = {result}, expected {expected}"
            );
        }
    }

    #[test]
    fn test_std_normal_cdf_minus_half_agrees_with_the_cdf() {
        for z in [-2.0, -1.0, -0.5, 0.0, 0.5, 1.0, 2.0] {
            let difference = std_normal_cdf_minus_half(z) - (std_normal_cdf(z) - 0.5);
            assert!(
                difference.abs() < 1e-16,
                "disagreement at {z}: {difference}"
            );
        }
    }
}
