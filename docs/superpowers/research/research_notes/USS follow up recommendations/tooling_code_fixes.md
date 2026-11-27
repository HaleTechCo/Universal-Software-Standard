# Fixes for the `uss` tool failures of the USS self-audit (follow-ups 4.2, 4.3, 4.4, 4.8 and USS-PRF-001)

Read on 2026-11-25. Repository facts come from the files named; local measurements are mine (build `cargo build --release --locked`, PowerShell `Measure-Command`, `.tools/bin/cargo-deny.exe` 0.22.0 run with a scratch config outside the repository; nothing was installed and no repository file was changed). Version and licence numbers come from `gh api -X GET` (GitHub releases, read-only) and `cargo info` (crates.io index) on 2026-11-25.

Sections follow the order requested: code items by risk (4.2), configuration (4.3), scanners (4.4), PRF-001, section 4.8. A cross-cutting design note comes first because six code items share it.

## 0. Cross-cutting design for the code items (UX-011, SEC-014, UX-001, UX-009, OBS-002, I18N-002, QLT-005)

### Takeaway
Build one small output layer in `tools/uss/src/cli/` (a `Ui` that owns `out`, `err` and the verbosity) before the individual fixes. All seven items then become changes to one place: stdout for results, stderr for problems (UX-011), control-character escaping at the sink (SEC-014), checked writes with a stated reason (QLT-005), a quiet/verbose level (OBS-002), `--json` rendering (UX-009), and messages looked up by key (I18N-002, UX-001). Without it the same ~12 `let _ = writeln!` sites and ~16 cli-level `format!` calls get edited several times.

