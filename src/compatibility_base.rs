// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::area::{
    process_area_float_float_bool_to_float, process_area_float_float_float_bool_to_float,
    process_area_float_float_float_opt_float_opt_float_to_float,
    process_area_float_float_int_to_float, process_area_float_multi_to_float,
    process_area_int_float_bool_to_float, process_area_int_float_float_to_int,
    process_area_int_int_float_bool_to_float, process_area_int_int_float_to_float,
    process_area_int_multi_to_float,
};
use crate::compatibility::codcel_beta_inv::codcel_beta_inv;
use crate::compatibility::codcel_betadist::codcel_betadist;
use crate::compatibility::codcel_binom_dist::codcel_binom_dist;
use crate::compatibility::codcel_chi_dist::codcel_chi_dist_vec;
use crate::compatibility::codcel_chi_inv::codcel_chi_inv_vec;
use crate::compatibility::codcel_chi_test::codcel_chi_test;
use crate::compatibility::codcel_co_var::codcel_co_var;
use crate::compatibility::codcel_confidence::codcel_confidence;
use crate::compatibility::codcel_crit_binom::codcel_crit_binom;
use crate::compatibility::codcel_expon_dist::codcel_expon_dist;
use crate::compatibility::codcel_f_dist::codcel_f_dist_vec;
use crate::compatibility::codcel_f_inv::codcel_f_inv_vec;
use crate::compatibility::codcel_f_test::codcel_f_test;
use crate::compatibility::codcel_gamma_dist::codcel_gamma_dist;
use crate::compatibility::codcel_gamma_inv::codcel_gamma_inv_vec;
use crate::compatibility::codcel_hypgeom_dist::codcel_hypgeom_dist_vec;
use crate::compatibility::codcel_log_inv::codcel_log_inv_vec;
use crate::compatibility::codcel_log_norm_dist::codcel_log_norm_dist_vec;
use crate::compatibility::codcel_mode::codcel_mode;
use crate::compatibility::codcel_neg_binom_dist::codcel_neg_binom_dist;
use crate::compatibility::codcel_norm_dist::codcel_norm_dist;
use crate::compatibility::codcel_norm_inv::codcel_norm_inv_vec;
use crate::compatibility::codcel_norm_s_dist::codcel_norm_s_dist_vec;
use crate::compatibility::codcel_norm_s_inv::codcel_norm_s_inv_vec;
use crate::compatibility::codcel_percent_rank::codcel_percent_rank;
use crate::compatibility::codcel_percentile::codcel_percentile;
use crate::compatibility::codcel_poisson::codcel_poisson;
use crate::compatibility::codcel_quartile::codcel_quartile;
use crate::compatibility::codcel_rank::codcel_rank;
use crate::compatibility::codcel_st_dev::codcel_st_dev;
use crate::compatibility::codcel_st_dev_p::codcel_st_dev_p;
use crate::compatibility::codcel_t_dist::codcel_t_dist;
use crate::compatibility::codcel_t_inv::codcel_t_inv_vec;
use crate::compatibility::codcel_t_test::codcel_t_test;
use crate::compatibility::codcel_var::codcel_var;
use crate::compatibility::codcel_weibull::codcel_weibull;
use crate::compatibility::codcel_z_test::codcel_z_test;
use crate::value::{vec_value_to_vec_f64, Value};
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `BETADIST` function.
/// Returns the cumulative beta probability density function.
/// - `x`: the value at which to evaluate the function (between `a` and `b`).
/// - `alpha`: shape parameter of the distribution (must be > 0).
/// - `beta`: shape parameter of the distribution (must be > 0).
/// - `a`: optional lower bound of the interval (defaults to 0).
/// - `b`: optional upper bound of the interval (defaults to 1).
/// - `value_format`: format settings for value conversion.
///
/// Returns an error if `alpha` or `beta` <= 0, or if `x` is outside [a, b].
pub fn betadist(
    x: Value,
    alpha: Value,
    beta: Value,
    a: Value,
    b: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;
    let alpha = alpha.f64(value_format)?;
    let beta = beta.f64(value_format)?;

    let a = a.option_f64(value_format)?;
    let b = b.option_f64(value_format)?;

    Ok(Value::F64(codcel_betadist(x, alpha, beta, a, b)?))
}

