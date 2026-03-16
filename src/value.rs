// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::arithmetic_base::{boolean, float, float_to_string, integer};
use crate::date_time_base::{
    date_time_to_excel, date_time_to_iso, date_time_to_time, excel_to_date_time, excel_to_time,
    force_string_to_date_time, force_string_to_time, time_to_date_time, time_to_excel, time_to_iso,
};
use crate::value_format::ValueFormat;
use chrono::{DateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    F64(f64),
    I32(i32),
    String(String),
    Bool(bool),
    OptionF64(Option<f64>),
    OptionI32(Option<i32>),
    OptionString(Option<String>),
    OptionBool(Option<bool>),
    VecValue(Vec<Value>),
    OptionVecValue(Option<Vec<Value>>),
    AreaValue(Vec<Vec<Value>>), // The outer Vec are the rows.  The inner Vec are the columns.
    OptionAreaValue(Option<Vec<Vec<Value>>>), // The outer Vec are the rows.  The inner Vec are the columns.
    OptionChronoDateTime(Option<DateTime<Utc>>),
    ChronoDateTime(DateTime<Utc>),
    OptionTime(Option<NaiveTime>),
    Time(NaiveTime),
    None,
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::F64(a), Value::F64(b)) => a.partial_cmp(b),
            (Value::I32(a), Value::F64(b)) => (*a as f64).partial_cmp(b),
            (Value::F64(a), Value::I32(b)) => a.partial_cmp(&(*b as f64)),
            (Value::I32(a), Value::I32(b)) => a.partial_cmp(b),
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::OptionF64(Some(a)), Value::OptionF64(Some(b))) => a.partial_cmp(b),
            (Value::OptionI32(Some(a)), Value::OptionI32(Some(b))) => a.partial_cmp(b),
            (Value::OptionString(Some(a)), Value::OptionString(Some(b))) => a.partial_cmp(b),
            (Value::OptionBool(Some(a)), Value::OptionBool(Some(b))) => a.partial_cmp(b),
            (Value::None, Value::None) => Some(std::cmp::Ordering::Equal),
            (Value::ChronoDateTime(a), Value::ChronoDateTime(b)) => a.partial_cmp(b),
            (Value::OptionChronoDateTime(Some(a)), Value::OptionChronoDateTime(Some(b))) => {
                a.partial_cmp(b)
            }
            (Value::Time(a), Value::Time(b)) => a.partial_cmp(b),
            (Value::OptionTime(Some(a)), Value::OptionTime(Some(b))) => a.partial_cmp(b),

            // Comparisons involving Option types with None values
            (Value::OptionF64(None), _) => Some(std::cmp::Ordering::Less),
            (_, Value::OptionF64(None)) => Some(std::cmp::Ordering::Greater),
            (Value::OptionI32(None), _) => Some(std::cmp::Ordering::Less),
            (_, Value::OptionI32(None)) => Some(std::cmp::Ordering::Greater),
            (Value::OptionString(None), _) => Some(std::cmp::Ordering::Less),
            (_, Value::OptionString(None)) => Some(std::cmp::Ordering::Greater),
            (Value::OptionBool(None), _) => Some(std::cmp::Ordering::Less),
            (_, Value::OptionBool(None)) => Some(std::cmp::Ordering::Greater),
            (Value::OptionChronoDateTime(None), _) => Some(std::cmp::Ordering::Less),
            (_, Value::OptionChronoDateTime(None)) => Some(std::cmp::Ordering::Greater),
            (Value::OptionTime(None), _) => Some(std::cmp::Ordering::Less),
            (_, Value::OptionTime(None)) => Some(std::cmp::Ordering::Greater),

            // Comparisons involving VecValue
            (Value::VecValue(a), Value::VecValue(b)) => a.partial_cmp(b),
            (Value::OptionVecValue(Some(a)), Value::OptionVecValue(Some(b))) => a.partial_cmp(b),
            (Value::OptionVecValue(None), _) => Some(std::cmp::Ordering::Less),
            (_, Value::OptionVecValue(None)) => Some(std::cmp::Ordering::Greater),

            // Comparisons involving AreaValue
            (Value::AreaValue(a), Value::AreaValue(b)) => a.partial_cmp(b),
            (Value::OptionAreaValue(Some(a)), Value::OptionAreaValue(Some(b))) => a.partial_cmp(b),
            (Value::OptionAreaValue(None), _) => Some(std::cmp::Ordering::Less),
            (_, Value::OptionAreaValue(None)) => Some(std::cmp::Ordering::Greater),

            // Fallback for mismatched types
            _ => None, // Return None for comparisons between incompatible types
        }
    }
}

// Manual implementation of PartialEq for Value to handle f64 comparisons
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::F64(a), Value::F64(b)) => a == b || (a.is_nan() && b.is_nan()),
            (Value::I32(a), Value::I32(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::OptionF64(Some(a)), Value::OptionF64(Some(b))) => {
                a == b || (a.is_nan() && b.is_nan())
            }
            (Value::OptionF64(None), Value::OptionF64(None)) => true,
            (Value::OptionI32(Some(a)), Value::OptionI32(Some(b))) => a == b,
            (Value::OptionI32(None), Value::OptionI32(None)) => true,
            (Value::OptionString(Some(a)), Value::OptionString(Some(b))) => a == b,
            (Value::OptionString(None), Value::OptionString(None)) => true,
            (Value::OptionBool(Some(a)), Value::OptionBool(Some(b))) => a == b,
            (Value::OptionBool(None), Value::OptionBool(None)) => true,
            (Value::VecValue(a), Value::VecValue(b)) => a == b,
            (Value::OptionVecValue(Some(a)), Value::OptionVecValue(Some(b))) => a == b,
            (Value::OptionVecValue(None), Value::OptionVecValue(None)) => true,
            (Value::AreaValue(a), Value::AreaValue(b)) => a == b,
            (Value::OptionAreaValue(Some(a)), Value::OptionAreaValue(Some(b))) => a == b,
            (Value::OptionAreaValue(None), Value::OptionAreaValue(None)) => true,
            (Value::None, Value::None) => true,
            (Value::ChronoDateTime(a), Value::ChronoDateTime(b)) => a == b,
            (Value::OptionChronoDateTime(Some(a)), Value::OptionChronoDateTime(Some(b))) => a == b,
            (Value::Time(a), Value::Time(b)) => a == b,
            (Value::OptionTime(Some(a)), Value::OptionTime(Some(b))) => a == b,
            _ => false,
        }
    }
}

// Implement Eq for Value (Eq requires that PartialEq is implemented)
impl Eq for Value {}

// Implement Hash manually for Value to handle f64 and Option<f64>
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::F64(v) => {
                if v.is_nan() {
                    // Hash NaN values as a constant
                    0x7ff8000000000000u64.hash(state);
                } else {
                    // Convert the f64 to a u64 and hash it
                    v.to_bits().hash(state);
                }
            }
            Value::I32(v) => v.hash(state),
            Value::String(v) => v.hash(state),
            Value::Bool(v) => v.hash(state),
            Value::OptionF64(Some(v)) => {
                if v.is_nan() {
                    0x7ff8000000000000u64.hash(state);
                } else {
                    v.to_bits().hash(state);
                }
            }
            Value::OptionF64(None) => {
                "None::<f64>".hash(state);
            }
            Value::OptionI32(Some(v)) => v.hash(state),
            Value::OptionI32(None) => None::<i32>.hash(state),
            Value::OptionString(Some(v)) => v.hash(state),
            Value::OptionString(None) => None::<String>.hash(state),
            Value::OptionBool(Some(v)) => v.hash(state),
            Value::OptionBool(None) => None::<bool>.hash(state),
            Value::VecValue(v) => v.hash(state),
            Value::OptionVecValue(Some(v)) => v.hash(state),
            Value::OptionVecValue(None) => None::<Vec<Value>>.hash(state),
            Value::AreaValue(v) => v.hash(state),
            Value::OptionAreaValue(Some(v)) => v.hash(state),
            Value::OptionAreaValue(None) => None::<Vec<Vec<Value>>>.hash(state),
            Value::None => 0_u8.hash(state),
            Value::ChronoDateTime(v) => v.hash(state),
            Value::OptionChronoDateTime(None) => None::<String>.hash(state),
            Value::OptionChronoDateTime(Some(v)) => v.hash(state),
            Value::Time(v) => v.hash(state),
            Value::OptionTime(None) => None::<String>.hash(state),
            Value::OptionTime(Some(v)) => v.hash(state),
        }
    }
}

impl Value {
    /// Returns `true` when the value is cheap to clone (scalars or small collections).
    /// Large vectors/areas should not be stored in per-request caches that clone on read.
    pub fn is_cacheable(&self) -> bool {
        match self {
            Value::VecValue(v) | Value::OptionVecValue(Some(v)) => v.len() <= 100,
            Value::AreaValue(v) | Value::OptionAreaValue(Some(v)) => {
                v.iter().map(|r| r.len()).sum::<usize>() <= 100
            }
            _ => true, // scalars are always cheap to clone
        }
    }

