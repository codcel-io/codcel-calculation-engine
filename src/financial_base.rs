// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::area::{
    process_area_datetime_datetime_datetime_float_float_float_int_opt_int_to_float,
    process_area_datetime_datetime_datetime_float_float_int_opt_int_opt_bool_to_float,
    process_area_datetime_datetime_float_float_opt_int_to_float,
    process_area_float_datetime_datetime_float_int_float_opt_int_to_int,
    process_area_float_float_float_float_float_opt_float_opt_bool_to_float,
    process_area_float_float_float_float_opt_float_to_float,
    process_area_float_float_float_float_to_float,
    process_area_float_float_float_int_opt_int_to_float,
    process_area_float_float_float_opt_float_opt_int_opt_float_to_float,
    process_area_float_float_float_opt_float_opt_int_to_float,
    process_area_float_int_float_float_opt_float_opt_int_to_float,
    process_area_float_int_int_float_opt_float_opt_int_to_float, process_area_float_multi_to_float,
};
use crate::financial::codcel_accr_int::codcel_accr_int;
use crate::financial::codcel_accr_intm::codcel_accr_intm;
use crate::financial::codcel_amor_degrc::codcel_amor_degrc;
use crate::financial::codcel_amor_linc::codcel_amor_linc;
use crate::financial::codcel_coup_day_bs::codcel_coup_day_bs;
use crate::financial::codcel_coup_days::codcel_coup_days;
use crate::financial::codcel_coup_days_nc::codcel_coup_days_nc;
use crate::financial::codcel_coup_ncd::codcel_coup_ncd;
use crate::financial::codcel_coup_num::codcel_coup_num;
use crate::financial::codcel_coup_pcd::codcel_coup_pcd;
use crate::financial::codcel_cum_i_pmt::codcel_cum_i_pmt;
use crate::financial::codcel_cum_princ::codcel_cum_princ;
use crate::financial::codcel_db::codcel_db;
use crate::financial::codcel_ddb::codcel_ddb;
use crate::financial::codcel_disc::codcel_disc;
use crate::financial::codcel_dollar_de::codcel_dollar_de;
use crate::financial::codcel_dollar_fr::codcel_dollar_fr;
use crate::financial::codcel_duration::codcel_duration;
use crate::financial::codcel_effect::codcel_effect;
use crate::financial::codcel_fv::codcel_fv;
use crate::financial::codcel_fv_schedule::codcel_fv_schedule;
use crate::financial::codcel_i_pmt::codcel_i_pmt;
use crate::financial::codcel_int_rate::codcel_int_rate;
use crate::financial::codcel_irr::codcel_irr;
use crate::financial::codcel_is_pmt::codcel_is_pmt;
use crate::financial::codcel_m_duration::codcel_m_duration;
use crate::financial::codcel_m_irr::codcel_m_irr;
use crate::financial::codcel_n_per::codcel_n_per;
use crate::financial::codcel_nominal::codcel_nominal;
use crate::financial::codcel_npv::codcel_npv;
use crate::financial::codcel_odd_f_price::codcel_odd_f_price;
use crate::financial::codcel_odd_f_yield::codcel_odd_f_yield;
use crate::financial::codcel_odd_l_price::codcel_odd_l_price;
use crate::financial::codcel_odd_l_yield::codcel_odd_l_yield;
use crate::financial::codcel_p_duration::codcel_p_duration_vec;
use crate::financial::codcel_p_pmt::codcel_p_pmt;
use crate::financial::codcel_pmt::codcel_pmt;
use crate::financial::codcel_price::codcel_price;
use crate::financial::codcel_price_disc::codcel_price_disc;
use crate::financial::codcel_price_mat::codcel_price_mat;
use crate::financial::codcel_pv::codcel_pv;
use crate::financial::codcel_rate::codcel_rate;
use crate::financial::codcel_received::codcel_received;
use crate::financial::codcel_rri::codcel_rri_vec;
use crate::financial::codcel_sln::codcel_sln_vec;
use crate::financial::codcel_syd::codcel_syd_vec;
use crate::financial::codcel_t_bill_eq::codcel_t_bill_eq;
use crate::financial::codcel_t_bill_price::codcel_t_bill_price;
use crate::financial::codcel_t_bill_yield::codcel_t_bill_yield;
use crate::financial::codcel_vdb::codcel_vdb;
use crate::financial::codcel_x_irr::codcel_x_irr;
use crate::financial::codcel_x_npv::codcel_x_npv;
use crate::financial::codcel_yield::codcel_yield;
use crate::financial::codcel_yield_disc::codcel_yield_disc;
use crate::financial::codcel_yield_mat::codcel_yield_mat;
use crate::value::Value;
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `ACCRINT` function.
/// Calculates the accrued interest for a security that pays periodic interest.
/// - `issue_date`: the security's issue date.
/// - `first_interest`: the security's first interest date.
/// - `settlement_date`: the security's settlement date (date purchased).
/// - `rate`: the security's annual coupon rate.
/// - `par`: the security's par (face) value.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
/// - `calc_method`: optional calculation method (true or omitted = accrued from issue to settlement, false = from first interest to settlement).
///
/// Returns the accrued interest, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn accr_int(
    issue_date: Value,
    first_interest: Value,
    settlement_date: Value,
    rate: Value,
    par: Value,
    frequency: Value,
    basis: Value,
    calc_method: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_datetime_datetime_datetime_float_float_int_opt_int_opt_bool_to_float(
        issue_date,
        first_interest,
        settlement_date,
        rate,
        par,
        frequency,
        basis,
        calc_method,
        strict_type_conversion,
        value_format,
        "ACCRINT",
        codcel_accr_int,
    )
}

