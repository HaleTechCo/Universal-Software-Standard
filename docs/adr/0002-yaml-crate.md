# 0002: Use serde-saphyr for YAML

* Status: accepted
* Date: 2026-11-22

## Context and Problem Statement

USS reads YAML (`standard/registry.yaml` now; rule front matter, records and
configuration later). ADR 0001 notes that Rust has a weaker YAML ecosystem.
Which YAML crate should the validator use? The crate is called from one place,
`uss::yaml::from_slice`, so it can be replaced later.

## Decision Drivers

* A release within the last 12 months.
* No RustSec advisory.
* Pure safe Rust preferred over transpiled or bound C.
* Deserializes into structs with `deny_unknown_fields` and into
  `serde_json::Value`.
* Builds for `wasm32-unknown-unknown` (the library is embedded in a VS Code
  extension later).

## Considered Options

* `serde-saphyr`
* `serde_norway`
* `serde_yaml`: rejected without evaluation (archived in 2024).
* `serde_yml`: rejected without evaluation. The RustSec database lists an
  advisory for it (https://rustsec.org/packages/serde_yml.html, checked
  2026-11-22).

## Evidence (checked 2026-11-22)

| Criterion | serde-saphyr 2.2.0 | serde_norway 0.9.42 |
| --- | --- | --- |
| Latest release | 2026-11-18; releases since 2025-09, with 1.0.0 on 2026-07-31 and 2.0.0 on 2026-10-13 (https://crates.io/api/v1/crates/serde-saphyr) | 2024-12-21, about 23 months ago, so it fails criterion 1 (https://crates.io/api/v1/crates/serde_norway) |
| RustSec advisory | None (no entry at https://rustsec.org/packages/serde-saphyr.html; no `crates/serde-saphyr` directory in https://github.com/rustsec/advisory-db) | None (no entry at https://rustsec.org/packages/serde_norway.html) |
| Implementation | Pure Rust; `#![forbid(unsafe_code)]` in the crate and in its parser, `granit-parser` | Wraps a Rust port of libyaml; contains `unsafe` code in `src/libyaml/` |
| Strict structs and `serde_json::Value` | Yes, proven by the tests `strict_struct_rejects_unknown_fields` and `parses_into_json_value` in `tools/uss/src/yaml.rs` | Not tested (rejected on the first two criteria) |
| `wasm32-unknown-unknown` | Yes: `cargo build -p uss --lib --no-default-features --target wasm32-unknown-unknown --locked` succeeds | Not tested |

Repository: https://github.com/bourumir-wyngs/serde-saphyr (licence
MIT OR Apache-2.0).

## Decision Outcome

Chosen option: `serde-saphyr`, because it is actively released, has no
advisory, is safe Rust (which fits the workspace's `unsafe_code = "forbid"`
policy) and passes the strict-struct, `Value` and WebAssembly checks.
`serde_norway` has no release in the last 12 months and contains `unsafe` code.

The dependency is declared as `serde-saphyr = "2.2"` (the stable 2.x line, not
the 3.0 release candidate) and `Cargo.lock` records the exact version.

### Consequences

* The crate is young, so `cargo deny` and the lock file stay the guard against
  surprises. It has had a 2.x release line only since 2026-10-13.
* `uss::yaml::from_slice` strips a UTF-8 BOM and converts CRLF to LF before it
  calls the crate, so the choice of crate does not change how input is read.
* Replacing the crate means editing `tools/uss/src/yaml.rs` only.
