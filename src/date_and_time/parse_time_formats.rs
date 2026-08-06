// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::NaiveTime;
use std::error::Error;

/// Attempts to parse the time string using multiple formats
pub fn parse_time_formats(time_str: &str) -> Result<NaiveTime, Box<dyn Error + Send + Sync>> {
    // List of formats to try, from most specific to least specific
    let formats = [
        // 12-hour formats with seconds and AM/PM
        "%I:%M:%S.%f %p",
        "%I:%M:%S.%f%p",
        "%I:%M:%S %p",
        "%I:%M:%S%p",
        // 12-hour formats without seconds
        "%I:%M %p",
        "%I:%M%p",
        // 24-hour formats with seconds
        "%H:%M:%S.%f",
        "%H:%M:%S",
        // 24-hour formats without seconds
        "%H:%M",
        // Hours only
        "%I %p",
        "%I%p",
        "%H",
    ];

    // Try each format
    for format in &formats {
        if let Ok(time) = NaiveTime::parse_from_str(time_str, format) {
            return Ok(time);
        }
    }

    // Special case for Excel's handling of plain numbers
    if let Ok(hours) = time_str.parse::<f64>() {
        // Handle plain numbers as decimal hours
        let total_seconds = (hours * 3600.0).round() as u32;
        let h = total_seconds / 3600;
        let m = (total_seconds % 3600) / 60;
        let s = total_seconds % 60;

        if h < 24 {
            return Ok(NaiveTime::from_hms_opt(h, m, s)
                .ok_or_else(|| "TIMEVALUE: Invalid time value".to_string())?);
        }
    }

    Err(format!("TIMEVALUE: Could not parse '{time_str}' as a valid time").into())
}
