// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::area::{
    process_area_float_bool_to_float, process_area_float_float_bool_to_float,
    process_area_float_float_float_bool_to_float,
    process_area_float_float_float_float_bool_to_float,
    process_area_float_float_float_opt_float_opt_float_to_float,
    process_area_float_float_int_to_float, process_area_float_multi_to_float,
    process_area_int_float_bool_to_float, process_area_int_float_float_to_int,
    process_area_int_float_int_opt_int, process_area_int_int_float_bool_to_float,
};
use crate::statistical::codcel_beta_dist::codcel_beta_dist;
use crate::statistical::codcel_beta_dot_inv::codcel_beta_dot_inv;
use crate::statistical::codcel_binom_dot_dist::codcel_binom_dot_dist;
use crate::statistical::codcel_binom_dot_dist_dot_range::codcel_binom_dot_dist_dot_range;
use crate::statistical::codcel_binom_inv::codcel_binom_inv;
use crate::statistical::codcel_chisq_dist::codcel_chisq_dist;
use crate::statistical::codcel_chisq_dist_rt::codcel_chisq_dist_rt_vec;
use crate::statistical::codcel_chisq_inv::codcel_chisq_inv_vec;
use crate::statistical::codcel_chisq_inv_rt::codcel_chisq_inv_rt_vec;
use crate::statistical::codcel_confidence_norm::codcel_confidence_norm;
use crate::statistical::codcel_confidence_t::codcel_confidence_t;
use crate::statistical::codcel_correl::codcel_correl;
use crate::statistical::codcel_covariance_p::codcel_covariance_p;
use crate::statistical::codcel_covariance_s::codcel_covariance_s;
use crate::statistical::codcel_expon_dot_dist::codcel_expon_dot_dist;
use crate::statistical::codcel_f_dist_rt::codcel_f_dist_rt_vec;
use crate::statistical::codcel_f_dot_dist::codcel_f_dot_dist;
use crate::statistical::codcel_f_dot_inv::codcel_f_dot_inv_vec;
use crate::statistical::codcel_f_dot_test::codcel_f_dot_test;
use crate::statistical::codcel_f_inv_rt::codcel_f_inv_rt_vec;
use crate::statistical::codcel_fisher::codcel_fischer_vec;
use crate::statistical::codcel_fisher_inv::codcel_fischer_inv_vec;
use crate::statistical::codcel_forecast::codcel_forecast;
use crate::statistical::codcel_forecast_ets::codcel_forecast_ets;
use crate::statistical::codcel_forecast_ets_confint::codcel_forecast_ets_confint;
use crate::statistical::codcel_forecast_ets_seasonality::codcel_forecast_ets_seasonality;
use crate::statistical::codcel_forecast_ets_stat::codcel_forecast_ets_stat;
use crate::statistical::codcel_frequency::codcel_frequency;
use crate::statistical::codcel_gamma::codcel_gamma_vec;
use crate::statistical::codcel_gamma_dot_dist::codcel_gamma_dot_dist;
use crate::statistical::codcel_gamma_dot_inv::codcel_gamma_dot_inv_vec;
use crate::statistical::codcel_gamma_ln::codcel_gamma_ln_vec;
use crate::statistical::codcel_gamma_ln_precise::codcel_gamma_ln_precise_vec;
use crate::statistical::codcel_gauss::codcel_gauss_vec;
use crate::statistical::codcel_geo_mean::codcel_geo_mean;
use crate::statistical::codcel_growth::codcel_growth;
use crate::statistical::codcel_har_mean::codcel_har_mean;
use crate::statistical::codcel_hypgeom_dot_dist::codcel_hypgeom_dot_dist;
use crate::statistical::codcel_intercept::codcel_intercept;
use crate::statistical::codcel_kurt::codcel_kurt;
use crate::statistical::codcel_large::codcel_large;
use crate::statistical::codcel_linest::codcel_linest;
use crate::statistical::codcel_log_norm_dot_dist::codcel_log_norm_dot_dist;
use crate::statistical::codcel_log_norm_inv::codcel_log_norm_inv_vec;
use crate::statistical::codcel_logest::codcel_logest;
use crate::statistical::codcel_median::codcel_median;
use crate::statistical::codcel_mode_mult::codcel_mode_mult;
use crate::statistical::codcel_mode_sngl::codcel_mode_sngl;
use crate::statistical::codcel_neg_binom_dot_dist::codcel_neg_binom_dot_dist;
use crate::statistical::codcel_norm_dot_dist::codcel_norm_dot_dist;
use crate::statistical::codcel_norm_dot_inv::codcel_norm_dot_inv_vec;
use crate::statistical::codcel_norm_dot_s_dot_dist::codcel_norm_dot_s_dot_dist;
use crate::statistical::codcel_norm_dot_s_dot_inv::codcel_norm_dot_s_dot_inv_vec;
use crate::statistical::codcel_pearson::codcel_pearson;
use crate::statistical::codcel_percent_rank_exc::codcel_percent_rank_exc;
use crate::statistical::codcel_percent_rank_inc::codcel_percent_rank_inc;
use crate::statistical::codcel_percentile_exc::codcel_percentile_exc;
use crate::statistical::codcel_percentile_inc::codcel_percentile_inc;
use crate::statistical::codcel_phi::codcel_phi_vec;
use crate::statistical::codcel_poisson_dist::codcel_poisson_dist;
use crate::statistical::codcel_prob::codcel_prob;
use crate::statistical::codcel_quartile_exc::codcel_quartile_exc;
use crate::statistical::codcel_quartile_inc::codcel_quartile_inc;
use crate::statistical::codcel_rank_avg::codcel_rank_avg;
use crate::statistical::codcel_rank_eq::codcel_rank_eq;
use crate::statistical::codcel_rsq::codcel_rsq;
use crate::statistical::codcel_skew::codcel_skew;
use crate::statistical::codcel_skew_p::codcel_skew_p;
use crate::statistical::codcel_slope::codcel_slope;
use crate::statistical::codcel_st_dev_dot_p::codcel_st_dev_dot_p;
use crate::statistical::codcel_st_dev_s::codcel_st_dev_s;
use crate::statistical::codcel_standardize::codcel_standardize_vec;
use crate::statistical::codcel_stdeva::codcel_stdeva;
use crate::statistical::codcel_stdevpa::codcel_stdevpa;
use crate::statistical::codcel_steyx::codcel_steyx;
use crate::statistical::codcel_t_dist_rt::codcel_t_dist_rt_vec;
use crate::statistical::codcel_t_dot_dist::codcel_t_dot_dist;
use crate::statistical::codcel_t_dot_inv::codcel_t_dot_inv_vec;
use crate::statistical::codcel_t_dot_test::codcel_t_dot_test;
use crate::statistical::codcel_trend::codcel_trend;
use crate::statistical::codcel_trim_mean::codcel_trim_mean;
use crate::statistical::codcel_var_dot_p::{codcel_var_dot_p, codcel_var_p};
use crate::statistical::codcel_var_s::codcel_var_s;
use crate::statistical::codcel_vara::codcel_vara;
use crate::statistical::codcel_varpa::codcel_varpa;
use crate::statistical::codcel_weibull_dist::codcel_weibull_dist;
use crate::statistical::codcel_z_dot_test::codcel_z_dot_test;
use crate::value::{area_f64, flatten_value_to_vec_f64, vec_f64, vec_value_to_vec_f64, Value};
use crate::value_format::ValueFormat;
use crate::statistical::codcel_small::codcel_small;
use std::error::Error;

// Probability Distributions (29 functions)

/// Excel-compatible `BETA.DIST` function.
/// Evaluates the beta distribution.
/// - `x`: value at which to evaluate the distribution (between `a` and `b`).
/// - `alpha`: first shape parameter of the distribution (must be > 0).
/// - `beta`: second shape parameter of the distribution (must be > 0).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
/// - `a`: optional lower bound of the interval (defaults to 0).
/// - `b`: optional upper bound of the interval (defaults to 1).
///
/// Returns an error if `alpha` <= 0, `beta` <= 0, `x` < `a`, `x` > `b`, or `a` >= `b`.
pub fn beta_dist(
    x: Value,
    alpha: Value,
    beta: Value,
    cumulative: Value,
    a: Value,
    b: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;
    let alpha = alpha.f64(value_format)?;
    let beta = beta.f64(value_format)?;
    let cumulative = cumulative.bool(value_format)?;

    let a = a.option_f64(value_format)?;
    let b = b.option_f64(value_format)?;

    Ok(Value::F64(codcel_beta_dist(
        x, alpha, beta, cumulative, a, b,
    )?))
}

/// Excel-compatible `BETA.INV` function.
/// Returns the inverse of the cumulative beta distribution.
/// - `probability`: probability associated with the beta distribution (between 0 and 1).
/// - `alpha`: first shape parameter of the distribution (must be > 0).
/// - `beta`: second shape parameter of the distribution (must be > 0).
/// - `a`: optional lower bound of the interval (defaults to 0).
/// - `b`: optional upper bound of the interval (defaults to 1).
///
/// Returns the value `x` such that `BETA.DIST(x, alpha, beta, TRUE, a, b) = probability`.
pub fn beta_dot_inv(
    probability: Value,
    alpha: Value,
    beta: Value,
    a: Value,
    b: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_opt_float_opt_float_to_float(
        probability,
        alpha,
        beta,
        a,
        b,
        strict_type_conversion,
        value_format,
        "BETA.INV",
        codcel_beta_dot_inv,
    )
}

/// Excel-compatible `BINOM.DIST` function.
/// Evaluates the binomial distribution probability.
/// - `number_s`: number of successes in trials (must be >= 0 and <= `trials`).
/// - `trials`: total number of independent trials (must be >= 0).
/// - `probability_s`: probability of success on each trial (between 0 and 1).
/// - `cumulative`: `true` for cumulative probability `P(X <= number_s)`, `false` for probability mass `P(X = number_s)`.
///
/// Returns an error if parameters are out of valid ranges.
pub fn binom_dot_dist(
    number_s: Value,
    trials: Value,
    probability_s: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_int_float_bool_to_float(
        number_s,
        trials,
        probability_s,
        cumulative,
        strict_type_conversion,
        value_format,
        "BINOM.DIST",
        codcel_binom_dot_dist,
    )
}

