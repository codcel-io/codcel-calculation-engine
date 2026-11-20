<!--
SPDX-FileCopyrightText: Copyright (c) 2026 Codcel
SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Changelog

## 0.1.11 (unreleased)

### Fixed

- `+ - * /` on arrays and areas treat a blank as 0, as Excel does, instead of
  failing with "Cannot convert none to number value". Scalars already did this.
- `ROUND` takes the value at Excel's 15 significant digits before rounding, so
  `ROUND(934489.4999999999, 0)` is 934490 and `ROUND(2.675, 2)` is 2.68.
- `ROMAN(0)` returns empty text instead of `#VALUE!`.

### Added

- `Input::clone_sharing_caches`: a copy of the request that shares its caches.
  `Input::clone` still gives empty caches.
- `codcel_cached!`, with `Input::range_cache_get` and `range_cache_insert`, to
  cache a value of any size for the rest of the request.
- The `fill_down_cells` module. A filled-down range such as `$D$9:D9` reads
  each cell once per request instead of re-reading every earlier row.
- `Input::warmed` and `Input::set_warmed`, for generated fill-down warm-up.
- `value::into_area_of_value` and `into_option_area_of_value`, which take
  their value instead of copying it.
- The `profile` module: opt-in counters, enabled by `CODCEL_PROFILE` and
  `CODCEL_PROFILE_KEYS`.
- `Value::into_single_value`, `into_vec_value` and `into_flattened_vec_value`,
  which take their value instead of copying it.
- `Value::for_each_cell` and `count_cells`, which visit the cells of a range in
  place, in the order `area_of_value` gives them.
- `Value::flat_f64`, a range as one flat list of numbers.
- `Value::try_f64`, a number or `None`, without building an error message.

### Changed

- `codcel_cache!` caches errors as well as values, so a failing cell is
  calculated once per request.
- `SUMIFS`, `COUNTIFS`, `AVERAGEIFS`, `MAXIFS` and `MINIFS` combine their
  matches with a binary search.
- `SUM`, `SUMIF` and array flattening no longer copy their ranges.
- Number formatting happens only when a value is needed as text.
- Faster functions over ranges, with the same results:
  - `AVERAGE`, `MAX`, `MIN`, `COUNT`, `COUNTA`, `COUNTBLANK` and `SUMPRODUCT`
    read their ranges in place, and skip blank and text cells without
    building an error message.
  - `COUNTIF`, `SUMIF`, `AVERAGEIF`, the `*IFS` functions and the `D*`
    database functions read their criteria once per call instead of once per
    cell. The `*IFS` functions test each later criterion only on rows that
    met the earlier ones.
  - `INDEX`, `MATCH`, `VLOOKUP`, `HLOOKUP`, `LOOKUP`, `XLOOKUP`, `XMATCH`,
    `FILTER` and the array functions (`SORT`, `SORTBY`, `UNIQUE`,
    `TRANSPOSE`, `TAKE`, `DROP`, `HSTACK`, `VSTACK` and others) no longer
    copy their ranges. `SORT` and `SORTBY` work out each sort key once.
  - `RANK`, `RANK.EQ` and `RANK.AVG` count in one pass instead of sorting.
    `LARGE`, `SMALL` and `MEDIAN` find the value without a full sort.
  - `MODE.SNGL` and `MODE.MULT` no longer turn each number into text.
  - `SEARCH` and `SEARCHB` reuse their pattern, and `DATEVALUE` builds its
    date patterns once.
  - `MID`, `RIGHT` and `REPLACE` no longer copy the text before slicing it.
  - Whole-number and boolean conversions build an error message only when
    they fail.

Code generated for 0.1.11 uses the new API and does not build against 0.1.10.

## 0.1.10

### Breaking

- **Date conversions take `DateSemantics` instead of a `bool`.**
  `date_time_base::excel_to_date_time`, `date_time_base::date_time_to_excel`,
  `date_time_base::force_string_to_date_time`, `codcel_date_value` and
  `codcel_forecast_ets_seasonality` used to take `apply_lotus_bug: bool` (or no
  flag at all, for `codcel_date_value`). They now take a `DateSemantics`, from
  the new `date_system` module. Pass `value_format.date_semantics()` where you
  previously passed `value_format.allow_lotus_1_2_3_1900_date_bug`, or
  `DateSemantics::EXCEL_1900` for a literal `true`.

  A named type rather than a `bool` makes every call site say which convention
  it means. There is deliberately no `From<bool>`. The engine works in the 1900
  serial system only; a 1904-epoch workbook is rebased as it is read, before
  any value reaches this crate, so the 1462-day shift cannot be applied twice.

