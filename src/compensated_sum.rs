// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Compensated summation for accurate floating-point accumulation.
//!
//! Adding `f64` values left to right loses precision: every addition rounds to
//! the nearest representable double, and those rounding errors accumulate. Over
//! a long range, or over values of widely differing magnitude, the result can
//! drift far from the true sum. The classic demonstration is `1e16 + 1 - 1e16`,
//! which a naive fold evaluates to `0.0` rather than `1.0`.
//!
//! This module implements Neumaier's variant of Kahan summation, which tracks
//! the rounding error in a separate compensation term and folds it back in at
//! the end. Neumaier is used in preference to plain Kahan because it also
//! handles the case where the incoming value is larger in magnitude than the
//! running total — a case plain Kahan silently gets wrong.
//!
//! # Non-finite values
//!
//! Infinities and NaN are passed through exactly as a naive fold would produce
//! them. Once the running total stops being finite the compensation term is
//! meaningless (it becomes NaN), so [`CompensatedSum::total`] discards it and
//! returns the running total alone. This keeps `±inf` from silently degrading
//! into `NaN`, which would change the Excel error a caller reports.

/// Accumulator for compensated (Neumaier) summation.
///
/// Use this for `+=` loops, fallible folds, and nested accumulator pairs. For a
/// plain iterator, prefer [`compensated_sum`].
///
/// ```
/// use codcel_calculation_engine::compensated_sum::CompensatedSum;
///
/// let mut total = CompensatedSum::new();
/// for value in [1e16, 1.0, -1e16] {
///     total.add(value);
/// }
/// assert_eq!(total.total(), 1.0); // a naive fold yields 0.0
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CompensatedSum {
    /// The running total, rounded at every step exactly as a naive fold would be.
    sum: f64,
    /// The accumulated rounding error discarded by those steps.
    compensation: f64,
}

impl CompensatedSum {
    /// Creates an empty accumulator whose total is `0.0`.
    #[inline]
    pub fn new() -> Self {
        Self {
            sum: 0.0,
            compensation: 0.0,
        }
    }

    /// Adds a value, recording the rounding error the addition discarded.
    #[inline]
    pub fn add(&mut self, value: f64) {
        let t = self.sum + value;

        // Recover the low-order bits lost by the addition above. Which operand
        // the error is measured against depends on which is larger; this test
        // is what separates Neumaier's variant from plain Kahan.
        if self.sum.abs() >= value.abs() {
            self.compensation += (self.sum - t) + value;
        } else {
            self.compensation += (value - t) + self.sum;
        }

        self.sum = t;
    }

    /// Returns the compensated total.
    ///
    /// When the running total is not finite the compensation term is discarded,
    /// so infinities and NaN propagate exactly as they would from a naive fold.
    #[inline]
    pub fn total(&self) -> f64 {
        if self.sum.is_finite() {
            self.sum + self.compensation
        } else {
            self.sum
        }
    }
}

impl std::iter::Sum<f64> for CompensatedSum {
    #[inline]
    fn sum<I: Iterator<Item = f64>>(iter: I) -> Self {
        let mut accumulator = Self::new();
        for value in iter {
            accumulator.add(value);
        }
        accumulator
    }
}

/// Sums an iterator of `f64` using compensated (Neumaier) summation.
///
/// A drop-in replacement for `iter.sum::<f64>()`. An empty iterator sums to
/// `0.0`, matching the standard library.
///
/// ```
/// use codcel_calculation_engine::compensated_sum::compensated_sum;
///
/// // Ten thousand tenths: the naive fold drifts, this does not.
/// let values = vec![0.1; 10_000];
/// assert_eq!(compensated_sum(values.iter().copied()), 1000.0);
/// ```
#[inline]
pub fn compensated_sum<I: IntoIterator<Item = f64>>(values: I) -> f64 {
    let mut accumulator = CompensatedSum::new();
    for value in values {
        accumulator.add(value);
    }
    accumulator.total()
}

/// Extension trait adding [`compensated_sum`] as an iterator method.
///
/// This is the drop-in replacement for `.sum::<f64>()` on an existing iterator
/// chain. It accepts iterators of `f64` and of `&f64`, so `values.iter()` works
/// without an intervening `.copied()`.
///
/// ```
/// use codcel_calculation_engine::compensated_sum::CompensatedSumExt;
///
/// let values = vec![1e16, 1.0, -1e16];
/// assert_eq!(values.iter().sum::<f64>(), 0.0);
/// assert_eq!(values.iter().compensated_sum(), 1.0);
/// ```
pub trait CompensatedSumExt<T> {
    /// Sums the iterator using compensated (Neumaier) summation.
    fn compensated_sum(self) -> f64;
}

