# Repository host settings: design

Date: 2026-11-28. Status: for the maintainer's approval. A builder check
was done; its questions are answered below.

## Intent

USS must cover every setting of a repository host that matters to the safety
of a project: branch and tag protection, two-factor authentication, secret
blocking, vulnerability reporting, release integrity and the like. The
maintainer set these settings up on GitHub on 2026-11-27 and 2026-11-28, and
found that USS names most of them but not all.

The rules stay generic. They name GitHub and GitLab only as examples, as the
Verify of USS-SUP-007 does today. The concrete "how" for each host goes in a
host profile: a file that maps each rule to the setting that meets it, and
gives the command that reads the setting back. An adopter, or an agent, then
sets up and audits a repository from one place, and nothing in the standard
depends on one host.

## Goals

- Add generic rules for the settings that USS does not cover yet, each at its
  own level (MUST, SHOULD or MAY), as listed under "Rules".
- Change USS-SEC-029 so that the audit checks that the private reporting
  channel named in the policy is switched on.
- Change USS-SEC-005, USS-SEC-028 and USS-DEP-007 so that a scan that the
  host runs counts when it checks every change and blocks the merge.
- Cite the OpenSSF Open Source Project Security Baseline (OSPS Baseline) in
  each new rule that one of its controls matches, and in the existing rules
  of "Citations for existing rules". Citations in other existing rules (such
  as signed releases, the licence file or tests in CI) are a follow-up for
  `docs/superpowers/remaining.md`, not part of this work (my call: the
  subject here is host settings).
- Replace the planned USS-DST-021 "Describe the repository on its host" of
  the research report (section 7) with the two DOC rules below, which the
  maintainer approved on 2026-11-28. The planned DST-022 takes the number
  DST-021, and the metadata item in `remaining.md` changes to match (my call:
  the newer decision wins, and two rules on one subject would overlap).
- Write two host profiles: GitHub and GitLab.
- Extend conventions section 12 so that a profile can describe a repository
  host, not only a checking tool.
- Tell auditors in AGENTS.md to read the host profile of the project's host.
- Make the CI template `templates/ci/uss-release.yml` work when the project
  makes releases immutable, because the new release rule recommends that.

## Non-goals

- Other hosts (Bitbucket, Codeberg or Forgejo, Azure DevOps). Their profiles
  can come later, in the same format.
- A rule that requires "OSPS Baseline Level N". USS keeps its own rules, so
  that each setting has its own level, and because the Baseline has no
  control for force pushes, immutable releases or SHA pinning.
- A crosswalk of every OSPS control to USS rules.
- A `uss` command that applies or reads host settings.
- Rules for preferences: the merge method, auto-merge, deleting merged
  branches, turning off unused features (wiki, projects), and organisation
  defaults for new repositories. A profile can mention them as tips, outside
  the mapping table.

## Anti-goals

These would make the work a failure even if it passed every check:

- Host-specific requirements in the text of a core rule. Examples only.
- A MUST rule with no free way to meet it. A MUST must be within reach of
  any team (conventions section 7). One free route is enough, such as a
  public repository on GitHub Free or any repository on GitLab Free. The
  maintainer decided this on 2026-11-28: rulesets and branch protection need
  a paid GitHub plan for a private repository, and the new pull-request rule
  stays a MUST, as USS-SUP-007 and USS-SUP-008 already are. The GitHub
  profile says so in each row that needs a paid plan for a private
  repository.