/// Excel-compatible `BINOM.DIST.RANGE` function.
/// Returns the probability of a trial result using a binomial distribution.
/// - `trials`: total number of independent trials (must be >= 0).
/// - `probability`: probability of success on each trial (between 0 and 1).
/// - `number_s`: minimum number of successes in trials.
/// - `number_s2`: optional maximum number of successes. If omitted, returns `P(X = number_s)`.
///
/// Returns `P(number_s <= X <= number_s2)` when `number_s2` is provided.
pub fn binom_dot_dist_dot_range(
    trials: Value,
    probability: Value,
    number_s: Value,
    number_s2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_float_int_opt_int(
        trials,
        probability,
        number_s,
        number_s2,
        strict_type_conversion,
        value_format,
        "BINOM.DIST.RANGE",
        codcel_binom_dot_dist_dot_range,
    )
}

/// Excel-compatible `BINOM.INV` function.
/// Returns the smallest value for which the cumulative binomial distribution is >= `alpha`.
/// - `trials`: total number of independent trials (must be >= 0).
/// - `probability_s`: probability of success on each trial (between 0 and 1).
/// - `alpha`: criterion probability (between 0 and 1).
///
/// Returns the smallest integer `k` such that `BINOM.DIST(k, trials, probability_s, TRUE) >= alpha`.
pub fn binom_inv(
    trails: Value,
    probability_s: Value,
    alpha: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_float_float_to_int(
        trails,
        probability_s,
        alpha,
        strict_type_conversion,
        value_format,
        "BINOMINV",
        codcel_binom_inv,
    )
}

/// Excel-compatible `CHISQ.DIST` function.
/// Evaluates the chi-squared distribution.
/// - `x`: value at which to evaluate the distribution (must be >= 0).
/// - `degrees_freedom`: degrees of freedom (must be >= 1 and <= 10^10).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
///
/// Returns an error if `x` < 0 or `degrees_freedom` is out of range.
pub fn chisq_dist(
    x: Value,
    degrees_freedom: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_bool_to_float(
        x,
        degrees_freedom,
        cumulative,
        strict_type_conversion,
        value_format,
        "CHISQ.DIST",
        codcel_chisq_dist,
    )
}

/// Excel-compatible `CHISQ.DIST.RT` function.
/// Returns the right-tailed probability of the chi-squared distribution.
/// - `x`: value at which to evaluate the distribution (must be >= 0).
/// - `degrees_freedom`: degrees of freedom (must be >= 1 and <= 10^10).
///
/// Returns `P(X > x)` where X follows a chi-squared distribution.
pub fn chisq_dist_rt(
    x: Value,
    degrees_freedom: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x, degrees_freedom],
        strict_type_conversion,
        value_format,
        "CHISQ.DIST.RT",
        codcel_chisq_dist_rt_vec,
    )
}

/// Excel-compatible `CHISQ.INV` function.
/// Returns the inverse of the left-tailed probability of the chi-squared distribution.
/// - `probability`: probability associated with the chi-squared distribution (between 0 and 1).
/// - `degrees_freedom`: degrees of freedom (must be >= 1 and <= 10^10).
///
/// Returns the value `x` such that `CHISQ.DIST(x, degrees_freedom, TRUE) = probability`.
pub fn chisq_inv(
    probability: Value,
    degrees_freedom: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![probability, degrees_freedom],
        strict_type_conversion,
        value_format,
        "CHISQ.INV",
        codcel_chisq_inv_vec,
    )
}

/// Excel-compatible `CHISQ.INV.RT` function.
/// Returns the inverse of the right-tailed probability of the chi-squared distribution.
/// - `probability`: right-tailed probability (between 0 and 1).
/// - `degrees_freedom`: degrees of freedom (must be >= 1 and <= 10^10).
///
/// Returns the value `x` such that `CHISQ.DIST.RT(x, degrees_freedom) = probability`.
pub fn chisq_inv_rt(
    probability: Value,
    degrees_freedom: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![probability, degrees_freedom],
        strict_type_conversion,
        value_format,
        "CHISQ.INV.RT",
        codcel_chisq_inv_rt_vec,
    )
}

/// Excel-compatible `EXPON.DIST` function.
/// Evaluates the exponential distribution.
/// - `x`: value at which to evaluate the distribution (must be >= 0).
/// - `lambda`: rate parameter of the distribution (must be > 0).
/// - `cumulative`: `true` for cumulative distribution function `1 - e^(-lambda*x)`, `false` for probability density function `lambda * e^(-lambda*x)`.
///
/// Returns an error if `x` < 0 or `lambda` <= 0.
pub fn expon_dot_dist(
    x: Value,
    lambda: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_bool_to_float(
        x,
        lambda,
        cumulative,
        strict_type_conversion,
        value_format,
        "EXPON.DIST",
        codcel_expon_dot_dist,
    )
}

/// Excel-compatible `F.DIST` function.
/// Evaluates the F probability distribution.
/// - `x`: value at which to evaluate the distribution (must be >= 0).
/// - `df1`: numerator degrees of freedom (must be >= 1).
/// - `df2`: denominator degrees of freedom (must be >= 1).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
///
/// Returns an error if parameters are out of valid ranges.
pub fn f_dot_dist(
    x: Value,
    df1: Value,
    df2: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_bool_to_float(
        x,
        df1,
        df2,
        cumulative,
        strict_type_conversion,
        value_format,
        "F.DIST",
        codcel_f_dot_dist,
    )
}

/// Excel-compatible `F.DIST.RT` function.
/// Returns the right-tailed F probability distribution.
/// - `x`: value at which to evaluate the distribution (must be >= 0).
/// - `df1`: numerator degrees of freedom (must be >= 1).
/// - `df2`: denominator degrees of freedom (must be >= 1).
///
/// Returns `P(X > x)` where X follows an F distribution.
pub fn f_dist_rt(
    x: Value,
    df1: Value,
    df2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x, df1, df2],
        strict_type_conversion,
        value_format,
        "F.DIST.RT",
        codcel_f_dist_rt_vec,
    )
}

/// Excel-compatible `F.INV` function.
/// Returns the inverse of the F probability distribution (left-tailed).
/// - `p`: probability associated with the F distribution (between 0 and 1).
/// - `df1`: numerator degrees of freedom (must be >= 1).
/// - `df2`: denominator degrees of freedom (must be >= 1).
///
/// Returns the value `x` such that `F.DIST(x, df1, df2, TRUE) = p`.
pub fn f_dot_inv(
    p: Value,
    df1: Value,
    df2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![p, df1, df2],
        strict_type_conversion,
        value_format,
        "F.INV",
        codcel_f_dot_inv_vec,
    )
}

/// Excel-compatible `F.INV.RT` function.
/// Returns the inverse of the right-tailed F probability distribution.
/// - `p`: right-tailed probability (between 0 and 1).
/// - `df1`: numerator degrees of freedom (must be >= 1).
/// - `df2`: denominator degrees of freedom (must be >= 1).
///
/// Returns the value `x` such that `F.DIST.RT(x, df1, df2) = p`.
pub fn f_inv_rt(
    p: Value,
    df1: Value,
    df2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![p, df1, df2],
        strict_type_conversion,
        value_format,
        "F.INV.RT",
        codcel_f_inv_rt_vec,
    )
}

/// Excel-compatible `GAMMA` function.
/// Returns the gamma function value.
/// - `x`: value at which to evaluate the gamma function.
///
/// Returns `Γ(x)`. Returns an error if `x` is zero or a negative integer.
pub fn gamma(
    x: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x],
        strict_type_conversion,
        value_format,
        "GAMMA",
        codcel_gamma_vec,
    )
}

/// Excel-compatible `GAMMA.DIST` function.
/// Evaluates the gamma distribution.
/// - `x`: value at which to evaluate the distribution (must be >= 0).
/// - `alpha`: shape parameter (must be > 0).
/// - `beta`: scale parameter (must be > 0).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
///
/// Returns an error if parameters are out of valid ranges.
pub fn gamma_dot_dist(
    x: Value,
    alpha: Value,
    beta: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_bool_to_float(
        x,
        alpha,
        beta,
        cumulative,
        strict_type_conversion,
        value_format,
        "GAMMA.DIST",
        codcel_gamma_dot_dist,
    )
}

/// Excel-compatible `GAMMA.INV` function.
/// Returns the inverse of the gamma cumulative distribution.
/// - `probability`: probability associated with the gamma distribution (between 0 and 1).
/// - `alpha`: shape parameter (must be > 0).
/// - `beta`: scale parameter (must be > 0).
///
/// Returns the value `x` such that `GAMMA.DIST(x, alpha, beta, TRUE) = probability`.
pub fn gamma_dot_inv(
    probability: Value,
    alpha: Value,
    beta: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![probability, alpha, beta],
        strict_type_conversion,
        value_format,
        "GAMMA.INV",
        codcel_gamma_dot_inv_vec,
    )
}

/// Excel-compatible `GAMMALN` function.
/// Returns the natural logarithm of the gamma function.
/// - `x`: value at which to evaluate (must be > 0).
///
/// Returns `ln(Γ(x))`.
pub fn gamma_ln(
    x: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x],
        strict_type_conversion,
        value_format,
        "GAMMALN",
        codcel_gamma_ln_vec,
    )
}

/// Excel-compatible `GAMMALN.PRECISE` function.
/// Returns the natural logarithm of the gamma function with higher precision.
/// - `x`: value at which to evaluate (must be > 0).
///
/// Returns `ln(Γ(x))` with improved numerical precision compared to `GAMMALN`.
pub fn gamma_ln_precise(
    x: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x],
        strict_type_conversion,
        value_format,
        "GAMMALN.PRECISE",
        codcel_gamma_ln_precise_vec,
    )
}

