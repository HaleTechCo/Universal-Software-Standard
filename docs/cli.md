# The `uss` command

`uss` calculates which rules of the Universal Software Standard apply to a
project, checks the project's record, and writes audit reports. It carries the
standard, the schemas, the templates, `AGENTS.md` and `CHANGELOG.md` inside
the binary.

```text
uss --help
```

## Settings

- `uss` reads no environment variable.
- `uss` reads no configuration file.
- Each setting is a command-line option. This page lists each one with its
  default.
- No option turns off or weakens a security feature or a check. The option
  `--standard-root` changes which standard `uss` reads, and the record must
  pin that standard's version.

## Exit codes

| Code | Meaning                                                           |
| ---- | ----------------------------------------------------------------- |
| 0    | Success. The command found no problem.                            |
| 1    | The command found problems, or it could not read or write a file. |
| 2    | The command line is wrong, or `uss` could not write its output.   |

`uss --help` and `uss --version` exit with 0. `uss` with no command prints the
help to standard error and exits with 2.

## Global options

These options belong to `uss` and to every command.

| Option                  | Default                        | Effect                                                          |
| ----------------------- | ------------------------------ | --------------------------------------------------------------- |
| `--standard-root <DIR>` | The standard embedded in `uss` | Use the USS checkout at `DIR` instead of the embedded standard. |
| `-h`, `--help`          | Off                            | Print help.                                                     |
| `-V`, `--version`       | Off                            | Print the version of the `uss` program.                         |

## Commands

### `uss lint`

Check the standard's files for problems. Exit 1 when it finds any. Use it
when you change the standard.

### `uss version`

Print the version of the standard. Compare it with `standard.version` in a
record.

### `uss applicable`

List the rules that apply to a project. The form is `uss applicable <RECORD>`.
`RECORD` is the path of the record, such as `.uss/record.yaml`. It has no
default.

| Option   | Default                                | Effect                            |
| -------- | -------------------------------------- | --------------------------------- |
| `--json` | Off. Output is tab-separated lines.    | Print a JSON array instead.       |
| `--full` | Off. Output is one line for each rule. | Print the full text of each rule. |

The two options cannot be used together. With both, `uss` exits with 2.

### `uss export`

Print the whole rule catalog as JSON, for other tools. The form is
`uss export --json`. The option `--json` is required. Without it, `uss` exits
with 2.

### `uss check`

Check the record against the standard and the project's files. The form is
`uss check <RECORD>`. It prints each problem and a last line, "ok" or the
number of problems. Exit 1 when it finds a problem. The record must pin the
version of the standard that this `uss` carries.

### `uss report`

Write the Markdown audit report for a results file. The form is
`uss report <RESULTS>`. `RESULTS` is the path of the file, such as
`.uss/audits/2026-11-25.yaml`. It has no default.

| Option              | Default                                  | Effect                     |
| ------------------- | ---------------------------------------- | -------------------------- |
| `--record <RECORD>` | `.uss/record.yaml`                       | The project record.        |
| `--out <OUT>`       | `RESULTS` with `.yaml` replaced by `.md` | Where to write the report. |

It prints the compliance statement and the score line. It writes no report
when the record or the results have problems.

### `uss release-check`

Check that the latest audit of each record is for the release commit, and
write the reports. The commit that an audit names must be in the clone and an
ancestor of `HEAD`. Only files under `.uss` may have changed since. The command
runs `git`, so it needs a clone with full history.

| Option                | Default            | Effect                                      |
| --------------------- | ------------------ | ------------------------------------------- |
| `--record <RECORD>`   | `.uss/record.yaml` | The project record.                         |
| `--out <OUT>`         | `uss-report.md`    | Where to write the reports.                 |
| `--require-compliant` | Off                | Exit with 1 when a record is not compliant. |

### `uss show [PATH]`

Print a file that `uss` carries, such as `standard/registry.yaml`. With no
`PATH`, it lists the files. The workflows read the standard and `AGENTS.md`
through this command.

## Versions and the public API

`uss` has the version of the standard that it carries, and `uss version`
prints it. The version follows Semantic Versioning 2.0.0. Section 10 of
`standard/conventions.md` says which changes to the standard each part of the
version marks.

The public API of `uss` is what a user can depend on. It has these parts:

- the commands, the options and the exit codes on this page;
- the JSON that `uss applicable --json` and `uss export --json` print;
- the format of the record, which `schema/record.schema.json` defines;
- the format of the audit results, which `schema/audit-results.schema.json`
  defines.

The Rust library in `tools/uss` is not part of the public API. It is not
published (`publish = false` in `Cargo.toml`). While the version is 0.x, a
release can change the public API (Semantic Versioning 2.0.0, item 4).
