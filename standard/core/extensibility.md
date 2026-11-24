# Extensibility

- **Prefix:** EXT
- **Kind:** area

This area covers software that lets others add features through plugins, and
the marketplace through which plugins are found and installed. Both are
optional: the first rule of each group is a MAY rule that, once a project
adopts it, makes the rest of the group apply. Its rules count toward Features
by default; the rules that keep plugins from harming the host or its users
count toward Safety. The Release (REL) area holds the versions of the
project's own releases (USS-REL-001), the Supply chain (SUP) area holds the
signing and build provenance of the project's own artifacts (USS-SUP-014,
USS-SUP-015 and USS-SUP-016), the Dependencies (DEP) area holds the scanning
of the project's own dependencies (USS-DEP-007), and the Licensing (LIC) area
holds the project's own licence (USS-LIC-001). The rules here apply those
ideas to plugins.

## Plugin interface

### USS-EXT-001: Provide a plugin interface

- **Level:** MAY
- **Applies when:** has-ui or has-cli or is-network-service or is-library
- **Category:** Features
- **Verify:** Adoption means that the software offers an interface through
  which plugins add features. It is adopted when the documentation names the
  interface (such as an extension API, a plugin directory with a loading
  protocol, or a hook system) and the code loads and runs code that a third
  party wrote against it, without a change to the software's own source. Write
  a minimal plugin from the documentation and load it: it adds a feature. An
  interface that exists in the code but that the documentation does not
  describe fails once the MAY is adopted. Adoption (`adopt: true`) enables
  `has-plugin-interface`; the record does not answer it.
- **References:** none
- **Enables:** has-plugin-interface

The software MAY offer an interface through which plugins add features.

**Rationale:** A plugin interface lets others add what the project's team
would not build, and keeps those additions out of the core. It also opens the
software to code that the project did not write, which is why the rules that
follow apply once the interface exists.

### USS-EXT-002: Version the plugin contract

- **Level:** MUST
- **Applies when:** has-plugin-interface
- **Category:** Features
- **Verify:** Find the documentation of the plugin contract (the functions,
  events, data formats and permissions that plugins use). It states a contract
  version, and the version follows Semantic Versioning 2.0.0 (a number of the
  form MAJOR.MINOR.PATCH). It also states the compatibility guarantees: which
  kinds of change keep a plugin that was written for an earlier version working,
  and which kinds do not. In the contract's change history since the last audit,
  or the last 12 months when there is none, each incompatible change, as SemVer
  defines it, raises the MAJOR version; while MAJOR is 0, SemVer item 4 applies
  and this rule adds nothing to it. A contract with no version, no stated
  guarantees, or an incompatible change under a lower increment fails.
- **References:** Semantic Versioning 2.0.0, items 1, 3, 4 and 8

The plugin contract MUST be documented and versioned under Semantic
Versioning, with stated compatibility guarantees.

**Rationale:** Plugin authors write against the contract, not against the
software, so a change that breaks the contract breaks code that the project
cannot see. The version of the contract is separate from the version of the
software (USS-REL-001), because the two change at different rates.

### USS-EXT-003: Require a plugin manifest

- **Level:** MUST
- **Applies when:** has-plugin-interface
- **Category:** Features
- **Verify:** Find the manifest format in the plugin documentation and the
  code that reads it. The format has fields for the plugin's identity, its
  version, the versions of the host that it works with, the permissions that
  it requests and its licence. Open the example plugin, or a plugin from the
  test suite: its manifest fills each field. A format that lacks a field, or
  a plugin whose manifest leaves one empty, fails.
- **References:** Visual Studio Code, Extension Manifest (read 2026-11-24), as
  an example of such a manifest

Each plugin MUST carry a manifest that states its identity, its version, the
host versions it works with, the permissions it requests and its licence.

**Rationale:** The host and its users decide what to load from the manifest,
before any plugin code runs. Without the permissions and the compatible host
versions, they decide blind.

### USS-EXT-004: Grant plugins only the permissions they request

