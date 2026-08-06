// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};

/// Parses month/day-only strings (e.g. `"Jan 30"`, `"30/01"`) assuming the current year.
///
/// Returns a midnight UTC `DateTime` when a format matches.
pub fn try_parse_month_day_formats(date_text: &str) -> Option<DateTime<Utc>> {
    // Try to handle month and day only formats (assumes current year)
    let current_year = chrono::Utc::now().year();

    let month_day_formats = [
        "%b %d", // Jan 30
        "%B %d", // January 30
        "%d %b", // 30 Jan
        "%d %B", // 30 January
        "%m/%d", // 01/30
        "%d/%m", // 30/01
    ];

    for format in &month_day_formats {
        if let Ok(date) = NaiveDate::parse_from_str(
            &format!("{date_text} {current_year}"),
            &format!("{format} %Y"),
        ) {
            let dt = date.and_hms_opt(0, 0, 0).unwrap();
            return Some(Utc.from_utc_datetime(&dt));
        }
    }

    None
}