/// Excel-compatible `ACCRINTM` function.
/// Calculates the accrued interest for a security that pays interest at maturity.
/// - `issue_date`: the security's issue date.
/// - `maturity_date`: the security's maturity date.
/// - `rate`: the security's annual coupon rate.
/// - `par_value`: the security's par (face) value.
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the accrued interest at maturity, or an error if inputs are invalid.
pub fn accr_intm(
    issue_date: Value,
    maturity_date: Value,
    rate: Value,
    par_value: Value,
    basis: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Excel defaults par_value to 1000 when omitted
    let par_value = if par_value.is_none() {
        Value::F64(1000.0)
    } else {
        par_value
    };
    process_area_datetime_datetime_float_float_opt_int_to_float(
        issue_date,
        maturity_date,
        rate,
        par_value,
        basis,
        strict_type_conversion,
        value_format,
        "ACCRINTM",
        codcel_accr_intm,
    )
}

/// Excel-compatible `AMORDEGRC` function.
/// Calculates depreciation for each accounting period using a degressive (declining) depreciation coefficient.
/// This is primarily used in French accounting systems.
/// - `cost`: the initial cost of the asset.
/// - `date_purchased`: the date the asset was purchased.
/// - `first_period`: the date of the end of the first period.
/// - `salvage_value`: the salvage value at the end of the asset's life.
/// - `period`: the period for which to calculate depreciation.
/// - `rate`: the rate of depreciation.
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the depreciation amount for the specified period (as an integer), or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn amor_degrc(
    cost: Value,
    date_purchased: Value,
    first_period: Value,
    salvage_value: Value,
    period: Value,
    rate: Value,
    basis: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    process_area_float_datetime_datetime_float_int_float_opt_int_to_int(
        cost,
        date_purchased,
        first_period,
        salvage_value,
        period,
        rate,
        basis,
        strict_type_conversion,
        value_format,
        "AMORDEGRC",
        codcel_amor_degrc,
    )
}

/// Excel-compatible `AMORLINC` function.
/// Calculates depreciation for each accounting period using linear (straight-line) depreciation.
/// This is primarily used in French accounting systems.
/// - `cost`: the initial cost of the asset.
/// - `date_purchased`: the date the asset was purchased.
/// - `first_period_end`: the date of the end of the first period.
/// - `salvage`: the salvage value at the end of the asset's life.
/// - `period`: the period for which to calculate depreciation.
/// - `rate`: the rate of depreciation.
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the depreciation amount for the specified period, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn amor_linc(
    cost: Value,
    date_purchased: Value,
    first_period_end: Value,
    salvage: Value,
    period: Value,
    rate: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let cost = cost.f64(value_format)?;
    let date_purchased = date_purchased.date_time(value_format)?;
    let first_period_end = first_period_end.date_time(value_format)?;
    let salvage = salvage.f64(value_format)?;
    let period = period.i32(value_format)?;
    let rate = rate.f64(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_amor_linc(
        cost,
        date_purchased,
        first_period_end,
        salvage,
        period,
        rate,
        basis,
    )?))
}

/// Excel-compatible `COUPDAYBS` function.
/// Calculates the number of days from the beginning of the coupon period to the settlement date.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the number of days as an integer, or an error if inputs are invalid.
pub fn coup_day_bs(
    settlement: Value,
    maturity: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::I32(codcel_coup_day_bs(
        settlement, maturity, frequency, basis,
    )?))
}

/// Excel-compatible `COUPDAYS` function.
/// Calculates the number of days in the coupon period that contains the settlement date.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the number of days in the coupon period, or an error if inputs are invalid.
pub fn coup_days(
    settlement: Value,
    maturity: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_coup_days(
        settlement, maturity, frequency, basis,
    )?))
}

/// Excel-compatible `COUPDAYSNC` function.
/// Calculates the number of days from the settlement date to the next coupon date.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the number of days to the next coupon date, or an error if inputs are invalid.
pub fn coup_days_nc(
    settlement: Value,
    maturity: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_coup_days_nc(
        settlement, maturity, frequency, basis,
    )?))
}

/// Excel-compatible `COUPNCD` function.
/// Returns the next coupon date after the settlement date.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the next coupon date as a datetime, or an error if inputs are invalid.
pub fn coup_ncd(
    settlement: Value,
    maturity: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::ChronoDateTime(codcel_coup_ncd(
        settlement, maturity, frequency, basis,
    )?))
}

/// Excel-compatible `COUPNUM` function.
/// Returns the number of coupon payments between the settlement date and maturity date.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the number of remaining coupon payments as an integer, or an error if inputs are invalid.
pub fn coup_num(
    settlement: Value,
    maturity: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::I32(codcel_coup_num(
        settlement, maturity, frequency, basis,
    )?))
}

/// Excel-compatible `COUPPCD` function.
/// Returns the previous coupon date before the settlement date.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the previous coupon date as a datetime, or an error if inputs are invalid.
pub fn coup_pcd(
    settlement: Value,
    maturity: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::ChronoDateTime(codcel_coup_pcd(
        settlement, maturity, frequency, basis,
    )?))
}

