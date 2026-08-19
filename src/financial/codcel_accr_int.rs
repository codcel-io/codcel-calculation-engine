// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSum;
use crate::date_time_base::{actual_actual_days, thirty_360_days, thirty_e_360_days};
use chrono::{DateTime, Datelike, TimeZone, Utc};
use std::error::Error;

/// Add months to a date, preserving end-of-month behavior
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
    let max_day = days_in_month(year, month);

    // For end-of-month dates, preserve end-of-month behavior
    let day = if original_day >= days_in_month(date.year(), date.month() as i32) {
        max_day
    } else {
        original_day.min(max_day)
    };

    Utc.with_ymd_and_hms(year, month as u32, day, 0, 0, 0)
        .unwrap()
}

fn days_in_month(year: i32, month: i32) -> u32 {
    match month {
        1 => 31,
        2 if is_leap_year(year) => 29,
        2 => 28,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Calculate the numerator days (Ai) for the accrual based on the day-count basis
fn accrual_days(start: DateTime<Utc>, end: DateTime<Utc>, basis: i32) -> f64 {
    match basis {
        1..=3 => actual_actual_days(start, end),
        4 => thirty_e_360_days(start, end),
        _ => thirty_360_days(start, end), // basis 0
    }
}

/// Calculates the accrued interest for a security that pays periodic interest.
///
/// Uses Excel's per-period summation algorithm:
/// ACCRINT = par * (rate / frequency) * SUM(Ai / NLi)
///
/// Full quasi-coupon periods contribute ratio 1.0.
/// Partial periods use Ai/NLi where NLi depends on the basis.
#[allow(clippy::too_many_arguments)]
pub fn codcel_accr_int(
    issue_date: DateTime<Utc>,
    first_interest: DateTime<Utc>,
    settlement_date: DateTime<Utc>,
    rate: f64,
    par: f64,
    frequency: i32,
    basis: Option<i32>,
    calc_method: Option<bool>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let basis = basis.unwrap_or(0);
    let calc_method = calc_method.unwrap_or(true);

    // Input validation
    if !(1..=4).contains(&frequency) {
        return Err("ACCRINT: Frequency must be 1, 2, or 4".into());
    }
    if settlement_date < issue_date {
        return Err("ACCRINT: Settlement date must be after issue date".into());
    }
    if first_interest < issue_date {
        return Err("ACCRINT: First interest date must be after issue date".into());
    }
    if rate < 0.0 || par < 0.0 {
        return Err("ACCRINT: Rate and par value must be positive".into());
    }

    // If settlement equals issue date, accrued interest is 0
    if settlement_date == issue_date {
        return Ok(0.0);
    }

    let months_per_period = 12 / frequency;

    // Generate quasi-coupon dates by stepping back from first_interest
    // until we pass the issue_date, then forward to cover settlement_date.
    let mut quasi_dates: Vec<DateTime<Utc>> = Vec::new();

    // Step backward from first_interest to find dates before/at issue_date
    let mut date = first_interest;
    quasi_dates.push(date);
    while date > issue_date {
        date = add_months(date, -months_per_period);
        quasi_dates.push(date);
    }
    quasi_dates.sort();

    // Step forward from first_interest to cover settlement_date
    date = first_interest;
    while date < settlement_date {
        date = add_months(date, months_per_period);
        if date > *quasi_dates.last().unwrap() {
            quasi_dates.push(date);
        }
    }

    // For basis 1 (actual/actual), compute the reference period NL:
    // actual days of the quasi-coupon period immediately preceding first_interest.
    // This is used as NL for partial periods after the first odd period, per Excel.
    // The first odd partial period (containing issue) uses its own quasi-coupon
    // period length instead.
    let reference_nl = if basis == 1 {
        let prev = add_months(first_interest, -months_per_period);
        actual_actual_days(prev, first_interest)
    } else {
        0.0
    };

    // Determine start_date based on calc_method
    // TRUE (default): accrue from issue_date to settlement_date
    // FALSE: skip the first quasi-coupon period containing the issue date,
    //        start from the first quasi-coupon date >= issue_date
    let start_date = if calc_method {
        issue_date
    } else {
        // Find the first quasi-coupon date >= issue_date
        let mut first_qc_after_issue = issue_date;
        for &qd in &quasi_dates {
            if qd >= issue_date {
                first_qc_after_issue = qd;
                break;
            }
        }
        first_qc_after_issue
    };

    // If settlement is at or before start_date, return 0
    if settlement_date <= start_date {
        return Ok(0.0);
    }

    // Sum Ai/NLi across all quasi-coupon periods that overlap [start_date, settlement_date]
    let mut sum = CompensatedSum::new();
    for i in 0..quasi_dates.len() - 1 {
        let period_start = quasi_dates[i];
        let period_end = quasi_dates[i + 1];

        // Skip periods entirely before start_date or entirely after settlement_date
        if period_end <= start_date || period_start >= settlement_date {
            continue;
        }

        // A period is 'full' when it lies entirely within [start_date, settlement_date].
        // When the period's end date equals the settlement date exactly, this is a
        // complete coupon period and contributes exactly 1.0, matching Excel's behavior.
        let is_full = period_start >= start_date && period_end <= settlement_date;

        if is_full {
            // Full quasi-coupon periods contribute exactly 1.0.
            sum.add(1.0);
        } else {
            // Partial period: clamp to [start_date, settlement_date]
            let accrual_start = if period_start < start_date {
                start_date
            } else {
                period_start
            };
            let accrual_end = if period_end > settlement_date {
                settlement_date
            } else {
                period_end
            };

            let a = accrual_days(accrual_start, accrual_end, basis);

            let nl = match basis {
                // For basis 1 (actual/actual):
                // - First odd partial period (issue falls within it, period_start < start_date):
                //   use the actual days of that specific quasi-coupon period.
                // - All other partial periods: use the reference NL (actual days of the
                //   quasi-coupon period immediately before first_interest).
                1 => {
                    if period_start < start_date {
                        actual_actual_days(period_start, period_end)
                    } else {
                        reference_nl
                    }
                }
                3 => 365.0 / frequency as f64,
                _ => 360.0 / frequency as f64, // basis 0, 2, 4
            };

            if nl > 0.0 {
                sum.add(a / nl);
            }
        }
    }

    let result = par * (rate / frequency as f64) * sum.total();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn dt(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    // -------------------------------------------------------------------------
    // Basis 0 (US 30/360)
    // -------------------------------------------------------------------------

    #[test]
    fn test_basis_0_semi_basic() {
        // =ACCRINT(DATE(2024,1,1),DATE(2024,7,1),DATE(2024,10,1),0.08,1000,2,0)
        // Settlement (Oct 1) is mid-way through [Jul,Jan 2025] coupon period.
        // Full period [Jan,Jul] = 1.0; partial [Jul,Oct] = 90/180.
        // result = 1000 * 0.04 * (1.0 + 90/180) = 1000 * 0.04 * 1.5 = 60.0
        let result = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            2,
            Some(0),
            None,
        )
        .unwrap();
        assert!((result - 60.0).abs() < 0.000001, "got {result}");
    }

    #[test]
    fn test_basis_0_quarterly_settlement_on_quasi() {
        // =ACCRINT(DATE(2024,1,1),DATE(2024,7,1),DATE(2024,10,1),0.08,1000,4,0)
        // Quasi-coupon dates: Jan1, Apr1, Jul1, Oct1.
        // Periods [Jan,Apr] and [Apr,Jul] are full (< settlement). [Jul,Oct] is
        // a "partial" period that ends exactly at settlement: 90/90 = 1.0.
        // result = 1000 * 0.02 * (1 + 1 + 1) = 60.0
        let result = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            4,
            Some(0),
            None,
        )
        .unwrap();
        assert!((result - 60.0).abs() < 0.000001, "got {result}");
    }

    #[test]
    fn test_basis_0_annual() {
        // =ACCRINT(DATE(2024,1,1),DATE(2025,1,1),DATE(2025,7,1),0.06,1000,1,0)
        // Full period [Jan 2024, Jan 2025] = 1.0; partial [Jan 2025, Jul 2025] = 180/360.
        // result = 1000 * 0.06 * 1.5 = 90.0
        let result = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2025, 1, 1),
            dt(2025, 7, 1),
            0.06,
            1000.0,
            1,
            Some(0),
            None,
        )
        .unwrap();
        assert!((result - 90.0).abs() < 0.000001, "got {result}");
    }

    // -------------------------------------------------------------------------
    // Basis 1 (actual/actual) — including the bug-fix case
    // -------------------------------------------------------------------------

    #[test]
    fn test_basis_1_annual_issue_on_quasi() {
        // =ACCRINT(DATE(2024,1,1),DATE(2025,1,1),DATE(2025,6,15),0.05,1000,1,1)
        // issue=Jan1 2024 is on quasi schedule.
        // Full period [Jan1 2024, Jan1 2025] <= settlement -> 1.0.
        // Partial [Jan1 2025, Jan1 2026]: a = actual(Jan1 2025, Jun15 2025) = 165;
        // NL = reference_nl = actual(Jan1 2024, Jan1 2025) = 366 (2024 is leap year).
        // result = 1000 * 0.05 * (1.0 + 165/366)
        let expected = 1000.0 * 0.05 * (1.0 + 165.0 / 366.0);
        let result = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2025, 1, 1),
            dt(2025, 6, 15),
            0.05,
            1000.0,
            1,
            Some(1),
            None,
        )
        .unwrap();
        assert!(
            (result - expected).abs() < 0.000001,
            "got {result}, expected {expected}"
        );
    }

    #[test]
    fn test_basis_1_annual_issue_not_on_quasi() {
        // =ACCRINT(DATE(2023,3,15),DATE(2023,9,15),DATE(2024,6,15),0.12,5000,1,1)
        // Issue NOT on quasi-coupon schedule.
        // Period [2022-09-15,2023-09-15] partial (first odd): a=184,
        //   NL=actual(2022-09-15,2023-09-15)=365 (per-period NL).
        // Period [2023-09-15,2024-09-15] partial: a=274,
        //   NL=reference_nl=365.
        // result = 5000 * 0.12 * (184/365 + 274/365)
        let result = codcel_accr_int(
            dt(2023, 3, 15),
            dt(2023, 9, 15),
            dt(2024, 6, 15),
            0.12,
            5000.0,
            1,
            Some(1),
            None,
        )
        .unwrap();
        assert!(
            (result - 752.8767123287671).abs() < 0.000001,
            "got {result}"
        );
    }

    #[test]
    fn test_basis_1_semi_issue_not_on_quasi() {
        // =ACCRINT(DATE(2023,3,15),DATE(2023,9,15),DATE(2024,6,15),0.05,1000,2,1,TRUE)
        // Issue=Mar15 not on semi-annual quasi schedule.
        // reference_nl = actual(2023-03-15, 2023-09-15) = 184.
        // [2022-09-15,2023-09-15] partial to [2023-03-15,2023-09-15]: a=184, NL=184 → 1.0.
        // [2023-09-15,2024-03-15] full (< settlement=Jun15): 1.0.
        // [2024-03-15,2024-09-15] partial to [2024-03-15,2024-06-15]: a=92, NL=184.
        // result = 1000 * 0.025 * (1.0 + 1.0 + 92/184)
        let nl: f64 = 184.0;
        let expected = 1000.0 * 0.025 * (1.0 + 1.0 + 92.0 / nl);
        let result = codcel_accr_int(
            dt(2023, 3, 15),
            dt(2023, 9, 15),
            dt(2024, 6, 15),
            0.05,
            1000.0,
            2,
            Some(1),
            Some(true),
        )
        .unwrap();
        assert!(
            (result - expected).abs() < 0.000001,
            "got {result}, expected {expected}"
        );
    }

    // -------------------------------------------------------------------------
    // Basis 2 (actual/360) — including the two bug-fix cases
    // -------------------------------------------------------------------------

    #[test]
    fn test_basis_2_qtr_settlement_on_quasi() {
        // =ACCRINT(DATE(2024,1,1),DATE(2024,7,1),DATE(2024,10,1),0.08,1000,4,2)
        // All three quarterly periods [Jan,Apr], [Apr,Jul], [Jul,Oct] are full
        // since settlement falls exactly on the quasi-coupon boundary.
        // result = 1000 * 0.02 * (1 + 1 + 1) = 60.0
        let result = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            4,
            Some(2),
            None,
        )
        .unwrap();
        assert!((result - 60.0).abs() < 0.000001, "got {result}");
    }

    #[test]
    fn test_basis_2_qtr_high_par_settlement_on_quasi() {
        // =ACCRINT(DATE(2024,1,1),DATE(2024,7,1),DATE(2024,10,1),0.12,5000,4,2)
        // Same structure as above, par=5000, rate=0.12.
        // result = 5000 * 0.03 * (1 + 1 + 1) = 450.0
        let result = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.12,
            5000.0,
            4,
            Some(2),
            None,
        )
        .unwrap();
        assert!((result - 450.0).abs() < 0.000001, "got {result}");
    }

    #[test]
    fn test_basis_2_semi_settlement_mid_period() {
        // =ACCRINT(DATE(2024,1,1),DATE(2024,7,1),DATE(2024,10,1),0.08,1000,2,2)
        // [Jan,Jul] full (< Oct 1) → 1.0.
        // [Jul,Jan 2025] partial to [Jul,Oct]: actual=92 / NL=180.
        // result = 1000 * 0.04 * (1.0 + 92/180) = 60.44444...
        let result = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            2,
            Some(2),
            None,
        )
        .unwrap();
        assert!(
            (result - 60.44444444444444).abs() < 0.000001,
            "got {result}"
        );
    }

    // -------------------------------------------------------------------------
    // Basis 3 (actual/365)
    // -------------------------------------------------------------------------

    #[test]
    fn test_basis_3_semi_settlement_mid_period() {
        // =ACCRINT(DATE(2024,1,1),DATE(2024,7,1),DATE(2024,10,1),0.08,1000,2,3)
        // [Jan,Jul] full → 1.0; partial [Jul,Oct]: actual=92 / NL=182.5.
        // result = 1000 * 0.04 * (1.0 + 92/182.5)
        let expected = 1000.0 * 0.04 * (1.0 + 92.0 / (365.0 / 2.0));
        let result = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            2,
            Some(3),
            None,
        )
        .unwrap();
        assert!(
            (result - expected).abs() < 0.000001,
            "got {result}, expected {expected}"
        );
    }

    #[test]
    fn test_basis_3_annual_long_period() {
        // =ACCRINT(DATE(2023,3,15),DATE(2023,9,15),DATE(2024,6,15),0.05,1000,2,3,TRUE)
        // [Mar15,Sep15] full → 1.0; [Sep15,Mar15 2024] full → 1.0;
        // partial [Mar15 2024,Jun15 2024]: actual=92 / NL=182.5.
        // result = 1000 * 0.025 * (1 + 1 + 92/182.5)
        let expected = 1000.0 * 0.025 * (1.0 + 1.0 + 92.0 / (365.0 / 2.0));
        let result = codcel_accr_int(
            dt(2023, 3, 15),
            dt(2023, 9, 15),
            dt(2024, 6, 15),
            0.05,
            1000.0,
            2,
            Some(3),
            Some(true),
        )
        .unwrap();
        assert!(
            (result - expected).abs() < 0.000001,
            "got {result}, expected {expected}"
        );
    }

    // -------------------------------------------------------------------------
    // Basis 4 (European 30/360)
    // -------------------------------------------------------------------------

    #[test]
    fn test_basis_4_semi() {
        // =ACCRINT(DATE(2024,1,1),DATE(2024,7,1),DATE(2024,10,1),0.08,1000,2,4)
        // European 30/360: [Jan,Jul] = 180 days, NL=180 → 1.0.
        // Partial [Jul,Oct]: 30E/360 days = 90 / NL=180.
        // result = 1000 * 0.04 * (1.0 + 90/180) = 60.0
        let result = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            2,
            Some(4),
            None,
        )
        .unwrap();
        assert!((result - 60.0).abs() < 0.000001, "got {result}");
    }

    // -------------------------------------------------------------------------
    // Default basis (omitted = 0)
    // -------------------------------------------------------------------------

    #[test]
    fn test_default_basis_is_zero() {
        // Omitting basis should give the same result as basis=0.
        let with_basis = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            2,
            Some(0),
            None,
        )
        .unwrap();
        let default_basis = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            2,
            None,
            None,
        )
        .unwrap();
        assert!((with_basis - default_basis).abs() < 0.000001);
    }

    // -------------------------------------------------------------------------
    // calc_method = FALSE
    // -------------------------------------------------------------------------

    #[test]
    fn test_calc_method_false_skips_first_partial() {
        // calc_method=FALSE: start accruing from the first quasi-coupon date >= issue,
        // not from issue itself. With issue=Jan1 on a quarterly coupon date, the
        // first quasi-coupon >= issue IS Jan1, so calc_method makes no difference here.
        // But settlement is Oct1 (on coupon boundary), so:
        // [Jan,Apr] full, [Apr,Jul] full, [Jul,Oct] partial-at-boundary.
        // result = 1000 * 0.02 * (1 + 1 + 92/90) — same as calc_method=TRUE in this layout.
        let result_true = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            4,
            Some(2),
            Some(true),
        )
        .unwrap();
        let result_false = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            4,
            Some(2),
            Some(false),
        )
        .unwrap();
        assert!((result_true - result_false).abs() < 0.000001);
    }

    #[test]
    fn test_calc_method_false_issue_not_on_quasi() {
        // With issue=Feb1 (NOT on quarterly coupon dates Apr1/Jul1/Oct1/Jan1),
        // calc_method=FALSE starts from Apr1 (first quasi-coupon >= issue).
        // Settlement=Jul1: single full period [Apr1,Jul1] ends exactly at settlement → full.
        // result = 1000 * 0.02 * 1.0 = 20.0
        let result = codcel_accr_int(
            dt(2024, 2, 1),
            dt(2024, 7, 1),
            dt(2024, 7, 1),
            0.08,
            1000.0,
            4,
            Some(2),
            Some(false),
        )
        .unwrap();
        assert!((result - 20.0).abs() < 0.000001, "got {result}");
    }

    // -------------------------------------------------------------------------
    // Settlement equals issue — always zero
    // -------------------------------------------------------------------------

    #[test]
    fn test_settlement_equals_issue_returns_zero() {
        let result = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 1, 1),
            0.08,
            1000.0,
            2,
            Some(0),
            None,
        )
        .unwrap();
        assert_eq!(result, 0.0);
    }

    // -------------------------------------------------------------------------
    // End-of-month: issue on Feb 29 leap day
    // -------------------------------------------------------------------------

    #[test]
    fn test_leap_year_issue_feb29_basis_2() {
        // =ACCRINT(DATE(2024,2,29),DATE(2024,8,31),DATE(2024,11,30),0.035,1000,2,2)
        // add_months(Aug31, -6) → Feb 29 (preserved end-of-month in leap year).
        // [Feb29,Aug31] full (< Nov30) → 1.0.
        // Partial [Aug31,Feb28 2025] clipped to [Aug31,Nov30]: actual=91, NL=180.
        // result = 1000 * 0.0175 * (1.0 + 91/180)
        let expected = 1000.0 * 0.0175 * (1.0 + 91.0 / 180.0);
        let result = codcel_accr_int(
            dt(2024, 2, 29),
            dt(2024, 8, 31),
            dt(2024, 11, 30),
            0.035,
            1000.0,
            2,
            Some(2),
            None,
        )
        .unwrap();
        assert!(
            (result - expected).abs() < 0.000001,
            "got {result}, expected {expected}"
        );
    }

    // -------------------------------------------------------------------------
    // Error cases
    // -------------------------------------------------------------------------

    #[test]
    fn test_error_invalid_frequency() {
        // Frequency must be in 1..=4; 5 is out of range.
        let err = codcel_accr_int(
            dt(2024, 1, 1),
            dt(2024, 7, 1),
            dt(2024, 10, 1),
            0.08,
            1000.0,
            5,
            Some(0),
            None,
        );
        assert!(err.is_err());
    }

    #[test]
    fn test_error_settlement_before_issue() {
        let err = codcel_accr_int(
            dt(2024, 6, 1),
            dt(2024, 12, 1),
            dt(2024, 1, 1),
            0.08,
            1000.0,
            2,
            Some(0),
            None,
        );
        assert!(err.is_err());
    }

    #[test]
    fn test_error_first_interest_before_issue() {
        let err = codcel_accr_int(
            dt(2024, 6, 1),
            dt(2024, 1, 1),
            dt(2024, 12, 1),
            0.08,
            1000.0,
            2,
            Some(0),
            None,
        );
        assert!(err.is_err());
    }
}
