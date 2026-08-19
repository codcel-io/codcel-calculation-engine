// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Shared root finder for the iterative financial functions.
//!
//! Excel's rate-solving functions (`IRR`, `XIRR`, `RATE`, `YIELD`, ...) all reduce to the same
//! problem: find the rate `r > -1` at which some present-value function crosses zero. A bare
//! Newton-Raphson iteration solves the easy cases and fails on the rest, so this module layers
//! four strategies and gates all of them behind a residual check:
//!
//! 1. Newton-Raphson from the caller's guess, with the singularity and step-damping guards that
//!    keep it from walking through the pole at `r = -1`.
//! 2. If that does not produce a root, a scan for a sign change over a fixed ladder of candidate
//!    rates, preferring the bracket nearest the caller's guess.
//! 3. Bisection inside that bracket, polished by a short Newton run to recover full precision.
//! 4. A final Newton attempt from a far guess, for steep roots the ladder can step over.
//!
//! # Why the residual gate matters
//!
//! Newton is normally stopped when its *step* becomes small. That is unsound: with a small or
//! inaccurate derivative the step can be small a long way from any root, so the iteration
//! "converges" onto a non-root and returns it as an answer. Every strategy here therefore only
//! yields a rate whose residual `|f(rate)|` is actually near zero, measured relative to the
//! magnitude of the inputs. A wrong answer is worse than an error, so failing is the fallback.
//!
//! # Known limits
//!
//! * When the cash flows change sign more than once there can be several valid rates. This
//!   returns the one nearest the caller's guess, which is the same contract Excel offers; it is
//!   not "the" IRR, because no such thing exists for those inputs.
//! * Roots extremely close to `-1` may be missed. The candidate ladder starts at `-0.999999`
//!   and the pole is approached but never crossed.
//! * The ladder is finite, so a root outside `(-1, 1e6)` is only reachable by Newton.

use std::error::Error;

/// Maximum Newton-Raphson iterations per attempt.
const MAX_NEWTON_ITERATIONS: usize = 200;

/// Maximum bisection steps. A bracket is halved to machine precision well inside this.
const MAX_BISECTION_ITERATIONS: usize = 200;

/// Below this the derivative is treated as flat and Newton gives up rather than dividing.
const DERIVATIVE_FLOOR: f64 = 1e-14;

/// Relative step size for the numerical derivative used by [`solve_rate_numeric`].
const DERIVATIVE_STEP: f64 = 1e-7;

/// Relative step size below which a Newton iteration is considered settled.
const STEP_TOLERANCE: f64 = 1e-14;

/// Fallback starting point for the final Newton attempt
const FAR_GUESS: f64 = 200.0;

/// Rates are only meaningful above the pole at `-1`; this is the closest the search will go.
const MIN_RATE: f64 = -0.999_999;

/// Residual below which a rate counts as a root.
///
/// The residual is a present value, so it scales with the cash flows: a $1e-7 error is exact for
/// a $100 investment and meaningless for a $10bn one. `scale` is the sum of the absolute input
/// magnitudes, with an absolute floor for inputs near zero.
fn residual_tolerance(scale: f64) -> f64 {
    (1e-10 * scale.abs()).max(1e-7)
}

/// Candidate rates scanned for a sign change, ascending.
///
/// Dense across the range real rates occupy, then geometric out to `1e6`. Built by addition and
/// multiplication only — no transcendental calls — so the ladder is bit-identical on every
/// platform and the bracket chosen never depends on the host math library.
fn candidate_rates() -> Vec<f64> {
    let mut rates = Vec::with_capacity(320);

    // Approach the pole at -1 without touching it.
    rates.extend_from_slice(&[MIN_RATE, -0.999_99, -0.999_9, -0.999, -0.995]);

    // -0.99 ..= 0.99 in 1% steps: where essentially every real-world rate lives.
    for step in -99..=99 {
        rates.push(f64::from(step) / 100.0);
    }

    // 1.0 ..= 10.0 in half steps: 100%-1000% returns.
    for step in 2..=20 {
        rates.push(f64::from(step) / 2.0);
    }

    // 15 ..= 100 in steps of 5.
    for step in 3..=20 {
        rates.push(f64::from(step) * 5.0);
    }

    // Geometric out to 1e6 for pathological inputs.
    let mut rate = 150.0;
    while rate <= 1e6 {
        rates.push(rate);
        rate *= 1.5;
    }

    rates
}

