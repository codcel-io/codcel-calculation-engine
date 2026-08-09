// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Datelike, TimeZone, Utc};
use std::error::Error;

/// Calculates the price of a security with an odd (short or long) first coupon period.
///
/// This follows Excel's `ODDFPRICE` function per the MS-OI29500 / ECMA-376 specification.
///
/// # Arguments
/// * `settlement` - Settlement date of the security.
/// * `maturity` - Maturity date of the security.
/// * `issue` - Issue date of the security.
/// * `first_coupon` - Date of the first coupon payment.
/// * `rate` - Annual coupon rate.
/// * `yield_rate` - Annual yield expected by the investor.
/// * `redemption` - Redemption value per $100 face value.
/// * `frequency` - Number of coupon payments per year (1, 2, or 4).
/// * `basis` - Optional day-count basis (0-4).
#[allow(clippy::too_many_arguments)]
pub fn codcel_odd_f_price(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    issue: DateTime<Utc>,
    first_coupon: DateTime<Utc>,
    rate: f64,
    yield_rate: f64,
    redemption: f64,
    frequency: i32,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let basis = basis.unwrap_or(0);
    let freq = frequency as f64;
    let coupon = 100.0 * rate / freq;
    let yld = yield_rate / freq;

    let months_per_period = 12 / frequency;

    // Find the previous quasi-coupon date before first_coupon
    let prev_quasi = add_months(first_coupon, -months_per_period);

    if issue >= prev_quasi {
        // SHORT first coupon period
        odd_f_price_short(
            settlement,
            maturity,
            issue,
            first_coupon,
            coupon,
            yld,
            redemption,
            frequency,
            basis,
        )
    } else {
        // LONG first coupon period
        odd_f_price_long(
            settlement,
            maturity,
            issue,
            first_coupon,
            coupon,
            yld,
            redemption,
            frequency,
            basis,
        )
    }
}

/// ODDFPRICE for short first coupon period.
///
/// Formula (from MS-OI29500):
///   price = redemption / (1+yld)^(N-1+DSC/E)
///         + 100*(rate/freq)*(DFC/E) / (1+yld)^(DSC/E)
///         + SUM[k=2..N]{ 100*(rate/freq) / (1+yld)^(k-1+DSC/E) }
///         - 100*(rate/freq)*(A/E)
#[allow(clippy::too_many_arguments)]
fn odd_f_price_short(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    issue: DateTime<Utc>,
    first_coupon: DateTime<Utc>,
    coupon: f64, // = 100 * rate / freq
    yld: f64,    // = yield_rate / freq
    redemption: f64,
    frequency: i32,
    basis: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let months_per_period = 12 / frequency;

    // Previous quasi-coupon date (start of the standard coupon period)
    let pcd = add_months(first_coupon, -months_per_period);

    // E = standard coupon period length (denominator)
    let e = coupon_period_days(pcd, first_coupon, basis, frequency);

    // DSC = days from settlement to first coupon (numerator day count)
    let dsc = day_count(settlement, first_coupon, basis);

    // A = days from issue to settlement (accrued days from the odd first period start)
    let a = day_count(issue, settlement, basis);

    // DFC = days from issue to first coupon (the actual odd first period)
    let dfc = day_count(issue, first_coupon, basis);

    // N = number of coupon periods from first coupon to maturity (inclusive of first_coupon)
    let n = count_coupon_periods(first_coupon, maturity, frequency);

    let dsc_e = dsc / e;

    // Term 1: PV of redemption
    let term1 = redemption / crate::portable_math::powf(1.0 + yld, (n - 1) as f64 + dsc_e);

    // Term 2: PV of the odd (short) first coupon
    let term2 = coupon * (dfc / e) / crate::portable_math::powf(1.0 + yld, dsc_e);

    // Term 3: PV of regular coupons (k=2 to N)
    let mut term3 = 0.0;
    for k in 2..=n {
        term3 += coupon / crate::portable_math::powf(1.0 + yld, (k - 1) as f64 + dsc_e);
    }

    // Term 4: Accrued interest
    let term4 = coupon * (a / e);

    Ok(term1 + term2 + term3 - term4)
}

