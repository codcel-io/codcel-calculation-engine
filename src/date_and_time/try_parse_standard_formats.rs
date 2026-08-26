// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

/// Attempts to parse common full-date formats (ISO, slash/dash-separated, and named months).
///
/// Returns a midnight UTC `DateTime` when successful.
pub fn try_parse_standard_formats(date_text: &str) -> Option<DateTime<Utc>> {
    // Common formats
    let formats = [
        "%Y-%m-%d",          // 2022-01-30
        "%d/%m/%Y",          // 30/01/2022
        "%m/%d/%Y",          // 01/30/2022
        "%Y/%m/%d",          // 2022/01/30
        "%d-%m-%Y",          // 30-01-2022
        "%m-%d-%Y",          // 01-30-2022
        "%b %d, %Y",         // Jan 30, 2022
        "%B %d, %Y",         // January 30, 2022
        "%d %b %Y",          // 30 Jan 2022
        "%d %B %Y",          // 30 January 2022
        "%Y-%m-%dT%H:%M:%S", // 2022-01-30T00:00:00
        "%Y/%m/%d %H:%M:%S", // 2022/01/30 00:00:00
    ];

    for format in &formats {
        if let Ok(dt) =
            NaiveDateTime::parse_from_str(&format!("{date_text} 00:00:00"), "%Y-%m-%d %H:%M:%S")
        {
            return Some(Utc.from_utc_datetime(&dt));
        }

        if let Ok(date) = NaiveDate::parse_from_str(date_text, format) {
            let dt = date.and_time(NaiveTime::MIN);
            return Some(Utc.from_utc_datetime(&dt));
        }
    }

    None
}