/// Excel-compatible `HYPGEOM.DIST` function.
/// Evaluates the hypergeometric distribution.
/// - `x`: number of successes in the sample (must be >= 0).
/// - `n`: size of the sample (must be >= 0).
/// - `m`: number of successes in the population (must be >= 0).
/// - `k`: population size (must be >= 0).
/// - `cumulative`: `true` for cumulative probability, `false` for probability mass.
///
/// Returns an error if parameters are out of valid ranges or inconsistent.
pub fn hypgeom_dot_dist(
    x: Value,
    n: Value,
    m: Value,
    k: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_float_bool_to_float(
        x,
        n,
        m,
        k,
        cumulative,
        strict_type_conversion,
        value_format,
        "HYPGEOM.DIST",
        codcel_hypgeom_dot_dist,
    )
}

/// Excel-compatible `LOGNORM.DIST` function.
/// Evaluates the lognormal distribution.
/// - `x`: value at which to evaluate the distribution (must be > 0).
/// - `mean`: mean of `ln(x)`.
/// - `std_dev`: standard deviation of `ln(x)` (must be > 0).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
///
/// Returns an error if `x` <= 0 or `std_dev` <= 0.
pub fn log_norm_dot_dist(
    x: Value,
    mean: Value,
    std_dev: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_bool_to_float(
        x,
        mean,
        std_dev,
        cumulative,
        strict_type_conversion,
        value_format,
        "LOGNORM.DIST",
        codcel_log_norm_dot_dist,
    )
}

/// Excel-compatible `LOGNORM.INV` function.
/// Returns the inverse of the lognormal cumulative distribution.
/// - `p`: probability associated with the lognormal distribution (between 0 and 1).
/// - `mean`: mean of `ln(x)`.
/// - `std_dev`: standard deviation of `ln(x)` (must be > 0).
///
/// Returns the value `x` such that `LOGNORM.DIST(x, mean, std_dev, TRUE) = p`.
pub fn log_norm_inv(
    p: Value,
    mean: Value,
    std_dev: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![p, mean, std_dev],
        strict_type_conversion,
        value_format,
        "LOGNORMINV",
        codcel_log_norm_inv_vec,
    )
}

/// Excel-compatible `NEGBINOM.DIST` function.
/// Evaluates the negative binomial distribution.
/// - `failures`: number of failures before the last success (must be >= 0).
/// - `successes`: threshold number of successes (must be >= 1).
/// - `probability`: probability of success on each trial (between 0 and 1).
/// - `cumulative`: `true` for cumulative probability, `false` for probability mass.
///
/// Returns an error if parameters are out of valid ranges.
pub fn neg_binom_dot_dist(
    failures: Value,
    successes: Value,
    probability: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_int_float_bool_to_float(
        failures,
        successes,
        probability,
        cumulative,
        strict_type_conversion,
        value_format,
        "NEGBINOM.DIST",
        codcel_neg_binom_dot_dist,
    )
}

/// Excel-compatible `NORM.DIST` function.
/// Evaluates the normal distribution.
/// - `x`: value at which to evaluate the distribution.
/// - `mean`: arithmetic mean of the distribution.
/// - `std_dev`: standard deviation of the distribution (must be > 0).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
///
/// Returns an error if `std_dev` <= 0.
pub fn norm_dot_dist(
    x: Value,
    mean: Value,
    std_dev: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_bool_to_float(
        x,
        mean,
        std_dev,
        cumulative,
        strict_type_conversion,
        value_format,
        "NORM.DIST",
        codcel_norm_dot_dist,
    )
}

/// Excel-compatible `NORM.INV` function.
/// Returns the inverse of the normal cumulative distribution.
/// - `probability`: probability associated with the normal distribution (between 0 and 1).
/// - `mean`: arithmetic mean of the distribution.
/// - `std_dev`: standard deviation of the distribution (must be > 0).
///
/// Returns the value `x` such that `NORM.DIST(x, mean, std_dev, TRUE) = probability`.
pub fn norm_dot_inv(
    probability: Value,
    mean: Value,
    std_dev: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![probability, mean, std_dev],
        strict_type_conversion,
        value_format,
        "NORM.INV",
        codcel_norm_dot_inv_vec,
    )
}

/// Excel-compatible `NORM.S.DIST` function.
/// Evaluates the standard normal distribution (mean = 0, standard deviation = 1).
/// - `z`: value at which to evaluate the distribution.
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
///
/// Returns the standard normal distribution value at `z`.
pub fn norm_dot_s_dot_dist(
    z: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_bool_to_float(
        z,
        cumulative,
        strict_type_conversion,
        value_format,
        "NORM.S.DIST",
        codcel_norm_dot_s_dot_dist,
    )
}

/// Excel-compatible `NORM.S.INV` function.
/// Returns the inverse of the standard normal cumulative distribution.
/// - `probability`: probability corresponding to the normal distribution (between 0 and 1).
///
/// Returns the value `z` such that `NORM.S.DIST(z, TRUE) = probability`.
pub fn norm_dot_s_dot_inv(
    probability: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![probability],
        strict_type_conversion,
        value_format,
        "NORM.S.INV",
        codcel_norm_dot_s_dot_inv_vec,
    )
}

/// Excel-compatible `POISSON`/`POISSON.DIST` function.
/// Evaluates the Poisson distribution.
/// - `x`: number of events (must be non-negative).
/// - `mean`: expected number of events (must be non-negative).
/// - `cumulative`: `true` for cumulative probability `P(X <= x)`, `false` for probability mass `P(X = x)`.
///
/// Returns an error on negative counts or negative mean.
pub fn poisson_dist(
    x: Value,
    mean: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_float_bool_to_float(
        x,
        mean,
        cumulative,
        strict_type_conversion,
        value_format,
        "POISSON.DIST",
        codcel_poisson_dist,
    )
}

/// Excel-compatible `WEIBULL.DIST` function.
/// Evaluates the Weibull distribution.
/// - `x`: value at which to evaluate the distribution (must be >= 0).
/// - `alpha`: shape parameter (must be > 0).
/// - `beta`: scale parameter (must be > 0).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
///
/// Returns an error if `x` < 0, `alpha` <= 0, or `beta` <= 0.
pub fn weibull_dist(
    x: Value,
    alpha: Value,
    beta: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_float_bool_to_float(
        x,
        alpha,
        beta,
        cumulative,
        strict_type_conversion,
        value_format,
        "WEIBULL.DIST",
        codcel_weibull_dist,
    )
}

// Correlation & Regression (14 functions)

/// Excel-compatible `CORREL` function.
/// Returns the correlation coefficient between two data sets.
/// - `x`: first data set (array of numeric values).
/// - `y`: second data set (array of numeric values, must have same length as `x`).
///
/// Returns a value between -1 and 1 indicating the linear correlation.
///
/// Returns an error if arrays have different lengths or are empty.
pub fn correl(
    x: Value,
    y: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = flatten_value_to_vec_f64(x, value_format)?;
    let y = flatten_value_to_vec_f64(y, value_format)?;

    Ok(Value::F64(codcel_correl(x, y)?))
}

/// Excel-compatible `COVARIANCE.P` function.
/// Returns the population covariance of two data sets.
/// - `x`: first data set (array of numeric values).
/// - `y`: second data set (array of numeric values, must have same length as `x`).
///
/// Returns the average of the products of deviations for each data point pair.
///
/// Returns an error if arrays have different lengths or are empty.
pub fn covariance_p(
    x: Value,
    y: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = flatten_value_to_vec_f64(x, value_format)?;
    let y = flatten_value_to_vec_f64(y, value_format)?;

    Ok(Value::F64(codcel_covariance_p(x, y)?))
}

/// Excel-compatible `COVARIANCE.S` function.
/// Returns the sample covariance of two data sets.
/// - `x`: first data set (array of numeric values).
/// - `y`: second data set (array of numeric values, must have same length as `x`).
///
/// Returns the sample covariance (divides by n-1 instead of n).
///
/// Returns an error if arrays have different lengths or fewer than 2 data points.
pub fn covariance_s(
    x: Value,
    y: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = flatten_value_to_vec_f64(x, value_format)?;
    let y = flatten_value_to_vec_f64(y, value_format)?;

    Ok(Value::F64(codcel_covariance_s(x, y)?))
}

/// Excel-compatible `FORECAST` function.
/// Calculates a predicted value using linear regression.
/// - `x`: the data point for which you want to predict a value.
/// - `known_ys`: dependent data set (array of known y-values).
/// - `known_xs`: independent data set (array of known x-values, must have same length as `known_ys`).
///
/// Returns the predicted y-value for the given x using the line of best fit.
///
/// Returns an error if arrays have different lengths or are empty.
pub fn forecast(
    x: Value,
    known_ys: Value,
    known_xs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;
    let known_ys = flatten_value_to_vec_f64(known_ys, value_format)?;
    let known_xs = flatten_value_to_vec_f64(known_xs, value_format)?;

    Ok(Value::F64(codcel_forecast(x, known_ys, known_xs)?))
}

/// Excel-compatible `FORECAST.LINEAR` function.
/// Calculates a predicted value using linear regression.
/// - `x`: the data point for which you want to predict a value.
/// - `known_ys`: dependent data set (array of known y-values).
/// - `known_xs`: independent data set (array of known x-values, must have same length as `known_ys`).
///
/// Returns the predicted y-value for the given x using the line of best fit.
///
/// This is functionally identical to `FORECAST`.
pub fn forecast_linear(
    x: Value,
    known_ys: Value,
    known_xs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // FORECAST.LINEAR is the newer version of FORECAST, it is exactly the same
    forecast(x, known_ys, known_xs, value_format)
}