### Cited Findings
- The library, not only the CLI, builds user-facing strings: 211 `format!(` calls in `evaluate/`, `record/`, `release.rs`, `report.rs` and `catalog/` against 16 non-test ones in `cli/`; `evaluate/check.rs` alone holds the check diagnostics — local count with `grep` over [tools/uss/src](../../../../../tools/uss/src).
- The audit's I18N-002 evidence names `cli/files.rs:116`, `cli/mod.rs:193`, `:246` and `evaluate/check.rs:103` as literals; only the report text is outside code (`templates/audit-report.md.j2`) — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml).
- Library has no I/O outside tests today: the only non-test uses of `std::env`/`std::fs` outside `cli/` and `main.rs` are in the `#[cfg(test)]` golden test of `report.rs:243-247` — local grep of [tools/uss/src](../../../../../tools/uss/src).
- clig.dev: primary and machine-readable output to stdout, log messages and errors to stderr; `-q/--quiet` less output; `-d/--debug`; note that `-v` can mean verbose or version; `--json`; "Catch errors and rewrite them for humans" with a next step — [clig.dev](https://clig.dev/) (read 2026-11-25).

### Inferences
- Make library diagnostics structured (`Diagnostic { key, args }` plus a default English `Display`) so that the same value renders as text, as JSON (UX-009) and, later, in another language (I18N-002). Doing this once is cheaper than a text-only fix followed by a second refactor.

### Gaps
- I did not build the refactor. Effort figures below are my estimates from reading the code.

## 1. USS-UX-002 (defect): `uss report --out` overwrites any file, also the results file

### Takeaway
Recommended: refuse to write when the target exists unless `--force` (`-f`) is given, open the file with `OpenOptions::new().write(true).create_new(true)` (atomic, no check-then-write race) when not forced, and refuse even with `--force` when the target is the same file as the results file or the record (compare `std::fs::canonicalize` of both). Apply the same rule to `release-check --out`, which writes a file too. Std only, no new crate. Files: `tools/uss/src/cli/mod.rs` (`run_report` ~l.397, `run_release_check` ~l.543, clap definitions ~l.83 and ~l.94), `docs/cli.md`, `AGENTS.md` workflows and `templates/ci/uss-release.yml` (every place that re-runs `uss report` for the same date must add `--force`), tests in `cli/mod.rs`. Effort S (S-M with the doc/workflow updates and the documented-commands test).

### Cited Findings
- Evidence: `uss report --out keep.txt <results>` replaced a user file; `--out .uss/audits/victim.yaml .uss/audits/victim.yaml` replaced the results file itself; with no `--out` it replaced a hand-written `.md` — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-UX-002).
- Current code calls `std::fs::write(&target, report)` with no check — [tools/uss/src/cli/mod.rs](../../../../../tools/uss/src/cli/mod.rs) lines 397-401.
- Verify accepts "does the action only when an explicit option such as `--force` or `--yes` is given" (or confirmation when interactive) — [standard/core/usability.md or the file holding USS-UX-002](../../../../../standard) (search `### USS-UX-002`).
- clig.dev: confirm dangerous actions; non-interactive use should require a flag such as `-f/--force` — [clig.dev](https://clig.dev/).
- `same-file` 1.0.6 (Unlicense/MIT) detects hard links and symlinks if canonicalize is not enough — `cargo info same-file`, 2026-11-25. Both licences would need adding to the `deny.toml` allow list (it lists MIT, so `MIT`-alternative passes under cargo-deny's OR evaluation; Unlicense alone would not).

### Inferences
- The AGENTS.md "Fix the project" step 7 and the Release audit re-run `uss report` for an existing date, so a pure `--force` rule changes those workflows. Alternative that keeps the workflows unchanged: overwrite without `--force` only when the existing file starts with a marker line that `uss report` writes (for example `<!-- uss-report -->`); refuse otherwise. It protects hand-written files but an auditor could judge it weaker against the Verify wording ("destroys data at once, with no option"). I recommend `--force`.
- Meets the Verify: default run refuses (exit 1, message names the file and says "use --force to replace it"); `--force` is the explicit option.

### Alternatives (one line each)
- Interactive prompt when stdin is a TTY: adds a TTY-detection dependency or `std::io::IsTerminal` (std, fine) but still needs `--force` for scripts; more code.
- Write to a temp file and rename: protects against partial writes, not against overwriting; combine later if wanted.

### Gaps
- Behaviour of `canonicalize` on Windows UNC/verbatim paths was not tested.

## 2. USS-SEC-014 (defect): control characters printed unescaped

### Takeaway
Recommended: one function `escape_control(&str) -> Cow<str>` in the CLI layer that replaces every `char::is_control()` character (C0, DEL, C1 including U+009B) and the bidirectional controls U+202A-202E and U+2066-2069 with a visible `\u{XXXX}` form, applied inside the `Ui` sink to every line that contains text from a record, results file, path, git output or OS error (all of `check` problems, `release-check` file names, `report` problems, error messages). Escape at the sink, not in the library, so the library stays I/O-free and the Markdown report keeps its own `cell` filter. Std only. Files: `tools/uss/src/cli/mod.rs` (new `ui.rs` suggested), tests. Effort S.

### Cited Findings
- Evidence: `uss check` on a record with `deviations[0].document` = `"\e[31mRED\e[0m.md"` printed 2 raw ESC bytes; sources `evaluate/check.rs:133,150,153`, `cli/mod.rs:446-449` — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-SEC-014).
- Verify: data written into "a log, a terminal ..." "encodes or escapes it for that context, through a library or the framework's automatic escaping" — USS-SEC-014 in [standard/core/security.md](../../../../../standard/core/security.md).
- Report cell escaping already exists in `tools/uss/src/report.rs:168-179` — audit evidence above.
- `uss` writes no ANSI colour itself (USS-UX-010 pass) so escaping all control characters loses nothing — audit evidence.

### Inferences
- Verify says "through a library". A 10-line std function is arguably not a library; to be safe, document in the function that it is the escaping routine and name `char::is_control` (std). If an auditor insists on a library, `console::strip_ansi_codes` removes rather than shows and misses C1/bidi, so it is a weaker fix. I could not check how an auditor will read "library".
- Escape `\n` and `\t` inside a single message too (each message is one line) so a file name cannot forge extra output lines (CWE-117).

### Alternatives
- Strip instead of escape (hides the attack, loses information).
- Escape in the library's `Display` impls (breaks the report text and the I/O-free layering).

## 3. USS-UX-011: problems that stop `report` and `release-check` go to stdout

### Takeaway
Recommended: write them to stderr. In `run_report` change the two `writeln!(out, ...)` loops (l.372, l.386) and in `run_release_check` l.539 to the error stream; keep the compliance statement and scores on stdout. Effort S; the tests in `cli/mod.rs` that read stdout for these cases (for example `run_report_rejects_invalid_record`, `run_report_consistency_error`, `release_check_*`) change to stderr. Update `docs/cli.md` ("It prints each problem") only if it names the stream.

### Cited Findings
- Evidence with line numbers — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-UX-011). `uss check` keeps its problem list on stdout and passes the Verify today (its list is its result).
- clig.dev rule on streams — [clig.dev](https://clig.dev/).

### Inferences
- With `--out -` (UX-006) the report itself goes to stdout, so the statement and scores must then go to stderr; handle in the same change.

## 4. USS-UX-001: error messages need a next step

### Takeaway
Recommended: add a next step to every user-visible error in the same pass as I18N-002 (the message catalogue holds "what went wrong" and "what to do" together). Do it key by key: file errors ("cannot read the record: ... Check the path, or pass --record.") and each `check` problem class (for example "decision for X, which does not apply. Remove it from `decisions`."). Effort M (about 160 message strings in `evaluate/check.rs` plus the 16 cli ones; most `check` messages already name the fix). If I18N-002 is deferred, do the cli messages and the ~10 most common `check` problems first (S).

### Cited Findings
- Failing examples: `files.rs:116`, `mod.rs:403`, `check.rs:103`, unknown fact — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-UX-001).
- clig.dev "Catch errors and rewrite them for humans" with an example that includes the fix — [clig.dev](https://clig.dev/).

### Alternatives
- Append a generic "Run `uss --help`" to every error: passes a lazy reading but is the generic text the Verify rejects.

## 5. USS-UX-006: clig.dev (`-o/--output`, `-` for stdin and stdout, support link)

### Takeaway
Recommended: (a) rename `--out` to `--output` with `short = 'o'` and keep `--out` as a hidden alias (`alias = "out"`) so the 0.x public API does not break; (b) accept `-` for the record (`check -`, `--record -`) and for the results file, reading stdin, and `--output -` for stdout (statement and scores then go to stderr); (c) add `after_help` to the top-level command: "Report problems: https://github.com/HaleTechCo/Universal-Software-Standard/issues" (the repository's own URL; read it from `CARGO_PKG_REPOSITORY` so it cannot drift). Files: `tools/uss/src/cli/mod.rs` (clap structs l.26-99), `cli/files.rs` (stdin read), `docs/cli.md` option tables, tests (documented-commands test). Effort M. Record from stdin has no directory, so define the project root as the current directory and say so in `docs/cli.md`.

### Cited Findings
- clig.dev: "If another commonly used command uses a flag name, it's best to follow that existing pattern" with `-o, --output`; "support `-` to read from stdin or write to stdout"; "Provide a support path for feedback and issues" (website or GitHub link in the top-level help) — [clig.dev](https://clig.dev/) read 2026-11-25.
- Evidence: `uss check -` fails and `uss report --out -` wrote a file called `-` — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-UX-006).
- `docs/cli.md` lists `--out <OUT>` for `report` and `release-check` — [docs/cli.md](../../../../../docs/cli.md).

### Gaps
- The Verify checks "each guideline about a feature the command line has"; I checked only the four guidelines the audit named. A fuller pass of clig.dev (for example `--no-input`, `NO_COLOR`, `--version`) was not done.

## 6. USS-UX-009: `--json` for `show`, `check`, `lint`, `report`

### Takeaway
Recommended: one global-per-command `--json` flag with these shapes. `check`/`lint`: `{"ok":bool,"problems":[{"message":"..."}]}` (add `key` and `args` once messages are structured); `report`: `{"statement":"...","compliant":bool,"scores":{...},"report":"path"}`; `show` with no PATH: a JSON array of paths (with a PATH, it prints a file verbatim and is not "data", so no flag needed; the Verify lists "a list, a record, a report"). Use `serde_json` (already a dependency); the exit code stays 0/1/2 with JSON on stdout. Files: `cli/mod.rs`, library `Diagnostic` and `Evaluation` get `Serialize`, `docs/cli.md`, tests that parse the output. Effort M. Reuse the `Ui` layer (section 0).

### Cited Findings
- Evidence: only `applicable` and `export` have `--json` — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-UX-009).
- clig.dev: "Display output as formatted JSON if `--json` is passed" — [clig.dev](https://clig.dev/).
- `docs/cli.md` already says `--json` and `--full` conflict for `applicable` (exit 2) — [docs/cli.md](../../../../../docs/cli.md).

### Alternatives
- `--format json|text` instead of a boolean: more extensible, but differs from the existing `--json` of `applicable`/`export`; inconsistent unless those change too.

## 7. USS-OBS-002: a log-level setting

### Takeaway
Recommended: global `-q/--quiet` and `-v/--verbose` (count, `-vv` for debug) command-line flags, read in `run` at start-up and carried in the `Ui`. Quiet prints only errors; default is today's output; verbose adds progress lines to stderr (which files were read, how many rules apply, which Standard version loaded, which `git` command ran). Keep `docs/cli.md` true: no environment variable is added. Std only; no `log`/`tracing` crates. Files: `cli/mod.rs`, `docs/cli.md` (Settings and Global options), tests. Effort S-M (the work is choosing useful verbose lines).

### Cited Findings
- Verify: the documentation names the setting; two runs with different levels differ; read at start-up — USS-OBS-002 in [standard](../../../../../standard) (search `### USS-OBS-002`).
- Current state: no option; `docs/cli.md` "uss reads no environment variable" — [docs/cli.md](../../../../../docs/cli.md).
- clig.dev: `-q/--quiet`, `-d/--debug`, and a warning that `-v` is ambiguous with version — [clig.dev](https://clig.dev/). `uss` uses `-V` for version, so `-v` is free.
- `clap-verbosity-flag` 3.0.4 (MIT OR Apache-2.0, rust-version 1.74) implements `-v/-q` for the `log` crate; `tracing-subscriber` 0.3.23 (MIT) for tracing — `cargo info`, 2026-11-25.

### Alternatives
- `--log-level <error|warn|info|debug>`: clearer for CI, more typing; can be added later as an alias.
- Environment variable (`USS_LOG`): contradicts the "reads no environment variable" setting, which USS-UX-003 and USS-SEC-002 evidence quotes; would need doc changes.
- `clap-verbosity-flag` + `log` + `env_logger`: extra dependencies for a tool with no logging today.

## 8. USS-I18N-002: messages in a resource file

### Takeaway
Recommended: Fluent. Add `fluent-bundle` 0.16.0 (Apache-2.0 OR MIT) with one embedded file `tools/uss/messages/en-US.ftl`, loaded with `include_str!` (no file I/O, so the library still compiles to wasm32). Keep a `Messages` type in the library; `Diagnostic` becomes `{ id, args }` and the bundle renders it; call `bundle.set_use_isolating(false)` so terminal output has no U+2068/2069 isolation marks. Add a unit test that every key used in code exists in the `.ftl` file and the reverse. Help text stays in clap doc comments only if the audit accepts it as "developer docs"; the Verify lists "help text", so plan to move it into `.ftl` too (clap `about`/`help` take runtime strings). Effort L (about 220 strings, plus structured diagnostics). Files: `tools/uss/Cargo.toml`, new `messages/`, `tools/uss/src/{evaluate,record,catalog,release,report}.rs`, `cli/`, `deny.toml` (licences already allowed: MIT, Apache-2.0, Unicode-3.0), `docs/architecture.md`, `docs/cli.md`.

### Cited Findings
- `fluent-bundle` 0.16.0, `Apache-2.0 OR MIT`, rust-version 1.67; `i18n-embed` 0.16.0, MIT; `rust-i18n` 4.2.4, MIT; `gettext-rs` was not resolvable by my query — `cargo info`, 2026-11-25.
- Dependency tree of `fluent-bundle` 0.16.0 in a scratch crate: about 20 packages (fluent-langneg, fluent-syntax, intl-memoizer, intl_pluralrules, unic-langid, smallvec, rustc-hash, self_cell, thiserror, tinystr, memchr, type-map), all pure Rust, no I/O crates — `cargo tree` in a scratch project outside the repository, 2026-11-25. It pulled `syn` 3.0.10, the same major as the workspace's other crates, so it adds no new `syn` duplicate.
- Verify: user-facing literals in code fail; text of logs and developer-only messages are exempt — USS-I18N-002 in [standard](../../../../../standard).

### Inferences
- Fluent over `rust-i18n`: `rust-i18n` uses a macro and YAML/JSON/TOML files loaded through a global static and generates code at compile time; it fits applications more than a library with typed arguments. Fluent has plurals and grammatical selection, which a later translation needs ("1 problem" / "2 problems" already occurs in the `check` summary line).
- Fluent over `i18n-embed`: `i18n-embed` adds locale detection from the environment, which contradicts "uss reads no environment variable"; the language would instead be selected by an option, which can be added later.
- Fluent over gettext: needs C tooling (`msgfmt`) or `gettext-rs` bindings to libintl, a poor fit for wasm32 and for three CI operating systems.
- Cheaper fallback if Fluent seems too big: a `messages/en.toml` key/value file parsed by `toml` plus `{name}` substitution; it meets the Verify but has no plural rules.

### Gaps
- I could not confirm the wasm32 build of `fluent-bundle`: the `wasm32-unknown-unknown` target is not installed on this machine (build failed with E0463 on `core`), and I installed nothing. The crates are pure Rust, so I expect it to build, but run `cargo build --target wasm32-unknown-unknown` (the existing xtask step) after adding it.
- `fluent-bundle`'s `concurrent::FluentBundle` vs the default (`!Sync`) was not tested; `uss` is single-threaded so the default is enough.

## 9. USS-QLT-005: the ten `let _ = writeln!(err, ...)` lines and the dropped `read_dir` errors

### Takeaway
Recommended: remove the ten sites by routing all error text through the `Ui::problem()` method, which handles the write error in one place with the reason in a comment ("stderr is closed or full: no channel is left to report it; the exit code still reports the failure"), and in `files.rs::collect` return `io::Result` / `Vec` of errors and report them as a "cannot list `<dir>`" problem (or comment the one remaining ignore). That also turns on the baseline lint `clippy::let_underscore_must_use` with no new `#[expect]`. Effort S (after section 0). Local Clippy run with the six baseline lints: 12 distinct sites, all in `tools/uss/src/cli/` — the ten production lines and two in test code (`files.rs:126`, `mod.rs:656`).

### Cited Findings
- Evidence lines — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-QLT-005).
- Local run: `cargo clippy --locked --workspace --all-targets --all-features -- -D clippy::dbg_macro -D clippy::let_underscore_must_use -D clippy::todo -D clippy::undocumented_unsafe_blocks -D clippy::unimplemented -D clippy::unused_result_ok` gave 10 errors for `uss` lib and 12 for the lib tests, all "non-binding `let` on an expression with `#[must_use]` type"; `dbg_macro`, `todo`, `undocumented_unsafe_blocks`, `unimplemented`, `unused_result_ok` found nothing — local, 2026-11-25 (output saved outside the repository).
- Verify allows "handled, passed on or reported, or has a comment ... that says why ignoring it is safe" — USS-QLT-005 in [standard](../../../../../standard).

## 10. USS-SEC-009: `maxLength` in the schemas and a size limit on files

### Takeaway
Recommended: (a) add `maxLength` to each free-text string and `maxItems` to each array in `schema/record.schema.json` and `schema/audit-results.schema.json`; (b) in `cli/files.rs` read through `File::open` + `take(MAX + 1)` and refuse a file larger than 4 MiB (the largest real file, the self-audit results file, is 63 666 bytes, so 4 MiB is 60 times larger) with a message that names the limit; apply to the record, results file and `--standard-root` reads, not to the embedded standard; (c) read from stdin with the same limit (section 5). Effort S-M. Files: both schemas, `cli/files.rs`, `cli/mod.rs:348` (results read), `docs/cli.md`, tests.

Proposed values (measured against the self-audit files, then roughly 4x headroom):

| Field | Now | Proposed |
|---|---|---|
| `project.name` | minLength 1 | maxLength 200 |
| `deviations[].document`, `audits[].results`, `audits[].report`, `suppressions[].file` (paths) | minLength 1 | maxLength 260 |
| `deviations[].approved-by` | minLength 1 | maxLength 200 |
| `auditor` | none | maxLength 100 |
| `suppressions[].rule`, `reason` | minLength 1 | maxLength 200 and 1 000 |
| `results[].evidence` | none | maxLength 10 000 (self-audit maximum is 2 434 characters in one line of 145; second is 1 083) |
| `components[]` (strings) | minLength 1 | maxLength 260 |
| `stack` ids | pattern | maxLength 64 |
| arrays (`deviations`, `audits`, `components`, `suppressions`) | none | `maxItems` 1 000 (`results` is an object: keep to the rule count, about 450 IDs, with `maxProperties` 1 000) |

### Cited Findings
- Evidence: neither schema sets `maxLength`; files read whole; serde-saphyr's default budget (64 MiB scalar bytes, 250 000 nodes, depth 64) is a ceiling, not a per-field check — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-SEC-009).
- My scan of the schemas: 10 string fields in the record schema and 8 in the results schema, none with `maxLength`, no `maxItems` anywhere — local read of [schema/](../../../../../schema).
- Largest `evidence` line in `.uss/audits/2026-11-25.yaml` is 2 434 characters; file 63 666 bytes; `uss applicable --full` output 148 349 bytes — local measurements.

### Inferences
- The limits must be generous because evidence is prose with quoted command output; the evidence limit protects the report renderer (Markdown table cells) more than memory.
- JSON Schema `maxLength` counts characters (code points), not bytes, which suits the validator.

### Gaps
- The numbers are my proposals, no source sets them; the maintainer should confirm 10 000 for evidence.

## 11. USS-ARC-005: allowed module dependencies and no I/O in the library

### Takeaway
Recommended: one file `docs/architecture.deps.toml` (or `tools/uss/architecture.toml`) that lists each library module and the modules it may use, plus a forbidden-API list for non-`cli` modules; one integration test `tools/uss/tests/architecture.rs` that reads `src/**/*.rs`, drops `#[cfg(test)]` modules, extracts `crate::<module>` paths with the `regex` crate (already a dependency; `tempfile` is the only dev-dependency today, so this adds nothing) and fails on a dependency not in the file or a use of `std::fs`, `std::net`, `std::process`, `std::env` or `std::io::stdin/stdout/stderr` outside `cli/` and `main.rs`. `cargo test` runs it in `cargo xtask check` and CI. Effort M. Link the file from `docs/architecture.md`. Seed the file from today's graph (measured with grep): `catalog -> expr, source, text, yaml`; `evaluate -> catalog, record, source`; `expr -> catalog`; `record -> catalog, source`; `report -> catalog, evaluate, record, source, text`; `yaml -> text`; `release`, `source`, `text` -> nothing (all `crate::testfixture` uses are test-only and are excluded).

### Cited Findings
- Verify: a file states the allowed dependencies between modules; a CI job checks it and fails on a new dependency — USS-ARC-005 in [standard](../../../../../standard).
- Evidence: prose boundary in `docs/architecture.md:30-35`; wasm32 build already enforces "library does not name `cli`" (`lib.rs:4-5`, `xtask` l.157-189); no file or test lists module dependencies — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml).
- Current non-test I/O in the library: none (only `report.rs:243-247` in a test) — local grep.

### Alternatives
- `cargo-modules` (a tool that prints the module graph; needs installing under `.tools`; I did not verify its current version) — more machinery than a 100-line test.
- Split the library into `uss-core` (no I/O, no clap) and `uss-cli` crates: Cargo then enforces the I/O boundary through the dependency graph, but the Verify wants a file for module dependencies too; large refactor (L).
- Parse with `syn` instead of `regex`: more exact, adds a heavy dev-dependency.

## 12. USS-TST-002: error paths of `build.rs`, xtask and `main.rs`

### Takeaway
Recommended: (a) `main.rs`: move the exit-code conversion `u8::try_from(code).unwrap_or(2)` into a `fn exit_code(i32) -> ExitCode`-style function in the library (`cli::exit_byte(code) -> u8`) and unit-test it for 0, 1, 2, 256, -1; (b) xtask: make `capture` and `run` take a `Command` builder or test them with a program that does not exist and with `cargo --version` that fails via a bad argument (`cargo` is always there in tests), and add `channel_from("[toolchain]\n")` -> `None`; (c) `build.rs`: move `walk` and the table writer into a small module that the `build.rs` includes with `include!`/`#[path]` and that the crate also compiles under `#[cfg(test)]`, then test it on a temp directory (a missing directory, an empty directory, a non-UTF-8 name). Effort M. Files: `tools/uss/src/main.rs`, `tools/uss/build.rs` (+ new `tools/uss/build_support.rs`), `tools/xtask/src/main.rs`.

### Cited Findings
- Evidence: `build.rs:20-67` has no test; xtask tests cover `toolchain_problem` and normal cases of the parsers only; `main.rs:8` exit-code fallback has no test — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-TST-002); xtask source read in [tools/xtask/src/main.rs](../../../../../tools/xtask/src/main.rs).

