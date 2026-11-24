# Supply chain

- **Prefix:** SUP
- **Kind:** area

This area covers the inputs to a build and the path from source to release:
toolchains and build inputs, protection of the source and of CI, and the SBOM,
signatures and provenance of release artifacts. Its rules count toward Safety
by default. Library and package dependencies are in the Dependencies area
(DEP), and this area has no rule for them.

## Toolchains and build inputs

### USS-SUP-001: Pin the toolchain

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** A file in the repository (such as a tool-version file, a lockfile
  for tools, a dev container definition or a pinned base image) records the
  version of each compiler, runtime and build tool that the build uses. The
  build configuration of local builds and of CI reads that file or names the
  same versions.
- **References:** NIST SP 800-218 (SSDF 1.1) PO.3

The version of every compiler, runtime and build tool that the build uses MUST
be pinned in a file in the repository, and local builds and CI MUST use that
version.

**Rationale:** A build that picks up whatever version is installed gives
different results on different machines and can change without any change to
the code. A pinned version makes the build repeatable and makes a toolchain
upgrade a reviewed change.

### USS-SUP-002: Install toolchains per project

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Safety
- **Verify:** The setup instructions and scripts install each toolchain and
  build tool into the project (through a version manager, a wrapper script, a
  dev container or the like), not into the system.
- **References:** none

Toolchains and build tools SHOULD be installed per project (such as through a
version manager, a wrapper script or a dev container), not system-wide.

**Rationale:** Per-project installs let each project pin its own versions and
leave the other software on the machine alone.

### USS-SUP-003: Leave system-wide installs alone

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Verify:** No build, setup or test script installs, upgrades or removes a system-wide runtime,
  package manager or global package. Look for system package manager commands,
  global install flags and changes to system paths.
- **References:** none

A build, setup or test script MUST NOT install, upgrade or remove a
system-wide runtime, package manager or global package.

**Rationale:** A script that changes the machine can break other projects and
the system itself, and it makes the result depend on the state of the machine.

### USS-SUP-004: Pin CI components by commit

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Every third-party CI component in the CI configuration (a GitHub
  Action, a GitLab CI include, a CircleCI orb or the like) is referenced by a
  full commit SHA or a content digest, and a comment gives its version.
  Scorecard Pinned-Dependencies reports unpinned GitHub Actions, as evidence
  for GitHub Actions only.
- **References:** GitHub Docs, Secure use reference (Pin actions to a
  full-length commit SHA); OpenSSF Scorecard v5.7.0 (Pinned-Dependencies)

Every third-party CI component (such as a GitHub Action, a GitLab CI include or
a CircleCI orb) MUST be referenced by a full commit SHA or content digest, with
its version in a comment.

**Rationale:** A tag or a branch can be moved to different code after the
project has reviewed it. A commit SHA or digest always names the same code.

### USS-SUP-005: Pin container images by digest

- **Level:** MUST
- **Applies when:** uses-container-images
- **Category:** Safety
- **Verify:** Each container image reference in Dockerfiles, Compose files,
  Kubernetes manifests, Helm charts and CI jobs includes a content digest
  (`@sha256:` followed by the digest). Scorecard Pinned-Dependencies covers
  Dockerfiles and workflows, so it is partial evidence.
- **References:** OpenSSF Scorecard v5.7.0 (Pinned-Dependencies)

Every container image reference in the repository MUST include its content
digest.

**Rationale:** A tag names whatever image its owner last pushed to it. A digest
names one image, so the build gets the image that was reviewed.

### USS-SUP-006: Verify downloaded build inputs

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Each download in build scripts and CI configuration that does
  not go through a package manager's lockfile (an installer, a script or a
  binary) names a fixed version, and the same step checks the file against a
  checksum or signature.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.4.4 (notional example on
  confirming integrity through digital signatures or other mechanisms)

Every file that the build or CI downloads outside a package manager's lockfile
(installers, scripts, binaries) MUST be pinned to a version and checked against
a checksum or signature.