/// Excel-compatible `CUMIPMT` function.
/// Calculates the cumulative interest paid on a loan between two periods.
/// - `rate`: the interest rate per period.
/// - `nper`: the total number of payment periods.
/// - `pv`: the present value (principal amount of the loan).
/// - `start_period`: the first period in the calculation (must be >= 1).
/// - `end_period`: the last period in the calculation (must be >= start_period).
/// - `payment_type`: when payments are due (0 = end of period, 1 = beginning of period).
///
/// Returns the cumulative interest paid, or an error if inputs are invalid.
pub fn cum_i_pmt(
    rate: Value,
    nper: Value,
    pv: Value,
    start_period: Value,
    end_period: Value,
    payment_type: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let rate = rate.f64(value_format)?;
    let nper = nper.i32(value_format)?;
    let pv = pv.f64(value_format)?;
    let start_period = start_period.i32(value_format)?;
    let end_period = end_period.i32(value_format)?;
    let payment_type = payment_type.i32(value_format)?;

    Ok(Value::F64(codcel_cum_i_pmt(
        rate,
        nper,
        pv,
        start_period,
        end_period,
        payment_type,
    )?))
}

/// Excel-compatible `CUMPRINC` function.
/// Calculates the cumulative principal paid on a loan between two periods.
/// - `rate`: the interest rate per period.
/// - `nper`: the total number of payment periods.
/// - `pv`: the present value (principal amount of the loan).
/// - `start_period`: the first period in the calculation (must be >= 1).
/// - `end_period`: the last period in the calculation (must be >= start_period).
/// - `payment_type`: when payments are due (0 = end of period, 1 = beginning of period).
///
/// Returns the cumulative principal paid, or an error if inputs are invalid.
pub fn cum_princ(
    rate: Value,
    nper: Value,
    pv: Value,
    start_period: Value,
    end_period: Value,
    payment_type: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let rate = rate.f64(value_format)?;
    let nper = nper.i32(value_format)?;
    let pv = pv.f64(value_format)?;
    let start_period = start_period.i32(value_format)?;
    let end_period = end_period.i32(value_format)?;
    let payment_type = payment_type.i32(value_format)?;

    Ok(Value::F64(codcel_cum_princ(
        rate,
        nper,
        pv,
        start_period,
        end_period,
        payment_type,
    )?))
}

/// Excel-compatible `DB` function.
/// Calculates the depreciation of an asset using the fixed-declining balance method.
/// - `cost`: the initial cost of the asset.
/// - `salvage`: the salvage value at the end of the asset's useful life.
/// - `life`: the number of periods over which the asset is depreciated (useful life).
/// - `period`: the period for which to calculate depreciation (must be in the same units as life).
/// - `month`: optional number of months in the first year (defaults to 12).
///
/// Returns the depreciation for the specified period, or an error if inputs are invalid.
pub fn db(
    cost: Value,
    salvage: Value,
    life: Value,
    period: Value,
    month: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_int_opt_int_to_float(
        cost,
        salvage,
        life,
        period,
        month,
        strict_type_conversion,
        value_format,
        "DB",
        codcel_db,
    )
}

/// Excel-compatible `DDB` function.
/// Calculates the depreciation of an asset using the double-declining balance method or another specified factor.
/// - `cost`: the initial cost of the asset.
/// - `salvage`: the salvage value at the end of the asset's useful life.
/// - `life`: the number of periods over which the asset is depreciated (useful life).
/// - `period`: the period for which to calculate depreciation.
/// - `factor`: optional rate at which the balance declines (defaults to 2 for double-declining).
///
/// Returns the depreciation for the specified period, or an error if inputs are invalid.
pub fn ddb(
    cost: Value,
    salvage: Value,
    life: Value,
    period: Value,
    factor: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_float_opt_float_to_float(
        cost,
        salvage,
        life,
        period,
        factor,
        strict_type_conversion,
        value_format,
        "DDB",
        codcel_ddb,
    )
}

/// Excel-compatible `DISC` function.
/// Calculates the discount rate for a security.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `price`: the security's price per $100 face value.
/// - `redemption`: the security's redemption value per $100 face value.
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the discount rate, or an error if inputs are invalid.
pub fn disc(
    settlement: Value,
    maturity: Value,
    price: Value,
    redemption: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let price = price.f64(value_format)?;
    let redemption = redemption.f64(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_disc(
        settlement, maturity, price, redemption, basis,
    )?))
}

/// Excel-compatible `DOLLARDE` function.
/// Converts a dollar price expressed as a fraction into a decimal number.
/// - `fractional_dollar`: a number expressed as an integer part and a fraction part, separated by a decimal point.
/// - `fraction`: the integer to use in the denominator of the fraction.
///
/// Returns the decimal representation of the fractional dollar price, or an error if inputs are invalid.
pub fn dollar_de(
    fractional_dollar: Value,
    fraction: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let fractional_dollar = fractional_dollar.f64(value_format)?;
    let fraction = fraction.i32(value_format)?;

    Ok(Value::F64(codcel_dollar_de(fractional_dollar, fraction)?))
}

/// Excel-compatible `DOLLARFR` function.
/// Converts a decimal dollar price into a fractional dollar price.
/// - `fractional_dollar`: a decimal number.
/// - `fraction`: the integer to use in the denominator of the fraction.
///
/// Returns the fractional representation of the decimal dollar price, or an error if inputs are invalid.
pub fn dollar_fr(
    fractional_dollar: Value,
    fraction: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let fractional_dollar = fractional_dollar.f64(value_format)?;
    let fraction = fraction.i32(value_format)?;

    Ok(Value::F64(codcel_dollar_fr(fractional_dollar, fraction)?))
}

/// Excel-compatible `DURATION` function.
/// Calculates the Macaulay duration of a security with periodic interest payments.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `coupon`: the security's annual coupon rate.
/// - `yield_rate`: the security's annual yield.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the Macaulay duration in years, or an error if inputs are invalid.
pub fn duration(
    settlement: Value,
    maturity: Value,
    coupon: Value,
    yield_rate: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let coupon = coupon.f64(value_format)?;
    let yield_rate = yield_rate.f64(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_duration(
        settlement, maturity, coupon, yield_rate, frequency, basis,
    )?))
}