/// Excel-compatible `BETAINV` function.
/// Returns the inverse of the cumulative beta probability density function.
/// - `probability`: probability associated with the beta distribution (0 to 1).
/// - `alpha`: shape parameter of the distribution (must be > 0).
/// - `beta`: shape parameter of the distribution (must be > 0).
/// - `a`: optional lower bound of the interval (defaults to 0).
/// - `b`: optional upper bound of the interval (defaults to 1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the value `x` such that `BETADIST(x, alpha, beta, a, b) = probability`.
pub fn beta_inv(
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
        "BETAINV",
        codcel_beta_inv,
    )
}

/// Excel-compatible `BINOMDIST` function.
/// Returns the individual term binomial distribution probability.
/// - `number_s`: number of successes in trials (must be >= 0 and <= trials).
/// - `trials`: number of independent trials (must be >= 0).
/// - `probability_s`: probability of success on each trial (0 to 1).
/// - `cumulative`: `true` for cumulative probability `P(X <= number_s)`, `false` for exact probability `P(X = number_s)`.
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns an error if parameters are out of valid ranges.
pub fn binom_dist(
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
        "BINOMDIST",
        codcel_binom_dist,
    )
}

/// Excel-compatible `CHIDIST` function.
/// Returns the right-tailed probability of the chi-squared distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `degrees`: degrees of freedom (must be a positive integer, 1 to 10^10).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns `P(X > x)` where X follows a chi-squared distribution.
pub fn chi_dist(
    x: Value,
    degrees: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x, degrees],
        strict_type_conversion,
        value_format,
        "CHIDIST",
        codcel_chi_dist_vec,
    )
}

/// Excel-compatible `CHIINV` function.
/// Returns the inverse of the right-tailed probability of the chi-squared distribution.
/// - `probability`: the right-tailed probability (0 to 1).
/// - `degrees_freedom`: degrees of freedom (must be a positive integer, 1 to 10^10).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the value `x` such that `CHIDIST(x, degrees_freedom) = probability`.
pub fn chi_inv(
    probability: Value,
    degrees_freedom: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![probability, degrees_freedom],
        strict_type_conversion,
        value_format,
        "CHIINV",
        codcel_chi_inv_vec,
    )
}

/// Excel-compatible `CHITEST` function.
/// Returns the chi-squared test for independence.
/// - `actual_range`: the range of observed (actual) values.
/// - `expected_range`: the range of expected values.
/// - `value_format`: format settings for value conversion.
///
/// Returns the p-value from the chi-squared distribution for the test statistic.
///
/// Both ranges must have the same dimensions and contain numeric values.
pub fn chi_test(
    actual_range: Value,
    expected_range: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    // Extract 2D dimensions before flattening for degrees of freedom calculation
    let (rows, cols) = match &actual_range {
        Value::AreaValue(area) => {
            let r = area.len();
            let c = if r > 0 { area[0].len() } else { 0 };
            (r, c)
        }
        _ => (1, 1),
    };
    let actual_range = actual_range.to_flatterned_vec_f64(value_format)?;
    let expected_range = expected_range.to_flatterned_vec_f64(value_format)?;
    // For 1D (single row) data, use total elements as cols
    let (rows, cols) = if rows == 1 && cols <= 1 {
        (1, actual_range.len())
    } else {
        (rows, cols)
    };

    Ok(Value::F64(codcel_chi_test(actual_range, expected_range, rows, cols)?))
}

/// Excel-compatible `COVAR` function.
/// Returns the population covariance of two data sets.
/// - `array1`: the first range of numeric values.
/// - `array2`: the second range of numeric values.
/// - `value_format`: format settings for value conversion.
///
/// Returns the average of the products of deviations for each data point pair.
///
/// Both arrays must have the same number of data points.
pub fn co_var(
    array1: Value,
    array2: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array1 = array1.to_flatterned_vec_f64(value_format)?;
    let array2 = array2.to_flatterned_vec_f64(value_format)?;
    Ok(Value::F64(codcel_co_var(array1, array2)?))
}

