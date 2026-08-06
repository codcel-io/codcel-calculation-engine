// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `ISTEXT` that checks whether a value is text.
/// - `value`: the cell value to test.
/// - `_value_format`: unused; retained for signature consistency with other functions.
///
/// Returns `true` if the value is a text string, `false` otherwise.
pub fn codcel_is_text(
    value: &Value,
    _value_format: &ValueFormat,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    Ok(value.is_excel_single_text())
}