- A profile that restates or changes what a rule requires (section 12: "The
  profile does not change what a rule requires").
- A rule that an audit fails for a matter of taste.

## Constraints

- `standard/conventions.md` binds every rule: one requirement, verifiable
  from the project's artifacts, references read from the current source and
  pinned, the next free ID at the end of the file, the keyword scan, and the
  level guidance of section 7.
- New rules take the next free number in their area file. On 2026-11-28 the
  last IDs are USS-SUP-016, USS-SEC-033, USS-REL-012 and USS-DOC-007.
- USS stays at version 0.1.0, unreleased. A change of rule meaning before
  0.1.0 needs no changelog entry.
- Steps follow ASD-STE100 (AGENTS.md). Prose wraps at 80 columns. UTF-8, LF.
- The maintainer wants to stay on free plans. The profiles state the tier of
  each host feature they use.
- `cargo xtask check` passes at the end.

## Approaches

Three ways to hold the per-host "how" were considered. All three add the same
generic rules first.

1. **Host profiles** (chosen). One file per host under `standard/vendors/`,
   in the vendor-profile format: a mapping table from each rule to the host
   setting, with the command that reads it back. It reuses an existing
   format and its `uss lint` checks, and section 12 already requires a
   profile to map every applicable MUST and MUST NOT rule within its reach,
   which gives the coverage check the maintainer wants. Cost: section 12
   assumes a checking tool with one configuration file, so its text must
   allow a host.
2. **Host modules.** `github` and `gitlab` as stack identifiers, with
   module rules that refine the core rules. Most concrete, but it puts
   host-specific requirements in the standard, roughly doubles the rules on
   this subject, and turns each host UI change into a change of the
   standard.
3. **Setup guides in `docs/`.** Cheapest, but nothing checks that they cover
   every rule or stay correct.

The maintainer chose approach 1.

## Rules

### New rules

The levels and applicability below were approved by the maintainer. The
titles are working titles; the builder writes the final titles and Verify
texts. Each new rule cites the OSPS Baseline control named in its row.

| Working title | Area | Level | Applies when | OSPS Baseline v2026.08.28 |
|---|---|---|---|---|
| Require two-factor authentication for every account with write access | Supply chain (SUP) | MUST | always | OSPS-AC-01.01 (Level 1) |
| Send every change to the default branch through a pull or merge request | Supply chain (SUP) | MUST | always | OSPS-AC-03.01 (Level 1) |
| Give new collaborators the lowest access by default | Supply chain (SUP) | MUST | always | OSPS-AC-02.01 (Level 1) |
| Make the host enforce SHA pins of CI components | Supply chain (SUP) | MAY | always | none |
| Block pushes that contain secrets | Security (SEC) | SHOULD | always | OSPS-BR-07.01 (Level 1) |
| Make published releases immutable | Release (REL) | SHOULD | publishes-releases | none |
| Describe the repository on its host | Documentation (DOC) | SHOULD | is-foss | none |
| Tag the repository with topics | Documentation (DOC) | MAY | is-foss | none |

Notes for the builder:

- **Two-factor authentication.** The host enforces it for every account with
  write access: on GitHub, the organisation setting "Require two-factor
  authentication"; on GitLab, the group setting "All users in this group must
  set up two-factor authentication" (Free tier). Decide in the Verify what
  holds for a repository that a personal account owns. Read the host
  documentation on that case first.
- **Pull or merge requests.** This is separate from USS-SUP-008, which
  requires checks before merge. On GitLab, "Pipelines must succeed" applies
  only to merge requests, so a Maintainer can still push directly unless the
  branch rule says "Allowed to push and merge: No one".
- **Lowest access.** On GitHub, the organisation's base permission is "Read"
  or "No permission". On GitLab, a role is chosen at each invitation. Find
  out from the host documentation what a repository owned by a personal
  account gives a collaborator, and decide the Verify for that case.
- **SHA pins.** A MAY rule, because only some hosts can enforce it. On
  GitHub, the Actions policy "Require actions to be pinned to a full-length
  commit SHA". GitLab has no such setting (2026-11-28). It refines nothing:
  USS-SUP-003 still requires the pins themselves.
- **Secrets at push.** A SHOULD rule, because the host features cost money in
  some cases: GitHub push protection is free on public repositories only, and
  GitLab secret push protection is Ultimate-tier only. A server-side hook
  (pre-receive) on a self-hosted server also meets the rule. A client-side
  hook does not, because a developer can skip it. A project that cannot meet
  the rule writes a Standard Deviation that gives the reason.
- **Immutable releases.** A published release's tag and files cannot be
  changed or replaced. On GitHub, the repository setting "Enable release
  immutability". GitLab has no equivalent (2026-11-28); protected tags stop
  only the tag from moving. The Verify must cover each channel where the
  project publishes release files. Some package registries already refuse to
  replace a published version; read their documentation before naming one.
- **Description and topics.** `is-foss` stands in for "the repository is
  public", because USS has no fact for that. The description is a SHOULD
  because it costs one line. Topics are a MAY.

### Changed rules

- **USS-SEC-005, USS-SEC-028 and USS-DEP-007: scans that the host runs.**
  Today each Verify asks for a scanner in "the CI configuration" whose
  finding "fails that CI check". The maintainer decided on 2026-11-28 that a
  scan that the host runs from its settings also counts, when it checks
  every change (each pull request and each push to the default branch) and
  a finding blocks the merge. An alert alone does not count. A dismissed
  host finding counts as a suppression, and needs a recorded reason like any
  other (USS-QLT-006 to USS-QLT-014). USS-DEP-007 keeps its scheduled scan
  and its lockfile and transitive scope. The builder may retitle the rules
  ("… in CI") to match. Consequences:
  - CodeQL default setup meets USS-SEC-028 when the default-branch ruleset
    also requires code scanning results. Confirm from GitHub's documentation
    that this ruleset rule is free on a public repository, and on which
    plans it exists for a private one.
  - GitHub secret scanning alerts and Dependabot alerts do not meet USS-SEC-005
    or USS-DEP-007 on their own. Push protection blocks some secrets at push;
    the builder decides whether its mapping row for USS-SEC-005 is `full` or
    `partial`.
  - The open USS-CTB-003 question (checks that cannot run locally) stays
    open, outside this work.
- **USS-SEC-029.** The Verify also checks that the private channel that the
  policy names is open: a host feature such as GitHub private vulnerability
  reporting is switched on, or the named address exists and belongs to the
  project. On 2026-11-27 this repository's `SECURITY.md` named private
  vulnerability reporting while the feature was off, and the rule still
  passed. Cite OSPS-VM-02.01 (Level 1) and OSPS-VM-03.01 (Level 2). On
  GitLab, Service Desk (Free tier) creates confidential tickets from email.

USS-SUP-009 needs no change. Its Verify already accepts the history when the
host cannot require an approval, which covers GitLab Free, where required
approvals are a Premium feature.

### Citations for existing rules

Add the matching OSPS Baseline control to the References of these rules.
Read each control's text, and add the citation only where the control
matches what the rule requires:

| Rule | OSPS Baseline v2026.08.28 control |
|---|---|
| USS-SUP-007 | OSPS-AC-03.02 (deletion of the primary branch) |
| USS-SUP-008 | OSPS-QA-03.01 |
| USS-SUP-009 | OSPS-QA-07.01 |
| USS-SUP-010 | OSPS-AC-04.01, OSPS-AC-04.02 |
| USS-SUP-011 | OSPS-BR-01.03 |
| USS-SUP-012 | OSPS-BR-01.01 |
| USS-SEC-004 | OSPS-BR-07.01 |
| USS-SEC-030 | OSPS-VM-01.01 |
| USS-CTB-009 | OSPS-LE-01.01 |

Sources: <https://baseline.openssf.org/versions/2026-08-28> and the release
<https://github.com/ossf/security-baseline/releases/tag/v2026.08.28>. The
Baseline defines Level 1 as "for any code or non-code project with any number
of maintainers or users".

## Host profiles

### Files

- `standard/vendors/github/github.md`
- `standard/vendors/gitlab/gitlab.md`

Both are `community` profiles. They have no `## Rules` section.

### Conventions section 12

Change the text so that a profile can describe a repository host:

- A profile states how one checking tool **or repository host** meets rules.
- For a host, `<vendor>` is the lowercase name of the organisation that runs
  it (`github`, `gitlab`).
- For a hosted service without versions, `Versions` names the service and the
  date its documentation was read, such as `GitHub.com (read 2026-11-28)`.
  For GitLab, also name the tiers that the profile covers.
- For a host, `## Baseline configuration` is one code block that lists each
  setting with its expected value, grouped by the API endpoint that reads it.
  My call: YAML, keyed by endpoint, so that an auditor can compare it with
  the output of `gh api` or `glab api`.
- For a host, `## Check-integrity guards` lists the settings that stop a rule
  from being switched off quietly, such as an active ruleset with an empty
  bypass list.
- Nothing in a module names a host profile. AGENTS.md does (below).

`uss lint` already accepts this: `Versions` is free text, and the baseline
needs exactly one code block (`tools/uss/src/catalog/profiles.rs`). Change
the code only if the builder finds that a check fails. Then add a test.

### Content

Each profile maps every applicable MUST and MUST NOT rule within the host's
reach, as section 12 requires, plus the SHOULD and MAY rules that a host
setting meets. A row whose host feature needs a paid tier says so in its
`Coverage` column and names the free alternative, when one exists.

The GitHub settings below were read back from this repository on 2026-11-28
with `gh api`. Use them as the starting point of the GitHub baseline:

| Endpoint | Field and value | Rules |
|---|---|---|
| `repos/{owner}/{repo}/rulesets` (each ruleset by ID) | branch ruleset on `~DEFAULT_BRANCH`: `deletion`, `non_fast_forward`, `pull_request`, `required_status_checks` (strict); tag ruleset on `refs/tags/v*`: `deletion`, `non_fast_forward`, `update`; `enforcement: active`; `bypass_actors: []` | SUP-007, SUP-008, CON-003, the new pull-request rule |
| `repos/{owner}/{repo}` | `security_and_analysis.secret_scanning.status: enabled`, `secret_scanning_push_protection.status: enabled`, `dependabot_security_updates.status: enabled`; `description`; `topics` | SEC-005 (partial), the new secrets rule, DEP-014, the description and topics rules |
| `repos/{owner}/{repo}/private-vulnerability-reporting` | `enabled: true` | SEC-029 |
| `repos/{owner}/{repo}/vulnerability-alerts` | HTTP 204 | DEP-007 |
| `repos/{owner}/{repo}/code-scanning/default-setup` | `state: configured`, plus a `code_scanning` rule in the branch ruleset (not set yet on 2026-11-28) | SEC-028 |
| `repos/{owner}/{repo}/actions/permissions` | `sha_pinning_required: true` | the new SHA-pin rule |
| `repos/{owner}/{repo}/actions/permissions/workflow` | `default_workflow_permissions: read`, `can_approve_pull_request_reviews: false` | SUP-010 |
| `repos/{owner}/{repo}/actions/permissions/fork-pr-contributor-approval` | `approval_policy: first_time_contributors` | CTB-012, SUP-011 |
| `repos/{owner}/{repo}/immutable-releases` | `enabled: true` | the new release rule |
| `orgs/{org}` | `two_factor_requirement_enabled: true`, `default_repository_permission: read` | the new 2FA and access rules |

Some endpoints need more than the default `gh` scopes. For example,
`orgs/{org}/actions/permissions` needs `admin:org`. The profile names the
scope where one is needed.

The GitLab research of 2026-11-28 (docs.gitlab.com, tiers as the pages state
them) gives these starting points. Read each page again before writing the
profile:

| Need | GitLab feature | Tier | Read back |
|---|---|---|---|
| Block force push and deletion | Protected branches | Free | `GET /projects/:id/protected_branches`, `allow_force_push` |
| Protect release tags | Protected tags | Free (roles); Premium for users or groups | `GET /projects/:id/protected_tags` |
| Checks before merge | Merge check "Pipelines must succeed" | Free | `GET /projects/:id`, `only_allow_merge_if_pipeline_succeeds` |
| No direct pushes | Branch rule "Allowed to push and merge: No one" | Free | `push_access_levels` on protected branches |
| Required approvals | Merge request approval rules | Premium | `GET /projects/:id/approval_rules` |
| Secrets at push | Secret push protection | Ultimate | `GET /projects/:id/security_settings` |
| Secret scan in CI | Pipeline secret detection | Free (report artifact only) | the CI file |
| SAST | SAST template | Free (report artifact only) | the CI file |
| Dependency scanning | Dependency scanning | Ultimate | the CI file; a free scanner such as OSV-Scanner in CI instead |
| Dependency updates | none built in | — | Renovate (third party; not checked) |
| Private reporting | Service Desk (confidential tickets) | Free | `GET /projects/:id`, `service_desk_enabled` |
| Immutable releases | none | — | — |
| 2FA | Group 2FA enforcement | Free | `GET /groups/:id` (field not confirmed) |
| CI token | Job token allowlist, default "only itself" | Free | `GET /projects/:id/job_token_scope` |
| SHA pins | Components accept a SHA; nothing enforces it | Free | the CI file |
| Fork pipelines | Run in the fork by default; a parent member starts a parent pipeline by hand | Free | `ci_allow_fork_pipelines_to_run_in_parent_project` (not confirmed) |
| Description, topics | Project description and topics | Free | `GET /projects/:id`, `description`, `topics` |

The `glab api <endpoint>` command reads any endpoint
(`docs.gitlab.com/cli/api/`).

## AGENTS.md

In "Audit an existing project", before "Judge each rule", add a step: if the
repository is on a host that has a profile, read it with
`uss show standard/vendors/<host>/<host>.md`, and use its mapping table and
read-back commands for the host settings. Keep "Judge each rule" steps 12 and
13 (host settings) as they are. Check every other document that describes
vendor profiles as tools only (`docs/architecture.md`, `CONTRIBUTING.md`, the
design spec), and update it in the same change.

## The release CI template

`templates/ci/uss-release.yml` runs on `release: published` and then runs
`gh release upload "$TAG" uss-report.md --clobber`. GitHub's guidance for
immutable releases is to create the release as a draft, attach every asset,
then publish it. GitHub's documentation does not say whether adding an asset
after publication works; replacing one (`--clobber` on a second run) does
not. The builder chooses the mechanism. The result must:

- put the report in the release without changing its tag or its files after
  it is published. Immutable releases still allow edits to the title and
  notes, and USS-REL-012 accepts the report in the notes or as a file, not
  as a link. So two routes work: attach the file to a draft before
  publishing, or write the report into the notes after publishing (my call:
  either is acceptable; the builder picks the simpler one and says why);
- still fail the release check when `uss release-check` fails;
- keep the template's comments and the AGENTS.md text ("It attaches the report
  to the release") in step with what it does;
- keep the existing template tests in `tools/uss/src/cli/mod.rs` passing, and
  add a test for the new trigger or step.

Read the GitHub documentation on release events and immutable releases before
choosing.

## This repository

The new rules apply to this repository. After the change:

- add a decision for each new MAY rule to `.uss/record.yaml` (both
  `adopt: true`: this repository meets both);
- run `uss check .uss/record.yaml` until it reports no problems;
- tell the maintainer to add "Require code scanning results" to the `main`
  ruleset, if GitHub offers it free on a public repository, so that CodeQL
  meets USS-SEC-028. The agent cannot change repository settings.

The re-audit of this repository stays a separate item in
`docs/superpowers/remaining.md`.

## Left to the builder

- Final rule IDs (the next free numbers), titles, Verify texts, rationales and
  score categories. My calls for the categories: Safety for the SUP, SEC and
  REL rules; Contributability for the two DOC rules, because a description and
  topics help people find the project and join it. The builder may choose
  otherwise with a reason.
- Where in the Verify texts the GitHub and GitLab examples go.
- The exact layout of the baseline block and the guards of each profile.
- Which mapping rows are `full` and which are `partial`.
- The mechanism of the release template change.
- Whether a lint change is needed (expected: none).

## Done when

- `cargo xtask check` passes.
- `uss lint` accepts both profiles, and each maps every applicable MUST and
  MUST NOT rule within its host's reach.
- Each new or changed reference was read from its source, with the date.
- `uss check .uss/record.yaml` reports no problems.
- `docs/superpowers/remaining.md` lists this work as done and keeps the
  re-audit item.