- **`date_and_time::datetime_to_excel_serial` is removed.** It hardcoded the
  Lotus 1-2-3 leap-year bug. Use `date_time_base::date_time_to_excel` with an
  explicit `DateSemantics`.

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
  deserialise.

  Code constructing a `ValueFormat` with an exhaustive struct literal will not
  compile. Fix it by naming only the fields that differ from the default and
  finishing with `..Default::default()`:

  ```rust
  ValueFormat {
      currency_symbol: String::new(),
      ..Default::default()
  }
  ```

  Prefer that over adding the two fields to the list. `Default` is a plain
  literal, so it stays independent of the host locale and of `CODCEL_*`
  variables, and the call site survives the next field addition.

### Added

- **`ISLOGICAL` and `PERCENTOF`.**

- **`HSTACK` and `VSTACK`** now combine arrays the way Excel does. The previous
  `VSTACK` also printed its input to stdout.

- **`compensated_sum`**: Neumaier summation, now used by `SUM` and the other
  functions that add up a range, so long columns of mixed-magnitude values no
  longer drift from Excel's total.

- **`date_system::DateSemantics`**, with `EXCEL_1900` (the default, matching
  Excel) and `ASTRONOMICAL_1900`. `ValueFormat::date_semantics()` derives one
  from `allow_lotus_1_2_3_1900_date_bug`.

- **`ValueFormat::with_calculation_flags_from`**, which copies the rounding,
  date-system and timezone settings from a project's format onto a format
  derived from a request's language. Every transport that builds a per-request
  format from `Accept-Language` needs it: those settings belong to the
  workbook, not to the reader.

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

### Changed

- **`IRR`, `MIRR`, `NPV`, `ODDFYIELD`, `ODDLYIELD`, `PMT`, `RATE`, `XIRR`, `XNPV`
  and `YIELD`** were reworked. The rate-solving functions share one root finder
  that accepts an answer only when the residual is small, not merely when the
  step is, so they agree with Excel on inputs where Newton's method used to
  stop early or wander.

- **`VLOOKUP` and `HLOOKUP`** follow Excel more closely. Exact-match mode
  supports `*`, `?` and `~` wildcards, matches case-insensitively and treats `2`
  and `2.0` as equal while never matching numbers to text. Approximate mode
  binary-searches a sorted first column and falls back to a full scan when the
  column holds errors, mixed types or empty rows. `#VALUE!`, `#REF!` and `#N/A`
  are returned in the cases Excel returns them.

- **The normal-distribution family shares one implementation.** `NORM.S.DIST`,
  `NORMSDIST`, `NORM.DIST`, `NORMDIST`, `NORM.S.INV`, `NORMSINV`, `NORM.INV`,
  `NORMINV`, `PHI`, `GAUSS`, `LOGNORM.DIST`, `LOGNORMDIST`, `LOGNORM.INV`,
  `LOGINV`, `CONFIDENCE`, `CONFIDENCE.NORM`, `Z.TEST` and `ZTEST` now agree with
  each other to the last bit.

- **The crate denies `clippy::unwrap_used`, `clippy::expect_used` and
  `clippy::panic`**, and production code contains none of them. A malformed
  input now returns an error instead of aborting the host process.

### Fixed

- `DATEDIF` with unit `"MD"` panicked when the end date fell in January and the
  day had to roll back into December.
- `PERCENTILE`, `PERCENTILE.INC`, `PERCENTILE.EXC`, `PERCENTRANK.INC`,
  `PERCENTRANK.EXC`, `QUARTILE.INC`, `QUARTILE.EXC` and the `FORECAST` family
  panicked when the range contained `NaN`, the in-band representation of an
  Excel error. They now sort with a total order.
- The month and weekday name tables were hardcoded English, contradicting the doc
  comment in `text/codcel_text.rs` that promised localized names.
- `mmmmm` returned the first character of the full month name. It now uses CLDR's
  narrow form, which is a digit in Japanese and Chinese and disambiguates months
  that would otherwise share a letter.
