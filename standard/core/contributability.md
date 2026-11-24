# Contributability

- **Prefix:** CTB
- **Kind:** area

This area covers how easily someone new, whether a person or a coding agent,
can start to contribute, and whether the project outlives any one person: the
development setup and the checks, the guides, the way that outside
contributions are received and the continuity of access and maintainers. Its
rules count toward Contributability by default. The Documentation (DOC) area
holds the README and the other documents about the software itself, and the
Supply chain (SUP) area holds the protection of CI.

## Setup and checks

### USS-CTB-001: Set up the development environment with one command

- **Level:** MUST
- **Applies when:** always
- **Category:** Contributability
- **Verify:** The contributing guide, the README or an `AGENTS.md` file gives
  one command (such as a make target or a script), or the repository has a
  dev container definition, that sets up a complete development environment.
  Run it in a fresh clone, on a clean machine or in a container: afterwards
  the build and the tests run with no other step. A guide that lists several
  manual steps in place of one command fails.
- **References:** Development Containers Specification (containers.dev, read
  2026-11-24)

One documented command, or a dev container, MUST set up a complete
development environment from a fresh clone.

**Rationale:** Each step that a newcomer does by hand is a place where they
can fail, and most who fail do not ask for help.

### USS-CTB-002: Install toolchains per project

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Contributability
- **Verify:** The setup command (USS-CTB-001) and the guides install each
  compiler, runtime and build tool for the project only: through a version
  manager, a wrapper script, a project-local directory or a dev container. They
  do not tell the contributor to install a tool for the whole system with a
  system package manager or with administrator rights. A setup script that
  installs system-wide is judged by USS-SUP-002, not here.
- **References:** none

Toolchains and build tools SHOULD be installed for the project only (such as
through a version manager, a wrapper script or a dev container), not
system-wide.

**Rationale:** A system-wide tool is shared by every project on the machine,
so two projects that need different versions break each other, and a setup
that changes the system is a risk that a contributor must trust.

### USS-CTB-003: Run the CI checks locally with one command

- **Level:** MUST
- **Applies when:** always
- **Category:** Contributability
- **Verify:** A documented command runs locally the same checks that CI requires
  for a change: each job that the branch protection or the merge rules of the
  project name as required, or each CI job when there are none. The CI
  configuration calls that command, or the two lists of checks are the same. A
  required CI job that the command does not run fails. Without access to the
  branch protection or merge settings, the auditor records `unverified`.
- **References:** none

One documented command MUST run locally the same checks that CI requires.

**Rationale:** If a contributor can see a failure only after they push, each
fix costs a wait. The same command in both places also keeps CI and the
local checks from drifting apart.

### USS-CTB-004: Run fast checks before each commit

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Contributability
- **Verify:** The repository provides pre-commit hooks (such as a
  `.pre-commit-config.yaml` file for pre-commit, or hook scripts in the
  repository). They run the fast checks that CI runs, such as formatting and
  lint.
- **References:** none

The repository SHOULD provide pre-commit hooks that run the fast checks that
CI runs, such as formatting and lint.

**Rationale:** A formatting error caught before the commit never reaches
review or CI.

### USS-CTB-005: State how long the checks take

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Contributability
- **Verify:** The contributing guide states how long the one-command checks
  (USS-CTB-003) take on a typical machine. Run the command, or read the
  duration of the matching CI job, on the default branch: the time is not
  more than 25% above the stated time. Without a run or a CI record, the
  auditor records `unverified` for the second part.
- **References:** none

The contributing guide SHOULD state how long the one-command checks take on a
typical machine, and the checks SHOULD stay within that time.

**Rationale:** A contributor who knows that the checks take ten minutes starts
them and waits, while one who does not may stop a run that is working. A stated
time also shows when the checks have become too slow. The margin of 25% is a
number that USS adds, to allow for differences between machines.

## Guides

### USS-CTB-006: Write a contributing guide