    pub fn to_string_for_wildcard(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::OptionString(Some(s)) => s.clone(),
            Value::F64(f) => f.to_string(),
            Value::I32(i) => i.to_string(),
            Value::Bool(b) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
            Value::OptionF64(Some(f)) => f.to_string(),
            Value::OptionI32(Some(i)) => i.to_string(),
            Value::OptionBool(Some(b)) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
            Value::ChronoDateTime(dt) => dt.to_string(),
            Value::OptionChronoDateTime(Some(dt)) => dt.to_string(),
            Value::Time(t) => time_to_iso(t),
            Value::OptionTime(Some(t)) => time_to_iso(t),
            Value::None => "".to_string(),
            _ => format!("{self:?}"), // Fallback for complex types
        }
    }

    pub fn to_single_value(&self) -> Value {
        let single_value = self.clone();
        match self {
            Value::F64(_) => single_value,
            Value::I32(_) => single_value,
            Value::String(_) => single_value,
            Value::Bool(_) => single_value,
            Value::OptionF64(_) => single_value,
            Value::OptionI32(_) => single_value,
            Value::OptionString(_) => single_value,
            Value::OptionBool(_) => single_value,
            Value::VecValue(value) => {
                if let Some(val) = value.first() {
                    val.clone()
                } else {
                    single_value
                }
            }
            Value::OptionVecValue(value_option) => {
                if let Some(value) = value_option {
                    if let Some(val) = value.first() {
                        return val.clone();
                    }
                }
                single_value
            }
            Value::AreaValue(value) => {
                if let Some(val_outer) = value.first() {
                    if let Some(val) = val_outer.first() {
                        return val.clone();
                    }
                }
                single_value
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if let Some(val_outer) = value.first() {
                        if let Some(val) = val_outer.first() {
                            return val.clone();
                        }
                    }
                }
                single_value
            }
            Value::None => single_value,
            Value::ChronoDateTime(_) => single_value,
            Value::OptionChronoDateTime(_) => single_value,
            Value::Time(_) => single_value,
            Value::OptionTime(_) => single_value,
        }
    }

    pub fn is_number_type(&self, other: &Value) -> bool {
        matches!(
            (self, other),
            (Value::F64(_), Value::F64(_))
                | (Value::F64(_), Value::I32(_))
                | (Value::I32(_), Value::I32(_))
                | (Value::I32(_), Value::F64(_))
        )
    }

    pub fn is_count_number_type(&self) -> bool {
        match self {
            Value::F64(_) => true,
            Value::I32(_) => true,
            Value::OptionF64(value) => value.is_some(),
            Value::OptionI32(value) => value.is_some(),
            Value::OptionChronoDateTime(value) => value.is_some(),
            Value::ChronoDateTime(_) => true,
            Value::OptionTime(value) => value.is_some(),
            Value::Time(_) => true,
            _ => false,
        }
    }

    pub fn is_excel_single_number(&self) -> bool {
        matches!(
            &self,
            Value::F64(_) | Value::I32(_) | Value::OptionI32(Some(_)) | Value::OptionF64(Some(_))
        )
    }

    pub fn is_excel_single_text(&self) -> bool {
        matches!(&self, Value::String(_) | Value::OptionString(Some(_)))
    }

    pub fn is_same_type(&self, other: &Value) -> bool {
        matches!(
            (self, other),
            (Value::OptionTime(_), Value::OptionTime(_))
                | (Value::Time(_), Value::Time(_))
                | (
                    Value::OptionChronoDateTime(_),
                    Value::OptionChronoDateTime(_)
                )
                | (Value::ChronoDateTime(_), Value::ChronoDateTime(_))
                | (Value::F64(_), Value::F64(_))
                | (Value::I32(_), Value::I32(_))
                | (Value::String(_), Value::String(_))
                | (Value::Bool(_), Value::Bool(_))
                | (Value::OptionF64(_), Value::OptionF64(_))
                | (Value::OptionI32(_), Value::OptionI32(_))
                | (Value::OptionString(_), Value::OptionString(_))
                | (Value::OptionBool(_), Value::OptionBool(_))
                | (Value::VecValue(_), Value::VecValue(_))
                | (Value::OptionVecValue(_), Value::OptionVecValue(_))
                | (Value::AreaValue(_), Value::AreaValue(_))
                | (Value::OptionAreaValue(_), Value::OptionAreaValue(_))
                | (Value::None, Value::None)
        )
    }

    pub fn is_single_string(&self) -> bool {
        match self {
            Value::String(value) => is_pure_string(value),
            Value::OptionString(Some(value)) => is_pure_string(value), // Matching directly with Some
            Value::OptionString(None) => false,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Value::VecValue(_) | Value::OptionVecValue(_))
    }

    pub fn is_area(&self) -> bool {
        matches!(self, Value::AreaValue(_))
    }

    pub fn is_datetime(&self) -> bool {
        matches!(
            self,
            Value::OptionChronoDateTime(_) | Value::ChronoDateTime(_)
        )
    }

    pub fn is_time(&self) -> bool {
        matches!(self, Value::OptionTime(_) | Value::Time(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::OptionString(_) | Value::String(_))
    }

    pub fn f64(&self, value_format: &ValueFormat) -> Result<f64, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(*value),
            Value::I32(value) => float(*value, &value_format.decimal_separator),
            Value::String(value) => float(value.as_str(), &value_format.decimal_separator),
            Value::Bool(value) => float(*value, &value_format.decimal_separator),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert float none to number value".into()),
                Some(value) => Ok(*value),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert integer none to number value".into()),
                Some(value) => float(*value, &value_format.decimal_separator),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to number value".into()),
                Some(value) => float(value.as_str(), &value_format.decimal_separator),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to number value".into()),
                Some(value) => float(*value, &value_format.decimal_separator),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to number value".into())
                } else {
                    value[0].f64(value_format)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to number value".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty velue list to number value".into())
                    } else {
                        value[0].f64(value_format)
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return value[0][0].f64(value_format);
                        }
                    }
                }
                Err("Cannot convert empty value area to number value".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return value[0][0].f64(value_format);
                            }
                        }
                    }
                }
                Err("Cannot convert empty value area to number value".into())
            }
            Value::None => Err("Cannot convert none to number value".into()),
            Value::ChronoDateTime(value) => date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug),
            Value::OptionChronoDateTime(value) => match value {
                None => Err("Cannot convert date time none to number value".into()),
                Some(value) => date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug),
            },
            Value::OptionTime(value) => match value {
                None => Err("Cannot convert time none to number value".into()),
                Some(value) => time_to_excel(value),
            },
            Value::Time(value) => time_to_excel(value),
        }
    }

    pub fn date_time(
        &self,
        value_format: &ValueFormat,
    ) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => excel_to_date_time(*value, value_format.allow_lotus_1_2_3_1900_date_bug),
            Value::I32(value) => {
                excel_to_date_time(float(*value, &value_format.decimal_separator)?, value_format.allow_lotus_1_2_3_1900_date_bug)
            }
            Value::String(value) => {
                force_string_to_date_time(value, &value_format.decimal_separator, value_format.allow_lotus_1_2_3_1900_date_bug)
            }
            Value::Bool(_value) => Err("Cannot convert boolean to date time value".into()),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert float none to date time value".into()),
                Some(value) => excel_to_date_time(*value, value_format.allow_lotus_1_2_3_1900_date_bug),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert integer none to date time value".into()),
                Some(value) => excel_to_date_time(float(*value, &value_format.decimal_separator)?, value_format.allow_lotus_1_2_3_1900_date_bug),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to date time value".into()),
                Some(value) => force_string_to_date_time(value, &value_format.decimal_separator, value_format.allow_lotus_1_2_3_1900_date_bug),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to date time value".into()),
                Some(_value) => Err("Cannot convert boolean to date time value".into()),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to date time value".into())
                } else {
                    value[0].date_time(value_format)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to date time value".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty velue list to date time value".into())
                    } else {
                        value[0].date_time(value_format)
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return value[0][0].date_time(value_format);
                        }
                    }
                }
                Err("Cannot convert empty value area to date time value".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return value[0][0].date_time(value_format);
                            }
                        }
                    }
                }
                Err("Cannot convert empty value area to date time value".into())
            }
            Value::None => Err("Cannot convert none to date time value".into()),
            Value::ChronoDateTime(value) => Ok(*value),
            Value::OptionChronoDateTime(value) => match value {
                None => Err("Cannot convert date time none to date time value".into()),
                Some(value) => Ok(*value),
            },
            Value::OptionTime(value) => match value {
                None => Err("Cannot convert time none to date time value".into()),
                Some(value) => Ok(time_to_date_time(*value)?),
            },
            Value::Time(value) => Ok(time_to_date_time(*value)?),
        }
    }

    pub fn option_date_time(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Option<DateTime<Utc>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(Some(excel_to_date_time(*value, value_format.allow_lotus_1_2_3_1900_date_bug)?)),
            Value::I32(value) => Ok(Some(excel_to_date_time(float(
                *value,
                &value_format.decimal_separator,
            )?, value_format.allow_lotus_1_2_3_1900_date_bug)?)),
            Value::String(value) => Ok(Some(force_string_to_date_time(
                value,
                &value_format.decimal_separator,
                value_format.allow_lotus_1_2_3_1900_date_bug,
            )?)),
            Value::Bool(_value) => Ok(None),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(excel_to_date_time(*value, value_format.allow_lotus_1_2_3_1900_date_bug)?)),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(excel_to_date_time(float(
                    *value,
                    &value_format.decimal_separator,
                )?, value_format.allow_lotus_1_2_3_1900_date_bug)?)),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(force_string_to_date_time(
                    value,
                    &value_format.decimal_separator,
                    value_format.allow_lotus_1_2_3_1900_date_bug,
                )?)),
            },
            Value::OptionBool(_value) => Ok(None),
            Value::VecValue(value) => {
                if value.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(value[0].date_time(value_format)?))
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => {
                    if value.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(value[0].date_time(value_format)?))
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return Ok(Some(value[0][0].date_time(value_format)?));
                        }
                    }
                }
                Ok(None)
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return Ok(Some(value[0][0].date_time(value_format)?));
                            }
                        }
                    }
                }
                Ok(None)
            }
            Value::None => Ok(None),
            Value::ChronoDateTime(value) => Ok(Some(*value)),
            Value::OptionChronoDateTime(value) => Ok(*value),
            Value::OptionTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(time_to_date_time(*value)?)),
            },
            Value::Time(value) => Ok(Some(time_to_date_time(*value)?)),
        }
    }

    pub fn time(
        &self,
        value_format: &ValueFormat,
    ) -> Result<NaiveTime, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => excel_to_time(*value),
            Value::I32(value) => excel_to_time(float(*value, &value_format.decimal_separator)?),
            Value::String(value) => force_string_to_time(value, &value_format.decimal_separator),
            Value::Bool(_value) => Err("Cannot convert boolean to time value".into()),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert float none to time value".into()),
                Some(value) => excel_to_time(*value),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert integer none to time value".into()),
                Some(value) => excel_to_time(float(*value, &value_format.decimal_separator)?),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to time value".into()),
                Some(value) => force_string_to_time(value, &value_format.decimal_separator),
            },
            Value::OptionBool(_value) => Err("Cannot convert boolean none to time value".into()),
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to time value".into())
                } else {
                    value[0].time(value_format)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to time value".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty velue list to time value".into())
                    } else {
                        value[0].time(value_format)
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return value[0][0].time(value_format);
                        }
                    }
                }
                Err("Cannot convert empty value area to time value".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return value[0][0].time(value_format);
                            }
                        }
                    }
                }
                Err("Cannot convert empty value area to time value".into())
            }
            Value::None => Err("Cannot convert none to time value".into()),
            Value::ChronoDateTime(value) => Ok(date_time_to_time(value)),
            Value::OptionChronoDateTime(value) => match value {
                None => Err("Cannot convert date time none to time value".into()),
                Some(value) => Ok(date_time_to_time(value)),
            },
            Value::OptionTime(value) => match value {
                None => Err("Cannot convert time none to date time value".into()),
                Some(value) => Ok(*value),
            },
            Value::Time(value) => Ok(*value),
        }
    }

    pub fn option_time(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Option<NaiveTime>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(Some(excel_to_time(*value)?)),
            Value::I32(value) => Ok(Some(excel_to_time(float(
                *value,
                &value_format.decimal_separator,
            )?)?)),
            Value::String(value) => Ok(Some(force_string_to_time(
                value,
                &value_format.decimal_separator,
            )?)),
            Value::Bool(_value) => Ok(None),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(excel_to_time(*value)?)),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(excel_to_time(float(
                    *value,
                    &value_format.decimal_separator,
                )?)?)),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(force_string_to_time(
                    value,
                    &value_format.decimal_separator,
                )?)),
            },
            Value::OptionBool(_value) => Ok(None),
            Value::VecValue(value) => {
                if value.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(value[0].time(value_format)?))
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => {
                    if value.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(value[0].time(value_format)?))
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return Ok(Some(value[0][0].time(value_format)?));
                        }
                    }
                }
                Ok(None)
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return Ok(Some(value[0][0].time(value_format)?));
                            }
                        }
                    }
                }
                Ok(None)
            }
            Value::None => Ok(None),
            Value::ChronoDateTime(value) => Ok(Some(date_time_to_time(value))),
            Value::OptionChronoDateTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(date_time_to_time(value))),
            },
            Value::OptionTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(*value)),
            },
            Value::Time(value) => Ok(Some(*value)),
        }
    }

    pub fn option_f64(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Option<f64>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(Some(*value)),
            Value::I32(value) => Ok(Some(float(*value, &value_format.decimal_separator)?)),
            Value::String(value) => Ok(Some(float(
                value.as_str(),
                &value_format.decimal_separator,
            )?)),
            Value::Bool(value) => Ok(Some(float(*value, &value_format.decimal_separator)?)),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(*value)),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(float(*value, &value_format.decimal_separator)?)),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(float(
                    value.as_str(),
                    &value_format.decimal_separator,
                )?)),
            },
            Value::OptionBool(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(float(*value, &value_format.decimal_separator)?)),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Ok(None)
                } else {
                    value[0].option_f64(value_format)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => {
                    if value.is_empty() {
                        Ok(None)
                    } else {
                        value[0].option_f64(value_format)
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return value[0][0].option_f64(value_format);
                        }
                    }
                }
                Ok(None)
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return value[0][0].option_f64(value_format);
                            }
                        }
                    }
                }

                Ok(None)
            }
            Value::None => Ok(None),
            Value::ChronoDateTime(value) => Ok(Some(date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?)),
            Value::OptionChronoDateTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?)),
            },
            Value::OptionTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(time_to_excel(value)?)),
            },
            Value::Time(value) => Ok(Some(time_to_excel(value)?)),
        }
    }

    pub fn vec_f64(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![*value]),
            Value::I32(value) => Ok(vec![float(*value, &value_format.decimal_separator)?]),
            Value::String(value) => Ok(vec![float(
                value.as_str(),
                &value_format.decimal_separator,
            )?]),
            Value::Bool(value) => Ok(vec![float(*value, &value_format.decimal_separator)?]),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert float none to number value list".into()),
                Some(value) => Ok(vec![*value]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert integer none to number value list".into()),
                Some(value) => Ok(vec![float(*value, &value_format.decimal_separator)?]),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to number value list".into()),
                Some(value) => Ok(vec![float(
                    value.as_str(),
                    &value_format.decimal_separator,
                )?]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to number value list".into()),
                Some(value) => Ok(vec![float(*value, &value_format.decimal_separator)?]),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to number value list".into())
                } else {
                    let vector: Vec<f64> = value
                        .iter()
                        .map(|val| {
                            val.f64(value_format)
                                .expect("Cannot convert value list to number list")
                        })
                        .collect();
                    Ok(vector)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to number value list".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty value list to number value list".into())
                    } else {
                        let vector: Vec<f64> = value
                            .iter()
                            .map(|val| {
                                val.f64(value_format)
                                    .expect("Cannot convert value list to number list")
                            })
                            .collect();
                        Ok(vector)
                    }
                }
            },
            Value::AreaValue(value) => {
                let values: Vec<f64> = value
                    .iter()
                    .flat_map(|inner_vec| {
                        inner_vec.iter().map(|val| {
                            val.f64(value_format)
                                .expect("Cannot convert empty value area to number value list")
                        })
                    })
                    .collect();

                if !values.is_empty() {
                    return Ok(values);
                }
                Err("Cannot convert empty value area to number value list".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    let values: Vec<f64> = value
                        .iter()
                        .flat_map(|inner_vec| {
                            inner_vec.iter().map(|val| {
                                val.f64(value_format)
                                    .expect("Cannot convert empty value area to number value list")
                            })
                        })
                        .collect();

                    if !values.is_empty() {
                        return Ok(values);
                    }
                }
                Err("Cannot convert empty value area to number value list".into())
            }
            Value::None => Err("Cannot convert none to number value list".into()),
            Value::ChronoDateTime(value) => Ok(vec![date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?]),
            Value::OptionChronoDateTime(value) => {
                if let Some(value) = value {
                    Ok(vec![date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?])
                } else {
                    Err("Cannot convert empty date time to number value list".into())
                }
            }
            Value::OptionTime(value) => {
                if let Some(value) = value {
                    Ok(vec![time_to_excel(value)?])
                } else {
                    Err("Cannot convert empty time to number value list".into())
                }
            }
            Value::Time(value) => Ok(vec![time_to_excel(value)?]),
        }
    }

    pub fn vec_value(&self) -> Result<Vec<Value>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![Value::F64(*value)]),
            Value::I32(value) => Ok(vec![Value::I32(*value)]),
            Value::String(value) => Ok(vec![Value::String(value.clone())]),
            Value::Bool(value) => Ok(vec![Value::Bool(*value)]),
            Value::OptionF64(value) => Ok(vec![Value::OptionF64(*value)]),
            Value::OptionI32(value) => Ok(vec![Value::OptionI32(*value)]),
            Value::OptionString(value) => Ok(vec![Value::OptionString(value.clone())]),
            Value::OptionBool(value) => Ok(vec![Value::OptionBool(*value)]),
            Value::VecValue(value) => Ok(value.clone()),
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert a none value list to value list".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty value list to number value list".into())
                    } else {
                        Ok(value.clone())
                    }
                }
            },
            Value::AreaValue(value) => {
                let values: Vec<Value> = value
                    .iter()
                    .flat_map(|inner_vec| inner_vec.iter().cloned())
                    .collect();

                if !values.is_empty() {
                    return Ok(values);
                }
                Err("Cannot convert empty value area to value list".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    let values: Vec<Value> = value
                        .iter()
                        .flat_map(|inner_vec| inner_vec.iter().cloned())
                        .collect();

                    if !values.is_empty() {
                        return Ok(values);
                    }
                }
                Err("Cannot convert empty value area to value list".into())
            }
            Value::None => Err("Cannot convert none to value list".into()),
            Value::ChronoDateTime(value) => Ok(vec![Value::ChronoDateTime(*value)]),
            Value::OptionChronoDateTime(value) => Ok(vec![Value::OptionChronoDateTime(*value)]),
            Value::OptionTime(value) => Ok(vec![Value::OptionTime(*value)]),
            Value::Time(value) => Ok(vec![Value::Time(*value)]),
        }
    }

    pub fn i32(&self, value_format: &ValueFormat) -> Result<i32, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => integer(*value, &value_format.decimal_separator),
            Value::I32(value) => Ok(*value),
            Value::String(value) => integer(value.as_str(), &value_format.decimal_separator),
            Value::Bool(value) => integer(*value, &value_format.decimal_separator),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert number none to whole number value".into()),
                Some(value) => integer(*value, &value_format.decimal_separator),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert whole number none to whole number value".into()),
                Some(value) => Ok(*value),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to whole number value".into()),
                Some(value) => integer(value.as_str(), &value_format.decimal_separator),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to whole number value".into()),
                Some(value) => integer(*value, &value_format.decimal_separator),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to whole number value".into())
                } else {
                    value[0].i32(value_format)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to whole number value".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty value list to whole number value".into())
                    } else {
                        value[0].i32(value_format)
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return value[0][0].i32(value_format);
                        }
                    }
                }
                Err("Cannot convert empty value area to whole number value".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return value[0][0].i32(value_format);
                            }
                        }
                    }
                }
                Err("Cannot convert empty value area to whole number value".into())
            }
            Value::None => Err("Cannot convert none to whole number value".into()),
            Value::ChronoDateTime(value) => {
                integer(date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?, &value_format.decimal_separator)
            }
            Value::OptionChronoDateTime(value) => {
                if let Some(value) = value {
                    integer(date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?, &value_format.decimal_separator)
                } else {
                    Err("Cannot convert empty date time to whole number value".into())
                }
            }
            Value::OptionTime(value) => {
                if let Some(value) = value {
                    integer(time_to_excel(value)?, &value_format.decimal_separator)
                } else {
                    Err("Cannot convert empty time to whole number value".into())
                }
            }
            Value::Time(value) => integer(time_to_excel(value)?, &value_format.decimal_separator),
        }
    }

    pub fn area_of_value(&self) -> Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![vec![Value::F64(*value)]]),
            Value::I32(value) => Ok(vec![vec![Value::I32(*value)]]),
            Value::String(value) => Ok(vec![vec![Value::String(value.to_string())]]),
            Value::Bool(value) => Ok(vec![vec![Value::Bool(*value)]]),
            Value::OptionF64(value) => match value {
                None => Ok(vec![vec![Value::None]]),
                Some(value) => Ok(vec![vec![Value::F64(*value)]]),
            },
            Value::OptionI32(value) => match value {
                None => Ok(vec![vec![Value::None]]),
                Some(value) => Ok(vec![vec![Value::I32(*value)]]),
            },
            Value::OptionString(value) => match value {
                None => Ok(vec![vec![Value::None]]),
                Some(value) => Ok(vec![vec![Value::String(value.to_string())]]),
            },
            Value::OptionBool(value) => match value {
                None => Ok(vec![vec![Value::None]]),
                Some(value) => Ok(vec![vec![Value::Bool(*value)]]),
            },
            Value::VecValue(vec) => Ok(vec![vec.clone()]),
            Value::OptionVecValue(value) => match value {
                None => Ok(vec![vec![Value::None]]),
                Some(value) => Ok(vec![value.clone()]),
            },
            Value::AreaValue(value) => Ok(value.clone()),
            Value::OptionAreaValue(value) => match value {
                None => Ok(vec![vec![Value::None]]),
                Some(value) => Ok(value.clone()),
            },
            Value::None => Ok(vec![vec![Value::None]]),
            Value::ChronoDateTime(value) => Ok(vec![vec![Value::ChronoDateTime(*value)]]),
            Value::OptionChronoDateTime(value) => match value {
                None => Ok(vec![vec![Value::None]]),
                Some(value) => Ok(vec![vec![Value::ChronoDateTime(*value)]]),
            },
            Value::OptionTime(value) => match value {
                None => Ok(vec![vec![Value::None]]),
                Some(value) => Ok(vec![vec![Value::Time(*value)]]),
            },
            Value::Time(value) => Ok(vec![vec![Value::Time(*value)]]),
        }
    }

    pub fn option_area_of_value(
        &self,
    ) -> Result<Option<Vec<Vec<Value>>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(Some(vec![vec![Value::F64(*value)]])),
            Value::I32(value) => Ok(Some(vec![vec![Value::I32(*value)]])),
            Value::String(value) => Ok(Some(vec![vec![Value::String(value.to_string())]])),
            Value::Bool(value) => Ok(Some(vec![vec![Value::Bool(*value)]])),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![vec![Value::F64(*value)]])),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![vec![Value::I32(*value)]])),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![vec![Value::String(value.to_string())]])),
            },
            Value::OptionBool(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![vec![Value::Bool(*value)]])),
            },
            Value::VecValue(vec) => Ok(Some(vec![vec.clone()])),
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![value.clone()])),
            },
            Value::AreaValue(value) => Ok(Some(value.clone())),
            Value::OptionAreaValue(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(value.clone())),
            },
            Value::None => Ok(None),
            Value::ChronoDateTime(value) => Ok(Some(vec![vec![Value::ChronoDateTime(*value)]])),
            Value::OptionChronoDateTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![vec![Value::ChronoDateTime(*value)]])),
            },
            Value::OptionTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![vec![Value::Time(*value)]])),
            },
            Value::Time(value) => Ok(Some(vec![vec![Value::Time(*value)]])),
        }
    }

    pub fn option_vec_of_value(&self) -> Result<Option<Vec<Value>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(Some(vec![Value::F64(*value)])),
            Value::I32(value) => Ok(Some(vec![Value::I32(*value)])),
            Value::String(value) => Ok(Some(vec![Value::String(value.to_string())])),
            Value::Bool(value) => Ok(Some(vec![Value::Bool(*value)])),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![Value::F64(*value)])),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![Value::I32(*value)])),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![Value::String(value.to_string())])),
            },
            Value::OptionBool(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![Value::Bool(*value)])),
            },
            Value::VecValue(vec) => Ok(Some(vec.clone())),
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(value.clone())),
            },
            Value::AreaValue(value) => {
                let value = value.iter().flat_map(|row| row.iter().cloned()).collect();
                Ok(Some(value))
            }
            Value::OptionAreaValue(value) => match value {
                None => Ok(None),
                Some(value) => {
                    let value = value.iter().flat_map(|row| row.iter().cloned()).collect();
                    Ok(Some(value))
                }
            },
            Value::None => Ok(None),
            Value::ChronoDateTime(value) => Ok(Some(vec![Value::ChronoDateTime(*value)])),
            Value::OptionChronoDateTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![Value::ChronoDateTime(*value)])),
            },
            Value::OptionTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![Value::Time(*value)])),
            },
            Value::Time(value) => Ok(Some(vec![Value::Time(*value)])),
        }
    }

    pub fn area_of_f64(
        &self,
        _strict_type_conversion: bool,
        value_format: &ValueFormat,
    ) -> Result<Vec<Vec<f64>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![vec![*value]]),
            Value::I32(_value) => Ok(vec![vec![self.f64(value_format)?]]),
            Value::String(_value) => Ok(vec![vec![self.f64(value_format)?]]),
            Value::Bool(_value) => Ok(vec![vec![self.f64(value_format)?]]),
            Value::OptionF64(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.f64(value_format)?]]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.f64(value_format)?]]),
            },
            Value::OptionString(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.f64(value_format)?]]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.f64(value_format)?]]),
            },
            Value::VecValue(value) => {
                let values = value
                    .iter()
                    .map(|val| val.f64(value_format).expect("Value must be a number"))
                    .collect();
                Ok(vec![values])
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => {
                    let values = value
                        .iter()
                        .map(|val| val.f64(value_format).expect("Value must be a number"))
                        .collect();
                    Ok(vec![values])
                }
            },
            Value::AreaValue(value) => Ok(value
                .iter()
                .map(|inner_vec| {
                    inner_vec
                        .iter()
                        .map(|val| val.f64(value_format).expect("Value must be a number"))
                        .collect()
                })
                .collect()),
            Value::OptionAreaValue(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(value
                    .iter()
                    .map(|inner_vec| {
                        inner_vec
                            .iter()
                            .map(|val| val.f64(value_format).expect("Value must be a number"))
                            .collect()
                    })
                    .collect()),
            },
            Value::None => Err("Value cannot be None".into()),
            Value::ChronoDateTime(_value) => Ok(vec![vec![self.f64(value_format)?]]),
            Value::OptionChronoDateTime(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.f64(value_format)?]]),
            },
            Value::OptionTime(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.f64(value_format)?]]),
            },
            Value::Time(_value) => Ok(vec![vec![self.f64(value_format)?]]),
        }
    }

    pub fn area_of_date_time(
        &self,
        _strict_type_conversion: bool,
        value_format: &ValueFormat,
    ) -> Result<Vec<Vec<DateTime<Utc>>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![vec![excel_to_date_time(*value, value_format.allow_lotus_1_2_3_1900_date_bug)?]]),
            Value::I32(value) => Ok(vec![vec![excel_to_date_time(float(
                *value,
                &value_format.decimal_separator,
            )?, value_format.allow_lotus_1_2_3_1900_date_bug)?]]),
            Value::String(value) => Ok(vec![vec![force_string_to_date_time(
                value,
                &value_format.decimal_separator,
                value_format.allow_lotus_1_2_3_1900_date_bug,
            )?]]),
            Value::Bool(_value) => Err("Cannot convert boolean to date time value".into()),
            Value::OptionF64(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(vec![vec![excel_to_date_time(*value, value_format.allow_lotus_1_2_3_1900_date_bug)?]]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(vec![vec![excel_to_date_time(float(
                    *value,
                    &value_format.decimal_separator,
                )?, value_format.allow_lotus_1_2_3_1900_date_bug)?]]),
            },
            Value::OptionString(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(vec![vec![force_string_to_date_time(
                    value,
                    &value_format.decimal_separator,
                    value_format.allow_lotus_1_2_3_1900_date_bug,
                )?]]),
            },
            Value::OptionBool(_value) => Err("Cannot convert boolean to date time value".into()),
            Value::VecValue(value) => {
                let values = value
                    .iter()
                    .map(|val| {
                        val.date_time(value_format)
                            .expect("Value must be a date time")
                    })
                    .collect();
                Ok(vec![values])
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => {
                    let values = value
                        .iter()
                        .map(|val| {
                            val.date_time(value_format)
                                .expect("Value must be a date time")
                        })
                        .collect();
                    Ok(vec![values])
                }
            },
            Value::AreaValue(value) => Ok(value
                .iter()
                .map(|inner_vec| {
                    inner_vec
                        .iter()
                        .map(|val| {
                            val.date_time(value_format)
                                .expect("Value must be a date time")
                        })
                        .collect()
                })
                .collect()),
            Value::OptionAreaValue(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(value
                    .iter()
                    .map(|inner_vec| {
                        inner_vec
                            .iter()
                            .map(|val| {
                                val.date_time(value_format)
                                    .expect("Value must be a date time")
                            })
                            .collect()
                    })
                    .collect()),
            },
            Value::None => Err("Value cannot be None".into()),
            Value::ChronoDateTime(value) => Ok(vec![vec![*value]]),
            Value::OptionChronoDateTime(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(vec![vec![*value]]),
            },
            Value::OptionTime(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(vec![vec![time_to_date_time(*value)?]]),
            },
            Value::Time(value) => Ok(vec![vec![time_to_date_time(*value)?]]),
        }
    }

    pub fn area_of_bool(
        &self,
        _strict_type_conversion: bool,
        value_format: &ValueFormat,
    ) -> Result<Vec<Vec<bool>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(_value) => Ok(vec![vec![self.bool(value_format)?]]),
            Value::I32(_value) => Ok(vec![vec![self.bool(value_format)?]]),
            Value::String(_value) => Ok(vec![vec![self.bool(value_format)?]]),
            Value::Bool(value) => Ok(vec![vec![*value]]),
            Value::OptionF64(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.bool(value_format)?]]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.bool(value_format)?]]),
            },
            Value::OptionString(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.bool(value_format)?]]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.bool(value_format)?]]),
            },
            Value::VecValue(value) => {
                let values = value
                    .iter()
                    .map(|val| val.bool(value_format).expect("Value must be a boolean"))
                    .collect();
                Ok(vec![values])
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => {
                    let values = value
                        .iter()
                        .map(|val| val.bool(value_format).expect("Value must be a boolean"))
                        .collect();
                    Ok(vec![values])
                }
            },
            Value::AreaValue(value) => Ok(value
                .iter()
                .map(|inner_vec| {
                    inner_vec
                        .iter()
                        .map(|val| val.bool(value_format).expect("Value must be a boolean"))
                        .collect()
                })
                .collect()),
            Value::OptionAreaValue(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(value
                    .iter()
                    .map(|inner_vec| {
                        inner_vec
                            .iter()
                            .map(|val| val.bool(value_format).expect("Value must be a boolean"))
                            .collect()
                    })
                    .collect()),
            },
            Value::None => Err("Value cannot be None".into()),
            Value::ChronoDateTime(_value) => Ok(vec![vec![self.bool(value_format)?]]),
            Value::OptionChronoDateTime(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.bool(value_format)?]]),
            },
            Value::OptionTime(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.bool(value_format)?]]),
            },
            Value::Time(_value) => Ok(vec![vec![self.bool(value_format)?]]),
        }
    }

    pub fn area_of_string(
        &self,
        _strict_type_conversion: bool,
        value_format: &ValueFormat,
    ) -> Result<Vec<Vec<String>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(_value) => Ok(vec![vec![self.string(value_format)?]]),
            Value::I32(_value) => Ok(vec![vec![self.string(value_format)?]]),
            Value::String(value) => Ok(vec![vec![value.to_owned()]]),
            Value::Bool(_value) => Ok(vec![vec![self.string(value_format)?]]),
            Value::OptionF64(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.string(value_format)?]]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.string(value_format)?]]),
            },
            Value::OptionString(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.string(value_format)?]]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.string(value_format)?]]),
            },
            Value::VecValue(value) => {
                let values = value
                    .iter()
                    .map(|val| val.string(value_format).expect("Value must be a string"))
                    .collect();
                Ok(vec![values])
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => {
                    let values = value
                        .iter()
                        .map(|val| val.string(value_format).expect("Value must be a string"))
                        .collect();
                    Ok(vec![values])
                }
            },
            Value::AreaValue(value) => Ok(value
                .iter()
                .map(|inner_vec| {
                    inner_vec
                        .iter()
                        .map(|val| val.string(value_format).expect("Value must be a string"))
                        .collect()
                })
                .collect()),
            Value::OptionAreaValue(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(value
                    .iter()
                    .map(|inner_vec| {
                        inner_vec
                            .iter()
                            .map(|val| val.string(value_format).expect("Value must be a string"))
                            .collect()
                    })
                    .collect()),
            },
            Value::None => Err("Value cannot be None".into()),
            Value::ChronoDateTime(_value) => Ok(vec![vec![self.string(value_format)?]]),
            Value::OptionChronoDateTime(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.string(value_format)?]]),
            },
            Value::OptionTime(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.string(value_format)?]]),
            },
            Value::Time(_value) => Ok(vec![vec![self.string(value_format)?]]),
        }
    }

    pub fn option_area_of_f64(
        &self,
        _strict_type_conversion: bool,
        value_format: &ValueFormat,
    ) -> Result<Option<Vec<Vec<f64>>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(Some(vec![vec![*value]])),
            Value::I32(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
            Value::String(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
            Value::Bool(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
            },
            Value::OptionBool(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
            },
            Value::VecValue(value) => {
                let values = value
                    .iter()
                    .map(|val| val.f64(value_format).expect("Value must be a number"))
                    .collect();
                Ok(Some(vec![values]))
            }
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => {
                    let values = value
                        .iter()
                        .map(|val| val.f64(value_format).expect("Value must be a number"))
                        .collect();
                    Ok(Some(vec![values]))
                }
            },
            Value::AreaValue(value) => Ok(Some(
                value
                    .iter()
                    .map(|inner_vec| {
                        inner_vec
                            .iter()
                            .map(|val| val.f64(value_format).expect("Value must be a number"))
                            .collect()
                    })
                    .collect(),
            )),
            Value::OptionAreaValue(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(
                    value
                        .iter()
                        .map(|inner_vec| {
                            inner_vec
                                .iter()
                                .map(|val| val.f64(value_format).expect("Value must be a number"))
                                .collect()
                        })
                        .collect(),
                )),
            },
            Value::None => Ok(None),
            Value::ChronoDateTime(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
            Value::OptionChronoDateTime(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
            },
            Value::OptionTime(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
            },
            Value::Time(_value) => Ok(Some(vec![vec![self.f64(value_format)?]])),
        }
    }

    pub fn area_of_i32(
        &self,
        _strict_type_conversion: bool,
        value_format: &ValueFormat,
    ) -> Result<Vec<Vec<i32>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(_value) => Ok(vec![vec![self.i32(value_format)?]]),
            Value::I32(value) => Ok(vec![vec![*value]]),
            Value::String(_value) => Ok(vec![vec![self.i32(value_format)?]]),
            Value::Bool(_value) => Ok(vec![vec![self.i32(value_format)?]]),
            Value::OptionF64(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.i32(value_format)?]]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(vec![vec![*value]]),
            },
            Value::OptionString(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.i32(value_format)?]]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.i32(value_format)?]]),
            },
            Value::VecValue(value) => {
                let values = value
                    .iter()
                    .map(|val| val.i32(value_format).expect("Value must be an integer"))
                    .collect();
                Ok(vec![values])
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => {
                    let values = value
                        .iter()
                        .map(|val| val.i32(value_format).expect("Value must be an integer"))
                        .collect();
                    Ok(vec![values])
                }
            },
            Value::AreaValue(value) => Ok(value
                .iter()
                .map(|inner_vec| {
                    inner_vec
                        .iter()
                        .map(|val| val.i32(value_format).expect("Value must be an integer"))
                        .collect()
                })
                .collect()),
            Value::OptionAreaValue(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(value) => Ok(value
                    .iter()
                    .map(|inner_vec| {
                        inner_vec
                            .iter()
                            .map(|val| val.i32(value_format).expect("Value must be an integer"))
                            .collect()
                    })
                    .collect()),
            },
            Value::None => Err("Value cannot be None".into()),
            Value::ChronoDateTime(_value) => Ok(vec![vec![self.i32(value_format)?]]),
            Value::OptionChronoDateTime(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.i32(value_format)?]]),
            },
            Value::OptionTime(value) => match value {
                None => Err("Value cannot be None".into()),
                Some(_value) => Ok(vec![vec![self.i32(value_format)?]]),
            },
            Value::Time(_value) => Ok(vec![vec![self.i32(value_format)?]]),
        }
    }

    pub fn option_area_of_i32(
        &self,
        _strict_type_conversion: bool,
        value_format: &ValueFormat,
    ) -> Result<Option<Vec<Vec<i32>>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(_value) => Ok(Some(vec![vec![self.i32(value_format)?]])),
            Value::I32(value) => Ok(Some(vec![vec![*value]])),
            Value::String(_value) => Ok(Some(vec![vec![self.i32(value_format)?]])),
            Value::Bool(_value) => Ok(Some(vec![vec![self.i32(value_format)?]])),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.i32(value_format)?]])),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![vec![*value]])),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.i32(value_format)?]])),
            },
            Value::OptionBool(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.i32(value_format)?]])),
            },
            Value::VecValue(value) => {
                let values = value
                    .iter()
                    .map(|val| val.i32(value_format).expect("Value must be an integer"))
                    .collect();
                Ok(Some(vec![values]))
            }
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => {
                    let values = value
                        .iter()
                        .map(|val| val.i32(value_format).expect("Value must be an integer"))
                        .collect();
                    Ok(Some(vec![values]))
                }
            },
            Value::AreaValue(value) => Ok(Some(
                value
                    .iter()
                    .map(|inner_vec| {
                        inner_vec
                            .iter()
                            .map(|val| val.i32(value_format).expect("Value must be an integer"))
                            .collect()
                    })
                    .collect(),
            )),
            Value::OptionAreaValue(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(
                    value
                        .iter()
                        .map(|inner_vec| {
                            inner_vec
                                .iter()
                                .map(|val| val.i32(value_format).expect("Value must be an integer"))
                                .collect()
                        })
                        .collect(),
                )),
            },
            Value::None => Ok(None),
            Value::ChronoDateTime(_value) => Ok(Some(vec![vec![self.i32(value_format)?]])),
            Value::OptionChronoDateTime(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.i32(value_format)?]])),
            },
            Value::OptionTime(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.i32(value_format)?]])),
            },
            Value::Time(_value) => Ok(Some(vec![vec![self.i32(value_format)?]])),
        }
    }

    pub fn option_area_of_bool(
        &self,
        _strict_type_conversion: bool,
        value_format: &ValueFormat,
    ) -> Result<Option<Vec<Vec<bool>>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(_value) => Ok(Some(vec![vec![self.bool(value_format)?]])),
            Value::I32(_value) => Ok(Some(vec![vec![self.bool(value_format)?]])),
            Value::String(_value) => Ok(Some(vec![vec![self.bool(value_format)?]])),
            Value::Bool(value) => Ok(Some(vec![vec![*value]])),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.bool(value_format)?]])),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.bool(value_format)?]])),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.bool(value_format)?]])),
            },
            Value::OptionBool(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(vec![vec![*value]])),
            },
            Value::VecValue(value) => {
                let values = value
                    .iter()
                    .map(|val| val.bool(value_format).expect("Value must be a boolean"))
                    .collect();
                Ok(Some(vec![values]))
            }
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => {
                    let values = value
                        .iter()
                        .map(|val| val.bool(value_format).expect("Value must be a boolean"))
                        .collect();
                    Ok(Some(vec![values]))
                }
            },
            Value::AreaValue(value) => Ok(Some(
                value
                    .iter()
                    .map(|inner_vec| {
                        inner_vec
                            .iter()
                            .map(|val| val.bool(value_format).expect("Value must be a boolean"))
                            .collect()
                    })
                    .collect(),
            )),
            Value::OptionAreaValue(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(
                    value
                        .iter()
                        .map(|inner_vec| {
                            inner_vec
                                .iter()
                                .map(|val| val.bool(value_format).expect("Value must be a boolean"))
                                .collect()
                        })
                        .collect(),
                )),
            },
            Value::None => Ok(None),
            Value::ChronoDateTime(_value) => Ok(Some(vec![vec![self.bool(value_format)?]])),
            Value::OptionChronoDateTime(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.bool(value_format)?]])),
            },
            Value::OptionTime(value) => match value {
                None => Ok(None),
                Some(_value) => Ok(Some(vec![vec![self.bool(value_format)?]])),
            },
            Value::Time(_value) => Ok(Some(vec![vec![self.bool(value_format)?]])),
        }
    }

    pub fn option_i32(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Option<i32>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(Some(integer(*value, &value_format.decimal_separator)?)),
            Value::I32(value) => Ok(Some(*value)),
            Value::String(value) => Ok(Some(integer(
                value.as_str(),
                &value_format.decimal_separator,
            )?)),
            Value::Bool(value) => Ok(Some(integer(*value, &value_format.decimal_separator)?)),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(integer(*value, &value_format.decimal_separator)?)),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(*value)),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(integer(
                    value.as_str(),
                    &value_format.decimal_separator,
                )?)),
            },
            Value::OptionBool(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(integer(*value, &value_format.decimal_separator)?)),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Ok(None)
                } else {
                    value[0].option_i32(value_format)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => {
                    if value.is_empty() {
                        Ok(None)
                    } else {
                        value[0].option_i32(value_format)
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return value[0][0].option_i32(value_format);
                        }
                    }
                }
                Ok(None)
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return value[0][0].option_i32(value_format);
                            }
                        }
                    }
                }

                Ok(None)
            }
            Value::None => Ok(None),
            Value::ChronoDateTime(value) => Ok(Some(integer(
                date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?,
                &value_format.decimal_separator,
            )?)),
            Value::OptionChronoDateTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(integer(
                    date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?,
                    &value_format.decimal_separator,
                )?)),
            },
            Value::OptionTime(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(integer(
                    time_to_excel(value)?,
                    &value_format.decimal_separator,
                )?)),
            },
            Value::Time(value) => Ok(Some(integer(
                time_to_excel(value)?,
                &value_format.decimal_separator,
            )?)),
        }
    }

    pub fn vec_i32(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Vec<i32>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![integer(*value, &value_format.decimal_separator)?]),
            Value::I32(value) => Ok(vec![*value]),
            Value::String(value) => Ok(vec![integer(
                value.as_str(),
                &value_format.decimal_separator,
            )?]),
            Value::Bool(value) => Ok(vec![integer(*value, &value_format.decimal_separator)?]),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert number none to whole number list".into()),
                Some(value) => Ok(vec![integer(*value, &value_format.decimal_separator)?]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert whole number none to whole number list".into()),
                Some(value) => Ok(vec![*value]),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to whole number value list".into()),
                Some(value) => Ok(vec![integer(
                    value.as_str(),
                    &value_format.decimal_separator,
                )?]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to whole number value list".into()),
                Some(value) => Ok(vec![integer(*value, &value_format.decimal_separator)?]),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to whole number value list".into())
                } else {
                    let vector: Vec<i32> = value
                        .iter()
                        .map(|val| {
                            val.i32(value_format)
                                .expect("Cannot convert empty value list to whole number list")
                        })
                        .collect();
                    Ok(vector)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to number value list".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty value list to whole number value list".into())
                    } else {
                        let vector: Vec<i32> = value
                            .iter()
                            .map(|val| {
                                val.i32(value_format)
                                    .expect("Cannot convert value list to whole number list")
                            })
                            .collect();
                        Ok(vector)
                    }
                }
            },
            Value::AreaValue(value) => {
                let values: Vec<i32> = value
                    .iter()
                    .flat_map(|inner_vec| {
                        inner_vec.iter().map(|val| {
                            val.i32(value_format).expect(
                                "Cannot convert empty value area to whole number value list",
                            )
                        })
                    })
                    .collect();

                if !values.is_empty() {
                    return Ok(values);
                }
                Err("Cannot convert empty value area to whole number value list".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    let values: Vec<i32> = value
                        .iter()
                        .flat_map(|inner_vec| {
                            inner_vec.iter().map(|val| {
                                val.i32(value_format).expect(
                                    "Cannot convert empty value area to whole number value list",
                                )
                            })
                        })
                        .collect();

                    if !values.is_empty() {
                        return Ok(values);
                    }
                }
                Err("Cannot convert empty value area to whole number value list".into())
            }
            Value::None => Err("Cannot convert none to whole number value list".into()),
            Value::ChronoDateTime(value) => Ok(vec![integer(
                date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?,
                &value_format.decimal_separator,
            )?]),
            Value::OptionChronoDateTime(value) => {
                if let Some(value) = value {
                    Ok(vec![integer(
                        date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?,
                        &value_format.decimal_separator,
                    )?])
                } else {
                    Err("Cannot convert empty date time to whole number value list".into())
                }
            }
            Value::OptionTime(value) => {
                if let Some(value) = value {
                    Ok(vec![integer(
                        time_to_excel(value)?,
                        &value_format.decimal_separator,
                    )?])
                } else {
                    Err("Cannot convert empty time to whole number value list".into())
                }
            }
            Value::Time(value) => Ok(vec![integer(
                time_to_excel(value)?,
                &value_format.decimal_separator,
            )?]),
        }
    }

    pub fn string(
        &self,
        value_format: &ValueFormat,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => float_to_string(*value, &value_format.decimal_separator),
            Value::I32(value) => Ok(value.to_string()),
            Value::String(value) => Ok(value.to_string()),
            Value::Bool(value) => Ok(if *value { "TRUE" } else { "FALSE" }.to_string()),
            Value::OptionF64(value) => {
                // TODO: PERHAPS THIS SHOULD RETURN AN EMPTY STRING
                match value {
                    None => Err("Cannot convert number none to string value".into()),
                    Some(value) => Ok(float_to_string(
                        value.to_string(),
                        &value_format.decimal_separator,
                    )?),
                }
            }
            Value::OptionI32(value) => {
                // TODO: PERHAPS THIS SHOULD RETURN AN EMPTY STRING
                match value {
                    None => Err("Cannot convert whole number none to string value".into()),
                    Some(value) => Ok(value.to_string()),
                }
            }
            Value::OptionString(value) => {
                // TODO: PERHAPS THIS SHOULD RETURN AN EMPTY STRING
                match value {
                    None => Err("Cannot convert string none to string value".into()),
                    Some(value) => Ok(value.to_string()),
                }
            }
            Value::OptionBool(value) => {
                // TODO: PERHAPS THIS SHOULD RETURN AN EMPTY STRING
                match value {
                    None => Err("Cannot convert boolean none to string value".into()),
                    Some(value) => Ok(value.to_string().to_lowercase()),
                }
            }
            Value::VecValue(value) => {
                // TODO: PERHAPS THIS SHOULD RETURN AN EMPTY STRING
                if value.is_empty() {
                    Err("Cannot convert empty value list to string value".into())
                } else {
                    value[0].string(value_format)
                }
            }
            Value::OptionVecValue(value) => {
                match value {
                    // TODO: PERHAPS THIS SHOULD RETURN AN EMPTY STRING
                    None => Err("Cannot convert none value list to string value".into()),
                    Some(value) => {
                        if value.is_empty() {
                            Err("Cannot convert empty value list to string value".into())
                        } else {
                            value[0].string(value_format)
                        }
                    }
                }
            }
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return value[0][0].string(value_format);
                        }
                    }
                }
                // TODO: PERHAPS THIS SHOULD RETURN AN EMPTY STRING
                Err("Cannot convert empty value area to string value".into())
            }
            Value::OptionAreaValue(value) => {
                // TODO: PERHAPS THIS SHOULD RETURN AN EMPTY STRING
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return value[0][0].string(value_format);
                            }
                        }
                    }
                }
                Err("Cannot convert empty value area to string value".into())
            }
            Value::None => Err("Cannot convert none to string value".into()),
            Value::ChronoDateTime(value) => Ok(date_time_to_iso(value)),
            Value::OptionChronoDateTime(value) => {
                if let Some(value) = value {
                    Ok(date_time_to_iso(value))
                } else {
                    Err("Cannot convert emoty date time to string value".into())
                }
            }
            Value::OptionTime(value) => {
                if let Some(value) = value {
                    Ok(time_to_iso(value))
                } else {
                    Err("Cannot convert empty time to string value".into())
                }
            }
            Value::Time(value) => Ok(time_to_iso(value)),
        }
    }

    pub fn string_with_excel_date(
        &self,
        value_format: &ValueFormat,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        if self.is_datetime() {
            match self {
                Value::OptionChronoDateTime(value) => {
                    return if let Some(value) = value {
                        Ok(date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?.to_string())
                    } else {
                        Err("Cannot convert an empty datetime to a string value".into())
                    };
                }
                Value::ChronoDateTime(value) => {
                    return Ok(date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?.to_string());
                }
                _ => {}
            }
        } else if self.is_time() {
            match self {
                Value::OptionTime(value) => {
                    return if let Some(value) = value {
                        Ok(time_to_excel(value)?.to_string())
                    } else {
                        Err("Cannot convert an empty time to a string value".into())
                    };
                }
                Value::Time(value) => {
                    return Ok(time_to_excel(value)?.to_string());
                }
                _ => {}
            }
        }
        self.string(value_format)
    }

    pub fn option_string(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(Some(float_to_string(
                *value,
                &value_format.decimal_separator,
            )?)),
            Value::I32(value) => Ok(Some(value.to_string())),
            Value::String(value) => Ok(Some(value.to_string())),
            Value::Bool(value) => Ok(Some(if *value { "TRUE" } else { "FALSE" }.to_string())),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(float_to_string(
                    value.to_string(),
                    &value_format.decimal_separator,
                )?)),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(value.to_string())),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(value.to_string())),
            },
            Value::OptionBool(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(value.to_string().to_lowercase())),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Ok(None)
                } else {
                    value[0].option_string(value_format)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => {
                    if value.is_empty() {
                        Ok(None)
                    } else {
                        value[0].option_string(value_format)
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return value[0][0].option_string(value_format);
                        }
                    }
                }
                Ok(None)
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return value[0][0].option_string(value_format);
                            }
                        }
                    }
                }

                Ok(None)
            }
            Value::None => Ok(None),
            Value::ChronoDateTime(value) => Ok(Some(date_time_to_iso(value))),
            Value::OptionChronoDateTime(value) => {
                if let Some(value) = value {
                    Ok(Some(date_time_to_iso(value)))
                } else {
                    Ok(None)
                }
            }
            Value::OptionTime(value) => {
                if let Some(value) = value {
                    Ok(Some(time_to_iso(value)))
                } else {
                    Ok(None)
                }
            }
            Value::Time(value) => Ok(Some(time_to_iso(value))),
        }
    }

    pub fn vec_string(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![float_to_string(
                *value,
                &value_format.decimal_separator,
            )?]),
            Value::I32(value) => Ok(vec![value.to_string()]),
            Value::String(value) => Ok(vec![value.to_string()]),
            Value::Bool(value) => Ok(vec![if *value { "TRUE" } else { "FALSE" }.to_string()]),
            Value::OptionF64(value) => match value {
                None => Ok(vec![String::new()]),
                Some(value) => Ok(vec![float_to_string(
                    *value,
                    &value_format.decimal_separator,
                )?]),
            },
            Value::OptionI32(value) => match value {
                None => Ok(vec![String::new()]),
                Some(value) => Ok(vec![value.to_string()]),
            },
            Value::OptionString(value) => match value {
                None => Ok(vec![String::new()]),
                Some(value) => Ok(vec![value.to_string()]),
            },
            Value::OptionBool(value) => match value {
                None => Ok(vec![String::new()]),
                Some(value) => Ok(vec![value.to_string().to_lowercase()]),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Ok(vec![String::new()])
                } else {
                    let vector: Vec<String> = value
                        .iter()
                        .map(|val| val.string(value_format).unwrap_or_default())
                        .collect();
                    Ok(vector)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Ok(vec![String::new()]),
                Some(value) => {
                    if value.is_empty() {
                        Ok(vec![String::new()])
                    } else {
                        let vector: Vec<String> = value
                            .iter()
                            .map(|val| val.string(value_format).unwrap_or_default())
                            .collect();
                        Ok(vector)
                    }
                }
            },
            Value::AreaValue(value) => {
                let values: Vec<String> = value
                    .iter()
                    .flat_map(|inner_vec| {
                        inner_vec
                            .iter()
                            .map(|val| val.string(value_format).unwrap_or_default())
                    })
                    .collect();

                if !values.is_empty() {
                    Ok(values)
                } else {
                    Ok(vec![String::new()])
                }
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    let values: Vec<String> = value
                        .iter()
                        .flat_map(|inner_vec| {
                            inner_vec
                                .iter()
                                .map(|val| val.string(value_format).unwrap_or_default())
                        })
                        .collect();

                    if !values.is_empty() {
                        return Ok(values);
                    }
                }
                Ok(vec![String::new()])
            }
            Value::None => Ok(vec![String::new()]),
            Value::ChronoDateTime(value) => Ok(vec![date_time_to_iso(value)]),
            Value::OptionChronoDateTime(value) => {
                if let Some(value) = value {
                    Ok(vec![date_time_to_iso(value)])
                } else {
                    Ok(vec![String::new()])
                }
            }
            Value::OptionTime(value) => {
                if let Some(value) = value {
                    Ok(vec![time_to_iso(value)])
                } else {
                    Ok(vec![String::new()])
                }
            }
            Value::Time(value) => Ok(vec![time_to_iso(value)]),
        }
    }

    pub fn raw_string(&self) -> String {
        match self {
            Value::F64(value) => value.to_string(),
            Value::I32(value) => value.to_string(),
            Value::String(value) => value.to_string(),
            Value::Bool(value) => if *value { "TRUE" } else { "FALSE" }.to_string(),
            Value::OptionF64(value) => match value {
                Some(value) => value.to_string(),
                None => "NONE".to_string(),
            },
            Value::OptionI32(value) => match value {
                Some(value) => value.to_string(),
                None => "NONE".to_string(),
            },
            Value::OptionString(value) => match value {
                Some(value) => value.to_string(),
                None => "NONE".to_string(),
            },
            Value::OptionBool(value) => match value {
                Some(value) => value.to_string().to_lowercase(),
                None => "NONE".to_string(),
            },
            Value::VecValue(value) => {
                let list: Vec<String> = value.iter().map(|val| val.raw_string()).collect();
                format!("[{}]", list.join(","))
            }
            Value::OptionVecValue(value) => match value {
                Some(value) => {
                    let list: Vec<String> = value.iter().map(|val| val.raw_string()).collect();
                    format!("[{}]", list.join(","))
                }
                None => "NONE".to_string(),
            },
            Value::AreaValue(value) => {
                let values: Vec<String> = value
                    .iter()
                    .flat_map(|inner_vec| inner_vec.iter().map(|val| val.raw_string()))
                    .collect();

                if !values.is_empty() {
                    return format!("[{}]", values.join(","));
                }

                "NONE".to_string()
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    let values: Vec<String> = value
                        .iter()
                        .flat_map(|inner_vec| inner_vec.iter().map(|val| val.raw_string()))
                        .collect();

                    if !values.is_empty() {
                        return format!("[{}]", values.join(","));
                    }
                }
                "NONE".to_string()
            }
            Value::None => "NONE".to_string(),
            Value::ChronoDateTime(value) => value.to_string(),
            Value::OptionChronoDateTime(value) => {
                if let Some(value) = value {
                    value.to_string()
                } else {
                    "NONE".to_string()
                }
            }
            Value::OptionTime(value) => {
                if let Some(value) = value {
                    time_to_iso(value)
                } else {
                    "NONE".to_string()
                }
            }
            Value::Time(value) => time_to_iso(value),
        }
    }

    pub fn bool(&self, value_format: &ValueFormat) -> Result<bool, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => boolean(*value, &value_format.decimal_separator),
            Value::I32(value) => boolean(*value, &value_format.decimal_separator),
            Value::String(value) => boolean(value.as_str(), &value_format.decimal_separator),
            Value::Bool(value) => Ok(*value),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert number none to boolean value".into()),
                Some(value) => boolean(*value, &value_format.decimal_separator),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert whole number none to boolean value".into()),
                Some(value) => boolean(*value, &value_format.decimal_separator),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to boolean value".into()),
                Some(value) => boolean(value.as_str(), &value_format.decimal_separator),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to boolean value".into()),
                Some(value) => Ok(*value),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to boolean value".into())
                } else {
                    value[0].bool(value_format)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to boolean value".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty value list to boolean value".into())
                    } else {
                        value[0].bool(value_format)
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return value[0][0].bool(value_format);
                        }
                    }
                }
                Err("Cannot convert empty value area to boolean value".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return value[0][0].bool(value_format);
                            }
                        }
                    }
                }
                Err("Cannot convert empty value area to boolean value".into())
            }
            Value::None => Err("Cannot convert none to boolean value".into()),
            Value::ChronoDateTime(_value) => {
                Err("Cannot convert date time to boolean value".into())
            }
            Value::OptionChronoDateTime(_value) => {
                Err("Cannot convert date time to boolean value".into())
            }
            Value::OptionTime(_) => Err("Cannot convert time to boolean value".into()),
            Value::Time(_) => Err("Cannot convert time to boolean value".into()),
        }
    }

    pub fn option_bool(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Option<bool>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(Some(boolean(*value, &value_format.decimal_separator)?)),
            Value::I32(value) => Ok(Some(boolean(*value, &value_format.decimal_separator)?)),
            Value::String(value) => Ok(Some(boolean(
                value.as_str(),
                &value_format.decimal_separator,
            )?)),
            Value::Bool(value) => Ok(Some(*value)),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(boolean(*value, &value_format.decimal_separator)?)),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(boolean(*value, &value_format.decimal_separator)?)),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(boolean(
                    value.as_str(),
                    &value_format.decimal_separator,
                )?)),
            },
            Value::OptionBool(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(*value)),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Ok(None)
                } else {
                    value[0].option_bool(value_format)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => {
                    if value.is_empty() {
                        Ok(None)
                    } else {
                        value[0].option_bool(value_format)
                    }
                }
            },
            Value::AreaValue(value) => {
                if !value.is_empty() {
                    if let Some(inside) = value.first() {
                        if !inside.is_empty() {
                            return value[0][0].option_bool(value_format);
                        }
                    }
                }
                Ok(None)
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    if !value.is_empty() {
                        if let Some(inside) = value.first() {
                            if !inside.is_empty() {
                                return value[0][0].option_bool(value_format);
                            }
                        }
                    }
                }

                Ok(None)
            }
            Value::None => Ok(None),
            Value::ChronoDateTime(_value) => {
                Err("Cannot convert date time to boolean value".into())
            }
            Value::OptionChronoDateTime(value) => {
                if let Some(_value) = value {
                    Err("Cannot convert date time to boolean value".into())
                } else {
                    Ok(None)
                }
            }
            Value::OptionTime(value) => {
                if let Some(_value) = value {
                    Err("Cannot convert time to boolean value".into())
                } else {
                    Ok(None)
                }
            }
            Value::Time(_) => Err("Cannot convert time to boolean value".into()),
        }
    }

    pub fn option_value(&self) -> Result<Option<Value>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(Some(Value::F64(*value))),
            Value::I32(value) => Ok(Some(Value::I32(*value))),
            Value::String(value) => Ok(Some(Value::String(value.to_string()))),
            Value::Bool(value) => Ok(Some(Value::Bool(*value))),
            Value::OptionF64(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(Value::F64(*value))),
            },
            Value::OptionI32(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(Value::I32(*value))),
            },
            Value::OptionString(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(Value::String(value.to_string()))),
            },
            Value::OptionBool(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(Value::Bool(*value))),
            },
            Value::VecValue(value) => Ok(Some(Value::VecValue(value.clone()))),
            Value::OptionVecValue(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(Value::VecValue(value.clone()))),
            },
            Value::AreaValue(value) => Ok(Some(Value::AreaValue(value.clone()))),
            Value::OptionAreaValue(value) => match value {
                None => Ok(None),
                Some(value) => Ok(Some(Value::AreaValue(value.clone()))),
            },
            Value::None => Ok(None),
            Value::ChronoDateTime(value) => Ok(Some(Value::ChronoDateTime(*value))),
            Value::OptionChronoDateTime(value) => {
                if let Some(value) = value {
                    Ok(Some(Value::ChronoDateTime(*value)))
                } else {
                    Ok(None)
                }
            }
            Value::OptionTime(value) => {
                if let Some(value) = value {
                    Ok(Some(Value::Time(*value)))
                } else {
                    Ok(None)
                }
            }
            Value::Time(value) => Ok(Some(Value::Time(*value))),
        }
    }

    pub fn vec_bool(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Vec<bool>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![boolean(*value, &value_format.decimal_separator)?]),
            Value::I32(value) => Ok(vec![boolean(*value, &value_format.decimal_separator)?]),
            Value::String(value) => Ok(vec![boolean(
                value.as_str(),
                &value_format.decimal_separator,
            )?]),
            Value::Bool(value) => Ok(vec![*value]),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert number none to boolean value list".into()),
                Some(value) => Ok(vec![boolean(*value, &value_format.decimal_separator)?]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert whole number none to boolean value list".into()),
                Some(value) => Ok(vec![boolean(*value, &value_format.decimal_separator)?]),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to boolean value list".into()),
                Some(value) => Ok(vec![boolean(
                    value.as_str(),
                    &value_format.decimal_separator,
                )?]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to boolean value list".into()),
                Some(value) => Ok(vec![*value]),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to boolean list".into())
                } else {
                    let vector: Vec<bool> = value
                        .iter()
                        .map(|val| {
                            val.bool(value_format)
                                .expect("Cannot convert empty value list to boolean list")
                        })
                        .collect();
                    Ok(vector)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to boolean list".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty value list to boolean value list".into())
                    } else {
                        let vector: Vec<bool> = value
                            .iter()
                            .map(|val| {
                                val.bool(value_format)
                                    .expect("Cannot convert value list to boolean list")
                            })
                            .collect();
                        Ok(vector)
                    }
                }
            },
            Value::AreaValue(value) => {
                let values: Vec<bool> = value
                    .iter()
                    .flat_map(|inner_vec| {
                        inner_vec.iter().map(|val| {
                            val.bool(value_format)
                                .expect("Cannot convert empty value area to boolean value list")
                        })
                    })
                    .collect();

                if !values.is_empty() {
                    return Ok(values);
                }
                Err("Cannot convert empty value area to boolean value list".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    let values: Vec<bool> = value
                        .iter()
                        .flat_map(|inner_vec| {
                            inner_vec.iter().map(|val| {
                                val.bool(value_format)
                                    .expect("Cannot convert empty value area to boolean value list")
                            })
                        })
                        .collect();

                    if !values.is_empty() {
                        return Ok(values);
                    }
                }
                Err("Cannot convert empty value area to boolean value list".into())
            }
            Value::None => Err("Cannot convert none to boolean value list".into()),
            Value::ChronoDateTime(_value) => {
                Err("Cannot convert date time to boolean value list".into())
            }
            Value::OptionChronoDateTime(_value) => {
                Err("Cannot convert date time to boolean value list".into())
            }
            Value::OptionTime(_) => Err("Cannot convert time to boolean value list".into()),
            Value::Time(_) => Err("Cannot convert time to boolean value list".into()),
        }
    }

    pub fn area_string(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Vec<Vec<String>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![vec![float_to_string(
                *value,
                &value_format.decimal_separator,
            )?]]),
            Value::I32(value) => Ok(vec![vec![value.to_string()]]),
            Value::String(value) => Ok(vec![vec![value.to_string()]]),
            Value::Bool(value) => Ok(vec![vec![if *value { "TRUE" } else { "FALSE" }.to_string()]]),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert number none to string value area".into()),
                Some(value) => Ok(vec![vec![float_to_string(
                    value.to_string(),
                    &value_format.decimal_separator,
                )?]]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert whole number none to string value area".into()),
                Some(value) => Ok(vec![vec![value.to_string()]]),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to string value array".into()),
                Some(value) => Ok(vec![vec![value.to_string()]]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to string value area".into()),
                Some(value) => Ok(vec![vec![value.to_string().to_lowercase()]]),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to string value area".into())
                } else {
                    let vector: Vec<String> = value
                        .iter()
                        .map(|val| {
                            val.string(value_format)
                                .expect("Cannot convert empty value list to string list")
                        })
                        .collect();
                    Ok(vec![vector])
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to string area".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty value list to string value area".into())
                    } else {
                        let vector: Vec<String> = value
                            .iter()
                            .map(|val| {
                                val.string(value_format)
                                    .expect("Cannot convert value list to string list")
                            })
                            .collect();
                        Ok(vec![vector])
                    }
                }
            },
            Value::AreaValue(value) => {
                let values: Vec<Vec<String>> = value
                    .iter()
                    .map(|values| {
                        values
                            .iter()
                            .map(|val| match val {
                                Value::None => String::new(),
                                other => other.string(value_format)
                                    .expect("Cannot convert value area to string value area"),
                            })
                            .collect()
                    })
                    .collect();

                if !values.is_empty() {
                    return Ok(values);
                }
                Err("Cannot convert empty value area to string value area".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    let values: Vec<Vec<String>> = value
                        .iter()
                        .map(|values| {
                            values
                                .iter()
                                .map(|val| match val {
                                    Value::None => String::new(),
                                    other => other.string(value_format).expect(
                                        "Cannot convert value area to string value area",
                                    ),
                                })
                                .collect()
                        })
                        .collect();

                    if !values.is_empty() {
                        return Ok(values);
                    }
                }
                Err("Cannot convert empty value area to string value area".into())
            }
            Value::None => Err("Cannot convert none to string value area".into()),
            Value::ChronoDateTime(value) => Ok(vec![vec![date_time_to_iso(value)]]),
            Value::OptionChronoDateTime(value) => match value {
                None => Err("Cannot convert datetime none to string value area".into()),
                Some(value) => Ok(vec![vec![date_time_to_iso(value)]]),
            },
            Value::OptionTime(value) => match value {
                None => Err("Cannot convert time none to string value area".into()),
                Some(value) => Ok(vec![vec![time_to_iso(value)]]),
            },
            Value::Time(value) => Ok(vec![vec![time_to_iso(value)]]),
        }
    }
    pub fn area_f64(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Vec<Vec<f64>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![vec![*value]]),
            Value::I32(value) => Ok(vec![vec![float(*value, &value_format.decimal_separator)?]]),
            Value::String(value) => Ok(vec![vec![float(
                value.as_str(),
                &value_format.decimal_separator,
            )?]]),
            Value::Bool(value) => Ok(vec![vec![float(*value, &value_format.decimal_separator)?]]),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert float none to number value area".into()),
                Some(value) => Ok(vec![vec![*value]]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert integer none to number value area".into()),
                Some(value) => Ok(vec![vec![float(*value, &value_format.decimal_separator)?]]),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to number value area".into()),
                Some(value) => Ok(vec![vec![float(
                    value.as_str(),
                    &value_format.decimal_separator,
                )?]]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to number value area".into()),
                Some(value) => Ok(vec![vec![float(*value, &value_format.decimal_separator)?]]),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to number value area".into())
                } else {
                    let vector: Vec<Vec<f64>> = value
                        .iter()
                        .map(|val| {
                            vec![val
                                .f64(value_format)
                                .expect("Cannot convert value list to number area")]
                        })
                        .collect();
                    Ok(vector)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to number value area".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty value list to number value area".into())
                    } else {
                        let vector: Vec<Vec<f64>> = value
                            .iter()
                            .map(|val| {
                                vec![val
                                    .f64(value_format)
                                    .expect("Cannot convert value list to number list")]
                            })
                            .collect();
                        Ok(vector)
                    }
                }
            },
            Value::AreaValue(value) => {
                let values: Vec<Vec<f64>> = value
                    .iter()
                    .map(|values| {
                        values
                            .iter()
                            .map(|val| match val {
                                Value::None => 0.0,
                                other => other.f64(value_format)
                                    .expect("Cannot convert value area to number value area"),
                            })
                            .collect()
                    })
                    .collect();

                if !values.is_empty() {
                    return Ok(values);
                }
                Err("Cannot convert empty value area to number value area".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    let values: Vec<Vec<f64>> = value
                        .iter()
                        .map(|values| {
                            values
                                .iter()
                                .map(|val| match val {
                                    Value::None => 0.0,
                                    other => other.f64(value_format).expect(
                                        "Cannot convert value area to number value area",
                                    ),
                                })
                                .collect()
                        })
                        .collect();

                    if !values.is_empty() {
                        return Ok(values);
                    }
                }
                Err("Cannot convert empty value area to number value area".into())
            }
            Value::None => Err("Cannot convert none to number value list".into()),
            Value::ChronoDateTime(value) => Ok(vec![vec![date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?]]),
            Value::OptionChronoDateTime(value) => {
                if let Some(value) = value {
                    Ok(vec![vec![date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?]])
                } else {
                    Err("Cannot convert empty date time to number value area".into())
                }
            }
            Value::OptionTime(value) => {
                if let Some(value) = value {
                    Ok(vec![vec![time_to_excel(value)?]])
                } else {
                    Err("Cannot convert empty time to number value area".into())
                }
            }
            Value::Time(value) => Ok(vec![vec![time_to_excel(value)?]]),
        }
    }

    pub fn area_i32(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Vec<Vec<i32>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![vec![integer(
                *value,
                &value_format.decimal_separator,
            )?]]),
            Value::I32(value) => Ok(vec![vec![*value]]),
            Value::String(value) => Ok(vec![vec![integer(
                value.as_str(),
                &value_format.decimal_separator,
            )?]]),
            Value::Bool(value) => Ok(vec![vec![integer(
                *value,
                &value_format.decimal_separator,
            )?]]),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert float none to number value area".into()),
                Some(value) => Ok(vec![vec![integer(
                    *value,
                    &value_format.decimal_separator,
                )?]]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert integer none to number value area".into()),
                Some(value) => Ok(vec![vec![*value]]),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to number value area".into()),
                Some(value) => Ok(vec![vec![integer(
                    value.as_str(),
                    &value_format.decimal_separator,
                )?]]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to number value area".into()),
                Some(value) => Ok(vec![vec![integer(
                    *value,
                    &value_format.decimal_separator,
                )?]]),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to number value area".into())
                } else {
                    let vector: Vec<Vec<i32>> = value
                        .iter()
                        .map(|val| {
                            vec![val
                                .i32(value_format)
                                .expect("Cannot convert value list to number area")]
                        })
                        .collect();
                    Ok(vector)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to number value area".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty value list to number value area".into())
                    } else {
                        let vector: Vec<Vec<i32>> = value
                            .iter()
                            .map(|val| {
                                vec![val
                                    .i32(value_format)
                                    .expect("Cannot convert value list to number list")]
                            })
                            .collect();
                        Ok(vector)
                    }
                }
            },
            Value::AreaValue(value) => {
                let values: Vec<Vec<i32>> = value
                    .iter()
                    .map(|values| {
                        values
                            .iter()
                            .map(|val| match val {
                                Value::None => 0,
                                other => other.i32(value_format)
                                    .expect("Cannot convert value area to number value area"),
                            })
                            .collect()
                    })
                    .collect();

                if !values.is_empty() {
                    return Ok(values);
                }
                Err("Cannot convert empty value area to number value area".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    let values: Vec<Vec<i32>> = value
                        .iter()
                        .map(|values| {
                            values
                                .iter()
                                .map(|val| match val {
                                    Value::None => 0,
                                    other => other.i32(value_format).expect(
                                        "Cannot convert value area to number value area",
                                    ),
                                })
                                .collect()
                        })
                        .collect();

                    if !values.is_empty() {
                        return Ok(values);
                    }
                }
                Err("Cannot convert empty value area to number value area".into())
            }
            Value::None => Err("Cannot convert none to number value list".into()),
            Value::ChronoDateTime(value) => Ok(vec![vec![integer(
                date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?,
                &value_format.decimal_separator,
            )?]]),
            Value::OptionChronoDateTime(value) => {
                if let Some(value) = value {
                    Ok(vec![vec![integer(
                        date_time_to_excel(value, value_format.allow_lotus_1_2_3_1900_date_bug)?,
                        &value_format.decimal_separator,
                    )?]])
                } else {
                    Err("Cannot convert empty date time to number value area".into())
                }
            }
            Value::OptionTime(value) => {
                if let Some(value) = value {
                    Ok(vec![vec![integer(
                        time_to_excel(value)?,
                        &value_format.decimal_separator,
                    )?]])
                } else {
                    Err("Cannot convert empty time to number value area".into())
                }
            }
            Value::Time(value) => Ok(vec![vec![integer(
                time_to_excel(value)?,
                &value_format.decimal_separator,
            )?]]),
        }
    }

    pub fn to_flatterned_vec_f64(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
        let array = self.area_f64(value_format)?;
        Ok(array
            .iter()
            .flat_map(|row| row.iter().cloned())
            .collect::<Vec<f64>>())
    }

    pub fn to_flatterned_vec_i32(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Vec<i32>, Box<dyn Error + Send + Sync>> {
        let array = self.area_i32(value_format)?;
        Ok(array
            .iter()
            .flat_map(|row| row.iter().cloned())
            .collect::<Vec<i32>>())
    }

    pub fn to_flatterned_vec_value(&self) -> Result<Vec<Value>, Box<dyn Error + Send + Sync>> {
        let array = self.area_of_value()?;
        Ok(array
            .iter()
            .flat_map(|row| row.iter().cloned())
            .collect::<Vec<Value>>())
    }

    pub fn to_flatterned_vec_date_time(
        &self,
        strict_type_conversion: bool,
        value_format: &ValueFormat,
    ) -> Result<Vec<DateTime<Utc>>, Box<dyn Error + Send + Sync>> {
        let array = self.area_of_date_time(strict_type_conversion, value_format)?;
        Ok(array
            .iter()
            .flat_map(|row| row.iter().cloned())
            .collect::<Vec<DateTime<Utc>>>())
    }

    pub fn area_bool(
        &self,
        value_format: &ValueFormat,
    ) -> Result<Vec<Vec<bool>>, Box<dyn Error + Send + Sync>> {
        match self {
            Value::F64(value) => Ok(vec![vec![boolean(
                *value,
                &value_format.decimal_separator,
            )?]]),
            Value::I32(value) => Ok(vec![vec![boolean(
                *value,
                &value_format.decimal_separator,
            )?]]),
            Value::String(value) => Ok(vec![vec![boolean(
                value.as_str(),
                &value_format.decimal_separator,
            )?]]),
            Value::Bool(value) => Ok(vec![vec![*value]]),
            Value::OptionF64(value) => match value {
                None => Err("Cannot convert float none to boolean value area".into()),
                Some(value) => Ok(vec![vec![boolean(
                    *value,
                    &value_format.decimal_separator,
                )?]]),
            },
            Value::OptionI32(value) => match value {
                None => Err("Cannot convert integer none to boolean value area".into()),
                Some(value) => Ok(vec![vec![boolean(
                    *value,
                    &value_format.decimal_separator,
                )?]]),
            },
            Value::OptionString(value) => match value {
                None => Err("Cannot convert string none to boolean value area".into()),
                Some(value) => Ok(vec![vec![boolean(
                    value.as_str(),
                    &value_format.decimal_separator,
                )?]]),
            },
            Value::OptionBool(value) => match value {
                None => Err("Cannot convert boolean none to boolean value area".into()),
                Some(value) => Ok(vec![vec![*value]]),
            },
            Value::VecValue(value) => {
                if value.is_empty() {
                    Err("Cannot convert empty value list to boolean value area".into())
                } else {
                    let vector: Vec<Vec<bool>> = value
                        .iter()
                        .map(|val| {
                            vec![val
                                .bool(value_format)
                                .expect("Cannot convert value list to boolean area")]
                        })
                        .collect();
                    Ok(vector)
                }
            }
            Value::OptionVecValue(value) => match value {
                None => Err("Cannot convert none value list to boolean value area".into()),
                Some(value) => {
                    if value.is_empty() {
                        Err("Cannot convert empty value list to boolean value area".into())
                    } else {
                        let vector: Vec<Vec<bool>> = value
                            .iter()
                            .map(|val| {
                                vec![val
                                    .bool(value_format)
                                    .expect("Cannot convert value list to boolean list")]
                            })
                            .collect();
                        Ok(vector)
                    }
                }
            },
            Value::AreaValue(value) => {
                let values: Vec<Vec<bool>> = value
                    .iter()
                    .map(|values| {
                        values
                            .iter()
                            .map(|val| {
                                val.bool(value_format)
                                    .expect("Cannot convert empty value area to boolean value area")
                            })
                            .collect()
                    })
                    .collect();

                if !values.is_empty() {
                    return Ok(values);
                }
                Err("Cannot convert empty value area to number value area".into())
            }
            Value::OptionAreaValue(value) => {
                if let Some(value) = value {
                    let values: Vec<Vec<bool>> = value
                        .iter()
                        .map(|values| {
                            values
                                .iter()
                                .map(|val| {
                                    val.bool(value_format).expect(
                                        "Cannot convert empty value area to boolean value area",
                                    )
                                })
                                .collect()
                        })
                        .collect();

                    if !values.is_empty() {
                        return Ok(values);
                    }
                }
                Err("Cannot convert empty value area to boolean value area".into())
            }
            Value::None => Err("Cannot convert none to boolean value area".into()),
            Value::ChronoDateTime(_value) => {
                Err("Cannot convert date time to boolean value area".into())
            }
            Value::OptionChronoDateTime(_value) => {
                Err("Cannot convert date time to boolean value area".into())
            }
            Value::OptionTime(_) => Err("Cannot convert time to boolean value area".into()),
            Value::Time(_) => Err("Cannot convert time to boolean value area".into()),
        }
    }

    pub fn is_none(&self) -> bool {
        match self {
            Value::F64(_) => false,
            Value::I32(_) => false,
            Value::String(_) => false,
            Value::Bool(_) => false,
            Value::OptionF64(value) => value.is_none(),
            Value::OptionI32(value) => value.is_none(),
            Value::OptionString(value) => value.is_none(),
            Value::OptionBool(value) => value.is_none(),
            Value::VecValue(_) => false,
            Value::OptionVecValue(value) => value.is_none(),
            Value::AreaValue(_) => false,
            Value::OptionAreaValue(value) => value.is_none(),
            Value::OptionChronoDateTime(value) => value.is_none(),
            Value::ChronoDateTime(_) => false,
            Value::OptionTime(value) => value.is_none(),
            Value::Time(_) => false,
            Value::None => true,
        }
    }
}

