// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the internal rate of return for a schedule of cash flows that is not necessarily periodic.
///
/// # Arguments
/// * `cash_flows` - A series of cash flows.
/// * `dates` - A schedule of payment dates that corresponds to the cash flow values.
/// * `guess` - An optional guess for the internal rate of return.
///
/// # Returns
/// The internal rate of return for the schedule of cash flows.
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

    // Use the initial rate guess or default to 0.1 (10%)
    let mut rate = guess.unwrap_or(0.1);

    // Helper function: Calculate the difference in days between two DateTime values
    let days_between =
        |start: &DateTime<Utc>, end: &DateTime<Utc>| (*end - *start).num_days() as f64;

    // Constants for iterative method (Newton's method)
    const MAX_ITERATIONS: usize = 1000;
    const TOLERANCE: f64 = 1e-8;

    let first_date = dates[0]; // Reference date for cash flows

    for _ in 0..MAX_ITERATIONS {
        // Calculate the function (NPV) and its derivative (dNPV/dRate) at the current rate guess
        let mut npv = 0.0;
        let mut npv_derivative = 0.0;

        for (date, cash) in dates.iter().zip(cash_flows.iter()) {
            let days = days_between(&first_date, date) / 365.0;
            let factor = crate::portable_math::powf(1.0 + rate, days);

            npv += cash / factor;
            npv_derivative += -days * cash / factor / (1.0 + rate);
        }

        // Check if the NPV is close enough to zero
        if npv.abs() < TOLERANCE {
            return Ok(rate);
        }

        // Update the rate using Newton's method
        let new_rate = rate - (npv / npv_derivative);

        // Check if the rate has converged
        if (new_rate - rate).abs() < TOLERANCE {
            return Ok(new_rate);
        }

        rate = new_rate;
    }

    Err("XIRR: Failed to converge to a solution.".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_x_irr_basic() {
        let cash_flows = vec![-1000.0, 300.0, 400.0, 400.0, 300.0];
        let dates = vec![
            Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2020, 4, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2020, 10, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2021, 2, 1, 0, 0, 0).unwrap(),
            Utc.with_ymd_and_hms(2021, 7, 1, 0, 0, 0).unwrap(),
        ];

        let result = codcel_x_irr(cash_flows, dates, None).unwrap();
        assert!(result > 0.0);
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
    }
}