### Inferences
- Because the lint profile denies `unwrap_used`, test code relies on `clippy.toml` `allow-unwrap-in-tests` — fine for these tests.

## 13. USS-TST-005: end-to-end tests of the built binary

### Takeaway
Recommended: plain `std::process::Command::new(env!("CARGO_BIN_EXE_uss"))` in `tools/uss/tests/e2e.rs`, with `tempfile` (already a dev-dependency) for scratch projects and a 20-line helper that returns `(code, stdout, stderr)`. One test per documented workflow: README quick start (`applicable .uss/record.yaml`), `check`, `report` (success, refuse-to-overwrite, `--force`), `release-check` (in a temp git repo), `show`, `export --json`, `lint`, and the exit codes in `docs/cli.md`. Cargo builds the binary before integration tests, and `CARGO_BIN_EXE_uss` needs the `cli` feature (the `[[bin]]` has `required-features`; `cargo test --all-features` in xtask satisfies it). Effort M. No new dependency.

### Cited Findings
- `assert_cmd` 2.2.2 (MIT OR Apache-2.0, rust-version 1.85); `snapbox` 1.2.2 (MIT OR Apache-2.0, rust-version 1.85) — `cargo info`, 2026-11-25. I did not measure their transitive trees.
- Verify: end-to-end test drives "the built command" through the steps of the documentation; "a test that calls internal functions in place of the interface does not count" — USS-TST-005 in [standard](../../../../../standard).
- Evidence: no test uses `CARGO_BIN_EXE_uss`; CLI tests call `uss::cli::run` — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml).
- The repository already has a documented-commands test that parses command lines in docs (`cli/mod.rs` tests `doc_command_parser`, `documented_commands_exist`) — [tools/uss/src/cli/mod.rs](../../../../../tools/uss/src/cli/mod.rs).

