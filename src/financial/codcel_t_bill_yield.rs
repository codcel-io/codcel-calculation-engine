// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the yield for a Treasury bill.
///
/// # Arguments
/// * `settlement` - The settlement date of the Treasury bill.
/// * `maturity` - The maturity date of the Treasury bill.
/// * `price` - The price per $100 face value of the Treasury bill.
///
/// # Returns
/// The yield for the Treasury bill.
pub fn codcel_t_bill_yield(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    price: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if price <= 0.0 || price > 100.0 {
        return Err(
            "TBILLYIELD: Price must be greater than 0 and less than or equal to 100.".into(),
        );
    }
    if maturity <= settlement {
        return Err("TBILLYIELD: Maturity date must be after settlement date.".into());
    }

    // Calculate the number of days between settlement and maturity
    let days = (maturity - settlement).num_days() as f64;

    // Validate the number of days (Excel allows up to ~366 for leap years)
    if days <= 0.0 || days > 366.0 {
        return Err("TBILLYIELD: The number of days between settlement and maturity must be greater than zero and less than or equal to 366.".into());
    }

    // Calculate the T-bill yield using the formula:
    // TBILLYIELD = ((100 - price) / price) * (360 / days)
    let yield_rate = ((100.0 - price) / price) * (360.0 / days);

    Ok(yield_rate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_t_bill_yield_basic() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 7, 1, 0, 0, 0).unwrap();
        let result = codcel_t_bill_yield(settlement, maturity, 95.0).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_t_bill_yield_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();

        // Maturity must be after settlement
        assert!(codcel_t_bill_yield(settlement, maturity, 95.0).is_err());

        // Price must be greater than 0 and less than or equal to 100
        let maturity = Utc.with_ymd_and_hms(2022, 7, 1, 0, 0, 0).unwrap();
        assert!(codcel_t_bill_yield(settlement, maturity, 0.0).is_err());
        assert!(codcel_t_bill_yield(settlement, maturity, 101.0).is_err());
    }
}
