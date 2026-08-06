// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use crate::area::process_area_int_multi_to_int;
use crate::engineering::codcel_bassel_j::codcel_bessel_j;
use crate::engineering::codcel_bassel_k::codcel_bessel_k;
use crate::engineering::codcel_bessel_i::codcel_bessel_i;
use crate::engineering::codcel_bessel_y::codcel_bessel_y;
use crate::engineering::codcel_bin_2_dec::codcel_bin_2_dec;
use crate::engineering::codcel_bin_2_hex::codcel_bin_2_hex;
use crate::engineering::codcel_bin_2_oct::codcel_bin_2_oct;
use crate::engineering::codcel_bit_and::codcel_bit_and_vec;
use crate::engineering::codcel_bit_l_shift::codcel_bit_l_shift_vec;
use crate::engineering::codcel_bit_or::codcel_bit_or_vec;
use crate::engineering::codcel_bit_r_shift::codcel_bit_r_shift_vec;
use crate::engineering::codcel_bit_xor::codcel_bit_xor_vec;
use crate::engineering::codcel_complex::codcel_complex;
use crate::engineering::codcel_convert::codcel_convert;
use crate::engineering::codcel_dec_2_bin::codcel_dec_2_bin;
use crate::engineering::codcel_dec_2_hex::codcel_dec_2_hex;
use crate::engineering::codcel_dec_2_oct::codcel_dec_2_oct;
use crate::engineering::codcel_delta::codcel_delta;
use crate::engineering::codcel_erf::codcel_erf;
use crate::engineering::codcel_erf_precise::codcel_erf_precise;
use crate::engineering::codcel_erfc::codcel_erfc;
use crate::engineering::codcel_erfc_precise::codcel_erfc_precise;
use crate::engineering::codcel_ge_step::codcel_ge_step;
use crate::engineering::codcel_hex_2_bin::codcel_hex_2_bin;
use crate::engineering::codcel_hex_2_dec::codcel_hex_2_dec;
use crate::engineering::codcel_hex_2_oct::codcel_hex_2_oct;
use crate::engineering::codcel_im_abs::codcel_im_abs;
use crate::engineering::codcel_im_argument::codcel_im_argument;
use crate::engineering::codcel_im_conjugate::codcel_im_conjugate;
use crate::engineering::codcel_im_cos::codcel_im_cos;
use crate::engineering::codcel_im_cosh::codcel_im_cosh;
use crate::engineering::codcel_im_cot::codcel_im_cot;
use crate::engineering::codcel_im_csc::codcel_im_csc;
use crate::engineering::codcel_im_csch::codcel_im_csch;
use crate::engineering::codcel_im_div::codcel_im_div;
use crate::engineering::codcel_im_exp::codcel_im_exp;
use crate::engineering::codcel_im_ln::codcel_im_ln;
use crate::engineering::codcel_im_log10::codcel_im_log10;
use crate::engineering::codcel_im_log2::codcel_im_log2;
use crate::engineering::codcel_im_power::codcel_im_power;
use crate::engineering::codcel_im_product::codcel_im_product;
use crate::engineering::codcel_im_real::codcel_im_real;
use crate::engineering::codcel_im_sec::codcel_im_sec;
use crate::engineering::codcel_im_sech::codcel_im_sech;
use crate::engineering::codcel_im_sin::codcel_im_sin;
use crate::engineering::codcel_im_sinh::codcel_im_sinh;
use crate::engineering::codcel_im_sqrt::codcel_im_sqrt;
use crate::engineering::codcel_im_sub::codcel_im_sub;
use crate::engineering::codcel_im_sum::codcel_im_sum;
use crate::engineering::codcel_im_tan::codcel_im_tan;
use crate::engineering::codcel_imaginary::codcel_imaginary;
use crate::engineering::codcel_oct_2_bin::codcel_oct_2_bin;
use crate::engineering::codcel_oct_2_dec::codcel_oct_2_dec;
use crate::engineering::codcel_oct_2_hex::codcel_oct_2_hex;
use crate::value::{vec_value_to_vec_string, Value};
use crate::value_format::ValueFormat;
use std::error::Error;