impl<T: std::borrow::Borrow<f64>, I: Iterator<Item = T>> CompensatedSumExt<T> for I {
    #[inline]
    fn compensated_sum(self) -> f64 {
        let mut accumulator = CompensatedSum::new();
        for value in self {
            accumulator.add(*value.borrow());
        }
        accumulator.total()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compensated_sum_empty() {
        assert_eq!(compensated_sum(std::iter::empty()), 0.0);
    }

    #[test]
    fn test_compensated_sum_single_value() {
        assert_eq!(compensated_sum(vec![42.0]), 42.0);
    }

    #[test]
    fn test_compensated_sum_all_zeros() {
        assert_eq!(compensated_sum(vec![0.0; 100]), 0.0);
    }

    #[test]
    fn test_compensated_sum_matches_naive_on_simple_input() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(compensated_sum(values.iter().copied()), 15.0);
    }

    #[test]
    fn test_compensated_sum_recovers_cancelled_magnitude() {
        // The classic case: the 1.0 is lost entirely by a left-to-right fold.
        let values = [1e16, 1.0, -1e16];
        assert_eq!(values.iter().sum::<f64>(), 0.0);
        assert_eq!(compensated_sum(values.iter().copied()), 1.0);
    }

    #[test]
    fn test_compensated_sum_handles_growing_running_total() {
        // Plain Kahan gets this wrong because the incoming value is larger in
        // magnitude than the running total. Neumaier's swap handles it.
        let values = [1.0, 1e100, 1.0, -1e100];
        assert_eq!(values.iter().sum::<f64>(), 0.0);
        assert_eq!(compensated_sum(values.iter().copied()), 2.0);
    }

    #[test]
    fn test_compensated_sum_no_drift_over_long_range() {
        // 0.1 is not representable, so a naive fold accumulates visible error.
        let values = vec![0.1; 10_000];
        assert!((values.iter().sum::<f64>() - 1000.0).abs() > 0.0);
        assert_eq!(compensated_sum(values.iter().copied()), 1000.0);
    }

    #[test]
    fn test_compensated_sum_negative_and_mixed() {
        assert_eq!(compensated_sum(vec![-1.0, -2.0, -3.0, -4.0, -5.0]), -15.0);
        assert_eq!(compensated_sum(vec![1.0, -2.0, 3.0, -4.0, 5.0]), 3.0);
    }

    #[test]
    fn test_compensated_sum_preserves_positive_infinity() {
        let result = compensated_sum(vec![1.0, f64::INFINITY, 2.0]);
        assert!(result.is_infinite() && result.is_sign_positive());
    }

    #[test]
    fn test_compensated_sum_preserves_negative_infinity() {
        let result = compensated_sum(vec![1.0, f64::NEG_INFINITY, 2.0]);
        assert!(result.is_infinite() && result.is_sign_negative());
    }

    #[test]
    fn test_compensated_sum_opposing_infinities_are_nan() {
        // Matches the naive fold: inf + -inf is NaN.
        let values = [f64::INFINITY, f64::NEG_INFINITY];
        assert!(values.iter().sum::<f64>().is_nan());
        assert!(compensated_sum(values.iter().copied()).is_nan());
    }

    #[test]
    fn test_compensated_sum_preserves_nan() {
        assert!(compensated_sum(vec![1.0, f64::NAN, 2.0]).is_nan());
    }

    #[test]
    fn test_compensated_sum_accumulator_add_and_total() {
        let mut accumulator = CompensatedSum::new();
        assert_eq!(accumulator.total(), 0.0);

        accumulator.add(1e16);
        accumulator.add(1.0);
        accumulator.add(-1e16);
        assert_eq!(accumulator.total(), 1.0);
    }

    #[test]
    fn test_compensated_sum_accumulator_default_is_empty() {
        assert_eq!(CompensatedSum::default().total(), 0.0);
    }

    #[test]
    fn test_compensated_sum_ext_on_borrowed_items() {
        let values = [1e16, 1.0, -1e16];
        assert_eq!(values.iter().sum::<f64>(), 0.0);
        assert_eq!(values.iter().compensated_sum(), 1.0);
    }

    #[test]
    fn test_compensated_sum_ext_on_owned_items() {
        assert_eq!(
            vec![1.0, 1e100, 1.0, -1e100].into_iter().compensated_sum(),
            2.0
        );
    }

    #[test]
    fn test_compensated_sum_ext_matches_free_function() {
        let values = vec![0.1; 1_000];
        assert_eq!(
            values.iter().compensated_sum(),
            compensated_sum(values.iter().copied())
        );
    }

    #[test]
    fn test_compensated_sum_accumulator_collects_from_iterator() {
        let accumulator: CompensatedSum = vec![1e16, 1.0, -1e16].into_iter().sum();
        assert_eq!(accumulator.total(), 1.0);
    }
}