/// Excel-compatible `EFFECT` function.
/// Calculates the effective annual interest rate given the nominal annual interest rate and number of compounding periods per year.
/// - `nominal_rate`: the nominal annual interest rate.
/// - `npery`: the number of compounding periods per year.
///
/// Returns the effective annual interest rate, or an error if inputs are invalid.
pub fn effect(
    nominal_rate: Value,
    npery: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let nominal_rate = nominal_rate.f64(value_format)?;
    let npery = npery.i32(value_format)?;
    Ok(Value::F64(codcel_effect(nominal_rate, npery)?))
}

/// Excel-compatible `FV` function.
/// Calculates the future value of an investment based on periodic, constant payments and a constant interest rate.
/// - `rate`: the interest rate per period.
/// - `nper`: the total number of payment periods.
/// - `pmt`: the payment made each period (negative for payments out).
/// - `pv`: optional present value or lump-sum amount (defaults to 0).
/// - `type_`: optional timing of payments (0 = end of period, 1 = beginning of period; defaults to 0).
///
/// Returns the future value of the investment, or an error if inputs are invalid.
pub fn fv(
    rate: Value,
    nper: Value,
    pmt: Value,
    pv: Value,
    type_: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_opt_float_opt_int_to_float(
        rate,
        nper,
        pmt,
        pv,
        type_,
        strict_type_conversion,
        value_format,
        "FV",
        codcel_fv,
    )
}

/// Excel-compatible `FVSCHEDULE` function.
/// Calculates the future value of an initial principal after applying a series of compound interest rates.
/// - `principal`: the initial investment amount.
/// - `schedule`: an array or range of interest rates to apply.
///
/// Returns the future value after all interest rates have been applied, or an error if inputs are invalid.
pub fn fv_schedule(
    principal: Value,
    schedule: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let principal = principal.f64(value_format)?;
    let schedule = schedule.to_flatterned_vec_f64(value_format)?;

    Ok(Value::F64(codcel_fv_schedule(principal, schedule)?))
}

/// Excel-compatible `INTRATE` function.
/// Calculates the interest rate for a fully invested security.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `investment`: the amount invested in the security.
/// - `redemption`: the amount to be received at maturity.
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the interest rate, or an error if inputs are invalid.
pub fn int_rate(
    settlement: Value,
    maturity: Value,
    investment: Value,
    redemption: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let investment = investment.f64(value_format)?;
    let redemption = redemption.f64(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_int_rate(
        settlement, maturity, investment, redemption, basis,
    )?))
}

/// Excel-compatible `IPMT` function.
/// Calculates the interest payment for a given period of an investment based on periodic, constant payments and a constant interest rate.
/// - `rate`: the interest rate per period.
/// - `per`: the period for which to calculate interest (must be between 1 and nper).
/// - `nper`: the total number of payment periods.
/// - `pv`: the present value (principal amount of the loan).
/// - `fv`: optional future value or cash balance after the last payment (defaults to 0).
/// - `type_`: optional timing of payments (0 = end of period, 1 = beginning of period; defaults to 0).
///
/// Returns the interest portion of the payment for the specified period, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn i_pmt(
    rate: Value,
    per: Value,
    nper: Value,
    pv: Value,
    fv: Value,
    type_: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_int_int_float_opt_float_opt_int_to_float(
        rate,
        per,
        nper,
        pv,
        fv,
        type_,
        strict_type_conversion,
        value_format,
        "IPMT",
        codcel_i_pmt,
    )
}

/// Excel-compatible `IRR` function.
/// Calculates the internal rate of return for a series of cash flows.
/// - `cash_flows`: an array or range of cash flows (must contain at least one positive and one negative value).
/// - `guess`: optional initial guess for the rate (defaults to 0.1 or 10%).
///
/// Returns the internal rate of return, or an error if the calculation does not converge or inputs are invalid.
pub fn irr(
    cash_flows: Value,
    guess: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let guess = guess.option_f64(value_format)?;
    let cash_flows = cash_flows.to_flatterned_vec_f64(value_format)?;

    Ok(Value::F64(codcel_irr(cash_flows, guess)?))
}

/// Excel-compatible `ISPMT` function.
/// Calculates the interest paid during a specific period of an investment using a simple interest calculation.
/// - `rate`: the interest rate per period.
/// - `per`: the period for which to calculate interest (must be between 1 and nper).
/// - `nper`: the total number of payment periods.
/// - `pv`: the present value (principal amount of the loan).
///
/// Returns the interest for the specified period, or an error if inputs are invalid.
pub fn is_pmt(
    rate: Value,
    per: Value,
    nper: Value,
    pv: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_float_to_float(
        rate,
        per,
        nper,
        pv,
        strict_type_conversion,
        value_format,
        "ISPMT",
        codcel_is_pmt,
    )
}

/// Excel-compatible `MDURATION` function.
/// Calculates the modified Macaulay duration of a security with an assumed par value of $100.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `coupon`: the security's annual coupon rate.
/// - `yield_rate`: the security's annual yield.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the modified duration in years, or an error if inputs are invalid.
pub fn m_duration(
    settlement: Value,
    maturity: Value,
    coupon: Value,
    yield_rate: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let coupon = coupon.f64(value_format)?;
    let yield_rate = yield_rate.f64(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_m_duration(
        settlement, maturity, coupon, yield_rate, frequency, basis,
    )?))
}