/// ODDFPRICE for long first coupon period.
///
/// Formula (from MS-OI29500, adapted to match PRICE exponent convention):
///   price = redemption / (1+yld)^(N-1+Nq+DSC/E)
///         + 100*(rate/freq)*SUM[i=1..NC](DC_i/NL_i) / (1+yld)^(Nq+DSC/E)
///         + SUM[k=2..N]{ 100*(rate/freq) / (1+yld)^(k-1+Nq+DSC/E) }
///         - 100*(rate/freq)*SUM[i=1..NC](A_i/NL_i)
#[allow(clippy::too_many_arguments)]
fn odd_f_price_long(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    issue: DateTime<Utc>,
    first_coupon: DateTime<Utc>,
    coupon: f64, // = 100 * rate / freq
    yld: f64,    // = yield_rate / freq
    redemption: f64,
    frequency: i32,
    basis: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let months_per_period = 12 / frequency;

    // Generate quasi-coupon dates by stepping backward from first_coupon
    // until we reach or pass the issue date
    let mut quasi_dates = vec![first_coupon];
    loop {
        let prev = add_months(*quasi_dates.last().unwrap(), -months_per_period);
        quasi_dates.push(prev);
        if prev <= issue {
            break;
        }
    }
    // quasi_dates is [first_coupon, qd1, qd2, ..., qdN] where qdN <= issue
    // Reverse so it goes from earliest to latest
    quasi_dates.reverse();

    // NC = number of quasi-coupon periods in the odd period
    let nc = quasi_dates.len() - 1;

    // Find which quasi-coupon period contains the settlement date
    // quasi_dates[0] is the earliest (at or before issue), quasi_dates[nc] = first_coupon

    // Compute DC_i, NL_i, A_i for each quasi-coupon period
    let mut dc_nl_sum = 0.0; // sum of DC_i / NL_i (for the odd coupon fraction)
    let mut a_nl_sum = 0.0; // sum of A_i / NL_i (for accrued interest)

    for i in 0..nc {
        let period_start = quasi_dates[i];
        let period_end = quasi_dates[i + 1];

        // NL_i = standard length of this quasi-coupon period (denominator)
        let nl_i = coupon_period_days(period_start, period_end, basis, frequency);

        // DC_i: for the first quasi-period (i=0), DC = days from issue to end of period
        // For subsequent full quasi-periods, DC = NL_i (full period)
        let dc_i = if i == 0 {
            day_count(issue, period_end, basis)
        } else {
            nl_i
        };

        dc_nl_sum += dc_i / nl_i;

        // A_i: accrued interest fraction
        // For periods entirely before settlement: A_i = DC_i (full accrued)
        // For the period containing settlement: A_i = days from period start to settlement
        //   (but capped by DC_i for the first period where issue may be after period_start)
        // For periods entirely after settlement: A_i = 0
        let a_i = if settlement >= period_end {
            // Period is entirely before settlement - fully accrued
            dc_i
        } else if settlement > period_start {
            // Settlement falls within this period
            if i == 0 {
                // First quasi-period: accrued from issue to settlement
                day_count(issue, settlement, basis)
            } else {
                day_count(period_start, settlement, basis)
            }
        } else {
            // Period is entirely after settlement
            0.0
        };

        a_nl_sum += a_i / nl_i;
    }

    // Find the next quasi-coupon date after settlement (NQD)
    // and count whole quasi-coupon periods between NQD and first_coupon (Nq)
    let mut nqd_idx = 0;
    for (i, &qd) in quasi_dates.iter().enumerate().take(nc + 1) {
        if qd > settlement {
            nqd_idx = i;
            break;
        }
        // If settlement falls exactly on a quasi-date, next is the following one
        if qd == settlement && i < nc {
            nqd_idx = i + 1;
            break;
        }
    }

    let nqd = quasi_dates[nqd_idx]; // next quasi-coupon date after settlement

    // DSC = days from settlement to next quasi-coupon date
    let dsc = day_count(settlement, nqd, basis);

    // E = standard coupon period length for the period containing settlement (denominator)
    let e = if nqd_idx > 0 {
        coupon_period_days(quasi_dates[nqd_idx - 1], nqd, basis, frequency)
    } else {
        coupon_period_days(quasi_dates[0], quasi_dates[1], basis, frequency)
    };

    // Nq = number of WHOLE quasi-coupon periods between NQD and first_coupon
    let nq = (nc as i32) - (nqd_idx as i32);

    // N = number of coupon periods from first coupon to maturity
    let n = count_coupon_periods(first_coupon, maturity, frequency);

    let dsc_e = dsc / e;

    // Term 1: PV of redemption
    let term1 =
        redemption / crate::portable_math::powf(1.0 + yld, (n - 1) as f64 + nq as f64 + dsc_e);

    // Term 2: PV of the odd (long) first coupon
    let term2 = coupon * dc_nl_sum / crate::portable_math::powf(1.0 + yld, nq as f64 + dsc_e);

    // Term 3: PV of regular coupons (k=2 to N)
    let mut term3 = 0.0;
    for k in 2..=n {
        term3 += coupon / crate::portable_math::powf(1.0 + yld, (k - 1) as f64 + nq as f64 + dsc_e);
    }

    // Term 4: Accrued interest
    let term4 = coupon * a_nl_sum;

    Ok(term1 + term2 + term3 - term4)
}

