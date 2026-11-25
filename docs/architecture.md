# Architecture

USS has two parts: the standard (files) and `uss` (the tool that reads them).
Agents apply the standard by following the workflows in `AGENTS.md`. They use
`uss` for every calculation.

## Where things live

| Directory      | What it holds                                                      |
| -------------- | ------------------------------------------------------------------ |
| `standard/`    | The rules (Markdown), the registry (`registry.yaml`), the conventions and the vendor profiles. |
| `schema/`      | The JSON Schemas of the record and of the audit results.           |
| `templates/`   | The audit report template, the `AGENTS.md` snippet and the CI workflow for adopters. |
| `skills/`      | The two skills of the Claude plugin: `adopt` and `audit`.          |
| `.claude-plugin/` | The plugin manifest and the marketplace file.                   |
| `tools/uss/`   | The `uss` library and command (a Rust crate).                      |
| `tools/xtask/` | `cargo xtask check`: every check that CI runs.                     |
| `docs/`        | The user documents, the architecture decision records and the design spec. |
| `.github/`     | The CI workflow.                                                   |

## The library and the command

`tools/uss` is one crate with a library and a thin command.

- The library does no file, network or process I/O. It takes its input as
  bytes and values, and it returns values. This keeps it testable and lets a
  later extension embed it, as ADR 0001 explains.
- The command is the module `cli/`. All I/O is in `cli/`: it reads records
  and results, runs `git` for `release-check`, and writes reports. It maps
  each outcome to an exit code. See [cli.md](cli.md).

## The embedded standard

`build.rs` embeds `standard/`, `schema/`, `templates/`, `AGENTS.md` and
`CHANGELOG.md` in the binary. The module `source` gives read-only access to
them through the trait `Source`. A checkout given with `--standard-root` is
another `Source`. The library never reads a path by itself.

## Library modules

- `catalog` loads the registry, the rule files and the vendor profiles. It
  checks them (`uss lint`) and exports the catalog (`uss export`).
- `expr` parses and evaluates the `Applies when` expressions.
- `record` parses and checks the project record and the audit results against
  the JSON Schemas.
- `evaluate` calculates which rules apply, checks the record against the
  catalog and the project's files, and calculates scores and compliance.
- `report` renders the Markdown audit report from an evaluation.
- `release` holds the check for `release-check` that needs no I/O: which
  changed paths break an audit's claim to describe the release commit.
- `yaml` is the single place that calls the YAML crate (ADR 0002).
- `text` normalises input text: it removes a byte order mark and turns CRLF
  into LF.
- `source` gives access to the embedded or checked-out standard.

## Data flow

1. `uss` loads the catalog from a `Source`.
2. It reads the record and checks it against the catalog.
3. `evaluate` calculates the rules that apply. An agent audits them and writes
   a results file.
4. `report` renders the report from the evaluation. `release-check` does the
   same for each record in a repository, after it proves that the audits
   are for the release commit.

## Architecture decisions

- [0001: Write the validator in Rust](adr/0001-validator-in-rust.md)
- [0002: Use serde-saphyr for YAML](adr/0002-yaml-crate.md)
