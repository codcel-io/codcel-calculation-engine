// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Per-request cache for computation results.
//!
//! Used via `task_local!` in generated code to avoid redundant lookups
//! when parameterized table functions call the same lookup repeatedly.

use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;

/// Per-request cache for computation results.
/// Uses `RefCell` since it's only accessed within a single task via `task_local!`.
pub struct CodcelCache {
    cache: RefCell<HashMap<String, Value>>,
}

impl CodcelCache {
    pub fn new() -> Self {
        Self {
            cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, key: &str) -> Option<Value> {
        self.cache.borrow().get(key).cloned()
    }

    pub fn insert(&self, key: String, value: Value) {
        if value.is_cacheable() {
            self.cache.borrow_mut().insert(key, value);
        }
    }

    /// Invalidate all cached entries (called after mutations).
    pub fn clear(&self) {
        self.cache.borrow_mut().clear();
    }
}

impl Default for CodcelCache {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! codcel_cache {
    ($input:expr, $key:expr, $body:expr) => {{
        if let Some(cached) = $input.fn_cache_get($key) {
            return Ok(cached);
        }
        // Errors are kept as their message, which is all callers read of them (IFERROR,
        // ISERROR and cell values classify an error by its message).
        if let Some(message) = $input.fn_error_get($key) {
            return Err(::std::boxed::Box::<dyn ::std::error::Error + Send + Sync>::from(message));
        }
        let __result: ::std::result::Result<
            _,
            ::std::boxed::Box<dyn ::std::error::Error + Send + Sync>,
        > = $body;
        match __result {
            Ok(ref val) => {
                if val.is_cacheable() {
                    $input.fn_cache_insert($key.to_string(), val.clone());
                } else {
                    $crate::profile::count(&$crate::profile::FN_CACHE_SKIPPED);
                }
            }
            Err(ref error) => $input.fn_error_insert($key.to_string(), error.to_string()),
        }
        __result
    }};
}

/// Evaluates `$body`, a range value that depends only on `$input`, once per request.
///
/// Unlike [`codcel_cache!`], this is an expression, so it can wrap a range argument inside a
/// formula, and it keeps values of any size. `$key` must be the same for the same expression and
/// differ between expressions; the generator uses a hash of the expression's code. An error
/// raised with `?` inside `$body` leaves the enclosing function as before and is not cached.
#[macro_export]
macro_rules! codcel_cached {
    ($input:expr, $key:expr, $body:expr) => {{
        match $input.range_cache_get($key) {
            Some(cached) => cached,
            None => {
                let __value = $body;
                $input.range_cache_insert($key, __value.clone());
                __value
            }
        }
    }};
}