/// Excel-compatible `CONFIDENCE` function.
/// Returns the confidence interval for a population mean using a normal distribution.
/// - `alpha`: the significance level (0 to 1); e.g., 0.05 for 95% confidence.
/// - `standard_dev`: the population standard deviation (must be > 0).
/// - `size`: the sample size (must be >= 1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the margin of error: `NORM.S.INV(1 - alpha/2) * standard_dev / SQRT(size)`.
pub fn confidence(
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
        "CONFIDENCE",
        codcel_confidence,
    )
}

/// Excel-compatible `CRITBINOM` function.
/// Returns the smallest value for which the cumulative binomial distribution is >= criterion.
/// - `trials`: the number of Bernoulli trials (must be >= 0).
/// - `probability`: the probability of success on each trial (0 to 1).
/// - `alpha`: the criterion value (0 to 1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the smallest integer `k` such that `BINOMDIST(k, trials, probability, TRUE) >= alpha`.
pub fn crit_binom(
    trials: Value,
    probability: Value,
    alpha: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_float_float_to_int(
        trials,
        probability,
        alpha,
        strict_type_conversion,
        value_format,
        "CRITBINOM",
        codcel_crit_binom,
    )
}

/// Excel-compatible `EXPONDIST` function.
/// Returns the exponential distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `lambda`: the rate parameter (must be > 0).
/// - `cumulative`: `true` for cumulative distribution function `1 - exp(-lambda * x)`, `false` for probability density function `lambda * exp(-lambda * x)`.
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns an error if `x` < 0 or `lambda` <= 0.
pub fn expon_dist(
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
        "EXPONDIST",
        codcel_expon_dist,
    )
}

/// Excel-compatible `FDIST` function.
/// Returns the right-tailed F probability distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `d1`: the numerator degrees of freedom (must be >= 1).
/// - `d2`: the denominator degrees of freedom (must be >= 1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns `P(X > x)` where X follows an F-distribution.
pub fn f_dist(
    x: Value,
    d1: Value,
    d2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x, d1, d2],
        strict_type_conversion,
        value_format,
        "FDIST",
        codcel_f_dist_vec,
    )
}

/// Excel-compatible `FINV` function.
/// Returns the inverse of the right-tailed F probability distribution.
/// - `p`: the right-tailed probability (0 to 1, exclusive).
/// - `d1`: the numerator degrees of freedom (must be >= 1).
/// - `d2`: the denominator degrees of freedom (must be >= 1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the value `x` such that `FDIST(x, d1, d2) = p`.
pub fn f_inv(
    p: Value,
    d1: Value,
    d2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![p, d1, d2],
        strict_type_conversion,
        value_format,
        "FINV",
        codcel_f_inv_vec,
    )
}

/// Excel-compatible `FTEST` function.
/// Returns the result of an F-test (two-tailed probability that variances are not significantly different).
/// - `array1`: the first range of numeric values.
/// - `array2`: the second range of numeric values.
/// - `value_format`: format settings for value conversion.
///
/// Returns the two-tailed probability that the variances in the two arrays are not significantly different.
pub fn f_test(
    array1: Value,
    array2: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array1 = array1.to_flatterned_vec_f64(value_format)?;
    let array2 = array2.to_flatterned_vec_f64(value_format)?;

    Ok(Value::F64(codcel_f_test(array1, array2)?))
}

/// Excel-compatible `GAMMADIST` function.
/// Returns the gamma distribution.
/// - `x`: the value at which to evaluate the distribution (must be >= 0).
/// - `alpha`: the shape parameter of the distribution (must be > 0).
/// - `beta`: the scale parameter of the distribution (must be > 0).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns an error if `x` < 0, `alpha` <= 0, or `beta` <= 0.
pub fn gamma_dist(
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
        "GAMMADIST",
        codcel_gamma_dist,
    )
}

