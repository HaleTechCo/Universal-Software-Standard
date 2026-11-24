# Distribution and install

- **Prefix:** DST
- **Kind:** area
- **Applies when:** publishes-releases

This area covers the ways in which the software reaches its users and stays
current: the install methods it offers, how it installs, upgrades and rolls
back, and what comes with each release artifact. Its rules count toward
Compatibility by default; the install methods count toward Features. The
Release (REL) area holds versions, changelogs and migration guides, the
Supply chain (SUP) area holds signing (USS-SUP-014) and the Backup (BAK) area
holds the backup before an upgrade (USS-BAK-003). The Deployment hardening
(HRD) area holds how published images and manifests are secured.

## Install methods

### USS-DST-001: Offer native packages

- **Level:** MAY
- **Applies when:** has-cli or has-ui or is-network-service
- **Category:** Features
- **Verify:** Adoption means that the release has packages for the package
  managers of the platforms that the software supports. It is adopted when
  the documentation lists the supported platforms, and the release holds a
  package for at least one package manager of each (such as deb or RPM for
  Linux, Homebrew for macOS, winget for Windows), built by the release
  pipeline. Install one package on a clean machine: the software runs. A
  supported platform without a package fails once the MAY is adopted.
- **References:** none

The software MAY be released as packages for the package managers of the
platforms it supports (such as deb, RPM, Homebrew or winget).

**Rationale:** Users trust the package manager they already use, and it gives
them upgrades and removal without extra work.

### USS-DST-002: Offer standalone binaries

- **Level:** MAY
- **Applies when:** has-cli or has-ui or is-network-service
- **Category:** Features
- **Verify:** Adoption means that the release has self-contained binaries or
  archives. It is adopted when the documentation lists the supported
  platforms, and the release holds, for each, a binary or an archive that
  runs after it is unpacked, with no install step and no language runtime or
  library to install first (except those of the operating system). Unpack one
  on a clean machine: it runs. A supported platform without one fails once the
  MAY is adopted.
- **References:** none

The software MAY be released as self-contained binaries or archives for each
platform it supports.

**Rationale:** A binary that runs where it lands suits users who cannot or do
not want to use a package manager, such as in a locked-down environment.

### USS-DST-003: Offer a container image

- **Level:** MAY
- **Applies when:** has-cli or is-network-service
- **Category:** Features
- **Verify:** Adoption means that the release has an OCI container image. It
  is adopted when the repository holds the build file of the image, the
  release pipeline pushes the image to a registry, and the documentation says
  how to pull and run it. Pull the image of the latest release and run it as
  the documentation says: the software starts. The record answers
  `ships-container-image` `true`; a record that answers `false` fails once the
  MAY is adopted.
- **References:** OCI Image Format Specification v1.1.1

The software MAY be released as an OCI container image.

**Rationale:** An image runs the same on any host with a container runtime,
and it carries the software's dependencies with it.

### USS-DST-004: Offer a Compose file

- **Level:** MAY
- **Applies when:** is-network-service
- **Category:** Features
- **Verify:** Adoption means that the release has a Compose file. It is
  adopted when the repository or the release holds a Compose file that the
  documentation names. It passes `docker compose config` (or the same check
  of another Compose implementation), and `docker compose up` on a clean
  machine starts the software and each service it needs, so that the software
  answers a request. A Compose file that does not start fails once the MAY is
  adopted.
- **References:** Compose Specification (read 2026-11-24)

The software MAY be released with a Compose file that runs it and the services
it needs.

**Rationale:** One file turns a long list of set-up steps into one command, for
a trial or a small deployment.

### USS-DST-005: Offer a Helm chart

- **Level:** MAY
- **Applies when:** is-network-service
- **Category:** Features
- **Verify:** Adoption means that the release has a Helm chart. It is adopted
  when the repository holds a chart (a directory with `Chart.yaml`) that the
  documentation names, and the release publishes it (a chart repository or an
  OCI registry). `helm lint` passes on it, and `helm template` renders it
  without error with its default values. The record answers
  `deploys-to-kubernetes` `true`; a record that answers `false` fails once the
  MAY is adopted.
