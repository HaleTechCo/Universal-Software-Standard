# Release

- **Prefix:** REL
- **Kind:** area
- **Applies when:** publishes-releases

This area covers how the project numbers, tags, supports and describes its
releases, how it handles breaking changes, and how each release is audited.
Its rules count toward Maintainability by default; the rules on breaking
changes count toward Compatibility. Signing release artifacts is
USS-SUP-014, in the Supply chain (SUP) area. The Distribution (DST) area
holds how releases are installed and upgraded, and the Security (SEC) area
holds the security policy and its advisories (USS-SEC-031).

## Versions

### USS-REL-001: Use Semantic Versioning

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** The documentation defines the public API: it lists what a user
  can depend on, in each of these that the software has (a library's exported
  interface, the commands and options of a command-line interface, the
  configuration keys, the format of stored data). Take the releases of the
  last 12 months, or since the last audit, and compare each with the one
  before it. Each version has the form MAJOR.MINOR.PATCH. A release with an
  incompatible change to the public API raises MAJOR (or MINOR while MAJOR is
  0), a release with only added features raises MINOR, and a release with
  only fixes raises PATCH. A release that does not follow this fails.
- **References:** Semantic Versioning 2.0.0, item 1

Releases MUST follow Semantic Versioning 2.0.0, and the documentation MUST
define the public API, including any command-line interface, configuration and
stored-data format.

**Rationale:** A version number is a promise about what an upgrade breaks.
The promise only means something when users can read what the public API
is.

### USS-REL-002: Tag each release

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** Take each release of the last 12 months, or since the last
  audit. The repository has a tag whose name is the version, with or without
  a leading `v`. The tag points to the commit that the release was built from
  (the commit that the release workflow ran on, or that the release page
  names). A release without a tag, or a tag on another commit, fails.
- **References:** Semantic Versioning 2.0.0, FAQ (`git tag v1.2.3`)

Each release MUST have a tag in the repository, on the commit it was built
from, named after its version, with or without a leading `v`.

**Rationale:** The tag is how anyone finds the source of a release, to
rebuild it, to read it or to diff it against the next one.

### USS-REL-003: Make release builds reproducible

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** The documentation gives the steps to rebuild the release
  artifacts from a tag (the tools and their versions, the environment, the
  commands). Follow them on a clean machine for the latest release, and
  compare each rebuilt artifact with the published one by checksum: all are
  identical. An artifact that differs, or a missing step, fails.
- **References:** Reproducible Builds, Definitions (read 2026-11-24)

Anyone SHOULD be able to rebuild each release artifact from its tag, bit for
bit, by documented steps.

**Rationale:** When a rebuild gives the same bytes, a user can check that the
published artifact comes from the tagged source and nothing else. This differs
from the build levels of USS-SUP-015 and USS-SUP-016, which are about how the
build is run and attested.

### USS-REL-004: Publish a support policy

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** The repository or its documentation has a published support
  policy (such as in `SECURITY.md`, `SUPPORT.md` or a page on the website).
  It names the versions that get fixes and the versions that get security
  fixes, and gives for each an end: a date, or a rule that yields a date (such
  as 12 months after the next major release). A policy without the versions,
  or without an end, fails.
- **References:** none

A published policy MUST say which versions get fixes, security fixes included,
and until when.

**Rationale:** Users decide when to upgrade by knowing when their version stops
getting fixes. How quickly the project answers a report is USS-SEC-030.

## Changelog

### USS-REL-005: Follow Common Changelog

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** The repository root has `CHANGELOG.md`. Read it against the
  Common Changelog guide: it begins with `# Changelog`, has an entry for each
  release tag of the last 12 months, or since the last audit, newest first,
  and each entry has a heading of the form `## VERSION - DATE`. Changes sit
  under the guide's categories, in its order, with its wording and
  references, and breaking changes are marked as the guide says. A release
  without an entry, or a file that departs from the guide, fails.
- **References:** Common Changelog (no version number; read 2026-11-24),
  sections 2.1 to 2.4

CHANGELOG.md MUST follow Common Changelog.

**Rationale:** A changelog written for people, in one known form, lets users
and tools find what changed, and which changes break, without reading the
history.