/// Excel-compatible `GAMMAINV` function.
/// Returns the inverse of the gamma cumulative distribution.
/// - `probability`: the probability associated with the gamma distribution (0 to 1).
/// - `alpha`: the shape parameter of the distribution (must be > 0).
/// - `beta`: the scale parameter of the distribution (must be > 0).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the value `x` such that `GAMMADIST(x, alpha, beta, TRUE) = probability`.
pub fn gamma_inv(
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
        "GAMMAINV",
        codcel_gamma_inv_vec,
    )
}

/// Excel-compatible `HYPGEOMDIST` function.
/// Returns the hypergeometric distribution probability.
/// - `sample_successes`: number of successes in the sample (must be >= 0).
/// - `sample_size`: the sample size (0 to population_size).
/// - `total_successes`: number of successes in the population (0 to population_size).
/// - `population_size`: the population size (must be > 0).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the probability of getting exactly `sample_successes` successes in a sample drawn without replacement.
pub fn hypgeom_dist(
    sample_successes: Value,
    sample_size: Value,
    total_successes: Value,
    population_size: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    process_area_int_multi_to_float(
        vec![
            sample_successes,
            sample_size,
            total_successes,
            population_size,
        ],
        strict_type_conversion,
        value_format,
        "HYPGEOMDIST",
        codcel_hypgeom_dist_vec,
    )
}

/// Excel-compatible `LOGINV` function.
/// Returns the inverse of the lognormal cumulative distribution function.
/// - `probability`: the probability associated with the lognormal distribution (0 to 1).
/// - `mean`: the mean of ln(x).
/// - `std_dev`: the standard deviation of ln(x) (must be > 0).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the value `x` such that `LOGNORMDIST(x, mean, std_dev) = probability`.
pub fn log_inv(
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
        "LOGINV",
        codcel_log_inv_vec,
    )
}

/// Excel-compatible `LOGNORMDIST` function.
/// Returns the cumulative lognormal distribution.
/// - `x`: the value at which to evaluate the function (must be > 0).
/// - `mean`: the mean of ln(x).
/// - `std_dev`: the standard deviation of ln(x) (must be > 0).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the probability that ln(x) is less than or equal to the given parameters.
pub fn log_norm_dist(
    x: Value,
    mean: Value,
    std_dev: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![x, mean, std_dev],
        strict_type_conversion,
        value_format,
        "LOGNORMDIST",
        codcel_log_norm_dist_vec,
    )
}

/// Excel-compatible `MODE` function.
/// Returns the most frequently occurring value in a data set.
/// - `values`: a vector of numeric values.
/// - `value_format`: format settings for value conversion.
///
/// Returns an error if no value occurs more than once or if the data set is empty.
pub fn mode(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_mode(values)?))
}

/// Excel-compatible `NEGBINOMDIST` function.
/// Returns the negative binomial distribution probability.
/// - `failures`: the number of failures (must be >= 0).
/// - `successes`: the threshold number of successes (must be >= 1).
/// - `probability`: the probability of success on each trial (0 to 1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the probability that there will be exactly `failures` failures before the `successes`-th success.
pub fn neg_binom_dist(
    failures: Value,
    successes: Value,
    probability: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_int_float_to_float(
        failures,
        successes,
        probability,
        strict_type_conversion,
        value_format,
        "NEGBINOMDIST",
        codcel_neg_binom_dist,
    )
}

/// Excel-compatible `NORMDIST` function.
/// Returns the normal distribution for the specified mean and standard deviation.
/// - `x`: the value at which to evaluate the distribution.
/// - `mean`: the arithmetic mean of the distribution.
/// - `std_dev`: the standard deviation of the distribution (must be > 0).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns an error if `std_dev` <= 0.
pub fn norm_dist(
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
        "NORMDIST",
        codcel_norm_dist,
    )
}

/// Excel-compatible `NORMINV` function.
/// Returns the inverse of the normal cumulative distribution.
/// - `probability`: the probability corresponding to the normal distribution (0 to 1).
/// - `mean`: the arithmetic mean of the distribution.
/// - `std_dev`: the standard deviation of the distribution (must be > 0).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the value `x` such that `NORMDIST(x, mean, std_dev, TRUE) = probability`.
pub fn norm_inv(
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
        "NORMINV",
        codcel_norm_inv_vec,
    )
}

