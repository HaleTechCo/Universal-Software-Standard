# USS: remaining work

This file lists the open work on 2026-11-27. It replaces
[follow-ups.md](follow-ups.md). The reason and the details for each item are
in the [research report](<research/reports/USS follow up recommendations.md>);
the section numbers below point into it.

The repository is public at
<https://github.com/HaleTechCo/Universal-Software-Standard>. USS 0.1.0 is not
released, and no alpha, beta or release candidate is planned before it.

## Done on 2026-11-27

- The install defect (R10) is fixed. AGENTS.md installs a pinned toolchain
  (`1.100.0`), and a test keeps the number equal to `rust-version`.
- Ruling R8 and plan decision 12 are reversed. Before the first release, an
  audit judges the release process at the audited commit.
- The changes of rule meaning for 0.1.0 are made: `stores-persistent-data`,
  `distributed-as-package`, USS-PRF-002 and conventions section 8,
  USS-PRF-005, USS-DEP-006, USS-TST-003 with the commit types, USS-TST-011,
  USS-TST-012, USS-SAF-004, USS-SAF-007, USS-CTB-005, USS-K8S-007, and
  USS-I18N-006 with the new fact `offers-multiple-languages`.
- `todo.txt` and `Initial_Idea.md` are gone from every commit, and Git ignores
  them. The files stay on the maintainer's machine.
- gitleaks 8.30.1 scanned the full history before and after the rewrite (149
  and 162 commits): no leaks.
- The repository was recreated as a public repository with the rewritten
  history. The old private repository is kept as
  `HaleTechCo/Universal-Software-Standard-old`.

## Done on 2026-11-28

The repository settings are on. Changes to `main` now go through pull
requests.

- Security: private vulnerability reporting, Dependabot alerts and security
  updates, secret scanning with push protection, and CodeQL default setup for
  Rust and GitHub Actions.
- Actions: each action must be pinned to a full-length commit SHA. The
  workflow token stays read-only. A first-time contributor's pull request
  waits for approval before CI runs.
- Ruleset `main` on the default branch, with no bypass: it blocks deletion and
  force pushes. It requires a pull request (0 approvals) and the checks
  `check (ubuntu-latest)`, `check (windows-latest)` and
  `check (macos-latest)`, on a branch that is up to date with `main`.
- Ruleset `release tags` on `refs/tags/v*`: it blocks updates, deletion and
  force pushes. Tags can still be created.
- Releases are immutable once published.
- Pull requests merge by squash only, with the pull request title and
  description as the commit message. Auto-merge is allowed, and merged
  branches are deleted.
- The topics are set, and Projects is off.

## 1. Reminders

- [ ] **A second admin (USS-CTB-016).** Invite the second person as owner of
      the organisation. Then name them in `GOVERNANCE.md`, and raise the
      required approvals of the `main` ruleset to 1 (report 2.1 and 2.3).
- [ ] **Optional settings, left off.** Secret scanning of non-provider
      patterns, and validity checks. The organisation's "GitHub recommended"
      configuration as the default for new public repositories. The DCO
      sign-off setting goes with the USS-CTB-009 decision.
- [ ] **Confirm the USS-I18N-006 decision.** The rule now applies to `uss`
      through `has-cli`. `.uss/record.yaml` declines it (`adopt: false`),
      because `uss` speaks English only.
- [ ] **Remove the old history from this machine.** The four worktrees under
      `.claude/worktrees/` and their branches still hold the old commits. End
      the Claude Code sessions that use them first. Then run, for each
      worktree:

      ```sh
      git worktree unlock .claude/worktrees/<name>
      git worktree remove .claude/worktrees/<name>
      git branch -D worktree-<name>
      ```

      The names are `uss-plan2-security`, `uss-plan3-core-areas`,
      `uss-plan4-modules` and `uss-plan5-workflows`. `-D` is needed, because
      the new `main` holds new copies of those commits. Then run
      `git reflog expire --expire=now --all` and `git gc --prune=now`.
- [ ] **Delete the backup repository** when you no longer need it:
      `gh repo delete HaleTechCo/Universal-Software-Standard-old`. Its
      history still holds the two notes files.
- [ ] **The commit email.** The history keeps the personal address, as
      decided. To keep it out of future commits, set
      `git config user.email` to the GitHub noreply address. Past commits keep
      the old address.

## 2. The trial

- [ ] Run the trial as the README section "Try USS before its first release"
      says, on a branch of the project. Log the points in report section 1.
- [ ] The repository is public now, so two shorter routes also work:
      `/plugin marketplace add HaleTechCo/Universal-Software-Standard` adds
      the marketplace from GitHub, and
      `cargo install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard uss`
      installs `uss` from `main`.

## 3. Decisions that remain yours

