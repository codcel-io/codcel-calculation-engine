<!--
SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Changelog

## Unreleased

### Breaking

- **`NOW()` and `TODAY()` return local wall-clock time, not UTC.**

  Excel reads the machine's wall clock, and has no timezone concept at all — a
  date serial is a zoneless local reading. Returning `Utc::now()` was a different
  function: at UTC+13 it handed back yesterday's date for thirteen hours of every
  day, and at UTC-8 it rolled over to tomorrow eight hours early.

  The zone comes from `ValueFormat::timezone` when set, otherwise the host's.
  Set `CODCEL_TIMEZONE=UTC` to keep the previous behaviour.

- **`codcel_now` and `codcel_today` take `&ValueFormat`.** They need it to
  resolve the timezone. Callers going through `date_time_base::now` and
  `date_time_base::today` are unaffected — those already took one.

- **`DOLLAR` places the currency symbol the way the locale places it.** German
  now returns `1.234,56 €` rather than `€1.234,56`, and the separator before the
  symbol is a non-breaking space, both matching Excel. The symbol itself still
  comes from `ValueFormat::currency_symbol`; only its position is new.

- **`ValueFormat` gained `region` and `timezone`.** Both default to empty and the
  struct is `#[serde(default)]`, so existing FFI and JNI JSON payloads still
  deserialise. Code constructing `ValueFormat` with a struct literal and no
  `..Default::default()` needs the two new fields.

### Added

- **CLDR locale data** in `codcel_calculation_engine::locale`: the thirteen CLDR
  number symbols, month and weekday names, AM/PM markers, date and time patterns,
  and currency patterns for 77 locales, generated from Unicode CLDR 48.2.1.
  Behind the default `locale-data` feature; turning it off collapses the table to
  `en` and saves about 82 KB of a wasm bundle.

- **Localized `TEXT` output.** `mmmm`, `mmm`, `mmmmm`, `dddd`, `ddd` and `AM/PM`
  now follow the language rather than always returning English. The percent,
  minus, plus, exponent and time separators follow the locale's number symbols.

- **`[$SYMBOL-LCID]` currency prefixes in format codes.** `[$€-407]#,##0.00` used
  to tokenize as a colour code and be discarded, taking the symbol with it.

- **Locale-driven date-code aliases.** German `jjjj-mm-tt`, French `aaaa-mm-jj`
  and Italian `aaaa-mm-gg` are understood, from a generated per-language table,
  replacing three hardcoded languages' worth of `String::replace`. The separator
  canonicalisation that went with it now also covers the `,`-plus-space and
  `.`-plus-`'` conventions, which the previous exact-pair guard skipped entirely.

- **`ExcelError::display_localized`** and `err_to_box_localized`, giving `#WERT!`
  in German and `#VALEUR!` in French. `display()` stays English — it is the wire
  format and `from_legacy_string` round-trips it.

- **`locale::function_name` / `function_name_from`**, a partial Excel function
  name table. Deliberately unwired: a workbook stores formula text in canonical
  English regardless of authoring language, so nothing localized reaches the
  parser. See the note on `locale::function_name`.

- **`CODCEL_REGION` and `CODCEL_TIMEZONE`** environment variables.
  `CODCEL_LANGUAGE` now accepts a full tag, so `en-GB` sets the region too.

- **`CODCEL_MOCK_NOW`** freezes the clock at a fixed RFC 3339 instant, behind the
  default `mock-clock` feature. A workbook with a `TODAY()` cell generates a test
  that otherwise only passes on the day the workbook was saved.

- **`named-timezones` feature** (default on) for IANA zone names. It embeds the
  timezone database, which costs roughly 890 KB of a wasm bundle once anything
  reaches for it, so generated wasm crates are built without it. `NOW` and
  `TODAY` still read the browser's local clock without it.

### Fixed

- The month and weekday name tables were hardcoded English, contradicting the doc
  comment in `text/codcel_text.rs` that promised localized names.
- `mmmmm` returned the first character of the full month name. It now uses CLDR's
  narrow form, which is a digit in Japanese and Chinese and disambiguates months
  that would otherwise share a letter.
