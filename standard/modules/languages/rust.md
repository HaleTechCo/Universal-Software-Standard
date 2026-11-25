# Rust

- **Prefix:** RUST
- **Kind:** module
- **Applies when:** lang.rust

This module covers Rust projects: Clippy and rustfmt, the compiler's safety
checks and unsafe code, the pinned toolchain, and how the project keeps
`Cargo.toml` and `Cargo.lock`. Clippy and rustfmt ship with the toolchain, so
their rules are MUSTs, not tool SHOULDs; Clippy has no separate tool rule, and
USS-RUST-001 names it directly. This repository's own `Cargo.toml` lint table
is an example of the Clippy baseline. The module uses two vendor profiles:
`standard/vendors/rust-lang/clippy.md` and
`standard/vendors/embarkstudios/cargo-deny.md`. It names Miri without a
profile. USS-RUST-003 and USS-RUST-004 are the Rust safety checks that
USS-SAF-001 makes binding. The core rules that this module does not refine
apply as written.

## Checks

### USS-RUST-001: Lint with Clippy

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001, USS-QLT-003
- **Verify:** CI runs `cargo clippy` on all targets and all features (such as
  `--all-targets --all-features`; a workspace project can add
  `--workspace`), and turns warnings into errors: `-D warnings` after `--`,
  the environment variable `CARGO_BUILD_WARNINGS=deny`, or
  `build.warnings = "deny"` in a Cargo configuration file. The job fails when
  the command exits with a non-zero status (it does so when a lint at deny
  level fires, or when a warning is an error). The effective lint levels set
  each lint of the baseline of the Clippy profile
  (`standard/vendors/rust-lang/clippy.md`) that the profile applies to the
  project at `deny` or `forbid`. Cargo's `[lints]` or `[workspace.lints]`
  tables, `rustflags` in a Cargo configuration file, command-line `-D` flags
  and crate attributes (`#![deny(...)]`) are all places that set a level. A run
  on the default branch reports no finding.
- **References:** The Clippy Book, Usage (read 2026-11-24); The rustc Book,
  Lints, lint levels (read 2026-11-24); The Cargo Book, The Manifest Format,
  the `[lints]` section (read 2026-11-24)

CI MUST run `cargo clippy` on all targets and features with warnings as
errors, at the lint levels of the Clippy profile
(`standard/vendors/rust-lang/clippy.md`).

**Rationale:** Clippy ships with the toolchain and finds correctness, style and
performance problems that the compiler does not report. Its lints that are off
by default hold the checks that keep unwrap calls, placeholders and unexplained
suppressions out of the code, so the levels matter as much as the run.

### USS-RUST-002: Check formatting with rustfmt

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-002
- **Verify:** CI runs `cargo fmt --check` (a workspace project can add `--all`),
  and the job fails when it exits with a non-zero status (`rustfmt` exits with
  status 1 in `--check` mode when it would change a file).
- **References:** rustfmt README (rust-lang/rustfmt, read 2026-11-24)

CI MUST run `cargo fmt --check`.

**Rationale:** rustfmt ships with the toolchain and fixes one format, so a
format check ends style arguments in review and keeps each diff to the change
itself.

### USS-RUST-003: Forbid unsafe code where none is needed

- **Level:** MUST
- **Applies when:** not has-memory-unsafe-code
- **Category:** Safety
- **Refines:** USS-SAF-001
- **Verify:** For each crate of the project, the effective lint table sets
  `unsafe_code = "forbid"`: the `[lints.rust]` table of its `Cargo.toml`, or
  the `[workspace.lints.rust]` table of the workspace root that the crate
  inherits with `workspace = true` under `[lints]`. A crate whose table sets
  another level, or none, fails.
- **References:** The Cargo Book, The Manifest Format, the `[lints]` section
  (read 2026-11-24); The rustc Book, Lints, lint levels (read 2026-11-24)

A crate with no unsafe code MUST set `unsafe_code = "forbid"` in its lint
table.

