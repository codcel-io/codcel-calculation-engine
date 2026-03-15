// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::financial::helpers::ppmt_cum_princ;
use std::error::Error;

/// Calculates the cumulative principal paid between start_period and end_period.
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
/// The cumulative principal paid between start_period and end_period.
pub fn codcel_cum_princ(
    rate: f64,
    nper: i32,
    pv: f64,
    start_period: i32,
    end_period: i32,
    payment_type: i32,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate input parameters
    if rate <= 0.0 {
        return Err("CUMPRINC: Rate must be greater than 0".into());
    }
    if nper <= 0 {
        return Err("CUMPRINC: Number of periods must be greater than 0".into());
    }
    if pv <= 0.0 {
        return Err("CUMPRINC: Present value must be greater than 0".into());
    }
    if start_period < 1 || start_period > end_period {
        return Err(
            "CUMPRINC: Start period must be greater than 0 and less than or equal to end period"
                .into(),
        );
    }
    if end_period > nper {
        return Err("CUMPRINC: End period cannot be greater than total number of periods".into());
    }
    if payment_type != 0 && payment_type != 1 {
        return Err(
            "CUMPRINC: Payment type must be 0 (end of period) or 1 (beginning of period)".into(),
        );
    }

    let mut total_principal = 0.0;
    for period in start_period..=end_period {
        total_principal += ppmt_cum_princ(rate, period, nper, pv, payment_type);
    }

    Ok(total_principal)
}
