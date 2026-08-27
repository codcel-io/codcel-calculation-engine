// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::area::{
    process_area_datetime_datetime_opt_bool_to_int, process_area_datetime_datetime_to_int,
};
use crate::arithmetic_base::float;
use crate::date_and_time::{
    codcel_date::codcel_date,
    codcel_date_dif::codcel_date_dif,
    codcel_date_value::codcel_date_value,
    codcel_day::codcel_day,
    codcel_days::codcel_days,
    codcel_days_360::codcel_days_360,
    codcel_e_date::codcel_e_date,
    codcel_eo_month::codcel_eo_month,
    codcel_hour::codcel_hour,
    codcel_iso_week_num::codcel_iso_week_num,
    codcel_minute::codcel_minute,
    codcel_month::codcel_month,
    codcel_networkdays::codcel_networkdays,
    codcel_networkdays_intl::{codcel_networkdays_intl, parse_weekend_mask, parse_weekend_string},
    codcel_now::codcel_now,
    codcel_second::codcel_second,
    codcel_time::codcel_time,
    codcel_time_value::codcel_time_value,
    codcel_today::codcel_today,
    codcel_week_day::codcel_week_day,
    codcel_week_num::codcel_week_num,
    codcel_workday::codcel_workday,
    codcel_workday_intl::codcel_workday_intl,
    codcel_year::codcel_year,
    codcel_year_frac::codcel_year_frac,
};
use crate::date_system::DateSemantics;
use crate::value::Value;
use crate::value_format::ValueFormat;
use chrono::{
    DateTime, Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, SecondsFormat, TimeZone,
    Timelike, Utc,
};
use std::error::Error;

pub fn excel_to_date_time(
    excel_date: f64,
    dates: DateSemantics,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    // Adjust for Excel's leap year bug and offset. The threshold must be
    // `>= 60.0`: serial 59.5 is 1900-02-28 12:00, still one day before the
    // phantom 1900-02-29. (The inverse below tests `>= 59.0` because it compares
    // a *day count* rather than a serial — the asymmetry is deliberate.)
    let adjusted_excel_date = if dates.lotus_1900_bug && excel_date >= 60.0 {
        excel_date - 2.0
    } else {
        excel_date - 1.0
    };

    // Calculate the date components
    let days = adjusted_excel_date.floor() as i64;
    let fraction = adjusted_excel_date.fract();

    // Base date is 1900-01-01 (Excel's actual start point, accounting for the leap year bug)
    let base_date = NaiveDate::from_ymd_opt(1900, 1, 1).ok_or("Invalid base date")?;

    // Add the number of days
    let date = base_date + Duration::days(days);

    // Convert fractional part to time of day (preserving sub-second precision)
    let total_seconds_f64 = fraction * 86_400.0;
    let whole_seconds = total_seconds_f64.floor() as u32;
    let frac_seconds = total_seconds_f64 - whole_seconds as f64;
    let nanos = (frac_seconds * 1_000_000_000.0).round().min(999_999_999.0) as u32;
    let nanos = (nanos / 1000) * 1000; // Truncate to microsecond (6-digit) precision for cross-platform compatibility

    // Create NaiveDateTime by combining date and time
    let naive_datetime = NaiveDateTime::new(
        date,
        chrono::NaiveTime::from_num_seconds_from_midnight_opt(whole_seconds, nanos)
            .ok_or("Invalid time")?,
    );

    // Convert to UTC DateTime and handle potential timezone conversion
    Utc.from_local_datetime(&naive_datetime)
        .single()
        .ok_or_else(|| "Ambiguous datetime conversion".into())
}

pub fn time_to_excel(time: &NaiveTime) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let seconds_since_midnight = time.num_seconds_from_midnight() as f64;
    let nanos = time.nanosecond() % 1_000_000_000; // fractional part only
    let total_seconds = seconds_since_midnight + nanos as f64 / 1_000_000_000.0;
    Ok(total_seconds / 86400.0) // 86400 seconds in a day
}

