// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use regex::Regex;

/// Attempts to parse Excel-like date strings (MDY/DMY/YMD and `DD-MMM-YYYY`).
///
/// Also handles 2-digit years using Excel's cutoff (00–29 => 2000s, 30–99 => 1900s) and
/// returns a midnight UTC `DateTime` when successful.
pub fn try_parse_excel_formats(date_text: &str) -> Option<DateTime<Utc>> {
    // Handle Excel specific formats with regex
    let re_mdy = Regex::new(r"^(\d{1,2})[/-](\d{1,2})[/-](\d{2,4})$").ok()?;
    let re_dmy = Regex::new(r"^(\d{1,2})[/-](\d{1,2})[/-](\d{2,4})$").ok()?;
    let re_ymd = Regex::new(r"^(\d{4})[/-](\d{1,2})[/-](\d{1,2})$").ok()?;
    let re_dmmy = Regex::new(r"^(\d{1,2})[-]([A-Za-z]{3,9})[-](\d{2,4})$").ok()?;

    if let Some(caps) = re_mdy.captures(date_text) {
        let month: u32 = caps.get(1)?.as_str().parse().ok()?;
        let day: u32 = caps.get(2)?.as_str().parse().ok()?;
        let mut year: i32 = caps.get(3)?.as_str().parse().ok()?;

        // Convert 2-digit year to 4-digit (Excel rules)
        if year < 100 {
            year = if year < 30 { 2000 + year } else { 1900 + year };
        }

        if (1..=12).contains(&month) && (1..=31).contains(&day) {
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let dt = date.and_hms_opt(0, 0, 0).unwrap();
                return Some(Utc.from_utc_datetime(&dt));
            }
        }
    }

    if let Some(caps) = re_dmy.captures(date_text) {
        let day: u32 = caps.get(1)?.as_str().parse().ok()?;
        let month: u32 = caps.get(2)?.as_str().parse().ok()?;
        let mut year: i32 = caps.get(3)?.as_str().parse().ok()?;

        // Convert 2-digit year to 4-digit (Excel rules)
        if year < 100 {
            year = if year < 30 { 2000 + year } else { 1900 + year };
        }

        if (1..=12).contains(&month) && (1..=31).contains(&day) {
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let dt = date.and_hms_opt(0, 0, 0).unwrap();
                return Some(Utc.from_utc_datetime(&dt));
            }
        }
    }

    if let Some(caps) = re_ymd.captures(date_text) {
        let year: i32 = caps.get(1)?.as_str().parse().ok()?;
        let month: u32 = caps.get(2)?.as_str().parse().ok()?;
        let day: u32 = caps.get(3)?.as_str().parse().ok()?;

        if (1..=12).contains(&month) && (1..=31).contains(&day) {
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let dt = date.and_hms_opt(0, 0, 0).unwrap();
                return Some(Utc.from_utc_datetime(&dt));
            }
        }
    }

    // Handle DD-MMM-YYYY format (e.g., 20-May-2023)
    if let Some(caps) = re_dmmy.captures(date_text) {
        let day: u32 = caps.get(1)?.as_str().parse().ok()?;
        let month_str = caps.get(2)?.as_str().to_lowercase();
        let mut year: i32 = caps.get(3)?.as_str().parse().ok()?;

        // Convert 2-digit year to 4-digit (Excel rules)
        if year < 100 {
            year = if year < 30 { 2000 + year } else { 1900 + year };
        }

        // Map month name to month number
        let month = match month_str.as_str() {
            "jan" | "january" => 1,
            "feb" | "february" => 2,
            "mar" | "march" => 3,
            "apr" | "april" => 4,
            "may" => 5,
            "jun" | "june" => 6,
            "jul" | "july" => 7,
            "aug" | "august" => 8,
            "sep" | "september" => 9,
            "oct" | "october" => 10,
            "nov" | "november" => 11,
            "dec" | "december" => 12,
            _ => return None,
        };

        if (1..=31).contains(&day) {
            if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                let dt = date.and_hms_opt(0, 0, 0).unwrap();
                return Some(Utc.from_utc_datetime(&dt));
            }
        }
    }

    None
}
