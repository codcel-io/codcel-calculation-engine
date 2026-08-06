// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_time_base::{get_days_in_month, is_last_day_of_month};
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use std::error::Error;

// Helper function for 30/360 US (NASD) day count
pub(crate) fn calculate_30_360_days_a(start: DateTime<Utc>, end: DateTime<Utc>) -> f64 {
    let mut d1 = start.day() as i32;
    let mut d2 = end.day() as i32;
    let m1 = start.month() as i32;
    let m2 = end.month() as i32;
    let y1 = start.year();
    let y2 = end.year();

    // 30/360 US adjustments
    if d1 == 31 {
        d1 = 30;
    }
    if d2 == 31 && d1 >= 30 {
        d2 = 30;
    }

    (360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)) as f64
}

// Helper function for 30/360 European day count
pub(crate) fn calculate_30_360_european(start: DateTime<Utc>, end: DateTime<Utc>) -> f64 {
    let d1 = start.day().min(30) as i32;
    let d2 = end.day().min(30) as i32;
    let m1 = start.month() as i32;
    let m2 = end.month() as i32;
    let y1 = start.year();
    let y2 = end.year();

    (360 * (y2 - y1) + 30 * (m2 - m1) + (d2 - d1)) as f64
}

/// Calculate the number of days between two dates based on the basis
pub(crate) fn calculate_days_between(
    start_date: &DateTime<Utc>,
    end_date: &DateTime<Utc>,
    basis: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    match basis {
        0 => {
            // US (NASD) 30/360
            Ok(calculate_30_360_days_a(*start_date, *end_date))
        }
        1 => {
            // Actual/actual
            Ok((end_date.signed_duration_since(*start_date)).num_days() as f64)
        }
        2 => {
            // Actual/360
            Ok((end_date.signed_duration_since(*start_date)).num_days() as f64)
        }
        3 => {
            // Actual/365
            Ok((end_date.signed_duration_since(*start_date)).num_days() as f64)
        }
        4 => {
            // European 30/360
            Ok(calculate_30_360_european(*start_date, *end_date))
        }
        _ => Err(format!("Invalid basis: {basis}").into()),
    }
}

pub(crate) fn get_previous_coupon_date(
    settlement: NaiveDate,
    maturity: NaiveDate,
    frequency: i32,
) -> Result<NaiveDate, Box<dyn Error + Send + Sync>> {
    get_previous_coupon_date_eom(settlement, maturity, frequency, is_last_day_of_month(maturity))
}

pub(crate) fn get_previous_coupon_date_eom(
    settlement: NaiveDate,
    maturity: NaiveDate,
    frequency: i32,
    eom: bool,
) -> Result<NaiveDate, Box<dyn Error + Send + Sync>> {
    let months_between = 12 / frequency;
    let mut current_date = maturity;

    while current_date > settlement {
        // Move back by the coupon period
        let year = current_date.year()
            - (if current_date.month() <= months_between as u32 {
                1
            } else {
                0
            });
        let month = if current_date.month() <= months_between as u32 {
            12 - (months_between as u32 - current_date.month())
        } else {
            current_date.month() - months_between as u32
        };

        let day = if eom {
            get_days_in_month(year, month)
        } else {
            std::cmp::min(current_date.day(), get_days_in_month(year, month))
        };

        current_date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date calculation")?;
    }

    Ok(current_date)
}

pub(crate) fn get_next_coupon_date_coup_num(
    settlement: NaiveDate,
    maturity: NaiveDate,
    frequency: i32,
) -> Result<NaiveDate, Box<dyn Error + Send + Sync>> {
    let months_between = 12 / frequency;
    let mut current = maturity;
    let mut previous = None;

    while current > settlement {
        previous = Some(current);

        // Move back by one period
        let new_month = if current.month() <= months_between as u32 {
            12 - (months_between - current.month() as i32) as u32
        } else {
            current.month() - months_between as u32
        };

        let new_year = current.year()
            - if current.month() <= months_between as u32 {
                1
            } else {
                0
            };

        if let Some(new_date) = NaiveDate::from_ymd_opt(
            new_year,
            new_month,
            std::cmp::min(current.day(), get_days_in_month(new_year, new_month)),
        ) {
            current = new_date;
        } else {
            return Err("COUPNUM: Invalid date calculation".into());
        }
    }

    previous.ok_or_else(|| "COUPNUM: Could not find next coupon date".into())
}

