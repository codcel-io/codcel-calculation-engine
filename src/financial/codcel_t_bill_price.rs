// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the price per $100 face value for a Treasury bill.
///
/// # Arguments
/// * `settlement` - The settlement date of the Treasury bill.
/// * `maturity` - The maturity date of the Treasury bill.
/// * `discount` - The discount rate of the Treasury bill.
///
/// # Returns
/// The price per $100 face value for the Treasury bill.
pub fn codcel_t_bill_price(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    discount: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if discount <= 0.0 || discount >= 1.0 {
        return Err("TBILLPRICE: Discount must be greater than 0 and less than 1.".into());
    }
    if maturity <= settlement {
        return Err("TBILLPRICE: Maturity date must be after settlement date.".into());
    }

    // Calculate the number of days between settlement and maturity
    let days = (maturity - settlement).num_days() as f64;

    // Validate the number of days (Excel allows up to ~366 for leap years)
    if days <= 0.0 || days > 366.0 {
        return Err("TBILLPRICE: The number of days between settlement and maturity must be greater than zero and less than or equal to 366.".into());
    }

    // Calculate the T-bill price using the formula:
    // TBILLPRICE = 100 * (1 - (discount * (days / 360)))
    let price = 100.0 * (1.0 - (discount * (days / 360.0)));

    Ok(price)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_t_bill_price_basic() {
        let settlement = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let maturity = Utc
            .with_ymd_and_hms(2022, 7, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let result = codcel_t_bill_price(settlement, maturity, 0.05).unwrap();
        assert!(result > 0.0 && result < 100.0);
    }

    #[test]
    fn test_t_bill_price_error_cases() {
        let settlement = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        let maturity = Utc
            .with_ymd_and_hms(2022, 1, 1, 0, 0, 0)
            .single()
            .expect("valid test date");

        // Maturity must be after settlement
        assert!(codcel_t_bill_price(settlement, maturity, 0.05).is_err());

        // Discount must be greater than 0 and less than 1
        let maturity = Utc
            .with_ymd_and_hms(2022, 7, 1, 0, 0, 0)
            .single()
            .expect("valid test date");
        assert!(codcel_t_bill_price(settlement, maturity, 0.0).is_err());
        assert!(codcel_t_bill_price(settlement, maturity, 1.0).is_err());
    }
}