/// Excel-compatible `NORMSDIST` function.
/// Returns the standard normal cumulative distribution function.
/// - `z`: the value at which to evaluate the distribution (z-score).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns `P(Z <= z)` where Z follows a standard normal distribution (mean = 0, std_dev = 1).
pub fn norm_s_dist(
    z: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![z],
        strict_type_conversion,
        value_format,
        "NORMSDIST",
        codcel_norm_s_dist_vec,
    )
}

/// Excel-compatible `NORMSINV` function.
/// Returns the inverse of the standard normal cumulative distribution.
/// - `probability`: the probability corresponding to the standard normal distribution (0 to 1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the z-score such that `NORMSDIST(z) = probability`.
pub fn norm_s_inv(
    probability: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![probability],
        strict_type_conversion,
        value_format,
        "NORMSINV",
        codcel_norm_s_inv_vec,
    )
}

/// Excel-compatible `PERCENTRANK` function.
/// Returns the percentage rank of a value in a data set.
/// - `array`: the range of numeric values defining the data set.
/// - `x`: the value for which to find the percentage rank.
/// - `significance`: optional number of significant digits for the returned percentage (defaults to 3).
/// - `value_format`: format settings for value conversion.
///
/// Returns a value between 0 and 1 (inclusive) representing the relative position of `x` in the data set.
pub fn percent_rank(
    array: Value,
    x: Value,
    significance: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = array.to_flatterned_vec_f64(value_format)?;
    let x = x.f64(value_format)?;
    let significance = significance.option_i32(value_format)?;
    Ok(Value::F64(codcel_percent_rank(array, x, significance)?))
}

/// Excel-compatible `PERCENTILE` function.
/// Returns the k-th percentile of values in a range.
/// - `array`: the range of numeric values.
/// - `k`: the percentile value (0 to 1 inclusive).
/// - `value_format`: format settings for value conversion.
///
/// Returns the value at the k-th percentile. For example, `k = 0.5` returns the median.
pub fn percentile(
    array: Value,
    k: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let array = array.to_flatterned_vec_f64(value_format)?;
    let k = k.f64(value_format)?;
    Ok(Value::F64(codcel_percentile(array, k)?))
}

/// Excel-compatible `POISSON` function.
/// Evaluates the Poisson distribution.
/// - `x`: number of events (must be non-negative integer).
/// - `mean`: expected number of events (must be non-negative).
/// - `cumulative`: `true` for cumulative probability `P(X <= x)`, `false` for probability mass `P(X = x)`.
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns an error on negative counts or negative mean.
pub fn poisson(
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
        "POISSON",
        codcel_poisson,
    )
}

/// Excel-compatible `QUARTILE` function.
/// Returns the quartile of a data set.
/// - `array`: the range of numeric values.
/// - `quart`: the quartile to return (0 = minimum, 1 = 25th percentile, 2 = median, 3 = 75th percentile, 4 = maximum).
/// - `value_format`: format settings for value conversion.
///
/// Returns an error if `quart` is not 0, 1, 2, 3, or 4.
pub fn quartile(
    array: Value,
    quart: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
    let array = array.to_flatterned_vec_f64(value_format)?;
    let quart = quart.i32(value_format)?;
    Ok(Value::F64(codcel_quartile(array, quart)?))
}

/// Excel-compatible `RANK` function.
/// Returns the rank of a number in a list of numbers.
/// - `value`: the number whose rank to find.
/// - `array`: the range of numeric values.
/// - `order`: optional; `false` or omitted for descending order (largest = 1), `true` for ascending order (smallest = 1).
/// - `value_format`: format settings for value conversion.
///
/// Returns the rank as an integer. If `value` is not found in `array`, returns an error.
pub fn rank(
    value: Value,
    array: Value,
    order: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    let array = array.to_flatterned_vec_f64(value_format)?;
    let order = order.option_bool(value_format)?;
    Ok(Value::I32(codcel_rank(value, array, order)?))
}

/// Excel-compatible `STDEV` function.
/// Estimates the sample standard deviation based on a sample.
/// - `values`: a vector of numeric values representing the sample.
/// - `value_format`: format settings for value conversion.
///
/// Returns the sample standard deviation (using n-1 denominator). Ignores text and logical values.
pub fn st_dev(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_st_dev(values)?))
}

