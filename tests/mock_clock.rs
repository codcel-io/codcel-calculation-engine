// SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// This file is part of Codcel (https://codcel.io).
// See LICENSE-MIT and LICENSE-APACHE in the project root.

//! `CODCEL_MOCK_NOW` end to end.
//!
//! # Why this is an integration test rather than a unit test
//!
//! `CODCEL_MOCK_NOW` is process-wide state, and `clock::now` consults it before
//! anything else. A unit test that set it would be visible to every other
//! clock-reading test in the library test binary for as long as it was set,
//! which made three of them fail on some thread interleavings and pass on
//! others. An integration test compiles to its own binary, so the variable set
//! here cannot reach them.
//!
//! # Why there is exactly one test in this file
//!
//! Two would race each other in this process exactly as they did in that one.
//! **Assert additional behaviour inside this test, not alongside it.** Anything
//! that does not need the environment belongs in `src/clock.rs`, where
//! `parse_mock_now` is already tested as a pure function.

// The whole file is gated: with `mock-clock` off there is no environment
// variable to exercise, and the imports would be unused.
#[cfg(feature = "mock-clock")]
use codcel_calculation_engine::{clock, value_format::ValueFormat};

#[cfg(feature = "mock-clock")]
#[test]
fn the_mock_clock_freezes_now_and_today() {
    let vf = ValueFormat::default();

    std::env::set_var("CODCEL_MOCK_NOW", "2023-05-15T14:30:45Z");

    assert_eq!(
        clock::now(&vf).to_rfc3339(),
        "2023-05-15T14:30:45+00:00",
        "NOW is frozen at the mocked instant"
    );
    assert_eq!(
        clock::today(&vf).unwrap().to_rfc3339(),
        "2023-05-15T00:00:00+00:00",
        "TODAY is that instant truncated to midnight"
    );

    // Repeated reads agree, which is the property that makes a workbook's
    // TODAY() fixture deterministic instead of only passing on its save date.
    assert_eq!(clock::now(&vf), clock::now(&vf));

    // The mock outranks an explicitly named zone, so a suite can pin the clock
    // without also having to pin every project's timezone.
    let tokyo = ValueFormat {
        timezone: "Asia/Tokyo".to_string(),
        ..Default::default()
    };
    assert_eq!(clock::now(&tokyo), clock::now(&vf));

    // An unparseable value leaves the real clock in place rather than
    // propagating an error.
    std::env::set_var("CODCEL_MOCK_NOW", "not a timestamp");
    let live = clock::now(&vf);
    let host = chrono::Local::now().naive_local();
    assert!(
        (live.naive_utc() - host).num_seconds().abs() <= 5,
        "fell back to the host clock: {live} vs {host}"
    );

    std::env::remove_var("CODCEL_MOCK_NOW");
}