- **References:** Helm documentation, Chart Best Practices (read 2026-11-24)

The software MAY be released with a Helm chart.

**Rationale:** Helm is the common way to install software on Kubernetes, and a
chart lets users set their own values without editing manifests.

### USS-DST-006: Offer an IDE extension

- **Level:** MAY
- **Applies when:** is-developer-tool
- **Category:** Features
- **Verify:** Adoption means that the release has an extension for an IDE or
  editor. It is adopted when the repository holds the extension, with the
  manifest that the IDE's extension format needs, and the release publishes it
  (to the IDE's marketplace or as a file in the release) for at least one
  widely used IDE or editor. Install it from that source into the IDE: it
  loads without error and offers the functions that the documentation names.
- **References:** none

The software MAY be released as an extension for at least one widely used IDE
or editor.

**Rationale:** A developer tool is used most where the developer already works,
and an extension puts it there.

### USS-DST-007: Offer an MCP server

- **Level:** MAY
- **Applies when:** has-cli or exposes-network-api
- **Category:** Features
- **Verify:** Adoption means that the software has an MCP server. It is
  adopted when the repository or the release holds the server, and the
  documentation says how to run it and how to connect an MCP client to it.
  Connect an MCP client (such as the MCP Inspector) to it: the connection
  completes, and the client lists the functions that the documentation names.
  The record answers `exposes-mcp-server` `true`; a record that answers
  `false` fails once the MAY is adopted.
- **References:** Model Context Protocol Specification, revision 2026-07-28

The software MAY offer its functions to AI agents through an MCP server.

**Rationale:** MCP is the common way for AI agents to call a program's
functions, so a server lets agents use the software without custom glue.

### USS-DST-008: Offer an agent skill

- **Level:** MAY
- **Applies when:** has-cli or exposes-network-api
- **Category:** Features
- **Verify:** Adoption means that the release has an agent skill or plugin.
  It is adopted when the repository holds it, in the format of at least one
  coding agent whose documentation the project names, and the release or the
  documentation says how to install it. Open it: it says when an agent is to
  use the software and gives the commands or calls to use, and the agent that
  it targets loads it without error.
- **References:** Claude Code documentation, Extend Claude with skills (read
  2026-11-24), as an example of an agent-skill format

The software MAY be released with an agent skill or plugin that tells coding
agents how to use it.

**Rationale:** A coding agent that has the skill uses the software correctly
the first time, instead of guessing from the command's help text.

## Install and upgrade

### USS-DST-009: Document install, upgrade and uninstall

- **Level:** MUST
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** List the install methods that the release offers (each kind of
  package, binary or archive, image, Compose file, chart or extension in the
  latest release). For each, the documentation has steps to install, steps to
  upgrade and steps to uninstall. A method that lacks one of the three fails.
- **References:** none

For each install method offered, the documentation MUST give the steps to
install, to upgrade and to uninstall.

**Rationale:** Users meet each of the three, and the ones that are left out are
the ones that go wrong: an upgrade that breaks, or an uninstall that leaves
data and services behind.

### USS-DST-010: Upgrade in place

- **Level:** MUST
- **Applies when:** has-cli or has-ui or is-network-service
- **Category:** Compatibility
- **Verify:** Install the previous release on a clean machine and give it
  configuration and data. Install the latest release over it, as the
  documentation says, with no uninstall first. The installation detects the
  existing one and upgrades it: the version is the new one, and the
  configuration and the data are as they were. An upgrade that needs an
  uninstall first, or that loses either, fails.
- **References:** none

Installing a new version over an existing install MUST detect it and upgrade
it in place, keeping its configuration and data.

**Rationale:** Users upgrade often. An upgrade that asks them to remove the
old version first, or that starts from empty, is one they put off.

### USS-DST-011: Carry configuration forward

- **Level:** MUST
- **Applies when:** has-cli or has-ui or is-network-service
- **Category:** Compatibility
- **Verify:** Take the supported earlier versions: those that the support
  policy names, or the releases of the last 12 months where there is none.
  For each, take its documented configuration (the sample from that release,
  with its options set). Upgrade a test instance that holds it to the latest
  release: the software starts and behaves as that configuration says, either
  because the latest release reads the old configuration as it is, or because
  the upgrade converts it by itself, with no manual edit. A configuration that
  needs a manual edit fails.
- **References:** none

Configuration from every supported earlier version MUST keep working after an
upgrade, or the upgrade MUST migrate it automatically.

**Rationale:** Configuration is where users keep their choices. Losing it makes
each upgrade a rework. A change to stored data belongs to USS-DAT-001.

### USS-DST-012: Support rollback

- **Level:** SHOULD
- **Applies when:** has-cli or has-ui or is-network-service
- **Category:** Compatibility
- **Verify:** The documentation has steps to roll back an upgrade to the
  previous version together with its data. Upgrade a test instance from the
  previous release, as the documentation says, then follow the rollback
  steps: the previous version runs, and its configuration and data are as they
  were before the upgrade. Rollback steps that fail, or that leave data from
  the newer version that the previous one cannot read, fail.
- **References:** none

An upgrade SHOULD be able to be rolled back to the previous version together
with its data, by documented steps. The backup before an upgrade
(USS-BAK-003) is what makes this possible for persistent data.

**Rationale:** A release that turns out wrong on a user's machine needs a way
back that does not cost them their data.

### USS-DST-013: Support offline install

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** List the install methods that the release offers (as in
  USS-DST-009). For each, the documentation has a way to install without
  internet access: the files to fetch beforehand (packages, archives, images,
  charts, with their dependencies) and the steps to install from them. Follow
  one method's steps on a machine without internet: the software installs and
  runs. A method without such steps fails.
- **References:** none

Each install method SHOULD have a documented way to install without internet
access.

**Rationale:** Many sites, such as secure networks and factory floors, have no
route to the internet. A method that needs one is closed to them.

### USS-DST-014: Build for x86-64 and ARM64

- **Level:** SHOULD
- **Applies when:** has-cli or has-ui or is-network-service
- **Category:** Compatibility
- **Verify:** List the operating systems that the documentation says the
  software supports. For each, take the release artifacts that are specific to
  a processor architecture (binaries, packages, archives, image variants). Each
  has a build for x86-64 and a build for ARM64, in the latest release. An
  operating system with an artifact for one of the two only fails. A
  multi-architecture image counts as one artifact with both.
- **References:** none

Each release artifact that is specific to a processor architecture SHOULD be
built for both x86-64 and ARM64 on each supported operating system.

**Rationale:** ARM64 is common in laptops, servers and cloud instances. A
release without it leaves those users to build the software themselves.

## Release artifacts

### USS-DST-015: Explain how to verify signatures

- **Level:** SHOULD
- **Applies when:** distributed-as-package or ships-container-image
- **Category:** Compatibility
- **Verify:** List the kinds of release artifact (each package, archive,
  binary and image of the latest release). The release documentation (the
  README, the release notes or an install page) says, for each, how to verify
  its signature: the command to run (such as `cosign verify`) and the key or
  signer identity to check against. A kind of artifact without such
  instructions fails, an unsigned one included.
- **References:** Sigstore documentation, Verifying signatures (read
  2026-11-24), as an example

The release documentation SHOULD say how to verify the signature of each
release artifact (USS-SUP-014).

**Rationale:** A signature that users do not know how to check protects no one.
Signing itself is USS-SUP-014; this rule makes the signature usable.

### USS-DST-016: Name the project's own images by digest

- **Level:** SHOULD
- **Applies when:** ships-container-image
- **Category:** Compatibility
- **Verify:** List each chart, manifest and Compose file that the project
  publishes (in the repository for users to use, or in a release or a chart
  repository), the default values of a chart included. Each reference in them
  to an image that the project builds includes its content digest (such as
  `@sha256:...`). A reference by tag only fails. References to images that the
  project does not build are USS-SUP-004's.
- **References:** none

Each chart, manifest or Compose file that the project publishes SHOULD name the
project's own images by digest.

**Rationale:** A tag names whatever was last pushed to it. A digest names the
image that the release was tested with, so a user gets the same bytes the
project shipped.