/// Excel-compatible `STDEVP` function.
/// Calculates the population standard deviation based on the entire population.
/// - `values`: a vector of numeric values representing the entire population.
/// - `value_format`: format settings for value conversion.
///
/// Returns the population standard deviation (using n denominator). Ignores text and logical values.
pub fn st_dev_p(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_st_dev_p(values)?))
}

/// Excel-compatible `TDIST` function.
/// Returns the Student's t-distribution.
/// - `x`: the numeric value at which to evaluate the distribution (must be >= 0).
/// - `degrees_freedom`: degrees of freedom (must be >= 1).
/// - `tails`: number of distribution tails (1 for one-tailed, 2 for two-tailed).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the probability from the t-distribution.
pub fn t_dist(
    x: Value,
    degrees_freedom: Value,
    tails: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_float_int_to_float(
        x,
        degrees_freedom,
        tails,
        strict_type_conversion,
        value_format,
        "TDIST",
        codcel_t_dist,
    )
}

/// Excel-compatible `TINV` function.
/// Returns the inverse of the two-tailed Student's t-distribution.
/// - `probability`: the two-tailed probability (0 to 1).
/// - `degrees_freedom`: degrees of freedom (must be >= 1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns the value `t` such that `TDIST(t, degrees_freedom, 2) = probability`.
pub fn t_inv(
    probability: Value,
    degrees_freedom: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_float_multi_to_float(
        vec![probability, degrees_freedom],
        strict_type_conversion,
        value_format,
        "TINV",
        codcel_t_inv_vec,
    )
}

/// Excel-compatible `TTEST` function.
/// Returns the probability associated with a Student's t-test.
/// - `array1`: the first data set.
/// - `array2`: the second data set.
/// - `tails`: number of distribution tails (1 for one-tailed, 2 for two-tailed).
/// - `type_value`: the type of t-test (1 = paired, 2 = two-sample equal variance, 3 = two-sample unequal variance).
/// - `value_format`: format settings for value conversion.
///
/// Returns the p-value of the t-test.
pub fn t_test(
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

    Ok(Value::F64(codcel_t_test(
        array1, array2, tails, type_value,
    )?))
}

/// Excel-compatible `VAR` function.
/// Estimates the sample variance based on a sample.
/// - `values`: a vector of numeric values representing the sample.
/// - `value_format`: format settings for value conversion.
///
/// Returns the sample variance (using n-1 denominator). Ignores text and logical values.
pub fn var(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_f64(values, value_format)?;
    Ok(Value::F64(codcel_var(values)?))
}

/// Excel-compatible `WEIBULL` function.
/// Returns the Weibull distribution.
/// - `x`: the value at which to evaluate the function (must be >= 0).
/// - `alpha`: the shape parameter of the distribution (must be > 0).
/// - `beta`: the scale parameter of the distribution (must be > 0).
/// - `cumulative`: `true` for cumulative distribution function, `false` for probability density function.
/// - `strict_type_conversion`: if `true`, enforces strict type conversion.
/// - `value_format`: format settings for value conversion.
///
/// Returns an error if `x` < 0, `alpha` <= 0, or `beta` <= 0.
pub fn weibull(
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
        "WEIBULL",
        codcel_weibull,
    )
}

/// Excel-compatible `ZTEST` function.
/// Returns the one-tailed p-value of a z-test.
/// - `data`: the range of numeric values to test against the hypothesized mean.
/// - `hyp_mean`: the hypothesized population mean.
/// - `sigma`: optional population standard deviation; if omitted, the sample standard deviation is used.
/// - `value_format`: format settings for value conversion.
///
/// Returns the one-tailed probability that the sample mean is greater than the observed mean.
pub fn z_test(
    data: Value,
    hyp_mean: Value,
    sigma: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let data = data.to_flatterned_vec_f64(value_format)?;
    let hyp_mean = hyp_mean.f64(value_format)?;
    let sigma = sigma.option_f64(value_format)?;

    Ok(Value::F64(codcel_z_test(data, hyp_mean, sigma)?))
}
