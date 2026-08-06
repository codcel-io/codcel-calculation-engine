// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

/// Converts time components to Excel's fractional day
pub fn time_to_excel_fraction(hours: u32, minutes: u32, seconds: u32) -> f64 {
    let total_seconds = hours as f64 * 3600.0 + minutes as f64 * 60.0 + seconds as f64;
    total_seconds / 86400.0 // 86400 seconds in a day
}