/// Excel-compatible `CONVERT` function.
/// Converts a number from one measurement unit to another.
/// - `value`: the numeric value to convert.
/// - `from_unit`: the unit of the input value (e.g., "m", "ft", "kg").
/// - `to_unit`: the unit to convert to (e.g., "cm", "in", "lb").
///
/// Returns the converted value, or an error if the units are incompatible or unrecognized.
pub fn convert(
    value: Value,
    from_unit: Value,
    to_unit: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let value = value.f64(value_format)?;
    let from_unit = from_unit.string(value_format)?;
    let to_unit = to_unit.string(value_format)?;
    Ok(Value::F64(codcel_convert(value, from_unit, to_unit)?))
}

/// Excel-compatible `BESSELI` function.
/// Returns the modified Bessel function of the first kind, I_n(x).
/// - `x`: the value at which to evaluate the function.
/// - `n`: the order of the Bessel function (must be non-negative integer).
///
/// Returns the value of I_n(x), or an error if n is negative.
pub fn bessel_i(
    x: Value,
    n: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;
    let n = n.i32(value_format)?;

    Ok(Value::F64(codcel_bessel_i(x, n)?))
}

/// Excel-compatible `BESSELJ` function.
/// Returns the Bessel function of the first kind, J_n(x).
/// - `x`: the value at which to evaluate the function.
/// - `n`: the order of the Bessel function (must be non-negative integer).
///
/// Returns the value of J_n(x), or an error if n is negative.
pub fn bessel_j(
    x: Value,
    n: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;
    let n = n.i32(value_format)?;

    Ok(Value::F64(codcel_bessel_j(x, n)?))
}

/// Excel-compatible `BESSELK` function.
/// Returns the modified Bessel function of the second kind, K_n(x).
/// - `x`: the value at which to evaluate the function (must be positive).
/// - `n`: the order of the Bessel function (must be non-negative integer).
///
/// Returns the value of K_n(x), or an error if x is non-positive or n is negative.
pub fn bessel_k(
    x: Value,
    n: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;
    let n = n.i32(value_format)?;

    Ok(Value::F64(codcel_bessel_k(x, n)?))
}

/// Excel-compatible `BESSELY` function.
/// Returns the Bessel function of the second kind, Y_n(x).
/// - `x`: the value at which to evaluate the function (must be positive).
/// - `n`: the order of the Bessel function (must be non-negative integer).
///
/// Returns the value of Y_n(x), or an error if x is non-positive or n is negative.
pub fn bessel_y(
    x: Value,
    n: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;
    let n = n.i32(value_format)?;

    Ok(Value::F64(codcel_bessel_y(x, n)?))
}

/// Excel-compatible `BIN2DEC` function.
/// Converts a binary number (as a string) to its decimal equivalent.
/// - `text`: a string representing a binary number (up to 10 characters, using two's complement for negative values).
///
/// Returns the decimal integer value, or an error if the input is not a valid binary string.
pub fn bin_2_dec(
    text: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    Ok(Value::I32(codcel_bin_2_dec(text)?))
}

/// Excel-compatible `BIN2HEX` function.
/// Converts a binary number (as a string) to its hexadecimal equivalent.
/// - `text`: a string representing a binary number (up to 10 characters).
/// - `places`: optional number of characters in the result; pads with leading zeros if needed.
///
/// Returns the hexadecimal string, or an error if the input is invalid or places is insufficient.
pub fn bin_2_hex(
    text: Value,
    places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let places = places.option_i32(value_format)?;

    Ok(Value::String(codcel_bin_2_hex(text, places)?))
}

/// Excel-compatible `BIN2OCT` function.
/// Converts a binary number (as a string) to its octal equivalent.
/// - `text`: a string representing a binary number (up to 10 characters).
/// - `places`: optional number of characters in the result; pads with leading zeros if needed.
///
/// Returns the octal string, or an error if the input is invalid or places is insufficient.
pub fn bin_2_oct(
    text: Value,
    places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let places = places.option_i32(value_format)?;

    Ok(Value::String(codcel_bin_2_oct(text, places)?))
}

/// Excel-compatible `BITAND` function.
/// Returns a bitwise AND of two non-negative integers.
/// - `value1`: the first integer (must be non-negative and <= 2^48-1).
/// - `value2`: the second integer (must be non-negative and <= 2^48-1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion rules.
///
/// Returns the bitwise AND result, or an error if values are out of range.
pub fn bit_and(
    value1: Value,
    value2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_multi_to_int(
        vec![value1, value2],
        strict_type_conversion,
        value_format,
        "BITAND",
        codcel_bit_and_vec,
    )
}