- **Level:** MUST
- **Applies when:** always
- **Category:** Contributability
- **Verify:** The repository has a `CONTRIBUTING.md` file (or a guide that the
  README links to). It covers the setup, the workflow for a change (such as
  branch, commit, pull request), how to run the checks, and what makes a change
  acceptable (such as tests and documentation; a pointer to the Definition of
  Done that USS-CON-002 holds is enough). A guide that omits one of these four
  fails.
- **References:** OpenSSF Best Practices Badge criteria (read 2026-11-24),
  contribution and contribution_requirements

A `CONTRIBUTING.md` MUST cover the setup, the workflow for a change, how to
run the checks and what makes a change acceptable.

**Rationale:** A contributor who has to guess the process sends changes that
the project cannot accept, and both sides lose time.

### USS-CTB-007: Write an AGENTS.md

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Contributability
- **Verify:** The repository root has an `AGENTS.md` file. It gives the exact
  commands to set up, build, test and run the checks, and the project's
  conventions. Each command in it runs as written in a fresh clone. A file
  that points to the contributing guide and gives no commands fails.
- **References:** AGENTS.md (agents.md, read 2026-11-24)

An `AGENTS.md` SHOULD give coding agents the exact setup, build, test and
check commands and the project's conventions.

**Rationale:** An agent reads files and runs commands, and it cannot ask what
the team means by "run the usual checks". A file that holds the exact
commands lets it work as a new contributor would after reading the guide.

## External contributions

### USS-CTB-008: Adopt a code of conduct

- **Level:** MUST
- **Applies when:** accepts-external-contributions
- **Category:** Contributability
- **Verify:** The repository has a code of conduct (such as
  `CODE_OF_CONDUCT.md`, in a standard location). It says how to report a
  problem, with a contact that is filled in and is not a template
  placeholder. A copy of the Contributor Covenant whose reporting section
  still holds its placeholder text fails.
- **References:** Contributor Covenant 3.0; OpenSSF Best Practices Badge
  criteria (read 2026-11-24), code_of_conduct

The repository MUST have a code of conduct, such as the Contributor Covenant,
that says how to report a problem.

**Rationale:** Contributors from outside the team need to know what behaviour
to expect and whom to tell when it fails.

### USS-CTB-009: Use DCO sign-off, not a CLA

- **Level:** SHOULD
- **Applies when:** accepts-external-contributions
- **Category:** Contributability
- **Verify:** The contributing guide asks for a `Signed-off-by` line under the
  Developer Certificate of Origin. A CI job or a repository app checks that each
  commit of a pull request has one. The project asks for no contributor licence
  agreement (CLA). The commits since the last audit, or the last 12 months when
  there is none, that came from outside the team have the line.
- **References:** Developer Certificate of Origin 1.1

Contributors SHOULD certify their right to contribute by Developer
Certificate of Origin sign-off, which CI checks, and no contributor licence
agreement SHOULD be required.

**Rationale:** A sign-off is a line in the commit, so it costs a contributor
no extra step, and unlike a CLA it gives no one the right to relicense the
work.

### USS-CTB-010: Provide issue and pull request templates

- **Level:** SHOULD
- **Applies when:** accepts-external-contributions
- **Category:** Contributability
- **Verify:** The repository has a template for bug reports, one for feature
  requests and one for pull requests (such as files in `.github/ISSUE_TEMPLATE`
  and `.github/pull_request_template.md`, or the equivalent for the host).
- **References:** GitHub Docs, About issue and pull request templates (read
  2026-11-24)

The repository SHOULD have templates for bug reports, feature requests and
pull requests.

**Rationale:** A template asks for the facts that a maintainer needs, so a
report arrives with them instead of after three rounds of questions.

### USS-CTB-011: Label issues for newcomers

- **Level:** SHOULD
- **Applies when:** accepts-external-contributions
- **Category:** Contributability
- **Verify:** The contributing guide names the label for issues that suit a
  first contribution (such as `good first issue`). The label exists in the
  issue tracker, and at least one open issue carries it, or the guide says
  that none is open at present. Without access to the tracker, the auditor
  records `unverified`.
- **References:** none

Issues that suit a first contribution SHOULD carry a label that says so (such
as `good first issue`), and the contributing guide SHOULD name the label.

