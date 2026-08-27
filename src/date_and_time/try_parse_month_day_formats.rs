// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, NaiveDate, NaiveTime, TimeZone, Utc};

/// Parses month/day-only strings (e.g. `"Jan 30"`, `"30/01"`) assuming the current year.
///
/// Returns a midnight UTC `DateTime` when a format matches.
pub fn try_parse_month_day_formats(date_text: &str) -> Option<DateTime<Utc>> {
    // "Which year is it" is a wall-clock question, and Excel answers it from
    // local time — on 31 December a caller west of Greenwich is still in the
    // old year while UTC has already rolled over.
    //
    // This resolves the host zone and honours `CODCEL_MOCK_NOW`, but not an
    // explicit `ValueFormat::timezone`: `DATEVALUE` is handed a
    // `DateSemantics` rather than a `ValueFormat`, and threading one in would
    // change a signature the transpiler generates calls against for a
    // discrepancy that can only show at a year boundary.
    let current_year = crate::clock::now(&crate::value_format::ValueFormat::default()).year();

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
            let dt = date.and_time(NaiveTime::MIN);
            return Some(Utc.from_utc_datetime(&dt));
        }
    }

    None
}