### Alternatives
- `assert_cmd` + `predicates`: nicer assertions and timeouts, more dependencies for three or four helper functions.
- `snapbox`/`trycmd`: can run the commands in Markdown files as tests, which matches "through the steps in the documentation" best (every fenced command in `docs/cli.md` and `README.md` becomes a test). Cost: the docs would need a `trycmd` block syntax and `.md` outputs to keep in sync; consider after the std version exists.

### Gaps
- I did not check which README steps need `claude` (the plugin steps cannot run in CI); only the `uss` commands can be tested.

## 14. USS-TST-010: property-based tests

### Takeaway
Recommended: `proptest` 1.12.0 (MIT OR Apache-2.0, rust-version 1.88; the project's `rust-version` is 1.100) as a dev-dependency of `tools/uss`. Properties: (1) `expr` parse/print round trip for `Applies when` expressions; (2) the YAML/record parsers never panic on arbitrary UTF-8 and bytes and return `Ok` or a diagnostic; (3) `text` normalisation is idempotent and never leaves CR or a BOM; (4) score calculation: scores lie in 0..=100, `met <= applies`, order of results does not change the score; (5) `escape_control` output has no control character (section 2). Effort M. Files: `tools/uss/Cargo.toml`, tests in the modules named in the audit evidence (`catalog/markdown.rs`, `expr.rs`, `yaml.rs`, `record/mod.rs`, `evaluate/score.rs`).

### Cited Findings
- `proptest` 1.12.0 `MIT OR Apache-2.0`; `quickcheck` 1.1.0 `Unlicense OR MIT` — `cargo info`, 2026-11-25.
- Verify names proptest as an example library — USS-TST-010 in [standard](../../../../../standard).

### Inferences
- `proptest` over `quickcheck`: strategies compose (needed to build records and expressions) and shrinking follows the strategy rather than an `Arbitrary` impl; `quickcheck` is smaller but its `Unlicense OR MIT` licence needs the `MIT` branch of the OR to be accepted by cargo-deny (it is in the allow list, and cargo-deny evaluates SPDX OR expressions).
- proptest writes `proptest-regressions/` files next to the source; commit them and add nothing to `.gitignore`.

## 15. USS-TST-012: fuzzing the parsers with cargo-fuzz on a schedule

### Takeaway
Recommended: `cargo-fuzz` 0.13.2 (Apache-2.0) with a `fuzz/` directory that is its own workspace (so `libfuzzer-sys`, licence `(MIT OR Apache-2.0) AND NCSA`, never enters the main `Cargo.lock` or `cargo deny`) with targets `record` (bytes into the record parser and validator) and `results` (bytes into `parse_results`), seeded from `tools/uss/src/record/testdata/` and `.uss/`. Add a new workflow `.github/workflows/fuzz.yml`: `schedule` weekly (a different slot than the Monday 05:23 CI cron), `workflow_dispatch`, `ubuntu-latest` only, nightly toolchain installed with `rustup toolchain install nightly --profile minimal`, `cargo install cargo-fuzz --locked --version 0.13.2` (or the release binary with its SHA-256), `cargo +nightly fuzz run <target> -- -max_total_time=300` per target, corpus restored/saved with `actions/cache` keyed by target, and `actions/upload-artifact` of `fuzz/artifacts/` on failure. Leave the nightly toolchain out of `rust-toolchain.toml`; `cargo +nightly` selects it only in this workflow, and the xtask toolchain guard is not involved. Add a 30-second smoke run of each target on pull requests that change `record/`, `yaml.rs` or `schema/` (optional). Effort M. Satisfies "CI runs it ... on a schedule; a recent run exists" once the first scheduled run has happened.

### Cited Findings
- `cargo-fuzz` 0.13.2 released 2026-06-09, Apache-2.0; release assets for x86_64 Linux (musl), x86_64 macOS and x86_64 Windows (MSVC) only (no ARM64 asset) — `gh api -X GET repos/rust-fuzz/cargo-fuzz/releases/latest`, 2026-11-25.
- cargo-fuzz requires a nightly compiler because it uses the `-Z` flag for AddressSanitizer; it lists Windows (MSVC AddressSanitizer), x86-64 Linux and macOS — [Rust Fuzz Book, setup](https://rust-fuzz.github.io/book/cargo-fuzz/setup.html) read 2026-11-25. (This contradicts the brief's "Linux and macOS only"; the book lists Windows too. A Windows run is not needed for the Verify.)
- `libfuzzer-sys` 0.4.13, licence `(MIT OR Apache-2.0) AND NCSA` — `cargo info libfuzzer-sys`, 2026-11-25. NCSA is not in `deny.toml`'s allow list; keeping `fuzz/` out of the workspace avoids the change.
- Verify: a fuzz target "that CI runs, for a fixed time on each change or on a schedule ... A recent run exists" — USS-TST-012 in [standard](../../../../../standard).
- Parsers to cover per the audit: `record/mod.rs`, `record/results.rs`, `yaml.rs` — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml).

### Inferences
- A fuzz job on a private repository on the free plan costs Linux minutes only; a 2 x 5 minute weekly run is small. The weekly job also counts as "recent run" evidence for audits within the artifact retention period; keep the run history.
- The fuzz targets must call the same public library functions as the CLI (`parse_results`, the record loader) so the I/O-free library stays the fuzzed unit; no `cli` feature is needed.

### Gaps
- Windows and macOS fuzz runs were not tested; I did not check the cost of the nightly toolchain install on the runner.

## 16. Section 4.3: the six Clippy lints and the cargo-deny baseline keys

### Takeaway
Recommended: add the six lints to `[workspace.lints.clippy]` in `Cargo.toml` as `"deny"`, and replace `deny.toml` with the baseline from `standard/vendors/embarkstudios/cargo-deny.md` (plus `multiple-versions = "deny"` and a `skip` for `syn@2.0.119` with a reason, see QLT-003). I ran the proposed `deny.toml` with the installed cargo-deny 0.22.0 against the repository: `advisories ok, bans ok, licenses ok, sources ok`, exit 0, no warning, so all five baseline keys (`db-urls`, `unmaintained = "all"`, `unused-ignored-advisory = "deny"`, `unused-license-exception = "deny"`, `required-git-spec = "rev"`) are valid and pass. Order: fix the 12 `let _` sites first (section 9), then turn the lints on. Effort S.

### Cited Findings
- The six lints `dbg_macro`, `let_underscore_must_use`, `todo`, `undocumented_unsafe_blocks`, `unimplemented`, `unused_result_ok` and the baseline TOML — [standard/vendors/rust-lang/clippy.md](../../../../../standard/vendors/rust-lang/clippy.md) lines 30-51 and [standard/vendors/embarkstudios/cargo-deny.md](../../../../../standard/vendors/embarkstudios/cargo-deny.md) lines 32-55.
- Current `Cargo.toml`/`deny.toml` lack them — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-QLT-006, USS-RUST-001, USS-RUST-010).
- Clippy run with the six lints at deny: only `let_underscore_must_use` fires (12 sites) — local run, 2026-11-25.
- Scratch `deny.toml` run: `.tools/bin/cargo-deny.exe --config <scratch> check` exit 0 — local run, 2026-11-25 (the scratch file lives outside the repository).