/// Excel-compatible `MIRR` function.
/// Calculates the modified internal rate of return for a series of periodic cash flows, considering both the cost of investment and the interest received on reinvestment of cash.
/// - `cash_flows`: an array or range of cash flows (must contain at least one positive and one negative value).
/// - `finance_rate`: the interest rate paid on money used in the cash flows.
/// - `reinvest_rate`: the interest rate received on cash flows as they are reinvested.
///
/// Returns the modified internal rate of return, or an error if inputs are invalid.
pub fn m_irr(
    cash_flows: Value,
    finance_rate: Value,
    reinvest_rate: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let cash_flows = cash_flows.to_flatterned_vec_f64(value_format)?;
    let finance_rate = finance_rate.f64(value_format)?;
    let reinvest_rate = reinvest_rate.f64(value_format)?;

    Ok(Value::F64(codcel_m_irr(
        cash_flows,
        finance_rate,
        reinvest_rate,
    )?))
}

/// Excel-compatible `NOMINAL` function.
/// Calculates the nominal annual interest rate given the effective annual interest rate and number of compounding periods per year.
/// - `effect_rate`: the effective annual interest rate.
/// - `npery`: the number of compounding periods per year.
///
/// Returns the nominal annual interest rate, or an error if inputs are invalid.
pub fn nominal(
    effect_rate: Value,
    npery: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let effect_rate = effect_rate.f64(value_format)?;
    let npery = npery.i32(value_format)?;
    Ok(Value::F64(codcel_nominal(effect_rate, npery)?))
}

/// Excel-compatible `NPER` function.
/// Calculates the number of periods for an investment based on periodic, constant payments and a constant interest rate.
/// - `rate`: the interest rate per period.
/// - `pmt`: the payment made each period (must be constant).
/// - `pv`: the present value or lump-sum amount.
/// - `fv`: optional future value or cash balance after the last payment (defaults to 0).
/// - `type_`: optional timing of payments (0 = end of period, 1 = beginning of period; defaults to 0).
///
/// Returns the number of payment periods, or an error if inputs are invalid.
pub fn n_per(
    rate: Value,
    pmt: Value,
    pv: Value,
    fv: Value,
    type_: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_opt_float_opt_int_to_float(
        rate,
        pmt,
        pv,
        fv,
        type_,
        strict_type_conversion,
        value_format,
        "NPER",
        codcel_n_per,
    )
}

/// Excel-compatible `NPV` function.
/// Calculates the net present value of an investment based on a discount rate and a series of future cash flows.
/// - `rate`: the discount rate per period.
/// - `cash_flows`: an array or range of future cash flows (assumed to occur at the end of each period).
///
/// Returns the net present value, or an error if inputs are invalid.
///
/// Note: Unlike some implementations, the first cash flow is assumed to occur at the end of period 1.
pub fn npv(
    rate: Value,
    cash_flows: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let cash_flows = cash_flows.to_flatterned_vec_f64(value_format)?;
    let rate = rate.f64(value_format)?;

    Ok(Value::F64(codcel_npv(rate, cash_flows)?))
}

/// Excel-compatible `ODDFPRICE` function.
/// Calculates the price per $100 face value of a security with an odd (short or long) first period.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `issue`: the security's issue date.
/// - `first_coupon`: the security's first coupon date.
/// - `rate`: the security's annual coupon rate.
/// - `yield_rate`: the security's annual yield.
/// - `redemption`: the security's redemption value per $100 face value.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the price per $100 face value, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn odd_f_price(
    settlement: Value,
    maturity: Value,
    issue: Value,
    first_coupon: Value,
    rate: Value,
    yield_rate: Value,
    redemption: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let issue = issue.date_time(value_format)?;
    let first_coupon = first_coupon.date_time(value_format)?;
    let rate = rate.f64(value_format)?;
    let yield_rate = yield_rate.f64(value_format)?;
    let redemption = redemption.f64(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_odd_f_price(
        settlement,
        maturity,
        issue,
        first_coupon,
        rate,
        yield_rate,
        redemption,
        frequency,
        basis,
    )?))
}

/// Excel-compatible `ODDFYIELD` function.
/// Calculates the yield of a security with an odd (short or long) first period.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `issue`: the security's issue date.
/// - `first_coupon`: the security's first coupon date.
/// - `rate`: the security's annual coupon rate.
/// - `price`: the security's price per $100 face value.
/// - `redemption`: the security's redemption value per $100 face value.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the annual yield, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn odd_f_yield(
    settlement: Value,
    maturity: Value,
    issue: Value,
    first_coupon: Value,
    rate: Value,
    price: Value,
    redemption: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let issue = issue.date_time(value_format)?;
    let first_coupon = first_coupon.date_time(value_format)?;
    let rate = rate.f64(value_format)?;
    let price = price.f64(value_format)?;
    let redemption = redemption.f64(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_odd_f_yield(
        settlement,
        maturity,
        issue,
        first_coupon,
        rate,
        price,
        redemption,
        frequency,
        basis,
    )?))
}

/// Excel-compatible `ODDLPRICE` function.
/// Calculates the price per $100 face value of a security with an odd (short or long) last coupon period.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `last_interest`: the security's last interest (coupon) date before maturity.
/// - `rate`: the security's annual coupon rate.
/// - `yield_rate`: the security's annual yield.
/// - `redemption`: the security's redemption value per $100 face value.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the price per $100 face value, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn odd_l_price(
    settlement: Value,
    maturity: Value,
    last_interest: Value,
    rate: Value,
    yield_rate: Value,
    redemption: Value,
    frequency: Value,
    basis: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_datetime_datetime_datetime_float_float_float_int_opt_int_to_float(
        settlement,
        maturity,
        last_interest,
        rate,
        yield_rate,
        redemption,
        frequency,
        basis,
        strict_type_conversion,
        value_format,
        "ODDLPRICE",
        codcel_odd_l_price,
    )
}

