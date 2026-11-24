// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Opt-in performance counters for finding where a calculation spends its time.
//!
//! - `CODCEL_PROFILE` enables the counters below; [`take`] reads and resets them.
//! - `CODCEL_PROFILE_KEYS` also counts per-request cache lookups and misses by key; [`take_top_keys`]
//!   reads and resets them.
//!
//! When the variables are unset, each count costs one load of a cached flag.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

/// Per-request cache lookups.
pub static FN_CACHE_GETS: AtomicU64 = AtomicU64::new(0);
/// Per-request cache lookups that found a value.
pub static FN_CACHE_HITS: AtomicU64 = AtomicU64::new(0);
/// Values not cached because they were too large.
pub static FN_CACHE_SKIPPED: AtomicU64 = AtomicU64::new(0);
/// Per-request cache lookups answered with an error kept earlier.
pub static FN_ERROR_HITS: AtomicU64 = AtomicU64::new(0);
/// Per-request range cache lookups.
pub static RANGE_CACHE_GETS: AtomicU64 = AtomicU64::new(0);
/// Per-request range cache lookups that found a value.
pub static RANGE_CACHE_HITS: AtomicU64 = AtomicU64::new(0);
/// Fill-down ranges answered entirely from cells already read.
pub static FILL_DOWN_HITS: AtomicU64 = AtomicU64::new(0);
/// Cells of fill-down ranges computed (each once per request).
pub static FILL_DOWN_COMPUTED: AtomicU64 = AtomicU64::new(0);
/// Fill-down ranges read cell by cell because their column has too many first rows.
pub static FILL_DOWN_UNCACHED: AtomicU64 = AtomicU64::new(0);

fn enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("CODCEL_PROFILE").is_some())
}

pub(crate) fn keys_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("CODCEL_PROFILE_KEYS").is_some())
}

fn keys() -> &'static Mutex<HashMap<String, u64>> {
    static KEYS: OnceLock<Mutex<HashMap<String, u64>>> = OnceLock::new();
    KEYS.get_or_init(Default::default)
}

/// Adds one to `counter`, when `CODCEL_PROFILE` is set.
#[inline]
pub fn count(counter: &AtomicU64) {
    if enabled() {
        counter.fetch_add(1, Ordering::Relaxed);
    }
}

/// With `CODCEL_PROFILE_KEYS` set, counts a per-request cache lookup under its key with any
/// trailing row number removed, so the lookups of a fill-down column add up under one name.
#[inline]
pub fn count_key(key: &str) {
    if keys_enabled() {
        let name = key.trim_end_matches(|c: char| c.is_ascii_digit());
        add_key(name);
    }
}

/// With `CODCEL_PROFILE_KEYS` set, counts a per-request cache miss under its full key, row number
/// included, so a cell computed more than once stands out.
#[inline]
pub fn count_miss(key: &str) {
    if keys_enabled() {
        add_key(&format!("MISS {key}"));
    }
}

fn add_key(name: &str) {
    let mut keys = keys()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    *keys.entry(name.to_string()).or_insert(0) += 1;
}

/// The counters as (name, value) pairs, which are then reset.
pub fn take() -> Vec<(&'static str, u64)> {
    let take = |counter: &AtomicU64| counter.swap(0, Ordering::Relaxed);
    vec![
        ("fn_cache_gets", take(&FN_CACHE_GETS)),
        ("fn_cache_hits", take(&FN_CACHE_HITS)),
        ("fn_cache_skipped_large", take(&FN_CACHE_SKIPPED)),
        ("fn_error_hits", take(&FN_ERROR_HITS)),
        ("range_cache_gets", take(&RANGE_CACHE_GETS)),
        ("range_cache_hits", take(&RANGE_CACHE_HITS)),
        ("fill_down_hits", take(&FILL_DOWN_HITS)),
        ("fill_down_computed", take(&FILL_DOWN_COMPUTED)),
        ("fill_down_uncached", take(&FILL_DOWN_UNCACHED)),
    ]
}

/// The `n` keys counted most since the last call, most first; the counts are then reset.
pub fn take_top_keys(n: usize) -> Vec<(String, u64)> {
    let mut keys: Vec<(String, u64)> = keys()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .drain()
        .collect();
    keys.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
    keys.truncate(n);
    keys
}