pub fn f64(value: f64) -> Value {
    Value::F64(value)
}

pub fn date_time(value: DateTime<Utc>) -> Value {
    Value::ChronoDateTime(value)
}

pub fn time(value: NaiveTime) -> Value {
    Value::Time(value)
}

pub fn i32(value: i32) -> Value {
    Value::I32(value)
}

pub fn str(value: &str) -> Value {
    Value::String(value.to_string())
}

pub fn string(value: String) -> Value {
    Value::String(value)
}

pub fn bool(value: bool) -> Value {
    Value::Bool(value)
}

pub fn vec_f64(value: Vec<f64>) -> Value {
    let value_list: Vec<Value> = value.iter().map(|val| Value::F64(*val)).collect();
    Value::VecValue(value_list)
}

pub fn vec_i32(value: Vec<i32>) -> Value {
    let value_list: Vec<Value> = value.iter().map(|val| Value::I32(*val)).collect();
    Value::VecValue(value_list)
}

pub fn vec_string(value: Vec<String>) -> Value {
    let value_list: Vec<Value> = value
        .iter()
        .map(|val| Value::String(val.to_string()))
        .collect();
    Value::VecValue(value_list)
}

pub fn vec_bool(value: Vec<bool>) -> Value {
    let value_list: Vec<Value> = value.iter().map(|val| Value::Bool(*val)).collect();
    Value::VecValue(value_list)
}

