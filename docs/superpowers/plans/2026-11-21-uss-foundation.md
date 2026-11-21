# USS Foundation Implementation Plan (Plan 1 of 5)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the repository skeleton, the machine-readable framework of the
standard (registry, fact vocabulary, schemas, templates, conventions) and the
`uss` validator, a Rust library plus CLI with all its commands, so rule
content can be written and checked from Plan 2 on.

**Architecture:** A Cargo workspace at the repository root with two members:
`tools/uss` (the `uss` library and CLI) and `tools/xtask` (`cargo xtask check`,
every check in one command). The **library does no I/O**. It works on file
contents handed to it through two small traits (`Source` for the standard's
own files, `ProjectFiles` for an adopting project's files), so it can later run
inside a VS Code extension through WebAssembly. All filesystem and `git` work
lives in the `cli` module, behind the default `cli` feature. A build script
embeds `standard/`, `schema/` and `templates/` into the binary, so the binary's
version is always the standard's version. The Markdown parser is a strict
line-based parser for USS's own fixed format.

**Tech Stack:** Rust 1.100.0 (installed; edition 2024). Crates: `serde` +
`serde_json`, one YAML crate chosen in Task 2, `jsonschema` (default features
off), `regex`, `minijinja`, `clap` (only with the `cli` feature); dev:
`tempfile`. Tooling: rustfmt, Clippy, cargo-deny (installed project-locally),
GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-11-21-universal-software-standard-design.md`

**Roadmap** (each later plan is written when the previous one is done; Plan 2's
review calibrates the content plans):

1. **This plan:** foundation and the `uss` tool (spec §13 step 1).
2. Security, Supply chain and Dependencies areas, then **stop for the user's
   review**.
3. Remaining core areas, in batches.
4. Modules, vendor profiles and regulation modules, plus validator support for
   the profile and regulation mapping format.
5. AGENTS.md workflows, Claude plugin and skills, adopter templates
   (`AGENTS.snippet.md`, `templates/ci/`), CODEOWNERS, community files
   (CONTRIBUTING, SECURITY, CODE_OF_CONDUCT), CHANGELOG, release binaries,
   this repo's `.uss/` self-audit, release 0.1.0.

Later, outside these five: the VS Code extension built on the `uss` library.

## Before you start: toolchain check

Builds MUST run on the **rustup-managed** toolchain pinned in
`rust-toolchain.toml`. A standalone Rust install earlier on `PATH` silently
ignores that file. On the user's machine, standalone winget installs (GNU,
GNU-LLVM, MSVC) currently come before rustup on `PATH`, so plain `cargo`
runs the GNU-LLVM toolchain. Before Task 1, and again after Task 1 creates
`rust-toolchain.toml`, run:

```
rustc --print sysroot
rustc -vV
```

Expected: the sysroot is under `.rustup\toolchains\` (or `.rustup/toolchains/`),
and `rustc -vV` shows `release: 1.100.0`. If not, **stop and ask the user** to
put `%USERPROFILE%\.cargo\bin` first on `PATH` or remove the standalone
installs. Do not change `PATH` or uninstall anything yourself.

## Global Constraints

- Toolchain: only the rustup-managed toolchain from `rust-toolchain.toml`.
  `cargo xtask check` refuses to run on any other (Task 15).
- Keywords: exactly `MUST`, `MUST NOT`, `SHOULD`, `SHOULD NOT`, `MAY` (RFC 2119
  as clarified by RFC 8174). `SHALL`, `SHALL NOT`, `REQUIRED`, `RECOMMENDED`,
  `NOT RECOMMENDED`, `OPTIONAL` are never used. One rule, one level.
- Rule IDs: `USS-<PREFIX>-<NNN>`, prefix `^[A-Z][A-Z0-9]{1,7}$`, three digits.
  Never reused or renumbered; retired in place.
- Audit outcomes: exactly `pass`, `fail`, `deviation`, `declined`,
  `unverified`, `not-applicable`.
- Score categories, in this order: Safety, Reliability, Efficiency,
  Maintainability, Contributability, Compatibility, Fairness,
  Accessibility & Usability, Features.
- Score per category = floor(passed × 100 ÷ applicable); `n/a` when nothing
  applies; equal weight per rule; **no overall score**.
- Compliant = every applicable MUST/MUST NOT `pass`; every applicable
  SHOULD/SHOULD NOT `pass` or `deviation`; every applicable MAY has a recorded
  decision; nothing `unverified`.
- Standard version and crate version both start at `0.1.0` and must stay
  equal; 0.x until the user declares 1.0.0.
- **Library purity:** outside `tools/uss/src/cli/` (and tests, `main.rs` and
  `build.rs`), no `std::fs`, `std::process`, `std::net` or `std::env`.
- **Safety and lint policy** (workspace `[lints]`): `unsafe_code = "forbid"`;
  `missing_docs` deny; Clippy `all` and `pedantic` deny; `unwrap_used`,
  `expect_used` and `panic` deny outside tests; `allow_attributes` and
  `allow_attributes_without_reason` deny. Suppress only with
  `#[expect(clippy::<lint>, reason = "<why>")]`, which also fails if the
  warning goes away. This is the check-integrity rule from spec §6.5 applied
  to this repo.
- Dependencies: only those in Tech Stack. Any other needs a reason in the
  commit message and must pass `cargo deny check`. `Cargo.lock` is committed,
  and every cargo build, test, Clippy and run in xtask and CI uses `--locked`
  (the Dependencies area's "exact, verified resolution" rule, applied here).
  Version requirements stay in Cargo's default caret form (e.g. `1.2`), which
  is a soft pin bounded by the next major version.
- Never modify system-level installs or global config. Tools such as
  cargo-deny install under the repository's own `.tools/` directory.
- CI actions pinned by full commit SHA with the version in a comment; workflow
  `permissions: contents: read`.
- Must work on Windows, Linux and macOS. Repository text files use LF
  (`.gitattributes`), but the tool accepts CRLF and a UTF-8 BOM in any input.
- Step-by-step instructions in README files are written in ASD-STE100 style
  (one instruction per sentence, imperative, active voice).
- The content of `Initial_Idea.md` and `todo.txt` is never modified
  (line-ending normalisation by `.gitattributes` is the only allowed change).
- Licence: MIT, `Copyright (c) 2026 HaleTechCo`.

## Plan-level decisions (refining the spec, for the builder)

- **Workspace at the root, crate at `tools/uss`.** The crate embeds
  `../../standard` and friends. That works for local builds and for
  `cargo install --locked --git <repo> --tag vX.Y.Z uss`, but not for
  crates.io, so `publish = false` (distribution is decided in Plan 5).
- **Embedding through `build.rs`**, not the `include_dir` crate. The build
  script writes a sorted `(path, include_bytes!)` table and emits
  `cargo:rerun-if-changed` for each embedded directory, so an edited standard
  file always triggers a rebuild. That is one fewer dependency, and no stale
  embedded copies.
- **Two I/O traits** keep the library pure: `Source` (USS's own files:
  embedded, in memory, or a `--standard-root` checkout) and `ProjectFiles`
  (an adopting project's files, relative to its root).
- **Prefixes** are declared in each standard file's header and checked for
  uniqueness by `uss lint`, rather than listed again in `conventions.md`.
  Categories and stack IDs live in `standard/registry.yaml`.
- **Facts set by a MAY** (`Enabled by`) are added to `applicability.md`
  together with the rule that enables them (Plan 3). Plan 1 writes only asked
  facts. Facts gain an `Implies` field (e.g. `has-web-ui` implies `has-ui`).
- **Report template:** `templates/audit-report.md.j2` (minijinja).
- **Audit results** record the full 40-character SHA of the audited commit.
- **ADRs** in MADR format under `docs/adr/`: 0001 records the choice of Rust,
  0002 the YAML crate. This applies the Architecture area's own rule to this
  repo.
- `--standard-root DIR` (a USS checkout) replaces the embedded copy for every
  command.

## Review Focus

1. **CRLF line endings** in standard files, records and results (Windows
   editors, adopters without `.gitattributes`): parsed the same as LF.
   Tests: Task 4 `parse_doc_crlf_and_bom`, Task 5
   `rule_file_crlf_and_bom`, Task 7 `record_crlf_and_bom`.
2. **UTF-8 BOM** at the start of Markdown or YAML (Notepad): ignored. Same
   tests as 1.
3. **Headings, fields and keywords inside fenced code blocks**: not parsed as
   rules or fields, and not keyword-scanned. Tests: Task 4
   `parse_doc_fences`, Task 5 `rule_file_code_fence`.
4. **Shallow CI clones** (`actions/checkout` defaults to depth 1), where the
   audited commit is missing: `release-check` says to fetch full history
   instead of passing on a raw git error. Test: Task 12
   `release_check_shallow_clone`.
5. **Windows path separators** in record paths (`docs\deviations\X.md`) and
   git's forward-slash output: deviation documents resolve on every OS, and
   `.uss/` detection works for nested components. Tests: Task 9
   `check_backslash_path` (library and CLI), Task 12 `only_uss_changes_table`.

---

### Task 1: Workspace bootstrap, lint policy and CLI entry point

**Files:**
- Create: `Cargo.toml` (workspace), `rust-toolchain.toml`, `.cargo/config.toml`, `clippy.toml`
- Create: `.gitattributes`, `.gitignore`, `.editorconfig`, `LICENSE`, `README.md`
- Create: `docs/adr/0001-validator-in-rust.md`
- Create: `tools/uss/Cargo.toml`, `tools/uss/src/lib.rs`, `tools/uss/src/main.rs`, `tools/uss/src/cli/mod.rs`
- Create: `tools/xtask/Cargo.toml`, `tools/xtask/src/main.rs` (stub that prints usage; filled in Task 15)

**Interfaces:**
- Produces: `uss::cli::run(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> i32`
  (behind `#[cfg(feature = "cli")]`). `main.rs` calls it with `std::env::args()`
  and exits with its code. Exit codes: `0` OK, `1` problems found, `2` usage
  error. Every later command is a clap subcommand handled in `run`.

- [ ] **Step 1: Create the workspace files**

`Cargo.toml`:

```toml
[workspace]
members = ["tools/uss", "tools/xtask"]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.100"
license = "MIT"
repository = "https://github.com/HaleTechCo/Universal-Software-Standard"
publish = false

[workspace.lints.rust]
unsafe_code = "forbid"
missing_docs = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
allow_attributes = "deny"
allow_attributes_without_reason = "deny"
```

`rust-toolchain.toml`: `channel = "1.100.0"`, `components = ["rustfmt", "clippy"]`,
`targets = ["wasm32-unknown-unknown"]`, `profile = "minimal"`.
`.cargo/config.toml`: `[alias] xtask = "run --quiet --package xtask --"`.
`clippy.toml`: `allow-unwrap-in-tests = true`, `allow-expect-in-tests = true`.
Each member's `Cargo.toml` inherits `version`, `edition`, `rust-version`,
`license`, `repository` and `publish` with `.workspace = true`, and
`[lints] workspace = true`.

Then run `rustup toolchain install` (it reads `rust-toolchain.toml`) and repeat
the toolchain check from "Before you start".

`tools/uss/Cargo.toml`: package `uss`; `[features] default = ["cli"]`,
`cli = ["dep:clap"]`; `[[bin]] name = "uss"`, `path = "src/main.rs"`,
`required-features = ["cli"]`; dependency `clap = { version = "<latest 4.x>", features = ["derive"], optional = true }`.

- [ ] **Step 2: Create the repository hygiene files**

`.gitattributes`: `* text=auto eol=lf`. `.gitignore`: `/target`, `/.tools`,
`uss-report.md`. `.editorconfig`: root, utf-8, lf, final newline, trim
trailing whitespace; `[*.rs]` 4 spaces; `[*.{md,yaml,yml,json,toml}]` 2 spaces.
Then run `git add --renormalize .` and check that
`git diff --cached --ignore-cr-at-eol -- Initial_Idea.md todo.txt` prints nothing.

- [ ] **Step 3: Create `LICENSE`, `README.md` and ADR 0001**

`LICENSE`: standard MIT text, `Copyright (c) 2026 HaleTechCo`. `README.md`:
title "Universal Software Standard (USS)", one paragraph from spec §1 (the
promise and the two uses), "Status: 0.x, in development; rules not yet
published", and a link to the design spec.
`docs/adr/0001-validator-in-rust.md` in MADR format:
- Context: the validator must be easy to run anywhere and must later run
  inside a VS Code extension.
- Options considered: Go, Rust, Python, TypeScript.
- Decision: Rust, for WebAssembly and library embedding (VS Code), its type
  system, and the original "OS-native → Rust" rule.
- Consequences: more dependencies (so cargo-deny); a weaker YAML ecosystem
  (so ADR 0002); slower builds; an I/O-free library core.

- [ ] **Step 4: Write the failing tests** (in `tools/uss/src/cli/mod.rs`)

```rust
#[test]
fn run_no_args_prints_usage() {
    let (code, _out, err) = run_capture(&[]);
    assert_eq!(code, 2);
    assert!(err.contains("Usage: uss"), "stderr: {err}");
}
#[test]
fn run_unknown_command() {
    let (code, _out, err) = run_capture(&["bogus"]);
    assert_eq!(code, 2);
    assert!(err.contains("bogus"), "stderr: {err}");
}
```

`run_capture(args) -> (i32, String, String)` is a test helper that prepends
`"uss"` and captures both streams.

- [ ] **Step 5: Run the tests to verify they fail**

Run: `cargo test -p uss`
Expected: FAIL (`run` not defined).

- [ ] **Step 6: Implement `run` with a clap-derived `Cli`**

Use `Cli::try_parse_from`, with `arg_required_else_help`. On a clap error,
write `e.render()` to `err` when `e.use_stderr()` and to `out` otherwise, then
return `e.exit_code()`. Declare the subcommands as they are implemented
(none yet).

- [ ] **Step 7: Run the tests and Clippy to verify they pass**

Run: `cargo test -p uss && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --all --check`
Expected: PASS, no Clippy findings, no formatting diff.

- [ ] **Step 8: Commit**

```bash
git add -A
git commit -m "chore: bootstrap Rust workspace, lint policy and uss CLI entry point"
```

---

### Task 2: Registry, embedded files, YAML crate and `uss version`

**Files:**
- Create: `standard/registry.yaml`, `tools/uss/build.rs`
- Create: `tools/uss/src/source.rs`, `tools/uss/src/yaml.rs`, `tools/uss/src/catalog/mod.rs`, `tools/uss/src/catalog/diag.rs`, `tools/uss/src/catalog/registry.rs`
- Create: `docs/adr/0002-yaml-crate.md`
- Modify: `tools/uss/Cargo.toml`, `tools/uss/src/lib.rs`, `tools/uss/src/cli/mod.rs`

**Interfaces:**
- Produces:
  - `uss::source::Source` trait:
    `fn read(&self, path: &str) -> Option<Cow<'_, [u8]>>` (forward-slash path
    relative to the USS root, e.g. `standard/registry.yaml`) and
    `fn list(&self, dir: &str) -> Vec<String>` (every file under `dir`,
    recursively, forward slashes, sorted).
  - `uss::source::MemSource(pub BTreeMap<String, Vec<u8>>)` and
    `uss::source::Embedded` (a unit struct over the generated table); both
    implement `Source`.
  - `uss::catalog::Diagnostic { pub file: String, pub line: usize, pub message: String }`
    with `Display` → `file:line: message`, or `file: message` when `line == 0`.
  - `uss::catalog::Category { pub id: String, pub name: String }`
  - `uss::catalog::Registry { pub version: String, pub categories: Vec<Category>, pub stack: BTreeMap<String, Vec<String>> }`,
    with `fn category_by_name(&self, name: &str) -> Option<&Category>` and
    `fn has_stack(&self, kind: &str, id: &str) -> bool`.
  - `uss::catalog::load_registry(src: &dyn Source) -> Result<Registry, Vec<Diagnostic>>`
  - `uss::yaml::from_slice<T: DeserializeOwned>(data: &[u8]) -> Result<T, String>`:
    the one place the chosen YAML crate is called. It strips a UTF-8 BOM and
    turns CRLF into LF before parsing.
  - In `cli`: `fn source_for(standard_root: Option<&Path>) -> Box<dyn Source>`
    (`Embedded` by default, otherwise a `DirSource` reading the given
    checkout); a global `--standard-root DIR` option.

- [ ] **Step 1: Choose the YAML crate and record it in ADR 0002**

Candidates: `serde-saphyr` and `serde_norway`. Rejected without evaluation:
`serde_yaml` (archived in 2024) and `serde_yml` (check RustSec; reject if it
has an advisory). For each candidate, check:
- a release within the last 12 months;
- no RustSec advisory;
- pure safe Rust preferred over transpiled C;
- deserializes into structs with `deny_unknown_fields` and into
  `serde_json::Value`;
- builds for `wasm32-unknown-unknown`.

Pick one, record the evidence in `docs/adr/0002-yaml-crate.md` (MADR), and add
it to `tools/uss/Cargo.toml`. If neither passes, stop and report. Also run
`cargo add -p uss serde --features derive`, `cargo add -p uss serde_json` and
`cargo add -p uss regex`. All three are needed from this task on.

- [ ] **Step 2: Create `standard/registry.yaml`**

```yaml
version: 0.1.0
categories:
  - { id: safety, name: Safety }
  - { id: reliability, name: Reliability }
  - { id: efficiency, name: Efficiency }
  - { id: maintainability, name: Maintainability }
  - { id: contributability, name: Contributability }
  - { id: compatibility, name: Compatibility }
  - { id: fairness, name: Fairness }
  - { id: accessibility-usability, name: Accessibility & Usability }
  - { id: features, name: Features }
stack:
  lang: [go, rust, typescript, javascript, python, java]
  framework: [react]
  platform: [docker-compose, kubernetes, helm, postgresql, sqlite, mariadb, mysql]
  tool: [maven]
```

- [ ] **Step 3: Write `build.rs`**

Walk `../../standard`, `../../schema` and `../../templates`, relative to
`CARGO_MANIFEST_DIR`; skip any that do not exist yet. Write
`$OUT_DIR/embedded.rs` containing
`pub(crate) static FILES: &[(&str, &[u8])] = &[("standard/registry.yaml", include_bytes!(<path>)), …];`,
sorted by path. Write each absolute path with `{:?}`, so Windows backslashes
are escaped. Emit `cargo:rerun-if-changed=` for each of the three directories
and for `build.rs`. `source.rs` `include!`s the file.

- [ ] **Step 4: Write the failing tests**

- `load_registry_valid` (`MemSource` holding the real file): `version == "0.1.0"`,
  9 categories, `categories[7].name == "Accessibility & Usability"`,
  `has_stack("lang", "go")`, `!has_stack("lang", "cobol")`.
- `load_registry_errors`, a table of `(yaml, expected substring)`:
  `version: "1.0"` → `"version"`; duplicate category name →
  `"duplicate category"`; stack kind `os` → `"unknown stack kind \"os\""`;
  unknown top-level key `colour` → `"colour"`; missing file →
  `"standard/registry.yaml"`.
- `embedded_contains_registry`: `Embedded.read("standard/registry.yaml").is_some()`.
- `crate_version_matches_registry`: `env!("CARGO_PKG_VERSION")` equals the
  embedded registry's version.
- `run_version` (cli): `["version"]` → exit 0, stdout `"uss 0.1.0\n"`.

- [ ] **Step 5: Run to verify they fail; implement; run to verify they pass**

Run: `cargo test -p uss`
Expected: FAIL before implementation, PASS after. Decode strictly
(`deny_unknown_fields`). Version regex `^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$`.
Stack kinds: `lang`, `framework`, `platform`, `tool`. IDs match
`^[a-z0-9]+(-[a-z0-9]+)*$`.

- [ ] **Step 6: Verify that rebuilds track standard files**

Run `cargo run -p uss -- version`. Change `version:` in
`standard/registry.yaml` to `0.1.1` and run it again: it prints `uss 0.1.1`.
Put `0.1.0` back.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat(uss): embed the standard, add the registry and the version command"
```

---

### Task 3: Applicability expressions

**Files:**
- Create: `tools/uss/src/expr.rs`

**Interfaces:**
- Produces:
  - `pub enum Expr { Always, Ident(String), Not(Box<Expr>), And(Box<Expr>, Box<Expr>), Or(Box<Expr>, Box<Expr>) }`
  - `pub fn parse(s: &str) -> Result<Expr, String>`
  - `impl Expr { pub fn eval(&self, facts: &BTreeSet<String>) -> bool; pub fn idents(&self) -> BTreeSet<String>; pub fn and(a: Expr, b: Expr) -> Expr }`.
    `and` returns the other operand when one side is `Always`. `idents`
    excludes `always`.
  - `impl Display for Expr`: `Ident` and `Always` print bare; `Not(x)` prints
    `not x`; `And` and `Or` print `l and r` / `l or r`, with an operand
    wrapped in parentheses when it is itself an `And` or `Or`.

Grammar (pinned):

```
expr  = and { "or" and }
and   = unary { "and" unary }
unary = "not" unary | atom
atom  = "always" | ident | "(" expr ")"
ident = fact | kind "." fact
kind  = "lang" | "framework" | "platform" | "tool"
fact  = [a-z0-9]+ ( "-" [a-z0-9]+ )*
```

Keywords are lowercase only; tokens are separated by whitespace or parentheses.

- [ ] **Step 1: Write the failing tests**

- `eval_table`: `"always"`→true; `"has-ui"` with {has-ui}→true;
  `"a or b and c"` with {b}→false; `"not a and b"` with {b}→true;
  `"(a or b) and c"` with {a,c}→true; `"lang.go"` with {lang.go}→true;
  a missing fact is false.
- `parse_errors`: each of these fails: `""`, `"a and"`, `"(a"`, `"a b"`,
  `"AND"`, `"has_ui"`, `"lang."`, `"os.linux"` (message contains
  `unknown stack kind "os"`).
- `idents_sorted_unique`: `"not b or (a and lang.go) or always"` →
  `{"a","b","lang.go"}`.
- `display_forms`: `parse("a and b")` displays `a and b`;
  `Expr::and(parse("lang.go"), parse("has-ui or b"))` displays
  `lang.go and (has-ui or b)`; `Expr::and(Expr::Always, parse("x"))` displays `x`.

- [ ] **Step 2: Run to verify they fail; implement a recursive-descent parser; run to verify they pass**

Run: `cargo test -p uss expr`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add tools/uss/src
git commit -m "feat(uss): parse and evaluate applicability expressions"
```

---

### Task 4: Markdown scanner and fact vocabulary

**Files:**
- Create: `tools/uss/src/catalog/markdown.rs`, `tools/uss/src/catalog/facts.rs`
- Create: `standard/applicability.md`

**Interfaces:**
- Produces (crate-internal):
  - `struct Field { value: String, line: usize }`
  - `struct Section { heading: String, line: usize, fields: BTreeMap<String, Field>, body: String, prose: String }`.
    `prose` is `body` with fenced blocks removed.
  - `struct Doc { file: String, title: String, header: BTreeMap<String, Field>, sections: Vec<Section> }`
  - `fn parse_doc(file: &str, data: &[u8]) -> (Doc, Vec<Diagnostic>)`
- Produces (public):
  - `pub enum FactSource { Question(String), EnabledBy(String) }`
  - `pub struct Fact { pub id: String, pub source: FactSource, pub definition: String, pub implies: Vec<String>, pub file: String, pub line: usize }`
  - crate-internal `fn parse_facts(file: &str, data: &[u8]) -> (BTreeMap<String, Fact>, Vec<Diagnostic>)`

Scanner rules (pinned):
- Strip a leading UTF-8 BOM; turn `\r\n` into `\n`.
- Lines starting with three backticks or `~~~` open and close a fence. Inside
  a fence nothing is a heading or a field; the lines go to `body` only.
- The first `# ` line is the title, and the header field block follows it.
- `## ` lines are grouping headings: they end the current section and are
  otherwise ignored.
- `### ` lines start a section.
- A field block starts on the first non-blank line after a heading and
  continues while lines match `^- \*\*([A-Za-z ]+):\*\* (.*)$` or are
  continuation lines (two-space indent, non-empty; appended with one space).
- A duplicate field name gives `duplicate field "<name>"`.

Fact rules: headings `### <fact-id>`; allowed fields `Question`, `Definition`,
`Enabled by`, `Implies` (comma-separated). `Definition` is required, and
exactly one of `Question` or `Enabled by`. Implied facts must exist, and
implications must not form a cycle.

- [ ] **Step 1: Write the failing scanner tests**

`parse_doc_basic` (title, header, two sections, a `##` between them that is
not a section); `parse_doc_fences` (a section body with a fenced
`### USS-X-001: y` and `- **Level:** MAY` → one section; the fenced lines are
in `body` but not in `prose`); `parse_doc_crlf_and_bom` (the `_basic` input
with BOM and CRLF → an equal `Doc`); `parse_doc_continuation` (a two-line
field joined with one space); `parse_doc_duplicate_field` (a diagnostic with
the line number).

- [ ] **Step 2: Run to verify they fail; implement `markdown.rs`; run to verify they pass**

Run: `cargo test -p uss parse_doc`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Write the failing fact tests**

`parse_facts_valid` (two facts, `has-web-ui` implies `has-ui`);
`parse_facts_errors`, a table: bad ID `Has_UI`; both `Question` and
`Enabled by`; neither; `Implies: nope` → `unknown fact "nope"`; cycle a→b→a →
`implication cycle`; unknown field `Colour` → `unknown field "Colour"`.
`applicability_file_is_valid`: `parse_facts` on the embedded
`standard/applicability.md` gives zero diagnostics and exactly the 27 IDs in
Step 5.

- [ ] **Step 4: Implement `facts.rs`**

- [ ] **Step 5: Write `standard/applicability.md`**

H1 "Applicability facts". An intro paragraph explains that facts are yes/no
statements about a project, that agents infer them from the code and then
confirm them with the developer, and that rules use them in `Applies when`.
Then one `###` section per fact, each with a `Question` (one short question)
and a `Definition` (when the answer is yes). Add `Implies` where listed:

| Fact | Implies | Yes when |
|---|---|---|
| has-ui | | people interact with it through any UI (GUI, web, TUI) |
| has-web-ui | has-ui | it serves a UI in a web browser |
| has-cli | | it has a command-line interface |
| stores-persistent-data | | it keeps data across restarts |
| stores-user-data | stores-persistent-data | it stores data about or created by its users |
| uses-database | stores-persistent-data | it uses a database engine |
| uses-relational-database | uses-database | that database is relational (SQL) |
| exposes-network-api | | it accepts requests over a network |
| is-network-service | exposes-network-api | it runs as a long-lived network service others connect to |
| operates-service | | the project itself runs a production deployment |
| handles-authentication | | it authenticates users or clients |
| handles-sensitive-data | | it handles personal, financial, health, credential or similar data |
| high-assurance | | failure could cause serious harm (safety, critical infrastructure, large financial loss) |
| uses-cryptography | | it encrypts, signs, hashes for security, or manages keys |
| requires-fips | uses-cryptography | it must run in a FIPS 140-3 validated configuration |
| ships-container-image | | it publishes an OCI container image |
| deploys-to-kubernetes | | it ships Kubernetes manifests or Helm charts |
| is-library | | others use it as a library or SDK |
| distributed-as-package | | it is released as an installable package or binary |
| accepts-external-contributions | | people outside the core team may contribute |
| uses-ai-models | | it calls or runs AI models |
| exposes-mcp-server | | it provides an MCP server |
| has-multiple-users | | more than one person or account uses one installation |
| is-commercial | | any edition, tier, feature or licence is sold |
| is-foss | | it is released under an OSI-approved licence |
| sold-in-eu | | it is placed on the EU market |
| processes-eu-personal-data | | it processes personal data of people in the EU |

- [ ] **Step 6: Run all tests**

Run: `cargo test -p uss`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add -A
git commit -m "feat(uss): add Markdown scanner and applicability fact vocabulary"
```

---

### Task 5: Rule file parsing

**Files:**
- Create: `tools/uss/src/catalog/rules.rs`

**Interfaces:**
- Consumes: `parse_doc` (Task 4); `expr::parse` and `Expr::and` (Task 3).
- Produces:
  - `pub enum Level { Must, MustNot, Should, ShouldNot, May }` with `Display`
    giving `MUST`, `MUST NOT`, `SHOULD`, `SHOULD NOT`, `MAY`, and `FromStr`.
  - `pub enum FileKind { Area, Module }`
  - `pub struct Active { pub level: Level, pub applies_when: String, pub applies: Expr, pub category: String, pub verify: String, pub references: String, pub enables: Vec<String>, pub refines: Vec<String>, pub body: String }`.
    `applies` is `Expr::and(file applies, rule applies)`.
  - `pub enum RuleState { Active(Active), Retired { retired_in: String, replaced_by: Option<String> } }`
  - `pub struct Rule { pub id: String, pub title: String, pub kind: FileKind, pub file: String, pub line: usize, pub state: RuleState }`
    with `fn active(&self) -> Option<&Active>`.
  - `pub struct RuleFile { pub file: String, pub title: String, pub prefix: String, pub kind: FileKind, pub applies_when: String, pub rules: Vec<Rule> }`
  - crate-internal `fn parse_rule_file(file: &str, data: &[u8]) -> (RuleFile, Vec<Diagnostic>)`

File-local checks (pinned):
- Header: `Prefix` (required, `^[A-Z][A-Z0-9]{1,7}$`); `Kind` (required,
  `area` or `module`); `Applies when` (optional, default `always`).
- Every `###` heading matches `^(USS-([A-Z][A-Z0-9]{1,7})-(\d{3})): (.+)$`,
  otherwise `heading is not a rule`. The prefix equals the file's `Prefix`.
- Allowed fields: Level, Applies when, Category, Verify, References, Enables,
  Refines, Status, Retired in, Replaced by.
- Active rules require Level, Applies when, Category, Verify, References and a
  non-empty body. A retired rule (`Status: Retired`) requires `Retired in`
  (semver) and `Replaced by` (a rule ID or `none`).
- `Enables` only on `MAY`. `Refines` only in `module` files.
- **Keyword scan** of `prose` with the regex
  `\b(MUST NOT|SHOULD NOT|SHALL NOT|NOT RECOMMENDED|MUST|SHOULD|SHALL|MAY|REQUIRED|RECOMMENDED|OPTIONAL)\b`:
  a forbidden synonym → `forbidden keyword "<kw>"`; any other level's keyword
  → `keyword "<kw>" in a <LEVEL> rule`; the rule's own keyword absent →
  `body does not state the rule with "<LEVEL>"`.

- [ ] **Step 1: Write the failing tests**

- `rule_file_valid`: the spec §5.2 example (`USS-SEC-012`, MUST,
  `uses-database`, Safety) in a file with `Prefix: SEC`, `Kind: area` → one
  active rule with each field equal to the spec's values.
- `rule_file_module_applies_when`: a module file with `Applies when: lang.go`
  and a rule with `always` → `applies.to_string() == "lang.go"`.
- `rule_file_errors`: a table, one bad input per check above, each with its
  expected substring.
- `rule_file_code_fence`: a body with a fenced example containing
  `### USS-SEC-999: x` and `SHOULD` → one rule, no diagnostics.
- `rule_file_crlf_and_bom`: the `_valid` input with BOM and CRLF → equal rules.
- `rule_file_retired`: a retired rule without `Verify` is valid; one missing
  `Replaced by` is not.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `cargo test -p uss rule_file`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add tools/uss/src
git commit -m "feat(uss): parse and check rule files"
```

---

### Task 6: Catalog loading, cross-file lint and `uss lint`

**Files:**
- Modify: `tools/uss/src/catalog/mod.rs`
- Create: `tools/uss/src/testfixture.rs` (`#[cfg(test)]`)
- Modify: `tools/uss/src/cli/mod.rs`

**Interfaces:**
- Consumes: `load_registry`, `parse_facts`, `parse_rule_file`.
- Produces:
  - `pub struct Catalog { pub registry: Registry, pub facts: BTreeMap<String, Fact>, pub rules: Vec<Rule>, pub file_count: usize }`.
    `rules` holds every rule, retired ones included, sorted by ID.
  - `pub fn load(src: &dyn Source) -> Result<Catalog, Vec<Diagnostic>>`. It
    reads `standard/registry.yaml`, `standard/applicability.md`, every `.md`
    under `standard/core/` (kind must be `area`), and every `.md` under
    `standard/modules/` recursively (kind must be `module`). A missing
    directory is fine. Diagnostics are sorted by file, then line.
  - `impl Catalog { pub fn rule(&self, id: &str) -> Option<&Rule>; pub fn known_ident(&self, id: &str) -> bool }`.
    `known_ident` is true for a defined fact or a `kind.id` whose ID is in the
    registry stack.
  - `testfixture::standard() -> MemSource`: the mini-standard below.
  - CLI: `uss lint`.

Cross-file checks: duplicate rule IDs; the same prefix in two files; an
unregistered category; an expression identifier that `known_ident` rejects;
an `Enables` fact that does not exist or whose `Enabled by` differs; a fact
whose `Enabled by` is not a MAY rule listing it in `Enables`; a `Refines`
target that is missing or not in an `area` file; a `Replaced by` target that
is missing.

Fixture: registry version `0.1.0`; categories Safety and Features; stack
`lang: [go]`; facts `has-ui`, `uses-database`, `has-web-ui` (implies `has-ui`)
with Questions, and `has-plugin-interface` (`Enabled by: USS-EXT-001`).

| ID | Level | Applies when | Category | Notes |
|---|---|---|---|---|
| USS-SEC-001 | MUST | always | Safety | |
| USS-SEC-002 | SHOULD | uses-database | Safety | |
| USS-SEC-003 | MUST NOT | has-ui | Safety | |
| USS-SEC-004 | (retired) | | | Replaced by USS-SEC-001 |
| USS-EXT-001 | MAY | always | Features | Enables has-plugin-interface |
| USS-EXT-002 | MUST | has-plugin-interface | Features | |
| USS-GO-001 | MUST | always (file: lang.go) | Safety | module, Refines USS-SEC-001 |

- [ ] **Step 1: Write the failing tests**

- `load_fixture`: Ok; 7 rules; `rules[0].id == "USS-EXT-001"`; the
  `USS-GO-001` `applies` displays `lang.go`.
- `load_cross_file_errors`: a table that alters the fixture once per check,
  each with its expected substring.
- `run_lint_embedded`: `["lint"]` → exit 0, stdout starts with `uss lint: ok (`.
- `run_lint_standard_root` (first run `cargo add -p uss --dev tempfile`):
  write the fixture to a `tempfile::TempDir` with
  one rule missing `Verify`, then
  `["--standard-root", <dir>, "lint"]` → exit 1; stdout contains
  `standard/core/security.md:` and ends with `uss lint: 1 problem\n`.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `cargo test -p uss`
Expected: FAIL before implementation, PASS after. Output: one diagnostic per
line, then `uss lint: ok (N rules, M files)` or `uss lint: N problem(s)`
("problem" when N is 1).

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(uss): load the catalog and add the lint command"
```

---

### Task 7: Schemas, records and audit results

**Files:**
- Create: `schema/record.schema.json`, `schema/audit-results.schema.json`
- Create: `tools/uss/src/record/mod.rs`, `tools/uss/src/record/results.rs`, `tools/uss/src/record/testdata/`

**Interfaces:**
- Produces (module `uss::record`; serde with `deny_unknown_fields` and
  `rename_all = "kebab-case"`):
  - `StandardRef { version: String }`, `Project { name: String }`,
    `Stack { languages, frameworks, platforms, tools: Vec<String> }` (each defaults to empty)
  - `Decision { adopt: bool }`
  - `Deviation { rule, document, approved_by, date: String, review_by: Option<String> }`
  - `AuditRef { date, results, report: String }`
  - `Record { standard, project, stack, facts: BTreeMap<String, bool>, decisions: BTreeMap<String, Decision>, deviations: Vec<Deviation>, audits: Vec<AuditRef>, components: Vec<String>, #[serde(skip)] path: String }`,
    with `fn latest_audit(&self) -> Option<&AuditRef>` (greatest `date`; on
    a tie, the later entry).
  - `pub enum Outcome { Pass, Fail, Deviation, Declined, Unverified, NotApplicable }`
    (kebab-case in YAML, with `Display` giving the same text)
  - `Entry { result: Outcome, evidence: String (default empty) }`
  - `Suppression { file: String, line: Option<u32>, rule: String, reason: String }`
  - `Results { standard, commit, date, auditor: String (default empty), results: BTreeMap<String, Entry>, suppressions: Vec<Suppression>, #[serde(skip)] path: String }`
  - `pub struct LoadError { pub path: String, pub problems: Vec<String> }`,
    with `Display` → `path: p1; p2`
  - `pub fn parse_record(src: &dyn Source, path: &str, data: &[u8]) -> Result<Record, LoadError>`
  - `pub fn parse_results(src: &dyn Source, path: &str, data: &[u8]) -> Result<Results, LoadError>`

Pipeline: `yaml::from_slice::<serde_json::Value>` → validate against
`schema/*.json`, read from `src` with `jsonschema` (default features off, no
remote references) → `serde_json::from_value` into the struct. Each problem
names the JSON pointer of the failing instance.

Schema content (draft 2020-12, `additionalProperties: false` everywhere, `$id`
under `https://github.com/HaleTechCo/Universal-Software-Standard/schema/`):
- record: required `standard`, `project`, `facts`. Version uses the semver
  pattern. Facts have kebab-case keys and boolean values. Decision keys use
  the rule-ID pattern, with values `{adopt: boolean}` (required). Deviation
  items require `rule`, `document`, `approved-by`, `date`. Dates match
  `^\d{4}-\d{2}-\d{2}$`. Audit items require `date`, `results`, `report`.
  `components` and stack arrays hold unique strings.
- audit-results: required `standard`, `commit`, `date`, `results`. `commit`
  matches `^[0-9a-f]{40}$`. Result keys use the rule-ID pattern; `result` is
  the six-value enum. `evidence` is required (minLength 1) when the result is
  `pass`, `fail` or `deviation`. Suppression items require `file`, `rule`,
  `reason`; `line` is an integer ≥ 1.

- [ ] **Step 1: Add `jsonschema` (default features off)**

Run: `cargo add -p uss jsonschema --no-default-features`
Expected: `cargo build -p uss` succeeds.

- [ ] **Step 2: Write the failing tests**

- `record_valid` (`testdata/record-valid.yaml` with an unquoted
  `date: 2026-11-21`): fields populated; `deviations[0].date == "2026-11-21"`.
- `record_crlf_and_bom`: the same file with BOM and CRLF → an equal `Record`.
- `record_schema_errors`, a table: unknown top-level key `colour` → a problem
  containing `colour`; `has-ui: "yes"` → `/facts/has-ui`; decision key
  `SEC-1` → `/decisions`; deviation without `approved-by` → `/deviations/0`;
  `date: 21/11/2026` → `/deviations/0/date`.
- `results_valid`; `results_errors`: commit `1ab35de` → `/commit`; result
  `passed` → `/results/USS-SEC-001/result`; `pass` without evidence →
  `/results/USS-SEC-001`.
- `latest_audit_tie_takes_later`.

- [ ] **Step 3: Run to verify they fail; write the schemas and implement; run to verify they pass**

Run: `cargo test -p uss record`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat(uss): add record and audit-results schemas and parsers"
```

---

### Task 8: Fact resolution and `uss applicable`

**Files:**
- Create: `tools/uss/src/evaluate/mod.rs`, `tools/uss/src/evaluate/facts.rs`
- Create: `tools/uss/src/cli/files.rs` (`DirSource`, `OsProject`, `read_record`, `project_root`)
- Modify: `tools/uss/src/cli/mod.rs`

**Interfaces:**
- Consumes: `Catalog`, `Record`.
- Produces:
  - `pub fn resolve_facts(c: &Catalog, r: &Record) -> BTreeSet<String>`
    (the set of true facts)
  - `pub fn applicable<'c>(c: &'c Catalog, r: &Record) -> Vec<&'c Rule>`:
    active rules whose `applies` is true, sorted by ID.
  - cli: `fn project_root(record_path: &Path) -> PathBuf` (the parent of the
    `.uss` directory that holds the record, or the record's own directory);
    `fn read_record(src: &dyn Source, path: &Path) -> Result<Record, String>`
    (sets `path` to the given path, with forward slashes);
    `fn version_error(pinned: &str, carried: &str) -> String`, returning
    `record pins USS <pinned>; this uss carries USS <carried>. Use uss <pinned>: download it from the v<pinned> release, or run: cargo install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v<pinned> uss`.
  - CLI: `uss applicable [--json] RECORD`.

Resolution (pinned): start from the record's true facts; add `lang.<x>` for
each `stack.languages`, and `framework.<x>`, `platform.<x>` and `tool.<x>`
likewise. Then repeat until nothing changes: add `implies` closures; for each
active MAY rule that applies and has `decisions[id].adopt == true`, add its
`enables` facts.

Output: one line per rule, `ID<TAB>LEVEL<TAB>Category<TAB>Title`. With
`--json`: an array of `{"id","title","level","category","file"}`.

- [ ] **Step 1: Write the failing tests** (on `testfixture::standard()`)

- `resolve_facts_rules`: `{has-web-ui}` → contains `has-ui`; stack `[go]` →
  contains `lang.go`; `has-plugin-interface` present only when
  `USS-EXT-001` is adopted.
- `applicable_sets`: all facts false, no stack, no decisions →
  `[USS-EXT-001, USS-SEC-001]`; with `USS-EXT-001` adopted → adds
  `USS-EXT-002`; with stack `[go]` → adds `USS-GO-001`; `USS-SEC-004` never
  appears.
- `run_applicable_version_mismatch`: a record pinning `9.9.9` → exit 1, stderr
  contains `--tag v9.9.9 uss`.
- `project_root_cases`: `<tmp>/proj/.uss/record.yaml` → `<tmp>/proj`;
  `<tmp>/x/record.yaml` → `<tmp>/x`.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `cargo test -p uss`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(uss): resolve facts and add the applicable command"
```

---

### Task 9: Record checks and `uss check`

**Files:**
- Create: `tools/uss/src/evaluate/check.rs`, `tools/uss/src/evaluate/project.rs`
- Modify: `tools/uss/src/cli/files.rs`, `tools/uss/src/cli/mod.rs`

**Interfaces:**
- Consumes: `resolve_facts`, `applicable`, `parse_record`.
- Produces:
  - `pub trait ProjectFiles { fn exists(&self, rel: &str) -> bool; fn read(&self, rel: &str) -> Option<Vec<u8>>; }`.
    `rel` is a forward-slash path relative to the project root.
  - `pub struct MemProject(pub BTreeMap<String, Vec<u8>>)` and
    `pub struct SubProject<'a> { pub inner: &'a dyn ProjectFiles, pub prefix: String }`
    (a component's view of its parent's files); both implement `ProjectFiles`.
  - cli: `OsProject { root: PathBuf }` implements `ProjectFiles` by joining
    `rel`'s `/`-separated parts onto `root`.
  - `pub fn check_record(src: &dyn Source, c: &Catalog, r: &Record, project: &dyn ProjectFiles) -> Vec<Diagnostic>`.
    `file` is `r.path`. Component records are parsed and checked
    recursively, and their diagnostics carry their own path
    (`<component>/.uss/record.yaml`).
  - CLI: `uss check RECORD`. It prints the diagnostics, then `uss check: ok`
    (exit 0) or `uss check: N problem(s)` (exit 1).

Checks and messages (pinned). Normalise every path from a record with
`replace('\\', "/")` before using it.
- A fact key that is not defined → `unknown fact "<id>"`.
- A fact set by a rule → `fact "<id>" is set by <rule>, not answered directly`.
- An asked fact missing → `fact "<id>" is not answered`.
- A decision for an unknown rule → `decision for unknown rule <id>`.
- A decision for a non-MAY rule → `decision for <id>, a <LEVEL> rule; only MAY rules take decisions`.
- A decision for a MAY that does not apply → `decision for <id>, which does not apply`.
- An applicable MAY without a decision → `no decision for applicable MAY rule <id>`.
- A deviation for an unknown rule, a MAY rule, a non-applicable rule, or the
  same rule twice → messages naming the rule.
- A missing deviation document → `deviation document "<doc>" not found`.
- `review-by` earlier than `date` → `review-by is before date for <id>`.
- A missing audit results file → `audit results "<path>" not found`.
- A component path that is absolute or contains `..` →
  `component path "<p>" leaves the project`. A missing
  `<p>/.uss/record.yaml` → `component record "<path>" not found`.

- [ ] **Step 1: Write the failing tests**

- `check_valid` (`MemProject`): a complete fixture record (all three asked
  facts answered, a decision for `USS-EXT-001`, `uses-database: true`, a
  deviation for `USS-SEC-002` with its document present) → no diagnostics.
- `check_errors`: a table, one case per check, each with its message substring.
- `check_backslash_path` (library): document `docs\deviations\USS-SEC-002.md`
  with `docs/deviations/USS-SEC-002.md` present in the `MemProject` → no
  diagnostic.
- `check_backslash_path_cli`: the same with `OsProject` on a temp dir, run
  through `["check", <record>]` → exit 0, on every OS.
- `check_components`: root `components: [svc]`, and `svc/.uss/record.yaml`
  leaves `has-ui` unanswered → one diagnostic whose file is
  `svc/.uss/record.yaml`.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `cargo test -p uss`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(uss): check project records and add the check command"
```

---

### Task 10: Compliance status and category scores

**Files:**
- Create: `tools/uss/src/evaluate/score.rs`

**Interfaces:**
- Consumes: `applicable`, `Results`.
- Produces:
  - `pub struct RuleOutcome<'c> { pub rule: &'c Rule, pub outcome: Outcome, pub evidence: String }`
  - `pub struct Status { pub compliant: bool, pub must_failures: usize, pub must_deviations: usize, pub should_failures: usize, pub unverified: usize, pub missing_decisions: usize }`
  - `pub struct Score { pub category: String, pub pass: usize, pub applicable: usize, pub percent: Option<u32> }`
    (`None` means n/a)
  - `pub struct Evaluation<'c> { pub version: String, pub date: String, pub commit: String, pub auditor: String, pub status: Status, pub scores: Vec<Score>, pub rules: Vec<RuleOutcome<'c>>, pub suppressions: Vec<Suppression>, pub deviations: Vec<Deviation> }`
  - `pub fn evaluate<'c>(c: &'c Catalog, r: &Record, res: &Results) -> Result<Evaluation<'c>, Vec<Diagnostic>>`
  - `impl Evaluation<'_> { pub fn statement(&self) -> String; pub fn score_line(&self) -> String }`

Rules (pinned):
- Each applicable rule takes its outcome from `res`. A missing one is
  `Unverified` with evidence `no result recorded`.
- Consistency diagnostics (`file` = `res.path`) make `evaluate` return `Err`:
  - a result for an unknown, retired or non-applicable rule;
  - `not-applicable` for an applicable rule;
  - `deviation` on a MAY, or without a record deviation for that rule;
  - `declined` on a non-MAY, or without `adopt: false`;
  - a MAY with `adopt: false` and any outcome other than `declined`;
  - a results version different from the record version.
- Counting: MUST/MUST NOT `fail` → `must_failures`, `deviation` →
  `must_deviations`; SHOULD/SHOULD NOT `fail` → `should_failures`;
  `unverified` at any level → `unverified`; an applicable MAY without a
  decision → `missing_decisions`. `compliant` means every count is 0.
- Scores: for each registry category in order,
  `percent = Some(pass * 100 / applicable)` (integer division), or `None` when
  `applicable == 0`.
- `statement()`: `Compliant with USS <v> as of <date>.` or
  `Not compliant with USS <v> as of <date>: <parts>.`. The parts are, in this
  order and only when non-zero, joined by `, `: `N MUST failure(s)`,
  `N documented MUST deviation(s)`, `N SHOULD failure(s)`,
  `N unverified rule(s)`, `N missing MAY decision(s)`. Singular when N is 1.
- `score_line()`: `<Name> <P>%` or `<Name> n/a` for every category, joined by
  `, `, ending with `.`.

- [ ] **Step 1: Write the failing tests** (fixture standard and records)

- `compliant_with_declined_may`: all facts false; `USS-EXT-001` declined
  (`adopt: false`, outcome `declined`); `USS-SEC-001` pass →
  `statement() == "Compliant with USS 0.1.0 as of 2026-11-21."`,
  `score_line() == "Safety 100%, Features 0%."`.
- `not_compliant_parts`: `uses-database: true`; `USS-SEC-001` deviation
  (recorded); `USS-SEC-002` fail; no decision and no result for `USS-EXT-001`
  → `"Not compliant with USS 0.1.0 as of 2026-11-21: 1 documented MUST deviation, 1 SHOULD failure, 1 unverified rule, 1 missing MAY decision."`
- `floor_rounding`: `uses-database` and `has-ui` true; SEC-001 and SEC-002
  pass, SEC-003 fail; EXT-001 declined → Safety `Some(66)`; the statement
  contains `1 MUST failure`.
- `missing_result_is_unverified`.
- `consistency_errors`: a table, one case per consistency rule.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `cargo test -p uss evaluate`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add tools/uss/src
git commit -m "feat(uss): compute compliance status and category scores"
```

---

### Task 11: Audit report and `uss report`

**Files:**
- Create: `templates/audit-report.md.j2`
- Create: `tools/uss/src/report.rs`, `tools/uss/src/testdata/report.golden.md`
- Modify: `tools/uss/src/cli/mod.rs`

**Interfaces:**
- Consumes: `evaluate`, `Evaluation`.
- Produces:
  - `pub fn render(src: &dyn Source, project: &str, ev: &Evaluation) -> Result<String, String>`.
    It loads `templates/audit-report.md.j2` from `src` and renders it with
    minijinja, using filters `percent` (`Option<u32>` → `N%` or `n/a`) and
    `cell` (escapes `|` as `\|`, turns newlines into `<br>`).
  - CLI: `uss report [--record PATH] [--out PATH] RESULTS`. `--record`
    defaults to `.uss/record.yaml`; `--out` defaults to RESULTS with `.yaml`
    replaced by `.md`. Consistency diagnostics → print them, exit 1, write
    nothing. Otherwise write the report, print `statement()` and
    `score_line()`, and exit 0, even when not compliant.

Report layout (pinned headings; empty sections omitted):

```
# USS audit report: <project>
<statement>
**Audited commit:** `<commit>` · **Date:** <date> · **Auditor:** <auditor>
## Scores            table: Category | Score | Passed | Applicable
## Results
### Fail | ### Unverified | ### Deviation | ### Declined | ### Pass
                     each a table: Rule | Level | Category | Title | Evidence
## Standard Deviations   table: Rule | Document | Approved by | Date | Review by
## Suppressions          table: File | Line | Rule | Reason
```

Within each outcome section, sort by level (MUST, MUST NOT, SHOULD,
SHOULD NOT, MAY), then by ID, so MUST failures come first.

- [ ] **Step 1: Add minijinja**

Run: `cargo add -p uss minijinja`
Expected: the build succeeds.

- [ ] **Step 2: Write the failing tests**

- `render_golden`: the `not_compliant_parts` evaluation plus one suppression
  → equals `testdata/report.golden.md`. When `USS_UPDATE_GOLDEN=1` is set,
  the test writes the file instead. Check the golden file by eye against the
  layout above before committing it.
- `render_escapes_cells`: evidence `a|b\nc` renders as `a\|b<br>c`.
- `run_report_writes_file`: record and results in a temp dir → the `.md` file
  exists, stdout starts with `Compliant with USS 0.1.0`, exit 0.
- `run_report_consistency_error`: a result for an unknown rule → exit 1, no
  `.md` written.

- [ ] **Step 3: Run to verify they fail; implement; run to verify they pass**

Run: `cargo test -p uss`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 4: Commit**

```bash
git add -A
git commit -m "feat(uss): render audit reports and add the report command"
```

---

### Task 12: `uss release-check`

**Files:**
- Create: `tools/uss/src/release.rs` (pure), `tools/uss/src/cli/git.rs`
- Modify: `tools/uss/src/cli/mod.rs`

**Interfaces:**
- Consumes: `read_record`, `latest_audit`, `parse_results`, `evaluate`, `render`.
- Produces:
  - `uss::release::only_uss_changes(paths: &[String]) -> Vec<String>`: returns
    the offending paths. A path is allowed when any `/`-separated segment
    equals `.uss`.
  - cli `git` (each runs `git -C <repo> …` through `std::process::Command`):
    `commit_exists(repo, sha) -> Result<bool, String>`
    (`cat-file -e <sha>^{commit}`); `is_shallow(repo) -> Result<bool, String>`
    (`rev-parse --is-shallow-repository`); `is_ancestor(repo, sha) -> Result<bool, String>`
    (`merge-base --is-ancestor <sha> HEAD`); `changed_files(repo, sha) -> Result<Vec<String>, String>`
    (`diff --name-only <sha> HEAD`).
  - CLI: `uss release-check [--record PATH] [--out PATH] [--require-compliant]`.
    `--out` defaults to `uss-report.md`.

Behaviour (pinned). For the root record and each component record:
- No audit → `no audit recorded in <record>`.
- A missing commit → if the clone is shallow,
  `audited commit <sha> is not in this clone; the clone is shallow. Fetch full history (actions/checkout: fetch-depth: 0).`,
  otherwise `audited commit <sha> not found`.
- Not an ancestor → `audited commit <sha> is not an ancestor of HEAD`.
- Offending changes → `changes since the audit outside .uss/:` followed by one
  path per line.

Any of these → exit 1. Otherwise evaluate (consistency errors → exit 1),
render each record's report, join them (root first) with `\n---\n\n` into
`--out`, and print each `statement()`. With `--require-compliant`, exit 1 if
any record is not compliant.

- [ ] **Step 1: Write the failing tests**

Helper `new_repo()`: `git init` in a temp dir; commit with
`git -c user.name=uss-test -c user.email=uss-test@example.invalid commit`.
- `only_uss_changes_table`: `.uss/audits/a.yaml` allowed;
  `svc/.uss/record.yaml` allowed; `src/main.rs` offending; `.ussx/a` offending.
- `release_check_ok`: commit A holds the source and the record. Commit B adds
  `.uss/audits/a.yaml` (results with A's full SHA) and the record's `audits`
  entry, and changes nothing outside `.uss/` → exit 0 and the report file
  exists.
- `release_check_changes_outside_uss` → exit 1; output lists `src/main.rs`.
- `release_check_no_audit` → exit 1.
- `release_check_shallow_clone`: `git clone --depth 1 <file URL>` where the
  audit is at an older commit → exit 1; message contains `fetch-depth: 0`.
  Use a `file://` URL, because git ignores `--depth` for plain local paths. On
  Windows the URL is `file:///C:/...`.
- Every git test returns early with a printed note when `git --version` fails.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `cargo test -p uss release`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(uss): add release-check tying audits to release commits"
```

---

### Task 13: `uss export --json`

**Files:**
- Create: `tools/uss/src/catalog/export.rs`
- Modify: `tools/uss/src/cli/mod.rs`

**Interfaces:**
- Consumes: `Catalog`.
- Produces: `pub struct Export { version, categories, facts, rules }`
  (`Serialize`, camelCase) and `impl Catalog { pub fn export(&self) -> Export }`.
  Each exported rule has `id`, `title`, `level` (`null` when retired),
  `appliesWhen` (the effective `applies` display), `category`, `verify`,
  `references`, `enables`, `refines`, `status` (`active` or `retired`),
  `retiredIn`, `replacedBy`, `file` and `body`. Each exported fact has `id`,
  `question`, `enabledBy`, `definition` and `implies`. Facts and rules are
  sorted by ID. CLI: `uss export --json` prints indented JSON; without
  `--json` → exit 2 with `export: only --json is supported`.

- [ ] **Step 1: Write the failing tests**

- `export_fixture`: 7 rules; `USS-GO-001` has `appliesWhen == "lang.go"`;
  `USS-SEC-004` has `status == "retired"`; 4 facts.
- `run_export_json`: stdout parses as JSON, and `version` is `0.1.0`.
- `run_export_no_format` → exit 2.

- [ ] **Step 2: Run to verify they fail; implement; run to verify they pass**

Run: `cargo test -p uss export`
Expected: FAIL before implementation, PASS after.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "feat(uss): export the rule catalog as JSON"
```

---

### Task 14: Conventions and adopter templates

**Files:**
- Create: `standard/conventions.md`, `templates/record.yaml`, `templates/standard-deviation.md`
- Modify: `tools/uss/src/record/mod.rs` (one test)

**Interfaces:**
- Consumes: `parse_record`, `catalog::load`, `Embedded`.

`standard/conventions.md` is the normative home of everything below. The spec
is design history; write each item here once.
1. Keywords: RFC 2119 + RFC 8174; the five keywords; synonyms not used; one
   rule, one level.
2. Standard files: area and module files; header fields Prefix, Kind,
   Applies when.
3. Rule format: the spec §5.2 field table plus `Refines`; body;
   `**Rationale:**`; the keyword-scan rules from Task 5. Use the spec's
   `USS-SEC-012` as the example, in a code fence.
4. Rule IDs and retiring rules (spec §5.4).
5. Applicability: facts (`applicability.md`), `Implies`, stack identifiers,
   the Task 3 grammar, `Enables`, module-level `Applies when`.
6. Score categories: the nine, with the "Measures" text from spec §5.5.
7. Choosing a level, reference don't reinvent, and pairing "zero violations"
   rules with check integrity (spec §6.4).
8. Compliance and scores: the outcome values, the compliant definition,
   scoring, "as perfect as it can be", and the exact statement formats from
   Task 10.
9. Standard Deviations (spec §5.6), and that a MUST deviation means not
   compliant.
10. Versioning of the standard (spec §5.7, including the 0.x rule).
11. Records and audit results: `.uss/record.yaml`,
    `.uss/audits/<date>.yaml` and `.md`, the schemas, components.

`templates/record.yaml`: a starter record with a comment above each key. It
lists every asked fact as `false`, each preceded by its Question as a comment
and the line `# Answer this. false is not a default.` Version `0.1.0`; project
name `example`; empty stack lists, decisions, deviations and audits.

`templates/standard-deviation.md`: headings Rule, Level, USS version, Scope,
Reason, Risks and impact, Compensating controls, Approval (Approver, Date;
"follow your organisation's approval policy"), Review by, Remediation plan.

- [ ] **Step 1: Write the failing test**

`record_template_matches_standard`: `parse_record(&Embedded, "templates/record.yaml", <embedded bytes>)`
is Ok; its `standard.version` equals the embedded registry version; and its
fact keys equal exactly the set of facts whose source is
`FactSource::Question` in the embedded catalog.

- [ ] **Step 2: Run to verify it fails**

Run: `cargo test -p uss record_template`
Expected: FAIL (the template does not exist).

- [ ] **Step 3: Write the three documents**

- [ ] **Step 4: Run all tests and lint the embedded standard**

Run: `cargo test --workspace && cargo run -p uss -- lint`
Expected: PASS; `uss lint: ok (0 rules, 0 files)`.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "docs: add standard conventions and adopter templates"
```

---

### Task 15: One-command checks, supply-chain policy, WebAssembly check and CI

**Files:**
- Create: `deny.toml`
- Modify: `tools/xtask/src/main.rs`
- Create: `.github/workflows/ci.yml`
- Modify: `README.md` (a "Development" section)

**Interfaces:**
- Produces: `cargo xtask check`, the single command that runs every check,
  used both locally and in CI. xtask uses only `std`.

- [ ] **Step 1: Create `deny.toml`**

`[advisories]`: `version = 2`, `yanked = "deny"`. `[licenses]`: `version = 2`,
with an `allow` list of only the licences the current dependency tree needs
(start from MIT, Apache-2.0, Unicode-3.0; add any other only with a note in
the commit message). `[bans]`: `wildcards = "deny"`,
`multiple-versions = "warn"`. `[sources]`: `unknown-registry = "deny"`,
`unknown-git = "deny"`.

- [ ] **Step 2: Implement `cargo xtask check`**

It runs these in order, prints `==> <step>` before each, and stops with exit
1 at the first failure:
0. Toolchain guard: read `channel` from `rust-toolchain.toml`; run
   `rustc -vV` and `rustc --print sysroot`. Fail unless the release equals the
   channel and the sysroot path contains a `.rustup` segment. The message is:
   `The active rustc is <release> at <sysroot>. This repository needs the rustup toolchain <channel>. Put the rustup bin directory first on PATH. Then run: rustup toolchain install`.
   Test it with a pure function
   `fn toolchain_problem(channel: &str, release: &str, sysroot: &str) -> Option<String>`
   and unit tests for: a match (None); a wrong release; a standalone sysroot
   such as `C:\Program Files\Rust stable LLVM 1.100`.
1. `cargo fmt --all --check`
2. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
3. `cargo test --locked --workspace --all-features`
4. `cargo build --locked -p uss --lib --no-default-features --target wasm32-unknown-unknown`
5. `.tools/bin/cargo-deny check`. If the binary is missing, first run
   `cargo install cargo-deny --locked --version <pinned> --root .tools`, with
   the version as a constant in xtask (the latest release at implementation
   time).
6. `cargo run --locked --quiet -p uss -- lint`

- [ ] **Step 3: Run it and fix every finding**

Run: `cargo xtask check`
Expected: all seven steps (0–6) pass. Fix findings in the code; suppress only with
`#[expect(..., reason = "...")]`. If step 4 fails because a dependency does
not support `wasm32-unknown-unknown`, do not work around it beyond that
crate's documented options. Record the blocking crate in ADR 0001's
consequences, remove step 4, and report it.

- [ ] **Step 4: Create `.github/workflows/ci.yml`**

`on: push` (branch `main`) and `pull_request`; `permissions: contents: read`.
Job `check`: matrix `os: [ubuntu-latest, windows-latest, macos-latest]`.
Steps:
1. `actions/checkout` with `persist-credentials: false`.
2. `rustup toolchain install`, which reads `rust-toolchain.toml`.
3. `Swatinem/rust-cache`.
4. `actions/cache` for `.tools`, keyed on `hashFiles('tools/xtask/src/main.rs')`.
5. `cargo xtask check`.

Pin each action to its latest release's full commit SHA, with the tag in a
trailing comment. Resolve the SHA with
`gh api repos/<owner>/<repo>/git/ref/tags/<tag> --jq .object`; when `type` is
`tag`, dereference it with
`gh api repos/<owner>/<repo>/git/tags/<sha> --jq .object.sha`.

- [ ] **Step 5: Add the README "Development" section (STE style)**

"Install rustup. Run `cargo xtask check` to do all checks. Run
`cargo run -p uss -- lint` to check the standard files." rustup installs the
pinned toolchain from `rust-toolchain.toml` automatically.

- [ ] **Step 6: Verify locally, push, and verify CI**

Run: `cargo xtask check`
Expected: exit 0.
Then commit, run `git push`, and run `gh run watch --exit-status`.
Expected: the `check` job passes on all three operating systems.

- [ ] **Step 7: Commit** (before the push in Step 6)

```bash
git add -A
git commit -m "ci: add one-command checks, cargo-deny policy, wasm check and CI workflow"
```