pub fn excel_to_time(fraction: f64) -> Result<NaiveTime, Box<dyn Error + Send + Sync>> {
    // Remove the left-hand side of the fraction
    let fraction = fraction.fract();

    let seconds_in_day = 86400.0; // Total seconds in a day
    let total_seconds_f64 = fraction * seconds_in_day;
    let whole_seconds = total_seconds_f64.floor() as u32;
    let frac_seconds = total_seconds_f64 - whole_seconds as f64;
    let nanos = (frac_seconds * 1_000_000_000.0).round().min(999_999_999.0) as u32;
    let nanos = (nanos / 1000) * 1000; // Truncate to microsecond (6-digit) precision for cross-platform compatibility
    let hours = whole_seconds / 3600;
    let minutes = (whole_seconds % 3600) / 60;
    let seconds = whole_seconds % 60;
    NaiveTime::from_hms_nano_opt(hours, minutes, seconds, nanos)
        .ok_or_else(|| "Value is out of range for a time of day".into())
}

pub fn time_to_date_time(time: NaiveTime) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    // Use a default date: January 1, 0001
    let default_date =
        NaiveDate::from_ymd_opt(1, 1, 1).ok_or("Could not construct the epoch date 0001-01-01")?;
    let naive_datetime = default_date.and_time(time); // Combine the default date with the given time
    Ok(Utc.from_utc_datetime(&naive_datetime)) // Convert to DateTime<Utc>
}

pub fn date_time_to_excel(
    date_time: &DateTime<Utc>,
    dates: DateSemantics,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Define the base date: 1900-01-01
    let base_date = NaiveDate::from_ymd_opt(1900, 1, 1).ok_or("Invalid base date")?;

    // Calculate the difference in days between the base date and the given date
    let naive_date_time = date_time.naive_utc();
    let duration_since_base = naive_date_time.date() - base_date;

    let days = duration_since_base.num_days() as f64;

    // Calculate the fraction of the day (time part, including sub-second precision)
    let seconds_since_midnight = naive_date_time.time().num_seconds_from_midnight() as f64;
    let nanos = (naive_date_time.time().nanosecond() % 1_000_000_000) as f64;
    let fraction_of_day = (seconds_since_midnight + nanos / 1_000_000_000.0) / 86_400.0;

    // Combine the days and fraction
    let excel_date = days + fraction_of_day;

    // Adjust for Excel's leap year bug (Excel incorrectly treats 1900 as a leap
    // year). `excel_date` is still a day count here, not a serial, which is why
    // the threshold is `>= 59.0` rather than the `>= 60.0` used on decode.
    if dates.lotus_1900_bug && excel_date >= 59.0 {
        Ok(excel_date + 2.0)
    } else {
        Ok(excel_date + 1.0)
    }
}

pub fn date_time_to_iso(date_time: &DateTime<Utc>) -> String {
    date_time.to_rfc3339_opts(SecondsFormat::Secs, false)
}

pub fn date_time_to_iso_display(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%d %H:%M").to_string()
}

pub fn time_to_iso(time: &NaiveTime) -> String {
    let nanos = time.nanosecond() % 1_000_000_000;
    if nanos == 0 {
        format!(
            "{:02}:{:02}:{:02}",
            time.hour(),
            time.minute(),
            time.second()
        )
    } else {
        let micros = nanos / 1000; // 6-digit value (0..=999_999)
        let frac_str = format!("{micros:06}");
        let trimmed = frac_str.trim_end_matches('0');
        format!(
            "{:02}:{:02}:{:02}.{}",
            time.hour(),
            time.minute(),
            time.second(),
            trimmed
        )
    }
}

pub fn force_string_to_date_time(
    value: &str,
    decimal_separator: &str,
    dates: DateSemantics,
) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    match iso_to_date_time(value) {
        Ok(value) => Ok(value),
        Err(_) => excel_to_date_time(float(value, decimal_separator)?, dates),
    }
}

