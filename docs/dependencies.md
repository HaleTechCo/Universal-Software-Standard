# Dependencies

This page gives the purpose of each direct dependency (USS-DEP-011). A new
dependency adds its entry here in the same change. `cargo deny` checks
licences, advisories and sources (`deny.toml`), and `Cargo.lock` pins the
exact versions.

## `tools/uss`

| Dependency       | Kind     | Purpose and reason                                                  |
| ---------------- | -------- | ------------------------------------------------------------------- |
| `clap`           | Optional (feature `cli`) | Parses the command line of `uss`. Writing a parser by hand would give worse help text and error messages. The library builds without it. |
| `jsonschema`     | Normal   | Checks records and results against the JSON Schemas in `schema/`, so each problem names its place in the file. |
| `minijinja`      | Normal   | Renders the audit report from the template in `templates/`. The template, not code, holds the report text. |
| `regex`          | Normal   | Reads rule IDs and rule fields in the Markdown files, and checks ID formats. |
| `serde`          | Normal   | Turns parsed YAML and JSON into typed values.                       |
| `serde-saphyr`   | Normal   | Parses YAML. ADR 0002 records why it was chosen.                    |
| `serde_json`     | Normal   | Reads the JSON Schemas, and writes the catalog as JSON for `uss export` and `uss applicable --json`. |
| `tempfile`       | Development | Makes scratch directories in the tests of `cli/`.                |

## `tools/xtask`

`tools/xtask` has no dependencies. It uses only the Rust standard library, so
`cargo xtask check` builds fast and has nothing to audit.

## Tools that `cargo xtask check` downloads

`cargo xtask check` installs `cargo-deny` under `.tools`. It is a build tool,
not a dependency of the code. The version is the constant
`CARGO_DENY_VERSION` in `tools/xtask/src/main.rs`.
