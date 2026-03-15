// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::date_and_time::time_to_excel_fraction::time_to_excel_fraction;
use chrono::{DateTime, NaiveDate, Timelike, Utc};

/// Converts a UTC `DateTime` to an Excel serial number, including the time fraction.
///
/// Uses the 1899-12-30 epoch to mirror Excel's 1900 leap-year bug and adds the fractional part
/// of the day from the time component.
pub fn datetime_to_excel_serial(dt: &DateTime<Utc>) -> f64 {
    // Excel uses 1900-01-01 as day 1
    // Also need to account for Excel's leap year bug where it treats 1900 as a leap year
    let excel_epoch = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap();

    let days_since_epoch = (dt.date_naive() - excel_epoch).num_days();

    // Convert to f64 (days are whole numbers in Excel)
    days_since_epoch as f64 + time_to_excel_fraction(dt.hour(), dt.minute(), dt.second())
}
