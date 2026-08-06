// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the net present value for a schedule of cash flows that is not necessarily periodic.
///
/// # Arguments
/// * `rate` - The discount rate to apply to the cash flows.
/// * `cash_flows` - A series of cash flows.
/// * `dates` - A schedule of payment dates that corresponds to the cash flow values.
///
/// # Returns
/// The net present value of the cash flows.
pub fn codcel_x_npv(
    rate: f64,
    cash_flows: Vec<f64>,
    dates: Vec<DateTime<Utc>>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Minimum input validation
    if dates.is_empty() || cash_flows.is_empty() {
        return Err("XNPV: Dates and cash flows cannot be empty.".into());
    }
    if dates.len() != cash_flows.len() {
        return Err("XNPV: Dates and cash flows must have the same length.".into());
    }
    if rate < -1.0 {
        return Err("XNPV: Discount rate must be greater than or equal to -100%.".into());
    }

    // Reference date (the first date in the series)
    let first_date = dates[0];

    // Helper function: Calculate the difference in days between two DateTime values
    let days_between =
        |start: &DateTime<Utc>, end: &DateTime<Utc>| (*end - *start).num_days() as f64;

    // NPV calculation
    let mut xnpv = 0.0;
    for (date, cash) in dates.iter().zip(cash_flows.iter()) {
        let days = days_between(&first_date, date) / 365.0; // Convert to fractional years
        xnpv += cash / crate::portable_math::powf(1.0 + rate, days);
    }

    Ok(xnpv)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_x_npv_error_cases() {
        // Empty inputs
        assert!(codcel_x_npv(0.1, vec![], vec![]).is_err());

        // Mismatched lengths
        assert!(codcel_x_npv(
            0.1,
            vec![100.0, 200.0],
            vec![Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap()],
        )
        .is_err());

        // Invalid rate
        assert!(codcel_x_npv(
            -1.1, // Less than -100%
            vec![100.0, 200.0],
            vec![
                Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2020, 2, 1, 0, 0, 0).unwrap(),
            ],
        )
        .is_err());
    }
}