**Rationale:** A download that is not checked runs whatever the server returns
at that moment, so a compromised server or a changed file becomes part of the
build.

### USS-SUP-007: No end-of-life platforms

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Verify:** The versions of the runtimes, compilers, build tools and base
  images that the project builds with or runs on (from the version files,
  Dockerfiles and CI configuration) are all within the supplier's published
  support period for security fixes at the audit date.
- **References:** The supplier's support policy for each platform; CWE-1104
  (a runtime or base image is a third-party component that the product relies
  on)

The project MUST NOT build with or run on a runtime, compiler, build tool or
base image that its supplier no longer supports with security fixes.

**Rationale:** A flaw found in an unsupported platform is not fixed, so every
project that uses it keeps the flaw.

## Source and CI protection

### USS-SUP-008: Protect the default branch

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The repository host's settings (for example GitHub branch
  protection or rulesets, or GitLab protected branches and protected tags,
  read through the host's settings or API) block force-pushes and deletion of
  the default branch and of release tags. OpenSSF Scorecard Branch-Protection
  is evidence for the default and release branches only, so it is partial
  evidence: it does not cover tags. Without access to the settings, the
  auditor records `unverified`.
- **References:** OpenSSF Scorecard v5.7.0 (Branch-Protection)

The default branch and release tags MUST be protected against force-pushes and
deletion.

**Rationale:** History that can be rewritten or deleted cannot be trusted as
the record of what was released.

### USS-SUP-009: Require CI checks before merge

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The repository host's settings (for example GitHub branch
  protection or rulesets, or GitLab merge request settings, read through the
  host's settings or API) list the CI checks that must pass before a change
  merges into the default branch. OpenSSF Scorecard Branch-Protection
  (required status checks) is evidence. Without access to the settings, the
  auditor records `unverified`.
- **References:** OpenSSF Scorecard v5.7.0 (Branch-Protection)

A change MUST reach the default branch only after the required CI checks pass.

**Rationale:** A check that a change can skip does not protect the default
branch.

### USS-SUP-010: Review changes before merge

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Safety
- **Verify:** The repository host's settings (for example GitHub branch
  protection or rulesets that require approving reviews, or GitLab merge
  request approval rules, read through the host's settings or API) require
  each change to be approved before it merges, and the merged changes show an
  approval from someone other than the author. OpenSSF Scorecard Code-Review
  is evidence, but only partial: it samples recent commits, and it can accept
  a merger who differs from the committer in place of an explicit approval.
  Without access to the settings, the auditor records `unverified`.
- **References:** OpenSSF Scorecard v5.7.0 (Code-Review)

Each change SHOULD be reviewed and approved by someone other than its author
before it merges.

**Rationale:** A second person catches mistakes and malicious changes that the
author misses. A project with one maintainer cannot meet this rule, because
nobody else can approve. It records a Standard Deviation instead and relies on
its other controls.

### USS-SUP-011: Limit CI token permissions

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The CI configuration sets read-only token permissions by default
  (for GitHub, a top-level `permissions` block, or the repository's default
  workflow permissions setting), and each job that needs more declares only
  those permissions. OpenSSF Scorecard Token-Permissions is evidence, but only
  partial: it cannot see the repository-level setting. Without access to the
  settings, the auditor records `unverified`.
- **References:** OpenSSF Scorecard v5.7.0 (Token-Permissions); GitHub Docs,
  Secure use reference (Use of the GITHUB_TOKEN with least privilege)

Each CI job MUST run with the least token permissions it needs, read-only by
default.

**Rationale:** A job that is compromised can do what its token allows. A token
with fewer permissions limits the damage.

### USS-SUP-012: Keep untrusted code away from secrets

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Verify:** In the CI configuration, no job that has access to secrets or a
  write token runs code from an untrusted change (a pull request from a fork,
  for example). Look at triggers that run with the target repository's
  privileges (such as `pull_request_target` or `workflow_run` on GitHub) for a
  checkout of the change's code. Scorecard Dangerous-Workflow reports this
  pattern, but it does not decide whether a given checkout is safe.