### Gaps
- The run used cargo-deny 0.22.0; I did not re-run with 0.22.1 (not installed, and I installed nothing). 0.22.1's changelog adds `advisories.patched` and fixes; I read no key removals.

## 17. USS-RUST-010: update cargo-deny

### Takeaway
Recommended: change `CARGO_DENY_VERSION` in `tools/xtask/src/main.rs` from `0.22.0` to `0.22.1`. Latest release is 0.22.1, published 2026-11-24, Apache-2.0 (a tool, not a dependency, so the `deny.toml` allow list does not apply; Apache-2.0 is on it anyway). Bumping the constant changes the `hashFiles('tools/xtask/src/main.rs')` cache key in `ci.yml`, so CI reinstalls once. Delete the stale local `.tools/bin/cargo-deny` (or the xtask reinstall check should compare versions: today `binary.exists()` skips the install, so an existing 0.22.0 stays; make xtask run `cargo-deny --version` and reinstall on a mismatch). Effort S.

### Cited Findings
- cargo-deny 0.22.1, published 2026-11-24T16:13, Apache-2.0; assets: aarch64-apple-darwin, aarch64-unknown-linux-musl, x86_64-pc-windows-msvc, x86_64-unknown-linux-musl, each with a `.sha256` file (no x86_64-apple-darwin) — `gh api -X GET repos/EmbarkStudios/cargo-deny/releases/latest`, 2026-11-25.
- 0.22.1 changelog: "Fixed" items for unsound and unmaintained advisories emitted by scope, an exit code that no longer depends on the output format, and workspace dependencies wrongly marked unused with source replacement; "Added" `advisories.patched` — same release body.
- The profile needs `>=0.22.1` — [standard/vendors/embarkstudios/cargo-deny.md](../../../../../standard/vendors/embarkstudios/cargo-deny.md) (Versions), per the audit's USS-RUST-010 evidence.
- xtask installs with `cargo install cargo-deny --locked --version ... --root .tools` and skips when the binary exists — [tools/xtask/src/main.rs](../../../../../tools/xtask/src/main.rs) lines 98-125.

### Alternatives
- Download the prebuilt release and verify its `.sha256` (saves compile time on CI; no Intel macOS asset, so Intel Mac contributors would need `cargo install` anyway). Worth doing only if the scanner downloader of section 21 exists.

## 18. USS-QLT-003: warnings in the CI log

### Takeaway
Recommended, one change per warning:
1. Cargo PATH warning (from `cargo install --root .tools`): in `xtask::deny`, set the child environment `PATH` to `.tools/bin` prepended to the current `PATH` for the `cargo install` command (`Command::env("PATH", ...)` with `std::env::join_paths`). Cargo prints that warning when `<root>/bin` is not on `PATH`. S.
2. Duplicate `syn`: unavoidable for now, because `jsonschema` 0.63.0 -> `strum` 0.28.0 -> `strum_macros` 0.28.0 uses `syn` 2.0.119, while `clap_derive`, `ref-cast-impl`, `serde_derive` and others use `syn` 3.0.10. Set `multiple-versions = "deny"` and a documented `skip = [{ crate = "syn@2.0.119", reason = "..." }]` in `deny.toml`. Verified: cargo-deny 0.22.0 prints no warning with this config (the skip entry matched). Remove the skip when `jsonschema` or `strum_macros` moves to `syn` 3. S.
3. rustup notice "default toolchain implicitly overridden ... by rustup toolchain file": the notice appears because CI runners have a preinstalled default toolchain and the toolchain file overrides it. Try in `ci.yml` after `rustup toolchain install`: `rustup default 1.100.0` (read the channel from the file, or hard-code and keep it checked by the xtask guard). If the notice remains, run the job with `RUSTUP_TOOLCHAIN` set from `rust-toolchain.toml`. I could not reproduce the message locally and found no document that says which setting silences it, so test on a branch run. S.
Total effort S; item 3 is the uncertain one.

