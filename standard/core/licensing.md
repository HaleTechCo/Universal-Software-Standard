# Licensing

- **Prefix:** LIC
- **Kind:** area

This area covers the licence of the software and the licences of what it
includes: declaring the licence, choosing one that others can rely on, and
keeping the licences of dependencies compatible and their notices intact. Its
rules count toward Compatibility by default, because a licence decides whether
others can use and integrate the software. The Dependencies (DEP) area holds
the choice and health of each dependency, and the Distribution (DST) area
holds how releases reach users.

### USS-LIC-001: Declare the licence

- **Level:** MUST
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** The repository root has a licence file (such as `LICENSE` or
  `COPYING`) that holds the licence text. Each package manifest in the
  repository (such as `Cargo.toml`, `package.json` or `pyproject.toml`) has
  a licence field whose value is a valid SPDX licence expression, or its
  language's equivalent of one. A manifest that names a licence file instead
  of an expression, or an expression that is not valid, fails.
- **References:** SPDX Specification 3.0.1, Annex B (SPDX license
  expressions); SPDX License List 3.29.0

The repository root MUST have a licence file, and each package manifest MUST
declare the licence as an SPDX licence expression.

**Rationale:** Without a licence, the law gives others no right to use the
software, and a licence that tools can read lets users and their scanners
decide at once whether they may use it.

### USS-LIC-002: Use an OSI-approved licence

- **Level:** SHOULD
- **Applies when:** publishes-releases
- **Category:** Compatibility
- **Verify:** Take the SPDX identifier of each licence in the manifests and
  the licence file. Each one is on the list of licences that the Open Source
  Initiative has approved. A licence that is not on the list fails, unless a
  Standard Deviation in the record covers it. A proprietary product records
  such a Standard Deviation.
- **References:** OSI, Open Source Licenses (read 2026-11-24); SPDX License
  List 3.29.0

The software SHOULD be released under a licence that the Open Source
Initiative has approved.

**Rationale:** An approved licence has met the Open Source Definition and is
known to users, lawyers and tools, so adopters need not review new terms. A
permissive licence (such as MIT) is the usual choice and gives the widest
reuse, but a copyleft licence is also approved and also passes. A proprietary
product does not meet this rule and records a Standard Deviation.

### USS-LIC-003: Allow only compatible dependency licences

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Compatibility
- **Verify:** A written policy (a file in the repository, or a configuration
  file of a licence checker such as `cargo-deny`, ScanCode or
  `license-checker`) lists the dependency licences that are compatible with
  the project's licence and with the way that the project is distributed. A
  CI job checks every dependency, direct and transitive, against the policy
  and fails on one whose licence the policy does not allow, or whose licence
  is unknown. A run on the default branch passes.
- **References:** SPDX Specification 3.0.1, Annex B (SPDX license
  expressions); ISO/IEC 5230:2020 (OpenChain)

A written policy MUST list the dependency licences that are compatible with
the project's licence and the way it is distributed, and CI MUST fail on a
dependency whose licence the policy does not allow.

**Rationale:** Compatibility depends on both licences and on how the software
is shipped, and a dependency with an incompatible licence can make a release
unlawful. A check in CI finds it when the dependency is added, not when a
user raises it.

### USS-LIC-004: Ship the notices that dependencies require

- **Level:** MUST
- **Applies when:** has-dependencies and publishes-releases
- **Category:** Compatibility
- **Verify:** For one release artifact of each kind that the project
  publishes (such as a package, a binary and a container image), list the
  dependencies that it contains. For each licence of those dependencies that
  requires its text or a notice to be shipped, the artifact includes it, in
  the artifact or in a file that the artifact names. A generated notices file
  that lists every included dependency passes.
- **References:** ISO/IEC 5230:2020 (OpenChain)

Each release artifact MUST include the licence texts and notices that its
dependencies' licences require.

**Rationale:** Most open source licences, permissive ones included, allow
redistribution only if the licence text or an attribution notice goes with
it.