/// Excel-compatible `FORECAST.ETS` function.
/// Predicts a future value using Exponential Triple Smoothing (Holt-Winters).
/// - `target_date`: numeric value for which to predict.
/// - `values`: historical data array.
/// - `timeline`: time periods array matching values.
/// - `seasonality`: 0=none, 1=auto (default), positive int=manual period.
/// - `data_completion`: 0=missing as zero, 1=interpolate (default).
/// - `aggregation`: 1=AVERAGE (default), 2=COUNT, 3=COUNTA, 4=MAX, 5=MEDIAN, 6=MIN, 7=SUM.
pub fn forecast_ets(
    target_date: Value,
    values: Value,
    timeline: Value,
    seasonality: Value,
    data_completion: Value,
    aggregation: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let target_date = target_date.f64(value_format)?;
    let values = flatten_value_to_vec_f64(values, value_format)?;
    let timeline = flatten_value_to_vec_f64(timeline, value_format)?;
    let seasonality = seasonality.option_i32(value_format)?;
    let data_completion = data_completion.option_i32(value_format)?;
    let aggregation = aggregation.option_i32(value_format)?;

    Ok(Value::F64(codcel_forecast_ets(
        target_date,
        values,
        timeline,
        seasonality,
        data_completion,
        aggregation,
    )?))
}

/// Excel-compatible `FORECAST.ETS.CONFINT` function.
/// Returns the half-width of the confidence interval for an ETS forecast.
/// - `target_date`: numeric value for which to predict.
/// - `values`: historical data array.
/// - `timeline`: time periods array matching values.
/// - `confidence_level`: confidence level (0-1 exclusive, default 0.95).
/// - `seasonality`: 0=none, 1=auto (default), positive int=manual period.
/// - `data_completion`: 0=missing as zero, 1=interpolate (default).
/// - `aggregation`: 1=AVERAGE (default), 2=COUNT, 3=COUNTA, 4=MAX, 5=MEDIAN, 6=MIN, 7=SUM.
#[allow(clippy::too_many_arguments)]
pub fn forecast_ets_confint(
    target_date: Value,
    values: Value,
    timeline: Value,
    confidence_level: Value,
    seasonality: Value,
    data_completion: Value,
    aggregation: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let target_date = target_date.f64(value_format)?;
    let values = flatten_value_to_vec_f64(values, value_format)?;
    let timeline = flatten_value_to_vec_f64(timeline, value_format)?;
    let confidence_level = confidence_level.option_f64(value_format)?;
    let seasonality = seasonality.option_i32(value_format)?;
    let data_completion = data_completion.option_i32(value_format)?;
    let aggregation = aggregation.option_i32(value_format)?;

    Ok(Value::F64(codcel_forecast_ets_confint(
        target_date,
        values,
        timeline,
        confidence_level,
        seasonality,
        data_completion,
        aggregation,
    )?))
}

/// Excel-compatible `FORECAST.ETS.SEASONALITY` function.
/// Returns the detected seasonal period length for the given time series.
/// - `values`: historical data array.
/// - `timeline`: time periods array matching values.
/// - `data_completion`: 0=missing as zero, 1=interpolate (default).
/// - `aggregation`: 1=AVERAGE (default), 2=COUNT, 3=COUNTA, 4=MAX, 5=MEDIAN, 6=MIN, 7=SUM.
pub fn forecast_ets_seasonality(
    values: Value,
    timeline: Value,
    data_completion: Value,
    aggregation: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = flatten_value_to_vec_f64(values, value_format)?;
    let timeline = flatten_value_to_vec_f64(timeline, value_format)?;
    let data_completion = data_completion.option_i32(value_format)?;
    let aggregation = aggregation.option_i32(value_format)?;

    Ok(Value::I32(codcel_forecast_ets_seasonality(
        values,
        timeline,
        data_completion,
        aggregation,
    )?))
}

/// Excel-compatible `FORECAST.ETS.STAT` function.
/// Returns a statistical value for the ETS model.
/// - `values`: historical data array.
/// - `timeline`: time periods array matching values.
/// - `stat_type`: 1=Alpha, 2=Beta, 3=Gamma, 4=MASE, 5=SMAPE, 6=MAE, 7=RMSE, 8=Step.
/// - `seasonality`: 0=none, 1=auto (default), positive int=manual period.
/// - `data_completion`: 0=missing as zero, 1=interpolate (default).
/// - `aggregation`: 1=AVERAGE (default), 2=COUNT, 3=COUNTA, 4=MAX, 5=MEDIAN, 6=MIN, 7=SUM.
pub fn forecast_ets_stat(
    values: Value,
    timeline: Value,
    stat_type: Value,
    seasonality: Value,
    data_completion: Value,
    aggregation: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = flatten_value_to_vec_f64(values, value_format)?;
    let timeline = flatten_value_to_vec_f64(timeline, value_format)?;
    let stat_type = stat_type.i32(value_format)?;
    let seasonality = seasonality.option_i32(value_format)?;
    let data_completion = data_completion.option_i32(value_format)?;
    let aggregation = aggregation.option_i32(value_format)?;

    Ok(Value::F64(codcel_forecast_ets_stat(
        values,
        timeline,
        stat_type,
        seasonality,
        data_completion,
        aggregation,
    )?))
}

/// Excel-compatible `GROWTH` function.
/// Calculates predicted exponential growth using existing data.
/// - `known_y`: dependent data set (array of known y-values, must be positive).
/// - `known_x`: optional independent data set (defaults to {1, 2, 3, ...}).
/// - `new_x`: optional new x-values for which to predict y-values.
/// - `const_b`: optional; if `false`, forces the curve through the origin (b = 1).
///
/// Returns an array of predicted y-values based on exponential regression `y = b * m^x`.
pub fn growth(
    known_y: Value,
    known_x: Value,
    new_x: Value,
    const_b: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let const_b = const_b.option_bool(value_format)?;
    let known_x = known_x.option_area_of_f64(strict_type_conversion, value_format)?;
    let new_x = new_x.option_area_of_f64(strict_type_conversion, value_format)?;

    let known_y_flattened = flatten_value_to_vec_f64(known_y, value_format)?;
    let known_x_flattened = known_x.map(|value| {
        value
            .iter()
            .flat_map(|row| row.iter().cloned())
            .collect::<Vec<f64>>()
    });
    let new_x_flattened = new_x.map(|value| {
        value
            .iter()
            .flat_map(|row| row.iter().cloned())
            .collect::<Vec<f64>>()
    });

    let result = codcel_growth(
        known_y_flattened,
        known_x_flattened,
        new_x_flattened,
        const_b,
    )?;
    let values = result
        .iter()
        .map(|val| Value::F64(*val))
        .collect::<Vec<Value>>();

    Ok(Value::VecValue(values))
}

/// Excel-compatible `INTERCEPT` function.
/// Returns the y-intercept of the linear regression line.
/// - `known_ys`: dependent data set (array of known y-values).
/// - `known_xs`: independent data set (array of known x-values, must have same length as `known_ys`).
///
/// Returns the point where the regression line crosses the y-axis.
///
/// Returns an error if arrays have different lengths or are empty.
pub fn intercept(
    known_ys: Value,
    known_xs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let known_ys = known_ys.to_flatterned_vec_f64(value_format)?;
    let known_xs = known_xs.to_flatterned_vec_f64(value_format)?;

    Ok(Value::F64(codcel_intercept(known_ys, known_xs)?))
}

/// Excel-compatible `LINEST` function.
/// Returns statistics describing a linear trend fitting the data using least squares.
/// - `known_ys`: dependent data set (array of known y-values).
/// - `known_xs`: optional independent data set (defaults to {1, 2, 3, ...}).
/// - `constant`: optional; if `false`, forces the line through the origin.
/// - `stats`: optional; if `true`, returns additional regression statistics.
///
/// Returns slope and intercept, or an extended array with additional statistics if `stats` is `true`.
pub fn linest(
    known_ys: Value,
    known_xs: Value,
    constant: Value,
    stats: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let known_ys = known_ys.to_flatterned_vec_f64(value_format)?;
    let known_xs = known_xs.option_area_of_value()?;
    let known_xs = if let Some(known_xs) = known_xs {
        // Convert to Vec<Vec<f64>> preserving 2D structure (rows x cols)
        let mut xs_2d: Vec<Vec<f64>> = Vec::new();
        for row in &known_xs {
            let mut r: Vec<f64> = Vec::new();
            for cell in row {
                r.push(cell.f64(value_format)?);
            }
            xs_2d.push(r);
        }
        Some(xs_2d)
    } else {
        None
    };
    let constant = constant.option_bool(value_format)?;
    let stats = stats.option_bool(value_format)?;

    let result = codcel_linest(known_ys, known_xs, constant, stats)?;

    Ok(area_f64(result))
}

/// Excel-compatible `LOGEST` function.
/// Returns statistics describing an exponential curve fitting the data.
/// - `known_ys`: dependent data set (array of known y-values, must be positive).
/// - `known_xs`: optional independent data set (defaults to {1, 2, 3, ...}).
/// - `constant`: optional; if `false`, forces the curve so that b = 1.
/// - `stats`: optional; if `true`, returns additional regression statistics.
///
/// Returns parameters for the curve `y = b * m^x`, or extended statistics if `stats` is `true`.
pub fn logest(
    known_ys: Value,
    known_xs: Value,
    constant: Value,
    stats: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let known_ys = known_ys.to_flatterned_vec_f64(value_format)?;
    let known_xs = known_xs.option_area_of_value()?;
    let known_xs = if let Some(known_xs) = known_xs {
        let values = Value::AreaValue(known_xs);
        Some(values.to_flatterned_vec_f64(value_format)?)
    } else {
        None
    };
    let constant = constant.option_bool(value_format)?;
    let stats = stats.option_bool(value_format)?;

    let result = codcel_logest(known_ys, known_xs, constant, stats)?;

    Ok(area_f64(result))
}

/// Excel-compatible `PEARSON` function.
/// Returns the Pearson product-moment correlation coefficient.
/// - `array1`: first data set (array of numeric values).
/// - `array2`: second data set (array of numeric values, must have same length as `array1`).
///
/// Returns a value between -1 and 1 indicating the linear correlation.
///
/// This is functionally equivalent to `CORREL`.
pub fn pearson(
    array1: Value,
    array2: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array1 = array1.to_flatterned_vec_f64(value_format)?;
    let array2 = array2.to_flatterned_vec_f64(value_format)?;
    Ok(Value::F64(codcel_pearson(array1, array2)?))
}