/// Count coupon periods from `start` to `maturity` (inclusive of both endpoints).
fn count_coupon_periods(start: DateTime<Utc>, maturity: DateTime<Utc>, frequency: i32) -> i32 {
    let months_per_period = 12 / frequency;
    let mut count = 0;
    let mut current = start;
    while current <= maturity {
        count += 1;
        current = add_months(current, months_per_period);
    }
    count
}

/// Calculate day count between two dates based on the specified basis (numerator).
/// For basis 0,4: uses 30/360 day counting.
/// For basis 1,2,3: uses actual calendar days.
fn day_count(start: DateTime<Utc>, end: DateTime<Utc>, basis: i32) -> f64 {
    match basis {
        0 => days_30_360_us(start, end),
        1..=3 => (end - start).num_days() as f64,
        4 => days_30_360_eu(start, end),
        _ => (end - start).num_days() as f64,
    }
}

/// Calculate the standard coupon period length (denominator NL_i / E).
/// For basis 0,4: 360/freq (fixed).
/// For basis 1: actual days between the quasi-coupon period dates.
/// For basis 2: 360/freq (fixed).
/// For basis 3: 365/freq (fixed).
fn coupon_period_days(start: DateTime<Utc>, end: DateTime<Utc>, basis: i32, frequency: i32) -> f64 {
    match basis {
        0 | 4 => 360.0 / frequency as f64,
        1 => (end - start).num_days() as f64,
        2 => 360.0 / frequency as f64,
        3 => 365.0 / frequency as f64,
        _ => (end - start).num_days() as f64,
    }
}

/// US (NASD) 30/360 day count.
fn days_30_360_us(start: DateTime<Utc>, end: DateTime<Utc>) -> f64 {
    let mut d1 = start.day() as i32;
    let mut d2 = end.day() as i32;
    let m1 = start.month() as i32;
    let m2 = end.month() as i32;
    let y1 = start.year();
    let y2 = end.year();

    // Standard NASD rules for bond pricing
    if d1 == 31 {
        d1 = 30;
    }
    if d2 == 31 && d1 >= 30 {
        d2 = 30;
    }

    ((y2 - y1) * 360 + (m2 - m1) * 30 + (d2 - d1)) as f64
}

/// European 30/360 day count.
fn days_30_360_eu(start: DateTime<Utc>, end: DateTime<Utc>) -> f64 {
    let d1 = start.day().min(30) as i32;
    let d2 = end.day().min(30) as i32;
    let m1 = start.month() as i32;
    let m2 = end.month() as i32;
    let y1 = start.year();
    let y2 = end.year();

    ((y2 - y1) * 360 + (m2 - m1) * 30 + (d2 - d1)) as f64
}

