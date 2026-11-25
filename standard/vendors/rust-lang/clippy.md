# Clippy

- **Kind:** profile
- **Product:** Clippy
- **Vendor:** Rust Project
- **Maintainer:** community
- **Versions:** >=1.100.0, <1.101.0

Clippy is the linter of the Rust toolchain. It runs with `cargo clippy`,
together with the compiler's own lints, and ships with each Rust release, so
its version is the Rust release's. A new release can rename, move or add a
lint, so the range holds one release. The Rust module names it in
USS-RUST-001. Put the baseline below in the lint tables of `Cargo.toml`: in
`[workspace.lints.rust]` and `[workspace.lints.clippy]` of the workspace root,
with `[lints] workspace = true` in each member crate, or in `[lints.rust]` and
`[lints.clippy]` of a single crate. The baseline sets the `missing_docs` lint
for libraries only; a project that is not a library leaves that line out. The
lints that the mappings name are those of Clippy 0.1.100 (Rust 1.100.0), from
the Clippy lint list (rust-lang.github.io/rust-clippy, read 2026-11-24) and the
rustc lint list. A finding at deny level makes `cargo clippy` exit with a
non-zero status (101 when Cargo fails the build). The `all` and `pedantic`
groups are denied at priority -1 so that a lint set at the default priority
overrides them.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```toml
[workspace.lints.rust]
# Libraries only: leave this line out of a project that is not a library.
missing_docs = "deny"
unfulfilled_lint_expectations = "deny"
unused = { level = "deny", priority = -1 }

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
allow_attributes = "deny"
allow_attributes_without_reason = "deny"
dbg_macro = "deny"
expect_used = "deny"
let_underscore_must_use = "deny"
panic = "deny"
todo = "deny"
undocumented_unsafe_blocks = "deny"
unimplemented = "deny"
unused_result_ok = "deny"
unwrap_used = "deny"
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-RUST-006 | Safety | `clippy::undocumented_unsafe_blocks` (an `unsafe` block or impl with no `// SAFETY:` comment) | partial: the lint finds a missing comment and does not judge what the comment says, and it skips blocks that come from external macros | Run `cargo clippy`; read each `SAFETY` comment for the reason. |
| USS-SAF-003 | Safety | `clippy::undocumented_unsafe_blocks` | partial: the lint checks that a safety comment exists, not that it says why the code is sound, and it does not check that a block wraps only the operation that needs it | Run `cargo clippy`; read each `unsafe` use, its comment and its size. |
| USS-CON-001 | Maintainability | `clippy::todo` (`todo!`), `clippy::unimplemented` (`unimplemented!`) | partial: the lints find the two macros only, so stub functions that return a constant, `TODO` markers and sample data returned as a result need a search | Run `cargo clippy`; search the production code for the other placeholders. |
| USS-DOC-005 | Maintainability | `missing_docs` (a public item with no documentation), `clippy::missing_safety_doc` (a public `unsafe fn` with no `# Safety` section) | partial: the lints check that documentation exists, not what it says, and `missing_docs` needs the baseline line for libraries | Run `cargo clippy`; read the documentation of the public items. |
| USS-QLT-001 | Maintainability | the lint tables of the baseline | partial: CI must run `cargo clippy` with warnings as errors and fail on any violation, and Clippy lints Rust only | Read the CI job; run `cargo clippy --all-targets --all-features` and expect exit status 0. |
| USS-QLT-003 | Maintainability | the `deny` levels of the baseline | partial: the baseline denies only the lints that it names, so CI must also turn the other warnings into errors (`CARGO_BUILD_WARNINGS=deny` or `-D warnings`), and the other compilers and build tools of the project need the same | Read the CI job for the setting; run `cargo clippy` and expect no warning. |
| USS-QLT-004 | Maintainability | `unused` group of the compiler (`dead_code`, `unused_imports`, `unused_variables`, `unreachable_code`) | partial: `dead_code` does not report a public item of a library, and commented-out code is not covered | Run `cargo clippy`; search for commented-out code. |
| USS-QLT-005 | Maintainability | `unused_must_use` (in the `unused` group), `clippy::let_underscore_must_use`, `clippy::unused_result_ok` | partial: the lints find a dropped `#[must_use]` value, and an error that another form drops (such as an `if let Ok(..)` with no `else`, or a `map_err` closure that ignores the error) needs a review | Run `cargo clippy`; search for places that drop an error. |
| USS-QLT-006 | Maintainability | the lint tables of the baseline, with none of its lints set lower | partial: the project's lint levels must be at least those of the baseline, and a lint turned off for the whole project is judged by USS-QLT-007 | Compare the project's lint tables, `rustflags` and crate attributes with the baseline above. |
| USS-QLT-009 | Maintainability | `clippy::allow_attributes_without_reason`, `clippy::allow_attributes` | partial: the lints read `allow` and `expect` attributes only, they do not judge the text of a reason, and suppressions of other tools are outside them | Run `cargo clippy`; read each reason and search for suppressions of other tools. |
| USS-QLT-011 | Maintainability | `clippy::allow_attributes` (reports `#[allow]` and suggests `#[expect]`) | partial: the lint reports outer attributes only, so a crate-level `#![allow]` is not reported, and other languages of the project need their own check | Run `cargo clippy`; search for `#![allow` in the source. |
| USS-QLT-012 | Maintainability | `unfulfilled_lint_expectations` (a `#[expect]` whose lint does not fire) | partial: the lint covers `#[expect]` only, so a crate-level `#![allow]` that suppresses nothing is not reported, and the stale suppressions of other tools need their own check | Run `cargo clippy`; search for `#![allow` in the source. |
| USS-RUST-001 | Maintainability | the lint tables of the baseline | partial: CI must run `cargo clippy` on all targets and features with warnings as errors, and the project's lint levels must be those of the baseline | Read the CI job and the lint tables; run `cargo clippy --all-targets --all-features` and expect exit status 0. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| An `#[allow]` attribute is reported, so a suppression uses `#[expect]` | `clippy.allow_attributes = "deny"` | USS-QLT-011 |
| An `allow` or `expect` attribute needs a reason | `clippy.allow_attributes_without_reason = "deny"` | USS-QLT-009 |
| An `#[expect]` attribute that suppresses nothing is an error | `rust.unfulfilled_lint_expectations = "deny"` | USS-QLT-012 |
