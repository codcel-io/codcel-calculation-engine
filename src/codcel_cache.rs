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

/// [`codcel_cache!`] for a row dispatcher's rows: `$dispatcher` (a `&'static str`, the
/// dispatcher's name) and `$row` (an `i32`) in place of the key `"{dispatcher}_{row}"`, which is
/// then not built on every call. The same values and errors are kept, and returned, as with
/// that key.
#[macro_export]
macro_rules! codcel_cache_row {
    ($input:expr, $dispatcher:expr, $row:expr, $body:expr) => {{
        if let Some(cached) = $input.row_cache_get($dispatcher, $row) {
            return Ok(cached);
        }
        if let Some(message) = $input.row_error_get($dispatcher, $row) {
            return Err(::std::boxed::Box::<dyn ::std::error::Error + Send + Sync>::from(message));
        }
        let __result: ::std::result::Result<
            _,
            ::std::boxed::Box<dyn ::std::error::Error + Send + Sync>,
        > = $body;
        match __result {
            Ok(ref val) => $input.row_cache_insert($dispatcher, $row, val.clone()),
            Err(ref error) => $input.row_error_insert($dispatcher, $row, error.to_string()),
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

#[cfg(test)]
mod tests {
    use crate::input::Input;
    use crate::value::Value;
    use crate::value_format::ValueFormat;
    use std::error::Error;
    use std::sync::atomic::{AtomicUsize, Ordering};

    type Result = std::result::Result<Value, Box<dyn Error + Send + Sync>>;

    static CALLS: AtomicUsize = AtomicUsize::new(0);

    fn value_of(row: i32) -> Result {
        CALLS.fetch_add(1, Ordering::SeqCst);
        match row {
            3 => Err("row 3 fails".into()),
            4 => Ok(Value::VecValue(vec![Value::F64(1.0); 101])),
            _ => Ok(Value::F64(f64::from(row) * 1.5)),
        }
    }

    fn by_key(input: &Input, row: i32) -> Result {
        let key = format!("dispatch_x_{row}");
        codcel_cache!(input, key.as_str(), { value_of(row) })
    }

    fn by_row(input: &Input, row: i32) -> Result {
        codcel_cache_row!(input, "dispatch_x", row, { value_of(row) })
    }

    fn outcomes(dispatcher: fn(&Input, i32) -> Result) -> (Vec<String>, usize) {
        let input = Input::new("Test", ValueFormat::default());
        CALLS.store(0, Ordering::SeqCst);
        let mut seen = Vec::new();
        for _ in 0..3 {
            for row in [1, 2, 3, 4, 2, 1] {
                seen.push(match dispatcher(&input, row) {
                    Ok(value) => format!("{value:?}"),
                    Err(error) => format!("error {error}"),
                });
            }
        }
        (seen, CALLS.load(Ordering::SeqCst))
    }

    /// Keyed by (dispatcher, row), the cache keeps and returns the same values and errors, and
    /// computes the same cells, as keyed by the string `"{dispatcher}_{row}"`.
    #[test]
    fn row_cache_behaves_as_the_string_keyed_cache() {
        let (by_key, key_calls) = outcomes(by_key);
        let (by_row, row_calls) = outcomes(by_row);
        assert_eq!(by_key, by_row);
        assert_eq!(key_calls, row_calls);
        // Rows 1, 2 and 3 once each; row 4 is too large to keep, so once per round.
        assert_eq!(row_calls, 3 + 3);
    }
}