- **Level:** MUST
- **Applies when:** has-plugin-interface
- **Category:** Safety
- **Verify:** Read the code that loads a plugin. It reads the permissions in
  the plugin's manifest, and it grants the plugin only those that the user or
  operator has approved, by an explicit step such as a prompt, a setting or a
  policy file. Test with a plugin that calls an operation that its manifest
  does not request (such as reading a file or opening a connection): the host
  refuses it. Test with a plugin whose requested permission the user declined:
  the host refuses that too. A plugin that can do more than its approved
  permissions allow fails.
- **References:** none

A plugin MUST get only the permissions that its manifest requests and that the
user or operator grants.

**Rationale:** A plugin runs with the trust that the host gives it. A plugin
that gets everything by default turns each plugin bug, and each malicious
plugin, into a compromise of the whole host. The least-privilege rule for the
tools of an AI model (USS-AI-003) follows the same idea for a different kind
of extension.

### USS-EXT-005: Isolate plugins from the host

- **Level:** MUST
- **Applies when:** has-plugin-interface
- **Category:** Safety
- **Verify:** Read the design documentation and the loader code to find how
  plugins are isolated: a separate process, a sandbox, a virtual machine, or a
  runtime with no access to the host's memory. In a test instance, load a
  plugin that crashes, one that loops forever, and one that tries to read the
  host's memory or to change its code or state outside the contract. The host
  keeps running, can stop the plugin, and its own data is unchanged. Software
  that loads plugins into its own address space with no boundary fails.
- **References:** none

A plugin that fails or misbehaves MUST be unable to crash or take over the
host, such as because it runs in its own process or sandbox.

**Rationale:** Plugin code comes from people whom the project does not
review. A boundary that the operating system or the runtime enforces holds
when the plugin is wrong or hostile; a convention that plugins behave does not.

### USS-EXT-006: Manage the plugin lifecycle

- **Level:** MUST
- **Applies when:** has-plugin-interface
- **Category:** Features
- **Verify:** Find the commands or screens through which a user installs,
  enables, disables, upgrades and uninstalls a plugin. Run each against a test
  plugin. Disabling stops the plugin from running without removing it.
  Upgrading replaces its code and keeps its settings. After uninstalling,
  compare the installation with a snapshot taken before the plugin was
  installed (its files, settings entries, database tables, scheduled jobs and
  registered handlers): nothing that the plugin added is left. An operation
  that is missing, or that leaves something behind, fails.
- **References:** none

Plugins MUST be installable, enableable, disableable, upgradeable and
uninstallable, and uninstalling a plugin MUST remove everything that the plugin
added.

**Rationale:** A plugin that cannot be removed cleanly is a cost that each user
pays for a trial. Leftovers also keep data and code on the system that the user
believed was gone.

### USS-EXT-007: Ship an SDK, an example plugin and a test harness