### USS-REL-006: Cite the advisory for each security fix

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** Take the changelog entries of the last 12 months, or since the
  last audit, that describe a security fix (the word "security" or
  "vulnerability" in the entry, or a fix that closes a published advisory).
  Each names an advisory ID or a CVE ID. A security fix whose entry names
  neither fails.
- **References:** GitHub Docs, About repository security advisories (read
  2026-11-24), as an example

Each changelog entry for a security fix MUST name its advisory or CVE ID.

**Rationale:** The ID links the fix to the advisory that tells users how bad
the flaw was and which versions have it. Publishing the advisory is
USS-SEC-031.

## Breaking changes

### USS-REL-007: Avoid breaking changes

- **Level:** SHOULD NOT
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** Take the changelog entries of the last 12 months, or since the
  last audit. No entry is marked as breaking (Common Changelog marks it with
  `**Breaking:**`), and no release raises MAJOR for an incompatible change to
  the public API (USS-REL-001). A release that contains a breaking change
  fails.
- **References:** Semantic Versioning 2.0.0

A release SHOULD NOT introduce a breaking change.

**Rationale:** Each breaking change costs every user work. Where a change
cannot be avoided, the rules that follow limit the cost.

### USS-REL-008: Ship a migration guide with each breaking release

- **Level:** MUST
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** Take each release of the last 12 months, or since the last
  audit, that has a breaking change (an entry marked breaking in the
  changelog, or a raised MAJOR). A migration guide comes with it, linked from
  the release notes or the changelog. It starts from the previous major
  version, and gives a step for each breaking change that the changelog lists.
  A breaking release without such a guide fails.
- **References:** none

Each release with a breaking change MUST come with a migration guide that says
how to move from the previous major version.

**Rationale:** The people who made the change know what it takes to adapt.
Writing that down once saves each user from finding it out alone.

### USS-REL-009: Deprecate before removing

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** Take each feature that a release of the last 12 months, or since
  the last audit, removed (the changelog's removed entries). An earlier
  release, at least one before the removing one, deprecated it: the code of
  that release has a warning that appears when the feature is used (a log
  line, a console message or an API warning), and the documentation or the
  changelog says it is deprecated. A removed feature that was not deprecated
  this way fails.
- **References:** none

A feature SHOULD be deprecated, with a run-time warning, in at least one
release before the release that removes it.

**Rationale:** A warning that users see while they run the software gives them
time to move. A note in a changelog is read only by those who look.

### USS-REL-010: Automate migrations

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** Take each breaking change of the last 12 months, or since the
  last audit (USS-REL-007 gives how to find them). The repository or the
  release holds an automated migration for it, such as a codemod, a script or
  a configuration converter, and the migration guide names it. Run it on a
  sample of the old form (code, configuration or a file): the output works
  with the new version. A breaking change without one fails.
- **References:** none

Each breaking change SHOULD come with an automated migration, such as a
codemod or a configuration converter.

**Rationale:** A tool applies the change the same way everywhere and costs a
user minutes instead of hours. Migrating stored data is USS-DAT-001.

## Release audit

### USS-REL-011: Audit each release

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** Run `uss release-check` on the release commit, with the full
  history fetched (conventions, section "Release audits"). It passes: the
  latest audit of the root record and of each component record is of the
  release commit, or of a commit whose only later changes are under `.uss/`,
  and the audit is of the USS version that the record pins. A failure of the
  command fails the rule.
- **References:** none

Each release MUST have an audit, against the USS version that the record pins,
of the release commit or of a commit whose only later changes are under
`.uss/`.

**Rationale:** An audit of some earlier state says nothing about the release
that users receive. The audit of the release commit itself does.

### USS-REL-012: Publish the audit report with each release

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** Open the latest release (the release page or the host API;
  without access, record `unverified`). It carries the audit report as an
  asset or in its notes, and the text equals the report that `uss
  release-check` writes for the release commit. A hand-written or edited
  report, or a release without one, fails.
- **References:** none

The audit report for each release MUST be generated by `uss` and published with
the release.

**Rationale:** Users and reviewers see what was checked and what was not,
without trusting the project's own summary. A generated report cannot be
edited to look better.
