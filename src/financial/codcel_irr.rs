// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::compensated_sum::CompensatedSum;
use crate::financial::root_finding::solve_rate;
use std::error::Error;

/// Calculates the internal rate of return (IRR) for a series of periodic cash flows.
///
/// The IRR is the discount rate at which the net present value of `cash_flows` is zero. The rate
/// is found by [`solve_rate`], which pairs Newton-Raphson with a bracketed bisection fallback so
/// that cash flows with several sign changes still resolve.
///
/// Powers use `powi` rather than `crate::portable_math::powf`: the exponents are exact periods,
/// and integer exponentiation is both more accurate and already free of the platform libm
/// differences `portable_math` exists to paper over. `codcel_npv` discounts the same way.
///
/// # Arguments
/// * `cash_flows` - Sequence of cash flows where the first value is typically the initial investment.
/// * `guess` - Optional initial guess for the rate (defaults to `0.1`).
///
/// # Errors
/// Returns an error when the cash flows are empty, all have the same sign, or no rate can be
/// found whose net present value is zero.
pub fn codcel_irr(
    cash_flows: Vec<f64>,
    guess: Option<f64>,
) -> Result<f64, Box<dyn Error + Send + Sync>> {
    if cash_flows.is_empty() {
        return Err("IRR: Cash flows must not be empty".into());
    }
    if cash_flows.iter().all(|&cf| cf >= 0.0) || cash_flows.iter().all(|&cf| cf <= 0.0) {
        return Err(
            "IRR: Cash flows must contain at least one positive and one negative value".into(),
        );
    }

    // NPV(r) = sum over i of cf_i / (1+r)^i
    let npv = |rate: f64| -> f64 {
        if 1.0 + rate <= 0.0 {
            return f64::NAN;
        }
        let mut total = CompensatedSum::new();
        for (period, &cash_flow) in cash_flows.iter().enumerate() {
            total.add(cash_flow / (1.0 + rate).powi(period as i32));
        }
        total.total()
    };

    // dNPV/dr = sum over i of -i * cf_i / (1+r)^(i+1)
    let npv_derivative = |rate: f64| -> f64 {
        if 1.0 + rate <= 0.0 {
            return f64::NAN;
        }
        let mut total = CompensatedSum::new();
        for (period, &cash_flow) in cash_flows.iter().enumerate().skip(1) {
            total.add(-(period as f64) * cash_flow / (1.0 + rate).powi(period as i32 + 1));
        }
        total.total()
    };

    let scale = cash_flows.iter().map(|cf| cf.abs()).sum::<f64>();

    solve_rate(npv, npv_derivative, guess.unwrap_or(0.1), scale, "IRR")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Independent net present value, used to prove a returned rate really is a root.
    fn npv_at(cash_flows: &[f64], rate: f64) -> f64 {
        cash_flows
            .iter()
            .enumerate()
            .map(|(period, cf)| cf / (1.0 + rate).powi(period as i32))
            .sum()
    }

    /// Asserts the rate matches Excel and is genuinely a zero of the NPV, not merely a point
    /// where the iteration stopped moving.
    fn assert_irr(cash_flows: &[f64], guess: Option<f64>, expected: f64) {
        let result = codcel_irr(cash_flows.to_vec(), guess).unwrap();
        println!("IRR({cash_flows:?}, {guess:?}) = {result}");
        assert!(
            (result - expected).abs() < 0.000001,
            "Expected {expected}, got {result}"
        );

        let residual = npv_at(cash_flows, result);
        let scale: f64 = cash_flows.iter().map(|cf| cf.abs()).sum();
        assert!(
            residual.abs() <= (1e-6 * scale).max(1e-6),
            "NPV at the returned rate should be zero, was {residual}"
        );
    }

    // Expected values below are Excel's own cached results, taken from
    // codcel-tests/financial-all.xlsx sheet "Irr" (cells B52-B71).

    #[test]
    fn test_irr_basic_cell_refs() {
        // =IRR({-10000,3000,4200,5800,2000})
        assert_irr(
            &[-10000.0, 3000.0, 4200.0, 5800.0, 2000.0],
            None,
            0.18615785468970958,
        );
    }

    #[test]
    fn test_irr_equal_payments() {
        // =IRR({-5000,1200,1200,1200,1200,1200})
        assert_irr(
            &[-5000.0, 1200.0, 1200.0, 1200.0, 1200.0, 1200.0],
            None,
            0.06402240764310108,
        );
    }

    #[test]
    fn test_irr_with_guess() {
        // =IRR({-20000,15000,8000},0.05)
        assert_irr(
            &[-20000.0, 15000.0, 8000.0],
            Some(0.05),
            0.11027205849263044,
        );
    }

    #[test]
    fn test_irr_many_periods() {
        // =IRR({-50000,8000,9000,10000,11000,12000,13000,7000,5000})
        assert_irr(
            &[
                -50000.0, 8000.0, 9000.0, 10000.0, 11000.0, 12000.0, 13000.0, 7000.0, 5000.0,
            ],
            None,
            0.10249901682079043,
        );
    }

    #[test]
    fn test_irr_break_even() {
        // =IRR({-1000,1000}) -- exactly zero, the boundary case for a relative tolerance.
        assert_irr(&[-1000.0, 1000.0], None, 0.0);
    }

    #[test]
    fn test_irr_mixed_signs() {
        // =IRR({-10000,5000,-2000,12000})
        // Two sign changes. Bare Newton-Raphson is unreliable here; this is the case the
        // bracketed fallback exists for.
        assert_irr(
            &[-10000.0, 5000.0, -2000.0, 12000.0],
            None,
            0.1853425117351235,
        );
    }

    #[test]
    fn test_irr_reinvestment_mixed_signs() {
        // =IRR({-10000,2000,-500,3000,4000,6000})
        assert_irr(
            &[-10000.0, 2000.0, -500.0, 3000.0, 4000.0, 6000.0],
            None,
            0.10346257885524679,
        );
    }

    #[test]
    fn test_irr_long_range_mixed_signs() {
        // =IRR({-10000,3000,4200,5800,2000,-5000,1200,1200,1200,1200,1200})
        assert_irr(
            &[
                -10000.0, 3000.0, 4200.0, 5800.0, 2000.0, -5000.0, 1200.0, 1200.0, 1200.0, 1200.0,
                1200.0,
            ],
            None,
            0.16077930091054426,
        );
    }

    #[test]
    fn test_irr_negative_return() {
        // =IRR({-100000,20000,20000,20000,20000})
        assert_irr(
            &[-100000.0, 20000.0, 20000.0, 20000.0, 20000.0],
            None,
            -0.08364541746614995,
        );
    }

    #[test]
    fn test_irr_large_investment_negative() {
        // =IRR({-250000,50000,75000,100000})
        assert_irr(
            &[-250000.0, 50000.0, 75000.0, 100000.0],
            None,
            -0.046013405493686155,
        );
    }

    #[test]
    fn test_irr_with_leading_and_interior_zeros() {
        // =IRR({0,-7500,0,10000})
        assert_irr(&[0.0, -7500.0, 0.0, 10000.0], None, 0.1547005383789415);
    }

    #[test]
    fn test_irr_with_negative_guess() {
        // =IRR({0,-7500,0,10000},-0.1)
        // Excel returns a slightly different value for a different guess; both are the same root.
        assert_irr(
            &[0.0, -7500.0, 0.0, 10000.0],
            Some(-0.1),
            0.15470053837537412,
        );
    }

    #[test]
    fn test_irr_small_values() {
        // =IRR({-100,10,20,30,40,50})
        assert_irr(
            &[-100.0, 10.0, 20.0, 30.0, 40.0, 50.0],
            None,
            0.12005761954170246,
        );
    }

    #[test]
    fn test_irr_partial_range() {
        // =IRR({-10000,3000,4200,5800})
        assert_irr(
            &[-10000.0, 3000.0, 4200.0, 5800.0],
            None,
            0.12808308730229379,
        );
    }

    #[test]
    fn test_irr_two_period() {
        // =IRR({-1000,1200})
        assert_irr(&[-1000.0, 1200.0], None, 0.20000000000000018);
    }

    #[test]
    fn test_irr_high_return() {
        // =IRR({-1000,5000}) -- a 400% rate, well outside the usual search neighbourhood.
        assert_irr(&[-1000.0, 5000.0], None, 4.000000000000001);
    }

    #[test]
    fn test_irr_five_period() {
        // =IRR({-50000,15000,15000,15000,15000})
        assert_irr(
            &[-50000.0, 15000.0, 15000.0, 15000.0, 15000.0],
            None,
            0.07713847295204346,
        );
    }

    #[test]
    fn test_irr_guess_zero() {
        // =IRR({-5000,1200,1200,1200,1200,1200},0)
        assert_irr(
            &[-5000.0, 1200.0, 1200.0, 1200.0, 1200.0, 1200.0],
            Some(0.0),
            0.06402240764309641,
        );
    }

    #[test]
    fn test_irr_default_guess_matches_explicit() {
        // =IRR({-10000,3000,4200,5800,2000},0.1) equals the same call with the guess omitted.
        let implicit = codcel_irr(vec![-10000.0, 3000.0, 4200.0, 5800.0, 2000.0], None).unwrap();
        let explicit =
            codcel_irr(vec![-10000.0, 3000.0, 4200.0, 5800.0, 2000.0], Some(0.1)).unwrap();
        assert!((implicit - explicit).abs() < f64::EPSILON);
    }

    #[test]
    fn test_irr_absurd_guess_still_converges() {
        // A guess far from the root must not prevent the bracketed fallback from finding it.
        assert_irr(
            &[-10000.0, 3000.0, 4200.0, 5800.0, 2000.0],
            Some(500.0),
            0.18615785468970958,
        );
    }

    // The cases below are not in the workbook. They are long-horizon and deep-discount cash
    // flows of the kind insurers and fund modellers actually run, and every one of them either
    // failed to converge, overflowed, or returned a silently wrong rate under the previous
    // unbracketed implementation. Expected values are roots verified independently by Brent's
    // method.

    #[test]
    fn test_irr_thirty_year_annuity() {
        let mut cash_flows = vec![-100_000.0];
        cash_flows.extend(std::iter::repeat_n(9_000.0, 30));
        assert_irr(&cash_flows, None, 0.08139601709452418);
    }

    #[test]
    fn test_irr_private_equity_j_curve() {
        // Three years of drawdowns before any distribution.
        assert_irr(
            &[-100.0, -200.0, -300.0, 50.0, 100.0, 200.0, 400.0, 600.0],
            None,
            0.19099621341171752,
        );
    }

    #[test]
    fn test_irr_single_payoff_after_twenty_idle_periods() {
        let mut cash_flows = vec![-1000.0];
        cash_flows.extend(std::iter::repeat_n(0.0, 19));
        cash_flows.push(3000.0);
        assert_irr(&cash_flows, None, 0.05646730854953788);
    }

    #[test]
    fn test_irr_fifty_periods() {
        // The previous implementation returned -0.754 here: a silently wrong answer, which is
        // the failure mode the residual gate exists to prevent.
        let mut cash_flows = vec![-50_000.0];
        cash_flows.extend(std::iter::repeat_n(1_500.0, 50));
        assert_irr(&cash_flows, None, 0.01723218197574849);
    }

    #[test]
    fn test_irr_monthly_mortgage_schedule() {
        // 360 monthly periods. Discount factors here overflowed the old iteration.
        let mut cash_flows = vec![-250_000.0];
        cash_flows.extend(std::iter::repeat_n(1_600.0, 360));
        assert_irr(&cash_flows, None, 0.00551691385445607);
    }

    #[test]
    fn test_irr_near_total_loss() {
        // A root close to -90%: the old derivative guard aborted before reaching it.
        assert_irr(&[-1000.0, 1.0, 1.0, 1.0], None, -0.896322674370506);
    }

    #[test]
    fn test_irr_deeply_negative_return() {
        assert_irr(
            &[-100_000.0, 10_000.0, 10_000.0, 10_000.0],
            None,
            -0.4244174438316309,
        );
    }

    #[test]
    fn test_irr_very_high_return() {
        // A 9900% rate, far outside the dense part of the bracket ladder.
        assert_irr(&[-1000.0, 100_000.0], None, 99.0);
    }

    #[test]
    fn test_irr_large_monetary_scale() {
        // Billions: the residual tolerance must scale with the inputs rather than being absolute.
        assert_irr(&[-1e9, 3e8, 4e8, 5e8], None, 0.08896339469758342);
    }

    #[test]
    fn test_irr_error_cases() {
        // Empty cash flows
        assert!(codcel_irr(vec![], None).is_err());

        // All positive cash flows
        assert!(codcel_irr(vec![100.0, 200.0, 300.0], None).is_err());

        // All negative cash flows
        assert!(codcel_irr(vec![-100.0, -200.0, -300.0], None).is_err());
    }
}
