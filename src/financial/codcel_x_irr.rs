// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSum;
use crate::financial::root_finding::solve_rate;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Days per year in Excel's `XIRR` discounting convention.
const DAYS_PER_YEAR: f64 = 365.0;

/// Calculates the internal rate of return for a schedule of cash flows that is not necessarily periodic.
///
/// Each cash flow is discounted by the number of days between it and the first date, divided by
/// 365 — Excel's convention, irrespective of leap years. The rate is found by [`solve_rate`],
/// which pairs Newton-Raphson with a bracketed bisection fallback.
///
/// # Arguments
/// * `cash_flows` - A series of cash flows.
/// * `dates` - A schedule of payment dates that corresponds to the cash flow values.
/// * `guess` - An optional guess for the internal rate of return (defaults to `0.1`).
///
/// # Returns
/// The internal rate of return for the schedule of cash flows.
///
/// # Errors
/// Returns an error when the inputs are empty, differ in length, all share the same sign, or no
/// rate can be found whose net present value is zero.
pub fn codcel_x_irr(
    cash_flows: Vec<f64>,
    dates: Vec<DateTime<Utc>>,
    guess: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Minimum input validation
    if dates.is_empty() || cash_flows.is_empty() {
        return Err("XIRR: Dates and cash flows cannot be empty.".into());
    }
    if dates.len() != cash_flows.len() {
        return Err("XIRR: Dates and cash flows must have the same length.".into());
    }
    if cash_flows.iter().all(|&cash| cash >= 0.0) || cash_flows.iter().all(|&cash| cash <= 0.0) {
        return Err("XIRR: Cash flows must contain at least one negative (outflow) and one positive (inflow) value.".into());
    }

    // Excel measures every cash flow from the first date in the schedule, not the earliest, so
    // an unsorted schedule can legitimately produce negative year fractions. Computed once
    // rather than per iteration, since the solver evaluates the objective many times.
    let first_date = dates[0];
    let year_fractions: Vec<f64> = dates
        .iter()
        .map(|date| (*date - first_date).num_days() as f64 / DAYS_PER_YEAR)
        .collect();

    // XNPV(r) = sum of cash_i / (1+r)^years_i
    let xnpv = |rate: f64| -> f64 {
        if 1.0 + rate <= 0.0 {
            return f64::NAN;
        }
        let mut total = CompensatedSum::new();
        for (years, cash) in year_fractions.iter().zip(cash_flows.iter()) {
            total.add(cash / crate::portable_math::powf(1.0 + rate, *years));
        }
        total.total()
    };

    // dXNPV/dr = sum of -years_i * cash_i / (1+r)^(years_i + 1)
    let xnpv_derivative = |rate: f64| -> f64 {
        if 1.0 + rate <= 0.0 {
            return f64::NAN;
        }
        let mut total = CompensatedSum::new();
        for (years, cash) in year_fractions.iter().zip(cash_flows.iter()) {
            let factor = crate::portable_math::powf(1.0 + rate, *years);
            total.add(-years * cash / factor / (1.0 + rate));
        }
        total.total()
    };

    let scale = cash_flows.iter().map(|cash| cash.abs()).sum::<f64>();

    solve_rate(xnpv, xnpv_derivative, guess.unwrap_or(0.1), scale, "XIRR")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, TimeZone};

    /// Excel serial to date, so schedules can be copied straight out of a spreadsheet.
    /// The epoch is 1899-12-30, accounting for the Lotus 1-2-3 1900 leap year bug.
    fn excel_serial_to_date(serial: i64) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(1899, 12, 30, 0, 0, 0).unwrap() + Duration::days(serial)
    }

    /// Independent XNPV, used to prove a returned rate really is a root.
    fn xnpv_at(cash_flows: &[f64], dates: &[DateTime<Utc>], rate: f64) -> f64 {
        let first = dates[0];
        cash_flows
            .iter()
            .zip(dates.iter())
            .map(|(cash, date)| {
                let years = (*date - first).num_days() as f64 / DAYS_PER_YEAR;
                cash / (1.0 + rate).powf(years)
            })
            .sum()
    }

    /// Asserts the rate matches Excel and is genuinely a zero of the XNPV.
    ///
    /// The tolerance is 1e-6 rather than anything tighter because Excel's own `XIRR` only
    /// converges to about 1e-9; asserting past that would be asserting against Excel's error.
    fn assert_x_irr(cash_flows: &[f64], serials: &[i64], guess: Option<f64>, expected: f64) {
        let dates: Vec<DateTime<Utc>> = serials.iter().map(|s| excel_serial_to_date(*s)).collect();
        let result = codcel_x_irr(cash_flows.to_vec(), dates.clone(), guess).unwrap();
        println!("XIRR({cash_flows:?}, {serials:?}, {guess:?}) = {result}");
        assert!(
            (result - expected).abs() < 0.000001,
            "Expected {expected}, got {result}"
        );

        let residual = xnpv_at(cash_flows, &dates, result);
        let scale: f64 = cash_flows.iter().map(|cash| cash.abs()).sum();
        assert!(
            residual.abs() <= (1e-6 * scale).max(1e-6),
            "XNPV at the returned rate should be zero, was {residual}"
        );
    }

    // Expected values below are Excel's own cached results, taken from
    // codcel-tests/financial-all.xlsx sheet "Xirr" (cells E1-E30). Serials 45292 and 45658
    // are 2024-01-01 and 2025-01-01.

    #[test]
    fn test_x_irr_basic() {
        // =XIRR({-10000,2750,4250,3250,2750},{45292,45658,46023,46388,46753})
        assert_x_irr(
            &[-10000.0, 2750.0, 4250.0, 3250.0, 2750.0],
            &[45292, 45658, 46023, 46388, 46753],
            None,
            0.11527050137519837,
        );
    }

    #[test]
    fn test_x_irr_basic_with_guess() {
        // The same schedule with an explicit guess returns the same rate.
        assert_x_irr(
            &[-10000.0, 2750.0, 4250.0, 3250.0, 2750.0],
            &[45292, 45658, 46023, 46388, 46753],
            Some(0.1),
            0.11527050137519837,
        );
    }

    #[test]
    fn test_x_irr_irregular_intervals() {
        // =XIRR({-50000,10000,15000,20000,25000},{45000,45183,45366,45548,45731})
        // Roughly semi-annual but unevenly spaced, which is the point of XIRR.
        assert_x_irr(
            &[-50000.0, 10000.0, 15000.0, 20000.0, 25000.0],
            &[45000, 45183, 45366, 45548, 45731],
            None,
            0.2724871575832368,
        );
    }

    #[test]
    fn test_x_irr_single_period() {
        // =XIRR({-5000,5500},{45292,45658}) -- 366 days, so the /365 convention is visible.
        assert_x_irr(
            &[-5000.0, 5500.0],
            &[45292, 45658],
            None,
            0.0997135818004608,
        );
    }

    #[test]
    fn test_x_irr_large_amounts() {
        // =XIRR({-100000,30000,40000,50000},{45292,45658,46023,46388})
        assert_x_irr(
            &[-100000.0, 30000.0, 40000.0, 50000.0],
            &[45292, 45658, 46023, 46388],
            None,
            0.08884315192699434,
        );
    }

    #[test]
    fn test_x_irr_far_guess_still_converges() {
        // A guess nowhere near the root must fall through to the bracketed search.
        assert_x_irr(
            &[-10000.0, 2750.0, 4250.0, 3250.0, 2750.0],
            &[45292, 45658, 46023, 46388, 46753],
            Some(50.0),
            0.11527050137519837,
        );
    }

    #[test]
    fn test_x_irr_mixed_signs_mid_schedule() {
        // A follow-on capital call part way through: two sign changes.
        let cash_flows = [-10000.0, 4000.0, -2000.0, 5000.0, 6000.0];
        let serials = [45292, 45658, 46023, 46388, 46753];
        let dates: Vec<DateTime<Utc>> = serials.iter().map(|s| excel_serial_to_date(*s)).collect();
        let result = codcel_x_irr(cash_flows.to_vec(), dates.clone(), None).unwrap();
        println!("XIRR with a mid-schedule outflow = {result}");

        // More than one rate can satisfy these flows, so assert the defining property rather
        // than a single value.
        let residual = xnpv_at(&cash_flows, &dates, result);
        assert!(
            residual.abs() < 1e-3,
            "XNPV at the returned rate should be zero, was {residual}"
        );
        assert!(result > -1.0);
    }

    #[test]
    fn test_x_irr_long_horizon() {
        // Twenty annual flows: the long-horizon shape that defeated unbracketed Newton in IRR.
        let mut cash_flows = vec![-100000.0];
        let mut serials = vec![45292_i64];
        for year in 1..=20 {
            cash_flows.push(9000.0);
            serials.push(45292 + year * 365);
        }
        let dates: Vec<DateTime<Utc>> = serials.iter().map(|s| excel_serial_to_date(*s)).collect();
        let result = codcel_x_irr(cash_flows.clone(), dates.clone(), None).unwrap();
        println!("XIRR over a twenty year horizon = {result}");

        let residual = xnpv_at(&cash_flows, &dates, result);
        assert!(
            residual.abs() < 1e-3,
            "XNPV at the returned rate should be zero, was {residual}"
        );
        // Exactly 365-day spacing makes this equivalent to the periodic IRR of the same flows.
        assert!(
            (result - 0.06394877709238642).abs() < 0.000001,
            "got {result}"
        );
    }

    #[test]
    fn test_x_irr_negative_return() {
        // A schedule that never returns its capital.
        let cash_flows = [-100000.0, 20000.0, 20000.0, 20000.0];
        let serials = [45292, 45658, 46023, 46388];
        let dates: Vec<DateTime<Utc>> = serials.iter().map(|s| excel_serial_to_date(*s)).collect();
        let result = codcel_x_irr(cash_flows.to_vec(), dates.clone(), None).unwrap();
        println!("XIRR of a loss-making schedule = {result}");

        assert!(result < 0.0, "expected a negative rate, got {result}");
        let residual = xnpv_at(&cash_flows, &dates, result);
        assert!(
            residual.abs() < 1e-3,
            "XNPV at the returned rate should be zero, was {residual}"
        );
    }

    #[test]
    fn test_x_irr_error_cases() {
        // Empty inputs
        assert!(codcel_x_irr(vec![], vec![], None).is_err());

        // Mismatched lengths
        assert!(codcel_x_irr(
            vec![100.0, 200.0],
            vec![Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap()],
            None
        )
        .is_err());

        // All positive cash flows
        assert!(codcel_x_irr(
            vec![100.0, 200.0],
            vec![
                Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2020, 2, 1, 0, 0, 0).unwrap(),
            ],
            None
        )
        .is_err());

        // All negative cash flows
        assert!(codcel_x_irr(
            vec![-100.0, -200.0],
            vec![
                Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2020, 2, 1, 0, 0, 0).unwrap(),
            ],
            None
        )
        .is_err());
    }
}
