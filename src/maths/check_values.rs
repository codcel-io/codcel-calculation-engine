// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

use std::error::Error;

pub(crate) fn check_value_f64(
    function: &str,
    value: f64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // NaN is the engine's #N/A (NA(), the #N/A cells of LINEST), so a calculation on it is #N/A,
    // as in Excel. Infinity only comes from an overflow, which Excel reports as #NUM!.
    if value.is_nan() {
        return Err(format!("#N/A {function}: Input is NaN.").into());
    }
    if value.is_infinite() {
        return Err(format!("#NUM! {function}: Input is infinity.").into());
    }

    Ok(())
}

pub(crate) fn check_bin_op(
    lhs: f64,
    rhs: f64,
    op_name: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // The labels are only needed for an error: not built for every operation.
    if lhs.is_finite() && rhs.is_finite() {
        return Ok(());
    }
    check_value_f64(&format!("{op_name}: - 1st"), lhs)?;
    check_value_f64(&format!("{op_name}: - 2nd"), rhs)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::excel_error::ExcelError;

    #[test]
    fn a_nan_operand_is_na_and_an_infinite_one_is_num() {
        let nan = check_bin_op(f64::NAN, 1.0, "ADD").unwrap_err();
        assert_eq!(ExcelError::from_err(nan.as_ref()), ExcelError::Na);
        let inf = check_bin_op(1.0, f64::INFINITY, "ADD").unwrap_err();
        assert_eq!(ExcelError::from_err(inf.as_ref()), ExcelError::Num);
    }
}
