<p align="center">
  <a href="https://codcel.io">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/codcel-io/codcel-calculation-engine/refs/tags/release-0.1.9/assets/codcel-logo-lockup-dark.svg">
      <img src="https://raw.githubusercontent.com/codcel-io/codcel-calculation-engine/refs/tags/release-0.1.9/assets/codcel-logo-lockup.svg" alt="Codcel" width="320">
    </picture>
  </a>
</p>

# Codcel Calculation Engine

[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#licensing)

A high-performance Rust library implementing 460+ Excel-compatible functions — financial, statistical, mathematical, engineering, text, date/time, and more.

## Overview

Codcel Calculation Engine provides pure-Rust implementations of Excel worksheet functions. Each function is designed to match Excel's behavior, including edge cases, rounding conventions, error propagation, and date serial number handling.

This is the open-source calculation core of [Codcel](https://codcel.io). Codcel converts your Excel spreadsheets into clean, human-readable source code — in Rust, Python, Java, C#, TypeScript, Go, Swift, and more. You get the full source code, and this engine is part of what you get: your generated projects call into it directly, so you can inspect exactly how every number is computed. No black boxes.

The engine can also be used as a standalone library in any Rust project that needs Excel-compatible calculations.

## Supported Function Categories

| Category | Examples | Count |
|---|---|---|
| Financial | `NPV`, `IRR`, `XIRR`, `PMT`, `PRICE`, `YIELD`, `ACCRINT` | 57 |
| Statistical | `NORM.DIST`, `T.TEST`, `LINEST`, `FORECAST`, `CORREL` | 111 |
| Math & Trig | `SUM`, `ROUND`, `MMULT`, `MDETERM`, `SEQUENCE` | 85 |
| Engineering | `CONVERT`, `COMPLEX`, `BESSELI`, `ERF`, `BIN2DEC` | 57 |
| Text | `TEXT`, `SUBSTITUTE`, `TEXTJOIN`, `CONCAT`, `TRIM` | 46 |
| Date & Time | `DATE`, `NETWORKDAYS.INTL`, `WORKDAY`, `YEARFRAC` | 34 |
| Lookup & Reference | `VLOOKUP`, `XLOOKUP`, `INDEX`, `MATCH`, `UNIQUE` | 31 |
| Compatibility | Legacy function aliases | 40 |
| Logical | `AND`, `OR`, `XOR`, `IF`, `IFS`, `SWITCH` | 7 |
| Comparison / Rounding / Info | `ISBLANK`, `ISERROR`, `TYPE`, rounding helpers | — |

## Quick Start

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
codcel-calculation-engine = "0.1.9"
```

Use a function directly:

```rust
use codcel_calculation_engine::financial::codcel_npv::codcel_npv;

fn main() {
    let rate = 0.08;
    let cash_flows = vec![-1000.0, 300.0, 400.0, 500.0];
    let npv = codcel_npv(rate, cash_flows).unwrap();
    println!("NPV: {npv:.2}");
}
```

Or use the `_base` modules for higher-level wrappers that work with the engine's `Value` type:

```rust
use codcel_calculation_engine::financial_base::FinancialBase;
```

## Excel Compatibility

- Aims for behavioral fidelity with Excel results, including edge cases
- Handles the 1900 date serial number system, including the Lotus 1-2-3 1900 leap-year bug
- Reproduces Excel's rounding behavior (banker's rounding, standard rounding)
- Covers day-count basis conventions (30/360, actual/actual, etc.) for financial functions

## Cross-Platform Determinism

By default, the engine uses your platform's native math library for transcendental functions (`sin`, `cos`, `exp`, `ln`, etc.). This gives the best performance but results may differ by 1 ULP (Unit in the Last Place) between platforms (e.g., macOS vs Linux) due to differences in their C math library implementations.

To get bit-identical results across all platforms, set the `CODCEL_USE_PORTABLE_MATH` environment variable to `true`. This routes all transcendental math through pure-Rust implementations from the [`libm`](https://crates.io/crates/libm) crate.

```bash
# Run tests with portable math for cross-platform consistency
CODCEL_USE_PORTABLE_MATH=true cargo test
```

| Variable | Default | Description |
|---|---|---|
| `CODCEL_USE_PORTABLE_MATH` | `false` | Use pure-Rust math implementations for cross-platform determinism |

## WebAssembly

The engine builds for `wasm32-unknown-unknown` with no feature flags and no platform shims — there is no filesystem, network, thread, or process usage anywhere in `src/`.

```bash
cargo check --target wasm32-unknown-unknown
```

Three behaviours differ inside a WASM sandbox and are worth knowing before you rely on them:

- **Locale detection is unavailable.** `sys-locale` has no provider on `wasm32-unknown-unknown` and returns nothing, and `std::env::var` always fails, so the `CODCEL_*` overrides listed above are ignored. `ValueFormat` falls back to the values supplied by the caller — pass one explicitly if you need particular separators or a currency symbol.
- **`NOW()` and `TODAY()` require a JavaScript host.** `chrono`'s default `wasmbind` feature resolves the current time through `js_sys::Date`, so these link a JS import. Browser and Node hosts via `wasm-bindgen` satisfy this; a bare runtime such as Wasmtime does not.
- **`RAND()` and `RANDBETWEEN()` are deterministic.** `fastrand` has no entropy source in the sandbox and falls back to a fixed seed, so every instance produces the same sequence. Seed explicitly if you need variation — this matters for any simulation or Monte Carlo work.

The crate exposes no `#[wasm_bindgen]` items and is not built as a `cdylib`; supply your own binding layer for the functions you want to call from JavaScript.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## About Codcel

[Codcel](https://codcel.io) turns Excel spreadsheets into production-ready software — real source code in Rust, Python, Java, C#, TypeScript, Go, Swift, and more, with zero platform lock-in.

This calculation engine is one of several open-source components that power Codcel. Learn more at [codcel.io](https://codcel.io).

## Licensing

Licensed under either of

- MIT License ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)

at your option. There are no field-of-use restrictions and no commercial carve-outs.

This crate is the calculation core of [Codcel](https://codcel.io), a commercial
product. It is published under permissive terms so that anyone — including customers
whose generated code depends on it — can read, audit and verify exactly how every
number is computed. Contributions are welcome, but support is best effort.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for
inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual
licensed as above, without any additional terms or conditions. Contributions require
a Developer Certificate of Origin sign-off — see [CONTRIBUTING.md](CONTRIBUTING.md).