/// Excel-compatible `ODDLYIELD` function.
/// Calculates the yield of a security with an odd (short or long) last coupon period.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `last_interest`: the security's last interest (coupon) date before maturity.
/// - `rate`: the security's annual coupon rate.
/// - `yield_rate`: the security's price per $100 face value.
/// - `redemption`: the security's redemption value per $100 face value.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the annual yield, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn odd_l_yield(
    settlement: Value,
    maturity: Value,
    last_interest: Value,
    rate: Value,
    yield_rate: Value,
    redemption: Value,
    frequency: Value,
    basis: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_datetime_datetime_datetime_float_float_float_int_opt_int_to_float(
        settlement,
        maturity,
        last_interest,
        rate,
        yield_rate,
        redemption,
        frequency,
        basis,
        strict_type_conversion,
        value_format,
        "ODDLYIELD",
        codcel_odd_l_yield,
    )
}

/// Excel-compatible `PDURATION` function.
/// Calculates the number of periods required for an investment to reach a specified value.
/// - `rate`: the interest rate per period.
/// - `present_value`: the present value of the investment.
/// - `future_value`: the desired future value of the investment.
///
/// Returns the number of periods required, or an error if inputs are invalid (e.g., rate <= 0).
pub fn p_duration(
    rate: Value,
    present_value: Value,
    future_value: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![rate, present_value, future_value],
        strict_type_conversion,
        value_format,
        "PDURATION",
        codcel_p_duration_vec,
    )
}

/// Excel-compatible `PMT` function.
/// Calculates the payment for a loan based on constant payments and a constant interest rate.
/// - `rate`: the interest rate per period.
/// - `nper`: the total number of payment periods.
/// - `pv`: the present value (principal amount of the loan).
/// - `fv`: optional future value or cash balance after the last payment (defaults to 0).
/// - `type_`: optional timing of payments (0 = end of period, 1 = beginning of period; defaults to 0).
///
/// Returns the periodic payment amount (negative for payments out), or an error if inputs are invalid.
pub fn pmt(
    rate: Value,
    nper: Value,
    pv: Value,
    fv: Value,
    type_: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_opt_float_opt_int_to_float(
        rate,
        nper,
        pv,
        fv,
        type_,
        strict_type_conversion,
        value_format,
        "PMT",
        codcel_pmt,
    )
}

/// Excel-compatible `PPMT` function.
/// Calculates the principal payment for a given period of an investment based on periodic, constant payments and a constant interest rate.
/// - `rate`: the interest rate per period.
/// - `per`: the period for which to calculate principal (must be between 1 and nper).
/// - `nper`: the total number of payment periods.
/// - `pv`: the present value (principal amount of the loan).
/// - `fv`: optional future value or cash balance after the last payment (defaults to 0).
/// - `type_`: optional timing of payments (0 = end of period, 1 = beginning of period; defaults to 0).
///
/// Returns the principal portion of the payment for the specified period, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn p_pmt(
    rate: Value,
    per: Value,
    nper: Value,
    pv: Value,
    fv: Value,
    type_: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_int_float_float_opt_float_opt_int_to_float(
        rate,
        per,
        nper,
        pv,
        fv,
        type_,
        strict_type_conversion,
        value_format,
        "PPMT",
        codcel_p_pmt,
    )
}

/// Excel-compatible `PRICE` function.
/// Calculates the price per $100 face value of a security that pays periodic interest.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `rate`: the security's annual coupon rate.
/// - `yld`: the security's annual yield.
/// - `redemption`: the security's redemption value per $100 face value.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the price per $100 face value, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn price(
    settlement: Value,
    maturity: Value,
    rate: Value,
    yld: Value,
    redemption: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let rate = rate.f64(value_format)?;
    let yld = yld.f64(value_format)?;
    let redemption = redemption.f64(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_price(
        settlement, maturity, rate, yld, redemption, frequency, basis,
    )?))
}

/// Excel-compatible `PRICEDISC` function.
/// Calculates the price per $100 face value of a discounted security.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `discount`: the security's discount rate.
/// - `redemption`: the security's redemption value per $100 face value.
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the price per $100 face value, or an error if inputs are invalid.
pub fn price_disc(
    settlement: Value,
    maturity: Value,
    discount: Value,
    redemption: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let discount = discount.f64(value_format)?;
    let redemption = redemption.f64(value_format)?;
    let basis = basis.option_i32(value_format)?;
    Ok(Value::F64(codcel_price_disc(
        settlement, maturity, discount, redemption, basis,
    )?))
}

/// Excel-compatible `PRICEMAT` function.
/// Calculates the price per $100 face value of a security that pays interest at maturity.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `issue`: the security's issue date.
/// - `rate`: the security's annual coupon rate.
/// - `yield_rate`: the security's annual yield.
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the price per $100 face value, or an error if inputs are invalid.
pub fn price_mat(
    settlement: Value,
    maturity: Value,
    issue: Value,
    rate: Value,
    yield_rate: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let issue = issue.date_time(value_format)?;
    let rate = rate.f64(value_format)?;
    let yield_rate = yield_rate.f64(value_format)?;
    let basis = basis.option_i32(value_format)?;
    Ok(Value::F64(codcel_price_mat(
        settlement, maturity, issue, rate, yield_rate, basis,
    )?))
}

