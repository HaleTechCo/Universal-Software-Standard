# Conduct

- **Prefix:** CON
- **Kind:** area

This area covers what a project writes down and checks about the quality of
its own work: no placeholder code, and a Definition of Done that CI enforces.
Its rules count toward Maintainability by default. Conduct that leaves no
artifact, such as no silent scope reduction, no fabricated success reports
and no changes outside the task, is not an audited rule. It belongs in the
"Agent conduct" section of AGENTS.md. The checks that CI runs are in the Code
quality (QLT) area, and the tests are in the Testing area.

### USS-CON-001: No placeholder code

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** Search the source of the production code (not tests, examples
  or fixtures) for stubs and placeholders: functions that only return a
  constant or an empty value in place of a result, "not implemented"
  exceptions and branches, `TODO` or `FIXME` markers that stand for missing
  behaviour, and mock or sample data returned as a real result. Each one that
  a user can reach fails, unless the user documentation names the feature as
  unsupported.
- **References:** none

A production code path MUST NOT hold a stub, a placeholder, a fake result or
a "not implemented" branch, unless the user documentation names that feature
as unsupported.

**Rationale:** A placeholder that looks like a working feature gives the
caller a wrong result with no warning. A feature that the documentation
names as unsupported tells the user the truth.

### USS-CON-002: Write down the Definition of Done

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** The repository holds a document (such as CONTRIBUTING.md, a
  pull request template or a development guide) that lists the conditions
  every change meets before it merges. The list is specific enough to decide
  by looking at a change, such as "all CI checks pass" or "the changelog is
  updated", and not only "the work is complete".
- **References:** The Scrum Guide (November 2020), Commitment: Definition of
  Done; NIST SP 800-218 (SSDF 1.1) PO.4.1

The repository MUST document the Definition of Done: the conditions that
every change meets before it merges.

**Rationale:** A change is done when it meets a shared, written list. Without
one, each author and reviewer decides alone, and the answer differs from
change to change.

### USS-CON-003: Check the Definition of Done in CI

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** For each condition of the documented Definition of Done that a
  tool can check (tests pass, lint passes, formatting, a changelog entry, a
  signed commit), a CI job runs that check. Each such job is a required
  check on the default branch, as the repository host's branch protection or
  ruleset shows (USS-SUP-008). The auditor records `unverified` without
  access to those settings. A condition that only a person can judge, such as
  a review, needs no CI job.
- **References:** The Scrum Guide (November 2020), Commitment: Definition of
  Done; NIST SP 800-218 (SSDF 1.1) PO.4.2

Each condition of the Definition of Done that a tool can check MUST run as a
required CI check, so CI, not a person's or an agent's report, decides
whether a change is done.

**Rationale:** A report that a change is done is a claim. A passing required
check is evidence, and it holds for every author, human or agent.