pub fn vec_date_time(value: Vec<DateTime<Utc>>) -> Value {
    let value_list: Vec<Value> = value
        .iter()
        .map(|val| Value::ChronoDateTime(*val))
        .collect();
    Value::VecValue(value_list)
}

pub fn vec_time(value: Vec<NaiveTime>) -> Value {
    let value_list: Vec<Value> = value.iter().map(|val| Value::Time(*val)).collect();
    Value::VecValue(value_list)
}

pub fn some_f64(value: f64) -> Value {
    Value::OptionF64(Some(value))
}

pub fn some_i32(value: i32) -> Value {
    Value::OptionI32(Some(value))
}

pub fn some_string(value: String) -> Value {
    Value::OptionString(Some(value))
}

pub fn some_str(value: &str) -> Value {
    Value::OptionString(Some(value.to_string()))
}

pub fn some_bool(value: bool) -> Value {
    Value::OptionBool(Some(value))
}

pub fn none() -> Value {
    Value::None
}

pub fn area_f64(value: Vec<Vec<f64>>) -> Value {
    let values: Vec<Vec<Value>> = value
        .into_iter()
        .map(|inner| inner.into_iter().map(Value::F64).collect::<Vec<Value>>())
        .collect();
    Value::AreaValue(values)
}

pub fn area_i32(value: Vec<Vec<i32>>) -> Value {
    let values: Vec<Vec<Value>> = value
        .into_iter()
        .map(|inner| inner.into_iter().map(Value::I32).collect::<Vec<Value>>())
        .collect();
    Value::AreaValue(values)
}

