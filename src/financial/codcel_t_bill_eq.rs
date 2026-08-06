// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the bond-equivalent yield for a Treasury bill.
///
/// # Arguments
/// * `settlement` - The settlement date of the Treasury bill.
/// * `maturity` - The maturity date of the Treasury bill.
/// * `discount` - The discount rate of the Treasury bill.
///
/// # Returns
/// The bond-equivalent yield for the Treasury bill.
pub fn codcel_t_bill_eq(
    settlement: DateTime<Utc>,
    maturity: DateTime<Utc>,
    discount: f64,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if discount <= 0.0 || discount >= 1.0 {
        return Err("TBILLEQ: Discount must be greater than 0 and less than 1.".into());
    }
    if maturity <= settlement {
        return Err("TBILLEQ: Maturity date must be after settlement date.".into());
    }

    // Calculate the number of days between settlement and maturity
    let days = (maturity - settlement).num_days() as f64;

    // Validate the number of days (Excel allows up to ~366 for leap years)
    if days <= 0.0 || days > 366.0 {
        return Err("TBILLEQ: The number of days between settlement and maturity must be greater than zero and less than or equal to 366.".into());
    }

    if days <= 182.0 {
        // Short formula for T-bills with 182 days or fewer
        // TBILLEQ = (365 * discount) / (360 - (discount * days))
        let tbilleq = (365.0 * discount) / (360.0 - (discount * days));
        Ok(tbilleq)
    } else {
        // Long formula for T-bills with more than 182 days
        // Excel uses a quadratic formula with year base = 365 or 366 (for leap year spans):
        // TBILLEQ = (-DSM/Y + sqrt((DSM/Y)^2 - (2*DSM/Y - 1) * (discount*DSM/(discount*DSM - 360)))) / (DSM/Y - 0.5)
        let year_base = if days > 365.0 { 366.0 } else { 365.0 };
        let dsm_over_y = days / year_base;
        let term = discount * days / (discount * days - 360.0);

        let discriminant = dsm_over_y * dsm_over_y
            - (2.0 * dsm_over_y - 1.0) * term;

        if discriminant < 0.0 {
            return Err("TBILLEQ: Cannot compute bond equivalent yield (negative discriminant).".into());
        }

        let tbilleq = (-dsm_over_y + crate::portable_math::sqrt(discriminant)) / (dsm_over_y - 0.5);
        Ok(tbilleq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_t_bill_eq_basic() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 7, 1, 0, 0, 0).unwrap();
        let result = codcel_t_bill_eq(settlement, maturity, 0.05).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_t_bill_eq_error_cases() {
        let settlement = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();
        let maturity = Utc.with_ymd_and_hms(2022, 1, 1, 0, 0, 0).unwrap();

        // Maturity must be after settlement
        assert!(codcel_t_bill_eq(settlement, maturity, 0.05).is_err());

        // Discount must be greater than 0 and less than 1
        let maturity = Utc.with_ymd_and_hms(2022, 7, 1, 0, 0, 0).unwrap();
        assert!(codcel_t_bill_eq(settlement, maturity, 0.0).is_err());
        assert!(codcel_t_bill_eq(settlement, maturity, 1.0).is_err());
    }
}