/// Excel-compatible `BITLSHIFT` function.
/// Returns a number shifted left by a specified number of bits.
/// - `value1`: the integer to shift (must be non-negative and <= 2^48-1).
/// - `value2`: the number of bits to shift left (negative values shift right).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion rules.
///
/// Returns the shifted result, or an error if values are out of range.
pub fn bit_l_shift(
    value1: Value,
    value2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_multi_to_int(
        vec![value1, value2],
        strict_type_conversion,
        value_format,
        "BITLSHIFT",
        codcel_bit_l_shift_vec,
    )
}

/// Excel-compatible `BITOR` function.
/// Returns a bitwise OR of two non-negative integers.
/// - `value1`: the first integer (must be non-negative and <= 2^48-1).
/// - `value2`: the second integer (must be non-negative and <= 2^48-1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion rules.
///
/// Returns the bitwise OR result, or an error if values are out of range.
pub fn bit_or(
    value1: Value,
    value2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_multi_to_int(
        vec![value1, value2],
        strict_type_conversion,
        value_format,
        "BITOR",
        codcel_bit_or_vec,
    )
}

/// Excel-compatible `BITRSHIFT` function.
/// Returns a number shifted right by a specified number of bits.
/// - `value1`: the integer to shift (must be non-negative and <= 2^48-1).
/// - `value2`: the number of bits to shift right (negative values shift left).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion rules.
///
/// Returns the shifted result, or an error if values are out of range.
pub fn bit_r_shift(
    value1: Value,
    value2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_multi_to_int(
        vec![value1, value2],
        strict_type_conversion,
        value_format,
        "BITRSHIFT",
        codcel_bit_r_shift_vec,
    )
}

/// Excel-compatible `BITXOR` function.
/// Returns a bitwise XOR (exclusive OR) of two non-negative integers.
/// - `value1`: the first integer (must be non-negative and <= 2^48-1).
/// - `value2`: the second integer (must be non-negative and <= 2^48-1).
/// - `strict_type_conversion`: if `true`, enforces strict type conversion rules.
///
/// Returns the bitwise XOR result, or an error if values are out of range.
pub fn bit_xor(
    value1: Value,
    value2: Value,
    strict_type_conversion: bool,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    process_area_int_multi_to_int(
        vec![value1, value2],
        strict_type_conversion,
        value_format,
        "BITXOR",
        codcel_bit_xor_vec,
    )
}