### Cited Findings
- Evidence: the three warnings, quoted from run 45650698265 on all three OSes — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-QLT-003).
- `cargo tree -d --locked`: `syn v2.0.119` only via `strum_macros v0.28.0` <- `strum v0.28.0` <- `jsonschema v0.63.0`; `syn v3.0.10` via `clap_derive`, `ref-cast-impl`, `serde_derive` and others — local run, 2026-11-25. The same output shows `getrandom` 0.3.4 (via `ahash`) and 0.4.3 (via dev-dependency `tempfile`); cargo-deny skips dev-dependencies by default and printed no `getrandom` warning in the audit.
- `cargo info`: `strum_macros` 0.28.0 is the latest version (MIT) and `jsonschema` 0.63.0 is the latest — 2026-11-25, so no upgrade removes the duplicate today.
- Scratch cargo-deny run with `multiple-versions = "deny"` and the `skip` entry: exit 0, no output besides the summary — local run, 2026-11-25.
- rustup: a toolchain file is the lowest-priority override after `cargo +toolchain`, `RUSTUP_TOOLCHAIN` and a directory override — [rustup book, overrides](https://rust-lang.github.io/rustup/overrides.html); an open rustup pull request ("Make message about rust-toolchain.toml a warning") shows the message is rustup's own, and I could not confirm whether it merged — [rust-lang/rustup PR 4581](https://gh.r-mj.com/rust-lang/rustup/pull/4581) (a mirror page found by search).
- `rustup 1.29.3` is installed locally — `rustup --version`.

### Gaps
- Which exact rustup setting silences the "implicitly overridden" notice is not confirmed (see item 3). The Cargo PATH warning's trigger is from my knowledge of Cargo behaviour; I did not run `cargo install` (installing is not allowed).

## 19. USS-QLT-001: ShellCheck on `.githooks/` in xtask on three operating systems

### Takeaway
Recommended: a shared "pinned tool" helper in xtask (also used for secrets, section 21): a table `{tool, version, os-arch -> (url, sha256, archive kind, binary path)}`; steps: `curl --fail --location --proto =https --tlsv1.2 -o <file> <url>` (curl and tar ship with Windows 10+, macOS and Linux runners; `C:\Windows\System32\curl.exe` and `tar.exe` exist here), verify the SHA-256 with the `sha2` crate (0.11.2, MIT OR Apache-2.0) before extracting, extract with `tar -xf` (bsdtar reads `.tar.xz`, `.tar.gz` and `.zip`), cache under `.tools/<tool>-<version>/`, then run `shellcheck .githooks/pre-commit`. Pin ShellCheck 0.11.0 (GPL-3.0; a separately run binary, never linked) with these SHA-256 values: linux x86_64 `8c3be12b05d5c177a04c29e3c78ce89ac86f1595681cab149b65b97c4e227198` (.tar.xz), linux aarch64 `12b331c1d2db6b9eb13cfca64306b1b157a86eb69db83023e261eaa7e7c14588`, darwin x86_64 `3c89db4edcab7cf1c27bff178882e0f6f27f7afdf54e859fa041fca10febe4c6`, darwin aarch64 `56affdd8de5527894dca6dc3d7e0a99a873b0f004d7aabc30ae407d3f48b0a79`, Windows `shellcheck-v0.11.0.zip` `8a4e35ab0b331c85d73567b12f2a444df187f483e5079ceffa6bda1faa2e740e`. Add a unit test for the hash check (good hash, bad hash) and for the asset table (every supported `(OS, arch)` has an entry). Effort M. Files: `tools/xtask/Cargo.toml`, `tools/xtask/src/main.rs` (or split into `tools.rs`), CI cache key (already hashes the xtask source), `docs`/`CONTRIBUTING.md` where checks are listed.

### Cited Findings
- ShellCheck v0.11.0 (released 2025-08-04), GPL-3.0; assets for Linux x86_64, aarch64, armv6hf, riscv64, macOS x86_64 and aarch64 (.tar.gz and .tar.xz each), plus `shellcheck-v0.11.0.zip` (the Windows build); the release publishes no separate checksum file, but the GitHub API gives a `digest` (sha256) for every asset — `gh api -X GET repos/koalaman/shellcheck/releases/latest`, 2026-11-25. The digests above are those API values; I did not download or hash the files.
- Audit evidence: `.githooks/pre-commit:1-6` is POSIX shell with no linter in CI — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-QLT-001); the only shell file tracked is `.githooks/pre-commit` (`git ls-files`).
- `sha2` 0.11.2 and `ureq` 3.5.2, both `MIT OR Apache-2.0` — `cargo info`, 2026-11-25.

### Inferences
- Use `curl` rather than an HTTP crate: `ureq` brings a TLS stack whose licences (for example `ring`) are not on the allow list, and I did not verify its tree. The helper must set `--proto =https` and check the digest even though the URL is pinned, because the hash is the real control (GitHub asset URLs redirect).
- The pre-commit hook has no `.sh` extension; run `shellcheck --shell=sh` explicitly. ShellCheck on Windows reads the file as given; test the line endings (`.gitattributes` should force LF).
- Alternative without any download: skip ShellCheck and convert the hook into `cargo xtask pre-commit` (Rust), leaving a one-line `.githooks/pre-commit` (`exec cargo xtask pre-commit`). The one-line shell wrapper still counts as shell code for QLT-001 ("for each language the project's own code uses"), so a linter would still be needed. Not recommended on its own.

### Gaps
- Windows ARM64: ShellCheck publishes no ARM64 Windows build in this release; the x86_64 build runs under emulation on Windows ARM. Not tested.
- Whether to also verify a signature: ShellCheck publishes none; the pinned digest is the only integrity check.

## 20. USS-CTB-001: one setup command

### Takeaway
Recommended: add `cargo xtask setup` that (1) runs `rustup toolchain install` is *not* done by xtask (xtask already runs inside the pinned toolchain, because `rust-toolchain.toml` makes `cargo xtask` start rustup's install on first use), so setup (1) sets `git config core.hooksPath .githooks`, (2) installs the pinned tools under `.tools` (cargo-deny, ShellCheck, secret scanner), (3) runs `cargo build --locked` to prefetch dependencies, and (4) prints what it did. Make `.cargo/config.toml` hold `[alias] xtask = "run --package xtask --"` if it does not exist (check), and change CONTRIBUTING.md/AGENTS.md "Set up" from five steps to two: install rustup, then `cargo xtask setup`. Also let `cargo xtask check` call the hook setup when `core.hooksPath` is unset? Not recommended: a check should not change git configuration. Effort S. Add a unit test for the argument parsing and an xtask test that `setup` is documented in CONTRIBUTING.md.

### Cited Findings
- Verify: one command (or a dev container) sets up a complete environment; afterwards build and tests run with no other step; a guide with several manual steps fails — USS-CTB-001 in [standard](../../../../../standard).
- Evidence: five steps listed in `CONTRIBUTING.md:10-18` and `AGENTS.md:428-436`; `cargo xtask check` does not set the hook (`main.rs:127-213`); no dev container — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml).
- xtask currently accepts only the argument `check` — [tools/xtask/src/main.rs](../../../../../tools/xtask/src/main.rs) `main`.

### Inferences
- The Verify counts installing rustup itself as outside "the command" in practice (any one command needs a Rust toolchain). The audit evidence treated `rustup toolchain install` as a step; since `cargo` under rustup installs the pinned toolchain on first use from `rust-toolchain.toml`, `cargo xtask setup` covers it. I did not test the automatic install in a clean environment.
- Alternative: a dev container (`.devcontainer/devcontainer.json`) with the Rust image and `postCreateCommand: cargo xtask setup`; satisfies the second branch of the Verify but needs a container runtime and was not requested.

### Gaps
- Whether `.cargo/config.toml` already defines the `xtask` alias was not checked (the audit says `cargo xtask check` works, so one exists).

## 21. Section 4.4: secret scanners (USS-SEC-004 and USS-SEC-005) and how xtask downloads and verifies a pinned binary

### Takeaway
Recommended: Gitleaks 8.30.1 (MIT), pinned, run from xtask as two steps: `gitleaks git --redact --no-banner` over the full history and `gitleaks dir --redact` over the working tree, with exit code 1 on a finding; CI calls it through `cargo xtask check` on every pull request and push (USS-SEC-005), and one run of the same command on the audited commit is the evidence for USS-SEC-004. Suppress findings only through `.gitleaksignore` or an allowlist in `.gitleaks.toml` with a comment per entry (the Verify wants a recorded reason). Use the helper from section 19 with the per-OS table below. Effort M (about S on top of the ShellCheck helper).

Comparison (GitHub API, 2026-11-25):

| Tool | Version, date | Licence | Assets (Linux/Windows/macOS, x64+ARM64) | Integrity on the release |
|---|---|---|---|---|
| gitleaks | v8.30.1, 2026-03-21 | MIT | linux x64/arm64 (+x32, armv6, armv7), windows x64/arm64 (+x32), darwin x64/arm64 | `gitleaks_8.30.1_checksums.txt` (no signature); API `digest` per asset |
| trufflehog | v3.100.1, 2026-11-23 | AGPL-3.0 | linux amd64/arm64, windows amd64/arm64, darwin amd64/arm64 | checksums file plus `.sig` and `.pem` (cosign keyless); API `digest` |
| betterleaks | v1.10.1, 2026-11-12 | MIT | linux x64/arm64, windows x64/arm64, darwin x64/arm64 | `checksums.txt` plus `checksums.txt.sigstore.json`; API `digest` |