**Rationale:** A newcomer cannot judge which issue is small. A label shows
where to start.

### USS-CTB-012: Run CI on pull requests from forks

- **Level:** MUST
- **Applies when:** accepts-external-contributions
- **Category:** Contributability
- **Verify:** The CI configuration runs the required checks (USS-CTB-003) on
  pull requests from forks, for example through a `pull_request` trigger on
  GitHub. The settings of the repository host do not stop these runs, or only
  hold them for approval by a maintainer. Without access to the host's
  settings, the auditor records `unverified`.
- **References:** none

CI MUST run the required checks on pull requests from forks. USS-SUP-011
keeps secrets away from that code.

**Rationale:** An outside contributor has no other way to see whether their
change passes, and a maintainer should not have to run the checks by hand to
review it.

### USS-CTB-013: Document the review process

- **Level:** SHOULD
- **Applies when:** accepts-external-contributions
- **Category:** Contributability
- **Verify:** The contributing guide says how changes are reviewed (who
  reviews, and what approval a change needs before it is merged) and how soon
  a contributor can expect a first response, as a number of days.
- **References:** none

The contributing guide SHOULD say how changes are reviewed and how soon a
contributor can expect a first response.

**Rationale:** A contributor who hears nothing does not know if the change is
being read. A stated time lets them wait, or ask.

### USS-CTB-014: Document governance and maintainers

- **Level:** SHOULD
- **Applies when:** accepts-external-contributions and has-multiple-maintainers
- **Category:** Contributability
- **Verify:** The repository has a `GOVERNANCE.md` that says how decisions are
  made and how people become maintainers, and a `MAINTAINERS` file (or an
  equivalent such as `CODEOWNERS` with names) that lists the current
  maintainers by name.
- **References:** OpenSSF Best Practices Badge criteria (read 2026-11-24),
  governance and roles_responsibilities

A `GOVERNANCE.md` SHOULD say how decisions are made and how people become
maintainers, and a `MAINTAINERS` file SHOULD list the current maintainers.

**Rationale:** When more than one person decides, the others need to know how
a decision is made and who has the right to make it.

### USS-CTB-015: Credit contributors

- **Level:** SHOULD
- **Applies when:** accepts-external-contributions
- **Category:** Contributability
- **Verify:** Take the contributors from outside the team whose changes were
  merged since the last audit, or the last 12 months when there is none. Each
  one is named in the release notes of the release that holds the change, or in
  a contributors list (such as `CONTRIBUTORS` or `AUTHORS`).
- **References:** none

Each contributor SHOULD be credited, such as in the release notes or a
contributors list.

**Rationale:** Credit is the main reward that most outside contributors get,
and it brings them back.

## Continuity

### USS-CTB-016: Give two people admin and release access

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Contributability
- **Verify:** In the access settings of the repository host and of each
  package registry that the project publishes to, at least two people have
  admin access to the repository and can publish a release. A backup who
  takes no part in reviews counts here, unlike for the fact
  `has-multiple-maintainers`. Without access to these settings, the auditor
  records `unverified`.
- **References:** OpenSSF Best Practices Badge criteria (read 2026-11-24),
  access_continuity and bus_factor; CHAOSS, Contributor Absence Factor, the
  metric formerly named Bus Factor (read 2026-11-24)

At least two people SHOULD have admin access to the repository and be able to
publish a release.

**Rationale:** If the only person with access leaves, falls ill or loses
their account, nobody can fix, release or hand over the project.

### USS-CTB-017: Document a succession plan

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Contributability
- **Verify:** A file in the repository (such as `GOVERNANCE.md` or a section of
  the contributing guide) says who takes the project over if its maintainers
  leave, or how the project is archived and its users told. It names a
  person, a role or an organisation, not only "the community".
- **References:** OpenSSF Best Practices Badge criteria (read 2026-11-24),
  access_continuity

A documented plan SHOULD say who takes the project over, or how it is
archived and its users told, if its maintainers leave.

**Rationale:** A project that ends without notice leaves its users with
software that no one maintains and no one has said so.
