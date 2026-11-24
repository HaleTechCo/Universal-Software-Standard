# Documentation

- **Prefix:** DOC
- **Kind:** area

This area covers the documents that tell people what the software is, how it
is built and how to use and configure it: the README, the architecture
document, the configuration reference, the API reference, the troubleshooting
guide and the language of instructions. Its rules count toward Maintainability
by default; the rule on instructions counts toward Accessibility & Usability.
Other areas hold the documents that they own: the Contributability (CTB) area
holds the development setup and how to run the checks, the Releases (REL)
area holds the changelog and migration guides, the Distribution (DST) area
holds the install, upgrade and uninstall steps, and the Security (SEC) area
holds the security policy.

### USS-DOC-001: Keep a README

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** The repository root has a README file. It says what the software
  does, how to install or build it (or links to the page that does), and how
  to start using it, such as a first command or a first call. A README that
  names only the project and a licence fails.
- **References:** none

The repository root MUST have a README that says what the software does, how
to install or build it, and how to start using it.

**Rationale:** The README is the first file that a user or a contributor
reads, and often the only one.

### USS-DOC-002: Describe the architecture

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** The repository has an architecture document (such as
  `ARCHITECTURE.md`, or a section of the README or the documentation site).
  It names the main components, says what each does and how they interact,
  and says where each lives in the repository. Each directory at the top
  level of the source tree appears in it, or the document says why the
  directory is not a component.
- **References:** matklad, ARCHITECTURE.md (2021-02-06)

An architecture document SHOULD name the main components, say what each does
and how they interact, and say where each lives in the repository.

**Rationale:** A new contributor can read a file faster than they can read the
code, and a map of the components shows where a change belongs. The decisions
behind the structure are in the ADRs of the Architecture (ARC) area.

### USS-DOC-003: Document every setting

- **Level:** MUST
- **Applies when:** has-cli or has-ui or is-network-service
- **Category:** Maintainability
- **Verify:** Collect the settings that the code reads: command-line options,
  environment variables and configuration keys. A search for the code that
  reads them (for example, calls that read the environment, or the option
  parser's definitions) finds them. A configuration reference lists each one
  with its default and its effect. A setting that the code reads and the
  reference omits fails.
- **References:** none

A configuration reference MUST document every setting (option, environment
variable or configuration key) with its default and its effect.

**Rationale:** An undocumented setting can be found only by reading the code,
and a user cannot judge a setting that they do not know exists.

### USS-DOC-004: Mark the settings that weaken security

- **Level:** MUST
- **Applies when:** has-cli or has-ui or is-network-service
- **Category:** Maintainability
- **Verify:** From the configuration reference, list each setting that turns
  off or weakens a security feature (such as authentication, encryption,
  certificate checks, access control or logging of security events). Each one
  is marked as such in the reference and states the risk of the setting. A
  setting of this kind that the reference does not mark fails.
- **References:** CISA, Shifting the Balance of Cybersecurity Risk:
  Principles and Approaches for Secure by Design Software (revised
  2023-10-25), Secure by Default (loosening guides)

The configuration reference MUST mark each setting that turns off or weakens
a security feature, and state the risk.

**Rationale:** The default configuration is secure (USS-SEC-002), so a setting
that weakens it is a deliberate choice. A user can make that choice
knowingly only if the reference says what it costs.

### USS-DOC-005: Document the public API

- **Level:** MUST
- **Applies when:** is-library
- **Category:** Maintainability
- **Verify:** List the public items of the library: each exported function,
  type, constant and module, in the way that the language defines "public".
  Each one has reference documentation, either generated from the code (such
  as doc comments that a documentation tool reads) or kept next to it. A
  documentation tool's own report of undocumented public items, where one
  exists, gives the list.
- **References:** none

Every public item of the library MUST have reference documentation, generated
from the code or kept next to it.

**Rationale:** The public items are the library's contract with its callers,
and a caller cannot rely on a contract that is not written down.

### USS-DOC-006: Write a troubleshooting guide

- **Level:** SHOULD
- **Applies when:** publishes-releases and (has-cli or has-ui or
  is-network-service)
- **Category:** Maintainability
- **Verify:** The user documentation has a troubleshooting section or page.
  It lists the common errors and failure modes, each with its cause and its
  fix. Each error that the last 12 months of closed bug reports name more
  than once has an entry.
- **References:** none

The user documentation SHOULD list common errors and failure modes, each with
its cause and its fix.

**Rationale:** A guide that answers the same question once saves it from
being asked again in every issue, and the answer reaches users who never
file one.

### USS-DOC-007: Write instructions in Simplified Technical English

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Accessibility & Usability
- **Verify:** List the documents that give steps for a person or an agent to
  follow: install, upgrade and runbook steps, and the workflows in an
  `AGENTS.md` file. Check a sample of at least ten steps from them, or all of
  them when there are fewer, against the writing rules of ASD-STE100: one
  instruction per sentence, the imperative mood, the active voice, and the
  sentence-length limits. Without the specification, the auditor judges
  these writing rules and records full conformance to the dictionary as
  `unverified`. With the specification or an STE checker, the auditor also
  checks the approved words.
- **References:** ASD-STE100 Simplified Technical English, Issue 9 (2025-01-15)

Every set of steps for a person or an agent to follow (such as install,
upgrade and runbook steps, and `AGENTS.md` workflows) SHOULD be written in
ASD-STE100 Simplified Technical English.

**Rationale:** Short, direct steps are read the same way by readers who do not
have the project's language as their first one, by people under stress during
an incident and by agents. This rule refers to the specification and does not
reproduce it; ASD gives out a copy free of charge on request at
asd-ste100.org.
