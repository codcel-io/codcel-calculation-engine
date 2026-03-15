// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::value::{
    area_bool, area_date_time, area_f64, area_string, area_time, vec_bool, vec_date_time, vec_f64,
    vec_string, vec_time, Value,
};
use crate::value_format::ValueFormat;
use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Input {
    input_struct_name: String,
    map: HashMap<String, Value>,
    value_format: ValueFormat,
    fn_cache: Mutex<HashMap<String, Value>>,
}

impl Clone for Input {
    fn clone(&self) -> Self {
        Input {
            input_struct_name: self.input_struct_name.clone(),
            map: self.map.clone(),
            value_format: self.value_format.clone(),
            fn_cache: Mutex::new(HashMap::new()),
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
            map: HashMap<String, Value>,
            value_format: ValueFormat,
        }
        let helper = InputHelper::deserialize(deserializer)?;
        Ok(Input {
            input_struct_name: helper.input_struct_name,
            map: helper.map,
            value_format: helper.value_format,
            fn_cache: Mutex::new(HashMap::new()),
        })
    }
}

impl Input {
    pub fn new(input_struct_name: &str, value_format: ValueFormat) -> Input {
        Input {
            input_struct_name: input_struct_name.to_string(),
            map: Default::default(),
            value_format,
            fn_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Check the per-request function cache.
    pub fn fn_cache_get(&self, key: &str) -> Option<Value> {
        self.fn_cache.lock().unwrap().get(key).cloned()
    }

    /// Insert into the per-request function cache.
    /// Skips caching for large collections where the clone cost exceeds the lookup cost.
    pub fn fn_cache_insert(&self, key: String, value: Value) {
        if value.is_cacheable() {
            self.fn_cache.lock().unwrap().insert(key, value);
        }
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
        if self.map.contains_key(name) {
            Ok(self.map[name].clone())
        } else {
            Err(format!("{} does not exist on {}", name, self.input_struct_name).into())
        }
    }
}
