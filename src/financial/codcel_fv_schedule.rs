// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

/// Calculates the future value of an investment based on a schedule of interest rates.
///
/// # Arguments
/// * `principal` - The initial investment amount.
/// * `schedule` - A vector of interest rates to apply sequentially.
///
/// # Returns
/// The future value of the investment after applying all interest rates.
pub fn codcel_fv_schedule(
    principal: f64,
    schedule: Vec<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if schedule.is_empty() {
        return Err("FVSCHEDULE: The schedule of interest rates must not be empty".into());
    }

    let mut future_value = principal;

    for rate in schedule {
        future_value *= 1.0 + rate;
    }

    Ok(future_value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fv_schedule_basic() {
        let result = codcel_fv_schedule(1000.0, vec![0.05, 0.05, 0.05]).unwrap();
        assert!((result - 1157.625).abs() < 0.001);
    }

    #[test]
    fn test_fv_schedule_empty() {
        let result = codcel_fv_schedule(1000.0, vec![]);
        assert!(result.is_err());
    }
}
