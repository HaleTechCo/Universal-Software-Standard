# GitHub platform and governance decisions before USS 0.1.0 (follow-ups 2.1, 2.2, 2.3, 2.4, 4.5)

All sources were read on 2026-11-25. "Inference" means my reasoning, not a source. Read-only `gh api -X GET` calls on this repository gave the local facts (org plan `free`, 1 seat, repo private, `allow_forking` false, fork-PR-workflows-for-private-repos all false, 149 commits all by one author, CI job `check` with name `check (${{ matrix.os }})`).

## 2.2 Repository visibility (and what to do before going public)

### Takeaway
RECOMMENDED: make the repository public (effort M, cost $0). It is the only path that unlocks, for free, rulesets/branch protection, required checks, private vulnerability reporting, secret scanning, push protection, CodeQL, artifact attestations and fork-PR settings. Do the history scan and personal-data review first. Alternative paths are in the table below; GitHub Team ($4/user/month) fixes only SUP-007/008/CON-003 and leaves the rest unmet or paid.

Alternatives (one line each):
- Team, stay private: $4/month for one seat; gives rules and CODEOWNERS and 3,000 Actions minutes, but private vulnerability reporting is documented for public repos only, attestations need Enterprise Cloud for private repos, and secret scanning/CodeQL cost $19 + $30 per active committer extra (about $53/month for one committer in total).
- Enterprise Cloud, stay private: $21/user/month; the only plan with attestations on private repos; overkill.
- Stay on Free and private: SUP-007, SUP-008, CON-003, CTB-012 keep failing (HTTP 403 "Upgrade to GitHub Pro or make this repository public", as in the audit).
- Move the repo to a personal account with Pro: not researched here; one person only, and CTB-016 is then harder.

