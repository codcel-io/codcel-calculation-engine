// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

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
        let __result = $body;
        if let Ok(ref val) = __result {
            if val.is_cacheable() {
                $input.fn_cache_insert($key.to_string(), val.clone());
            }
        }
        __result
    }};
}
