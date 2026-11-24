// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::{
    area_bool, area_date_time, area_f64, area_string, area_time, vec_bool, vec_date_time, vec_f64,
    vec_string, vec_time, Value,
};
use crate::value_format::ValueFormat;
use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, Mutex};

/// A map with a fast hasher. Its keys are the generator's names and numbers, never data from
/// outside, so the hashing need not resist chosen keys; looking them up is a large part of the
/// time of a calculation.
pub(crate) type FastMap<K, V> = HashMap<K, V, rustc_hash::FxBuildHasher>;

/// What a request has computed so far, for the same input values.
#[derive(Debug, Default)]
struct Caches {
    fn_cache: Mutex<FastMap<String, Value>>,
    /// Shared, so a range is handed out without copying its cells (see
    /// [`range_cache_get_shared`](Input::range_cache_get_shared)).
    range_cache: Mutex<FastMap<u64, Arc<Value>>>,
    fn_error_cache: Mutex<FastMap<String, String>>,
    /// Row dispatchers' values, by (dispatcher, row): see [`codcel_cache_row`](crate::codcel_cache_row).
    row_cache: Mutex<FastMap<(&'static str, i32), Value>>,
    row_error_cache: Mutex<FastMap<(&'static str, i32), String>>,
    fill_down_cache: Mutex<crate::fill_down_cells::FillDownCache>,
    warmed: Mutex<FastMap<u64, usize>>,
}

#[derive(Debug)]
pub struct Input {
    input_struct_name: String,
    map: FastMap<String, Value>,
    value_format: ValueFormat,
    caches: Arc<Caches>,
}

/// A clone starts with empty caches: it may be given other values. See
/// [`Input::clone_sharing_caches`] for a clone that keeps them.
impl Clone for Input {
    fn clone(&self) -> Self {
        Input {
            input_struct_name: self.input_struct_name.clone(),
            map: self.map.clone(),
            value_format: self.value_format.clone(),
            caches: Default::default(),
        }
    }
}

impl Serialize for Input {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Input", 3)?;
        state.serialize_field("input_struct_name", &self.input_struct_name)?;
        state.serialize_field("map", &self.map)?;
        state.serialize_field("value_format", &self.value_format)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Input {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct InputHelper {
            input_struct_name: String,
            map: FastMap<String, Value>,
            value_format: ValueFormat,
        }
        let helper = InputHelper::deserialize(deserializer)?;
        Ok(Input {
            input_struct_name: helper.input_struct_name,
            map: helper.map,
            value_format: helper.value_format,
            caches: Default::default(),
        })
    }
}

impl Input {
    /// A clone that shares this input's per-request caches, for code that evaluates further
    /// cells of the same request with the same values, such as a table cell that calls back into
    /// a generated function. Its values must not be changed, or the caches would be wrong.
    pub fn clone_sharing_caches(&self) -> Input {
        Input {
            input_struct_name: self.input_struct_name.clone(),
            map: self.map.clone(),
            value_format: self.value_format.clone(),
            caches: Arc::clone(&self.caches),
        }
    }

    pub fn new(input_struct_name: &str, value_format: ValueFormat) -> Input {
        Input {
            input_struct_name: input_struct_name.to_string(),
            map: Default::default(),
            value_format,
            caches: Default::default(),
        }
    }

    /// Check the per-request function cache.
    pub fn fn_cache_get(&self, key: &str) -> Option<Value> {
        crate::profile::count(&crate::profile::FN_CACHE_GETS);
        crate::profile::count_key(key);
        let value = self
            .caches
            .fn_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(key)
            .cloned();
        if value.is_some() {
            crate::profile::count(&crate::profile::FN_CACHE_HITS);
        } else {
            crate::profile::count_miss(key);
        }
        value
    }

    /// Insert into the per-request function cache.
    /// Skips caching for large collections where the clone cost exceeds the lookup cost.
    pub fn fn_cache_insert(&self, key: String, value: Value) {
        if value.is_cacheable() {
            self.caches
                .fn_cache
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .insert(key, value);
        } else {
            crate::profile::count(&crate::profile::FN_CACHE_SKIPPED);
        }
    }

    /// The message of the error a cached function returned earlier in this request, if it did.
    /// Excel computes an error cell once like any other cell; without this, every reader of a
    /// cell that fails would compute it, and everything it reads, again.
    pub fn fn_error_get(&self, key: &str) -> Option<String> {
        let message = self
            .caches
            .fn_error_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(key)
            .cloned();
        if message.is_some() {
            crate::profile::count(&crate::profile::FN_ERROR_HITS);
        }
        message
    }

    /// Remember that the cached function `key` returned the error `message` in this request.
    pub fn fn_error_insert(&self, key: String, message: String) {
        self.caches
            .fn_error_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(key, message);
    }

    /// The value row `row` of row dispatcher `dispatcher` returned earlier in this request, if
    /// it did. Like [`fn_cache_get`](Self::fn_cache_get) with the key `"{dispatcher}_{row}"`,
    /// without building the key.
    pub fn row_cache_get(&self, dispatcher: &'static str, row: i32) -> Option<Value> {
        crate::profile::count(&crate::profile::FN_CACHE_GETS);
        if crate::profile::keys_enabled() {
            let key = format!("{dispatcher}_{row}");
            crate::profile::count_key(&key);
        }
        let value = self
            .caches
            .row_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(&(dispatcher, row))
            .cloned();
        if value.is_some() {
            crate::profile::count(&crate::profile::FN_CACHE_HITS);
        } else if crate::profile::keys_enabled() {
            crate::profile::count_miss(&format!("{dispatcher}_{row}"));
        }
        value
    }

    /// Keeps the value of row `row` of row dispatcher `dispatcher` for the rest of the request,
    /// unless it is too large (as [`fn_cache_insert`](Self::fn_cache_insert)).
    pub fn row_cache_insert(&self, dispatcher: &'static str, row: i32, value: Value) {
        if value.is_cacheable() {
            self.caches
                .row_cache
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .insert((dispatcher, row), value);
        } else {
            crate::profile::count(&crate::profile::FN_CACHE_SKIPPED);
        }
    }

    /// The message of the error row `row` of row dispatcher `dispatcher` returned earlier in
    /// this request, if it did (as [`fn_error_get`](Self::fn_error_get)).
    pub fn row_error_get(&self, dispatcher: &'static str, row: i32) -> Option<String> {
        let message = self
            .caches
            .row_error_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(&(dispatcher, row))
            .cloned();
        if message.is_some() {
            crate::profile::count(&crate::profile::FN_ERROR_HITS);
        }
        message
    }

    /// Remembers that row `row` of row dispatcher `dispatcher` returned the error `message`.
    pub fn row_error_insert(&self, dispatcher: &'static str, row: i32, message: String) {
        self.caches
            .row_error_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert((dispatcher, row), message);
    }

    /// How many of a row dispatcher's rows have been warmed (evaluated in order, so a chain of
    /// rows never recurses deeply) in this request. `dispatcher` identifies the dispatcher.
    pub fn warmed(&self, dispatcher: u64) -> usize {
        *self
            .caches
            .warmed
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(&dispatcher)
            .unwrap_or(&0)
    }

    /// Records that the first `count` rows of a row dispatcher have been warmed.
    pub fn set_warmed(&self, dispatcher: u64, count: usize) {
        let mut warmed = self
            .caches
            .warmed
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let entry = warmed.entry(dispatcher).or_insert(0);
        *entry = (*entry).max(count);
    }

    /// Check the per-request range cache, which holds values of any size, such as a column
    /// of a fill-down range that several formulas read. The key identifies the expression that
    /// computed the value (see [`codcel_cached`](crate::codcel_cached)).
    pub fn range_cache_get(&self, key: u64) -> Option<Value> {
        self.range_cache_get_shared(key)
            .map(|value| Value::clone(&value))
    }

    /// [`range_cache_get`](Self::range_cache_get) without copying the range: for functions that
    /// read a range by reference.
    pub fn range_cache_get_shared(&self, key: u64) -> Option<Arc<Value>> {
        crate::profile::count(&crate::profile::RANGE_CACHE_GETS);
        let value = self
            .caches
            .range_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .get(&key)
            .cloned();
        if value.is_some() {
            crate::profile::count(&crate::profile::RANGE_CACHE_HITS);
        }
        value
    }

    /// Insert into the per-request range cache. Unlike [`fn_cache_insert`](Self::fn_cache_insert),
    /// large values are kept: rebuilding a range costs at least as much as cloning it.
    pub fn range_cache_insert(&self, key: u64, value: Value) {
        self.range_cache_insert_shared(key, Arc::new(value));
    }

    /// [`range_cache_insert`](Self::range_cache_insert) of a range already shared.
    pub fn range_cache_insert_shared(&self, key: u64, value: Arc<Value>) {
        self.caches
            .range_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .insert(key, value);
    }

    pub(crate) fn fill_down_cache(
        &self,
    ) -> std::sync::MutexGuard<'_, crate::fill_down_cells::FillDownCache> {
        self.caches
            .fill_down_cache
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Returns a reference to the `ValueFormat` for this calculation.
    pub fn format(&self) -> &ValueFormat {
        &self.value_format
    }

    /// Replaces the `ValueFormat` for this calculation (per-call override).
    pub fn set_format(&mut self, value_format: ValueFormat) {
        self.value_format = value_format;
    }

    // ── Strings ──────────────────────────────────────────────────────────────────
    pub fn add_string<S: AsRef<str>>(&mut self, name: &str, value: S) {
        self.map
            .insert(name.to_string(), Value::String(value.as_ref().to_string()));
    }

    pub fn add_string_option<S: AsRef<str>>(&mut self, name: &str, value: &Option<S>) {
        if let Some(value) = value {
            self.map
                .insert(name.to_string(), Value::String(value.as_ref().to_string()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    pub fn add_area_string(&mut self, name: &str, value: &[Vec<String>]) {
        self.map
            .insert(name.to_string(), area_string(value.to_owned()));
    }

    pub fn add_area_string_option(&mut self, name: &str, value: &Option<Vec<Vec<String>>>) {
        if let Some(value) = value {
            self.map
                .insert(name.to_string(), area_string(value.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    pub fn add_vec_string(&mut self, name: &str, value: &[String]) {
        self.map
            .insert(name.to_string(), vec_string(value.to_owned()));
    }

    pub fn add_vec_string_option(&mut self, name: &str, value: &Option<Vec<String>>) {
        if let Some(value) = value {
            self.map
                .insert(name.to_string(), vec_string(value.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    // ── Floats ──────────────────────────────────────────────────────────────────
    pub fn add_float(&mut self, name: &str, value: f64) {
        self.map.insert(name.to_string(), Value::F64(value));
    }

    pub fn add_area_float(&mut self, name: &str, value: &[Vec<f64>]) {
        self.map
            .insert(name.to_string(), area_f64(value.to_owned()));
    }

    pub fn add_vec_float(&mut self, name: &str, value: &[f64]) {
        self.map.insert(name.to_string(), vec_f64(value.to_owned()));
    }

    pub fn add_float_option(&mut self, name: &str, value: &Option<f64>) {
        if let Some(v) = value {
            self.map.insert(name.to_string(), Value::F64(*v));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    pub fn add_area_float_option(&mut self, name: &str, value: &Option<Vec<Vec<f64>>>) {
        if let Some(v) = value {
            self.map.insert(name.to_string(), area_f64(v.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    pub fn add_vec_float_option(&mut self, name: &str, value: &Option<Vec<f64>>) {
        if let Some(v) = value {
            self.map.insert(name.to_string(), vec_f64(v.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    // ── Booleans ────────────────────────────────────────────────────────────────
    pub fn add_boolean(&mut self, name: &str, value: bool) {
        self.map.insert(name.to_string(), Value::Bool(value));
    }

    pub fn add_area_boolean(&mut self, name: &str, value: &[Vec<bool>]) {
        self.map
            .insert(name.to_string(), area_bool(value.to_owned()));
    }

    pub fn add_vec_boolean(&mut self, name: &str, value: &[bool]) {
        self.map
            .insert(name.to_string(), vec_bool(value.to_owned()));
    }

    pub fn add_boolean_option(&mut self, name: &str, value: &Option<bool>) {
        if let Some(v) = value {
            self.map.insert(name.to_string(), Value::Bool(*v));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    pub fn add_area_boolean_option(&mut self, name: &str, value: &Option<Vec<Vec<bool>>>) {
        if let Some(v) = value {
            self.map.insert(name.to_string(), area_bool(v.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    pub fn add_vec_boolean_option(&mut self, name: &str, value: &Option<Vec<bool>>) {
        if let Some(v) = value {
            self.map.insert(name.to_string(), vec_bool(v.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    // ── DateTime<Utc> ──────────────────────────────────────────────────────────
    pub fn add_date_time(&mut self, name: &str, value: DateTime<Utc>) {
        self.map
            .insert(name.to_string(), Value::ChronoDateTime(value));
    }

    pub fn add_area_date_time(&mut self, name: &str, value: &[Vec<DateTime<Utc>>]) {
        self.map
            .insert(name.to_string(), area_date_time(value.to_owned()));
    }

    pub fn add_vec_date_time(&mut self, name: &str, value: &[DateTime<Utc>]) {
        self.map
            .insert(name.to_string(), vec_date_time(value.to_owned()));
    }

    pub fn add_date_time_option(&mut self, name: &str, value: &Option<DateTime<Utc>>) {
        if let Some(v) = value {
            self.map
                .insert(name.to_string(), Value::ChronoDateTime(v.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    pub fn add_area_date_time_option(
        &mut self,
        name: &str,
        value: &Option<Vec<Vec<DateTime<Utc>>>>,
    ) {
        if let Some(v) = value {
            self.map
                .insert(name.to_string(), area_date_time(v.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    pub fn add_vec_date_time_option(&mut self, name: &str, value: &Option<Vec<DateTime<Utc>>>) {
        if let Some(v) = value {
            self.map
                .insert(name.to_string(), vec_date_time(v.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    // ── NaiveTime ───────────────────────────────────────────────────────────────
    pub fn add_time(&mut self, name: &str, value: NaiveTime) {
        self.map.insert(name.to_string(), Value::Time(value));
    }

    pub fn add_area_time(&mut self, name: &str, value: &[Vec<NaiveTime>]) {
        self.map
            .insert(name.to_string(), area_time(value.to_owned()));
    }

    pub fn add_vec_time(&mut self, name: &str, value: &[NaiveTime]) {
        self.map
            .insert(name.to_string(), vec_time(value.to_owned()));
    }

    pub fn add_time_option(&mut self, name: &str, value: &Option<NaiveTime>) {
        if let Some(v) = value {
            self.map.insert(name.to_string(), Value::Time(v.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    pub fn add_area_time_option(&mut self, name: &str, value: &Option<Vec<Vec<NaiveTime>>>) {
        if let Some(v) = value {
            self.map.insert(name.to_string(), area_time(v.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    pub fn add_vec_time_option(&mut self, name: &str, value: &Option<Vec<NaiveTime>>) {
        if let Some(v) = value {
            self.map.insert(name.to_string(), vec_time(v.to_owned()));
        } else {
            self.map.insert(name.to_string(), Value::None);
        }
    }

    // ── Get ───────────────────────────────────────────────────────────────
    pub fn get(&self, name: &str) -> Result<Value, Box<dyn Error + Send + Sync>> {
        match self.map.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(format!("{} does not exist on {}", name, self.input_struct_name).into()),
        }
    }
}