/// Excel-compatible `RSQ` function.
/// Returns the square of the Pearson product-moment correlation coefficient (R²).
/// - `known_ys`: dependent data set (array of known y-values).
/// - `known_xs`: independent data set (array of known x-values, must have same length as `known_ys`).
///
/// Returns a value between 0 and 1 representing the proportion of variance explained by the linear model.
///
/// Returns an error if arrays have different lengths or are empty.
pub fn rsq(
    known_ys: Value,
    known_xs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let known_ys = known_ys.to_flatterned_vec_f64(value_format)?;
    let known_xs = known_xs.to_flatterned_vec_f64(value_format)?;

    Ok(Value::F64(codcel_rsq(known_ys, known_xs)?))
}

/// Excel-compatible `SLOPE` function.
/// Returns the slope of the linear regression line.
/// - `known_ys`: dependent data set (array of known y-values).
/// - `known_xs`: independent data set (array of known x-values, must have same length as `known_ys`).
///
/// Returns the slope (m) of the line `y = mx + b` that best fits the data.
///
/// Returns an error if arrays have different lengths or are empty.
pub fn slope(
    known_ys: Value,
    known_xs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let known_ys = known_ys.to_flatterned_vec_f64(value_format)?;
    let known_xs = known_xs.to_flatterned_vec_f64(value_format)?;

    Ok(Value::F64(codcel_slope(known_ys, known_xs)?))
}

/// Excel-compatible `STEYX` function.
/// Returns the standard error of the predicted y-value for each x in a linear regression.
/// - `known_ys`: dependent data set (array of known y-values).
/// - `known_xs`: independent data set (array of known x-values, must have same length as `known_ys`).
///
/// Returns the standard error of the estimate, measuring the accuracy of predictions.
///
/// Returns an error if arrays have different lengths or have fewer than 3 data points.
pub fn steyx(
    known_ys: Value,
    known_xs: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let known_ys = known_ys.to_flatterned_vec_f64(value_format)?;
    let known_xs = known_xs.to_flatterned_vec_f64(value_format)?;

    Ok(Value::F64(codcel_steyx(known_ys, known_xs)?))
}

/// Excel-compatible `TREND` function.
/// Returns values along a linear trend using least squares regression.
/// - `known_y`: dependent data set (array of known y-values).
/// - `known_x`: optional independent data set (defaults to {1, 2, 3, ...}).
/// - `new_x`: optional new x-values for which to calculate trend values.
/// - `const_flag`: optional; if `false`, forces the line through the origin.
///
/// Returns an array of predicted y-values based on linear regression `y = mx + b`.
pub fn trend(
    known_y: Value,
    known_x: Value,
    new_x: Value,
    const_flag: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let known_y = known_y.to_flatterned_vec_f64(value_format)?;

    let known_x = known_x.option_area_of_f64(strict_type_conversion, value_format)?;
    let known_x_flattened = known_x.map(|value| {
        value
            .iter()
            .flat_map(|row| row.iter().cloned())
            .collect::<Vec<f64>>()
    });

    let new_x = new_x.option_area_of_f64(strict_type_conversion, value_format)?;
    let new_x_flattened = new_x.map(|value| {
        value
            .iter()
            .flat_map(|row| row.iter().cloned())
            .collect::<Vec<f64>>()
    });

    let const_flag = const_flag.option_bool(value_format)?;

    let values = codcel_trend(known_y, known_x_flattened, new_x_flattened, const_flag)?;
    let result = values.iter().map(|value| Value::F64(*value)).collect();
    Ok(Value::VecValue(result))
}

// Descriptive Statistics & Analysis (42 functions)

/// Excel-compatible `AVERAGE` function.
/// Returns the arithmetic mean of the provided values.
/// - `values`: one or more numeric values, arrays, or ranges.
///
/// Non-numeric values in arrays/ranges are ignored unless `strict_type_conversion` is enabled.
///
/// Returns an error if no numeric values are provided or if strict mode encounters non-numeric data.
pub fn average(
    values: Vec<Value>,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::F64(0.0));
    }

    let (sum, count) = values.iter().fold(
        (0.0, 0),
        |(acc_sum, acc_count), value| {
            if value.is_array() || value.is_area() {
                // Excel AVERAGE ignores non-numeric cells in ranges/arrays
                match value.area_of_value() {
                    Ok(area) => {
                        let mut local_sum = 0.0;
                        let mut local_count = 0;
                        for row in &area {
                            for cell in row {
                                if let Ok(val) = cell.f64(value_format) {
                                    local_sum += val;
                                    local_count += 1;
                                }
                            }
                        }
                        (acc_sum + local_sum, acc_count + local_count)
                    }
                    Err(_) => (acc_sum, acc_count),
                }
            } else if is_non_numeric_cell(value) {
                // The transpiler expands ranges into individual values.
                // Excel AVERAGE ignores text, booleans, and empty cells in ranges.
                (acc_sum, acc_count)
            } else {
                match value.f64(value_format) {
                    Ok(val) => (acc_sum + val, acc_count + 1),
                    Err(_) => (acc_sum, acc_count),
                }
            }
        },
    );

    if count == 0 {
        return Ok(Value::F64(0.0));
    }
    Ok(Value::F64(sum / count as f64))
}

/// Returns true for values that Excel range-aware functions (AVERAGE, AVEDEV, etc.)
/// should skip when they appear as scalar arguments from expanded ranges.
/// This includes text strings, empty strings, and boolean values.
fn is_non_numeric_cell(value: &Value) -> bool {
    matches!(
        value,
        Value::String(_)
            | Value::OptionString(_)
            | Value::Bool(_)
            | Value::OptionBool(_)
    )
}

/// Excel-compatible `AVERAGEA` function.
/// Returns the arithmetic mean of the provided values, including text and logical values.
/// - `values`: one or more values, arrays, or ranges.
///
/// Text values are counted as 0, `TRUE` as 1, `FALSE` as 0.
///
/// Returns 0 if no values are provided.
pub fn average_a(
    values: Vec<Value>,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::F64(0.0));
    }

    let (sum, count) = values.iter().fold((0.0, 0), |(acc_sum, acc_count), value| {
        if value.is_array() {
            match value {
                Value::VecValue(vec) => {
                    let (s, c) = average_a_fold_values(vec.iter(), value_format);
                    (acc_sum + s, acc_count + c)
                }
                Value::OptionVecValue(Some(vec)) => {
                    let (s, c) = average_a_fold_values(vec.iter(), value_format);
                    (acc_sum + s, acc_count + c)
                }
                _ => (acc_sum, acc_count),
            }
        } else if value.is_area() {
            match value {
                Value::AreaValue(area) => {
                    let (s, c) = average_a_fold_values(
                        area.iter().flat_map(|row| row.iter()),
                        value_format,
                    );
                    (acc_sum + s, acc_count + c)
                }
                Value::OptionAreaValue(Some(area)) => {
                    let (s, c) = average_a_fold_values(
                        area.iter().flat_map(|row| row.iter()),
                        value_format,
                    );
                    (acc_sum + s, acc_count + c)
                }
                _ => (acc_sum, acc_count),
            }
        } else {
            let (s, c) = average_a_value(value, value_format);
            (acc_sum + s, acc_count + c)
        }
    });

    if count == 0 {
        return Ok(Value::F64(0.0));
    }
    Ok(Value::F64(sum / count as f64))
}

/// Converts a single value using AVERAGEA semantics:
/// - Numbers: use as-is
/// - TRUE: 1, FALSE: 0
/// - Non-empty text: 0 (but still counted)
/// - Empty strings / empty cells: ignored
///   Returns (value, count) where count is 0 for empty/none values.
fn average_a_value(value: &Value, value_format: &ValueFormat) -> (f64, usize) {
    // Check for empty strings first — they represent empty cells and should be ignored
    match value {
        Value::String(s) if s.is_empty() => return (0.0, 0),
        Value::OptionString(Some(s)) if s.is_empty() => return (0.0, 0),
        Value::OptionString(None) => return (0.0, 0),
        _ => {}
    }
    match value.f64(value_format) {
        Ok(val) => (val, 1),
        Err(_) => {
            if value.is_string() {
                (0.0, 1) // Non-empty text counts as 0
            } else {
                (0.0, 0) // Skip none/unknown
            }
        }
    }
}

/// Folds an iterator of values using AVERAGEA semantics.
fn average_a_fold_values<'a>(
    values: impl Iterator<Item = &'a Value>,
    value_format: &ValueFormat,
) -> (f64, usize) {
    values.fold((0.0, 0), |(acc_sum, acc_count), val| {
        let (s, c) = average_a_value(val, value_format);
        (acc_sum + s, acc_count + c)
    })
}

/// Excel-compatible `MAXA` function.
/// Returns the largest value in a set of values, including text and logical values.
/// - `values`: one or more values, arrays, or ranges.
///
/// Text values are counted as 0, `TRUE` as 1, `FALSE` as 0.
///
/// Returns 0 if no values are provided.
pub fn maxa(
    values: Vec<Value>,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::F64(0.0));
    }

    let mut max_value = f64::MIN;

    for value in &values {
        if value.is_array() {
            if let Ok(vec) = value.vec_f64(value_format) {
                for &val in &vec {
                    max_value = max_value.max(val);
                }
            }
        } else if value.is_area() {
            if let Ok(area) = value.area_f64(value_format) {
                for row in &area {
                    for &val in row {
                        max_value = max_value.max(val);
                    }
                }
            }
        } else {
            // MAXA treats text as 0, booleans as 0/1
            if value.is_string() {
                max_value = max_value.max(0.0);
            } else if let Ok(val) = value.f64(value_format) {
                max_value = max_value.max(val)
            }
        }
    }

    Ok(Value::F64(max_value))
}