pub fn force_string_to_time(
    value: &str,
    decimal_separator: &str,
) -> Result<NaiveTime, Box<dyn Error + Send + Sync>> {
    match iso_to_time(value) {
        Ok(value) => Ok(value),
        Err(_) => excel_to_time(float(value, decimal_separator)?),
    }
}

pub fn date_time_to_time(datetime: &DateTime<Utc>) -> chrono::NaiveTime {
    datetime.time() // Extract the NaiveTime portion
}

pub fn iso_to_date_time(iso: &str) -> Result<DateTime<Utc>, Box<dyn Error + Send + Sync>> {
    if let Ok(value) = DateTime::parse_from_rfc3339(iso) {
        return Ok(value.with_timezone(&Utc));
    }

    let formats = [
        "%Y-%m-%dT%H:%M:%S%.f%:z", // ISO 8601 with milliseconds and offset
        "%Y-%m-%dT%H:%M:%S%:z",    // ISO 8601 without milliseconds
        "%Y-%m-%dT%H:%M:%S%.fZ",   // ISO 8601 with milliseconds and UTC 'Z'
        "%Y-%m-%dT%H:%M:%SZ",      // ISO 8601 with UTC 'Z'
        "%Y-%m-%dT%H:%M:%S%.f",    // ISO 8601 with milliseconds, no offset
        "%Y-%m-%dT%H:%M:%S",       // ISO 8601 without milliseconds or offset
        "%Y-%m-%d",                // Date only
    ];

    // Try parsing with the formats that include a timezone first
    for &format in &formats {
        if let Ok(dt) = DateTime::parse_from_str(iso, format) {
            return Ok(dt.with_timezone(&Utc));
        }
    }

    // If no timezone information is provided, try parsing as NaiveDate or NaiveDateTime
    if let Ok(date) = NaiveDate::parse_from_str(iso, "%Y-%m-%d") {
        return Ok(Utc.from_utc_datetime(
            &date
                .and_hms_opt(0, 0, 0)
                .ok_or_else(|| format!("Invalid date: {iso}"))?,
        ));
    }

    if let Ok(naive_dt) = NaiveDateTime::parse_from_str(iso, "%Y-%m-%dT%H:%M:%S%.f") {
        return Ok(Utc.from_utc_datetime(&naive_dt));
    }

    if let Ok(naive_dt) = NaiveDateTime::parse_from_str(iso, "%Y-%m-%dT%H:%M:%S") {
        return Ok(Utc.from_utc_datetime(&naive_dt));
    }

    Err(format!("Unsupported date format {iso}").into())
}

pub fn iso_to_time(iso: &str) -> Result<NaiveTime, Box<dyn Error + Send + Sync>> {
    // Define the possible time formats
    let formats = [
        "%H:%M:%S%.f", // Time with fractional seconds
        "%H:%M:%S",    // Time without fractional seconds
    ];

    // Try parsing the input with each format
    for &format in &formats {
        if let Ok(time) = NaiveTime::parse_from_str(iso, format) {
            return Ok(time);
        }
    }

    // If no formats match, return an error
    Err(format!("Unsupported time format: {iso}").into())
}

fn map_value_elementwise<F>(value: Value, op: F) -> Result<Value, Box<dyn Error + Send + Sync>>
where
    F: Fn(Value) -> Result<Value, Box<dyn Error + Send + Sync>>,
{
    match value {
        Value::VecValue(vec) => {
            let mut results = Vec::with_capacity(vec.len());
            for elem in vec {
                results.push(op(elem)?);
            }
            Ok(Value::VecValue(results))
        }
        Value::AreaValue(rows) => {
            let mut result_rows = Vec::with_capacity(rows.len());
            for row in rows {
                let mut result_row = Vec::with_capacity(row.len());
                for elem in row {
                    result_row.push(op(elem)?);
                }
                result_rows.push(result_row);
            }
            Ok(Value::AreaValue(result_rows))
        }
        single => op(single),
    }
}

pub fn year(
    date_time: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    map_value_elementwise(date_time, |v| {
        Ok(Value::I32(codcel_year(v.date_time(value_format)?)?))
    })
}

