// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::date_and_time::codcel_year_frac::codcel_year_frac;
use chrono::{DateTime, Utc};
use std::error::Error;

/// Calculates the accrued interest at maturity for a security.
///
/// Excel formula: ACCRINTM = par * rate * YEARFRAC(issue, maturity, basis)
///
/// # Arguments
///
/// * `issue_date` - The date the security was issued
/// * `maturity_date` - The maturity date of the security
/// * `rate` - The annual coupon rate
/// * `par_value` - The par value of the security
/// * `basis` - The day count basis to use (0-4)
pub fn codcel_accr_intm(
    issue_date: DateTime<Utc>,
    maturity_date: DateTime<Utc>,
    rate: f64,
    par_value: f64,
    basis: Option<i32>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Validate inputs
    if issue_date >= maturity_date {
        return Err("ACCRINTM: Issue date must be before maturity date".into());
    }
    if rate < 0.0 {
        return Err("ACCRINTM: Rate cannot be negative".into());
    }
    if par_value < 0.0 {
        return Err("ACCRINTM: Par value cannot be negative".into());
    }

    let basis_val = basis.unwrap_or(0);
    if !(0..=4).contains(&basis_val) {
        return Err("ACCRINTM: Basis must be between 0 and 4".into());
    }

    // Use YEARFRAC to compute the year fraction — matches Excel exactly
    let year_fraction = codcel_year_frac(issue_date, maturity_date, basis)?;

    Ok(par_value * rate * year_fraction)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn dt(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
    }

    #[test]
    fn test_accr_intm_basis_0() {
        // =ACCRINTM(DATE(2020,1,1),DATE(2021,1,1),0.05,1000,0) = 50.0
        let result =
            codcel_accr_intm(dt(2020, 1, 1), dt(2021, 1, 1), 0.05, 1000.0, Some(0)).unwrap();
        assert!((result - 50.0).abs() < 0.000001);
    }

    #[test]
    fn test_accr_intm_basis_1_annual() {
        // =ACCRINTM(DATE(2018,3,15),DATE(2019,3,15),0.065,5000,1) = 325.0
        let result =
            codcel_accr_intm(dt(2018, 3, 15), dt(2019, 3, 15), 0.065, 5000.0, Some(1)).unwrap();
        assert!((result - 325.0).abs() < 0.000001);
    }

    #[test]
    fn test_accr_intm_basis_1_10yr() {
        // =ACCRINTM(DATE(2010,1,1),DATE(2020,1,1),0.03,1000,1) = 299.940268790443
        let result =
            codcel_accr_intm(dt(2010, 1, 1), dt(2020, 1, 1), 0.03, 1000.0, Some(1)).unwrap();
        assert!((result - 299.940268790443).abs() < 0.000001);
    }

    #[test]
    fn test_accr_intm_basis_1_dec31() {
        // =ACCRINTM(DATE(2022,12,31),DATE(2023,6,30),0.08,2000,1) = 79.34246575342466
        let result =
            codcel_accr_intm(dt(2022, 12, 31), dt(2023, 6, 30), 0.08, 2000.0, Some(1)).unwrap();
        assert!((result - 79.34246575342466).abs() < 0.000001);
    }

    #[test]
    fn test_accr_intm_default_par_basis_0() {
        // =ACCRINTM(DATE(2020,1,1),DATE(2020,7,1),0.1,1000,0) = 50.0
        let result =
            codcel_accr_intm(dt(2020, 1, 1), dt(2020, 7, 1), 0.1, 1000.0, Some(0)).unwrap();
        assert!((result - 50.0).abs() < 0.000001);
    }

    #[test]
    fn test_accr_intm_basis_3() {
        // =ACCRINTM(DATE(2021,1,1),DATE(2021,10,1),0.05,10000,3) = 373.972602739726
        let result =
            codcel_accr_intm(dt(2021, 1, 1), dt(2021, 10, 1), 0.05, 10000.0, Some(3)).unwrap();
        assert!((result - 373.972602739726).abs() < 0.000001);
    }
}