/// Excel-compatible `MINA` function.
///
/// Returns the smallest value in a set of values, including text and logical values.
///
/// Text values are counted as 0, `TRUE` as 1, `FALSE` as 0.
///
/// Returns 0 if no values are provided.
pub fn mina(
    values: Vec<Value>,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::F64(0.0));
    }

    let mut min_value = f64::MAX;

    for value in &values {
        if value.is_array() {
            if let Ok(vec) = value.vec_f64(value_format) {
                for &val in &vec {
                    min_value = min_value.min(val);
                }
            }
        } else if value.is_area() {
            if let Ok(area) = value.area_f64(value_format) {
                for row in &area {
                    for &val in row {
                        min_value = min_value.min(val);
                    }
                }
            }
        } else {
            // MINA treats text as 0, booleans as 0/1
            if value.is_string() {
                min_value = min_value.min(0.0);
            } else if let Ok(val) = value.f64(value_format) {
                min_value = min_value.min(val)
            }
        }
    }

    Ok(Value::F64(min_value))
}

/// Excel-compatible `AVEDEV` function.
/// Returns the average of the absolute deviations from the mean.
/// - `values`: one or more numeric values, arrays, or ranges.
///
/// Returns the average absolute deviation, a measure of variability.
///
/// Returns an error if no numeric values are provided or if strict mode encounters non-numeric data.
pub fn ave_dev(
    values: Vec<Value>,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Ok(Value::F64(0.0));
    }

    // First pass: compute sum and count, skipping non-numeric values
    let (sum, count) = values.iter().fold(
        (0.0, 0),
        |(acc_sum, acc_count), value| {
            if value.is_array() {
                match value.vec_f64(value_format) {
                    Ok(array_values) => {
                        let array_sum: f64 = array_values.iter().sum();
                        let array_count = array_values.len();
                        (acc_sum + array_sum, acc_count + array_count)
                    }
                    Err(_) => (acc_sum, acc_count),
                }
            } else if value.is_area() {
                match value.area_f64(value_format) {
                    Ok(area_values) => {
                        let area_sum: f64 = area_values.iter().flat_map(|row| row.iter()).sum();
                        let area_count: usize = area_values.iter().map(|row| row.len()).sum();
                        (acc_sum + area_sum, acc_count + area_count)
                    }
                    Err(_) => (acc_sum, acc_count),
                }
            } else if is_non_numeric_cell(value) {
                // Skip text, booleans, empty strings from expanded ranges
                (acc_sum, acc_count)
            } else {
                match value.f64(value_format) {
                    Ok(val) => (acc_sum + val, acc_count + 1),
                    Err(_) => (acc_sum, acc_count),
                }
            }
        },
    );

    if count == 0 {
        return Ok(Value::F64(0.0));
    }

    let mean = sum / count as f64;

    // Second pass: compute deviation sum, skipping non-numeric values
    let deviation_sum = values.iter().fold(0.0, |acc_dev_sum, value| {
        if value.is_array() {
            match value.vec_f64(value_format) {
                Ok(array_values) => {
                    acc_dev_sum + array_values.iter().map(|v| (v - mean).abs()).sum::<f64>()
                }
                Err(_) => acc_dev_sum,
            }
        } else if value.is_area() {
            match value.area_f64(value_format) {
                Ok(area_values) => {
                    acc_dev_sum
                        + area_values
                            .iter()
                            .flat_map(|row| row.iter())
                            .map(|v| (v - mean).abs())
                            .sum::<f64>()
                }
                Err(_) => acc_dev_sum,
            }
        } else if is_non_numeric_cell(value) {
            acc_dev_sum
        } else {
            match value.f64(value_format) {
                Ok(val) => acc_dev_sum + (val - mean).abs(),
                Err(_) => acc_dev_sum,
            }
        }
    });

    Ok(Value::F64(deviation_sum / count as f64))
}

/// Excel-compatible `CONFIDENCE.NORM` function.
/// Returns the confidence interval for a population mean using a normal distribution.
/// - `alpha`: significance level (between 0 and 1, e.g., 0.05 for 95% confidence).
/// - `standard_dev`: population standard deviation (must be > 0).
/// - `size`: sample size (must be >= 1).
///
/// Returns the margin of error for the confidence interval.
pub fn confidence_norm(
    alpha: Value,
    standard_dev: Value,
    size: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_int_to_float(
        alpha,
        standard_dev,
        size,
        strict_type_conversion,
        value_format,
        "CONFIDENCE.NORM",
        codcel_confidence_norm,
    )
}

/// Excel-compatible `CONFIDENCE.T` function.
/// Returns the confidence interval for a population mean using a Student's t-distribution.
/// - `alpha`: significance level (between 0 and 1, e.g., 0.05 for 95% confidence).
/// - `standard_dev`: sample standard deviation (must be > 0).
/// - `size`: sample size (must be >= 2).
///
/// Returns the margin of error for the confidence interval.
pub fn confidence_t(
    alpha: Value,
    standard_dev: Value,
    size: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_int_to_float(
        alpha,
        standard_dev,
        size,
        strict_type_conversion,
        value_format,
        "CONFIDENCE.T",
        codcel_confidence_t,
    )
}

/// Excel-compatible `CHISQ.TEST` function.
/// Returns the chi-squared test for independence.
/// - `actual_range`: observed data (2D array of frequencies).
/// - `expected_range`: expected data (2D array of expected frequencies, same dimensions).
///
/// Returns the p-value from the chi-squared distribution.
///
/// Returns an error if ranges have different dimensions or contain invalid values.
pub fn chisq_test(
    actual_range: Value,
    expected_range: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let actual_area = actual_range.area_of_value()?;
    let expected_area = expected_range.area_of_value()?;

    // Use original range dimensions for degrees of freedom
    let num_rows = actual_area.len();
    let num_cols = if num_rows > 0 { actual_area[0].len() } else { 0 };

    // Compute chi-squared statistic, skipping non-numeric cells
    let mut chi_squared_statistic = 0.0;
    for (obs_row, exp_row) in actual_area.iter().zip(expected_area.iter()) {
        for (obs_cell, exp_cell) in obs_row.iter().zip(exp_row.iter()) {
            let obs_val = match obs_cell.f64(value_format) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let exp_val = match exp_cell.f64(value_format) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if exp_val <= 0.0 {
                return Err("CHISQ.TEST: Expected values must be greater than 0.".into());
            }
            chi_squared_statistic += (obs_val - exp_val).powi(2) / exp_val;
        }
    }

    // Degrees of freedom based on original range dimensions
    let degrees_of_freedom = if num_rows > 1 && num_cols > 1 {
        (num_rows - 1) * (num_cols - 1)
    } else if num_rows == 1 && num_cols > 1 {
        num_cols - 1
    } else if num_rows > 1 && num_cols == 1 {
        num_rows - 1
    } else {
        return Err("CHISQ.TEST: Degrees of freedom must be positive.".into());
    };

    use statrs::distribution::ContinuousCDF;
    match statrs::distribution::ChiSquared::new(degrees_of_freedom as f64) {
        Ok(dist) => Ok(Value::F64(1.0 - dist.cdf(chi_squared_statistic))),
        Err(_) => Err("CHISQ.TEST: Error creating chi-squared distribution.".into()),
    }
}

/// Excel-compatible `F.TEST` function.
/// Returns the result of an F-test comparing variances of two data sets.
/// - `array1`: first data set (array of numeric values).
/// - `array2`: second data set (array of numeric values).
///
/// Returns the two-tailed probability that the variances are not significantly different.
///
/// Returns an error if either array has fewer than 2 data points.
pub fn f_dot_test(
    array1: Value,
    array2: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array1 = array1.to_flatterned_vec_f64(value_format)?;
    let array2 = array2.to_flatterned_vec_f64(value_format)?;

    Ok(Value::F64(codcel_f_dot_test(array1, array2)?))
}

/// Excel-compatible `FISHER` function.
/// Returns the Fisher transformation of a value.
/// - `x`: value for which to compute the transformation (must be between -1 and 1 exclusive).
///
/// Returns `0.5 * ln((1 + x) / (1 - x))`, useful for hypothesis testing on correlation coefficients.
///
/// Returns an error if `x` <= -1 or `x` >= 1.
pub fn fisher(
    x: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x],
        strict_type_conversion,
        value_format,
        "FISHER",
        codcel_fischer_vec,
    )
}

/// Excel-compatible `FISHERINV` function.
/// Returns the inverse of the Fisher transformation.
/// - `y`: value for which to compute the inverse transformation.
///
/// Returns `(e^(2*y) - 1) / (e^(2*y) + 1)`, the inverse of `FISHER`.
pub fn fisher_inv(
    y: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![y],
        strict_type_conversion,
        value_format,
        "FISHERINV",
        codcel_fischer_inv_vec,
    )
}

/// Excel-compatible `FREQUENCY` function.
/// Calculates how often values occur within a range of values.
/// - `data_array`: array of values for which to count frequencies.
/// - `bins_array`: array of intervals (bin boundaries) for grouping values.
///
/// Returns a vertical array of frequencies, with one more element than `bins_array`.
pub fn frequency(
    data_array: Value,
    bins_array: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let data_array = data_array.to_flatterned_vec_f64(value_format)?;
    let bins_array = bins_array.to_flatterned_vec_f64(value_format)?;

    let array = codcel_frequency(data_array, bins_array)?;

    let result: Vec<Value> = array.iter().map(|array| Value::I32(*array)).collect();

    Ok(Value::VecValue(result))
}

/// Excel-compatible `GAUSS` function.
/// Returns the probability that a standard normal random variable falls between 0 and z.
/// - `z`: value for which to compute the probability.
///
/// Returns `NORM.S.DIST(z, TRUE) - 0.5`, the area under the standard normal curve from 0 to z.
pub fn gauss(
    z: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![z],
        strict_type_conversion,
        value_format,
        "GAUSS",
        codcel_gauss_vec,
    )
}

/// Excel-compatible `GEOMEAN` function.
/// Returns the geometric mean of positive numeric values.
/// - `values`: one or more positive numeric values, arrays, or ranges.
///
/// Returns the nth root of the product of n values.
///
/// Returns an error if any value is <= 0.
pub fn geo_mean(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_geo_mean(values)?))
}

