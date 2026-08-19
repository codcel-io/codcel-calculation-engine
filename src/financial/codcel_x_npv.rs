// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSum;
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
    let mut xnpv = CompensatedSum::new();
    for (date, cash) in dates.iter().zip(cash_flows.iter()) {
        let days = days_between(&first_date, date) / 365.0; // Convert to fractional years
        xnpv.add(cash / crate::portable_math::powf(1.0 + rate, days));
    }

    Ok(xnpv.total())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    use chrono::Duration;

    /// Excel serial to date, so schedules can be copied straight out of a spreadsheet.
    /// The epoch is 1899-12-30, accounting for the Lotus 1-2-3 1900 leap year bug.
    fn excel_serial_to_date(serial: i64) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(1899, 12, 30, 0, 0, 0).unwrap() + Duration::days(serial)
    }

    // Expected values below are Excel's own cached results, taken from
    // codcel-tests/financial-all.xlsx sheet "Xnpv". XNPV is closed form, so the tolerance is
    // tighter than the 1e-6 used for the iterative solvers.

    #[test]
    fn test_x_npv_xnv_basic() {
        // =XNPV(B1,B2:B6,B7:B11) -> 329.75910145171906
        let result = codcel_x_npv(
            0.1,
            vec![-10000.0, 2750.0, 4250.0, 3250.0, 2750.0],
            vec![
                excel_serial_to_date(45292),
                excel_serial_to_date(45658),
                excel_serial_to_date(46023),
                excel_serial_to_date(46388),
                excel_serial_to_date(46753),
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 329.75910145171906).abs() < 1e-9,
            "Expected 329.75910145171906, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_low_rate() {
        // =XNPV(B12,B2:B6,B7:B11) -> 1542.283923761474
        let result = codcel_x_npv(
            0.05,
            vec![-10000.0, 2750.0, 4250.0, 3250.0, 2750.0],
            vec![
                excel_serial_to_date(45292),
                excel_serial_to_date(45658),
                excel_serial_to_date(46023),
                excel_serial_to_date(46388),
                excel_serial_to_date(46753),
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 1542.283923761474).abs() < 1e-9,
            "Expected 1542.283923761474, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_high_rate() {
        // =XNPV(B13,B2:B6,B7:B11) -> -689.4016839281569
        let result = codcel_x_npv(
            0.15,
            vec![-10000.0, 2750.0, 4250.0, 3250.0, 2750.0],
            vec![
                excel_serial_to_date(45292),
                excel_serial_to_date(45658),
                excel_serial_to_date(46023),
                excel_serial_to_date(46388),
                excel_serial_to_date(46753),
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -689.4016839281569).abs() < 1e-9,
            "Expected -689.4016839281569, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_v_high_rate() {
        // =XNPV(B14,B2:B6,B7:B11) -> -1554.1812539271295
        let result = codcel_x_npv(
            0.2,
            vec![-10000.0, 2750.0, 4250.0, 3250.0, 2750.0],
            vec![
                excel_serial_to_date(45292),
                excel_serial_to_date(45658),
                excel_serial_to_date(46023),
                excel_serial_to_date(46388),
                excel_serial_to_date(46753),
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -1554.1812539271295).abs() < 1e-9,
            "Expected -1554.1812539271295, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_v_low_rate() {
        // =XNPV(B15,B2:B6,B7:B11) -> 2685.798580741941
        let result = codcel_x_npv(
            0.01,
            vec![-10000.0, 2750.0, 4250.0, 3250.0, 2750.0],
            vec![
                excel_serial_to_date(45292),
                excel_serial_to_date(45658),
                excel_serial_to_date(46023),
                excel_serial_to_date(46388),
                excel_serial_to_date(46753),
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 2685.798580741941).abs() < 1e-9,
            "Expected 2685.798580741941, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_set_2() {
        // =XNPV(B1,B16:B19,B20:B23) -> -689.0612608394513
        let result = codcel_x_npv(
            0.1,
            vec![-50000.0, 15000.0, 25000.0, 20000.0],
            vec![
                excel_serial_to_date(45000),
                excel_serial_to_date(45366),
                excel_serial_to_date(45731),
                excel_serial_to_date(46096),
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -689.0612608394513).abs() < 1e-9,
            "Expected -689.0612608394513, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_set_2_low() {
        // =XNPV(B12,B16:B19,B20:B23) -> 4230.953590452424
        let result = codcel_x_npv(
            0.05,
            vec![-50000.0, 15000.0, 25000.0, 20000.0],
            vec![
                excel_serial_to_date(45000),
                excel_serial_to_date(45366),
                excel_serial_to_date(45731),
                excel_serial_to_date(46096),
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 4230.953590452424).abs() < 1e-9,
            "Expected 4230.953590452424, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_set_2_high() {
        // =XNPV(B13,B16:B19,B20:B23) -> -4919.870320121439
        let result = codcel_x_npv(
            0.15,
            vec![-50000.0, 15000.0, 25000.0, 20000.0],
            vec![
                excel_serial_to_date(45000),
                excel_serial_to_date(45366),
                excel_serial_to_date(45731),
                excel_serial_to_date(46096),
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -4919.870320121439).abs() < 1e-9,
            "Expected -4919.870320121439, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_simple() {
        // =XNPV(B1,B24:B25,B26:B27) -> -1.3054484521944687
        let result = codcel_x_npv(
            0.1,
            vec![-5000.0, 5500.0],
            vec![excel_serial_to_date(45292), excel_serial_to_date(45658)],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -1.3054484521944687).abs() < 1e-9,
            "Expected -1.3054484521944687, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_sim_low() {
        // =XNPV(B12,B24:B25,B26:B27) -> 237.3950998862174
        let result = codcel_x_npv(
            0.05,
            vec![-5000.0, 5500.0],
            vec![excel_serial_to_date(45292), excel_serial_to_date(45658)],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 237.3950998862174).abs() < 1e-9,
            "Expected 237.3950998862174, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_sim_high() {
        // =XNPV(B13,B24:B25,B26:B27) -> -219.22225975554375
        let result = codcel_x_npv(
            0.15,
            vec![-5000.0, 5500.0],
            vec![excel_serial_to_date(45292), excel_serial_to_date(45658)],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -219.22225975554375).abs() < 1e-9,
            "Expected -219.22225975554375, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_sim_v_high() {
        // =XNPV(B14,B24:B25,B26:B27) -> -418.9555209064856
        let result = codcel_x_npv(
            0.2,
            vec![-5000.0, 5500.0],
            vec![excel_serial_to_date(45292), excel_serial_to_date(45658)],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -418.9555209064856).abs() < 1e-9,
            "Expected -418.9555209064856, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_set_2_r_4() {
        // =XNPV(B14,B16:B19,B20:B23) -> -8585.506981932363
        let result = codcel_x_npv(
            0.2,
            vec![-50000.0, 15000.0, 25000.0, 20000.0],
            vec![
                excel_serial_to_date(45000),
                excel_serial_to_date(45366),
                excel_serial_to_date(45731),
                excel_serial_to_date(46096),
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - -8585.506981932363).abs() < 1e-9,
            "Expected -8585.506981932363, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_set_2_v_lo() {
        // =XNPV(B15,B16:B19,B20:B23) -> 8769.087205713378
        let result = codcel_x_npv(
            0.01,
            vec![-50000.0, 15000.0, 25000.0, 20000.0],
            vec![
                excel_serial_to_date(45000),
                excel_serial_to_date(45366),
                excel_serial_to_date(45731),
                excel_serial_to_date(46096),
            ],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 8769.087205713378).abs() < 1e-9,
            "Expected 8769.087205713378, got {result}"
        );
    }

    #[test]
    fn test_x_npv_xnv_sim_r_8() {
        // =XNPV(B28,B24:B25,B26:B27) -> 91.51892160317402
        let result = codcel_x_npv(
            0.08,
            vec![-5000.0, 5500.0],
            vec![excel_serial_to_date(45292), excel_serial_to_date(45658)],
        )
        .unwrap();
        println!("{result:?}");
        assert!(
            (result - 91.51892160317402).abs() < 1e-9,
            "Expected 91.51892160317402, got {result}"
        );
    }

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