**Rationale:** `forbid` makes the compiler reject each `unsafe` block, and an
attribute in the source cannot lower it, so a crate that needs none cannot gain
unchecked code by accident or in a later change.

### USS-RUST-004: Keep overflow checks in release builds

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SAF-001
- **Verify:** The release profile sets `overflow-checks = true`: in
  `[profile.release]` of the workspace root `Cargo.toml` (Cargo reads profile
  settings only from that manifest), or in a Cargo configuration file or the
  environment variable `CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS` that the release
  build uses (which override the manifest). A release profile that leaves the
  setting out (its default is `false`) or sets it to `false` fails.
- **References:** The Cargo Book, Profiles, overflow-checks (read 2026-11-24);
  CWE-190

The release profile MUST set `overflow-checks = true`.

**Rationale:** Without the check, integer arithmetic in a release build wraps
around without notice, and a wrapped size or index becomes a wrong result or a
memory error. The check turns the overflow into a panic at the line where it
happens.

### USS-RUST-005: Run Miri on unsafe code

- **Level:** MUST
- **Applies when:** has-memory-unsafe-code
- **Category:** Safety
- **Refines:** USS-SAF-004
- **Verify:** A CI step runs the tests that exercise the project's unsafe code
  under Miri (such as `cargo miri test`, which may select tests by name). A
  step that leaves out a test that exercises unsafe code fails.
- **References:** Miri README (rust-lang/miri, read 2026-11-24)

CI MUST run the tests that exercise unsafe code under Miri.

**Rationale:** Miri runs a test in an interpreter that detects undefined
behaviour, such as an out-of-bounds access, a use after free or a misaligned
read, and the compiler cannot check that in unsafe code. Miri finds a problem
only in an execution that a test runs, so the tests have to reach the unsafe
code.

### USS-RUST-006: Write a SAFETY comment for each unsafe block

- **Level:** MUST
- **Applies when:** has-memory-unsafe-code
- **Category:** Safety
- **Refines:** USS-SAF-003
- **Verify:** Search the project's own code for each `unsafe` block. Each one
  has a comment that starts with `// SAFETY:` and says why the operations in
  the block are sound: the condition that makes them safe and why it holds. A
  block with no such comment, or with a comment that does not say why, fails.
- **References:** The Rust Reference, Unsafe blocks (read 2026-11-24)

Each `unsafe` block MUST have a `// SAFETY:` comment that says why it is
sound.

**Rationale:** The compiler does not check what is inside an `unsafe` block,
so the reviewer has to. A reason in the comment says what the author relied
on, and a wrong reason is visible there.

## Toolchain

### USS-RUST-007: Pin the toolchain in rust-toolchain.toml

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-001
- **Verify:** `rust-toolchain.toml` exists and its `channel` names an exact
  release with three numbers (such as `1.100.0`). A channel of `stable`,
  `beta` or `nightly`, or a release with no patch number (such as `1.100`),
  fails. Its `components` list holds each component that the CI steps use
  (such as `rustfmt` for `cargo fmt` and `clippy` for `cargo clippy`).
- **References:** The rustup Book, Overrides, the toolchain file (read
  2026-11-24)

`rust-toolchain.toml` MUST pin an exact Rust release, such as `1.100.0`, with
the components that CI uses.

**Rationale:** Rustup reads the file, so every local build and CI run uses the
same compiler, Clippy and rustfmt. A channel such as `stable` moves with each
release and changes what the lints report without any change to the code.

## Dependencies

### USS-RUST-008: Build with --locked

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-002, USS-DEP-003
- **Verify:** `Cargo.lock` is in version control at the workspace root, for a
  library as well as for a binary. Each CI step that builds or tests the
  project passes `--locked` (or `--frozen`, which is `--locked` with
  `--offline`). A step that builds or tests without either flag fails.
- **References:** The Cargo Book, cargo build, the `--locked` option (read
  2026-11-24); The Cargo Book, FAQ, Why have `Cargo.lock` in version control
  (read 2026-11-24)

`Cargo.lock` MUST be committed, libraries included, and CI MUST build and test
with `--locked`.