/// Excel-compatible `COMPLEX` function.
/// Creates a complex number from real and imaginary coefficients.
/// - `real`: the real coefficient of the complex number.
/// - `imaginary`: the imaginary coefficient of the complex number.
/// - `suffix`: optional suffix for the imaginary unit ("i" or "j"); defaults to "i".
///
/// Returns a string representation of the complex number (e.g., "3+4i"), or an error if suffix is invalid.
pub fn complex(
    real: Value,
    imaginary: Value,
    suffix: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let real = real.f64(value_format)?;
    let imaginary = imaginary.f64(value_format)?;
    let suffix = suffix.option_string(value_format)?;

    Ok(Value::String(codcel_complex(
        real,
        imaginary,
        suffix,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `DEC2BIN` function.
/// Converts a decimal integer to its binary representation.
/// - `number`: the decimal integer to convert (must be between -512 and 511).
/// - `places`: optional number of characters in the result; pads with leading zeros if needed.
///
/// Returns the binary string, or an error if the number is out of range or places is insufficient.
pub fn dec_2_bin(
    number: Value,
    places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number = number.i32(value_format)?;
    let places = places.option_i32(value_format)?;

    Ok(Value::String(codcel_dec_2_bin(number, places)?))
}

/// Excel-compatible `DEC2HEX` function.
/// Converts a decimal integer to its hexadecimal representation.
/// - `number`: the decimal integer to convert (must be between -549755813888 and 549755813887).
/// - `places`: optional number of characters in the result; pads with leading zeros if needed.
///
/// Returns the hexadecimal string, or an error if the number is out of range or places is insufficient.
pub fn dec_2_hex(
    number: Value,
    places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number = number.f64(value_format)? as i64;
    let places = places.option_i32(value_format)?;

    Ok(Value::String(codcel_dec_2_hex(number, places)?))
}

/// Excel-compatible `DEC2OCT` function.
/// Converts a decimal integer to its octal representation.
/// - `number`: the decimal integer to convert (must be between -536870912 and 536870911).
/// - `places`: optional number of characters in the result; pads with leading zeros if needed.
///
/// Returns the octal string, or an error if the number is out of range or places is insufficient.
pub fn dec_2_oct(
    number: Value,
    places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number = number.i32(value_format)?;
    let places = places.option_i32(value_format)?;

    Ok(Value::String(codcel_dec_2_oct(number, places)?))
}

/// Excel-compatible `DELTA` function.
/// Tests whether two values are equal.
/// - `number_1`: the first number to compare.
/// - `number_2`: optional second number to compare; defaults to 0.
///
/// Returns 1 if the values are equal, 0 otherwise.
pub fn delta(
    number_1: Value,
    number_2: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number_1 = number_1.f64(value_format)?;
    let number_2 = number_2.option_f64(value_format)?;

    Ok(Value::I32(codcel_delta(number_1, number_2)?))
}

/// Excel-compatible `ERF` function.
/// Returns the error function integrated between specified bounds.
/// - `lower_limit`: the lower bound for integration (or the upper bound if `upper_limit` is omitted).
/// - `upper_limit`: optional upper bound for integration; if omitted, integrates from 0 to `lower_limit`.
///
/// Returns the error function value erf(x) = (2/√π) ∫₀ˣ e^(-t²) dt.
pub fn erf(
    lower_limit: Value,
    upper_limit: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let lower_limit = lower_limit.f64(value_format)?;
    let upper_limit = upper_limit.option_f64(value_format)?;

    Ok(Value::F64(codcel_erf(lower_limit, upper_limit)?))
}

/// Excel-compatible `ERF.PRECISE` function.
/// Returns the error function integrated between 0 and the specified value.
/// - `x`: the upper bound for integration.
///
/// Returns the error function value erf(x) = (2/√π) ∫₀ˣ e^(-t²) dt.
pub fn erf_precise(
    x: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;

    Ok(Value::F64(codcel_erf_precise(x)?))
}

/// Excel-compatible `ERFC.PRECISE` function.
/// Returns the complementary error function integrated from x to infinity.
/// - `x`: the lower bound for integration.
///
/// Returns the complementary error function value erfc(x) = 1 - erf(x) = (2/√π) ∫ₓ^∞ e^(-t²) dt.
pub fn erfc_precise(
    x: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;

    Ok(Value::F64(codcel_erfc_precise(x)?))
}

/// Excel-compatible `ERFC` function.
/// Returns the complementary error function integrated from x to infinity.
/// - `x`: the lower bound for integration.
///
/// Returns the complementary error function value erfc(x) = 1 - erf(x) = (2/√π) ∫ₓ^∞ e^(-t²) dt.
pub fn erfc(x: Value, value_format: &ValueFormat) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let x = x.f64(value_format)?;

    Ok(Value::F64(codcel_erfc(x)?))
}

/// Excel-compatible `GESTEP` function.
/// Tests whether a number is greater than or equal to a step value.
/// - `number`: the value to test.
/// - `step`: optional threshold value; defaults to 0.
///
/// Returns 1 if number >= step, 0 otherwise.
pub fn ge_step(
    number: Value,
    step: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number_1 = number.f64(value_format)?;
    let number_2 = step.option_f64(value_format)?;

    Ok(Value::I32(codcel_ge_step(number_1, number_2)?))
}

/// Excel-compatible `HEX2BIN` function.
/// Converts a hexadecimal number (as a string) to its binary equivalent.
/// - `text`: a string representing a hexadecimal number (up to 10 characters).
/// - `places`: optional number of characters in the result; pads with leading zeros if needed.
///
/// Returns the binary string, or an error if the input is invalid or places is insufficient.
pub fn hex_2_bin(
    text: Value,
    places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let places = places.option_i32(value_format)?;

    Ok(Value::String(codcel_hex_2_bin(text, places)?))
}

/// Excel-compatible `HEX2DEC` function.
/// Converts a hexadecimal number (as a string) to its decimal equivalent.
/// - `text`: a string representing a hexadecimal number (up to 10 characters, using two's complement for negative values).
///
/// Returns the decimal integer value, or an error if the input is not a valid hexadecimal string.
pub fn hex_2_dec(
    text: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;

    Ok(Value::F64(codcel_hex_2_dec(text)? as f64))
}

/// Excel-compatible `HEX2OCT` function.
/// Converts a hexadecimal number (as a string) to its octal equivalent.
/// - `text`: a string representing a hexadecimal number (up to 10 characters).
/// - `places`: optional number of characters in the result; pads with leading zeros if needed.
///
/// Returns the octal string, or an error if the input is invalid or places is insufficient.
pub fn hex_2_oct(
    text: Value,
    places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let places = places.option_i32(value_format)?;

    Ok(Value::String(codcel_hex_2_oct(text, places)?))
}

/// Excel-compatible `IMABS` function.
/// Returns the absolute value (modulus) of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns the modulus |z| = √(x² + y²), or an error if the input is not a valid complex number.
pub fn im_abs(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::F64(codcel_im_abs(complex)?))
}

/// Excel-compatible `IMAGINARY` function.
/// Returns the imaginary coefficient of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns the imaginary part y from the complex number x + yi, or an error if the input is invalid.
pub fn imaginary(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::F64(codcel_imaginary(complex)?))
}

/// Excel-compatible `IMARGUMENT` function.
/// Returns the argument (phase angle in radians) of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns the angle θ where z = |z| * e^(iθ), in the range (-π, π], or an error if the input is invalid.
pub fn im_argument(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::F64(codcel_im_argument(complex)?))
}

