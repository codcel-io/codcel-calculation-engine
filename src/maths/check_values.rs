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
    // TODO: PERHAPS WE NEED THIS CHECK ON ALL FUNCTIONS
    // PERHAPS IF IT HAPPENS WE SHOULD RETURN 0.0?????
    if value.is_nan() {
        return Err(format!("{function}: Input is NaN.").into());
    }
    if value.is_infinite() {
        return Err(format!("{function}: Input is infinity.").into());
    }

    Ok(())
}

pub(crate) fn check_bin_op(
    lhs: f64,
    rhs: f64,
    op_name: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    check_value_f64(&format!("{op_name}: - 1st"), lhs)?;
    check_value_f64(&format!("{op_name}: - 2nd"), rhs)?;
    Ok(())
}