**Rationale:** With `--locked`, Cargo stops with an error when it would have to
change `Cargo.lock`, so CI builds what the lockfile records and fails when the
lockfile is out of date. A library's lockfile does not reach its users, but it
fixes what the library's own CI tests.

### USS-RUST-009: Bound each version requirement

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-001
- **Verify:** In each `Cargo.toml` of the project (the dependency, development
  dependency and build dependency tables, the target-specific tables and
  `[workspace.dependencies]`), each requirement is read by Cargo's rules and
  has an upper bound, written or implied by the form of the requirement, below
  the next incompatible version, so the range does not admit it: the next major
  version, or the next minor version for a `0.x` version. The default caret
  form (`1.2` implies `<2.0.0`, `^1.2`, `0.3` implies `<0.4.0`), a tilde, an
  exact `=` and a range with such a written bound pass. `*`, a lower bound
  with no upper bound (such as `>=1.2`) and an upper bound beyond the next
  incompatible version (such as `<3` for a `1.x` version) fail. Each git
  dependency names a `rev`; one that names a `branch` or a `tag`, or none of
  the three, fails.
- **References:** The Cargo Book, Specifying Dependencies (read 2026-11-24)

Each dependency MUST have a requirement with an upper bound, written or
implied by the requirement's form, below the next incompatible version (Cargo's
default caret form `1.2` implies `<2.0.0`), and each git dependency MUST name a
`rev`.

**Rationale:** A requirement with no upper bound accepts a new incompatible
version, and a branch or tag of a git repository moves, so the same manifest
can build different code with no change to the project. A `rev` names one
commit. Cargo treats a change of the leftmost non-zero component of a version
as incompatible, so for a `0.x` version the next minor version is the next
incompatible version. This is Cargo's reading: the rules for other ecosystems
bound a range below the next major version.

### USS-RUST-010: Check dependencies with cargo-deny

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-001, USS-DEP-007, USS-LIC-003
- **Verify:** CI runs `cargo deny check` (with no check named, it runs the
  advisories, licenses, bans and sources checks), and the job fails when it
  exits with a non-zero status (the status is a bit set of the checks that
  reported an error). The `deny.toml` that it reads (or the file named with
  `--config`) includes the baseline of the cargo-deny profile
  (`standard/vendors/embarkstudios/cargo-deny.md`). A project that uses
  another tool fails this rule (it records a Standard Deviation) and meets the
  outcome rule through that tool, configured to at least the baseline that
  USS-QLT-006 sets.
- **References:** cargo-deny Book, the `check` command and the configuration
  pages (read 2026-11-24)

The project SHOULD check advisories, licences, bans and sources with
cargo-deny, configured per its vendor profile
(`standard/vendors/embarkstudios/cargo-deny.md`).

**Rationale:** One tool, with one configuration file, reads `Cargo.lock` and
checks every dependency, direct and transitive, against the advisory database,
a licence policy, a ban list and a list of allowed sources.

### USS-RUST-011: Test the minimal versions

- **Level:** SHOULD
- **Applies when:** has-dependencies and is-library
- **Category:** Safety
- **Refines:** USS-DEP-017
- **Verify:** A CI job resolves each direct dependency to the lowest version
  that its requirement allows, and builds and tests the library with that
  resolution. The `-Z direct-minimal-versions` flag of Cargo does this (such as
  `cargo +nightly update -Z direct-minimal-versions`, then a build and a test
  with `--locked`); it is unstable, so it needs a nightly toolchain. Any step
  that resolves direct dependencies to their lowest allowed versions meets the
  rule.
- **References:** The Cargo Book, Unstable Features, direct-minimal-versions
  (read 2026-11-24)

CI SHOULD build and test the library with the lowest versions that the
requirements allow for direct dependencies.

**Rationale:** A lower bound in `Cargo.toml` claims that the library works
with that version, and only a build with that version shows whether the claim
holds. The flag resolves only direct dependencies to their minimum, so a
transitive dependency with a wrong lower bound does not block the check.