/// Excel-compatible `IMCONJUGATE` function.
/// Returns the complex conjugate of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns the conjugate x - yi for the input x + yi, or an error if the input is invalid.
pub fn im_conjugate(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_conjugate(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMCOS` function.
/// Returns the cosine of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns cos(z) as a complex number string, or an error if the input is invalid.
pub fn im_cos(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_cos(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMCOSH` function.
/// Returns the hyperbolic cosine of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns cosh(z) as a complex number string, or an error if the input is invalid.
pub fn im_cosh(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_cosh(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMCOT` function.
/// Returns the cotangent of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns cot(z) = cos(z)/sin(z) as a complex number string, or an error if the input is invalid.
pub fn im_cot(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_cot(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMCSC` function.
/// Returns the cosecant of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns csc(z) = 1/sin(z) as a complex number string, or an error if the input is invalid.
pub fn im_csc(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_csc(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMCSCH` function.
/// Returns the hyperbolic cosecant of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns csch(z) = 1/sinh(z) as a complex number string, or an error if the input is invalid.
pub fn im_csch(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_csch(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMDIV` function.
/// Returns the quotient of two complex numbers.
/// - `numerator`: a string representing the numerator complex number.
/// - `denominator`: a string representing the denominator complex number.
///
/// Returns numerator / denominator as a complex number string, or an error if division by zero or invalid input.
pub fn im_div(
    numerator: Value,
    denominator: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let numerator = numerator.string(value_format)?;
    let denominator = denominator.string(value_format)?;

    Ok(Value::String(codcel_im_div(
        numerator,
        denominator,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMEXP` function.
/// Returns the exponential of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns e^z as a complex number string, or an error if the input is invalid.
pub fn im_exp(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_exp(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMLN` function.
/// Returns the natural logarithm of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns ln(z) as a complex number string, or an error if the input is zero or invalid.
pub fn im_ln(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_ln(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMLOG10` function.
/// Returns the base-10 logarithm of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns log₁₀(z) as a complex number string, or an error if the input is zero or invalid.
pub fn im_log10(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_log10(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMLOG2` function.
/// Returns the base-2 logarithm of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns log₂(z) as a complex number string, or an error if the input is zero or invalid.
pub fn im_log2(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_log2(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMPOWER` function.
/// Returns a complex number raised to a power.
/// - `complex`: a string representing the base complex number.
/// - `power`: a string representing the exponent (can be complex or real).
///
/// Returns z^n as a complex number string, or an error if the input is invalid.
pub fn im_power(
    complex: Value,
    power: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;
    let power = power.string(value_format)?;

    Ok(Value::String(codcel_im_power(
        complex,
        power,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMPRODUCT` function.
/// Returns the product of multiple complex numbers.
/// - `values`: a vector of complex number strings to multiply together.
///
/// Returns the product as a complex number string, or an error if any input is invalid.
pub fn im_product(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_string(values, value_format)?;
    Ok(Value::String(codcel_im_product(
        values,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMREAL` function.
/// Returns the real coefficient of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns the real part x from the complex number x + yi, or an error if the input is invalid.
pub fn im_real(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::F64(codcel_im_real(complex)?))
}

/// Excel-compatible `IMSEC` function.
/// Returns the secant of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns sec(z) = 1/cos(z) as a complex number string, or an error if the input is invalid.
pub fn im_sec(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_sec(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMSECH` function.
/// Returns the hyperbolic secant of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns sech(z) = 1/cosh(z) as a complex number string, or an error if the input is invalid.
pub fn im_sech(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_sech(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMSIN` function.
/// Returns the sine of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns sin(z) as a complex number string, or an error if the input is invalid.
pub fn im_sin(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_sin(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMSINH` function.
/// Returns the hyperbolic sine of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns sinh(z) as a complex number string, or an error if the input is invalid.
pub fn im_sinh(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_sinh(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMSQRT` function.
/// Returns the square root of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns √z as a complex number string (principal square root), or an error if the input is invalid.
pub fn im_sqrt(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_sqrt(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMSUB` function.
/// Returns the difference of two complex numbers.
/// - `number1`: a string representing the first complex number (minuend).
/// - `number2`: a string representing the second complex number (subtrahend).
///
/// Returns number1 - number2 as a complex number string, or an error if input is invalid.
pub fn im_sub(
    number1: Value,
    number2: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let number1 = number1.string(value_format)?;
    let number2 = number2.string(value_format)?;

    Ok(Value::String(codcel_im_sub(
        number1,
        number2,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMSUM` function.
/// Returns the sum of multiple complex numbers.
/// - `values`: a vector of complex number strings to add together.
///
/// Returns the sum as a complex number string, or an error if any input is invalid.
pub fn im_sum(
    values: Vec<Value>,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let values = vec_value_to_vec_string(values, value_format)?;
    Ok(Value::String(codcel_im_sum(
        values,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `IMTAN` function.
/// Returns the tangent of a complex number.
/// - `complex`: a string representing a complex number (e.g., "3+4i").
///
/// Returns tan(z) = sin(z)/cos(z) as a complex number string, or an error if the input is invalid.
pub fn im_tan(
    complex: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let complex = complex.string(value_format)?;

    Ok(Value::String(codcel_im_tan(
        complex,
        &value_format.decimal_separator,
        value_format.use_excel_rounding,
    )?))
}

/// Excel-compatible `OCT2BIN` function.
/// Converts an octal number (as a string) to its binary equivalent.
/// - `text`: a string representing an octal number (up to 10 characters).
/// - `places`: optional number of characters in the result; pads with leading zeros if needed.
///
/// Returns the binary string, or an error if the input is invalid or places is insufficient.
pub fn oct_2_bin(
    text: Value,
    places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let places = places.option_i32(value_format)?;

    Ok(Value::String(codcel_oct_2_bin(text, places)?))
}

/// Excel-compatible `OCT2DEC` function.
/// Converts an octal number (as a string) to its decimal equivalent.
/// - `text`: a string representing an octal number (up to 10 characters, using two's complement for negative values).
///
/// Returns the decimal integer value, or an error if the input is not a valid octal string.
pub fn oct_2_dec(
    text: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;

    Ok(Value::F64(codcel_oct_2_dec(text)? as f64))
}

/// Excel-compatible `OCT2HEX` function.
/// Converts an octal number (as a string) to its hexadecimal equivalent.
/// - `text`: a string representing an octal number (up to 10 characters).
/// - `places`: optional number of characters in the result; pads with leading zeros if needed.
///
/// Returns the hexadecimal string, or an error if the input is invalid or places is insufficient.
pub fn oct_2_hex(
    text: Value,
    places: Value,
    value_format: &ValueFormat,
) -> Result<Value, Box<dyn Error + Send + Sync>> {
    let text = text.string(value_format)?;
    let places = places.option_i32(value_format)?;

    Ok(Value::String(codcel_oct_2_hex(text, places)?))
}