- **References:** OpenSSF Scorecard v5.7.0 (Dangerous-Workflow); GitHub
  Security Lab, Keeping your GitHub Actions and workflows secure Part 1:
  Preventing pwn requests

CI MUST NOT run code from an untrusted change (such as a pull request from a
fork) in a job that has secrets or a write token.

**Rationale:** Code from an untrusted change can read every secret and use
every permission of the job that runs it.

### USS-SUP-013: Keep untrusted data out of CI scripts

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Verify:** In the CI configuration, no script text contains an expression
  that inserts untrusted event data (titles, branch names, commit messages,
  comments) directly. Such data reaches a script only through an environment
  variable or a file. Scorecard Dangerous-Workflow reports script injection
  for GitHub workflows.
- **References:** GitHub Docs, Secure use reference (Understanding the risk of
  script injections); CWE-78

CI MUST NOT insert untrusted event data (titles, branch names, commit
messages, comments) into a script it runs, and it passes such data through
environment variables or files.

**Rationale:** An expression that is expanded into script text becomes part of
the script, so whoever writes the title or the branch name can run commands in
the job.

## Release artifacts

### USS-SUP-014: Publish an SBOM

- **Level:** SHOULD
- **Applies when:** distributed-as-package or ships-container-image
- **Category:** Safety
- **Verify:** Each release has an SBOM in SPDX or CycloneDX format, published
  with the release. The SBOM lists the components of the release, and each
  entry shows whether the component is a direct or a transitive dependency.
- **References:** SPDX 3.0.1; CycloneDX 1.7; NTIA, The Minimum Elements For a
  Software Bill of Materials (SBOM) (2021), Data Fields (Dependency
  Relationship)

Each release SHOULD publish an SBOM in SPDX or CycloneDX format that lists
every component and marks each as direct or transitive.

**Rationale:** Users and downstream projects need to know what a release
contains to find out whether a newly disclosed flaw affects them.

### USS-SUP-015: Sign release artifacts

- **Level:** SHOULD
- **Applies when:** distributed-as-package or ships-container-image
- **Category:** Safety
- **Verify:** Each release artifact has a signature published with it or in a
  registry (a Sigstore signature, for example). The release documentation says
  how to verify the signature.
- **References:** Sigstore (Cosign signing and verification, docs.sigstore.dev)

Release artifacts SHOULD be signed, and the release documentation SHOULD say
how to verify the signature.

**Rationale:** A signature lets users check that an artifact comes from the
project and was not changed after the release.

### USS-SUP-016: Meet SLSA Build Level 2

- **Level:** SHOULD
- **Applies when:** distributed-as-package or ships-container-image
- **Category:** Safety
- **Verify:** Release builds run on a hosted build platform, and each release
  has provenance that is signed by the build platform (for example an
  attestation generated by the CI service), in line with the SLSA v1.2 Build
  Level 2 requirements.
- **References:** SLSA v1.2, Build track (Build L2)

Releases SHOULD be built on a hosted build platform that produces signed
provenance, meeting SLSA Build Level 2.

**Rationale:** Provenance signed by the build platform lets users check which
source and which build produced an artifact.

### USS-SUP-017: Meet SLSA Build Level 3

- **Level:** SHOULD
- **Applies when:** (distributed-as-package or ships-container-image) and high-assurance
- **Category:** Safety
- **Verify:** The release build platform and its provenance meet the SLSA v1.2
  Build Level 3 requirements, as the platform's documentation or an
  independent attestation shows.
- **References:** SLSA v1.2, Build track (Build L3)

Releases SHOULD meet SLSA Build Level 3.

**Rationale:** At this level the build platform keeps builds apart from one
another and keeps the signing secret away from the build steps, so a single
compromised build cannot forge provenance.
