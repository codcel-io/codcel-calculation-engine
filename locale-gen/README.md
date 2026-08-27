# locale-gen

Generates `../src/locale/data.rs`.

This directory is a build-time tool. It is **not** part of the published crate —
`Cargo.toml` publishes with `include = ["src/**/*.rs", …]`, so everything here is
excluded automatically, and the crate has no build script. Generated locale data
therefore has to be committed Rust source.

## Pipeline

```text
Unicode CLDR (pinned tag)  ─┐
                            ├─→ fetch.mjs ─→ cldr/cldr-condensed.json ─┐
excel/excel-locale.json  ───┴───────────────────────────────────────────┴─→ generate.mjs ─→ ../src/locale/data.rs
```

```bash
node fetch.mjs      # ~230 downloads; rewrites cldr/cldr-condensed.json
node generate.mjs   # rewrites ../src/locale/data.rs
```

`fetch.mjs --reuse` recomputes only the alias maps from the checked-in condensed
file, skipping the downloads. Use it when changing alias rules.

Both scripts are plain Node with no dependencies.

## Sources

| Data | Source |
|---|---|
| 13 number symbols, currency and decimal patterns | CLDR `cldr-numbers-full/main/<locale>/numbers.json` |
| Currency symbol per locale | CLDR `cldr-numbers-full/main/<locale>/currencies.json` |
| Region → currency | CLDR `cldr-core/supplemental/currencyData.json` |
| Month names, weekday names, AM/PM, date and time patterns | CLDR `cldr-dates-full/main/<locale>/ca-gregorian.json` |
| Locale id collapsing (`de-DE` → `de`, `zh-TW` → `zh-Hant`) | CLDR `cldr-core/supplemental/likelySubtags.json` |
| Excel date-code letters (`jjjj` → `yyyy`) | `excel/excel-locale.json`, hand-maintained |
| Excel error values (`#WERT!`) | `excel/excel-locale.json`, hand-maintained |
| Excel function names (`WENN`) | `excel/function-names.json`, hand-maintained |

CLDR does not model spreadsheet format codes, error values or function names, so
the last three have no upstream to generate from.

### Function names are partial, on purpose

`excel/function-names.json` covers a common core of 44 functions across eight
languages, not all 500. Microsoft's `FunctionsTranslations` workbook is the only
authoritative source and is not machine-fetchable, and guessing at the rest would
put plausible-looking wrong names in a shipped table.

Extending it is additive: add entries under the language key, keyed by the
canonical English name, and re-run `generate.mjs`. A function with no entry
resolves to its English name — which is what the file format stores anyway — so a
missing entry is a display gap, never a wrong answer.

**Nothing in the transpiler reads this table, and nothing should.** A workbook
stores formula text in canonical English in the sheet XML regardless of the
authoring language; Excel localizes only what it draws on screen. Codcel reads
files rather than keystrokes, so no localized name ever reaches its parser. The
table is for rendering a formula back to a person, and for a runtime formula
parser if one is ever added.

**Pinned CLDR release: `48.2.1`.** It lives in `CLDR_TAG` at the top of
`fetch.mjs`. Bumping it means re-running both scripts and reviewing the diff on
`../src/locale/data.rs` — CLDR does change separators between releases, and that
diff is the only place such a change becomes visible.

## Locale coverage

77 CLDR locales, chosen to cover every language and region subtag `ValueFormat`
already recognises plus the desktop app's ten UI languages. CLDR omits a locale
id that is the default for its language, so there is no `de-DE` entry (`de` is
`de-DE`) and no `pt-BR` (Portuguese defaults to Brazil, and `pt-PT` is the
variant). `generate.mjs` emits alias tables so those tags still resolve.

## LDML → Excel pattern conversion

CLDR writes date patterns in LDML letters, Excel in its own set. `ldmlToExcel` in
`generate.mjs` converts them: `y`/`yy` → `yy`, `yyy+` → `yyyy`, `M` → `m`,
`E`/`EEEE` → `ddd`/`dddd`, `a` → `AM/PM`, single-quoted literals → Excel
double-quoted literals, era and timezone fields dropped.

One asymmetry is worth knowing: LDML lowercase `m` is *minute* and uppercase `M`
is *month*, while Excel spells both `m` and resolves them from position. Both map
to `m`, and `text_function.rs`'s existing month-versus-minute pass sorts them out.

## What is not generated

`ValueFormat`'s three overridable settings — decimal separator, thousands
separator, currency symbol — are **not** taken from CLDR. See the module note at
the top of `../src/locale/mod.rs` for why: CLDR's French grouping separator is
U+202F and its Portuguese currency is the euro, neither of which is what Codcel
has emitted for those locales, and adopting them would change the output of every
generated project on upgrade.