/// Newton-Raphson from `start`, returning its final candidate.
///
/// The result is a *candidate*, not a verified root — the caller applies the residual gate.
/// Returns `None` when the iteration cannot proceed (flat derivative, non-finite value).
fn newton<F, D>(f: &F, df: &D, start: f64) -> Option<f64>
where
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    let mut rate = start;
    if !rate.is_finite() || rate <= -1.0 {
        return None;
    }

    for _ in 0..MAX_NEWTON_ITERATIONS {
        let y = f(rate);
        if !y.is_finite() {
            return None;
        }
        if y == 0.0 {
            return Some(rate);
        }

        let dy = df(rate);
        if !dy.is_finite() || dy.abs() < DERIVATIVE_FLOOR {
            return None;
        }

        let mut next = rate - y / dy;
        if !next.is_finite() {
            return None;
        }

        // Never step through the singularity at -1; halve the distance to it instead.
        if next <= -1.0 {
            next = (rate - 1.0) / 2.0;
        }

        // Damp wild steps so a bad derivative cannot fling the iterate out of range.
        let step = next - rate;
        if step.abs() > 1.0 {
            next = rate + step.signum();
        }

        if (next - rate).abs() <= STEP_TOLERANCE * (1.0 + rate.abs()) {
            return Some(next);
        }

        rate = next;
    }

    Some(rate)
}

/// Scans [`candidate_rates`] for sign changes and returns the bracket nearest `guess`.
fn bracket<F>(f: &F, guess: f64) -> Option<(f64, f64)>
where
    F: Fn(f64) -> f64,
{
    let rates = candidate_rates();
    let mut best: Option<(f64, f64)> = None;
    let mut best_distance = f64::INFINITY;

    let mut previous: Option<(f64, f64)> = None;
    for &rate in &rates {
        let value = f(rate);
        if !value.is_finite() {
            previous = None;
            continue;
        }

        if let Some((prev_rate, prev_value)) = previous {
            // A sign change (or an exact zero at either end) brackets a root.
            if prev_value == 0.0 {
                return Some((prev_rate, prev_rate));
            }
            if (prev_value < 0.0) != (value < 0.0) {
                let midpoint = 0.5 * (prev_rate + rate);
                let distance = (midpoint - guess).abs();
                if distance < best_distance {
                    best_distance = distance;
                    best = Some((prev_rate, rate));
                }
            }
        }

        if value == 0.0 {
            return Some((rate, rate));
        }
        previous = Some((rate, value));
    }

    best
}

/// Bisects `f` inside a bracket known to contain a sign change.
fn bisect<F>(f: &F, low: f64, high: f64) -> Option<f64>
where
    F: Fn(f64) -> f64,
{
    if low == high {
        return Some(low);
    }

    let mut low = low;
    let mut high = high;
    let mut low_value = f(low);
    let high_value = f(high);

    if !low_value.is_finite() || !high_value.is_finite() {
        return None;
    }
    if low_value == 0.0 {
        return Some(low);
    }
    if high_value == 0.0 {
        return Some(high);
    }
    if (low_value < 0.0) == (high_value < 0.0) {
        return None;
    }

    for _ in 0..MAX_BISECTION_ITERATIONS {
        let middle = 0.5 * (low + high);
        if middle <= low || middle >= high {
            break;
        }

        let middle_value = f(middle);
        if !middle_value.is_finite() {
            return None;
        }
        if middle_value == 0.0 {
            return Some(middle);
        }

        if (low_value < 0.0) == (middle_value < 0.0) {
            low = middle;
            low_value = middle_value;
        } else {
            high = middle;
        }
    }

    Some(0.5 * (low + high))
}

/// Finds the rate at which `f` is zero, given its analytic derivative `df`.
///
/// `guess` is the caller's starting estimate (Excel's optional `guess` argument), `scale` the sum
/// of the absolute input magnitudes used to size the residual tolerance, and `label` the Excel
/// function name used in the error message.
///
/// # Errors
/// Returns an error when no rate satisfying the residual gate can be found. Errors are plain
/// descriptive strings, consistent with the rest of the financial module.
pub(crate) fn solve_rate<F, D>(
    f: F,
    df: D,
    guess: f64,
    scale: f64,
    label: &str,
) -> Result<f64, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    let tolerance = residual_tolerance(scale);

    // A candidate is only an answer if its residual is actually near zero.
    let residual = |rate: f64| -> Option<f64> {
        if !rate.is_finite() || rate <= -1.0 {
            return None;
        }
        let value = f(rate);
        if value.is_finite() && value.abs() <= tolerance {
            Some(value.abs())
        } else {
            None
        }
    };

    let start = if guess.is_finite() && guess > -1.0 {
        guess
    } else {
        0.1
    };

    // 1. Newton from the caller's guess. Excel's contract is the root nearest the guess, so this
    //    runs first and wins when it succeeds.
    if let Some(rate) = newton(&f, &df, start) {
        if residual(rate).is_some() {
            return Ok(rate);
        }
    }

    // 2-3. Bracket, bisect, then polish. Keep whichever of the two has the smaller residual, so
    //      a polish that wanders cannot lose a good bisection result.
    if let Some((low, high)) = bracket(&f, start) {
        if let Some(bisected) = bisect(&f, low, high) {
            let mut best = residual(bisected).map(|r| (r, bisected));

            if let Some(polished) = newton(&f, &df, bisected) {
                if let Some(polished_residual) = residual(polished) {
                    if best.is_none_or(|(best_residual, _)| polished_residual < best_residual) {
                        best = Some((polished_residual, polished));
                    }
                }
            }

            if let Some((_, rate)) = best {
                return Ok(rate);
            }
        }
    }

    // 4. Last resort: a steep root far from the guess.
    if let Some(rate) = newton(&f, &df, FAR_GUESS) {
        if residual(rate).is_some() {
            return Ok(rate);
        }
    }

    Err(format!("{label}: Failed to converge to a solution.").into())
}