/// Excel-compatible `PV` function.
/// Calculates the present value of an investment based on periodic, constant payments and a constant interest rate.
/// - `rate`: the interest rate per period.
/// - `nper`: the total number of payment periods.
/// - `pmt`: the payment made each period (negative for payments out).
/// - `fv`: optional future value or cash balance after the last payment (defaults to 0).
/// - `type_`: optional timing of payments (0 = end of period, 1 = beginning of period; defaults to 0).
///
/// Returns the present value of the investment, or an error if inputs are invalid.
pub fn pv(
    rate: Value,
    nper: Value,
    pmt: Value,
    fv: Value,
    type_: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_opt_float_opt_int_to_float(
        rate,
        nper,
        pmt,
        fv,
        type_,
        strict_type_conversion,
        value_format,
        "PV",
        codcel_pv,
    )
}

/// Excel-compatible `RATE` function.
/// Calculates the interest rate per period of an annuity using iteration.
/// - `nper`: the total number of payment periods.
/// - `pmt`: the payment made each period (must be constant).
/// - `pv`: the present value (principal amount of the loan).
/// - `fv`: optional future value or cash balance after the last payment (defaults to 0).
/// - `type_`: optional timing of payments (0 = end of period, 1 = beginning of period; defaults to 0).
/// - `guess`: optional initial guess for the rate (defaults to 0.1 or 10%).
///
/// Returns the interest rate per period, or an error if the calculation does not converge.
#[allow(clippy::too_many_arguments)]
pub fn rate(
    nper: Value,
    pmt: Value,
    pv: Value,
    fv: Value,
    type_: Value,
    guess: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_opt_float_opt_int_opt_float_to_float(
        nper,
        pmt,
        pv,
        fv,
        type_,
        guess,
        strict_type_conversion,
        value_format,
        "RATE",
        codcel_rate,
    )
}

/// Excel-compatible `RECEIVED` function.
/// Calculates the amount received at maturity for a fully invested security.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `investment`: the amount invested in the security.
/// - `discount`: the security's discount rate.
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the amount received at maturity, or an error if inputs are invalid.
pub fn received(
    settlement: Value,
    maturity: Value,
    investment: Value,
    discount: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let investment = investment.f64(value_format)?;
    let discount = discount.f64(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_received(
        settlement, maturity, investment, discount, basis,
    )?))
}

/// Excel-compatible `RRI` function.
/// Calculates an equivalent interest rate for the growth of an investment.
/// - `nper`: the number of periods for the investment.
/// - `pv`: the present value of the investment.
/// - `fv`: the future value of the investment.
///
/// Returns the equivalent interest rate, or an error if inputs are invalid (e.g., nper = 0).
pub fn rri(
    nper: Value,
    pv: Value,
    fv: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![nper, pv, fv],
        strict_type_conversion,
        value_format,
        "RRI",
        codcel_rri_vec,
    )
}

/// Excel-compatible `SLN` function.
/// Calculates the straight-line depreciation of an asset for one period.
/// - `cost`: the initial cost of the asset.
/// - `salvage`: the salvage value at the end of the asset's useful life.
/// - `life`: the number of periods over which the asset is depreciated (useful life).
///
/// Returns the depreciation per period, or an error if inputs are invalid (e.g., life = 0).
pub fn sln(
    cost: Value,
    salvage: Value,
    life: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![cost, salvage, life],
        strict_type_conversion,
        value_format,
        "SLN",
        codcel_sln_vec,
    )
}

/// Excel-compatible `SYD` function.
/// Calculates the sum-of-years' digits depreciation of an asset for a specified period.
/// - `cost`: the initial cost of the asset.
/// - `salvage`: the salvage value at the end of the asset's useful life.
/// - `life`: the number of periods over which the asset is depreciated (useful life).
/// - `period`: the period for which to calculate depreciation (must be between 1 and life).
///
/// Returns the depreciation for the specified period, or an error if inputs are invalid.
pub fn syd(
    cost: Value,
    salvage: Value,
    life: Value,
    period: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![cost, salvage, life, period],
        strict_type_conversion,
        value_format,
        "SYD",
        codcel_syd_vec,
    )
}

/// Excel-compatible `TBILLEQ` function.
/// Calculates the bond-equivalent yield for a Treasury bill.
/// - `settlement`: the Treasury bill's settlement date.
/// - `maturity`: the Treasury bill's maturity date (must be within one year of settlement).
/// - `discount`: the Treasury bill's discount rate.
///
/// Returns the bond-equivalent yield, or an error if inputs are invalid.
pub fn t_bill_eq(
    settlement: Value,
    maturity: Value,
    discount: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let discount = discount.f64(value_format)?;

    Ok(Value::F64(codcel_t_bill_eq(
        settlement, maturity, discount,
    )?))
}

/// Excel-compatible `TBILLPRICE` function.
/// Calculates the price per $100 face value for a Treasury bill.
/// - `settlement`: the Treasury bill's settlement date.
/// - `maturity`: the Treasury bill's maturity date (must be within one year of settlement).
/// - `discount`: the Treasury bill's discount rate.
///
/// Returns the price per $100 face value, or an error if inputs are invalid.
pub fn t_bill_price(
    settlement: Value,
    maturity: Value,
    discount: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let discount = discount.f64(value_format)?;

    Ok(Value::F64(codcel_t_bill_price(
        settlement, maturity, discount,
    )?))
}

