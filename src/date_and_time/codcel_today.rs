// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::clock;
use crate::value_format::ValueFormat;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Excel-compatible `TODAY` that returns the current date at midnight.
///
/// Reads the wall clock of [`ValueFormat::timezone`], or of the host when that
/// is empty, which is what Excel does. Truncating UTC instead would hand a
/// caller at UTC+13 yesterday's date for thirteen hours of every day.
pub fn codcel_today(
    value_format: &ValueFormat,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    clock::today(value_format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Timelike};

    #[test]
    fn today_matches_the_local_date() {
        let vf = ValueFormat::default();
        let now = clock::now(&vf);
        let result = codcel_today(&vf).unwrap();

        assert_eq!(result.year(), now.year());
        assert_eq!(result.month(), now.month());
        assert_eq!(result.day(), now.day());
    }

    #[test]
    fn today_is_date_only() {
        let result = codcel_today(&ValueFormat::default()).unwrap();
        assert_eq!(result.hour(), 0);
        assert_eq!(result.minute(), 0);
        assert_eq!(result.second(), 0);
        assert_eq!(result.nanosecond(), 0);
    }
}