- **Level:** SHOULD
- **Applies when:** has-plugin-interface
- **Category:** Features
- **Verify:** The repository or the release holds an SDK for plugin authors (a
  library or a template that implements the contract's types and helpers), a
  working example plugin, and a test harness that runs a plugin against the
  host or a stand-in for it. Build the example plugin from its instructions
  and run the harness against it in a clean environment: both succeed. A
  missing part, or an example that does not build or run, fails.
- **References:** none

The project SHOULD ship an SDK, a working example plugin and a test harness for
plugin authors.

**Rationale:** The contract is easier to follow than to read. An example that
runs shows the intended use, and a harness lets authors find a break before
their users do.

## Plugin marketplace

### USS-EXT-008: Provide a plugin marketplace

- **Level:** MAY
- **Applies when:** has-plugin-interface
- **Category:** Features
- **Verify:** Adoption means that the project runs or ships a marketplace
  through which users find and install plugins. It is adopted when the
  documentation names the marketplace (a hosted service, or software that an
  operator runs) and the host can list plugins from it and install one with no
  manual file transfer. Install a listed plugin through the host: it works. A
  marketplace that the documentation does not describe fails once the MAY is
  adopted. Adoption (`adopt: true`) enables `has-plugin-marketplace`; the
  record does not answer it.
- **References:** none
- **Enables:** has-plugin-marketplace

The project MAY run or ship a marketplace through which users find and install
plugins.

**Rationale:** A marketplace is where most users meet plugins, and so where the
trust in them is decided. It also makes the project responsible for what it
lists, which is why the rules that follow apply once it exists.

### USS-EXT-009: Sign plugins and verify publishers

- **Level:** MUST
- **Applies when:** has-plugin-marketplace
- **Category:** Safety
- **Verify:** Read the marketplace's publishing process and the host's
  installer code. Each plugin package in the index carries a signature, and
  the process verifies the publisher's identity before the first package of a
  publisher is listed (by the documented method, such as a domain check or an
  identity check). The host checks the signature, against a key that it
  trusts, before it installs or upgrades a plugin. In a test instance, install
  a package with a changed byte, and a package that is signed by a key that
  does not belong to its publisher: the host refuses both. A package without
  a signature, a publisher without a check, or an installer that skips the
  signature check fails.
- **References:** The Update Framework Specification 1.0.37 (read 2026-11-24),
  section 2.1 (Roles and PKI) and section 5 (Detailed client workflow)

Each plugin package MUST be signed, its publisher's identity MUST be verified,
and the host MUST check the signature before it installs the plugin.

**Rationale:** Users install a plugin because of who published it, and a
signature is what ties the package to that publisher. The project's own
release artifacts are signed under USS-SUP-014; this rule is for the packages
that other publishers upload.

### USS-EXT-010: Publish plugin provenance

- **Level:** SHOULD
- **Applies when:** has-plugin-marketplace
- **Category:** Safety
- **Verify:** For each plugin version that the marketplace lists, find a
  provenance attestation that names the package by its digest, the source
  repository and revision it was built from, and the build process (such as a
  provenance document in the SLSA provenance format). The attestation is
  available from the listing or from a documented location. For a sample of
  five listed versions (or all, when there are fewer), the digest in the
  attestation equals the digest of the package. A version without
  provenance, or with a digest that differs, fails.
- **References:** SLSA v1.2, Build: Provenance

Each plugin package SHOULD come with build provenance that links it to its
source, such as SLSA provenance.

**Rationale:** A signature says who published a package, and provenance says
what it was built from. Together they let a user check that the code in the
package is the code in the public source. The build levels for the project's
own artifacts are in USS-SUP-015 and USS-SUP-016.

### USS-EXT-011: Scan and review plugins

- **Level:** MUST
- **Applies when:** has-plugin-marketplace
- **Category:** Safety
- **Verify:** Read the marketplace's publishing pipeline and its review policy.
  The pipeline scans each uploaded plugin version for malware and for known
  vulnerabilities in the plugin's dependencies, and a version that fails the
  scan is not listed. The policy documents a review (who reviews, what they
  check, and what makes them reject a plugin), and the pipeline lists a version
  only after the review is recorded. For the versions listed since the last
  audit, or the last 12 months when there is none, a sample of five (or all,
  when there are fewer) each have a scan result and a review record dated before
  the listing. A version listed without either fails.
- **References:** none

Each plugin version MUST be scanned automatically for malware and known
vulnerabilities, and MUST go through a documented review, before it is listed.

**Rationale:** The marketplace is the last place where the project can stop a
bad plugin before users install it. The scanning of the project's own
dependencies (USS-DEP-007) does not cover the plugins that others upload.

### USS-EXT-012: Keep a versioned index

- **Level:** MUST
- **Applies when:** has-plugin-marketplace
- **Category:** Features
- **Verify:** Find the marketplace's index (a file, an API or a database
  schema). It lists each plugin with each of its versions, and each version
  records the plugin's dependencies on other plugins and the host versions that
  it works with. Request an index entry for a plugin that depends on another:
  the host or the marketplace resolves the dependency and installs a version of
  it that satisfies the stated range. A plugin whose dependency is missing or
  conflicts is refused with a message. An index without versions, or one that
  installs without resolving dependencies, fails.
- **References:** none

The marketplace MUST keep a versioned index of plugins, and MUST resolve the
dependencies between them.

