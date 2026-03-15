// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;
use std::fmt;

// Defining a custom error type for better error handling
#[derive(Debug)]
struct ConversionError;

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot convert to boolean")
    }
}

impl Error for ConversionError {}

// Defining a trait for conversion to boolean
pub trait ToBool {
    fn to_bool(self) -> Result<bool, Box<dyn Error + Send + Sync>>;
}

pub fn any_string_to_bool(value: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
    match value.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        "1" => Ok(true),
        "1.0" => Ok(true),
        _ => Err(Box::new(ConversionError)),
    }
}

// Implementing ToBool for String
impl ToBool for String {
    fn to_bool(self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        any_string_to_bool(&self)
    }
}

impl ToBool for &str {
    fn to_bool(self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        any_string_to_bool(self)
    }
}

// Implementing ToBool for bool
impl ToBool for bool {
    fn to_bool(self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self)
    }
}

// Implementing ToBool for i32
impl ToBool for i32 {
    fn to_bool(self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self != 0)
    }
}

// Implementing ToBool for f64
impl ToBool for f64 {
    fn to_bool(self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self != 0.0)
    }
}
