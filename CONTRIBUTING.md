# Contributing to Codcel Calculation Engine

Thank you for your interest in contributing to the Codcel Calculation Engine.

This repository contains the Rust implementation of Codcel's Excel-compatible calculation engine, including formula behaviour, supporting utilities, and test coverage.

We welcome contributions that improve correctness, compatibility, performance, maintainability, and documentation.

---

## Scope of This Repository

This repository is intended for work related to the Rust calculation engine, including:

- Excel formula implementations
- Formula correctness fixes
- Edge-case handling
- Numerical behaviour improvements
- Date, financial, logical, text, and lookup function support
- Tests and compatibility validation
- Internal engine refactoring
- Developer documentation for the engine

If your issue is about the Codcel product itself rather than this specific Rust engine repository, please use the Codcel contact channels:

- General contact: https://codcel.io/contact
- Product bug reports: https://codcel.io/contact/bugs
- Feature requests: https://codcel.io/contact/features

---

## Before You Start

Before opening a Pull Request, please:

- Check whether a similar issue or pull request already exists
- Keep the change focused and limited in scope
- Prefer behaviour that matches Excel as closely as practical
- Add or update tests for any behaviour change
- Avoid unrelated formatting-only changes in the same PR

---

## Reporting Bugs

If you find a bug in this repository, please open a GitHub issue.

A good bug report includes:

- The Excel function name(s) involved
- The input values used
- The actual result
- The expected result
- Whether the expected result was verified against Excel
- A minimal reproducible example
- Any relevant logs, panic messages, or failing test output

Examples of useful issue titles:

- `PRICE returns incorrect result for basis=3 with irregular dates`
- `XIRR diverges for valid cashflow set`
- `TEXT function formatting mismatch with Excel`

If the problem is in the Codcel application rather than this Rust engine, report it via:

- https://codcel.io/contact/bugs
- bugs@codcel.io

---

## Suggesting Enhancements

Enhancements are welcome.

Examples include:

- New Excel function support
- Better compatibility with Excel edge cases
- Performance improvements
- Refactoring that improves readability or maintainability
- Improved test coverage
- Developer tooling improvements

For broader product ideas, language targets, or commercial feature requests, please use:

- https://codcel.io/contact/features
- features@codcel.io

---

## Contribution Workflow

All changes must come through Pull Requests.

Direct commits to protected branches should not be used except by repository owners when absolutely necessary.

Typical workflow:

1. Fork the repository
2. Create a branch for your change
3. Make your changes
4. Add or update tests
5. Run the test suite
6. Open a Pull Request

Example branch names:

- `fix-price-basis3`
- `add-oddfprice-tests`
- `refactor-date-serial-utils`

---

## Pull Request Guidelines

Please keep Pull Requests focused and clearly explained.

Each Pull Request should ideally include:

- A short summary of the change
- The reason for the change
- The Excel behaviour being matched or improved
- Notes on any edge cases
- Tests added or updated
- Any compatibility considerations

Please avoid mixing multiple unrelated changes into a single PR.

---

## Excel Compatibility Expectations

This project aims to match Excel behaviour as closely as practical.

When contributing formula logic:

- Prefer verified Excel behaviour over assumptions
- Document any known deviations from Excel
- Be careful with floating-point behaviour and rounding
- Consider date systems, basis handling, serial date quirks, and edge cases
- Consider error propagation behaviour
- Preserve backward compatibility where practical unless a bug fix requires otherwise

Where possible, include:

- Example Excel inputs and outputs
- Boundary cases
- Invalid input cases
- Cross-checks against known Excel results

---

## Tests

Tests are required for behaviour changes.

Please add or update tests when you:

- Fix a bug
- Add a formula
- Change formula behaviour
- Refactor logic that could affect results

Where useful, tests should include:

- Standard cases
- Edge cases
- Invalid argument cases
- Excel compatibility cases
- Regression tests for previous bugs

If the repository already has conventions for test placement or naming, follow those conventions.

Before submitting a PR, run:

```bash
cargo test
```

If applicable, also run:

```bash
cargo fmt
cargo clippy --all-targets --all-features
```

Only mention commands that actually exist in the repository workflow. If some are not currently used, keep the wording but do not add CI steps that would fail without setup.

---

## Coding Style

Please follow normal Rust best practices:

- Keep functions focused
- Prefer clear naming over cleverness
- Avoid unnecessary allocations where possible
- Keep public behaviour stable unless intentionally changing it
- Add comments where Excel behaviour is surprising or non-obvious
- Prefer small, reviewable refactors

Use `cargo fmt` formatting conventions.

---

## Documentation

If your change affects public behaviour, update relevant documentation as appropriate.

Examples:

- Supported formula list
- Behaviour notes
- Known limitations
- Examples
- Compatibility notes

If the change is primarily documentation-related, consider whether it belongs in `codcel-docs` instead of this repository.

---

## Confidentiality and Example Files

Do not submit confidential spreadsheets, customer models, or regulated data.

If a spreadsheet example is needed:

- Use anonymized or synthetic data
- Reduce the workbook to the smallest reproducible example
- Remove sensitive business information

---

## Review and Merge Process

All Pull Requests are reviewed by a maintainer.

Maintainers may request changes for:

- correctness
- Excel compatibility
- test coverage
- code clarity
- repository scope

A Pull Request may be rejected if it:

- lacks tests for behavioural changes
- changes unrelated areas unnecessarily
- introduces unclear behaviour differences from Excel
- belongs in another repository

---

## Licensing

By submitting a contribution to this repository, you agree that your contribution will be licensed under the same terms as this project.

See the repository licensing files for details.

---

## Thank You

We appreciate contributions that help improve Excel compatibility, correctness, and developer experience in the Codcel Calculation Engine.
