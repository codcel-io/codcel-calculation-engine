// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::arithmetic_base::float;
use std::error::Error;

pub trait ToVecF64 {
    fn to_vec_f64(self, decimal_separator: &str) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>>;
}

impl ToVecF64 for &str {
    fn to_vec_f64(self, decimal_separator: &str) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
        Ok(vec![float(self, decimal_separator)?])
    }
}

impl ToVecF64 for bool {
    fn to_vec_f64(self, decimal_separator: &str) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
        Ok(vec![float(self, decimal_separator)?])
    }
}

impl ToVecF64 for String {
    fn to_vec_f64(self, decimal_separator: &str) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
        Ok(vec![float(self, decimal_separator)?])
    }
}

impl ToVecF64 for i32 {
    fn to_vec_f64(self, decimal_separator: &str) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
        Ok(vec![float(self, decimal_separator)?])
    }
}

impl ToVecF64 for f64 {
    fn to_vec_f64(
        self,
        _decimal_separator: &str,
    ) -> Result<Vec<f64>, Box<dyn Error + Send + Sync>> {
        Ok(vec![self])
    }
}