/// Excel-compatible `HARMEAN` function.
/// Returns the harmonic mean of positive numeric values.
/// - `values`: one or more positive numeric values, arrays, or ranges.
///
/// Returns the reciprocal of the arithmetic mean of the reciprocals.
///
/// Returns an error if any value is <= 0.
pub fn har_mean(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_har_mean(values)?))
}

/// Excel-compatible `KURT` function.
/// Returns the kurtosis of a data set.
/// - `values`: one or more numeric values, arrays, or ranges (requires at least 4 values).
///
/// Returns a measure of the "tailedness" of the distribution relative to a normal distribution.
///
/// Returns an error if fewer than 4 data points are provided or if standard deviation is zero.
pub fn kurt(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_kurt(values)?))
}

/// Excel-compatible `LARGE` function.
/// Returns the k-th largest value in a data set.
/// - `array`: array of numeric values.
/// - `k`: position from the largest (1 returns the largest, 2 returns the second largest, etc.).
///
/// Returns an error if `k` is less than 1 or greater than the number of data points.
pub fn large(
    array: Value,
    k: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = array.to_flatterned_vec_f64(value_format)?;
    let k = k.i32(value_format)?;

    Ok(Value::F64(codcel_large(array, k)?))
}

/// Excel-compatible `MEDIAN` function.
/// Returns the median (middle value) of a data set.
/// - `values`: one or more numeric values, arrays, or ranges.
///
/// Returns the middle value when sorted; if even count, returns average of two middle values.
///
/// Returns an error if no numeric values are provided.
pub fn median(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_median(values)?))
}

/// Excel-compatible `MODE.SNGL` function.
/// Returns the most frequently occurring value in a data set.
/// - `values`: one or more numeric values, arrays, or ranges.
///
/// Returns the mode; if multiple modes exist, returns the first one encountered.
///
/// Returns an error if no value appears more than once.
pub fn mode_sngl(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_mode_sngl(values)?))
}

/// Excel-compatible `MODE.MULT` function.
/// Returns a vertical array of the most frequently occurring values in a data set.
/// - `values`: one or more numeric values, arrays, or ranges.
///
/// Returns all modes if multiple values have the same highest frequency.
///
/// Returns an error if no value appears more than once.
pub fn mode_mult(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    let result = vec_f64(codcel_mode_mult(values)?);
    Ok(result)
}

/// Excel-compatible `PERCENTRANK.EXC` function.
/// Returns the rank of a value as a percentage, excluding 0 and 1.
/// - `array`: array of numeric values defining the data set.
/// - `x`: value for which to find the percentile rank.
/// - `significance`: optional number of significant digits (defaults to 3).
///
/// Returns a value strictly between 0 and 1.
pub fn percent_rank_exc(
    array: Value,
    x: Value,
    significance: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = array.to_flatterned_vec_f64(value_format)?;
    let x = x.f64(value_format)?;
    let significance = significance.option_i32(value_format)?;
    Ok(Value::F64(codcel_percent_rank_exc(array, x, significance)?))
}

/// Excel-compatible `PERCENTRANK.INC` function.
/// Returns the rank of a value as a percentage, including 0 and 1.
/// - `array`: array of numeric values defining the data set.
/// - `x`: value for which to find the percentile rank.
/// - `significance`: optional number of significant digits (defaults to 3).
///
/// Returns a value between 0 and 1 (inclusive).
pub fn percent_rank_inc(
    array: Value,
    x: Value,
    significance: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = array.to_flatterned_vec_f64(value_format)?;
    let x = x.f64(value_format)?;
    let significance = significance.option_i32(value_format)?;
    Ok(Value::F64(codcel_percent_rank_inc(array, x, significance)?))
}

/// Excel-compatible `PERCENTILE.EXC` function.
/// Returns the k-th percentile of values, excluding the 0th and 100th percentiles.
/// - `array`: array of numeric values defining the data set.
/// - `k`: percentile value (strictly between 0 and 1).
///
/// Returns an error if `k` <= 0 or `k` >= 1, or if array is empty.
pub fn percentile_exc(
    array: Value,
    k: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = array.to_flatterned_vec_f64(value_format)?;
    let k = k.f64(value_format)?;
    Ok(Value::F64(codcel_percentile_exc(array, k)?))
}

/// Excel-compatible `PERCENTILE.INC` function.
/// Returns the k-th percentile of values, including the 0th and 100th percentiles.
/// - `array`: array of numeric values defining the data set.
/// - `k`: percentile value (between 0 and 1 inclusive).
///
/// Returns an error if `k` < 0 or `k` > 1, or if array is empty.
pub fn percentile_inc(
    array: Value,
    k: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = array.to_flatterned_vec_f64(value_format)?;
    let k = k.f64(value_format)?;
    Ok(Value::F64(codcel_percentile_inc(array, k)?))
}

/// Excel-compatible `PHI` function.
/// Returns the value of the standard normal distribution density function.
/// - `x`: value at which to evaluate the density function.
///
/// Returns `(1/√(2π)) * e^(-x²/2)`, the probability density at `x`.
pub fn phi(
    x: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x],
        strict_type_conversion,
        value_format,
        "PHI",
        codcel_phi_vec,
    )
}

/// Excel-compatible `PROB` function.
/// Returns the probability that values fall within specified limits.
/// - `values`: array of numeric values.
/// - `probabilities`: array of corresponding probabilities (must sum to 1).
/// - `lower_limit`: lower bound of the range.
/// - `upper_limit`: optional upper bound (defaults to `lower_limit` for exact match).
///
/// Returns the sum of probabilities for values between the limits.
pub fn prob(
    values: Value,
    probabilities: Value,
    lower_limit: Value,
    upper_limit: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = values.to_flatterned_vec_f64(value_format)?;
    let probabilities = probabilities.to_flatterned_vec_f64(value_format)?;
    let lower_limit = lower_limit.f64(value_format)?;
    let upper_limit = upper_limit.option_f64(value_format)?;
    Ok(Value::F64(codcel_prob(
        values,
        probabilities,
        lower_limit,
        upper_limit,
    )?))
}

/// Excel-compatible `QUARTILE.EXC` function.
/// Returns the quartile of a data set, excluding the minimum and maximum.
/// - `values`: array of numeric values.
/// - `quart`: quartile to return (1 = 25th percentile, 2 = median, 3 = 75th percentile).
///
/// Returns an error if `quart` is not 1, 2, or 3.
pub fn quartile_exc(
    values: Value,
    quart: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = values.to_flatterned_vec_f64(value_format)?;
    let quart = quart.i32(value_format)?;
    Ok(Value::F64(codcel_quartile_exc(values, quart)?))
}

/// Excel-compatible `QUARTILE.INC` function.
/// Returns the quartile of a data set, including the minimum and maximum.
/// - `values`: array of numeric values.
/// - `quart`: quartile to return (0 = min, 1 = 25th percentile, 2 = median, 3 = 75th percentile, 4 = max).
///
/// Returns an error if `quart` is not 0, 1, 2, 3, or 4.
pub fn quartile_inc(
    values: Value,
    quart: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = values.to_flatterned_vec_f64(value_format)?;
    let quart = quart.i32(value_format)?;
    Ok(Value::F64(codcel_quartile_inc(values, quart)?))
}

/// Excel-compatible `RANK.AVG` function.
/// Returns the rank of a value in a data set, averaging ranks for duplicate values.
/// - `value`: the value whose rank you want to find.
/// - `values`: array of numeric values defining the data set.
/// - `order`: optional; if `true` or non-zero, ranks in ascending order (default is descending).
///
/// Returns the average rank when there are duplicate values.
pub fn rank_avg(
    value: Value,
    values: Value,
    order: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    let values = values.to_flatterned_vec_f64(value_format)?;
    let order = order.option_bool(value_format)?;

    Ok(Value::F64(codcel_rank_avg(value, values, order)?))
}

/// Excel-compatible `RANK.EQ` function.
/// Returns the rank of a value in a data set, giving duplicate values the same rank.
/// - `value`: the value whose rank you want to find.
/// - `values`: array of numeric values defining the data set.
/// - `order`: optional; if `true` or non-zero, ranks in ascending order (default is descending).
///
/// Returns the rank as an integer; duplicates receive the same rank.
pub fn rank_eq(
    value: Value,
    values: Value,
    order: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    let values = values.to_flatterned_vec_f64(value_format)?;
    let order = order.option_bool(value_format)?;

    Ok(Value::I32(codcel_rank_eq(value, values, order)?))
}

/// Excel-compatible `SKEW` function.
/// Returns the skewness of a distribution based on a sample.
/// - `values`: one or more numeric values, arrays, or ranges (requires at least 3 values).
///
/// Returns a measure of the asymmetry of the distribution around its mean.
///
/// Positive skew indicates a longer right tail; negative skew indicates a longer left tail.
pub fn skew(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_skew(values)?))
}

/// Excel-compatible `SKEW.P` function.
/// Returns the skewness of a distribution based on a population.
/// - `values`: one or more numeric values, arrays, or ranges (requires at least 3 values).
///
/// Returns a measure of the asymmetry of the distribution (population-based formula).
///
/// Uses n instead of n-1 in the denominator compared to `SKEW`.
pub fn skew_p(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_skew_p(values)?))
}

/// Excel-compatible `SMALL` function.
/// Returns the k-th smallest value in a data set.
/// - `array`: array of numeric values.
/// - `k`: position from the smallest (1 returns the smallest, 2 returns the second smallest, etc.).
///
/// Returns an error if `k` is less than 1 or greater than the number of data points.
pub fn small(
    array: Value,
    k: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = array.to_flatterned_vec_f64(value_format)?;
    let k = k.i32(value_format)?;

    Ok(Value::F64(codcel_small(array, k)?))
}

/// Excel-compatible `STANDARDIZE` function.
/// Returns a normalized value (z-score) from a distribution.
/// - `x`: value to normalize.
/// - `mean`: arithmetic mean of the distribution.
/// - `standard_dev`: standard deviation of the distribution (must be > 0).
///
/// Returns `(x - mean) / standard_dev`.
pub fn standardize(
    x: Value,
    mean: Value,
    standard_dev: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x, mean, standard_dev],
        strict_type_conversion,
        value_format,
        "STANDARDIZE",
        codcel_standardize_vec,
    )
}