**Rationale:** Users and hosts need to know which versions exist and which of
them work together. An index that records only the latest version cannot
support upgrades, rollbacks or revocation of a single version.

### USS-EXT-013: Deliver plugin updates

- **Level:** MUST
- **Applies when:** has-plugin-marketplace
- **Category:** Features
- **Verify:** Find the update channel in the documentation and the host's
  code: the host asks the marketplace for new versions of the plugins that are
  installed (on a schedule, at start, or on request), and tells the user or
  operator which have updates. Publish a new version of a test plugin: the host
  reports it and applies it on request, or automatically when the setting is
  on. A host that cannot find out about updates, or cannot apply them, fails.
- **References:** The Update Framework Specification 1.0.37 (read 2026-11-24),
  section 5 (Detailed client workflow)

Hosts MUST learn of plugin updates through an update channel, and MUST be able
to apply them.

**Rationale:** An installed plugin that never gets an update keeps its
vulnerabilities. The update channel is also how a fixed version reaches users
after a revocation.

### USS-EXT-014: Revoke bad plugins

- **Level:** MUST
- **Applies when:** has-plugin-marketplace
- **Category:** Safety
- **Verify:** Find the revocation process in the documentation: how a version is
  marked as revoked, and how hosts learn of it. In a test instance, revoke a
  version of a test plugin that is installed: the marketplace stops offering it,
  and the host, at its next check, disables it. A marketplace with no way to
  revoke, or a host that keeps running a revoked version, fails.
- **References:** none

The marketplace MUST be able to revoke a plugin version, and hosts MUST then
disable it.

**Rationale:** A plugin found to be malicious or badly broken is already
installed on users' systems. Removing it from the listing does not remove it
from them; only a signal that the hosts act on does. The Update Framework
revokes keys and delegations, not single target files, so a marketplace that
uses it still needs its own signal for a version; revoking a publisher's key
is guidance, not this rule's test.

### USS-EXT-015: Show each plugin's licence

- **Level:** MUST
- **Applies when:** has-plugin-marketplace
- **Category:** Features
- **Verify:** Open the marketplace's listing for a sample of five plugins (or
  all, when there are fewer), in its user interface or API. Each listing shows
  the plugin's licence as an SPDX licence expression (such as `MIT` or
  `Apache-2.0 OR MIT`). A listing with no licence, one that is free text, or
  one whose identifiers are not on the SPDX License List (or are not
  `LicenseRef-` identifiers that the expression defines) fails.
- **References:** SPDX License List 3.29.0

Each listing MUST show the plugin's licence as an SPDX licence expression.

**Rationale:** Users and their lawyers decide whether they can use a plugin
from its licence, and a machine-readable expression lets tools check it. The
project's own licence is declared under USS-LIC-001.

### USS-EXT-016: Offer search

- **Level:** SHOULD
- **Applies when:** has-plugin-marketplace
- **Category:** Features
- **Verify:** In the marketplace's user interface or API, search for a plugin
  by a word from its name and by a word from its description: the plugin is
  among the results. Browse the listing by category, tag or another grouping:
  it shows plugins. A marketplace with no search, or one whose search does not
  find a listed plugin by its name, fails.
- **References:** none

Users SHOULD be able to search and browse the marketplace.

**Rationale:** A plugin that nobody can find is not used, and a user who
cannot browse cannot compare plugins before choosing one.

### USS-EXT-017: Allow self-hosting and mirroring

- **Level:** SHOULD
- **Applies when:** has-plugin-marketplace
- **Category:** Features
- **Verify:** Find the documentation on running the marketplace on the
  operator's own infrastructure, or on mirroring it. It says how to set up the
  mirror or the self-hosted marketplace and how to point hosts at it. Follow
  it in an environment with no internet access: the mirror serves the index
  and the packages, and a host that is pointed at it installs a plugin, with
  the signature check still working. A marketplace that works only from the
  project's own servers fails.
- **References:** none

The marketplace SHOULD be self-hostable or mirrorable, for use without
internet access.

**Rationale:** Organisations with no outbound access still need plugins. The
offline install of the software itself is in USS-DST-013; this rule is for the
plugins.
