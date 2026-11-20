// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! The value at one position of a list's sorted order, found without sorting the whole list.

use std::cmp::Ordering;

/// The value at `index` (0-based) of `values` sorted ascending, found by selection rather than
/// a full sort; `values` is reordered.
///
/// `None` when the list holds a NaN or the value found is a zero. Those are the cases where a
/// stable sort comparing with `partial_cmp` could put a different value at `index` (NaN is
/// unordered, and `-0.0` and `0.0` compare equal but print differently), so the caller sorts
/// instead.
pub(crate) fn select_ascending(values: &mut [f64], index: usize) -> Option<f64> {
    if index >= values.len() || values.iter().any(|v| v.is_nan()) {
        return None;
    }
    let (_, &mut value, _) =
        values.select_nth_unstable_by(index, |a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    (value != 0.0).then_some(value)
}

/// The two middle values of `values` sorted ascending, for an even number of values; as
/// [`select_ascending`], `None` when the list holds a NaN or either value is a zero.
pub(crate) fn select_middle_pair(values: &mut [f64]) -> Option<(f64, f64)> {
    let upper_index = values.len() / 2;
    let upper = select_ascending(values, upper_index)?;
    // After selection everything before the upper middle value is no greater than it, so the
    // lower middle value is the largest of those.
    let lower = values[..upper_index].iter().copied().reduce(f64::max)?;
    (lower != 0.0).then_some((lower, upper))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selection_matches_sorting() {
        let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
        let mut next = || {
            seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            seed >> 33
        };
        for _ in 0..3_000 {
            let len = 1 + (next() % 30) as usize;
            let values: Vec<f64> = (0..len).map(|_| (next() % 11) as f64 - 5.0).collect();
            let mut sorted = values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            let index = (next() as usize) % len;
            if let Some(value) = select_ascending(&mut values.clone(), index) {
                assert_eq!(value.to_bits(), sorted[index].to_bits());
            }
            if len.is_multiple_of(2) {
                if let Some((lower, upper)) = select_middle_pair(&mut values.clone()) {
                    assert_eq!(lower.to_bits(), sorted[len / 2 - 1].to_bits());
                    assert_eq!(upper.to_bits(), sorted[len / 2].to_bits());
                }
            }
        }
        assert_eq!(select_ascending(&mut [1.0, f64::NAN], 0), None);
        assert_eq!(select_ascending(&mut [0.0, -0.0, 1.0], 0), None);
    }
}