/// Excel-compatible `STDEV.P` function.
/// Calculates the standard deviation based on the entire population.
/// - `values`: one or more numeric values, arrays, or ranges.
///
/// Returns the population standard deviation (divides by n).
///
/// Returns an error if no numeric values are provided.
pub fn st_dev_dot_p(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_st_dev_dot_p(values)?))
}

/// Excel-compatible `STDEV.S` function.
/// Estimates the standard deviation based on a sample.
/// - `values`: one or more numeric values, arrays, or ranges (requires at least 2 values).
///
/// Returns the sample standard deviation (divides by n-1).
///
/// Returns an error if fewer than 2 numeric values are provided.
pub fn st_dev_s(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_st_dev_s(values)?))
}

/// Excel-compatible `STDEVA` function.
/// Estimates the standard deviation based on a sample, including text and logical values.
/// - `values`: one or more values, arrays, or ranges (requires at least 2 values).
///
/// Text values are treated as 0, `TRUE` as 1, `FALSE` as 0.
///
/// Returns the sample standard deviation (divides by n-1).
///
/// Returns an error if fewer than 2 values are provided.
pub fn stdeva(
    values: Vec<Value>,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("STDEVA: At least two values are required to calculate standard deviation.".into());
    }

    let mut collected_values: Vec<f64> = Vec::new();

    for value in &values {
        if value.is_array() {
            if let Ok(vec) = value.vec_f64(value_format) {
                collected_values.extend(vec);
            }
        } else if value.is_area() {
            if let Ok(area) = value.area_f64(value_format) {
                for row in &area {
                    collected_values.extend(row);
                }
            }
        } else {
            match value.f64(value_format) {
                Ok(val) => collected_values.push(val),
                Err(_) => collected_values.push(0.0),
            }
        }
    }

    Ok(Value::F64(codcel_stdeva(collected_values)?))
}

/// Excel-compatible `STDEVPA` function.
/// Calculates the population standard deviation, including text and logical values.
/// - `values`: one or more values, arrays, or ranges (requires at least 1 value).
///
/// Text values are treated as 0, `TRUE` as 1, `FALSE` as 0.
///
/// Returns the population standard deviation (divides by n).
///
/// Returns an error if no values are provided.
pub fn stdevpa(
    values: Vec<Value>,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("STDEVPA: At least one value is required to calculate standard deviation.".into());
    }

    let mut collected_values: Vec<f64> = Vec::new();

    for value in &values {
        if value.is_array() {
            if let Ok(vec) = value.vec_f64(value_format) {
                collected_values.extend(vec);
            }
        } else if value.is_area() {
            if let Ok(area) = value.area_f64(value_format) {
                for row in &area {
                    collected_values.extend(row);
                }
            }
        } else {
            match value.f64(value_format) {
                Ok(val) => collected_values.push(val),
                Err(_) => collected_values.push(0.0),
            }
        }
    }

    Ok(Value::F64(codcel_stdevpa(collected_values)?))
}

/// Excel-compatible `T.DIST` function.
/// Evaluates the Student's t-distribution.
/// - `x`: value at which to evaluate the distribution.
/// - `degrees_freedom`: degrees of freedom (must be >= 1).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
///
/// Returns an error if `degrees_freedom` < 1.
pub fn t_dot_dist(
    x: Value,
    degrees_freedom: Value,
    cumulative: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_bool_to_float(
        x,
        degrees_freedom,
        cumulative,
        strict_type_conversion,
        value_format,
        "T.DIST",
        codcel_t_dot_dist,
    )
}

/// Excel-compatible `T.DIST.RT` function.
/// Returns the right-tailed Student's t-distribution.
/// - `x`: value at which to evaluate the distribution.
/// - `degrees_freedom`: degrees of freedom (must be >= 1).
///
/// Returns `P(X > x)` where X follows a t-distribution.
pub fn t_dist_rt(
    x: Value,
    degrees_freedom: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x, degrees_freedom],
        strict_type_conversion,
        value_format,
        "T.DIST.RT",
        codcel_t_dist_rt_vec,
    )
}

/// Excel-compatible `T.INV` function.
/// Returns the left-tailed inverse of the Student's t-distribution.
/// - `probability`: probability associated with the t-distribution (between 0 and 1).
/// - `degrees_freedom`: degrees of freedom (must be >= 1).
///
/// Returns the value `x` such that `T.DIST(x, degrees_freedom, TRUE) = probability`.
pub fn t_dot_inv(
    probability: Value,
    degrees_freedom: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![probability, degrees_freedom],
        strict_type_conversion,
        value_format,
        "T.INV",
        codcel_t_dot_inv_vec,
    )
}

/// Excel-compatible `T.TEST` function.
/// Returns the probability associated with a Student's t-test.
/// - `array1`: first data set (array of numeric values).
/// - `array2`: second data set (array of numeric values).
/// - `tails`: number of distribution tails (1 for one-tailed, 2 for two-tailed).
/// - `type_value`: type of t-test (1 = paired, 2 = two-sample equal variance, 3 = two-sample unequal variance).
///
/// Returns the p-value associated with the t-test.
pub fn t_dot_test(
    array1: Value,
    array2: Value,
    tails: Value,
    type_value: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array1 = array1.to_flatterned_vec_f64(value_format)?;
    let array2 = array2.to_flatterned_vec_f64(value_format)?;
    let tails = tails.i32(value_format)?;
    let type_value = type_value.i32(value_format)?;

    Ok(Value::F64(codcel_t_dot_test(
        array1, array2, tails, type_value,
    )?))
}

/// Excel-compatible `TRIMMEAN` function.
/// Returns the mean of a data set after excluding a percentage of outliers.
/// - `data`: array of numeric values.
/// - `percent`: fraction of data points to exclude (between 0 and 1, e.g., 0.2 excludes 10% from each end).
///
/// Returns the mean of the interior data points after trimming.
///
/// Returns an error if `percent` < 0 or `percent` >= 1.
pub fn trim_mean(
    data: Value,
    percent: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let data = data.to_flatterned_vec_f64(value_format)?;
    let percent = percent.f64(value_format)?;

    Ok(Value::F64(codcel_trim_mean(data, percent)?))
}

/// Excel-compatible `VAR.P` function.
/// Calculates the variance based on the entire population.
/// - `values`: one or more numeric values, arrays, or ranges.
///
/// Returns the population variance (divides by n).
///
/// Returns an error if no numeric values are provided.
pub fn var_dot_p(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_var_dot_p(values)?))
}

/// Excel-compatible `VARP` function (legacy).
/// Calculates the variance based on the entire population.
/// - `values`: one or more numeric values, arrays, or ranges.
///
/// Returns the population variance (divides by n).
///
/// This is functionally equivalent to `VAR.P`.
pub fn var_p(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_var_p(values)?))
}

/// Excel-compatible `VAR.S` function.
/// Estimates the variance based on a sample.
/// - `values`: one or more numeric values, arrays, or ranges (requires at least 2 values).
///
/// Returns the sample variance (divides by n-1).
///
/// Returns an error if fewer than 2 numeric values are provided.
pub fn var_s(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_var_s(values)?))
}

/// Excel-compatible `VARA` function.
/// Calculates the sample variance, including text and logical values.
/// - `values`: one or more values, arrays, or ranges (requires at least 2 values).
///
/// Text values are treated as 0, `TRUE` as 1, `FALSE` as 0.
///
/// Returns the sample variance (divides by n-1).
///
/// Returns an error if fewer than 2 values are provided.
pub fn vara(
    values: Vec<Value>,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("VARA: At least two values are required to calculate variance.".into());
    }

    let mut collected_values: Vec<f64> = Vec::new();

    for value in &values {
        if value.is_array() {
            if let Ok(vec) = value.vec_f64(value_format) {
                collected_values.extend(vec);
            }
        } else if value.is_area() {
            if let Ok(area) = value.area_f64(value_format) {
                for row in &area {
                    collected_values.extend(row);
                }
            }
        } else {
            match value.f64(value_format) {
                Ok(val) => collected_values.push(val),
                Err(_) => collected_values.push(0.0),
            }
        }
    }

    Ok(Value::F64(codcel_vara(collected_values)?))
}

/// Excel-compatible `VARPA` function.
/// Returns the population variance, including text and logical values in the calculation.
/// Text values are treated as 0, TRUE as 1, FALSE as 0.
///
/// Returns an error if no values are provided.
pub fn varpa(
    values: Vec<Value>,
    _strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    if values.is_empty() {
        return Err("VARPA: At least one value is required to calculate variance.".into());
    }

    let mut collected_values: Vec<f64> = Vec::new();

    for value in &values {
        if value.is_array() {
            if let Ok(vec) = value.vec_f64(value_format) {
                collected_values.extend(vec);
            }
        } else if value.is_area() {
            if let Ok(area) = value.area_f64(value_format) {
                for row in &area {
                    collected_values.extend(row);
                }
            }
        } else {
            match value.f64(value_format) {
                Ok(val) => collected_values.push(val),
                Err(_) => collected_values.push(0.0),
            }
        }
    }

    Ok(Value::F64(codcel_varpa(collected_values)?))
}

/// Excel-compatible `Z.TEST` function.
/// Returns the one-tailed p-value of a z-test.
/// - `data`: array of sample data.
/// - `hyp_mean`: hypothesized population mean to test against.
/// - `sigma`: optional known population standard deviation. If omitted, uses sample standard deviation.
///
/// Returns the probability that the sample mean is greater than the observed mean if the population mean equals `hyp_mean`.
pub fn z_dot_test(
    data: Value,
    hyp_mean: Value,
    sigma: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let data = data.to_flatterned_vec_f64(value_format)?;
    let hyp_mean = hyp_mean.f64(value_format)?;
    let sigma = sigma.option_f64(value_format)?;

    Ok(Value::F64(codcel_z_dot_test(data, hyp_mean, sigma)?))
}