/// Excel-compatible `TBILLYIELD` function.
/// Calculates the yield for a Treasury bill.
/// - `settlement`: the Treasury bill's settlement date.
/// - `maturity`: the Treasury bill's maturity date (must be within one year of settlement).
/// - `price`: the Treasury bill's price per $100 face value.
///
/// Returns the yield, or an error if inputs are invalid.
pub fn t_bill_yield(
    settlement: Value,
    maturity: Value,
    price: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let price = price.f64(value_format)?;

    Ok(Value::F64(codcel_t_bill_yield(
        settlement, maturity, price,
    )?))
}

/// Excel-compatible `VDB` function.
/// Calculates the depreciation of an asset for any specified period using the variable declining balance method.
/// - `cost`: the initial cost of the asset.
/// - `salvage`: the salvage value at the end of the asset's useful life.
/// - `life`: the number of periods over which the asset is depreciated (useful life).
/// - `start_period`: the starting period for the depreciation calculation (fractional periods allowed).
/// - `end_period`: the ending period for the depreciation calculation (fractional periods allowed).
/// - `factor`: optional rate at which the balance declines (defaults to 2 for double-declining).
/// - `no_switch`: optional flag; if true, does not switch to straight-line depreciation even when it would give a greater depreciation.
///
/// Returns the depreciation for the specified period range, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn vdb(
    cost: Value,
    salvage: Value,
    life: Value,
    start_period: Value,
    end_period: Value,
    factor: Value,
    no_switch: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_float_float_opt_float_opt_bool_to_float(
        cost,
        salvage,
        life,
        start_period,
        end_period,
        factor,
        no_switch,
        strict_type_conversion,
        value_format,
        "VDB",
        codcel_vdb,
    )
}

/// Excel-compatible `XIRR` function.
/// Calculates the internal rate of return for a schedule of cash flows that is not necessarily periodic.
/// - `cash_flows`: an array or range of cash flows (must contain at least one positive and one negative value).
/// - `dates`: an array or range of dates corresponding to each cash flow.
/// - `guess`: optional initial guess for the rate (defaults to 0.1 or 10%).
///
/// Returns the internal rate of return, or an error if the calculation does not converge or inputs are invalid.
pub fn x_irr(
    cash_flows: Value,
    dates: Value,
    guess: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let cash_flows = cash_flows.to_flatterned_vec_f64(value_format)?;
    let dates = dates.to_flatterned_vec_date_time(strict_type_conversion, value_format)?;
    let guess = guess.option_f64(value_format)?;

    Ok(Value::F64(codcel_x_irr(cash_flows, dates, guess)?))
}

/// Excel-compatible `XNPV` function.
/// Calculates the net present value for a schedule of cash flows that is not necessarily periodic.
/// - `rate`: the discount rate to apply to the cash flows.
/// - `cash_flows`: an array or range of cash flows.
/// - `dates`: an array or range of dates corresponding to each cash flow.
///
/// Returns the net present value, or an error if inputs are invalid.
pub fn x_npv(
    rate: Value,
    cash_flows: Value,
    dates: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let rate = rate.f64(value_format)?;
    let cash_flows = cash_flows.to_flatterned_vec_f64(value_format)?;
    let dates = dates.to_flatterned_vec_date_time(strict_type_conversion, value_format)?;

    Ok(Value::F64(codcel_x_npv(rate, cash_flows, dates)?))
}

/// Excel-compatible `YIELD` function.
/// Calculates the yield on a security that pays periodic interest.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `rate`: the security's annual coupon rate.
/// - `price`: the security's price per $100 face value.
/// - `redemption`: the security's redemption value per $100 face value.
/// - `frequency`: number of coupon payments per year (1 = annual, 2 = semi-annual, 4 = quarterly).
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the annual yield, or an error if inputs are invalid.
#[allow(clippy::too_many_arguments)]
pub fn yield_(
    settlement: Value,
    maturity: Value,
    rate: Value,
    price: Value,
    redemption: Value,
    frequency: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let rate = rate.f64(value_format)?;
    let price = price.f64(value_format)?;
    let redemption = redemption.f64(value_format)?;
    let frequency = frequency.i32(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_yield(
        settlement, maturity, rate, price, redemption, frequency, basis,
    )?))
}

/// Excel-compatible `YIELDDISC` function.
/// Calculates the annual yield for a discounted security.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `price`: the security's price per $100 face value.
/// - `redemption`: the security's redemption value per $100 face value.
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the annual yield, or an error if inputs are invalid.
pub fn yield_disc(
    settlement: Value,
    maturity: Value,
    price: Value,
    redemption: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let price = price.f64(value_format)?;
    let redemption = redemption.f64(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_yield_disc(
        settlement, maturity, price, redemption, basis,
    )?))
}

/// Excel-compatible `YIELDMAT` function.
/// Calculates the annual yield of a security that pays interest at maturity.
/// - `settlement`: the security's settlement date.
/// - `maturity`: the security's maturity date.
/// - `issue`: the security's issue date.
/// - `rate`: the security's annual coupon rate.
/// - `price`: the security's price per $100 face value.
/// - `basis`: optional day count basis (0 = US 30/360, 1 = actual/actual, 2 = actual/360, 3 = actual/365, 4 = European 30/360).
///
/// Returns the annual yield, or an error if inputs are invalid.
pub fn yield_mat(
    settlement: Value,
    maturity: Value,
    issue: Value,
    rate: Value,
    price: Value,
    basis: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let settlement = settlement.date_time(value_format)?;
    let maturity = maturity.date_time(value_format)?;
    let issue = issue.date_time(value_format)?;
    let rate = rate.f64(value_format)?;
    let price = price.f64(value_format)?;
    let basis = basis.option_i32(value_format)?;

    Ok(Value::F64(codcel_yield_mat(
        settlement, maturity, issue, rate, price, basis,
    )?))
}