### Cited Findings
- Free plan: "Repository rules, code owners, and draft pull requests are limited to public repositories." Team ($4 per user/month) adds repository rules, code owners, required reviewers, 3,000 CI/CD minutes, 2 GB Packages. Enterprise $21 per user/month, 50,000 minutes. Free includes 2,000 minutes, 500 MB Packages. — [github.com/pricing](https://github.com/pricing)
- Rulesets are available to customers on GitHub Team and GitHub Enterprise plans; rules include block force pushes, require signed commits, tag rulesets (control who can delete or rename a tag); the page does not separately address public repos. — [About rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets)
- Free for public repositories on all plans: security policy, dependency graph, Dependabot alerts/security updates/version updates, repository security advisories, repository rulesets; push protection and secret scanning alerts for users on by default; code scanning, CodeQL CLI, Copilot Autofix and dependency review on by default. — [GitHub security features](https://docs.github.com/en/code-security/getting-started/github-security-features)
- Private repositories on Free and Team: Dependabot alerts etc. are free; secret scanning alerts for users, push protection, custom patterns need GitHub Secret Protection; code scanning, CodeQL, dependency review need GitHub Code Security; artifact attestations on private repos are only on GitHub Enterprise Cloud. — [same page](https://docs.github.com/en/code-security/getting-started/github-security-features)
- Prices: GitHub Secret Protection $19 and GitHub Code Security $30 per active committer per month. — [github.com/security/advanced-security](https://github.com/security/advanced-security) (the page does not say whether Team orgs are eligible; the billing doc says active committers must hold a Team or Enterprise licence). An active committer pushed a commit in the last 90 days. — [GHAS billing](https://docs.github.com/en/billing/concepts/product-billing/github-advanced-security)
- Private vulnerability reporting: docs describe it for public repositories ("Owners and administrators of public repositories can enable private vulnerability reporting"); enable it in Settings > Advanced Security > "Private vulnerability reporting" > Enable. The docs do not say whether private repositories can use it. — [Privately reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability), [configuring](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configuring-private-vulnerability-reporting-for-a-repository)
- Attestations meet SLSA v1.0 Build Level 2 alone (Level 3 with a reusable workflow); public repos publish to a public transparency log, private repos use GitHub's Sigstore instance with no transparency log. — [Artifact attestations](https://docs.github.com/en/actions/concepts/security/artifact-attestations)
- Actions: standard GitHub-hosted runners are free for public repos. Private-repo included usage is in dollars: Free $12, Pro $18, Team $18, Enterprise $300 per month. Per-minute rates: Linux 2-core x64 $0.006, Windows 2-core $0.010 (about 1.7x), macOS $0.062 (about 10x). The page states rates, not "multipliers"; ratios are derived. — [Actions billing](https://docs.github.com/en/billing/managing-billing-for-your-products/about-billing-for-github-actions)
- Fork PR workflows, public repo: Settings > Actions > General > "Approval for running fork pull request workflows from contributors": first-time contributors new to GitHub / first-time contributors (default) / all external contributors. Private repo: options "Run workflows from fork pull requests" (read-only token, no secrets), "Send write tokens", "Send secrets", "Require approval for fork pull request workflows"; an org can disable a policy for its repos. — [Managing Actions settings](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository)
- Forking a private repo: the org owner must first allow forks of private repos, then the repo setting "Allow forking" works. Plan requirement not stated. — [Forking policy](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/managing-the-forking-policy-for-your-repository)
- Push protection for repositories "Requires GitHub Secret Protection" on private repos; Team/Enterprise orgs can run a free report that scans for leaked secrets. — [About push protection](https://docs.github.com/en/code-security/concepts/secret-security/about-push-protection)
- Local: org plan is `free` with 1 filled seat; repo `visibility: private`; `security_and_analysis: null`. — `gh api -X GET orgs/...`, `gh api -X GET repos/...` (read 2026-11-25)

### Inferences
- Public fits the project: USS is an MIT standard that wants outside contributors (README, CTB rules), README/SECURITY.md already assume PVR and a public marketplace, and the plugin marketplace and the headless-audit CI template (follow-ups 2.2, 4.8) need public.
- Private vulnerability reporting is probably not usable while private; treat as public-only until proven.
- Before going public (my checklist; going public is hard to reverse because clones and forks keep content):
  1. Scan the full history (all refs, `.git` of each worktree), not just the tree, for secrets: gitleaks or trufflehog `git` mode; also covers USS-SEC-004, which is `unverified` today. Rotate anything found; rewriting history is the last resort and must happen before publishing.
  2. Personal data: all 149 commits carry one personal Gmail author address. Decide whether to keep it, or rewrite history to a GitHub noreply address before publishing (rewrite only before publishing); also check `docs/superpowers/*`, `todo.txt`, `Initial_Idea.md`, `.claude/worktrees` references, local paths and names in plans and audit evidence (e.g. `C:\Users\...` paths), and the AI co-author trailers.
  3. Check `.gitignore`d files do not matter, but also check large/binary files and the `.uss/audits` evidence for private info.
  4. Fill the code-of-conduct contact and confirm `SECURITY.md` wording (the "button is missing" fallback should be removed once PVR is on).
  5. After publishing, in this order: enable PVR, secret scanning + push protection, Dependabot alerts/updates, CodeQL default setup, create rulesets (section 4.5), set fork-PR approval, then re-run the audit.
  6. A public repo changes Actions behaviour: scheduled workflows are disabled after 60 days of no repo activity (see 2.4).

### Gaps
- Whether PVR works on a private repository: docs silent.
- Whether Team-plan orgs can buy Secret Protection / Code Security: the pricing page does not say; the billing doc implies a Team licence suffices.
- Whether "Allow forking" of private repos needs a paid plan: not stated.
- Whether the Free-plan secret-risk report exists on Free: docs mention Team/Enterprise only.

## 2.4 CI cost of the weekly three-OS run

### Takeaway
RECOMMENDED: if the repo goes public, change nothing (cost $0; keep weekly three-OS, effort S: none). If it stays private, run the scheduled trigger on Linux only (effort S, change the matrix with `if: github.event_name != 'schedule' || matrix.os == 'ubuntu-latest'` on the job) because the advisory check does not depend on the OS; cost then stays within the Free $12 allowance (about $0.02 per run). Schedule: weekly is enough; keep it, but move off the top of the hour (the cron `23 5 * * 1` already does).

Alternatives: keep three OS weekly private (cold-run estimate about $0.28 per run, about $1.2/month, plus push/PR runs; still inside $12); run daily Linux only (about $0.7/month); monthly; or drop the schedule and rely on Dependabot alerts (free; they flag advisories for the lockfile) with cargo-deny only on push.

### Cited Findings
- Rates and allowances: see 2.2 (Linux $0.006, Windows $0.010, macOS $0.062 per minute; Free $12 included; public standard runners free). — [Actions billing](https://docs.github.com/en/billing/managing-billing-for-your-products/about-billing-for-github-actions)
- Scheduled workflows run on the default branch and may be delayed or dropped at the top of the hour; in public repos they are automatically disabled after 60 days without repo activity. — [Events that trigger workflows](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows)
- Measured job times from the repo's own runs (read with `gh run view`, 2026-11-25): latest push run macOS 2m14s, Windows 5m39s, Ubuntu 3m15s; earlier run macOS 1m50s, Windows 6m47s, Ubuntu 3m01s; cached runs about 0.5 to 1.5 minutes.
- `.github/workflows/ci.yml` lines 3-9, 16-20: triggers push to main, pull_request, weekly cron; matrix `[ubuntu-latest, windows-latest, macos-latest]`.

### Inferences
- Estimated cold-run cost (minutes rounded up per job, my arithmetic): macOS 3 x $0.062 = $0.186, Windows 7 x $0.010 = $0.07, Linux 4 x $0.006 = $0.024, about $0.28 per run; about 4.3 runs a month gives about $1.2/month, 10% of the Free $12. The weekly run is not the budget risk; PR and push runs on macOS are, if activity grows.
- A Free-plan org cannot overspend by default unless a spending limit is set (not verified in docs fetched).
- On a public repo, an idle repo could lose the weekly run after 60 days; Dependabot PRs might count as activity, which I could not confirm.

### Gaps
- Whether Dependabot or scheduled-run commits count as "repository activity" for the 60-day rule.
- Exact billing rounding and any per-job minimum: not in the page read.

## 2.1 Values in the documents

### Takeaway
RECOMMENDED: keep 7 days acknowledge and 14 days assess; change the fix/mitigate target from 90 to 60 days for medium or higher severity (critical: "rapidly", say 30 days as a target), because the OpenSSF Passing criterion forbids unpatched medium+ vulnerabilities publicly known for more than 60 days and Project Zero moves to a 60-day window on 2027-02-02. Keep 7 days for first response to contributions (stricter than any norm found; if a single maintainer cannot keep it, use 14). Succession in GOVERNANCE.md is not enough as written. For the code of conduct, use a role address (e.g. `conduct@<domain the org controls>`) that forwards to two people. Effort S each; cost $0 (a domain is about $10-15/year if the org has none; not sourced).

Alternatives: CoC via a web form (needs hosting; weaker for private reports); CoC via the maintainer's personal email (meets the Verify text but is a conflict of interest when the report is about the maintainer); keep 90 days (matches Project Zero today, but fails the OpenSSF 60-day criterion in practice for medium+).

### Cited Findings
- OpenSSF Passing: `vulnerability_report_response`: initial response time for any vulnerability report in the last 6 months MUST be 14 days or less; `vulnerabilities_fixed_60_days`: no unpatched vulnerabilities of medium or higher severity publicly known for more than 60 days (MUST); `vulnerability_report_private` MUST say how to send privately if supported; `report_responses` MUST acknowledge a majority of bug reports from the last 2-12 months (no 14-day tie, and no fix required); `enhancement_responses` SHOULD respond to a majority. — [bestpractices.dev criteria](https://www.bestpractices.dev/en/criteria/0)
- OpenSSF Silver: `access_continuity` MUST be able to continue with minimal interruption if any one person dies or is incapacitated (text truncated in my fetch); `bus_factor` SHOULD be 2 or more (Silver), MUST (Gold); `code_of_conduct` MUST be adopted and posted in a standard location (Silver); `dco` SHOULD have a legal mechanism for contributors to assert the right to submit (Silver); `vulnerabilities_critical_fixed` SHOULD fix critical vulnerabilities rapidly. — [criteria.md](https://raw.githubusercontent.com/coreinfrastructure/best-practices-badge/main/docs/criteria.md), [other.md](https://github.com/coreinfrastructure/best-practices-badge/blob/main/docs/other.md)
- Project Zero: full disclosure after X days, X is currently 90; moving to 60 on 2027-02-02 and 30 on 2028-01-03; 7 days when exploited in the wild. — [Project Zero policy](https://projectzero.google/vulnerability-disclosure-policy.html)
- CERT/CC: generally targets publication 45 days after the initial contact attempt, or another negotiated date; clock starts at first vendor contact. — [CERT/CC search result, CVD guide](https://certcc.github.io/CERT-Guide-to-CVD/tutorials/coord_certcc/vendor) (secondary summary of a 2000-era policy; treat as indicative)
- ISO/IEC 29147:2018 has clauses on report tracking and acknowledgement; ISO/IEC 30111 is the vendor handling standard (2019 2nd ed.). I found no public statement of a numeric acknowledgement deadline. — [DIN TOC](https://www.din.de/de/mitwirken/normenausschuesse/nia/veroeffentlichungen/wdc-beuth:din21:324674445/toc-3182182/download)
- CHAOSS Time to First Response gives no benchmark. — [CHAOSS metric](https://chaoss.community/kb/metric-time-to-first-response/)
- Contributor Covenant 3.0: the Reporting section is a placeholder "[NOTE: describe your means of reporting here.]"; it gives no guidance on channel choice; investigations are handled by Community Moderators and enforcement is private. — [Contributor Covenant 3.0](https://www.contributor-covenant.org/version/3/0/code_of_conduct/)
- Repo: SECURITY.md 7/14/90 days; GOVERNANCE.md succession is "HaleTechCo appoints a new maintainer or archives"; the maintainer is also Community Moderator; the org has one owner/admin (audit USS-CTB-016, `gh api orgs/.../members?role=admin` -> one member).

### Inferences
- 7 and 14 days sit inside the only numeric norm (14 days initial response). Anything stricter than 14 for first response is a self-chosen service level.
- The 90-day fix time is a Project Zero number for researchers deciding when to publish, not a requirement on maintainers; the OpenSSF 60-day rule is the one an adopter would be measured against.
- Succession gap: HaleTechCo is only a legal wrapper if a second person is an org owner; with one owner who is also the maintainer, "HaleTechCo appoints" depends on the same person. Fix: name a second org owner (links to CTB-016), state in GOVERNANCE.md who that is or how the owner is found (for example a documented recovery contact), and where release keys/credentials live. A dormant name does not prove continuity.
- CoC: a role address that forwards to the maintainer plus the second org owner avoids reports about the maintainer going only to the maintainer. Update the "How to report" line, replace "added before the first release", and describe who handles conflicts of interest.

### Gaps
- Full text of `access_continuity` (truncated in fetch) and ISO 29147/30111 numeric timelines (paywalled).
- I did not verify what share of projects use a role address vs a form for CoC contact; no source ranks the channels.

## USS-CTB-009 DCO, CLA or nothing

### Takeaway
RECOMMENDED: DCO enforced by a small CI job (effort S, $0), plus the web-commit sign-off setting. Add to CONTRIBUTING.md the DCO v1.1 text link and "`git commit -s`", and add a `dco` job in a new `pull_request`-only workflow (or in ci.yml) that fails if any commit in `base..head` lacks `Signed-off-by:` matching the author (Linux-only, `permissions: contents: read`, checkout `fetch-depth: 0`). Verify text: "contributing guide asks for a Signed-off-by line... A CI job or a repository app checks... no CLA" — met by this.

Alternatives: the DCO GitHub App (zero code; third party; I could not confirm it works on private repos or its pricing; fine on a public repo); repo/org setting "Require contributors to sign off on web-based commits" (covers GitHub web UI only; CLI needs `--signoff`; not a check by itself); a CLA (heavier, needs a signing service, unnecessary for MIT inbound=outbound); nothing (CTB-009 stays a fail unless a Standard Deviation is written).

### Cited Findings
- DCO app: requires each commit message to carry a `Signed-off-by` line whose email matches the commit author; fails the status otherwise; contributors use `git commit -s`; third-party, separate terms. Private-repo support, price and maintenance not stated on the page. — [DCO app](https://github.com/apps/dco)
- Web sign-off setting: Settings > Commits > "Require contributors to sign off on web-based commits"; applies only to commits made through the GitHub.com web interface; CLI commits need `--signoff`. Plan requirements not stated. — [Commit signoff policy](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/managing-repository-settings/managing-the-commit-signoff-policy-for-your-repository)
- OpenSSF Silver `dco` SHOULD: a legal mechanism (DCO or CLA) for contributors to assert rights. — [other.md](https://github.com/coreinfrastructure/best-practices-badge/blob/main/docs/other.md)
- The rule (standard/core/contributability.md USS-CTB-009, SHOULD) says: sign-off line asked in the guide, CI job or repository app checks each PR commit, no CLA, and outside-team commits since the last audit have the line. Audit: all 149 commits are by one author without sign-off; none from outside.

### Inferences
- A CI job works on private free-plan repos because it is just Actions (within minutes). The App's private-repo behaviour is unconfirmed.
- The check must not be "required" until section 4.5 rulesets exist; then add `dco` to the required checks.
- The maintainer's own commits are "inside the team"; the rule only demands the line on outside commits, but a uniform check is simpler (maintainer uses `git config format.signoff true`).

### Gaps
- DCO 1.1 text was not fetched (developercertificate.org); the rule references DCO 1.1.
- Whether the web sign-off setting is available on the Free plan.

## USS-CTB-016 Second person with admin access

### Takeaway
RECOMMENDED: invite one trusted person (a co-maintainer or an individual the maintainer designates for emergencies) as an organisation Owner and as repo admin (effort S, $0 on Free: seats are not capped on Free per the org data, `seats: 0`; on Team +$4/month). Document it in GOVERNANCE.md. This also fixes the succession gap in 2.1.

Alternatives for a one-person organisation: (1) a second person with the Owner role who takes no part in reviews (the rule counts this); (2) write a Standard Deviation for CTB-016 (SHOULD, so no effect on compliance; only the score); (3) a second account of the same person: does not satisfy "at least two people" (my reading), so do not rely on it.

### Cited Findings
- The rule: "at least two people have admin access to the repository and can publish a release. A backup who takes no part in reviews counts here, unlike for the fact `has-multiple-maintainers`." — standard/core/contributability.md, USS-CTB-016 (read 2026-11-25).
- Audit: one collaborator with role admin; one org admin; Cargo.toml `publish = false`. — .uss/audits/2026-11-25.yaml USS-CTB-016
- Rule references OpenSSF `access_continuity` and `bus_factor` and CHAOSS Contributor Absence Factor.

### Inferences
- The task note says "a dormant backup admin does not count, per the project's notes", but the rule text itself says a backup who takes no part in reviews counts. I found the rule text more permissive than the note; resolve by editing whichever is wrong. A backup who is reachable and can publish a release meets the Verify; a person who cannot be reached or has no 2FA arguably fails the intent of `access_continuity`.
- Release-publishing ability needs only repo admin/maintain or the release workflow, so no extra step on GitHub; if a registry is added later, the second person needs access there too.

### Gaps
- No source ranks options for one-person orgs.

## USS-BAK-013 Mirror to a second host

### Takeaway
RECOMMENDED: a separate GitHub Actions workflow (`mirror.yml`, on push to main, on tag push and weekly) that pushes all branches and tags to a private project on GitLab.com Free or a Codeberg repo using a write deploy key stored as an Actions secret (effort S-M, $0). Use `git push --prune <url> '+refs/heads/*:refs/heads/*' '+refs/tags/*:refs/tags/*'` (or `--mirror` from a `fetch-depth: 0` checkout); the job must not run on `pull_request` (keeps USS-SUP-011 and fork PRs away from the secret); pin actions by SHA. Check afterwards with `git ls-remote` on both hosts, as the Verify says.

Alternatives: GitLab push mirror (Free tier; but it pushes FROM GitLab, so only helpful if GitLab is primary); GitLab pull mirror of GitHub (not verified; I recall it is a paid tier); Codeberg pull mirror (a secondary source says pull mirrors are disabled by admin on Codeberg; unverified); a cron on a developer machine or a home server running `git push --mirror` (cheap but not "automatic" on a host independent of the maintainer); a webhook to a mirroring service (extra moving part).

### Cited Findings
- GitLab push mirroring is available on Free, Premium and Ultimate (GitLab.com, Self-Managed, Dedicated); push mirrors update within five minutes of a push to the upstream repository (within one minute when only protected branches are mirrored); up to 10 push mirrors; do not push to the mirror; auth shown is username plus token; SSH not mentioned. Whether it supports private projects is not stated. — [GitLab push mirroring](https://docs.gitlab.com/user/project/repository/mirror/push/)
- Forgejo push mirror: configured in Settings > Repository > Mirror Settings; can generate an SSH key pair whose public key is added as a deploy key on the target; uses `git push --mirror` without a branch filter and overwrites the remote; LFS over SSH not mirrored. — [Forgejo mirror docs](https://forgejo.org/docs/latest/user/repo-mirror/) (via search summary)
- Codeberg's own migration page did not cover mirrors; one third-party tool states Codeberg pull mirrors are disabled by admin (secondary). — [Codeberg migrating docs](https://docs.codeberg.org/advanced/migrating-repos/); [gitmeout](https://github.com/guilycst/gitmeout)
- Local: no webhook, no deploy key, one remote (audit USS-BAK-013).

### Inferences
- A deploy key limited to the one mirror repo cannot touch GitHub, so a leak harms only the backup. Secret scope: use an environment secret (public repos get environments free) and run on `push` and `schedule` only.
- With the repo public, Actions minutes are free; private, a Linux job of one minute costs about $0.006 per run.
- Mirrors of a private source can be private on GitLab; Codeberg private repository allowance not verified.

### Gaps
- GitLab Free private-project mirroring and pull-mirror tiers; Codeberg mirror and private-repo policy (Codeberg's own docs not retrieved).

## 4.5 Exact ruleset for USS-SUP-007, USS-SUP-008, USS-CON-003 and USS-CTB-012

### Takeaway
RECOMMENDED: after the repo is public (or on Team), create two rulesets, active, with no bypass list, plus the fork-PR setting. Effort S, $0.

Ruleset 1, "main" (target: default branch): rules `deletion`, `non_fast_forward`, `required_status_checks` with `strict_required_status_checks_policy: true` and contexts `check (ubuntu-latest)`, `check (windows-latest)`, `check (macos-latest)` (the job name with matrix expanded; add `dco` once it exists), and `pull_request` with 0 required approvals (a solo maintainer cannot approve their own PR; 1 would lock them out unless a second person exists). Bypass: none, so admins are bound; if that is too strict for the solo maintainer, add "Repository admin" with bypass mode `pull_request` only.
Ruleset 2, "release tags" (target: tags `refs/tags/v*`): rules `deletion`, `non_fast_forward`, `update` (blocks moving or deleting a release tag; creation stays allowed).
Settings > Actions > General: public repo "Require approval for first-time contributors" (default; or "all external contributors"); workflow permissions read-only (already `default_workflow_permissions: read`). No secrets or write tokens for fork PRs (already the case: no Actions secrets).

Example API call (verify with `gh api -X GET repos/{owner}/{repo}/rulesets` afterwards; the JSON shape is from my knowledge of the REST schema and was not fetched from the docs, so test in "evaluate" mode first if available or check the response):
```
gh api -X POST repos/{owner}/{repo}/rulesets --input - <<'EOF'
{"name":"main","target":"branch","enforcement":"active",
 "conditions":{"ref_name":{"include":["~DEFAULT_BRANCH"],"exclude":[]}},
 "rules":[{"type":"deletion"},{"type":"non_fast_forward"},
  {"type":"pull_request","parameters":{"required_approving_review_count":0,"dismiss_stale_reviews_on_push":false,"require_code_owner_review":false,"require_last_push_approval":false,"required_review_thread_resolution":false}},
  {"type":"required_status_checks","parameters":{"strict_required_status_checks_policy":true,
   "required_status_checks":[{"context":"check (ubuntu-latest)"},{"context":"check (windows-latest)"},{"context":"check (macos-latest)"}]}}]}
EOF
```
Tag ruleset: same call with `"target":"tag"`, `"include":["refs/tags/v*"]`, rules `deletion`, `non_fast_forward`, `update`.

Alternatives: classic branch protection (`PUT repos/{o}/{r}/branches/main/protection`: required status checks, `allow_force_pushes:false`, `allow_deletions:false`, `enforce_admins:true`; no tag protection except the older "protected tags"); a ruleset with 1 required approval once a second admin exists (and CODEOWNERS review); keep checks required only on Linux (the Verify for CON-003 needs the job that runs `cargo xtask check`, so require all three or accept a Linux-only DoD and say so).

### Cited Findings
- By default a branch protection rule blocks force pushes and branch deletion; admins are not bound unless "Do not allow bypassing" is enabled; job names should be unique across workflows to avoid ambiguous required checks. — [About protected branches](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches)
- Rulesets target branches and tags by fnmatch pattern and support block force pushes; tag rulesets control delete and rename; only Active and Disabled statuses are listed. — [About rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets)
- Fork PR approval options for public repos (see 2.2) — [Actions settings](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository)
- Rule Verify texts: SUP-007 "block force-pushes and deletion of the default branch and of release tags"; SUP-008 requires CI checks before merge (audit evidence reads `protected`, `required_status_checks`, `rulesets`); CON-003 each DoD check "is a required check on the default branch"; CTB-012 CI runs on fork PRs, "settings... do not stop these runs, or only hold them for approval by a maintainer" — standard/core/supply-chain.md, conduct.md, contributability.md.
- ci.yml: job id `check`, name `check (${{ matrix.os }})`, so contexts are `check (ubuntu-latest)`, `check (windows-latest)`, `check (macos-latest)`. The workflow already uses `pull_request` (not `pull_request_target`) with read-only token and no secrets (audit USS-SUP-010, SUP-011). — .github/workflows/ci.yml, audit

### Inferences
- Required status checks mean even the sole maintainer must merge through a PR; direct pushes of unchecked commits to main will be rejected. That changes the solo workflow (feature branch, PR, wait for three OS runs, merge); with "strict" the branch must be up to date. A bypass for the admin in `pull_request` mode keeps a fix-forward path.
- Matrix check names appear only after a run has reported them; create the ruleset after one CI run on the PR branch, and keep the names in sync with ci.yml if the matrix changes (renaming the job silently makes merges wait forever).
- Tag rule `update` plus `deletion` protects release tags; creation of `v*` tags by the releaser stays possible. The release workflow in templates/ci/uss-release.yml pushes no tags, so no conflict is expected (not verified).
- CTB-012 on a public repo: forks are always possible; the Verify passes with the default first-time-contributor approval. I could not read this setting on a public repo from here; verify after publishing with `gh api -X GET repos/{o}/{r}/actions/permissions` and the fork-PR approval endpoint (I believe `.../actions/permissions/fork-pr-contributor-approval`; unverified).

### Gaps
- Official REST schema for rulesets was not fetched; the JSON above is from my knowledge and must be checked against the docs or the GET response.
- Whether Free orgs may use rulesets on public repos is supported by the pricing page statement that rules are "limited to public repositories" on Free, but the rulesets page does not say so itself.
