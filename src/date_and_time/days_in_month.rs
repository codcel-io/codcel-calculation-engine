// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Returns the number of days in the given `month`/`year`, matching Excel leap-year rules.
///
/// `function_name` is only used for error context. Errors on invalid month values.
pub fn days_in_month(
    function_name: &str,
    year: i32,
    month: u32,
) -> Result<u32, Box<dyn Error + Send + Sync>> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(31),
        4 | 6 | 9 | 11 => Ok(30),
        2 => {
            // February - check for leap year
            if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                Ok(29)
            } else {
                Ok(28)
            }
        }
        _ => Err(format!("{function_name}: Invalid month").into()),
    }
}