/// Add months to a date, preserving end-of-month behavior.
fn add_months(date: DateTime<Utc>, months: i32) -> DateTime<Utc> {
    let mut year = date.year();
    let mut month = date.month() as i32 + months;

    while month > 12 {
        month -= 12;
        year += 1;
    }
    while month < 1 {
        month += 12;
        year -= 1;
    }

    let original_day = date.day();
    let max_day = days_in_month(year, month as u32);

    // Preserve end-of-month: if original was last day of its month, use last day of target
    let day = if original_day >= days_in_month(date.year(), date.month()) {
        max_day
    } else {
        original_day.min(max_day)
    };

    Utc.with_ymd_and_hms(year, month as u32, day, 0, 0, 0)
        .unwrap()
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn dt(y: i32, m: u32, d: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_ofp_semi_std() {
        // Settlement: 2023-05-01, Maturity: 2028-09-01, Issue: 2023-02-01, First: 2023-09-01
        // Rate: 0.075, Yield: 0.06, Redemption: 100, Freq: 2, Basis: 0
        // Expected: 106.73614795309982
        let result = codcel_odd_f_price(
            dt(2023, 5, 1),
            dt(2028, 9, 1),
            dt(2023, 2, 1),
            dt(2023, 9, 1),
            0.075,
            0.06,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("ofp_semi_std: {result}");
        assert!((result - 106.73614795309982).abs() < 0.000001);
    }

    #[test]
    fn test_ofp_low_coup() {
        // Settlement: 2022-04-01, Maturity: 2024-07-01, Issue: 2022-02-01, First: 2022-07-01
        // Rate: 0.02, Yield: 0.05, Redemption: 100, Freq: 2, Basis: 0
        // Expected: 93.68901442820814
        let result = codcel_odd_f_price(
            dt(2022, 4, 1),
            dt(2024, 7, 1),
            dt(2022, 2, 1),
            dt(2022, 7, 1),
            0.02,
            0.05,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("ofp_low_coup: {result}");
        assert!((result - 93.68901442820814).abs() < 0.000001);
    }

    #[test]
    fn test_ofp_par_bond() {
        // Settlement: 2024-01-01, Maturity: 2027-07-01, Issue: 2023-10-01, First: 2024-07-01
        // Rate: 0.05, Yield: 0.05, Redemption: 100, Freq: 2, Basis: 0
        // Expected: 99.96951219512198
        let result = codcel_odd_f_price(
            dt(2024, 1, 1),
            dt(2027, 7, 1),
            dt(2023, 10, 1),
            dt(2024, 7, 1),
            0.05,
            0.05,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("ofp_par_bond: {result}");
        assert!((result - 99.96951219512198).abs() < 0.000001);
    }

    #[test]
    fn test_ofp_zero_yld() {
        // Settlement: 2022-04-01, Maturity: 2027-07-01, Issue: 2022-02-15, First: 2022-07-01
        // Rate: 0.05, Yield: 0.0, Redemption: 105, Freq: 2, Basis: 0
        // Expected: 131.25
        let result = codcel_odd_f_price(
            dt(2022, 4, 1),
            dt(2027, 7, 1),
            dt(2022, 2, 15),
            dt(2022, 7, 1),
            0.05,
            0.0,
            105.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("ofp_zero_yld: {result}");
        assert!((result - 131.25).abs() < 0.000001);
    }

    #[test]
    fn test_ofp_zero_coup() {
        // Settlement: 2022-02-01, Maturity: 2027-07-01, Issue: 2021-11-01, First: 2022-07-01
        // Rate: 0.0, Yield: 0.05, Redemption: 100, Freq: 2, Basis: 0
        // Expected: 76.52878028253171
        let result = codcel_odd_f_price(
            dt(2022, 2, 1),
            dt(2027, 7, 1),
            dt(2021, 11, 1),
            dt(2022, 7, 1),
            0.0,
            0.05,
            100.0,
            2,
            Some(0),
        )
        .unwrap();
        println!("ofp_zero_coup: {result}");
        assert!((result - 76.52878028253171).abs() < 0.000001);
    }

    #[test]
    fn test_ofp_discount() {
        // Settlement: 2020-03-01, Maturity: 2030-07-01, Issue: 2019-11-01, First: 2020-07-01
        // Rate: 0.03, Yield: 0.04, Redemption: 100, Freq: 2, Basis: 1
        // Expected: 91.59259282956167
        let result = codcel_odd_f_price(
            dt(2020, 3, 1),
            dt(2030, 7, 1),
            dt(2019, 11, 1),
            dt(2020, 7, 1),
            0.03,
            0.04,
            100.0,
            2,
            Some(1),
        )
        .unwrap();
        println!("ofp_discount: {result}");
        assert!((result - 91.59259282956167).abs() < 0.000001);
    }

    #[test]
    fn test_ofp_ann_par_eq() {
        // Settlement: 2022-10-01, Maturity: 2027-01-01, Issue: 2022-07-01, First: 2023-01-01
        // Rate: 0.055, Yield: 0.055, Redemption: 100, Freq: 1, Basis: 0
        // Expected: 100.00883513602
        let result = codcel_odd_f_price(
            dt(2022, 10, 1),
            dt(2027, 1, 1),
            dt(2022, 7, 1),
            dt(2023, 1, 1),
            0.055,
            0.055,
            100.0,
            1,
            Some(0),
        )
        .unwrap();
        println!("ofp_ann_par_eq: {result}");
        assert!((result - 100.00883513602).abs() < 0.000001);
    }
}