pub fn month(
    date_time: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    map_value_elementwise(date_time, |v| {
        Ok(Value::I32(codcel_month(v.date_time(value_format)?)?))
    })
}

pub fn day(
    date_time: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    map_value_elementwise(date_time, |v| {
        Ok(Value::I32(codcel_day(v.date_time(value_format)?)?))
    })
}

pub fn hour(
    time: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    map_value_elementwise(time, |v| {
        Ok(Value::I32(codcel_hour(v.time(value_format)?)?))
    })
}

pub fn minute(
    time: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    map_value_elementwise(time, |v| {
        Ok(Value::I32(codcel_minute(v.time(value_format)?)?))
    })
}

pub fn second(
    time: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    map_value_elementwise(time, |v| {
        Ok(Value::I32(codcel_second(v.time(value_format)?)?))
    })
}

pub fn date(
    year: Value,
    month: Value,
    day: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let year = year.i32(value_format)?;
    let month = month.i32(value_format)?;
    let day = day.i32(value_format)?;
    Ok(Value::ChronoDateTime(codcel_date(year, month, day)?))
}

pub fn time(
    hours: Value,
    minutes: Value,
    seconds: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let hours = hours.i32(value_format)?;
    let minutes = minutes.i32(value_format)?;
    let seconds = seconds.i32(value_format)?;
    Ok(Value::Time(codcel_time(hours, minutes, seconds)?))
}

pub fn date_dif(
    start_date: Value,
    end_date: Value,
    unit: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let start_date = start_date.date_time(value_format)?;
    let end_date = end_date.date_time(value_format)?;
    let unit = unit.string(value_format)?;
    Ok(Value::I32(codcel_date_dif(start_date, end_date, unit)?))
}
pub fn today(value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    Ok(Value::ChronoDateTime(codcel_today(value_format)?))
}

pub fn now(value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    Ok(Value::ChronoDateTime(codcel_now(value_format)?))
}

pub fn week_day(
    date_time: Value,
    return_type: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let date_time = date_time.date_time(value_format)?;
    let return_type = return_type.option_i32(value_format)?;
    Ok(Value::I32(codcel_week_day(date_time, return_type)?))
}

pub fn week_num(
    date_time: Value,
    return_type: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let date_time = date_time.date_time(value_format)?;
    let return_type = return_type.option_i32(value_format)?;
    Ok(Value::I32(codcel_week_num(date_time, return_type)?))
}

pub fn iso_week_num(
    date_time: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let date_time = date_time.date_time(value_format)?;
    Ok(Value::I32(codcel_iso_week_num(date_time)?))
}

pub fn date_value(
    date_text: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let date_value = date_text.area_of_string(strict_type_conversion, value_format)?;

    let values: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = date_value
        .into_iter()
        .map(|inner| {
            inner
                .into_iter()
                .map(|text| codcel_date_value(text, value_format.date_semantics()).map(Value::F64))
                .collect::<Result<Vec<Value>, Box<dyn Error + Send + Sync>>>()
        })
        .collect();

    Ok(Value::AreaValue(values?))
}

pub fn time_value(
    time_text: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let time_value = time_text.area_of_string(strict_type_conversion, value_format)?;

    let values: Result<Vec<Vec<Value>>, Box<dyn Error + Send + Sync>> = time_value
        .into_iter()
        .map(|inner| {
            inner
                .into_iter()
                .map(|text| codcel_time_value(text).map(Value::F64))
                .collect::<Result<Vec<Value>, Box<dyn Error + Send + Sync>>>()
        })
        .collect();

    Ok(Value::AreaValue(values?))
}