pub fn area_string(value: Vec<Vec<String>>) -> Value {
    let values: Vec<Vec<Value>> = value
        .into_iter()
        .map(|inner| inner.into_iter().map(Value::String).collect::<Vec<Value>>())
        .collect();
    Value::AreaValue(values)
}

pub fn area_bool(value: Vec<Vec<bool>>) -> Value {
    let values: Vec<Vec<Value>> = value
        .into_iter()
        .map(|inner| inner.into_iter().map(Value::Bool).collect::<Vec<Value>>())
        .collect();
    Value::AreaValue(values)
}

pub fn area_date_time(value: Vec<Vec<DateTime<Utc>>>) -> Value {
    let values: Vec<Vec<Value>> = value
        .into_iter()
        .map(|inner| {
            inner
                .into_iter()
                .map(Value::ChronoDateTime)
                .collect::<Vec<Value>>()
        })
        .collect();
    Value::AreaValue(values)
}

pub fn area_time(value: Vec<Vec<NaiveTime>>) -> Value {
    let values: Vec<Vec<Value>> = value
        .into_iter()
        .map(|inner| inner.into_iter().map(Value::Time).collect::<Vec<Value>>())
        .collect();
    Value::AreaValue(values)
}

pub fn flatten_value_to_partial_vec(value: Value) -> Vec<Value> {
    match value {
        Value::VecValue(vec) => vec
            .into_iter()
            .flat_map(flatten_value_to_partial_vec)
            .collect(),
        Value::OptionVecValue(Some(vec)) => vec
            .into_iter()
            .flat_map(flatten_value_to_partial_vec)
            .collect(),
        Value::AreaValue(vec) => vec
            .into_iter()
            .flat_map(|inner_vec| inner_vec.to_vec())
            .collect(),
        other => vec![other], // For all other types, wrap in a Vec
    }
}

