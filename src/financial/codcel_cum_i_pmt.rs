// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::financial::helpers::ipmt;
use std::error::Error;

/// Calculates the cumulative interest paid between start_period and end_period.
///
/// # Arguments
/// * `rate` - The interest rate per period.
/// * `nper` - The total number of payment periods.
/// * `pv` - The present value.
/// * `start_period` - The first period to include in the calculation.
/// * `end_period` - The last period to include in the calculation.
/// * `payment_type` - When payments are due (0 = end of period, 1 = beginning of period).
///
/// # Returns
/// The cumulative interest paid between start_period and end_period.
pub fn codcel_cum_i_pmt(
    rate: f64,
    nper: i32,
    pv: f64,
    start_period: i32,
    end_period: i32,
    payment_type: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validation checks remain the same
    if rate <= 0.0 {
        return Err("CUMIPMT: Rate must be positive".into());
    }
    if nper <= 0 {
        return Err("CUMIPMT: Number of periods must be positive".into());
    }
    if pv <= 0.0 {
        return Err("CUMIPMT: Present value must be positive".into());
    }
    if start_period < 1 || start_period > end_period {
        return Err(
            "CUMIPMT: Start period must be positive and not greater than end period".into(),
        );
    }
    if end_period > nper {
        return Err("CUMIPMT: End period cannot be greater than total number of periods".into());
    }
    if payment_type != 0 && payment_type != 1 {
        return Err(
            "CUMIPMT: Payment type must be 0 (end of period) or 1 (beginning of period)".into(),
        );
    }

    let mut cumulative_interest = 0.0;

    for period in start_period..=end_period {
        let ipmt = ipmt(rate, period, nper, pv, 0.0, payment_type)?;
        cumulative_interest += ipmt;
    }

    Ok(cumulative_interest)
}
