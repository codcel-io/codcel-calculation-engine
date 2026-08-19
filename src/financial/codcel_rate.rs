// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::financial::root_finding::solve_rate;
use std::error::Error;

/// Calculate the interest rate per period of an annuity.
///
/// # Arguments
/// * `nper` - The total number of payment periods.
/// * `pmt` - The payment made each period.
/// * `pv` - The present value.
/// * `fv` - The future value (optional, defaults to 0).
/// * `type_` - When payments are due: 0 for end of period, 1 for beginning of period (optional, defaults to 0).
/// * `guess` - Your guess for what the rate will be (optional, defaults to 0.1).
///
/// # Returns
/// The interest rate per period.
pub fn codcel_rate(
    nper: f64,
    pmt: f64,
    pv: f64,
    fv: Option<f64>,
    type_: Option<i32>,
    guess: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    // Set default values for optional parameters
    let fv = fv.unwrap_or(0.0);
    let type_ = type_.unwrap_or(0);
    let guess = guess.unwrap_or(0.1); // 10% initial guess

    if nper <= 0.0 {
        return Err("RATE: Number of periods must be positive".into());
    }

    // Validate type parameter
    if type_ != 0 && type_ != 1 {
        return Err("RATE: Type must be 0 or 1".into());
    }

    let w = type_ as f64;

    // f(rate) = fv + pv*(1+rate)^nper + pmt*(1+rate*w)*((1+rate)^nper - 1)/rate
    let f_val = |r: f64| -> f64 {
        if r.abs() < 1e-12 {
            fv + pv * (1.0 + nper * r) + pmt * (1.0 + r * w) * nper
        } else {
            let tmp = crate::portable_math::powf(1.0 + r, nper);
            fv + pv * tmp + pmt * (1.0 + r * w) * (tmp - 1.0) / r
        }
    };

    // Exact derivative of f with respect to rate
    let f_deriv = |r: f64| -> f64 {
        if r.abs() < 1e-12 {
            pv * nper + pmt * w * nper + pmt * nper * (nper - 1.0) / 2.0
        } else {
            let tmp = crate::portable_math::powf(1.0 + r, nper);
            let dtmp = nper * crate::portable_math::powf(1.0 + r, nper - 1.0);
            let d_pv = pv * dtmp;
            // Product rule: d/dr [pmt * (1+r*w) * ((1+r)^n - 1) / r]
            let u = 1.0 + r * w;
            let v = (tmp - 1.0) / r;
            let v_prime = (dtmp * r - (tmp - 1.0)) / (r * r);
            let d_pmt = pmt * (w * v + u * v_prime);
            d_pv + d_pmt
        }
    };

    // Total cash moving through the annuity, used to scale the solver's residual tolerance.
    let scale = fv.abs() + pv.abs() + pmt.abs() * nper;

    solve_rate(f_val, f_deriv, guess, scale, "RATE")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_basic() {
        let result = codcel_rate(10.0, -100.0, 800.0, None, None, None).unwrap();
        assert!((result - 0.05).abs() < 0.01);
    }

    #[test]
    fn test_rate_with_future_value() {
        let result = codcel_rate(10.0, -100.0, 800.0, Some(100.0), None, None).unwrap();
        assert!(result > 0.0);
    }

    #[test]
    fn test_rate_two_periods_high_guess() {
        // RATE(2, -600, 1000, 0, 0, 0.5)
        let result = codcel_rate(2.0, -600.0, 1000.0, Some(0.0), Some(0), Some(0.5)).unwrap();
        assert!((result - 0.1306623862918077).abs() < 1e-6);
    }

    // Expected values below are Excel's own cached results, taken from
    // codcel-tests/financial-all.xlsx sheet "Rate" (cells B112-B141).

    #[test]
    fn test_rate_basic_loan() {
        // =RATE(B1,B2,B3) -> 0.007701472488201707
        let result = codcel_rate(48.0, -500.0, 20000.0, None, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.007701472488201707).abs() < 0.000001,
            "Expected 0.007701472488201707, got {result}"
        );
    }

    #[test]
    fn test_rate_basic_loan_fv_type() {
        // =RATE(B1,B2,B3,B4,B5) -> 0.007701472488201707
        let result = codcel_rate(48.0, -500.0, 20000.0, Some(0.0), Some(0), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.007701472488201707).abs() < 0.000001,
            "Expected 0.007701472488201707, got {result}"
        );
    }

    #[test]
    fn test_rate_with_guess() {
        // =RATE(B1,B2,B3,B4,B5,B6) -> 0.007701472488201707
        let result = codcel_rate(48.0, -500.0, 20000.0, Some(0.0), Some(0), Some(0.1)).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.007701472488201707).abs() < 0.000001,
            "Expected 0.007701472488201707, got {result}"
        );
    }

    #[test]
    fn test_rate_mortgage_30yr() {
        // =RATE(B7,B8,B9) -> 0.004166644536345589
        let result = codcel_rate(360.0, -1073.64, 200000.0, None, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.004166644536345589).abs() < 0.000001,
            "Expected 0.004166644536345589, got {result}"
        );
    }

    #[test]
    fn test_rate_auto_loan_fv() {
        // =RATE(B10,B11,B12,B13) -> 0.008403502487931841
        let result = codcel_rate(60.0, -1000.0, 50000.0, Some(-5000.0), None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.008403502487931841).abs() < 0.000001,
            "Expected 0.008403502487931841, got {result}"
        );
    }

    #[test]
    fn test_rate_annuity_due() {
        // =RATE(B14,B15,B16,0,B17) -> 0.03503153036227801
        let result = codcel_rate(12.0, -1200.0, 12000.0, Some(0.0), Some(1), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.03503153036227801).abs() < 0.000001,
            "Expected 0.03503153036227801, got {result}"
        );
    }

    #[test]
    fn test_rate_zero_coupon() {
        // =RATE(B18,B19,B20,B21) -> 0.07177346253644194
        let result = codcel_rate(10.0, 0.0, -1000.0, Some(2000.0), None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.07177346253644194).abs() < 0.000001,
            "Expected 0.07177346253644194, got {result}"
        );
    }

    #[test]
    fn test_rate_single_period() {
        // =RATE(B22,B23,B24,B25) -> 0.1
        let result = codcel_rate(1.0, 0.0, -100.0, Some(110.0), None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1).abs() < 0.000001,
            "Expected 0.1, got {result}"
        );
    }

    #[test]
    fn test_rate_long_term_10yr() {
        // =RATE(B26,B27,B28) -> 0.008510875721123577
        let result = codcel_rate(120.0, -200.0, 15000.0, None, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.008510875721123577).abs() < 0.000001,
            "Expected 0.008510875721123577, got {result}"
        );
    }

    #[test]
    fn test_rate_with_small_guess() {
        // =RATE(B29,B30,B31,0,0,B32) -> 0.010207449002719483
        let result = codcel_rate(36.0, -500.0, 15000.0, Some(0.0), Some(0), Some(0.01)).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.010207449002719483).abs() < 0.000001,
            "Expected 0.010207449002719483, got {result}"
        );
    }

    #[test]
    fn test_rate_begin_period() {
        // =RATE(B33,B34,B35,0,B36) -> 0.006811336060173486
        let result = codcel_rate(24.0, -900.0, 20000.0, Some(0.0), Some(1), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.006811336060173486).abs() < 0.000001,
            "Expected 0.006811336060173486, got {result}"
        );
    }

    #[test]
    fn test_rate_15yr_mortgage() {
        // =RATE(B37,B38,B39) -> 0.003504118412456839
        let result = codcel_rate(180.0, -1500.0, 200000.0, None, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.003504118412456839).abs() < 0.000001,
            "Expected 0.003504118412456839, got {result}"
        );
    }

    #[test]
    fn test_rate_all_six_args() {
        // =RATE(B40,B41,B42,B43,B44,B45) -> 0.12589832496242429
        let result = codcel_rate(5.0, -2500.0, 10000.0, Some(0.0), Some(1), Some(0.05)).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.12589832496242429).abs() < 0.000001,
            "Expected 0.12589832496242429, got {result}"
        );
    }

    #[test]
    fn test_rate_savings_fv() {
        // =RATE(B46,B47,B48,B49) -> 0.03305341894229871
        let result = codcel_rate(30.0, -100.0, 0.0, Some(5000.0), None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.03305341894229871).abs() < 0.000001,
            "Expected 0.03305341894229871, got {result}"
        );
    }

    #[test]
    fn test_rate_savings_begin() {
        // =RATE(B50,B51,B52,B53,B54) -> 0.04645436870440599
        let result = codcel_rate(20.0, -300.0, 0.0, Some(10000.0), Some(1), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04645436870440599).abs() < 0.000001,
            "Expected 0.04645436870440599, got {result}"
        );
    }

    #[test]
    fn test_rate_short_term_fv() {
        // =RATE(B55,B56,B57,B58) -> 0.10847887169328899
        let result = codcel_rate(4.0, -3000.0, 10000.0, Some(-1000.0), None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.10847887169328899).abs() < 0.000001,
            "Expected 0.10847887169328899, got {result}"
        );
    }

    #[test]
    fn test_rate_20yr_mortgage() {
        // =RATE(B59,B60,B61) -> 0.006173646637175839
        let result = codcel_rate(240.0, -800.0, 100000.0, None, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.006173646637175839).abs() < 0.000001,
            "Expected 0.006173646637175839, got {result}"
        );
    }

    #[test]
    fn test_rate_fv_type_begin() {
        // =RATE(B62,B63,B64,B65,B66) -> 0.11861352355123267
        let result = codcel_rate(6.0, -2000.0, 10000.0, Some(-1500.0), Some(1), None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.11861352355123267).abs() < 0.000001,
            "Expected 0.11861352355123267, got {result}"
        );
    }

    #[test]
    fn test_rate_weekly_pmt() {
        // =RATE(B67,B68,B69) -> 0.010408967375366904
        let result = codcel_rate(52.0, -50.0, 2000.0, None, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.010408967375366904).abs() < 0.000001,
            "Expected 0.010408967375366904, got {result}"
        );
    }

    #[test]
    fn test_rate_two_periods() {
        // =RATE(B70,B71,B72,B73,B74,B75) -> 0.1306623862918077
        let result = codcel_rate(2.0, -600.0, 1000.0, Some(0.0), Some(0), Some(0.5)).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1306623862918077).abs() < 0.000001,
            "Expected 0.1306623862918077, got {result}"
        );
    }

    #[test]
    fn test_rate_fv_guess() {
        // =RATE(B76,B77,B78,B79,B80,B81) -> 0.00914748042805722
        let result = codcel_rate(48.0, -250.0, 10000.0, Some(-500.0), Some(0), Some(0.1)).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.00914748042805722).abs() < 0.000001,
            "Expected 0.00914748042805722, got {result}"
        );
    }

    #[test]
    fn test_rate_quick_payoff() {
        // =RATE(B82,B83,B84) -> 0.1204439829770753
        let result = codcel_rate(3.0, -5000.0, 12000.0, None, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.1204439829770753).abs() < 0.000001,
            "Expected 0.1204439829770753, got {result}"
        );
    }

    #[test]
    fn test_rate_all_args_used() {
        // =RATE(B85,B86,B87,B88,B89,B90) -> 0.13057438551183098
        let result = codcel_rate(10.0, -500.0, 3000.0, Some(-1000.0), Some(0), Some(0.1)).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.13057438551183098).abs() < 0.000001,
            "Expected 0.13057438551183098, got {result}"
        );
    }

    #[test]
    fn test_rate_daily_pmt() {
        // =RATE(B91,B92,B93) -> 0.0005037435917350579
        let result = codcel_rate(365.0, -30.0, 10000.0, None, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.0005037435917350579).abs() < 0.000001,
            "Expected 0.0005037435917350579, got {result}"
        );
    }

    #[test]
    fn test_rate_begin_guess() {
        // =RATE(B94,B95,B96,0,B97,B98) -> 0.006407985777790172
        let result = codcel_rate(60.0, -200.0, 10000.0, Some(0.0), Some(1), Some(0.005)).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.006407985777790172).abs() < 0.000001,
            "Expected 0.006407985777790172, got {result}"
        );
    }

    #[test]
    fn test_rate_six_args_full() {
        // =RATE(B99,B100,B101,B102,B103,B104) -> 0.012043456781418497
        let result = codcel_rate(12.0, -450.0, 5000.0, Some(0.0), Some(0), Some(0.02)).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.012043456781418497).abs() < 0.000001,
            "Expected 0.012043456781418497, got {result}"
        );
    }

    #[test]
    fn test_rate_compound_growth() {
        // =RATE(B105,B106,B107,B108,B109,B110) -> 0.06010141967983084
        let result = codcel_rate(15.0, 0.0, -5000.0, Some(12000.0), Some(0), Some(0.1)).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.06010141967983084).abs() < 0.000001,
            "Expected 0.06010141967983084, got {result}"
        );
    }

    #[test]
    fn test_rate_inline_loan() {
        // =RATE(36,-300,9000) -> 0.010207449002723698
        let result = codcel_rate(36.0, -300.0, 9000.0, None, None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.010207449002723698).abs() < 0.000001,
            "Expected 0.010207449002723698, got {result}"
        );
    }

    #[test]
    fn test_rate_inline_fv() {
        // =RATE(10,0,-1000,1500) -> 0.04137974399241937
        let result = codcel_rate(10.0, 0.0, -1000.0, Some(1500.0), None, None).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.04137974399241937).abs() < 0.000001,
            "Expected 0.04137974399241937, got {result}"
        );
    }

    #[test]
    fn test_rate_inline_all() {
        // =RATE(24,-500,10000,0,1,0.01) -> 0.01655011906667984
        let result = codcel_rate(24.0, -500.0, 10000.0, Some(0.0), Some(1), Some(0.01)).unwrap();
        println!("{result:?}");
        assert!(
            (result - 0.01655011906667984).abs() < 0.000001,
            "Expected 0.01655011906667984, got {result}"
        );
    }

    #[test]
    fn test_rate_error_cases() {
        let result = codcel_rate(-10.0, -100.0, 800.0, None, None, None);
        assert!(result.is_err());

        let result = codcel_rate(10.0, -100.0, 800.0, None, Some(2), None);
        assert!(result.is_err());
    }
}