/// Like [`solve_rate`], but derives the slope numerically by central difference.
///
/// For callers whose objective has no convenient closed-form derivative — the bond yield family,
/// which inverts `PRICE`. The difference falls back to a one-sided form near the pole at `-1`.
///
/// # Errors
/// As [`solve_rate`].
#[allow(dead_code)]
pub(crate) fn solve_rate_numeric<F>(
    f: F,
    guess: f64,
    scale: f64,
    label: &str,
) -> Result<f64, Box<dyn Error + Send + Sync>>
where
    F: Fn(f64) -> f64,
{
    let derivative = |rate: f64| -> f64 {
        let step = DERIVATIVE_STEP * (1.0 + rate.abs());
        let forward = f(rate + step);

        // Only reach backwards when doing so stays above the pole.
        if rate - step > -1.0 {
            let backward = f(rate - step);
            if forward.is_finite() && backward.is_finite() {
                return (forward - backward) / (2.0 * step);
            }
        }

        let here = f(rate);
        if forward.is_finite() && here.is_finite() {
            (forward - here) / step
        } else {
            f64::NAN
        }
    };

    solve_rate(&f, derivative, guess, scale, label)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// -1000 now against 1200 a period later: a single root at exactly 20%.
    fn simple_npv(rate: f64) -> f64 {
        -1000.0 + 1200.0 / (1.0 + rate)
    }

    fn simple_derivative(rate: f64) -> f64 {
        -1200.0 / ((1.0 + rate) * (1.0 + rate))
    }

    #[test]
    fn test_solve_rate_newton_path() {
        let result = solve_rate(simple_npv, simple_derivative, 0.1, 2200.0, "TEST").unwrap();
        assert!((result - 0.2).abs() < 1e-9, "expected 0.2, got {result}");
    }

    #[test]
    fn test_solve_rate_falls_back_to_bisection() {
        // A derivative that is always flat makes every Newton attempt bail out, so the answer can
        // only come from the bracket-and-bisect path.
        let result = solve_rate(simple_npv, |_| 0.0, 0.1, 2200.0, "TEST").unwrap();
        assert!((result - 0.2).abs() < 1e-9, "expected 0.2, got {result}");
    }

    #[test]
    fn test_solve_rate_recovers_from_useless_guess() {
        // A guess below the pole is discarded rather than propagated as NaN.
        let result = solve_rate(simple_npv, simple_derivative, -50.0, 2200.0, "TEST").unwrap();
        assert!((result - 0.2).abs() < 1e-9, "expected 0.2, got {result}");
    }

    #[test]
    fn test_solve_rate_errors_when_no_root_exists() {
        let result = solve_rate(|_| 1.0, |_| 0.0, 0.1, 1.0, "TEST");
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_rate_rejects_non_root_convergence() {
        // f is never within tolerance of zero, but its derivative is large enough that Newton
        // takes vanishing steps. Step-size convergence would return a non-root here; the
        // residual gate must reject it.
        let result = solve_rate(|_| 5.0, |r| 1e12 * (1.0 + r), 0.1, 1.0, "TEST");
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_rate_numeric_matches_analytic() {
        let result = solve_rate_numeric(simple_npv, 0.1, 2200.0, "TEST").unwrap();
        assert!((result - 0.2).abs() < 1e-9, "expected 0.2, got {result}");
    }

    #[test]
    fn test_candidate_rates_are_ascending_and_above_the_pole() {
        let rates = candidate_rates();
        assert!(rates.len() > 200);
        assert!(rates[0] > -1.0);
        for pair in rates.windows(2) {
            assert!(pair[0] < pair[1], "{} !< {}", pair[0], pair[1]);
        }
    }

    #[test]
    fn test_residual_tolerance_scales_with_inputs() {
        assert!((residual_tolerance(0.0) - 1e-7).abs() < f64::EPSILON);
        assert!(residual_tolerance(1e9) > residual_tolerance(1e3));
    }
}
