# cargo-deny

- **Kind:** profile
- **Product:** cargo-deny
- **Vendor:** Embark Studios
- **Maintainer:** community
- **Versions:** >=0.22.1, <0.23.0

cargo-deny reads the dependency graph of a Cargo project, `Cargo.lock`
included, and runs four checks on it: `advisories` (the RustSec advisory
database and yanked crates), `licenses` (an allow list of SPDX licences),
`bans` (wildcard requirements and banned crates) and `sources` (the registries
and git repositories that crates may come from). The Rust module recommends it
in USS-RUST-010. Put the baseline below in `deny.toml` in the workspace root
(cargo-deny also reads `.deny.toml`, `.cargo/deny.toml` and
`.config/deny.toml`, or the file named with `--config`); this repository's own
`deny.toml` is an example. The settings that the
mappings name are those of cargo-deny 0.22.1, from its documentation (read
2026-11-24). The licence allow list in the baseline is an example: it is the
project's own policy, and USS-LIC-003 asks for one that is compatible with the
project's licence and distribution. `cargo deny check` runs all four checks,
and its exit status is a bit set of the checks that reported an error. A
`reason` of an ignored advisory is optional in the tool, and a licence
exception has no `reason` field, so the auditor reads each ignore entry and
each exception for its reason.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```toml
[advisories]
version = 2
db-urls = ["https://github.com/rustsec/advisory-db"]
yanked = "deny"
unmaintained = "all"
unused-ignored-advisory = "deny"
# An ignored advisory uses the table form, with a reason and an expiry:
# ignore = [{ id = "RUSTSEC-0000-0000", reason = "...", expiry = "2027-01-31" }]

[licenses]
version = 2
# The project's own policy: list the licences that fit the project's licence.
allow = ["MIT", "Apache-2.0", "Unicode-3.0", "MIT-0", "BSD-3-Clause", "Zlib"]
unused-license-exception = "deny"

[bans]
wildcards = "deny"

[sources]
unknown-registry = "deny"
unknown-git = "deny"
required-git-spec = "rev"
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-DEP-001 | Safety | `bans.wildcards = "deny"` (a `*` requirement), `sources.required-git-spec = "rev"` (a git dependency with a `branch`, a `tag` or none) | partial: the `wildcards` check reports a `*` requirement only, so a lower bound with no upper bound (such as `>=1.2`) is not reported | Run `cargo deny check bans sources`; read each requirement in the manifests for an upper bound. |
| USS-DEP-006 | Safety | `advisories.unmaintained = "all"` (a dependency that an advisory marks as unmaintained, direct or transitive) | partial: the check reads the advisory database only, so a deprecation or end-of-life notice from another source needs a search | Run `cargo deny check advisories`; search for end-of-life notices of the dependencies in the lockfile. |
| USS-DEP-007 | Safety | `advisories` check with `db-urls` (the RustSec database, matched against every package of the resolved graph, transitive ones included) | partial: CI must run the check on each pull request, on each push to the default branch and on a schedule, and vendored code is not read | Read the CI job and its triggers; run `cargo deny check advisories` and expect exit status 0. |
| USS-DEP-008 | Safety | `advisories.ignore` (entries with `reason` and `expiry`), `advisories.unused-ignored-advisory = "deny"` | partial: an advisory that is neither fixed nor ignored fails the check, and the tool does not require a reason, an issue link or the 30-day limit on an ignore entry | Run `cargo deny check advisories`; read each ignore entry for its reason, issue and expiry. |
| USS-RUST-009 | Safety | `bans.wildcards = "deny"` (a `*` requirement), `sources.required-git-spec = "rev"` (a git dependency that does not name a `rev`) | partial: the check reports a `*` requirement only, so a lower bound with no upper bound (such as `>=1.2`) and an upper bound beyond the next incompatible version need a read of the manifests | Run `cargo deny check bans sources`; read each requirement in the manifests. |
| USS-LIC-003 | Compatibility | `licenses.allow` (a licence that is not listed is denied, an unlicensed crate included), `licenses.exceptions`, `licenses.unused-license-exception = "deny"` | partial: the list must be compatible with the project's licence and distribution, which is the project's choice, CI must run the check, and development dependencies are left out by default (`include-dev = false`) | Read `deny.toml` and the CI job; run `cargo deny check licenses` and expect exit status 0. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| An ignore entry that matches no advisory is an error | `advisories.unused-ignored-advisory = "deny"` | USS-DEP-008 |
| A licence exception that matches no crate is an error | `licenses.unused-license-exception = "deny"` | USS-LIC-003 |