pub(crate) fn thirty_360_days(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> f64 {
    let y1 = start_date.year();
    let m1 = start_date.month() as i32;
    let mut d1 = start_date.day() as i32;

    let y2 = end_date.year();
    let m2 = end_date.month() as i32;
    let mut d2 = end_date.day() as i32;

    // US 30/360 calculation
    d1 = d1.min(30);
    if d1 == 30 {
        d2 = d2.min(30);
    }

    ((y2 - y1) * 360 + (m2 - m1) * 30 + (d2 - d1)) as f64
}

pub(crate) fn actual_actual_days(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> f64 {
    (end_date - start_date).num_days() as f64
}

pub(crate) fn thirty_e_360_days(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> f64 {
    let y1 = start_date.year();
    let m1 = start_date.month() as i32;
    let mut d1 = start_date.day() as i32;

    let y2 = end_date.year();
    let m2 = end_date.month() as i32;
    let mut d2 = end_date.day() as i32;

    // European 30/360 calculation
    d1 = if d1 == 31 { 30 } else { d1 };
    d2 = if d2 == 31 { 30 } else { d2 };

    ((y2 - y1) * 360 + (m2 - m1) * 30 + (d2 - d1)) as f64
}

pub(crate) fn calculate_360_days(start: DateTime<Utc>, end: DateTime<Utc>) -> i32 {
    let start_year = start.year();
    let start_month = start.month() as i32;
    let mut start_day = start.day() as i32;

    let end_year = end.year();
    let end_month = end.month() as i32;
    let mut end_day = end.day() as i32;

    // Adjust for 30/360 rules
    if start_day == 31 {
        start_day = 30;
    }
    if end_day == 31 && start_day == 30 {
        end_day = 30;
    }

    360 * (end_year - start_year) + 30 * (end_month - start_month) + (end_day - start_day)
}

// Helper function to calculate actual days between dates
pub(crate) fn calculate_actual_days(start: DateTime<Utc>, end: DateTime<Utc>) -> i32 {
    (end - start).num_days() as i32
}

// Helper function to check if a year is a leap year
pub(crate) fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub(crate) fn calculate_30_360_days(start_date: NaiveDate, end_date: NaiveDate) -> i32 {
    let mut d1 = start_date.day() as i32;
    let mut d2 = end_date.day() as i32;
    let m1 = start_date.month() as i32;
    let m2 = end_date.month() as i32;
    let y1 = start_date.year();
    let y2 = end_date.year();

    if d1 == 31 {
        d1 = 30;
    }
    if d2 == 31 && d1 == 30 {
        d2 = 30;
    }

    360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)
}

pub(crate) fn calculate_30e_360_days(start_date: NaiveDate, end_date: NaiveDate) -> i32 {
    let d1 = std::cmp::min(start_date.day() as i32, 30);
    let d2 = std::cmp::min(end_date.day() as i32, 30);
    let m1 = start_date.month() as i32;
    let m2 = end_date.month() as i32;
    let y1 = start_date.year();
    let y2 = end_date.year();

    360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)
}

pub(crate) fn is_last_day_of_month(date: NaiveDate) -> bool {
    date.day() == get_days_in_month(date.year(), date.month())
}

pub(crate) fn get_days_in_month(year: i32, month: u32) -> u32 {
    match month {
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 31,
    }
}

pub(crate) fn is_last_day_of_february(date: DateTime<Utc>) -> bool {
    let month = date.month();
    let day = date.day();

    if month != 2 {
        return false;
    }

    let year = date.year();
    let is_leap_year = NaiveDate::from_ymd_opt(year, 1, 1)
        .map(|d| d.leap_year())
        .unwrap_or(false);

    (is_leap_year && day == 29) || (!is_leap_year && day == 28)
}

pub(crate) fn get_days_between(start: &DateTime<Utc>, end: &DateTime<Utc>, basis: i32) -> i32 {
    match basis {
        0 => {
            // US 30/360
            let s_day = start.day();
            let s_month = start.month();
            let s_year = start.year();
            let e_day = end.day();
            let e_month = end.month();
            let e_year = end.year();

            let mut adj_s_day = s_day;
            let mut adj_e_day = e_day;

            // Handle special cases for US 30/360
            if s_month == 2 && is_last_day_of_february(*start) {
                adj_s_day = 30;
            }
            if e_month == 2 && is_last_day_of_february(*end) {
                adj_e_day = 30;
            }
            if adj_s_day == 31 {
                adj_s_day = 30;
            }
            if adj_e_day == 31 && adj_s_day == 30 {
                adj_e_day = 30;
            }

            (e_year - s_year) * 360
                + (e_month as i32 - s_month as i32) * 30
                + (adj_e_day as i32 - adj_s_day as i32)
        }
        1 => {
            // Actual/Actual
            (*end - *start).num_days() as i32
        }
        2 | 3 => {
            // Actual days
            (*end - *start).num_days() as i32
        }
        4 => {
            // European 30/360
            let s_day = start.day();
            let s_month = start.month();
            let s_year = start.year();
            let e_day = end.day();
            let e_month = end.month();
            let e_year = end.year();

            let s_day = if s_day == 31 { 30 } else { s_day };
            let e_day = if e_day == 31 { 30 } else { e_day };

            (e_year - s_year) * 360
                + (e_month as i32 - s_month as i32) * 30
                + (e_day as i32 - s_day as i32)
        }
        _ => unreachable!(),
    }
}

pub fn days(
    end_date: Value,
    start_date: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_datetime_datetime_to_int(
        end_date,
        start_date,
        strict_type_conversion,
        value_format,
        "DAYS",
        codcel_days,
    )
}

pub fn days_360(
    start_date: Value,
    end_date: Value,
    use_european_method: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_datetime_datetime_opt_bool_to_int(
        start_date,
        end_date,
        use_european_method,
        strict_type_conversion,
        value_format,
        "DAYS360",
        codcel_days_360,
    )
}

pub fn networkdays(
    start_date: Value,
    end_date: Value,
    holidays: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let start = start_date.date_time(value_format)?;
    let end = end_date.date_time(value_format)?;
    let hols = match holidays.option_area_of_value()? {
        None => None,
        Some(area) => {
            let mut dates = Vec::new();
            for row in area {
                for val in row {
                    if !matches!(val, Value::None) {
                        dates.push(val.date_time(value_format)?);
                    }
                }
            }
            if dates.is_empty() {
                None
            } else {
                Some(dates)
            }
        }
    };
    Ok(Value::I32(codcel_networkdays(start, end, hols)?))
}

pub fn networkdays_intl(
    start_date: Value,
    end_date: Value,
    weekend: Value,
    holidays: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let start = start_date.date_time(value_format)?;
    let end = end_date.date_time(value_format)?;

    let weekend_mask = match &weekend {
        Value::None => parse_weekend_mask(1)?,
        Value::String(s) => parse_weekend_string(s)?,
        _ => {
            let code = weekend.i32(value_format)?;
            parse_weekend_mask(code)?
        }
    };

    let hols = match holidays.option_area_of_value()? {
        None => None,
        Some(area) => {
            let mut dates = Vec::new();
            for row in area {
                for val in row {
                    if !matches!(val, Value::None) {
                        dates.push(val.date_time(value_format)?);
                    }
                }
            }
            if dates.is_empty() {
                None
            } else {
                Some(dates)
            }
        }
    };
    Ok(Value::I32(codcel_networkdays_intl(
        start,
        end,
        weekend_mask,
        hols,
    )?))
}

pub fn workday(
    start_date: Value,
    days: Value,
    holidays: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let start = start_date.date_time(value_format)?;
    let d = days.i32(value_format)?;
    let hols = match holidays.option_area_of_value()? {
        None => None,
        Some(area) => {
            let mut dates = Vec::new();
            for row in area {
                for val in row {
                    if !matches!(val, Value::None) {
                        dates.push(val.date_time(value_format)?);
                    }
                }
            }
            if dates.is_empty() {
                None
            } else {
                Some(dates)
            }
        }
    };
    Ok(Value::ChronoDateTime(codcel_workday(start, d, hols)?))
}

pub fn workday_intl(
    start_date: Value,
    days: Value,
    weekend: Value,
    holidays: Value,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let start = start_date.date_time(value_format)?;
    let d = days.i32(value_format)?;

    let weekend_mask = match &weekend {
        Value::None => parse_weekend_mask(1)?,
        Value::String(s) => parse_weekend_string(s)?,
        _ => {
            let code = weekend.i32(value_format)?;
            parse_weekend_mask(code)?
        }
    };

    let hols = match holidays.option_area_of_value()? {
        None => None,
        Some(area) => {
            let mut dates = Vec::new();
            for row in area {
                for val in row {
                    if !matches!(val, Value::None) {
                        dates.push(val.date_time(value_format)?);
                    }
                }
            }
            if dates.is_empty() {
                None
            } else {
                Some(dates)
            }
        }
    };
    Ok(Value::ChronoDateTime(codcel_workday_intl(
        start,
        d,
        weekend_mask,
        hols,
    )?))
}

pub fn e_date(
    start_date: Value,
    months: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let start_date = start_date.date_time(value_format)?;
    let months = months.i32(value_format)?;

    Ok(Value::ChronoDateTime(codcel_e_date(start_date, months)?))
}

pub fn eo_month(
    start_date: Value,
    months: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let start_date = start_date.date_time(value_format)?;
    let months = months.i32(value_format)?;

    Ok(Value::ChronoDateTime(codcel_eo_month(start_date, months)?))
}

pub fn year_frac(
    start_date: Value,
    end_date: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let start_date = start_date.date_time(value_format)?;
    let end_date = end_date.date_time(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_year_frac(start_date, end_date, basis)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    /// Excel's own mapping, spot-checked against what Excel displays for each
    /// serial. Serial 60 is the phantom 29 February 1900 and has no true date.
    #[test]
    fn excel_1900_serials_match_excel() {
        let cases = [
            (1.0, "1900-01-01"),
            (59.0, "1900-02-28"),
            (61.0, "1900-03-01"),
            (1462.0, "1904-01-01"),
            (45066.0, "2023-05-20"),
        ];
        for (serial, expected) in cases {
            let dt = excel_to_date_time(serial, DateSemantics::EXCEL_1900).unwrap();
            assert_eq!(
                dt.format("%Y-%m-%d").to_string(),
                expected,
                "serial {serial}"
            );
        }
    }

    /// Regression: the forward conversion used to branch on `> 59.0` while the
    /// inverse branched on `>= 60.0`. The two agree on integers but not on
    /// fractions, so serial 59.5 decoded to 1900-02-27 12:00 — a day early.
    #[test]
    fn fractional_serial_below_sixty_is_not_shifted() {
        let dt = excel_to_date_time(59.5, DateSemantics::EXCEL_1900).unwrap();
        assert_eq!(dt.format("%Y-%m-%d %H:%M").to_string(), "1900-02-28 12:00");
    }

    /// Regression: the inverse branched on `>= 60.0` against a *day count*
    /// rather than a serial, so 1900-03-01 encoded to 60 instead of 61 and did
    /// not round-trip.
    #[test]
    fn first_of_march_1900_round_trips() {
        let dt = excel_to_date_time(61.0, DateSemantics::EXCEL_1900).unwrap();
        assert_eq!(
            date_time_to_excel(&dt, DateSemantics::EXCEL_1900).unwrap(),
            61.0
        );
    }

    /// Serial 59 and serial 60 both land on 1900-02-28: `chrono` cannot
    /// represent Excel's fictitious 1900-02-29, so the phantom day aliases onto
    /// the real one. Pinned deliberately — this is a documented limitation, not
    /// an accident.
    #[test]
    fn phantom_leap_day_aliases_onto_28_february() {
        let fifty_nine = excel_to_date_time(59.0, DateSemantics::EXCEL_1900).unwrap();
        let sixty = excel_to_date_time(60.0, DateSemantics::EXCEL_1900).unwrap();
        assert_eq!(fifty_nine, sixty);
    }

    /// 1904-01-01 is serial 1462 in Excel's 1900 system. This engine has no 1904
    /// convention — the loader rebases such workbooks before their values get
    /// here — but that 1462 relationship is what the rebase depends on, so pin it
    /// against a real calendar date rather than against another copy of the
    /// constant.
    #[test]
    fn the_1904_epoch_sits_1462_serials_into_the_1900_system() {
        let epoch = excel_to_date_time(1462.0, DateSemantics::EXCEL_1900).unwrap();
        assert_eq!(epoch.format("%Y-%m-%d").to_string(), "1904-01-01");
        assert_eq!(
            date_time_to_excel(&epoch, DateSemantics::EXCEL_1900).unwrap(),
            1462.0
        );
    }

    /// Turning the Lotus bug off re-bases the serial system rather than making
    /// dates "more correct": every date from 1900-03-01 onward moves one day.
    #[test]
    fn astronomical_1900_shifts_dates_after_february_1900() {
        let excel = excel_to_date_time(45066.0, DateSemantics::EXCEL_1900).unwrap();
        let astronomical = excel_to_date_time(45066.0, DateSemantics::ASTRONOMICAL_1900).unwrap();
        assert_eq!(excel.format("%Y-%m-%d").to_string(), "2023-05-20");
        assert_eq!(astronomical.format("%Y-%m-%d").to_string(), "2023-05-21");
    }

    /// Every convention must be its own inverse. Serial 60 is excluded: it has
    /// no true date, so it cannot round-trip by construction.
    #[test]
    fn serial_round_trips_under_every_convention() {
        let conventions = [DateSemantics::EXCEL_1900, DateSemantics::ASTRONOMICAL_1900];
        for dates in conventions {
            for serial in [1.0, 59.0, 59.5, 61.0, 1462.0, 45066.0, 45066.75] {
                let dt = excel_to_date_time(serial, dates).unwrap();
                let back = date_time_to_excel(&dt, dates).unwrap();
                assert!(
                    (back - serial).abs() < 1e-9,
                    "{dates:?} serial {serial} round-tripped to {back}"
                );
            }
        }
    }

    #[test]
    fn test_excel_to_date_time() {
        let excel_date = 45597.0;
        let expected_datetime = Utc
            .with_ymd_and_hms(2024, 11, 1, 0, 0, 0)
            .single()
            .expect("valid test date");

        match excel_to_date_time(excel_date, DateSemantics::EXCEL_1900) {
            Ok(datetime) => assert_eq!(datetime, expected_datetime),
            Err(e) => panic!("Failed to convert Excel date to DateTime<Utc>: {e}"),
        }
    }

    #[test]
    fn test_date_time_to_excel() {
        let test_date = DateTime::<Utc>::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2024, 11, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            Utc,
        );
        let excel_date = date_time_to_excel(&test_date, DateSemantics::EXCEL_1900).unwrap();
        assert!((45597.0 - excel_date).abs() < 1e-6);
    }

    #[test]
    fn test_time_round_trip_precision() {
        // 0.0785 = rate of 7.85% encoded as time fraction
        // 0.0785 * 86400 = 6782.4 seconds (has fractional seconds)
        let original = 0.0785_f64;
        let time = excel_to_time(original).unwrap();
        let iso = time_to_iso(&time);
        let parsed = iso_to_time(&iso).unwrap();
        let back = time_to_excel(&parsed).unwrap();
        assert!(
            (back - original).abs() < 1e-10,
            "Round-trip failed: {original} -> {iso} -> {back} (diff: {})",
            (back - original).abs()
        );
    }

    #[test]
    fn test_time_round_trip_whole_seconds() {
        // 0.05 = rate of 5% encoded as time fraction
        // 0.05 * 86400 = 4320.0 seconds (exact whole seconds)
        let original = 0.05_f64;
        let time = excel_to_time(original).unwrap();
        let back = time_to_excel(&time).unwrap();
        assert!(
            (back - original).abs() < 1e-10,
            "Round-trip failed: {original} -> {back}"
        );
    }
}
