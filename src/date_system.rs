// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! Which serial-number convention a date value is expressed in.
//!
//! This engine works in the **1900 date serial system** only, where 1900-01-01
//! is serial 1. Excel also has a 1904 system (originally the Mac default,
//! flagged by `workbookPr/@date1904`), which numbers 1904-01-01 as serial 0 and
//! sits exactly 1462 days below the 1900 system — but that is a property of a
//! *file*, not of a calculation. `codcel-excel-loader` rebases 1904 serials onto
//! 1900 as a workbook is read, so by the time any value reaches this crate there
//! is only one epoch in play.
//!
//! That is deliberate, and this type is where it is enforced: there is no way to
//! ask for the 1904 epoch here, so the 1462-day shift cannot be applied a second
//! time. It once could, and a Mac workbook's dates landed four years out.
//!
//! What is left to choose is the Lotus 1-2-3 leap-year bug: Excel believes
//! 1900-02-29 exists, occupying serial 60, so every serial from 61 onward is one
//! higher than a strictly correct 1900 system would give.
//!
//! Note that Excel's mapping is *correct for every real date*. The only
//! anomalies are serial 60, which denotes a day that never existed, and day
//! counts that span February 1900. Turning [`DateSemantics::lotus_1900_bug`]
//! off does not make dates "more accurate" — it re-bases the serial system, so
//! every date from 1900-03-01 onward moves one day later relative to Excel.

/// How to translate between a numeric Excel serial and a calendar instant.
///
/// Deliberately a named struct rather than a bare `bool`: a distinct type forces
/// every call site to state which convention it means, which is how a batch of
/// hardcoded `true`s scattered through the transpiler was found. Keep it that
/// way even though only one field remains. For the same reason there is no
/// `From<bool>` impl.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateSemantics {
    /// Serial 60 is the fictitious 1900-02-29, so serials at or above 60 are one
    /// higher than the true day count.
    pub lotus_1900_bug: bool,
}

impl DateSemantics {
    /// What Excel itself does: the 1900 epoch with the leap-year bug. This is the
    /// only convention that agrees with Excel about which calendar day a given
    /// serial denotes, so it is the right choice for anything read out of a
    /// `.xlsx`.
    pub const EXCEL_1900: Self = Self {
        lotus_1900_bug: true,
    };

    /// A strictly correct 1900 serial system with no phantom day. Self-consistent,
    /// but one day out of step with Excel for every date from 1900-03-01 onward.
    pub const ASTRONOMICAL_1900: Self = Self {
        lotus_1900_bug: false,
    };
}

impl Default for DateSemantics {
    fn default() -> Self {
        Self::EXCEL_1900
    }
}