// TODO: STRICT_CONVERSION
pub fn flatten_value_to_vec_f64(
    value: Value,
    value_format: &ValueFormat,
) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
    let value = value.area_f64(value_format)?;
    Ok(value
        .iter()
        .flat_map(|row| row.iter().cloned())
        .collect::<Vec<f64>>())
}

fn is_pure_string(value: &str) -> bool {
    // Check if the string value can be converted to a number or boolean
    if value.parse::<f64>().is_ok() || value.parse::<bool>().is_ok() {
        return false;
    }
    true
}

// TODO: THIS NEEDS TO CONSIDER strict_type_conversion
pub(crate) fn vec_value_to_vec_f64(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
    // Collect all values into a single Vec<f64> by flattening ranges from each Value
    Ok(values
        .into_iter()
        .flat_map(|value| {
            value
                .area_f64(value_format) // Get the range of f64 values for the current Value
                .unwrap_or_else(|_| vec![]) // Handle potential errors by returning an empty Vec (consider better error handling if needed)
                .into_iter()
                .flatten() // Flatten the 2D array into a 1D iterator
        })
        .collect())
}

// TODO: THIS NEEDS TO CONSIDER strict_type_conversion
pub(crate) fn vec_value_to_vec_i32(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Vec<i32>, Box<dyn Error + Send + Sync>> {
    // Collect all values into a single Vec<i32> by flattening ranges from each Value
    Ok(values
        .into_iter()
        .flat_map(|value| {
            value
                .area_i32(value_format) // Get the range of i32 values for the current Value
                .unwrap_or_else(|_| vec![]) // Handle potential errors by returning an empty Vec (consider better error handling if needed)
                .into_iter()
                .flatten() // Flatten the 2D array into a 1D iterator
        })
        .collect())
}

// TODO: THIS NEEDS TO CONSIDER strict_type_conversion
pub(crate) fn vec_value_to_vec_string(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    // Collect all values into a single Vec<String> by flattening ranges from each Value
    Ok(values
        .into_iter()
        .flat_map(|value| {
            value
                .area_string(value_format) // Get the range of f64 values for the current Value
                .unwrap_or_else(|_| vec![]) // Handle potential errors by returning an empty Vec (consider better error handling if needed)
                .into_iter()
                .flatten() // Flatten the 2D array into a 1D iterator
        })
        .collect())
}

pub(crate) fn vec_value_to_vec_value(
    values: Vec<Value>,
) -> Result<Vec<Value>, Box<dyn Error + Send + Sync>> {
    // Collect all values into a single Vec<String> by flattening ranges from each Value
    Ok(values
        .into_iter()
        .flat_map(|value| {
            value
                .area_of_value()
                .unwrap_or_else(|_| vec![]) // Handle potential errors by returning an empty Vec (consider better error handling if needed)
                .into_iter()
                .flatten() // Flatten the 2D array into a 1D iterator
        })
        .collect())
}

pub(crate) fn vec_value_to_vec_boolean(
    values: Vec<Value>,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Vec<bool>, Box<dyn Error + Send + Sync>> {
    let mut all_values = Vec::new();

    for value in values {
        if value.is_array() || value.is_area() {
            match value.vec_bool(value_format) {
                Ok(vec) => all_values.extend(vec),
                Err(_) if strict_type_conversion => {
                    return Err("Input contains non-boolean values in nested lists".into());
                }
                Err(_) => {} // Non-strict mode: silently skip invalid values
            }
        } else {
            match value.bool(value_format) {
                Ok(val) => all_values.push(val),
                Err(_) if strict_type_conversion => {
                    return Err("Input contains non-boolean scalar values".into());
                }
                Err(_) => {} // Non-strict mode: silently skip invalid values
            }
        }
    }

    Ok(all_values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveTime, TimeZone, Utc};

    // Helper function to create a default ValueFormat for testing
    fn default_value_format() -> ValueFormat {
        ValueFormat {
            decimal_separator: ".".to_string(),
            currency_symbol: "$".to_string(),
            thousands_separator: ",".to_string(),
            use_excel_rounding: false,
            language: "en".to_string(),
            allow_lotus_1_2_3_1900_date_bug: true,
        }
    }

    #[test]
    fn test_value_creation() {
        // Test creating different Value variants
        let f64_value = f64(42.5);
        let i32_value = i32(42);
        let string_value = string("test".to_string());
        let bool_value = bool(true);
        let none_value = none();

        // Print results
        println!("f64_value: {:?}", f64_value);
        println!("i32_value: {:?}", i32_value);
        println!("string_value: {:?}", string_value);
        println!("bool_value: {:?}", bool_value);
        println!("none_value: {:?}", none_value);

        // Assert the correct types were created
        assert!(matches!(f64_value, Value::F64(_)));
        assert!(matches!(i32_value, Value::I32(_)));
        assert!(matches!(string_value, Value::String(_)));
        assert!(matches!(bool_value, Value::Bool(_)));
        assert!(matches!(none_value, Value::None));
    }

    #[test]
    fn test_value_equality() {
        // Test equality between values
        let value1 = f64(42.0);
        let value2 = f64(42.0);
        let value3 = f64(43.0);
        let value4 = i32(42);

        let result1 = value1 == value2;
        let result2 = value1 == value3;
        let result3 = value1 == value4;

        println!("result1: {}", result1);
        println!("result2: {}", result2);
        println!("result3: {}", result3);

        assert!(result1);
        assert!(!result2);
        assert!(!result3);
    }

    #[test]
    fn test_value_comparison() {
        // Test comparison between values
        let value1 = f64(42.0);
        let value2 = f64(43.0);

        let result1 = value1 < value2;
        let result2 = value1 > value2;

        println!("result1: {}", result1);
        println!("result2: {}", result2);

        assert!(result1);
        assert!(!result2);
    }

    #[test]
    fn test_f64_conversion() {
        let value_format = default_value_format();

        // Test f64 conversion from different Value types
        let f64_value = f64(42.5);
        let i32_value = i32(42);
        let string_value = string("42.5".to_string());
        let bool_value = bool(true);

        let result1 = f64_value.f64(&value_format);
        let result2 = i32_value.f64(&value_format);
        let result3 = string_value.f64(&value_format);
        let result4 = bool_value.f64(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);

        assert_eq!(result1.unwrap(), 42.5);
        assert_eq!(result2.unwrap(), 42.0);
        assert_eq!(result3.unwrap(), 42.5);
        assert_eq!(result4.unwrap(), 1.0);
    }

    #[test]
    fn test_i32_conversion() {
        let value_format = default_value_format();

        // Test i32 conversion from different Value types
        let f64_value = f64(42.5);
        let i32_value = i32(42);
        let string_value = string("42".to_string());
        let bool_value = bool(true);

        let result1 = f64_value.i32(&value_format);
        let result2 = i32_value.i32(&value_format);
        let result3 = string_value.i32(&value_format);
        let result4 = bool_value.i32(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);

        assert_eq!(result1.unwrap(), 42); // Rounds down from 42.5
        assert_eq!(result2.unwrap(), 42);
        assert_eq!(result3.unwrap(), 42);
        assert_eq!(result4.unwrap(), 1);
    }

    #[test]
    fn test_string_conversion() {
        let value_format = default_value_format();

        // Test string conversion from different Value types
        let f64_value = f64(42.5);
        let i32_value = i32(42);
        let string_value = string("test".to_string());
        let bool_value = bool(true);

        let result1 = f64_value.string(&value_format);
        let result2 = i32_value.string(&value_format);
        let result3 = string_value.string(&value_format);
        let result4 = bool_value.string(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);

        assert_eq!(result1.unwrap(), "42.5");
        assert_eq!(result2.unwrap(), "42");
        assert_eq!(result3.unwrap(), "test");
        assert_eq!(result4.unwrap(), "TRUE");
    }

    #[test]
    fn test_bool_conversion() {
        let value_format = default_value_format();

        // Test bool conversion from different Value types
        let f64_value_true = f64(1.0);
        let f64_value_false = f64(0.0);
        let i32_value_true = i32(1);
        let i32_value_false = i32(0);
        let string_value_true = string("true".to_string());
        let string_value_false = string("false".to_string());
        let bool_value_true = bool(true);
        let bool_value_false = bool(false);

        let result1 = f64_value_true.bool(&value_format);
        let result2 = f64_value_false.bool(&value_format);
        let result3 = i32_value_true.bool(&value_format);
        let result4 = i32_value_false.bool(&value_format);
        let result5 = string_value_true.bool(&value_format);
        let result6 = string_value_false.bool(&value_format);
        let result7 = bool_value_true.bool(&value_format);
        let result8 = bool_value_false.bool(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);
        println!("result5: {:?}", result5);
        println!("result6: {:?}", result6);
        println!("result7: {:?}", result7);
        println!("result8: {:?}", result8);

        assert!(result1.unwrap());
        assert!(!result2.unwrap());
        assert!(result3.unwrap());
        assert!(!result4.unwrap());
        assert!(result5.unwrap());
        assert!(!result6.unwrap());
        assert!(result7.unwrap());
        assert!(!result8.unwrap());
    }

    #[test]
    fn test_date_time_conversion() {
        let value_format = default_value_format();

        // Create a DateTime value
        let dt = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let dt_value = date_time(dt);
        let string_value = string("2023-01-01T12:00:00Z".to_string());

        let result1 = dt_value.date_time(&value_format);
        let result2 = string_value.date_time(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);

        assert_eq!(result1.unwrap(), dt);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_time_conversion() {
        let value_format = default_value_format();

        // Create a Time value
        let time_val = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let time_value = Value::Time(time_val);
        let string_value = string("12:00:00".to_string());

        let result1 = time_value.time(&value_format);
        let result2 = string_value.time(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);

        assert_eq!(result1.unwrap(), time_val);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_option_conversions() {
        let value_format = default_value_format();

        // Test option conversions
        let some_f64_value = some_f64(42.5);
        let some_i32_value = some_i32(42);
        let some_string_value = some_string("test".to_string());
        let some_bool_value = some_bool(true);
        let none_value = none();

        let result1 = some_f64_value.option_f64(&value_format);
        let result2 = some_i32_value.option_i32(&value_format);
        let result3 = some_string_value.option_string(&value_format);
        let result4 = some_bool_value.option_bool(&value_format);
        let result5 = none_value.option_f64(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);
        println!("result5: {:?}", result5);

        assert_eq!(result1.unwrap(), Some(42.5));
        assert_eq!(result2.unwrap(), Some(42));
        assert_eq!(result3.unwrap(), Some("test".to_string()));
        assert_eq!(result4.unwrap(), Some(true));
        assert_eq!(result5.unwrap(), None);
    }

    #[test]
    fn test_vec_conversions() {
        let value_format = default_value_format();

        // Test vector conversions
        let vec_f64_values = vec_f64(vec![1.0, 2.0, 3.0]);
        let vec_i32_values = vec_i32(vec![1, 2, 3]);
        let vec_string_values = vec_string(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        let vec_bool_values = vec_bool(vec![true, false, true]);

        let result1 = vec_f64_values.vec_f64(&value_format);
        let result2 = vec_i32_values.vec_i32(&value_format);
        let result3 = vec_string_values.vec_string(&value_format);
        let result4 = vec_bool_values.vec_bool(&value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);

        assert_eq!(result1.unwrap(), vec![1.0, 2.0, 3.0]);
        assert_eq!(result2.unwrap(), vec![1, 2, 3]);
        assert_eq!(
            result3.unwrap(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
        assert_eq!(result4.unwrap(), vec![true, false, true]);
    }

    #[test]
    fn test_area_conversions() {
        let value_format = default_value_format();

        // Test area conversions
        let area_f64_values = area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let area_i32_values = area_i32(vec![vec![1, 2], vec![3, 4]]);
        let area_string_values = area_string(vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ]);
        let area_bool_values = area_bool(vec![vec![true, false], vec![false, true]]);

        let result1 = area_f64_values.area_of_f64(true, &value_format);
        let result2 = area_i32_values.area_of_i32(true, &value_format);
        let result3 = area_string_values.area_of_string(true, &value_format);
        let result4 = area_bool_values.area_of_bool(true, &value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);

        assert_eq!(result1.unwrap(), vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        assert_eq!(result2.unwrap(), vec![vec![1, 2], vec![3, 4]]);
        assert_eq!(
            result3.unwrap(),
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["c".to_string(), "d".to_string()]
            ]
        );
        assert_eq!(result4.unwrap(), vec![vec![true, false], vec![false, true]]);
    }

    #[test]
    fn test_is_functions() {
        // Test is_* functions
        let f64_value = f64(42.5);
        let string_value = string("test".to_string());
        let vec_value = vec_f64(vec![1.0, 2.0, 3.0]);
        let area_value = area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
        let dt = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let dt_value = date_time(dt);
        let time_val = NaiveTime::from_hms_opt(12, 0, 0).unwrap();
        let time_value = Value::Time(time_val);

        let result1 = f64_value.is_count_number_type();
        let result2 = string_value.is_single_string();
        let result3 = vec_value.is_array();
        let result4 = area_value.is_area();
        let result5 = dt_value.is_datetime();
        let result6 = time_value.is_time();
        let result7 = string_value.is_string();

        println!("result1: {}", result1);
        println!("result2: {}", result2);
        println!("result3: {}", result3);
        println!("result4: {}", result4);
        println!("result5: {}", result5);
        println!("result6: {}", result6);
        println!("result7: {}", result7);

        assert!(result1);
        assert!(result2);
        assert!(result3);
        assert!(result4);
        assert!(result5);
        assert!(result6);
        assert!(result7);
    }

    #[test]
    fn test_to_single_value() {
        // Test to_single_value function
        let vec_value = vec_f64(vec![1.0, 2.0, 3.0]);
        let area_value = area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let result1 = vec_value.to_single_value();
        let result2 = area_value.to_single_value();

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);

        assert!(matches!(result1, Value::F64(_)));
        assert!(matches!(result2, Value::F64(_)));
    }

    #[test]
    fn test_flatten_functions() {
        let value_format = default_value_format();

        // Test flatten functions
        let area_f64_values = area_f64(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

        let result1 = area_f64_values.to_flatterned_vec_f64(&value_format);

        println!("result1: {:?}", result1);

        assert_eq!(result1.unwrap(), vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_vec_value_to_vec_functions() {
        let value_format = default_value_format();

        // Test vec_value_to_vec_* functions
        let values = vec![f64(1.0), f64(2.0), f64(3.0)];

        let result1 = vec_value_to_vec_f64(values.clone(), &value_format);
        let result2 = vec_value_to_vec_i32(values.clone(), &value_format);
        let result3 = vec_value_to_vec_string(values.clone(), &value_format);
        let result4 = vec_value_to_vec_value(values.clone());
        let result5 = vec_value_to_vec_boolean(values.clone(), false, &value_format);

        println!("result1: {:?}", result1);
        println!("result2: {:?}", result2);
        println!("result3: {:?}", result3);
        println!("result4: {:?}", result4);
        println!("result5: {:?}", result5);

        assert_eq!(result1.unwrap(), vec![1.0, 2.0, 3.0]);
        assert_eq!(result2.unwrap(), vec![1, 2, 3]);
        assert_eq!(
            result3.unwrap(),
            vec!["1".to_string(), "2".to_string(), "3".to_string()]
        );
        assert_eq!(result4.unwrap().len(), 3);
        assert_eq!(result5.unwrap(), vec![true, true, true]);
    }

    #[test]
    fn test_error_handling() {
        // Test error handling when converting invalid values
        let value_format = default_value_format();

        let string_value = string("not a number".to_string());

        let result = string_value.f64(&value_format);

        println!("result: {:?}", result);

        // The conversion should fail because "not a number" can't be converted to f64
        assert!(result.is_err());
    }

    #[test]
    fn test_hash_implementation() {
        use std::collections::HashMap;

        // Test Hash implementation
        let mut map = HashMap::new();
        let key1 = f64(42.0);
        let key2 = f64(42.0);
        let key3 = f64(43.0);

        map.insert(key1, "value1");

        let result1 = map.contains_key(&key2);
        let result2 = map.contains_key(&key3);

        println!("result1: {}", result1);
        println!("result2: {}", result2);

        assert!(result1);
        assert!(!result2);
    }

    #[test]
    fn test_raw_string() {
        // Test raw_string function
        let f64_value = f64(42.5);
        let i32_value = i32(42);
        let string_value = string("test".to_string());
        let bool_value = bool(true);

        let result1 = f64_value.raw_string();
        let result2 = i32_value.raw_string();
        let result3 = string_value.raw_string();
        let result4 = bool_value.raw_string();

        println!("result1: {}", result1);
        println!("result2: {}", result2);
        println!("result3: {}", result3);
        println!("result4: {}", result4);

        assert_eq!(result1, "42.5");
        assert_eq!(result2, "42");
        assert_eq!(result3, "test");
        assert_eq!(result4, "TRUE");
    }

    #[test]
    fn test_is_pure_string() {
        // Test is_pure_string function
        let result1 = is_pure_string("test");
        let result2 = is_pure_string("123");
        let result3 = is_pure_string("123.45");

        println!("result1: {}", result1);
        println!("result2: {}", result2);
        println!("result3: {}", result3);

        assert!(result1);
        assert!(!result2);
        assert!(!result3);
    }
}