| Item | Recommendation | Report |
|---|---|---|
| Time to fix security reports | Change 90 days to 60 for medium or higher severity, with 30 for critical, in `SECURITY.md`. | 2.1 |
| Code of conduct contact (USS-CTB-008) | A role address that forwards to the maintainer and the second admin. | 2.1 |
| Succession text | Name the second admin, and where the release credentials are. | 2.1 |
| USS-CTB-009 | Require the Developer Certificate of Origin with a CI job. | 2.3 |
| USS-BAK-013 | A workflow that mirrors to GitLab or Codeberg. | 2.3 |
| USS-DOC-007 | Request the free copy of ASD-STE100 Issue 9, and reword the Verify. | 2.3 |
| USS-PRF-001 | A budget of 150 ms for `uss check` (median of 20 warm runs), with `cargo xtask bench`. | 2.3 |
| USS-DEP-006 | Build serde-saphyr with `deserialize` only, which drops nohash-hasher. | 2.3 |
| CodeQL and USS-CTB-003 | CodeQL default setup is on for USS-SEC-028, and it runs only in CI. Make a ruling or a Standard Deviation for USS-CTB-003. Or rule that Clippy with a security lint set counts as the SAST tool. | 4.4 |
| Repository host settings in USS | Decide how the standard covers the settings of "Done on 2026-11-28". USS-SUP, USS-SEC, USS-DEP, USS-CTB and USS-CON cover most of them. Some, such as blocking secrets at push and immutable releases, have no rule. | — |
| Additions to rulings R4, R5 and R9 | Add the sentences of the report to AGENTS.md. | 2.5 |
| USS-BAK-007 | Cite the US-CERT paper in place of "USS's own reading". | 2.5 |

## 4. The release plan for 0.1.0

The report planned new rules for 0.2.0. No release comes before 0.1.0, so
they can go into 0.1.0.

- [ ] **Code in `tools/uss` (15 results):** start with the two defects,
      USS-UX-002 and USS-SEC-014. Then build one output layer for the other
      CLI results (report 4.2).
- [ ] **Configuration and xtask (6 results):** cargo-deny 0.22.1, the Clippy
      and cargo-deny baselines, a warning-free CI log, ShellCheck, and one
      setup command (4.3).
- [ ] **Scanners (USS-SEC-004, USS-SEC-005):** gitleaks in
      `cargo xtask check` through a pinned, hash-checked download (4.4).
      CodeQL covers USS-SEC-028 after the USS-CTB-003 decision.
- [ ] **The release workflow:** dist, an SBOM with cargo-cyclonedx, GitHub
      artifact attestations (available now that the repository is public), a
      notices file with cargo-about, rebuild steps, and a 0.1.0 entry in
      `CHANGELOG.md` (4.6). Under the new step 16, the audit judges this
      workflow before the first release. Releases are immutable now, so the
      workflow attaches every asset to a draft release, then publishes it.
- [ ] **The CI template and immutable releases:**
      `templates/ci/uss-release.yml` runs when a release is published. It then
      uploads `uss-report.md` with `--clobber`. GitHub says to attach every
      asset to the draft before publishing, and does not say whether a later
      upload works. Change the template to upload to the draft, or test it
      on a release first.
- [ ] **Commit practices for the next audit:** a health column in
      `docs/dependencies.md` with a test, and the commit types (4.7).
- [ ] **Tool improvements:** filters for `uss applicable --full`, and
      `uss check` on results files (4.8).
- [ ] **References:** EN 301 549 V4.1.1, USS-CRA-004, CIS Controls 11.3, CEF,
      and a scheduled check of the vendor profiles (4.9).
- [ ] **Metadata rules:** USS-DST-017 to DST-022 and the module refinements
      (section 7).
- [ ] **Locale rules:** USS-I18N-007 to I18N-019 behind
      `offers-multiple-languages` (8A); the record key `project.language` and
      "Language of the audit" in AGENTS.md (8B); English as the normative text
      for translations (8C).
- [ ] **Audit this repository again.** The current audit
      (`.uss/audits/2026-11-25.yaml`) examines a commit from before the rule
      changes of 2026-11-27 and the settings of 2026-11-28. Expect these
      changes:
  - USS-LIC-004, USS-DST-014, USS-REL-003 and USS-REL-011 fail under the new
    step 16 until the release workflow exists;
  - USS-SEC-004 can pass with gitleaks;
  - USS-SEC-028 can pass with CodeQL, after the USS-CTB-003 decision;
  - USS-TST-003 and USS-DEP-006 change under their new texts;
  - the host settings now exist for USS-SUP-007, USS-SUP-008, USS-CON-003 and
    USS-CTB-012.

## 5. Later work

- [ ] Translations of USS itself, after 0.1.0 (report 8C).
- [ ] The items of spec section 11, in the order of report section 6.
- [ ] 1.0.0 when the maintainer declares it, after the record format settles.
