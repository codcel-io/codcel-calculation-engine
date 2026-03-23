// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

use crate::statistical::factorial::factorial;

// Helper function to calculate the Poisson probability mass function
pub(crate) fn poisson_probability(x: i32, mean: f64) -> f64 {
    crate::portable_math::exp(-mean) * mean.powi(x) / factorial(x)
}