pub(crate) fn get_next_coupon_date(
    settlement: NaiveDate,
    maturity: NaiveDate,
    frequency: i32,
) -> Result<NaiveDate, Box<dyn Error + Send + Sync>> {
    let months_between = 12 / frequency;
    let mut current_date = maturity;
    let mut next_date = maturity;

    // Walk backward from maturity, tracking the last position that was > settlement
    while current_date > settlement {
        next_date = current_date;

        // Move back by the coupon period
        let year = current_date.year()
            - (if current_date.month() <= months_between as u32 {
                1
            } else {
                0
            });
        let month = if current_date.month() <= months_between as u32 {
            12 - (months_between as u32 - current_date.month())
        } else {
            current_date.month() - months_between as u32
        };

        let day = std::cmp::min(current_date.day(), get_days_in_month(year, month));

        current_date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date calculation")?;
    }

    Ok(next_date)
}

/// EOM-aware version of get_next_coupon_date.
/// When maturity falls on end-of-month, all coupon dates snap to their month-end.
pub(crate) fn get_next_coupon_date_eom(
    settlement: NaiveDate,
    maturity: NaiveDate,
    frequency: i32,
) -> Result<NaiveDate, Box<dyn Error + Send + Sync>> {
    let months_between = 12 / frequency;
    let eom = is_last_day_of_month(maturity);
    let mut current_date = maturity;
    let mut next_date = maturity;

    while current_date > settlement {
        next_date = current_date;

        let year = current_date.year()
            - (if current_date.month() <= months_between as u32 {
                1
            } else {
                0
            });
        let month = if current_date.month() <= months_between as u32 {
            12 - (months_between as u32 - current_date.month())
        } else {
            current_date.month() - months_between as u32
        };

        let day = if eom {
            get_days_in_month(year, month)
        } else {
            std::cmp::min(current_date.day(), get_days_in_month(year, month))
        };

        current_date =
            NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date calculation")?;
    }

    Ok(next_date)
}

pub(crate) fn pmt(
    rate: f64,
    nper: i32,
    pv: f64,
    fv: f64,
    payment_type: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if rate == 0.0 {
        return Ok(-(pv + fv) / nper as f64);
    }

    let pvif = (1.0 + rate).powi(nper);
    let pmt = rate * pv * (pvif + fv / pv) / (pvif - 1.0);

    if payment_type == 1 {
        Ok(-pmt / (1.0 + rate))
    } else {
        Ok(-pmt)
    }
}

pub(crate) fn ipmt(
    rate: f64,
    per: i32,
    nper: i32,
    pv: f64,
    fv: f64,
    payment_type: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if per < 1 || per > nper {
        return Err("CUMIPMT: Period must be between 1 and nper".into());
    }

    let pmt = pmt(rate, nper, pv, fv, payment_type)?;

    if payment_type == 1 {
        if per == 1 {
            return Ok(0.0);
        }

        // For beginning-of-period payments:
        // Calculate interest based on the balance at the start of the period,
        // which is affected by the payment made at the beginning
        let balance_factor = (1.0 + rate).powi(per - 2);
        let adjusted_balance =
            pv * balance_factor + pmt * (balance_factor * (1.0 + rate) - 1.0) / rate;

        Ok(-adjusted_balance * rate)
    } else {
        // For end-of-period payments (unchanged)
        let previous_balance =
            pv * (1.0 + rate).powi(per - 1) + pmt * ((1.0 + rate).powi(per - 1) - 1.0) / rate;
        Ok(-previous_balance * rate)
    }
}

pub(crate) fn pmt_cum_princ(rate: f64, nper: i32, pv: f64, fv: f64, payment_type: i32) -> f64 {
    if rate == 0.0 {
        return -(pv + fv) / nper as f64;
    }

    let pvif = (1.0 + rate).powi(nper);
    let pmt = rate * pv * (pvif + fv / pv) / (pvif - 1.0);

    if payment_type == 1 {
        -pmt / (1.0 + rate)
    } else {
        -pmt
    }
}

pub(crate) fn ipmt_cum_princ(rate: f64, per: i32, nper: i32, pv: f64, payment_type: i32) -> f64 {
    if per < 1 || per > nper {
        return 0.0; // Error case
    }

    let payment = pmt_cum_princ(rate, nper, pv, 0.0, payment_type);

    if payment_type == 1 {
        if per == 1 {
            return 0.0;
        }

        // For beginning-of-period payments
        let balance_factor = (1.0 + rate).powi(per - 2);
        let adjusted_balance =
            pv * balance_factor + payment * (balance_factor * (1.0 + rate) - 1.0) / rate;

        -adjusted_balance * rate
    } else {
        // For end-of-period payments
        let previous_balance =
            pv * (1.0 + rate).powi(per - 1) + payment * ((1.0 + rate).powi(per - 1) - 1.0) / rate;
        -previous_balance * rate
    }
}

pub(crate) fn ppmt_cum_princ(rate: f64, per: i32, nper: i32, pv: f64, payment_type: i32) -> f64 {
    let payment = pmt_cum_princ(rate, nper, pv, 0.0, payment_type);
    let interest = ipmt_cum_princ(rate, per, nper, pv, payment_type);
    payment - interest
}
