// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use std::error::Error;

pub trait ToI32 {
    fn to_i32(self) -> Result<i32, Box<dyn Error + Send + Sync>>;
}

impl ToI32 for &str {
    fn to_i32(self) -> Result<i32, Box<dyn Error + Send + Sync>> {
        match self.parse::<i32>() {
            Ok(value) => Ok(value),
            Err(err) => Err(format!("Failed to convert &String to Integer: {err}").into()),
        }
    }
}

impl ToI32 for String {
    fn to_i32(self) -> Result<i32, Box<dyn Error + Send + Sync>> {
        match self.parse::<i32>() {
            Ok(value) => Ok(value),
            Err(err) => Err(format!("Failed to convert String to Integer: {err}").into()),
        }
    }
}

impl ToI32 for bool {
    fn to_i32(self) -> Result<i32, Box<dyn Error + Send + Sync>> {
        if self {
            Ok(1)
        } else {
            Ok(0)
        }
    }
}

impl ToI32 for i32 {
    fn to_i32(self) -> Result<i32, Box<dyn Error + Send + Sync>> {
        Ok(self)
    }
}

impl ToI32 for f64 {
    fn to_i32(self) -> Result<i32, Box<dyn Error + Send + Sync>> {
        if self < i32::MIN as f64 || self > i32::MAX as f64 {
            Err("Failed to convert Float to Integer: value out of range".into())
        } else {
            Ok(self as i32)
        }
    }
}

impl ToI32 for Vec<Vec<i32>> {
    fn to_i32(self) -> Result<i32, Box<dyn Error + Send + Sync>> {
        if !self.is_empty() && !self[0].is_empty() {
            return self[0][0].to_i32();
        }

        Err("Failed to convert AreaValue to Integer: AreaValue is empty".into())
    }
}
