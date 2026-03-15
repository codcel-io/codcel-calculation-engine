// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0 OR Codcel-Commercial
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT, LICENSE-APACHE, and LICENSE-CODCEL-COMMERCIAL in the project root.

// Helper function to calculate factorial
pub(crate) fn factorial(n: i32) -> f64 {
    (1..=n).fold(1.0, |acc, val| acc * val as f64)
}
