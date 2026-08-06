// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

pub trait ToF64 {
    fn to_f64(self) -> Result<f64, Box<dyn Error + Send + Sync>>;
}

impl ToF64 for &str {
    fn to_f64(self) -> Result<f64, Box<dyn Error + Send + Sync>> {
        match self.parse::<f64>() {
            Ok(value) => Ok(value),
            Err(err) => Err(format!("Failed to convert &str to f64: {err}").into()),
        }
    }
}

impl ToF64 for bool {
    fn to_f64(self) -> Result<f64, Box<dyn Error + Send + Sync>> {
        if self {
            Ok(1.0)
        } else {
            Ok(0.0)
        }
    }
}

impl ToF64 for String {
    fn to_f64(self) -> Result<f64, Box<dyn Error + Send + Sync>> {
        match self.parse::<f64>() {
            Ok(value) => Ok(value),
            Err(err) => Err(format!("Failed to convert String to f64: {err}").into()),
        }
    }
}

impl ToF64 for i32 {
    fn to_f64(self) -> Result<f64, Box<dyn Error + Send + Sync>> {
        Ok(self as f64)
    }
}

impl ToF64 for f64 {
    fn to_f64(self) -> Result<f64, Box<dyn Error + Send + Sync>> {
        Ok(self)
    }
}