Why gitleaks: MIT, all six OS/architecture pairs, the widest use (31 344 GitHub stars versus 3 118 for betterleaks), a stable CLI with `git` (history) and `dir` (tree) modes. Betterleaks is by the same author, MIT, created 2026-02-03, signs its checksums, and claims better detection and validation; it is young (its README says `main` is the v2 branch and package managers still ship v1), so it is the first alternative to re-evaluate. TruffleHog is AGPL-3.0: acceptable as a separately run CI binary but a licence hazard for anyone who embeds it, and its active-credential verification makes network calls (against the "no secrets leave the machine" intent).

Pinned download table for gitleaks 8.30.1 (the file `gitleaks_8.30.1_checksums.txt` has the SHA-256 for each asset; copy the values into the xtask table when implementing; I list the names only, since I did not fetch the checksum file): `gitleaks_8.30.1_linux_x64.tar.gz`, `..._linux_arm64.tar.gz`, `..._darwin_x64.tar.gz`, `..._darwin_arm64.tar.gz`, `..._windows_x64.zip`, `..._windows_arm64.zip`. The asset `gitleaks_8.30.1_checksums.txt` itself has the digest `061476c21adaf5441516f96f185c1a4706a83cd6329b9b38762271b3d4a52fae`.

### Cited Findings
- Release data (version, date, licence, asset names, digests) — `gh api -X GET repos/gitleaks/gitleaks/releases/latest`, `.../trufflesecurity/trufflehog/...`, `.../betterleaks/betterleaks/...`, 2026-11-25. Stars and creation date from `gh api -X GET repos/<owner>/<repo>`.
- Gitleaks commands and options: `gitleaks git -v`, `gitleaks dir -v path`, `--baseline-path`, `--exit-code` (default 1), `--gitleaks-ignore-path` (`.gitleaksignore`), `gitleaks:allow` comments, `--ignore-gitleaks-allow` — [gitleaks README](https://github.com/gitleaks/gitleaks#readme) read via `gh api` 2026-11-25.
- Betterleaks: "configurable, fast, and thorough secrets scanner ... maintained by the folks who made Gitleaks, including the original author"; features include secrets validation by HTTP requests and BPE filtering; `main` is v2 development, v1 on `v1.x` — [betterleaks README](https://github.com/betterleaks/betterleaks#readme) read 2026-11-25.
- Betterleaks digests: linux_x64 `c174b42d7aa60caf58c0c3e555c0e4bed01fe6b0d99a412bce777b5af4ce913e`, darwin_arm64 `ca91eb3b006f15478411f235b5e2712fb15cea74f6701ebadc2364efe0af1403`, windows_x64 `53d4b37635a999992fb8e1d79f8215279315cbb3b4139314289b69407e79143d` — GitHub API 2026-11-25.
- Verify (SEC-004): a secret scan of the working tree and full history finds no secret; suppressions need a recorded reason; (SEC-005): CI runs a scanner on each PR and push, a finding fails the check — [standard](../../../../../standard) (USS-SEC-004, USS-SEC-005). Audit evidence: SEC-004 unverified (no scanner installed), SEC-005 fail — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml).
- Hosting alternative: GitHub secret scanning is off and returns 404 on this private repository (needs a paid plan or a public repository) — audit evidence for USS-SEC-004.
- xtask helper design: `curl` and `tar` exist on this Windows machine (`C:\Windows\System32\curl.exe`, `tar.exe`); `sha2` 0.11.2, MIT OR Apache-2.0 — local check and `cargo info`.

### Inferences
- Trust model: pin the version and the SHA-256 of each asset in source (reviewed in a pull request), not the checksum file downloaded at run time (an attacker who replaces the asset can replace the file). Signatures (trufflehog, betterleaks) add provenance at update time: verify them once when bumping the pin, not on every run (that needs `cosign`).
- Dependabot cannot update these pins; add a calendar item or a scheduled workflow that opens an issue when a newer release exists (the weekly CI run could print "new gitleaks release").
- Scanning the history on every pull request is cheap for a repository of 150 commits; `fetch-depth: 0` is needed in `ci.yml` (currently the default shallow checkout would scan one commit, so USS-SEC-004's "full history" would pass vacuously). Add `fetch-depth: 0` to the checkout step.
- The first scan may flag test fixtures or the rule text that discusses secrets (the standard quotes secret patterns); expect to write a small `.gitleaks.toml` allowlist with reasons.

### Gaps
- I did not run any scanner, so I do not know whether it reports findings in this repository's history. I also did not download the gitleaks checksum file, so its per-asset hashes are not copied here.
- Betterleaks detection quality against gitleaks was not compared by any source I read; its claims are the vendor's.

## 22. USS-SEC-028: static analysis of Rust code in CI

### Takeaway
Recommended: make the repository public and enable CodeQL for Rust with a pinned `github/codeql-action` workflow (code scanning is free on public repositories; the SAST check then runs on each pull request and push, results appear in the Security tab, and the OpenSSF Scorecard SAST probe recognises it). That depends on the section 2.2 visibility decision. If the repository stays private, no free CodeQL path exists, and the practical choice is a standard ruling (see below) plus an open-source Semgrep-class tool. Effort M (workflow file) plus the visibility decision.

### Cited Findings
- CodeQL supports Rust: editions 2021 and 2024, needs `rustup` and `cargo`, no nightly features, analyses `.rs` and `Cargo.toml`, library models for actix-web, axum, rocket, warp, reqwest, sqlx, tokio — [CodeQL supported languages](https://codeql.github.com/docs/codeql-overview/supported-languages-and-frameworks/) read 2026-11-25. The page gives no GA/beta label.
- CodeQL CLI licence without a GitHub Advanced Security licence: only academic research, demonstrations, testing queries, analysis of open-source code, and generating databases for CI/CD "only for open source code hosted and maintained on GitHub.com"; private repositories are excluded — [codeql-cli-binaries LICENSE.md](https://github.com/github/codeql-cli-binaries/blob/main/LICENSE.md) read 2026-11-25. So CodeQL cannot be run locally or in CI on a private repository without a paid licence, and it cannot be a pinned binary in `cargo xtask check` for private code.
- The audit shows code scanning returns HTTP 403 "Advanced Security must be enabled" on this repository — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-SEC-028).
- Semgrep: Rust is generally available in Semgrep Code with cross-function dataflow and 40+ Pro rules; the page does not say what the Community Edition offers for Rust — [Semgrep supported languages](https://docs.semgrep.dev/supported-languages) read 2026-11-25.
- The open rules repository has `rust/lang/security` with 10 YAML rules (args, args-os, current-exe, insecure-hashes, reqwest-accept-invalid, reqwest-set-sensitive, rustls-dangerous, ssl-verify-none, temp-dir, unsafe-usage) — `gh api -X GET repos/semgrep/semgrep-rules/contents/rust/lang/security`, 2026-11-25; the repository's licence reads `NOASSERTION` in the API (it carries a custom Semgrep Rules licence, which I did not read), and the Semgrep engine is LGPL-2.1, latest release v1.187.0 — `gh api` 2026-11-25.
- OpenGrep (a community fork of Semgrep, LGPL-2.1) v2.1.0 has signed binaries (`.sig` and `.cert` per asset) for Linux x86 and aarch64, macOS x86 and arm64, and Windows x86 — `gh api -X GET repos/opengrep/opengrep/releases/latest`, 2026-11-25.
- Verify for SEC-028: CI runs a SAST tool on each PR and push; a finding fails the check; suppressions carry a reason — [standard/core/security.md](../../../../../standard/core/security.md). The Clippy vendor profile maps no rule to SEC-028; the profiles that do map it are Ruff (the `S` Bandit rules), golangci-lint (`gosec`), SpotBugs with Find Security Bugs, and ESLint (explicitly "not the SAST tool") — [standard/vendors](../../../../../standard/vendors) (grep `USS-SEC-028`).
- OpenSSF Scorecard's SAST check accepts `github/codeql-action/analyze`, Snyk, Pysa, Qodana or SonarCloud — [standard/vendors/ossf/scorecard.md](../../../../../standard/vendors/ossf/scorecard.md) line 110.

### Inferences
- The Verify as written does not accept Clippy as the SAST tool, because the profile maps no rule and the Plan 4 ruling only extended the Bandit-class rule sets (Ruff S, gosec, Find Security Bugs). There is precedent for a ruling: add to `clippy.md` a security lint set (for example `indexing_slicing`, `arithmetic_side_effects`, `string_slice`, `mem_forget`, `lossy_float_literal`, `exit`) and declare Clippy with that set to be the SAST tool of USS-SEC-028 for Rust, as Ruff `S` and `gosec` are. That is a choice about the standard (the maintainer's), not an engineering fix, and I did not validate any lint list. It would also give the Rust module a real answer for every Rust project, which matters for USS as a standard.
- Semgrep/OpenGrep with the 10 open Rust rules covers little (it flags `args`, `unsafe`, `temp_dir`), and this tool's code uses none of the risky APIs; it would pass but prove little. Semgrep CE on Windows has no native binary release (only pip), so it cannot be a pinned `xtask` download on all three operating systems; OpenGrep has a Windows x86 zip but I did not test it. Therefore run it in CI only (ubuntu), not in `cargo xtask check` on every OS.
- `cargo-audit`, `cargo-deny` and `cargo-geiger` are dependency or `unsafe` scanners, not SAST; I found no source that treats them as SAST for this Verify.

### Alternatives (one line each)
- Semgrep CE or OpenGrep in an ubuntu CI job, `--error` on findings, rules pinned by commit: works on a private repository; weak Rust coverage; custom rule licence needs reading first.
- SonarCloud: accepted by Scorecard; free only for public projects; account and token needed.
- Paid GitHub Advanced Security (Code Security) on the private repository: unlocks CodeQL and the other items of section 2.2.
- Clippy-as-SAST ruling in the standard (see Inferences).

### Gaps
- I could not confirm whether CodeQL's Rust support is GA or beta (the pages I read do not label it) nor its default-setup behaviour for Rust on a public repository.
- I did not read the Semgrep Rules licence text, and no source states what Community Edition covers for Rust beyond the open rules directory.

## 23. USS-PRF-001: a performance budget from a measurement

### Takeaway
Recommended budget (write it in `docs/performance.md`, one short document with the measure, the limit and the method; and check it in the CI job of USS-PRF-002 later): "`uss check .uss/record.yaml` on this repository, release build, warm file cache: median of 20 runs at most 150 ms; first run after a build at most 1 000 ms. Memory is not budgeted." Measured on 2026-11-25 on this Windows 11 machine with `cargo build --release --locked` (binary 10.6 MB) and PowerShell `Measure-Command` (which includes the PowerShell process-launch overhead of about 10-20 ms): `uss check .uss/record.yaml`: first run 472 ms (cold), then 32-37 ms (n=7: 33.2, 37.2, 34.6, 32.4, 37.2, 31.8, 35.3); `uss version` 12-22 ms (median about 12 ms, the process-start floor); `uss applicable .uss/record.yaml` 32-41 ms; `uss lint` 29-38 ms. The 150 ms limit is about four times the warm median, so shared CI runners do not flake, and still catches a tenfold regression. Effort S to write, M to enforce (PRF-002 needs the CI job).

### Cited Findings
- Verify: for a command-line tool, "the start-up time" with a number and a unit, and how it is measured — USS-PRF-001 in [standard](../../../../../standard); audit evidence: no budget exists — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml). The audit's earlier measurement on a debug build was 158 ms for `uss check` (USS-PRF-005 evidence) in the same file.
- Local measurements above, `target/release/uss.exe` in the repository checkout, 2026-11-25; PowerShell `Measure-Command`, no hyperfine installed or used.

### Inferences
- A budget of 150 ms in CI on `ubuntu-latest` is probably safe, since Linux process start is faster than Windows, but the CI machines were not measured. Take the first 10 CI numbers before fixing the limit.
- PRF-002 passes today only because no budget exists (the self-audit says so); once this budget exists, PRF-002 fails until a CI job measures it. Plan the xtask step `cargo xtask bench` (build release, run `uss check` 20 times with `std::time::Instant`, fail above the limit) in the same change; it needs no external tool and runs on all three operating systems. Do not put it in the pull-request path if it flakes; the Verify allows a schedule for slow benchmarks.
- Size is a second candidate measure for a CLI: the release binary is 10.6 MB (embedded standard); a limit of 15 MB would be a reasonable second budget, not needed for the Verify.

### Gaps
- Only one machine (Windows 11) was measured; no Linux or macOS numbers.
- I did not take memory measurements.

## 24. Section 4.8: a filter for `uss applicable --full` and results files in `uss check`

### Takeaway
Recommended filter shape: repeatable options on `uss applicable`, valid with the text output and with `--json`, combined with AND between kinds and OR within a kind: `--area <PREFIX>` (the ID prefix, such as `SEC`, case-insensitive; repeatable), `--level <MUST|MUST-NOT|SHOULD|SHOULD-NOT|MAY>` (repeatable), `--id <RULE-ID>` (repeatable; exact; the way an agent re-reads one rule), and `--category <NAME>` if useful. The workflow in `AGENTS.md` ("Read the rules one area at a time") then becomes `uss applicable --full --area SEC .uss/record.yaml`, which makes the agent's save-to-file step unnecessary. Unknown area or rule ID gives exit 2 with the list of valid values (a next step, UX-001). Filtering is a pure function over the rule list, so it lives in the library (`evaluate`) and is unit-tested and property-tested there. Effort S-M. Files: `tools/uss/src/cli/mod.rs` (clap `Applicable` struct near l.50-70), `tools/uss/src/evaluate/mod.rs`, `docs/cli.md` (Commands table), `AGENTS.md` workflows (Before every workflow, steps 7 and 9; Questions to the developer; Judge each rule), plugin skills that read the output (`plugin_matches_standard` test checks the plugin against the standard), tests.

`uss check` and results files: recommended yes, as an opt-in rather than a silent change: `uss check <RECORD>` also validates every results file that the record's `audits[].results` lists, for schema validity and the `commit`/rule-ID checks that need no evaluation (the checks that `uss report` runs), because `uss release-check` already examines the latest audit and `uss check` is the thing `AGENTS.md` tells the agent to run before commit. Report problems with the results file name as a prefix. Since the "Finish the audit" workflow runs `uss check` before `uss report`, the order stays valid. Make the check apply only to the latest audit by default and to all with `--all-audits`, because old results files cannot be edited after a standard upgrade (a results file of an earlier version has a different pinned version). Effort S-M. Files: `tools/uss/src/cli/mod.rs` (`run_check`), `evaluate/check.rs` or `record/results.rs`, `docs/cli.md`, `AGENTS.md` ("Finish the audit" step 4 note "uss check does not examine the content of the results file" must be removed), tests.

### Cited Findings
- `uss applicable --full` output for this repository is 148 349 bytes — `uss.exe applicable --full .uss/record.yaml | wc -c`, local, 2026-11-25; follow-ups 4.8 says 148 KB — [docs/superpowers/follow-ups.md](../../../../../docs/superpowers/follow-ups.md).
- The area is the ID prefix (`SEC` in `USS-SEC-013`) and the output carries `Level`, `Applies when`, `Category`, `File` fields per rule — [AGENTS.md](../../../../../AGENTS.md) "Questions to the developer" and local output of `--full`. The one-line output has tab-separated ID, level, category, title.
- Current docs: `applicable` options `--json` and `--full`, which conflict (exit 2) — [docs/cli.md](../../../../../docs/cli.md).
- `AGENTS.md` "Finish the audit": step 4 says "`uss check` does not examine the content of the results file" and step 5 `uss report` examines it — [AGENTS.md](../../../../../AGENTS.md).

### Inferences
- `--full` plus `--json` is rejected today; with the filter, `--json` can carry the full text once UX-009 is done, so the conflict can go away later.
- Prefix matching is ambiguous when one prefix starts another (for example `SEC` and `SECR` if such an area existed); match the area as the full segment between the first and second hyphen. The IDs match `^USS-[A-Z][A-Z0-9]{1,7}-\d{3}$` in the record schema, so the segment is well defined.

### Gaps
- I did not count the size of one area's output; the largest area was not measured.
