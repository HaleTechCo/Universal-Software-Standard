# Going public unblocks most USS follow-ups

Six decisions unblock most of the open follow-ups, and the one that unblocks the
most is making the repository public: at no cost it clears the four
GitHub-settings results (branch protection, required checks and fork runs),
private vulnerability reporting, secret and code scanning, free ARM64 runners,
artifact attestations (the cheapest route to signatures and SLSA provenance),
the public plugin marketplace and the headless-audit template. Two steps must
come first, because publication cannot be undone: a secret scan of the full
history, and a decision on the personal email address in the author field of all
149 commits, since a rewrite changes every commit SHA and is only safe while
nobody else holds a clone. The notes agree that the install defect (ruling R10)
should be fixed before the trial, and the strongest correction they make is to
reverse ruling R8 and plan decision 12: the release audit runs before the tag,
so under the current reading it can never see the release it judges, and `uss
release-check --require-compliant` can never pass on a first release. All four
questions about the standard resolve by narrowing or adding one sentence of rule
text, and each is cheap only before 0.1.0, after which a change of meaning costs
a retired rule ID. Seven of the fifteen code results share one output layer in
the CLI, and the scanner and ShellCheck results share one xtask helper for
pinned, hash-checked downloads. Metadata becomes six rules in the Distribution
area (one MUST that defers to each host's own validator, four SHOULDs, one MAY),
gated by the existing but unused fact `distributed-as-package`; locale becomes
an `Enables` on USS-I18N-006, 13 new I18N rules, an optional `project.language`
record key for the language of evidence and reports, and an English-is-normative
translation model built on po4a and Fluent. In this report, "not checked" marks
a claim that no source confirmed, and **Decide** marks a choice that only the
maintainer can make. Effort is S (hours), M (days) or L (a week or more).

## Decide first: six choices, in the order to take them

The first three must precede any publication. The last three set what the 0.1.0
release audit can pass.

| # | Decision | Recommendation | Unblocks | Effort |
|---|---|---|---|---|
| 1 | Ruling R10, the install defect | Reverse it: fix the defect now (section 4.1). | A trial of the text that ships; agents stop updating the developer's stable toolchain. | S |
| 2 | The history before publication | Run a secret scan of every ref, including the worktree branches, and rotate anything found. **Decide:** rewrite the personal author address to a GitHub noreply address, or accept that it becomes public for good. | Decision 3; the evidence for USS-SEC-004. | S |
| 3 | Repository visibility (2.2) | Make the repository public. | 2.2, 2.4, 4.5, SUP-013 to SUP-015, DST-014, SEC-028, CTB-007, CTB-012, the public marketplace and the headless template (4.8). | M |
| 4 | A second admin (USS-CTB-016) | Appoint a second person as organisation owner and repository admin. | CTB-016, the succession text, the code of conduct contact, a review rule with one approval. | S |
| 5 | Ruling R8 and plan decision 12 | Reverse them: before the first release, judge the release process at the audited commit (C1). | A first release audit that can pass. | S |
| 6 | Changes of meaning before 0.1.0 | Make them now, in place (list below). | Rule changes without retired IDs. | M |

A history rewrite changes the SHA of every commit. The audited commit of
`.uss/audits/2026-11-25.yaml` (51435cb) then no longer exists, so the self-audit
runs again; the four worktree branches go first (section 5); and `main` is
force-pushed, which is safe only while the repository is private and has no
forks. Accepting the address costs nothing now but cannot be withdrawn later,
because clones and forks keep the history. Either way, set a noreply address for
future commits. The same pass reviews personal data in `docs/superpowers/`,
`todo.txt`, `Initial_Idea.md`, local user paths in plans and audit evidence, and
the AI co-author trailers. The scan uses the same gitleaks binary that section
4.4 adds to CI.

Decision 6 is a window, not one choice. Conventions section 4 makes a change of
meaning retire the rule and take a new ID, so these changes are cheap only
before 0.1.0: the `stores-persistent-data` definition and the USS-PRF-002,
USS-DEP-006 and USS-TST-003 texts (section 3); the USS-K8S-007 wording and the
CTB-005, PRF-005, SAF-004, SAF-007, TST-011 and TST-012 refinements (2.5); the
`distributed-as-package` definition (section 7); and USS-I18N-006 with its
`Enables` (section 8). New rules are MINOR work and can wait for 0.2.0.

## Twelve conflicts between the notes, and how this report resolves them

| # | Conflict | Resolution |
|---|---|---|
| C1 | The rulings notes reverse R8 and decision 12; the release notes plan under R8 (rebuild steps wait for the release, CTB-007 uses the checkout install until the tag exists). | Reverse R8. The release audit precedes the tag (AGENTS.md "Release audit", steps 2 and 5), so under decision 12 the 0.1.0 audit sees no release and SUP-013, SUP-014, SUP-015 and DST-002 fail even when the workflow signs and attests everything. The release plan loses nothing: the notices, rebuild steps and workflow must exist before the tag anyway, and CTB-007 uses a named substitute, which AGENTS.md "Judge each rule" step 9 already allows. The audit only stops hiding gaps: LIC-004 (a MUST), DST-014, REL-003 and REL-011 become `fail` until the release process exists. OpenSSF Scorecard treats a repository with no releases as not applicable ([releasesAreSigned](https://github.com/ossf/scorecard/blob/main/probes/releasesAreSigned/impl.go)), the nearest precedent; no standard defines this case. **Decide:** every later release audit also precedes its tag, so consider the same reading for all release audits, plus a check of the previous release's artifacts. |
| C2 | The follow-ups call the R10 fix optional for the trial; both notes say fix it first. | Fix it first. The trial installs `uss` before the audit, so step 5 never runs, but the fix is S, the trial then tests the text that ships, and the defect changes the developer's machine against the intent of USS-SUP-002 and "Agent conduct". |
| C3 | The release notes could not confirm `--no-self-update`; the rulings notes saw it. | The flag exists: `rustup toolchain install --help` (rustup 1.29.3, local run) lists `--no-self-update` and `--no-update`, and describes the command as "Install or update the given toolchains". |
| C4 | The release notes add `claude plugin validate --strict` to the trial; the metadata notes ran it, and it fails on `ICON_MISSING`. | Use `claude plugin validate .` without `--strict`. The icon is read only by Anthropic's directory, which "Claude Code ignores at load time" ([plugin reference](https://code.claude.com/docs/en/plugins-reference)); the DST-019 draft excludes warnings about hosts the project does not use. |
| C5 | The references notes say vendor-profile CI runs free because the repository is public. | The repository is private (`gh api`, read by two other notes). The vendor-profile and fuzz workflows cost Linux minutes until 2.2. |
| C6 | Catalogue location: `tools/uss/messages/en-US.ftl` (tooling) or `i18n/en/*.ftl` (locale). | Use `i18n/en/` at the repository root: one tree for every language and for the translated standard, embedded by `build.rs`, and the shortest tag, as W3C advises ([Choosing a language tag](https://www.w3.org/International/questions/qa-choosing-language-tags)). |
| C7 | The locale notes read `LC_ALL`, `LC_MESSAGES` and `LANG` for messages; the tooling notes keep "uss reads no environment variable", which `docs/cli.md` states and the UX-003 and SEC-002 evidence quotes. | Use `--lang` only until a second catalogue ships. Then add the POSIX lookup and change `docs/cli.md` in the same commit. The report language never comes from the environment (section 8B). |
| C8 | The follow-ups suggest `--rev <sha>` in the CI template; the release notes keep `--tag`. | Keep `--tag`: the template lives in the release commit, so it cannot hold that commit's SHA. Protect `v*` tags with a ruleset (4.5). |
| C9 | The rulings notes put the dependency checks in commits, because the repository has no pull requests; the governance notes add a ruleset that requires pull requests. | Do both. Once the ruleset exists the PR template also runs, but the unit test in 4.7 is what enforces the rules. |
| C10 | A project note says a dormant backup admin does not count; USS-CTB-016 says "A backup who takes no part in reviews counts here". | The rule text governs. Count a backup who is reachable, uses two-factor authentication and can publish a release; correct the note. |
| C11 | One GitHub page says attestations work on all current plans; the action's README and dist's docs say private repositories need Enterprise Cloud. | Assume they do not work on a private Free repository ([attest-build-provenance README](https://raw.githubusercontent.com/actions/attest-build-provenance/main/README.md)); not checked by a test. The question ends once the repository is public. |
| C12 | The rulings notes flag that USS-PY-007 lets a library declare an untested end-of-life minimum; the metadata notes propose that the declared minimum equal the tested minimum. | USS-DST-018 and USS-PY-018 (section 7) close the flag, so PY-007 stays as it is. |

## 1. The trial on an existing repository: fix the installer first

**Recommendation.** Run the trial as the README describes, after the 4.1 fix,
with the edits below and a short log. **Reason.** The trial is the only source
of the release tweaks, and every failed step becomes an issue. **Effort** S plus
the trial. **Depends on** 4.1 (recommended, C2). **Alternatives:** run it
without the fix, which the trial alone does not need.

| Step | Recommendation |
|---|---|
| Fix the install defect first | Do it (C2). |
| `cargo install --locked --path tools/uss` | Keep. On Windows, close the sessions that run `uss.exe` before a `--force` reinstall, because a running executable cannot be replaced (not checked). |
| `/plugin marketplace add <path>` | Run `claude plugin validate .` first (C4). Use the shell form `claude plugin marketplace add <path>`, or a quoted absolute path; both register a directory ([Create a marketplace](https://code.claude.com/docs/en/plugin-marketplaces)). Windows paths were not tested (not checked). |
| `/plugin install uss@universal-software-standard` | Keep the ID: it is `<entry name>@<marketplace name>`. In a session the command opens the plugin's details, where you install it; `claude plugin install` from the shell installs directly. |
| Make a branch and run `/uss:audit` | Keep. |
| Reinstall with `--force`, then `/reload-plugins` | Keep: edits to a local-path plugin "take effect at the next session start or when you run `/reload-plugins`", with no version change. |

Log these, in addition to the five points of the follow-ups: the number of
question rounds and how many the developer answered with the default "comply";
wall time, and tokens or cost for a full and a resumed audit; the size of the
`uss applicable --full` output; the number of `unverified` results and why
(missing tool, host setting, no person); every AGENTS.md command that failed or
needed a correction, which counts toward USS-CTB-007; the compile time of `uss`,
which is the case for shipping binaries; any applicability or score that the
agent worked out itself instead of calling `uss`; false fails and ambiguous
Verify texts; and whether `uss check` and `uss report` passed at the first run.

## 2. Decisions for you

### 2.1 Values in the documents: keep three, change two, fill one

| Value (file) | Recommendation | Reason | Effort | Depends on | Alternatives |
|---|---|---|---|---|---|
| Acknowledge in 7 days, assess in 14, fix or mitigate in 90 (`SECURITY.md`) | Keep 7 and 14. Change 90 to 60 days for medium or higher severity, with a 30-day target for critical. | OpenSSF Passing wants a first response in 14 days and forbids unpatched medium or higher vulnerabilities publicly known for more than 60 days ([criteria](https://www.bestpractices.dev/en/criteria/0)); Project Zero moves from 90 to 60 days on 2027-02-02 ([policy](https://projectzero.google/vulnerability-disclosure-policy.html)). 90 days is a researcher's disclosure clock, not a maintainer's target. | S | None | Keep 90, which fails the OpenSSF 60-day criterion in practice. |
| GitHub private vulnerability reporting (`SECURITY.md`) | Keep it; turn it on right after publication, then remove the "button is missing" fallback. | GitHub documents it for public repositories only; use on a private one is not documented (not checked) ([Privately reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability)). | S | 2.2 | A security role address only. |
| First response to a contribution in 7 days (`CONTRIBUTING.md`) | Keep 7; use 14 if one maintainer cannot keep it. | No norm sets a number: CHAOSS gives no benchmark ([Time to First Response](https://chaoss.community/kb/metric-time-to-first-response/)), and OpenSSF asks only that a majority of reports get a response. | S | None | 14 days. |
| Succession: the organisation appoints a new maintainer or archives (`GOVERNANCE.md`) | Change it: name the second organisation owner, or how to find them, and where the release credentials live. | With one owner who is also the maintainer, "the organisation appoints" depends on the same person. OpenSSF Silver `access_continuity` asks that the project continue if any one person is lost ([criteria.md](https://raw.githubusercontent.com/coreinfrastructure/best-practices-badge/main/docs/criteria.md)). | S | CTB-016 | Keep the text and accept the gap. |
| The code owner's login (`.github/CODEOWNERS`) | Keep the maintainer's login; add the second admin's when appointed. | On the Free plan, code owners work only in public repositories ([pricing](https://github.com/pricing)), so the file has no effect before 2.2. | S | 2.2, CTB-016 | None. |
| Code of conduct contact (`CODE_OF_CONDUCT.md`, USS-CTB-008) | A role address at a domain the organisation controls that forwards to the maintainer and the second owner; say who handles a report about the maintainer. | Contributor Covenant 3.0 leaves the reporting channel as a placeholder ([CC 3.0](https://www.contributor-covenant.org/version/3/0/code_of_conduct/)). A personal address meets the Verify, but is a conflict of interest when a report concerns the maintainer. | S | CTB-016 | A web form (needs hosting); the personal address. |

### 2.2 Repository visibility: only public clears every item for free

**Recommendation.** Make the repository public, after the history steps of
"Decide first". **Reason.** On public repositories every plan gets rulesets,
secret scanning with push protection, code scanning with CodeQL, and Dependabot
([GitHub security
features](https://docs.github.com/en/code-security/getting-started/github-security-features));
standard runners, including ARM64, are free and unlimited
([runners](https://docs.github.com/en/actions/reference/runners/github-hosted-runners));
and attestations use the public Sigstore instance ([artifact
attestations](https://docs.github.com/en/actions/concepts/security/artifact-attestations)).
USS is an MIT standard that wants outside contributors, and the README and
`SECURITY.md` already assume a public repository. **Effort** M (the
preparation). **Depends on** "Decide first", item 2. **Alternatives:** GitHub
Team at $4 per user per month fixes only SUP-007, SUP-008 and CON-003, while
secret scanning ($19) and code scanning ($30) cost extra per active committer
and private vulnerability reporting stays undocumented for private repositories
([pricing](https://github.com/pricing), [Advanced
Security](https://github.com/security/advanced-security)); Enterprise Cloud at
$21 per user per month is the only plan with attestations on private
repositories; staying private on Free keeps SUP-007, SUP-008, CON-003 and
CTB-012 failing with HTTP 403.

Each blocked item clears as follows. Branch protection and required checks
(USS-SUP-007, SUP-008, CON-003): rulesets, section 4.5. Private vulnerability
reporting: 2.1. Forks and fork PR runs (USS-CTB-012): 4.5. Secret and code
scanning (USS-SEC-004, SEC-005, SEC-028): 4.4. The public marketplace and the
headless CI template: 4.6 and 4.8.

After publication, in this order: turn on private vulnerability reporting,
secret scanning with push protection, Dependabot alerts and updates, and CodeQL
default setup; create the rulesets (4.5); set the fork PR approval; then audit
again. Two side effects follow. Public repositories disable scheduled workflows
after 60 days without activity
([events](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows)),
which the SAF-004 change in 2.5 catches. And hosted Linux and Windows runners
have 4 CPUs on public repositories against 2 on private ones
([runners](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)),
which changes the USS-CTB-005 measurement.

### 2.3 Audit results that need your decision

| Rule | Recommendation | Reason | Effort | Depends on | Alternatives |
|---|---|---|---|---|---|
| USS-CTB-008 | The role address of 2.1. | See 2.1. | S | CTB-016 | See 2.1. |
| USS-CTB-009 | Require the DCO: a `dco` job in a `pull_request`-only workflow (read-only token, `fetch-depth: 0`) that fails when a commit in `base..head` lacks a `Signed-off-by` line that matches its author; the DCO link and `git commit -s` in `CONTRIBUTING.md`; the web sign-off setting; `git config format.signoff true` for the maintainer. Make it a required check after 4.5. | The Verify asks for the line, a CI job or app that checks it, and no CLA; a CI job works on any plan. The DCO 1.1 text itself was not read (not checked). | S | 4.5 for "required" | The DCO GitHub App, which checks the same ([DCO app](https://github.com/apps/dco)); private-repository support not checked. A CLA is heavier and unnecessary for MIT. |
| USS-CTB-016 | Invite one trusted person as organisation owner and repository admin, and name them in `GOVERNANCE.md` (C10). | The rule counts a backup who takes no part in reviews; seats cost nothing on Free. This also fixes the succession gap of 2.1. | S | None | A Standard Deviation (SHOULD, so only the score drops); a second account of the same person does not count. |
| USS-DEP-006 | Replace nohash-hasher: set `serde-saphyr = { version = "2.2", default-features = false, features = ["deserialize"] }` in `tools/uss/Cargo.toml`. Keep cfg-if and change the Verify (section 3). Record the change in `docs/dependencies.md` and ADR 0002. | `uss` only deserialises, and serde-saphyr pulls nohash-hasher only through its `serialize` feature ([Cargo.toml](https://github.com/bourumir-wyngs/serde-saphyr/blob/master/Cargo.toml)). In a scratch copy the change removed nohash-hasher and base64 from the lockfile, and 159 tests passed. nohash-hasher was archived in 2020 with no promise of fixes ([repository](https://github.com/paritytech/nohash-hasher)). The audit names two cfg-if paths but five exist (also getrandom 0.3.4 and 0.4.3, parking_lot_core); correct that at the next audit. | S | None | A Standard Deviation: DEP-006 is a MUST NOT, so it would make USS "not compliant". |
| USS-BAK-013 | A `mirror.yml` workflow on push to `main`, on tag push and weekly, that pushes all branches and tags to a private GitLab.com or Codeberg repository with a deploy key in an environment secret; never on `pull_request`; actions pinned by SHA. Check with `git ls-remote` on both hosts. | A key scoped to the mirror cannot touch GitHub. GitLab push mirroring is on the Free tier ([GitLab](https://docs.gitlab.com/user/project/repository/mirror/push/)), but it pushes from GitLab; GitLab private-project limits and Codeberg's policy are not checked. | S–M | None | A cron job on the maintainer's machine (not independent of the maintainer); a mirroring webhook. |
| USS-DOC-007 | Keep the dictionary part `unverified` until the maintainer requests the free copy of ASD-STE100 Issue 9 (January 2025) through the form on asd-ste100.org; the maintainer files it, because the form collects a name and an email. Reword the Verify: "Without a copy of ASD-STE100 Issue 9, the auditor checks the writing rules that this rule names and records conformance to the approved-word dictionary as unverified. With a copy (free from the owner on request) the auditor also checks each word of the sample against the dictionary. A checker's report counts only as evidence from that tool, not as ASD certification." Never copy the specification into the repository or a report. | ASD reserves all rights, issues no individual permissions and endorses no checker ([STE downloads](https://www.asd-ste100.org/STE_downloads.html), [STE FAQ](https://www.asd-ste100.org/STE_faq.html)). The STEMG, not ASD-STAN, runs the form. | S | None | Make the writing rules USS's own and drop the dictionary. |
| USS-PRF-001 | Write `docs/performance.md`: "`uss check .uss/record.yaml` on this repository, release build, warm file cache: median of 20 runs at most 150 ms; first run after a build at most 1,000 ms." Add `cargo xtask bench` in the same change, and take the first ten CI numbers before fixing the limit. | Measured on one Windows 11 machine: warm runs 32–37 ms, first run 472 ms. 150 ms leaves four times the warm median and still catches a tenfold regression. Once a budget exists, USS-PRF-002 fails until CI measures it. Linux and macOS not measured (not checked). | S (M with the CI job) | None | Budget the binary size (10.6 MB now) as a second measure. |

### 2.4 CI cost: change nothing once public, or run the schedule on Linux only

**Recommendation.** If the repository goes public, keep the weekly run on three
operating systems. If it stays private, restrict the scheduled trigger to Linux
with `if: github.event_name != 'schedule' || matrix.os == 'ubuntu-latest'`,
because the advisory check does not depend on the OS. **Reason.** At $0.006,
$0.010 and $0.062 per minute for Linux, Windows and macOS ([Actions
billing](https://docs.github.com/en/billing/managing-billing-for-your-products/about-billing-for-github-actions))
and the measured job times, a cold three-OS run costs about $0.28, about $1.2 a
month, a tenth of the Free plan's $12 of included usage; push and PR runs on
macOS are the real budget risk. **Effort** S. **Depends on** 2.2.
**Alternatives:** keep three OS while private; a daily Linux run; drop the
schedule and rely on Dependabot alerts with cargo-deny on push.

The uncommitted lines of `todo.txt` also ask whether CI can run on the
maintainer's own hardware or on Jenkins. Going public answers the cost question.
Self-hosted runners are free, but the owner pays for the machines ([self-hosted
runners](https://docs.github.com/en/actions/concepts/runners/self-hosted-runners)),
and GitHub says they "should almost never be used for public repositories"
because "any user can open pull requests against the repository and compromise
the environment" ([secure
use](https://docs.github.com/en/actions/reference/security/secure-use)). Keep
the split that exists: `cargo xtask check` runs every check on the developer's
machine (USS-CTB-003), and hosted CI builds and tests on three operating
systems. Jenkins was not researched; moving CI off GitHub would also change the
evidence that SUP-008 and CON-003 read.

### 2.5 Rulings to confirm or reverse: confirm eight, reverse R8 and R10

Every change in this section is S.

| Ruling | Recommendation | Reason | Alternative |
|---|---|---|---|
| R1 | Confirm. | The link was broken for one task and is fixed. | None. |
| R2 | Confirm. | The order matches the source; nothing depends on another order. | None. |
| R3 | Confirm. | CI runs the step on every push; the missing xtask error-path tests are already USS-TST-002 (4.2). | None. |
| R4 | Confirm for 0.1.0. Later, add a `questions:` list to the results format, which `uss report` counts as "not compliant". | REL-011 always applies, and `unverified` blocks compliance until a person answers, which is the intent. A 0.x MINOR release may change the format. | A dedicated rule for open questions. |
| R5 | Confirm, and add: "Show the developer each rule that passed before the upgrade and fails now." | Acceptance by the developer is the only workable gate when the project was not compliant before. | None. |
| R6 | Confirm. | It enforces the Definition of Done for documents that name a command. | None. |
| R7 | Confirm. | USS-RUST-004 requires it, and Cargo reads profiles only from the workspace root ([Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)). | None. |
| R8 | Reverse (C1), with the step below. | See C1. | Keep decision 12. |
| R9 | Confirm, with one addition to AGENTS.md audit step 12: "With no person present, do not add the snippet. Report that it is missing." | A headless audit that commits the snippet changes a file outside `.uss/`, so the audited commit is no longer the release commit and `uss release-check` fails. | Never add the snippet, only offer it (weaker uptake). |
| R10 | Reverse: fix now (4.1, C2). | The command installs or updates `stable`, which changes the machine outside the task. | `--no-update` on stable: an old stable then fails `rust-version`. |

The replacement for AGENTS.md "Judge each rule", step 16 (change decision 12 and
spec §8 to match):

```markdown
16. Before the first release, some Verify texts have no release to
    examine. Show with a command that no release exists, such as `git tag`.
    Then judge the release process at the audited commit in place of the
    release. The release process is the CI workflow or the documented steps
    that build and publish a release.
    - If the process makes what the rule requires, such as binaries, an
      SBOM, signatures, provenance or a notices file, write `pass`.
    - If it does not, or the project has no release process, write `fail`.
    - Examine each document now, such as the rebuild steps, the
      instructions to verify signatures and the definition of the public
      API.
    - If the Verify compares with a published artifact, use a substitute.
      For example, build twice from the audited commit and compare the
      checksums. Name the substitute in the evidence.
```

The readings that the maintainer did not confirm:

| Reading | Recommendation | Reason | Alternative |
|---|---|---|---|
| Before the first release (decision 12) | Reverse, as above. | C1. | Keep it (C1). |
| An adopted MAY rule that fails lowers the score but not compliance | Confirm. Optionally, list "Adopted MAY rules not met" in the report. | Spec §5.5 and conventions §8 say so, and RFC 2119 calls MAY "truly optional" ([RFC 2119](https://www.rfc-editor.org/rfc/rfc2119)). Otherwise a project regains compliance by switching to `adopt: false` at the same score. | Make `adopt: true` bind compliance, which rewards declining. |
| USS-CTB-005, 25% margin | Keep 25%, and compare like with like: "The contributing guide states how long the one-command checks take, and on what (a named machine class, or the CI job). Compare with a run on the same kind of machine, warm or cold as the guide states." | Private runners have half the CPUs of public ones (2.2). USS states about 9 s warm and 140 s cold; the audit measured 6.2 s warm and 301 s for a cold fresh clone. | A 50% margin: looser, and still compares different machines. |
| USS-DOC-006, "more than once" | Confirm. | The smallest count that means "recurs"; no outside threshold was found. | Three reports. |
| USS-PRF-005, 1% over 60 seconds | Keep 1%, and measure for 5 minutes after start-up ends (it reports ready, or 30 s pass). | Apple's idle target is 0.0% ([Apple](https://developer.apple.com/library/archive/documentation/Performance/Conceptual/power_efficiency_guidelines_osx/MonitoringEnergyUsage.html)); Microsoft's procedure idles for 5 minutes ([Microsoft](https://learn.microsoft.com/en-us/windows/apps/performance/power)). Sixty seconds right after start-up catches start-up work and misses a timer that fires every few minutes. Each idle test takes 4 more minutes. | Keep 60 s: faster audits, more false results. |
| USS-SAF-004, weekly schedule | Keep weekly. Add "The latest scheduled run is less than a week old." to SAF-004 and SAF-007, and the same weekly bound and sentence to TST-011 and TST-012. | ClusterFuzzLite's examples run every 6 hours and daily ([ClusterFuzzLite](https://google.github.io/clusterfuzzlite/running-clusterfuzzlite/github-actions/)), so weekly is lenient; the new sentence catches a schedule that GitHub turned off ([disable workflows](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/disable-and-enable-workflows)). | Daily runs: closer to fuzzing practice, more CI cost. |
| USS-BAK-007, the production copy counts | Confirm, and cite the source in place of "USS's own reading": "1 primary and 2 backups". | The 2012 US-CERT paper that CISA hosts says so ([Data Backup Options](https://www.cisa.gov/sites/default/files/publications/data_backup_options.pdf)); CISA's current page does not say. | Count only backup copies (four in all), stricter than the source. |

The rulings from Plans 3 and 4 hold up. The notes confirmed them from the rule
texts and their recorded sources; the Plan 4 sources (pnpm, Oxlint, Squawk,
kube-linter, Go) were not read again (not checked).

| Ruling | Recommendation | Reason |
|---|---|---|
| QLT-006 met by the baseline; QLT-007 and QLT-008 judge a baseline rule that is off | Confirm. | No rule is judged twice. |
| TRU-004 is a MUST | Confirm. | The spec says data "MUST NOT be held hostage"; the positive form is easier to verify. |
| TRU-001 for paywall moves; REL-007 for removal from every edition | Confirm. | The spec records the one gap (an operated service with no releases). |
| Overlaps AI-001 with SEC-009, -010, -013, -014; HRD-003 with HRD-002 | Confirm. | Different scopes; an overlap costs one extra result. |
| OPS-002 leaves out alerts that BAK-010, AUD-008 and AI-007 require | Confirm. | Those rules must page on a cause by design. |
| AUD-004, AUD-006 removal only at expiry; DAT-005 keeps audit records | Confirm. | The three rules agree. |
| REL-001 and EXT-002 follow SemVer, with no 0.x rule | Confirm. | SemVer item 4 already covers 0.x, and USS is 0.x itself. |
| Unreadable sources not cited; registration-gated documents cited by the publisher's version | Confirm. | Matches the spec's "checked at writing time". |
| GO-006 pins the exact Go release; GO-010 without GONOSUMCHECK | Confirm. | The remaining settings are the ones that turn checksum checks off. |
| RUST-010 no longer refines SUP-005 | Confirm. | cargo-deny checks lockfile dependencies; SUP-005 judges downloads outside a lockfile. |
| TS-015 uses pnpm `allowBuilds`; TS-004 judges the effective tsconfig | Confirm. | No conflict found. |
| JAVA-001 also refines SAF-001 | Confirm. | It makes `-Xlint:all -Werror` binding through SAF-001. |
| Oxlint replaces eslint-plugin-jsx-a11y (REACT-004) | Confirm. | It is a SHOULD, so ESLint users record a deviation and still meet ACC-002. |
| Squawk default rules; kube-linter without organisation-specific checks | Confirm. | No conflict found. |
| K8S-007 stricter than HRD-008 | Confirm, and fix the body: "An unset value leaves the choice to the ServiceAccount, whose default mounts the token." State the reason for the stricter rule in the rationale. | Kubernetes lets the ServiceAccount set the value too, and the Pod's value wins only when both are set ([service accounts](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/)); the spec says a module never "silently raises" a core rule. |
| MVN-004 accepts bounded ranges | Confirm. | MVN-006 locks the resolution. |
| Bandit-class rule sets count as the SAST tool of SEC-028; ESLint does not | Confirm. | The ESLint baseline has no security rule set. A similar ruling for Clippy is an open choice (4.4). |
| End-of-life rules judge what a project builds, tests and runs with | Confirm. | C12 closes the gap for declared minimums. |
| REACT-005 exempts plain string literals | Confirm. | Static markup that the project wrote is not untrusted input. |
| QLT-012 judges inline suppressions only | Confirm. | Profiles handle stale configuration entries, such as cargo-deny `unused-ignored-advisory`. |
| SAF-001 binds the MUST rules of the modules that refine it | Confirm. | No conflict found. |

## 3. Questions about the standard: narrow three texts, add one sentence

**`stores-persistent-data`: narrow the definition.** **Reason.** As written, it
is true for any program that writes a file. For USS, `true` adds 10 rules (145
become 155 in a scratch run of `uss applicable`), 7 of them MUST or MUST NOT,
such as a CI job that restores a backup of reports. Other facts already exclude
cases inside their definition (`parses-untrusted-input`, `exposes-web-api`,
`uses-concurrency`). Compilers, linters, formatters, generators and report tools
become `false`; a CLI that saves settings or tokens stays `true`, so USS-CRA-009
still applies to it. Also change the comment in `templates/record.yaml` and this
repository's record; USS keeps `false`. **Effort** S. **Depends on** the 0.1.0
window. **Alternatives:** answer `true` for USS (10 rules it cannot meet
usefully); guidance in AGENTS.md only (the definition stays wrong for every
reader); split into two facts (more questions, and no rule needs the split).
Module rules were not searched for a hidden dependence on the broad reading (not
checked).

```markdown
### stores-persistent-data

- **Question:** Does it keep data that it needs again after a restart?
- **Definition:** Yes when it writes data that it, or a later version of it, reads back after it stops and starts again, as its own state or as its users' data, such as a database, a data directory, uploaded files, keys, or saved settings. Output that it only hands to people or to other programs, such as reports, logs, build outputs and exported files, does not count, and neither does a cache that it can rebuild.
```

**USS-PRF-002: confirm, and make the empty case explicit.** Append to the
Verify: "A project with no budgeted measure has nothing to judge here;
USS-PRF-001 judges the missing budget." Add to conventions section 8: "When a
Verify judges each item of a set (each budget, each signed artifact, each closed
report) and the set is empty, the rule is `pass`, unless the Verify says
otherwise. The rule that requires the set judges its absence." **Reason.**
Failing both rules counts one gap twice and needs two Standard Deviations for
one decision; USS-DOC-006 and USS-DST-015 already pass on an empty set, and
OpenSSF marks dependent criteria "N/A allowed"
([criteria](https://www.bestpractices.dev/en/criteria/0)). The cost is a kinder
score: Efficiency shows 66% here in place of 33%. **Effort** S.
**Alternatives:** fail PRF-002 too; gate it on a fact, which lets a project
leave the rule by writing no budget.

**USS-DEP-006: a notice that promises security fixes is not end-of-life
evidence.** Insert in the Verify, after the evidence sentence: "A deprecation or
archive notice that also says that the supplier still ships security fixes for
the release line is not such evidence." Append to the body: "A notice that says
that the supplier still ships security fixes for the release line does not make
it end-of-life." **Reason.** The body defines end-of-life as "no longer receives
security fixes", and cfg-if's README says it will not be updated "Barring
breakages and security fixes" ([cfg-if](https://github.com/rust-lang/cfg-if)),
so the Verify was stricter than the body. Without the change, almost every Rust
project with a lockfile fails a MUST NOT: getrandom declined to drop cfg-if to
keep fixes reaching older toolchains ([issue
847](https://github.com/rust-random/getrandom/issues/847)). RustSec also counts
a crate as maintained when its author says so
([HOWTO_UNMAINTAINED](https://github.com/rustsec/advisory-db/blob/main/HOWTO_UNMAINTAINED.md)).
Add no general "finished crate" exemption: the OpenSSF Concise Guide has none
and calls unmaintained software a risk ([Concise
Guide](https://best.openssf.org/Concise-Guide-for-Evaluating-Open-Source-Software)).
The project change is in 2.3. **Effort** S. **Alternatives:** a Standard
Deviation per crate (makes USS "not compliant"); count only scanner flags
(weaker, and tied to one ecosystem's database).

**USS-TST-003: leave out fixes that change only prose, and adopt a commit-type
rule.** Insert in the Verify, after its first sentence: "Leave out each change
that changes only text that people read, such as documentation, comments and
prose, and no code, configuration or data that a program acts on." Change the
body to: "Each bug fix in code, configuration or data that a program acts on
MUST come with a test that fails without the fix." In `CONTRIBUTING.md` and
AGENTS.md "Conventions": `fix` for code or for rule fields that `uss` reads,
with its test in the same commit; `fix(standard)` for prose-only rule fixes;
`docs` for documents outside the product; `ci` and `build` for CI and build
settings. **Reason.** Conventional Commits ties `fix` to "a bug fix for your
application" ([Conventional
Commits](https://www.conventionalcommits.org/en/v1.0.0/)), and for USS the rule
text is the product, so relabelling prose fixes as `docs` would misstate
PATCH-level changes. Of the 71 `fix` commits up to 51435cb, 22 change Rust code,
47 change only Markdown (4 of them change fields that `uss` reads) and 2 change
other files; the change removes 43 of the 49 failures, and the remaining 6 need
a test or a truthful type. **Effort** S. **Alternatives:** relabel only (wrong
for USS); exempt all non-code changes, configuration included (configuration
bugs can have tests); keep the rule (rewards mislabelling).

## 4. The release plan for 0.1.0

### 4.1 Defect to fix first (ruling R10): pin the toolchain by number

| Step | Recommendation |
|---|---|
| Install a pinned toolchain | `rustup toolchain install 1.100.0 --profile minimal --no-self-update`. A full version number is a valid toolchain name ([toolchains](https://rust-lang.github.io/rustup/concepts/toolchains.html)), the minimal profile gives `rustc`, `rust-std` and `cargo` ([profiles](https://rust-lang.github.io/rustup/concepts/profiles.html)), and the flag exists (C3). A named version leaves `stable` and the default toolchain as they were. |
| `cargo +1.100.0 install …` | Yes: `+toolchain` wins over `RUSTUP_TOOLCHAIN`, directory overrides and `rust-toolchain.toml` ([overrides](https://rust-lang.github.io/rustup/overrides.html)). |
| A sentence for a newer `rust-version` | "Use the `rust-version` of `Cargo.toml` at the release tag, and add `.0`: `1.100` installs as `1.100.0`." |
| A sentence for a machine without rustup | "Ask the developer for permission to install rustup from rustup.rs with `--default-toolchain none`, or to use an existing `cargo` at `rust-version` or later (`cargo --version`). Do not install a system Rust package without asking. If neither works, stop (step 10)." |
| A test that ties the number to `rust-version` | Extend `ci_template_is_hardened` in `tools/uss/src/cli/mod.rs` to read AGENTS.md, `rust-toolchain.toml`, the CI template and `docs/troubleshooting.md`, and assert that each toolchain number equals `rust-version` plus `.0`. |

Also change `docs/troubleshooting.md` (lines 79–86), which repeats the `+stable`
advice. **Effort** S. **Alternatives:** `cargo binstall uss` or a direct
download once release binaries exist (4.6), which removes the toolchain step for
agents.

### 4.2 Code changes in `tools/uss`: build one output layer first

Seven items change the same 12 `let _ = writeln!` sites and 16 CLI `format!`
calls. Build one `Ui` type in `tools/uss/src/cli/` that owns stdout, stderr and
the verbosity, and make library diagnostics structured (`Diagnostic { key, args
}` with an English `Display`). UX-011, SEC-014, UX-001, UX-009, OBS-002,
I18N-002 and QLT-005 then each change one place. The library builds most user
text (211 `format!` calls against 16 in `cli/`), which makes I18N-002 the large
item. The stream, output and error guidance comes from clig.dev
([clig.dev](https://clig.dev/)).

| Rule | Recommendation | Reason | Effort | Depends on | Alternative |
|---|---|---|---|---|---|
| USS-UX-002 (defect) | Refuse an existing target unless `--force` (`-f`); open with `create_new(true)`; refuse even with `--force` when the target is the results file or the record (compare `canonicalize`). Do the same for `release-check --out`, and add `--force` where AGENTS.md and the CI template run `uss report` again for a date. | The Verify accepts a destructive action only behind an explicit option. | S–M | None | Overwrite only a file that starts with a `uss report` marker line (keeps the workflows; weaker against the Verify). |
| USS-SEC-014 (defect) | One `escape_control` at the `Ui` sink: each `char::is_control` character (C0, DEL, C1) and the bidi controls U+202A–U+202E and U+2066–U+2069 as `\u{XXXX}`; escape `\n` and `\t` inside a message too. | Escaping at the sink keeps the library free of I/O and leaves the report's own cell filter alone; `uss` writes no colour, so nothing is lost. How an auditor reads "through a library" for a std function is not checked. | S | `Ui` | Strip the characters (hides the attack). |
| USS-UX-011 | Print the problems that stop `report` and `release-check` on stderr; the statement and scores stay on stdout, and move to stderr when `--output -` sends the report to stdout. | Results on stdout, problems on stderr. | S | `Ui` | None. |
| USS-UX-001 | Give each error a next step, in the catalogue of I18N-002 (about 160 `check` strings and 16 CLI ones). | Each message then holds what went wrong and what to do. | M (S for the CLI messages and the 10 most common problems) | I18N-002 | A generic "Run `uss --help`", which the Verify rejects. |
| USS-UX-006 | Rename `--out` to `--output`/`-o`, with a hidden `out` alias; accept `-` for stdin (record, results) and for stdout; add a support link in the top-level `after_help`, read from `CARGO_PKG_REPOSITORY`. A record from stdin takes the current directory as the project root. | Only the four guidelines that the audit named were checked; a full clig.dev pass was not done (not checked). | M | `Ui` | None. |
| USS-UX-009 | `--json` on `show` (a list of paths), `check` and `lint` (`ok`, `problems`) and `report` (`statement`, `compliant`, `scores`, `report`), with `serde_json`, which is already a dependency. | Matches the existing `--json` of `applicable` and `export`. | M | `Ui` | `--format json` or `--format text` (inconsistent with the existing flag). |
| USS-OBS-002 | Global `-q`/`--quiet` and `-v`/`--verbose` (a count); verbose lines go to stderr. No environment variable and no logging crate. | Keeps "uss reads no environment variable" true; `-V` is the version, so `-v` is free. | S–M | `Ui` | `--log-level`; `USS_LOG`, which contradicts `docs/cli.md`. |
| USS-I18N-002 | Fluent: `fluent-bundle` 0.16.0 (Apache-2.0 OR MIT; about 20 pure-Rust crates; no new `syn` duplicate), one embedded `i18n/en/uss.ftl` (C6), `set_use_isolating(false)` for terminal output, a test that every key in the code exists in the file and the reverse, and the clap help text moved into the catalogue. | Fluent has the plurals and per-language variants that section 8 needs; gettext needs C tooling, and `i18n-embed` reads the environment. The wasm32 build is not checked. | L (about 220 strings) | `Ui`; section 8 | A TOML key-value file (no plurals). **Decide:** a Standard Deviation with this plan, if 0.1.0 cannot wait. |
| USS-QLT-005 | Route all error text through `Ui::problem()`, which handles a failed write once, with the reason in a comment; make `files.rs::collect` report `read_dir` errors. | Removes the 10 production and 2 test `let _` sites, so `clippy::let_underscore_must_use` can be denied with no `#[expect]`. | S | `Ui` | None. |
| USS-SEC-009 | `maxLength` on each free-text string and `maxItems` on each array of both schemas (names 200, paths 260, evidence 10,000, arrays 1,000); refuse files over 4 MiB through `take(MAX + 1)`. | The largest real file is 63,666 bytes and the longest evidence 2,434 characters, so the limits leave about four times the headroom. **Decide:** confirm 10,000 for evidence. | S–M | UX-006 for stdin | None. |
| USS-ARC-005 | A file such as `tools/uss/architecture.toml` lists each module's allowed dependencies, seeded from today's graph, and the APIs forbidden outside `cli/` and `main.rs`; `tests/architecture.rs` checks it with the `regex` crate already in use. | A file plus a CI check is what the Verify asks; no new dependency. | M | None | Split `uss-core` and `uss-cli` crates (L). |
| USS-TST-002 | Move the exit-code fallback of `main.rs` into a tested function; test xtask `capture` and `run` with a program that does not exist, and `channel_from("[toolchain]\n")`; move the `build.rs` helpers into a module that the tests also compile. | Each named error path gets a test. | M | None | None. |
| USS-TST-005 | `tests/e2e.rs` runs `env!("CARGO_BIN_EXE_uss")` through `std::process::Command`, with `tempfile`; one test per documented workflow and exit code. | The Verify rejects tests that call internal functions; no new dependency. Plugin steps that need `claude` cannot run in CI. | M | UX-002 | `assert_cmd`; `trycmd`, which runs the commands in the Markdown. |
| USS-TST-010 | `proptest` 1.12.0: an `Applies when` parse-print round trip, parsers that never panic, idempotent text normalisation, scores in range and independent of order, and `escape_control` output with no control character. | Strategies compose for records and expressions, and the Verify names proptest. | M | SEC-014 | `quickcheck` 1.1.0. |
| USS-TST-012 | `cargo-fuzz` 0.13.2 in a `fuzz/` directory that is its own workspace, with `record` and `results` targets; a weekly Ubuntu workflow with nightly Rust, 300 s per target, a corpus cache and crash artifacts. | cargo-fuzz needs a nightly compiler ([Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz/setup.html)); the separate workspace keeps the NCSA licence of `libfuzzer-sys` out of `deny.toml`. | M | None | Add a 30 s run on PRs that touch the parsers. |

### 4.3 Configuration and xtask: turn on the baselines after the 12 fixes

| Rule | Recommendation | Reason | Effort | Depends on | Alternative |
|---|---|---|---|---|---|
| USS-RUST-001, USS-QLT-006 | After QLT-005, deny the six lints in `[workspace.lints.clippy]`; replace `deny.toml` with the cargo-deny profile baseline, plus `multiple-versions = "deny"` and a `skip` for `syn@2.0.119` with its reason. | A local Clippy run with the six lints found only `let_underscore_must_use` (12 sites); the proposed `deny.toml` passed under cargo-deny 0.22.0 with exit 0 and no warning. Not run under 0.22.1 (not checked). | S | QLT-005 | None. |
| USS-RUST-010 | Set `CARGO_DENY_VERSION` to 0.22.1 (released 2026-11-24), and make xtask reinstall when `cargo-deny --version` differs, because today an existing binary skips the install. | The profile needs 0.22.1 or later. | S | None | The prebuilt release with its `.sha256` (no Intel macOS asset). |
| USS-QLT-003 | Put `.tools/bin` on the child `PATH` of `cargo install`; keep the `syn` duplicate behind the documented skip, because jsonschema 0.63.0 and strum_macros 0.28.0, both the latest, still use syn 2; try `rustup default 1.100.0` in CI for the override notice. | The first two are confirmed; the setting that silences the rustup notice is not checked, so test it on a branch. | S | None | `RUSTUP_TOOLCHAIN` set from the toolchain file. |
| USS-QLT-001 | A pinned-tool helper in xtask: per OS and architecture, a URL and SHA-256 in the source; `curl --proto =https`; a `sha2` check before `tar -xf`; a cache under `.tools/`. Then `shellcheck --shell=sh .githooks/pre-commit` with ShellCheck 0.11.0. | `curl` and `tar` ship on all three runner systems. ShellCheck publishes no checksum file; the digests in the notes come from the GitHub API and were not hashed locally (not checked). | M | None | A Rust hook through `cargo xtask pre-commit`; its one-line shell wrapper still needs a linter. |
| USS-CTB-001 | `cargo xtask setup`: set `core.hooksPath`, install the pinned tools, run `cargo build --locked`, and print what it did. The guides shrink to "install rustup, run `cargo xtask setup`". | rustup installs the pinned toolchain at the first `cargo` run from `rust-toolchain.toml` (not tested in a clean environment). A check should not change Git settings, so `check` stays as it is. | S | QLT-001 helper | A dev container. |

### 4.4 Scanners: gitleaks in xtask, CodeQL after publication

| Rule | Recommendation | Reason | Effort | Depends on | Alternatives |
|---|---|---|---|---|---|
| USS-SEC-004, USS-SEC-005 | Gitleaks 8.30.1 (MIT) through the xtask helper: `gitleaks git --redact` over the history and `gitleaks dir --redact` over the tree, in `cargo xtask check`. Add `fetch-depth: 0` to the CI checkout, or the history scan sees one commit. Allow findings only in `.gitleaks.toml`, each with a reason. The first run is the scan of "Decide first", item 2. | MIT, builds for all six OS and architecture pairs, separate history and tree modes ([gitleaks](https://github.com/gitleaks/gitleaks#readme)). Pin each asset hash in the source; the per-asset hashes were not fetched. Expect findings in rule text that quotes secret patterns. | M (S after the helper) | QLT-001 helper | Betterleaks 1.10.1 (same author, signed checksums, young); TruffleHog (AGPL-3.0, verifies credentials over the network). |
| USS-SEC-028 | After 2.2, CodeQL for Rust in a SHA-pinned `github/codeql-action` workflow. **Decide** how to square it with USS-CTB-003, because CodeQL then runs only in CI: a ruling or a Standard Deviation, or a standard ruling that Clippy with a security lint set counts as the SAST tool for Rust, as Ruff `S` and gosec do (lint list not validated). | Code scanning is free on public repositories; the CodeQL CLI licence excludes private code without Advanced Security ([licence](https://github.com/github/codeql-cli-binaries/blob/main/LICENSE.md)). CodeQL lists Rust, without saying whether the support is GA ([languages](https://codeql.github.com/docs/codeql-overview/supported-languages-and-frameworks/)). | M | 2.2 | Semgrep CE or OpenGrep with the 10 open Rust rules in Ubuntu CI (weak coverage; rule licence not read); SonarCloud (public only); paid Code Security. |

### 4.5 GitHub settings: two rulesets after publication

| Rule | Recommendation |
|---|---|
| USS-SUP-007 | Ruleset "main" on the default branch: `deletion` and `non_fast_forward`. Ruleset "release tags" on `refs/tags/v*`: `deletion`, `non_fast_forward` and `update`, so a release tag cannot move while creation stays allowed. |
| USS-SUP-008 | In "main": `required_status_checks`, strict, with `check (ubuntu-latest)`, `check (windows-latest)` and `check (macos-latest)`; and `pull_request` with 0 approvals, or 1 once the second admin exists. |
| USS-CON-003 | The three contexts are the jobs that run `cargo xtask check`; add `dco` when it exists. |
| USS-CTB-012 | Forks of a public repository are always possible. Keep the default approval for first-time contributors, which the Verify accepts ([Actions settings](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository)); read the setting back after publication (endpoint not checked). |

**Reason.** Rulesets are free for public repositories on every plan, and tag
rulesets control who can move or delete a tag
([rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets)).
With no bypass list, the rules bind admins too; a solo maintainer can allow a
"Repository admin" bypass in pull-request mode only. Create "main" after one CI
run has reported the three contexts, and keep the names in step with `ci.yml`,
because a renamed job makes every merge wait for a check that never comes. The
`gh api` JSON in the notes is from memory (not checked), so read the rulesets
back with `gh api -X GET repos/{owner}/{repo}/rulesets`. **Effort** S. **Depends
on** 2.2. **Alternative:** classic branch protection with `enforce_admins`,
which has no tag protection.

### 4.6 What the release itself creates: one workflow, then audit, then tag

| Item | Recommendation | Reason | Effort | Depends on | Alternatives |
|---|---|---|---|---|---|
| USS-DST-002, USS-DST-014 | dist v0.34.0 with six targets: x86-64 and ARM64 of `pc-windows-msvc` (ARM on `windows-11-arm`), `apple-darwin` (`macos-15-intel`, `macos-latest`) and `unknown-linux-musl`. List exactly these six in the README. First check the generated workflow against the SHA-pin rules; if it pins actions by tag (not checked), use a hand-written matrix. | One generated workflow gives archives, checksums, installers, the SBOM and attestations ([dist](https://axodotdev.github.io/cargo-dist/book/)). musl has no glibc floor, so "no runtime to install" holds (the musl build of the dependencies not tested). | M | 2.2 for free ARM runners | A hand-written matrix (more lines, easier to audit for REL-003); cross-compiling with `cross` or `cargo-zigbuild` while private (not checked). |
| USS-SUP-013 | cargo-cyclonedx 0.5.10 (`cargo-cyclonedx = true` in dist), one `uss-<version>.cdx.json` per release. | The Verify asks for SPDX or CycloneDX with the dependency graph. The tool lists spec versions up to 1.5 while the rule's reference names CycloneDX 1.7 ([README](https://raw.githubusercontent.com/CycloneDX/cyclonedx-rust-cargo/main/cargo-cyclonedx/README.md)), so align the reference; the Verify names no version. Graph content not inspected (not checked). | S | DST-002 workflow | syft as a cross-check; cargo-auditable as a supplement only. |
| USS-SUP-014, USS-SUP-015, USS-DST-015 | `actions/attest@v4` (4.2.2), pinned by SHA, with `id-token: write`, `attestations: write` and `contents: read`, over each archive, the SBOM, the notices and the checksum file. Add a README "Verify a download" table: kind, the command `gh attestation verify <file> -R <owner>/<repo>`, and the signer identity. | One attestation is both a signature and SLSA v1.0 Build Level 2 provenance ([use attestations](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations)); Level 2 needs authentic provenance from a hosted build platform ([SLSA v1.2](https://slsa.dev/spec/v1.2/build-requirements)). Private repositories need Enterprise Cloud (C11). | S–M | 2.2 | While private: cosign keyless for SUP-014, which puts the repository's identity in the public Rekor log ([Sigstore](https://docs.sigstore.dev/cosign/signing/overview/)), and SUP-015 `fail` or a deviation; minisign (a key in CI, no provenance); slsa-github-generator (last release v2.1.0, 2025-02-24). |
| USS-CTB-007 | Before the tag, audit it with a named substitute: the documented install from the audited commit in a fresh clone. After the tag, run the documented command on a clean machine and audit again. | The Verify runs each command "as written in a fresh clone", and the tag does not exist when the release audit runs (C1). | S | 2.2; the tag | Binaries remove the compile step. |
| USS-LIC-004 | cargo-about 0.9.3 generates `NOTICES.md` from `about.toml` and `about.hbs`. Commit it, embed it with `include_str!` so that `uss show NOTICES.md` prints it, and ship it in each archive with `LICENSE`. An xtask step compares the committed file with fresh output, and a unit test checks that it names each crate of the normal dependency tree. | A committed file needs no tool or network at build time ([cargo-about](https://raw.githubusercontent.com/EmbarkStudios/cargo-about/main/README.md)); the cargo-about book was not read, so the `about.toml` details are not checked. | M | QLT-001 helper | cargo-bundle-licenses (not read). |
| USS-REL-003 | Write `docs/release.md`: the tag, the runner image or container digest, the toolchain, `--locked`, `--remap-path-prefix`, `SOURCE_DATE_EPOCH` from the tag commit, `CARGO_INCREMENTAL=0`, the commands, and per artifact "reproducible" or "not reproducible, because …". Build the Linux musl archives twice before the release and compare the checksums. | The Verify wants identical rebuilds, which are realistic only for Linux musl in a pinned container; Windows embeds timestamps unless `/Brepro` is used, and macOS adds signatures and UUIDs. No source was read for this item (not checked). | M–L | DST-002 workflow | A Standard Deviation for the targets that do not reproduce. |
| A 0.1.0 entry in `CHANGELOG.md` | `## [0.1.0] - YYYY-MM-DD` with Keep a Changelog headings and the initial rule set by area, committed before the release audit. | The upgrade workflow reads the changelog for added, changed and retired IDs, and the entry is part of the audited commit. Keep a Changelog 1.1.0 uses "Fixed" for bug fixes ([Keep a Changelog](https://keepachangelog.com/en/1.1.0/)). | S | None | None. |
| The release audit and `uss release-check` | Audit the changelog commit under the C1 reading; commit the record, results and report; run `uss release-check --require-compliant`; delete `uss-report.md`. | `uss release-check` accepts later commits that touch only `.uss/`. | S | C1 | None. |
| The tag `v0.1.0` | Put it on the results commit, under the tag ruleset of 4.5. | AGENTS.md "Release audit", step 5. | S | 4.5 | None. |
| A public marketplace | Your own marketplace in the public repository: users run `claude plugin marketplace add <owner>/<repo>` and `claude plugin install uss@universal-software-standard`, with no submission form ([publish](https://code.claude.com/docs/en/plugins/publish.md)). Keep `version` in `plugin.json` equal to the USS version and bump it with each release, or users keep the old copy; let the skills compare `uss version` with their own (an inferred design). | Anthropic's directory needs a paid claude.ai plan and has review rules that were not read. | S | 2.2 | Omit `version`, so the commit SHA is the version. |

The order of the release, then: publish the repository; add the release workflow
with the SBOM, attestations, notices and rebuild steps; commit the changelog
entry; audit that commit; commit the results; run `uss release-check
--require-compliant`; tag the results commit; publish the release, so the CI
template attaches the report; install from the tag on a clean machine and audit
USS-CTB-007 again.

### 4.7 Results that clear at the next audit: enforce them in CI, not by habit

| Rule | Recommendation | Reason | Effort |
|---|---|---|---|
| USS-DEP-011, USS-DEP-012 | Add a "Health check" column to `docs/dependencies.md` (date; last release and activity in 12 months; deprecated or archived; maintainers; why the standard library is not enough), and a test `every_direct_dependency_is_documented` that fails when a dependency of any `tools/*/Cargo.toml` table has no row with a purpose and a health entry. | The repository has no pull requests or merges, so the PR template's checkbox never ran (C9). The OpenSSF Concise Guide checks activity in 12 months and more than one maintainer ([Concise Guide](https://best.openssf.org/Concise-Guide-for-Evaluating-Open-Source-Software)). | S |
| USS-TST-003 | The section 3 change, plus a `.gitmessage` set by `cargo xtask setup` and a `commit-msg` hook that warns about a `fix` that changes Rust code and no test. | A hook can be skipped, so it is a reminder; TST-003 needs judgement. | S |

**Alternative:** a CI script that checks each pushed commit range for the same
patterns.

### 4.8 Tool improvements that are not audit results

| Item | Recommendation | Reason | Effort | Depends on | Alternative |
|---|---|---|---|---|---|
| A filter for `uss applicable --full` | Repeatable `--area`, `--level`, `--id` and `--category`, AND across kinds and OR within one, valid with text and `--json`; match the area as the whole segment between the first two hyphens; exit 2 with the valid values on an unknown value. Change AGENTS.md "Before every workflow", steps 7 and 9, to `uss applicable --full --area SEC .uss/record.yaml`. | The 148,349-byte output then splits by area, and agents no longer save it to a file. A pure function in the library is easy to test. | S–M | None | None. |
| `uss check` and results files | Validate the latest audit's results file by default (schema, `commit`, rule IDs), and every one with `--all-audits`; remove the note in AGENTS.md "Finish the audit", step 4. | Agents run `uss check` before each commit; old results files cannot change after an upgrade, so the default is the latest. | S–M | None | Keep `uss report` as the only check of results files. |
| The CI template | Split it into `release-check` (`contents: read`, uploads the report as an artifact) and `attach` (`needs: release-check`, `contents: write`, only `gh release upload`). Keep `--tag` (C8). Add a header comment that tells adopters to put `github-actions` in their `dependabot.yml`. Later, download the release binary and verify its attestation in place of `cargo install`. | The write token stays away from the step that builds and runs code. Dependabot's handling of SHA pins with version comments is not checked ([ecosystems](https://docs.github.com/en/code-security/dependabot/ecosystems-supported-by-dependabot/supported-ecosystems-and-repositories)). | S | 4.5 tag ruleset | `--rev <sha>` (C8). |
| A headless audit template | `templates/ci/uss-audit.yml` on `workflow_dispatch` and `schedule`: install `uss` first; `anthropics/claude-code-action` pinned by SHA (the SHA was not resolved); `plugin_marketplaces`, `plugins: uss@universal-software-standard` and `prompt: /uss:audit`; narrow `--allowedTools`, `--max-turns`, a timeout and `concurrency`; `ANTHROPIC_API_KEY` for shared use; a second job opens a PR with the results. | A `prompt` input runs the action without a mention, and plugins install through those two inputs ([Claude Code GitHub Actions](https://code.claude.com/docs/en/github-actions)). It needs the public marketplace (no documented way to authenticate a private one, not checked) and the R9 exception. The cost of one run is not checked. | M | 2.2; R9 | None. |

### 4.9 References to check before the release

| Item | Recommendation | Reason | Effort | Alternatives |
|---|---|---|---|---|
| Vendor baselines that never ran | `.github/workflows/vendor-profiles.yml`, weekly, manual and on changes to `standard/vendors/**`: one matrix job per tool installs the lower bound and the newest version in range, writes the profile's single fenced block, runs it on a `clean/` fixture (exit 0, no configuration error) and a `dirty/` fixture (the expected codes), and opens an issue when the newest release leaves the range. Validate the Scorecard baseline, a workflow file, with actionlint. Before 0.1.0, do only the `clean/` runs. | All 17 lower bounds equal the current releases on 2026-11-25; golangci-lint failed only because the local copy was 2.12.2, below the profile's 2.15.0. The ranges start at the newest release, so they go stale fast (Clippy's every six weeks). A fixture proves that the configuration loads and the rules exist, not the coverage claims. | 2 days for `clean/`; 4–6 days in all (an estimate) | A weekly `clean/` run at the latest versions only (no check at the bound); a manual checklist before each release, which already missed golangci-lint. |
| EN 301 549 V4.1.1 | Cite the published V4.1.1 (2026-09) by the parts read: cover, foreword, clause A.2 and Annex ZB. Replace "no reference was found" with "As of 2026-11-25 no reference to EN 301 549 under Directive (EU) 2019/882 was found in the Official Journal; V4.1.1 says that its presumption of conformity starts only when the Official Journal cites it". Search the Official Journal again before the release. | V4.1.1 ties the presumption to the OJ citation ([ETSI PDF](https://www.etsi.org/deliver/etsi_en/301500_301599/301549/04.01.01_60/en_301549v040101p.pdf)); the Commission still names V3.2.1, for the Web Accessibility Directive only ([Commission](https://commission.europa.eu/strategy-and-policy/policies/justice-and-fundamental-rights/disability/accessibility-standardisation_en)); a vendor blog expects the citation on 2027-01-22 ([Deque](https://www.deque.com/blog/en-301-549-v4-1-1-is-final-what-changed-what-it-means-and-what-you-should-do/)), so 0.1.0 can straddle it. No OJ list was read. Use the `04.01.01_60` URL, not the `_20` draft. | S | Drop the version and say "the latest version"; wait for the OJ citation and reword then. |
| USS-CRA-004 | Keep the rule and the Verify. Reword the rationale: Article 13(8) sets at least five years "but it names no start date for the five years. USS counts them from the placing on the market of the release under audit; the Regulation does not say this, and a qualified person confirms it." Add Annex II point 7 and Recital 60 to the references. **Decide** whether the clock starts with each release (as now) or with the first placing of the product. | No start date is in the Regulation ([Regulation (EU) 2024/2847](http://data.europa.eu/eli/reg/2024/2847/oj)); counting from each release is stricter than recital 40, which lets support cover only the latest version. | S | Drop the start-date test, and check only that an end date is stated with its basis. |
| CIS Controls 11.3 | Cite Safeguard 11.3 "Protect Recovery Data" from the CIS Controls Assessment Specification, the publisher's free page, and check 11.2 and 11.5 against the same page. | It replaces the third-party copy; third-party summaries put text under the wrong safeguard ([CIS Controls Assessment Specification](https://cas.docs.cisecurity.org/en/latest/source/Controls11/)). The page does not state its Controls version (not checked). | S | Cite the v8.1 PDF, which needs a registration form. |
| USS-AUD-009 and CEF | Cite "Implementing ArcSight Common Event Format (CEF) – Version 27" (OpenText, January 2024), and add to the Verify: "for CEF, a parser that follows the header and extension rules of the Implementation Standard". | The 25.1 documentation set still carries Version 27, and later paths return 404 ([CEF PDF](https://www.microfocus.com/documentation/arcsight/arcsight-smartconnectors-24.1/pdfdoc/cef-implementation-standard/cef-implementation-standard.pdf)). The URL may move. | S | Cite "CEF 1.2", the format version inside the guide; drop CEF as a vendor format. |
| Reading EU law | Keep the ELI URI with `Accept: application/xhtml+xml`. | The URI redirected to EUR-Lex and still returned the full text (HTTP 200); the Cellar host itself was not reached. | None | Reach Cellar directly, as the follow-ups say. |

## 5. Housekeeping

| Item | Recommendation | Reason | Effort |
|---|---|---|---|
| Four locked worktrees | Remove them after the Claude Code session that locked them ends (a `claude.exe` with the lock's PID was running on 2026-11-25), and before any history rewrite. For each: `git worktree unlock`, `git worktree remove` without `--force`, `git branch -d`; `uss-plan5-workflows` last, and not from inside it; then `git worktree prune --dry-run -v`. | All four are clean and merged into `main`, and removal frees about 8.9 GB of build output. Never `remove -f -f`, which skips the lock and the cleanliness check together ([git-worktree](https://git-scm.com/docs/git-worktree)). | S |
| `todo.txt` | The uncommitted lines hold three topics: metadata (section 7), translations and locale (section 8), and self-hosted runners or Jenkins (2.4). The maintainer edits or commits the file, because AGENTS.md forbids agents to change it. Include it, and the untracked `docs/superpowers/follow-ups.md` and `docs/superpowers/research/`, in the review before publication. | The answers now exist; the files hold notes that become public. | S |

## 6. Later work: settle the record format before 1.0.0

| Item | Recommendation | Basis |
|---|---|---|
| "translations/locale" | It means three things: what an audit checks in a project, the audit in the developer's language, and translations of USS. See section 8. | Locale notes. |
| "metadata" | The metadata that a registry, marketplace or host tool requires of what a project publishes (MUST), and the metadata that improves compatibility and discovery (SHOULD). See section 7. | Metadata notes. |
| An MCP server for rules and records | Defer until the trial shows that CLI output is the bottleneck; the area filter (4.8) and `--json` (4.2) give agents most of what a server would serve. | Not researched; an inference. |
| More modules | Rank them by the "Stack without a module" sections of trial reports. For host metadata, GitHub Actions and VS Code extensions come first (section 7). | Partly the metadata notes. |
| Dashboards across projects | Build them on `uss report --json` (UX-009), not on Markdown reports. | An inference. |
| PCI DSS, HIPAA, US federal modules | Defer, and apply the lessons of 4.9: read the legal text at its source, date each "not found", and label USS's readings. | Not researched. |
| A VS Code extension, then other IDEs | Defer. The library already builds for wasm32, but `fluent-bundle` on wasm32 is not checked, and the extension would itself fall under DST-017 to DST-019. | Partly the tooling notes. |
| Cloud cost management; export controls on cryptography | Defer. | Not researched. |
| A compliance badge | Generate it from the statement of the committed report, with the USS version and date, and link the report, so a badge never claims more than the statement. | Not researched; an inference. |
| 1.0.0 | Declare it after the record format settles: land `project.language` (section 8) and the results `questions:` list (R4) first, because until 1.0.0 a MINOR release may change the record format, and after it a MINOR release may not. | Conventions §10, via the locale and rulings notes. |

## 7. Metadata rules: one MUST defers to each host's validator

USS already judges the licence field (USS-LIC-001), the root README
(USS-DOC-001), version numbers and tags (USS-REL-001, REL-002) and the manifests
of plugins for a project's own plugin host (USS-EXT-003, EXT-015). Nothing
requires a published package, plugin, extension, action, chart or image to carry
what its host requires, and nothing requires a declared minimum version of the
runtime, toolchain or host. The fact `distributed-as-package` exists, but no
rule uses it.

### Each host enforces a small required set and recommends a larger one

| Host (USS module) | Required: the host refuses or fails without it | Recommended: shown, searched or used for compatibility | Validator |
|---|---|---|---|
| crates.io (Rust) | `name`, `version`, `description`, `license` or `license-file` ([manifest](https://doc.rust-lang.org/cargo/reference/manifest.html)) | `repository`, `readme`, `keywords` and `categories` (at most 5 each), `documentation`, `rust-version` | `cargo publish --dry-run`; Clippy `cargo_common_metadata` |
| npm (TypeScript) | `name`, `version` ([package.json](https://docs.npmjs.com/cli/v11/configuring-npm/package-json)) | `description`, `keywords`, `repository`, `bugs`, `license`, `engines`, `exports` with `types` ([publint](https://publint.dev/rules)) | dry-run publish (not read); publint |
| PyPI (Python) | `Metadata-Version`, `Name`, `Version`; no `Private ::` classifier ([core metadata](https://packaging.python.org/en/latest/specifications/core-metadata/)) | `description`, `readme`, `requires-python`, `license`, `keywords`, `classifiers`, `urls` | `twine check --strict` |
| Go modules (Go) | one `module` directive; a detectable licence for full pkg.go.dev display ([Go modules](https://go.dev/ref/mod), [licence policy](https://pkg.go.dev/license-policy)) | the `go` directive, tagged and stable versions, a package comment | the `go` command |
| Maven Central (Maven, Java) | coordinates without `-SNAPSHOT`, `name`, `description`, `url`, `licenses`, `developers`, `scm`; sources and javadoc JARs; checksums; signatures ([requirements](https://central.sonatype.org/publish/requirements/)) | none stated | Central Portal validation (not read) |
| Helm chart (Kubernetes) | `apiVersion`, `name`, `version` ([charts](https://helm.sh/docs/topics/charts/)) | `kubeVersion`, `description`, `home`, `sources`, `maintainers`, `keywords`, `icon` | `helm lint` |
| OCI image (none) | none ([annotations](https://github.com/opencontainers/image-spec/blob/main/annotations.md)) | `org.opencontainers.image.*`; GitHub's registry reads `source`, `description` and `licenses` ([GHCR](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)) | none |
| Claude Code plugin and marketplace (none) | plugin `name`; marketplace `name`, `owner.name`, `plugins[].name`, `plugins[].source` ([plugin reference](https://code.claude.com/docs/en/plugins-reference), [marketplace reference](https://code.claude.com/docs/en/plugins/marketplace-reference.md)) | `version`, `description`, `author`, `license`, `keywords`, `engines` | `claude plugin validate`, `--strict` |
| VS Code Marketplace (none) | `name`, `version`, `publisher`, `engines.vscode` ([manifest](https://code.visualstudio.com/api/references/extension-manifest)) | `displayName`, `description`, `categories`, `keywords`, `icon`, `repository`, `bugs` | `vsce package` |
| JetBrains Marketplace (none) | `name`, `version`, `idea-version`, `vendor`, `description` ([plugin.xml](https://plugins.jetbrains.com/docs/intellij/plugin-configuration-file.html)) | `id`, `change-notes` | Plugin Verifier |
| GitHub Actions (none) | `name`, `description`, `runs`; for the Marketplace, one `action.yml` at the root and a unique name ([metadata syntax](https://docs.github.com/en/actions/reference/workflows-and-actions/metadata-syntax)) | `author`, `inputs`, `outputs`, `branding` | none found |
| Homebrew (none) | `homepage`; `license` for new formulae ([cookbook](https://docs.brew.sh/Formula-Cookbook)) | `desc` | `brew audit --strict` |
| winget (none) | `PackageIdentifier`, `PackageVersion`, `PackageLocale`, `Publisher`, `PackageName`, `License`, `ShortDescription`, `Installers` and the manifest fields ([manifest](https://learn.microsoft.com/en-us/windows/package-manager/package/manifest)) | `PublisherUrl`, `PackageUrl`, `Tags`, `MinimumOSVersion` | `winget validate` (not read) |

Two kinds of parent exist. A registry or marketplace refuses the upload; a host
tool that loads an item straight from a repository (Claude Code from a Git
marketplace, GitHub Actions through `uses:`, Go by module path) refuses to load
it, so the user meets the failure, which is why the MUST matters. Validators
already split the two levels: their errors map to the required metadata (MUST),
and their warnings or strict mode map to the recommended metadata (SHOULD). The
declared minimum version matters most among the recommended fields, because
tools act on it: Cargo reports an unsupported toolchain as an error ([Rust
version](https://doc.rust-lang.org/cargo/reference/rust-version.html)), Go
toolchains refuse modules that declare a newer Go, and Helm fails the install on
an unsupported cluster.

The Verify pattern follows from that: list the hosts; take each host's
documentation of required metadata at the audit date, as USS-PY-007 already does
for Python's release table; run the host's validator on the audited commit and
give the command, its version and its output as evidence; check the fields by
hand where no validator exists. Do not copy volatile limits, such as 5 keywords
on crates.io, into rules; the validator enforces them. A host that adds a
requirement after a release does not fail that release. Host field lists belong
in the modules where USS has one; the core Verify covers the other hosts until a
trial shows that auditors stumble.

### Six rules at the end of the Distribution area, behind facts that exist

| ID | Title | Level | Applies when (with DST's `publishes-releases`) | Category |
|---|---|---|---|---|
| USS-DST-017 | Carry the metadata that each host requires | MUST | `distributed-as-package` | Compatibility |
| USS-DST-018 | Declare the tested runtime and host versions | SHOULD | `distributed-as-package` | Compatibility |
| USS-DST-019 | Fill the listing metadata | SHOULD | `distributed-as-package` | Accessibility & Usability |
| USS-DST-020 | Annotate published images | SHOULD | `ships-container-image` | Compatibility |
| USS-DST-021 | Describe the repository on its host | SHOULD | `always` | Accessibility & Usability |
| USS-DST-022 | Provide citation metadata | MAY | `always` | Features |

The Distribution area keeps the rules behind `publishes-releases`, so the
all-false budget of the spec does not change, and the licence field stays with
LIC-001, the root README with DOC-001 and the project's own plugin host with
EXT. No new fact is needed: since no rule uses `distributed-as-package`, its
definition can be made precise before 0.1.0 without a ripple. Records keep their
answer unless they ship only a plugin, extension or action, which changes from
`false` to `true`. Repository-level metadata gets one SHOULD (description and
topics, read from the host) and one MAY (`CITATION.cff`, which GitHub turns into
a "Cite this repository" entry ([CITATION
files](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-citation-files))).
`FUNDING.yml` (a business choice), `publiccode.yml` (needs a public-sector fact
first) and `codemeta.json` (no host found that reads it) get no rule.

```markdown
### distributed-as-package

- **Question:** Is it released as a package, a binary, a chart, or a plugin or extension of another tool?
- **Definition:** Yes when it is released as something that people install or that another tool fetches or loads: a package in a registry (such as crates.io, npm, PyPI or Maven Central), a library module that others fetch, a binary, a Helm chart, or a plugin, extension or action for another tool (such as a Claude Code plugin, a VS Code extension or a GitHub Action). A container image alone does not count: `ships-container-image` covers it.
- **Implies:** publishes-releases
```

The core drafts, from the metadata notes:

```markdown
## Metadata

### USS-DST-017: Carry the metadata that each host requires

- **Level:** MUST
- **Applies when:** distributed-as-package
- **Category:** Compatibility
- **Verify:** List the hosts of the release. A host is a registry,
  marketplace or catalogue that the project publishes to, such as crates.io,
  npm or a plugin marketplace. A host tool that loads a published plugin,
  extension or action is a host too. For each host, take its documentation of
  required metadata at the audit date. Where the host has a validator, such
  as a dry-run publish or a validate command, run it on the audited commit.
  It reports no error. Where the host has no validator, each required field
  is present and has a valid value. A missing field, an invalid value or a
  validator error fails. A listing that someone else maintains is not judged.
  USS-LIC-001 judges the licence field, not this rule.
- **References:** The Cargo Book, The Manifest Format (read 2026-11-25); npm
  Docs, package.json (npm 11.21.3, read 2026-11-25); PyPA, Core metadata
  specifications 2.6; Sonatype Central, Requirements (read 2026-11-25);
  Claude Code Docs, Plugin manifest reference (read 2026-11-25), as examples
  of hosts

Each package, binary, chart, plugin, extension or action that the project
publishes MUST carry the metadata that its registry, marketplace or host tool
requires.

**Rationale:** A host refuses or misreads an item without its required
metadata. A plugin that a host tool loads from a repository has no registry to
refuse it, so the user meets the failure instead.

### USS-DST-018: Declare the tested runtime and host versions

- **Level:** SHOULD
- **Applies when:** distributed-as-package
- **Category:** Compatibility
- **Verify:** List each manifest that the release publishes whose format has
  a field for the lowest supported version of a runtime, a toolchain or a host
  tool. Examples are `rust-version` in `Cargo.toml`, `engines.node` in
  `package.json` and `requires-python` in `pyproject.toml`. Each such field is
  set. The lowest version that the field admits is the lowest version that CI
  builds and tests the package on. A field that is not set fails. A lowest
  version that CI does not test, or that is above the lowest tested version,
  fails. USS-DST-017 judges a field that the host requires.
- **References:** The Cargo Book, Rust Version (read 2026-11-25); npm Docs,
  package.json (npm 11.21.3, read 2026-11-25); PyPA, pyproject.toml
  specification (read 2026-11-25); Go Modules Reference (read 2026-11-25)

Each published package SHOULD declare, in the fields that its format defines,
the lowest runtime, toolchain and host versions that CI tests it on.

**Rationale:** Build tools and installers act on these fields. Cargo stops with
a clear error on an older toolchain, and Helm refuses an unsupported cluster.
A declared version that CI does not test is only a claim.

### USS-DST-019: Fill the listing metadata

- **Level:** SHOULD
- **Applies when:** distributed-as-package
- **Category:** Accessibility & Usability
- **Verify:** Take the hosts of USS-DST-017. For each package and host, take
  the fields that the host shows in its listing or uses for search. These are
  a short description, the source repository, a place to report issues,
  keywords or categories, and the README. Each counts only where the host has
  a field for it. Each such field is set and is not a placeholder. Where the
  host's validator has a strict mode, run it. It reports no warning about a
  missing field. Some warnings concern a field that only another host reads,
  such as a directory icon. Such a warning is not judged when the project
  does not publish there. A missing field fails.
- **References:** Clippy, `cargo_common_metadata` (read 2026-11-25); publint
  rules (read 2026-11-25); twine 7.0.0, `twine check`; Claude Code Docs,
  Plugin manifest reference (read 2026-11-25), as examples of validators

Each published package SHOULD fill the fields that its host shows in its
listing or uses for search. These are a description, the source repository, a
place to report issues, keywords or categories, and the README.

**Rationale:** Users find a package through the host's search and judge it from
its listing. A listing without a description, a source link or a place to
report issues leaves them to guess.

### USS-DST-020: Annotate published images

- **Level:** SHOULD
- **Applies when:** ships-container-image
- **Category:** Compatibility
- **Verify:** Take each image of the latest release from its registry. Read
  the annotations of its manifest, or of its image index for a
  multi-architecture image, and the labels of its configuration. Together
  they hold these keys of the OCI Image Format Specification:
  `org.opencontainers.image.source`, `org.opencontainers.image.revision`,
  `org.opencontainers.image.version`, `org.opencontainers.image.licenses` and
  `org.opencontainers.image.description`. The revision is the commit of the
  release. The licences are an SPDX licence expression. A missing key or an
  empty value fails.
- **References:** OCI Image Format Specification v1.1.1, Annotations; GitHub
  Docs, Working with the Container registry (read 2026-11-25)

Each image that the project publishes SHOULD carry the OCI annotations for its
source, revision, version, licences and description.

**Rationale:** Registries, scanners and users read these keys to link an image
to its source, its release and its licence. GitHub's registry links a package
to its repository through `org.opencontainers.image.source`.

### USS-DST-021: Describe the repository on its host

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Accessibility & Usability
- **Verify:** Read the settings of the repository on its host, such as the
  About section on GitHub. The description says in one sentence what the
  software does. Where the host has topics, the repository has at least one
  topic that names its subject. An empty description fails. No topic fails
  where the host has topics.
- **References:** GitHub Docs, Classifying your repository with topics (read
  2026-11-25)

The repository SHOULD have a description and topics on its host.

**Rationale:** The host's search and topic pages are where many users and
contributors first find a project. A repository without a description or
topics is hard to find and to tell apart from others.

### USS-DST-022: Provide citation metadata

- **Level:** MAY
- **Applies when:** always
- **Category:** Features
- **Verify:** Adoption means that the repository root has a `CITATION.cff`
  file. It is adopted when the file is valid against the schema of the
  Citation File Format 1.2.0, and it names the title and the authors of the
  software. A file that is missing or not valid fails once the MAY is
  adopted.
- **References:** Citation File Format 1.2.0; GitHub Docs, About CITATION
  files (read 2026-11-25)

The repository MAY have a `CITATION.cff` file that says how to cite the
software.

**Rationale:** GitHub reads the file and offers the citation in APA and BibTeX
on the repository page, so people who cite the software cite it correctly.
```

The module refinements put the host's field list where USS has a module. Each
one has the level and category of the core rule that it refines, and the full
texts are in the metadata notes (`docs/superpowers/research/research_notes/USS
follow up recommendations/metadata_rules.md`, Q6).

| ID | Title | Level | Applies when | Refines | What the Verify checks |
|---|---|---|---|---|---|
| USS-RUST-012 | Set the metadata that crates.io requires | MUST | `distributed-as-package` | DST-017 | `version` and a plain-text `description`; `cargo publish --dry-run` exits 0 |
| USS-RUST-013 | Declare and test rust-version | SHOULD | `distributed-as-package` | DST-018 | `rust-version` set, and CI builds and tests with that toolchain (a USS-RUST-007 pin counts) |
| USS-RUST-014 | Fill the crates.io listing | SHOULD | `distributed-as-package` | DST-019 | `repository`, `keywords`, `categories` (valid slugs) and a README that Cargo finds |
| USS-TS-018 | Set the metadata that npm requires | MUST | `distributed-as-package` | DST-017 | `name`, `version`; a dry-run publish exits 0 |
| USS-TS-019 | Declare and test engines.node | SHOULD | `distributed-as-package` | DST-018 | the lowest line that `engines.node` admits is the lowest line in the CI matrix |
| USS-TS-020 | Fill the npm listing | SHOULD | `distributed-as-package` | DST-019 | `description`, `keywords`, `repository`, `bugs`, a README; no publint `INVALID_REPOSITORY_VALUE` |
| USS-TS-021 | Publish type declarations through exports | SHOULD | `is-library and distributed-as-package` | none | a `types` condition first in each `exports` entry; no publint type errors |
| USS-PY-017 | Set the metadata that PyPI requires | MUST | `distributed-as-package` | DST-017 | `name`; `version` static or `dynamic`; no `Private ::` classifier; `twine check --strict` passes |
| USS-PY-018 | Declare and test requires-python | SHOULD | `distributed-as-package` | DST-018 | the lowest version that `requires-python` admits is the lowest in the CI matrix |
| USS-PY-019 | Fill the PyPI listing | SHOULD | `distributed-as-package` | DST-019 | `description`, `readme`, `keywords`, `classifiers`, source and issue links |
| USS-GO-015 | Set the go directive to a tested version | SHOULD | `distributed-as-package` | DST-018 | the `go` directive names the lowest Go release in the CI matrix |
| USS-MVN-010 | Meet the Maven Central POM requirements | MUST | `distributed-as-package` | DST-017 | the POM elements of Maven Central; the latest deployment passed Central Portal validation |
| USS-K8S-009 | Set the metadata that Helm requires | MUST | `platform.helm and distributed-as-package` | DST-017 | `apiVersion: v2`, `name`, a SemVer `version`; `helm lint --strict` and `helm package` succeed |
| USS-K8S-010 | Declare and test kubeVersion | SHOULD | `platform.helm and distributed-as-package` | DST-018 | the lowest version that `kubeVersion` admits is the lowest that CI installs the chart on |
| USS-K8S-011 | Fill the chart listing | SHOULD | `platform.helm and distributed-as-package` | DST-019 | `description`, `home` or `sources`, `maintainers`, `keywords`, `icon` |
| USS-K8S-012 | Label each object with the recommended labels | SHOULD | `deploys-to-kubernetes` | none | `app.kubernetes.io/name`, `instance`, `managed-by` on each rendered object, and `helm.sh/chart` from charts ([recommended labels](https://kubernetes.io/docs/concepts/overview/working-with-objects/common-labels/)) |

Also add `cargo_common_metadata = "deny"` to the Clippy profile's baseline,
mapped to USS-RUST-014 as "partial" (the lint does not check category slugs);
its `cargo-ignore-publish` option suggests that it skips crates with `publish =
false` by default, which a run must confirm (not checked). The rationale of
USS-GO-015 claims that `go mod init` writes the developer's own release; confirm
it before use (not checked).

Applied to USS itself, DST-017 passes: `claude plugin validate` reports no error
for the plugin or the marketplace, and `publish = false` keeps the crate off
crates.io. RUST-013 passes, because `rust-version` equals the pinned CI
toolchain. DST-019 passes under the draft, because the only strict warning is
the directory icon. DST-021 was not checked, and DST-022 needs a decision.
DST-018 fails for the plugin: `plugin.json` has a host-version field
(`engines.claude-code`) that is not set, and setting it locks out Claude Code
v2.0.15 to v2.1.75, which reject the field ([plugin
reference](https://code.claude.com/docs/en/plugins-reference)).

**Decide** these, then add the rules in 0.2.0, together with the I18N rules of
section 8, after `uss lint`, a keyword scan and a read of each reference. The
`distributed-as-package` definition goes into 0.1.0.

| Choice | Recommendation | Alternative |
|---|---|---|
| DST-018 equality: declared minimum = tested minimum | Keep it. With PY-007, TS-008 and GO-007 it stops a project from claiming an untested end-of-life minimum (C12). | "…unless it is end-of-life", which accepts untested claims. |
| USS's own `engines.claude-code` | A Standard Deviation for DST-018, because older hosts reject the field and CI runs no Claude Code version; that CI cannot pin an old host cheaply is not checked. | Set the field and add a CI job on the lowest version. |
| USS-DST-022 | Keep it as a MAY. | Drop it; most software is not cited. |
| Categories of DST-019 and DST-021 | Accessibility & Usability, because their effect is that users find and understand the software. | Compatibility, for one category across the metadata rules. |
| Several clauses in one body (RUST-014, TS-020, PY-017) | Split PY-017's render check into PY-019; keep the others. | One clause per body throughout. |

**Effort** M for USS. LIC-001 cites SPDX License List 3.29.0 while crates.io
accepts identifiers from list 3.20
([manifest](https://doc.rust-lang.org/cargo/reference/manifest.html)); DST-017
catches a mismatch through the registry's own check, so LIC-001 needs no change.

## 8. Locale and translation: audits check it, run in it, and later ship it

"Locale and translation in terms of the audit" covers three parts: (A) what an
audit checks in a project, (B) the audit in the developer's language, and (C)
translations of USS itself, later.

### 8A. What an audit checks: switch on seven rules for translated software

The six I18N rules cover encoding, text out of code, locale formatting for
display, UTC storage, RFC 3339 exchange and an optional second language, and
USS-ACC-001 already brings in WCAG 2.2's language-of-page and language-of-parts
criteria ([SC
3.1.2](https://www.w3.org/WAI/WCAG22/Understanding/language-of-parts.html)). The
gaps are Unicode text handling (normalisation, collation, grapheme clusters),
bidirectional text, time-zone display, machine formats that change with the host
locale, and every rule for software that is actually translated. The clean gate
for the last group is a fact that USS-I18N-006 enables, as USS-EXT-001 enables
`has-plugin-interface`. An enabled fact beats an asked one, because a project
that ships two languages adopts I18N-006 anyway, and a second question could
disagree with that decision.

Change USS-I18N-006 in place before 0.1.0, and add the fact:

```markdown
### offers-multiple-languages

- **Enabled by:** USS-I18N-006
- **Definition:** Yes when the project adopts USS-I18N-006: its user interface or command line is available in at least two languages.
```

```markdown
### USS-I18N-006: Offer more than one language

- **Level:** MAY
- **Applies when:** has-ui or has-cli
- **Category:** Accessibility & Usability
- **Enables:** offers-multiple-languages
- **Verify:** Adopting this rule means that the UI or the command line is
  available in at least two languages. The project lists its languages (in
  the documentation, or as the language files that it ships). The user can
  use each one, by the system language or by a setting. For each listed
  language, open the screens of the main workflows, or run the documented
  commands: their text is in that language. A listed language that shows the
  text of another language there fails. A project that adopts the rule and
  lists fewer than two languages fails.
- **References:** W3C Internationalization, Localization vs.
  Internationalization (read 2026-11-24)

The user interface or the command line MAY be available in more than one
language.

**Rationale:** Most people use software best in the language they think in.
The rule on keeping text out of code (USS-I18N-002) is what makes a second
language a matter of a new file and not of a new version. Adopting this rule
switches on the rules for translated software.
```

Then add 13 rules in 0.2.0: six for any UI or CLI and seven behind the fact.
Each Verify has a concrete test, shown in the table; the full texts are in the
locale notes (`locale_translation.md`, part A, in the same folder). MUST is used
where the absence is a defect in every project that the gate selects and
platform or library support makes it achievable; SHOULD where legitimate reasons
to skip exist.

| ID | Title | Level | Applies when | The test in the Verify | References to pin |
|---|---|---|---|---|---|
| USS-I18N-007 | Isolate the direction of text from outside | SHOULD | `has-ui` | A Hebrew name followed by a number or brackets keeps its order (`dir="auto"`, `bdi` or the platform's isolation). | UAX #9 (confirm); W3C RTL markup ([W3C](https://www.w3.org/International/questions/qa-html-dir)) |
| USS-I18N-008 | Keep machine formats independent of the locale | MUST | `has-ui or has-cli or stores-persistent-data or uses-network` | Run with `LC_ALL=C.UTF-8` and `LC_ALL=tr_TR.UTF-8`: the data that programs read is identical, and each run reads the other's (decimal comma, dotless `ı`). | POSIX.1-2024 §8.2 ([Open Group](https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/V1_chap08.html)); a source on invariant formats (confirm) |
| USS-I18N-009 | Show times in the user's time zone | SHOULD | `has-ui` | A stored January time moves by 10 h 30 min between `America/New_York` and `Asia/Kolkata`; no fixed offsets. | IANA tz 2026f ([IANA](https://www.iana.org/time-zones)); W3C time and time zones (section to confirm) |
| USS-I18N-010 | Compare text by Unicode equivalence | SHOULD | as I18N-008 | `café` with U+00E9 matches `e` + U+0301; with case ignored, `STRASSE` matches `straße`. | UAX #15, revision 58 ([UAX #15](https://www.unicode.org/reports/tr15/)); default case algorithms (confirm) |
| USS-I18N-011 | Sort text with the collation of the user's language | SHOULD | `has-ui` | `de-DE` gives `Äpfel, apple, Österreich, Zebra`; `sv-SE` gives `apple, Zebra, Äpfel, Österreich`; code-point order fails. | UTS #10 (confirm); CLDR 49 ([CLDR](https://cldr.unicode.org/index/downloads)) |
| USS-I18N-012 | Cut text only between grapheme clusters | SHOULD | `has-ui or has-cli` | `👍🏽` and `e` + U+0301 at the cut point keep their tone and accent, with no invalid UTF-8. | UAX #29 (confirm) |
| USS-I18N-013 | Build each message from one translatable pattern | MUST | `offers-multiple-languages` | No sentence joined from translated parts; no plural chosen in code; CLDR plural categories through the message format. | CLDR 49 plural rules; MessageFormat 2 ([MF2](https://messageformat.unicode.org/)); Fluent 1.0 ([Fluent](https://projectfluent.org/)) |
| USS-I18N-014 | Identify languages with BCP 47 tags | MUST | `offers-multiple-languages` | Every language value in settings, storage, APIs, `lang` and HTTP headers parses as BCP 47 with registered subtags. | RFC 5646, RFC 4647; IANA subtag registry; W3C ([tags](https://www.w3.org/International/questions/qa-choosing-language-tags)) |
| USS-I18N-015 | Negotiate the language of each message | SHOULD | `offers-multiple-languages` | Preferences `fr-CA, de` with `en` and `fr` give French; a message missing in French shows in the default language, not as its key. | RFC 4647 §3.4 ([RFC 4647](https://www.rfc-editor.org/rfc/rfc4647.html)); POSIX §8.2; RFC 9110 (confirm) |
| USS-I18N-016 | Mirror the layout for right-to-left languages | MUST | `offers-multiple-languages` | Each offered RTL language has an RTL base direction and a mirrored layout; with no RTL language offered, pass with the list as evidence. | W3C RTL markup; UAX #9 (confirm) |
| USS-I18N-017 | Check the message catalogues in CI | MUST | `offers-multiple-languages` | A CI job parses every catalogue and fails on a syntax error, an unknown message or a mismatched placeholder; a planted bad placeholder makes it fail. | Fluent 1.0; `msgfmt --check` (version to confirm) |
| USS-I18N-018 | Test with a pseudo-locale | SHOULD | `offers-multiple-languages` | The UI tests run in a pseudo-locale that accents, lengthens by about 40% and marks each text; unmarked, cut or split marks fail. | Microsoft pseudolocalization ([Microsoft](https://learn.microsoft.com/en-us/globalization/methodology/pseudolocalization)); RFC 5646 §2.2.1 |
| USS-I18N-019 | Give translators the context of each message | SHOULD | `offers-multiple-languages` | In a sample of 20, each message with a placeholder or of three words or fewer has a translator comment; a word with two meanings has two keys or contexts. | gettext manual (confirm); Fluent comments |

Right-to-left support sits in the I18N-016 Verify, not in a new fact, which
would add a question to every record for a rare case. Confusable identifiers
(UTS #39) and bidi controls in source code ("Trojan Source", CVE-2021-42574)
belong to the Security or Code quality areas. Optionally, before 0.1.0, widen
USS-I18N-003 to input ("shown and read in the format of the user's locale").
**Effort** M for USS (13 rules and about 10 references to read); S for an
adopter with one language (six SHOULDs and one MUST, mostly met by the
platform); M to L for translated software. **Alternatives:** all translation
quality inside the I18N-006 Verify (one MAY gives no score signal); an asked
fact `is-localized` (duplicates the MAY); a module (modules are for
technologies, and these rules are language-neutral).

### 8B. The audit in the developer's language, set by the record

English stays the language of everything normative or machine-read: rule IDs,
fact IDs, stack identifiers, record and results keys, outcomes, the five
keywords in capitals, the Verify text that the agent applies, quoted commands
and output, and the English compliance statement. The conversation follows the
developer and leaves nothing in the repository. The evidence, reasons, draft
Standard Deviations and the report follow one optional record key,
`project.language`, a BCP 47 tag that defaults to `en`, so one results file
stays readable by the next auditor, agent and CI run.

`uss report` and `uss release-check` take the report language from `--lang`,
then `project.language`, then English, and never from the environment, because
`uss release-check` writes each report again in CI and must match the committed
one. Other messages follow C7. An unknown tag falls back by RFC 4647 lookup,
which truncates subtags until one matches and otherwise returns the default
([RFC 4647](https://www.rfc-editor.org/rfc/rfc4647.html)), then to English, with
a note on stderr. Dates stay `YYYY-MM-DD` in every language, and percentages use
one message per language, not a locale library. A report in another language
shows its statement, then the English statement, marked as normative, so readers
and tools that look for "Compliant with USS" still find it. One template with
Fluent catalogues keeps the layout equal in every language; the English plurals
that `score.rs` builds in code today move into the catalogue:

```ftl
# i18n/en/report.ftl
part-must-failures =
    { $n ->
        [one] { $n } MUST failure
       *[other] { $n } MUST failures
    }

# i18n/de/report.ftl (the keyword stays in English capitals)
part-must-failures =
    { $n ->
        [one] { $n } MUST-Verstoß
       *[other] { $n } MUST-Verstöße
    }
```

The drafts for the workflow, schema and conventions:

```markdown
### Language of the audit

1. Speak to the developer in the language that the developer uses.
2. Find the language of the record. It is `project.language` in the record.
   With no key, it is English.
3. Write the evidence, the reasons and the draft Standard Deviations in the
   language of the record.
4. Copy commands, paths, code and command output exactly. Do not translate
   them.
5. Do not translate rule IDs, fact IDs, record keys, outcomes or stack
   identifiers.
6. Write MUST, MUST NOT, SHOULD, SHOULD NOT and MAY in English capitals.
7. Apply the English text of each rule. A translation is informative.
8. To show rules in the language of the developer, run
   `uss applicable --full --lang <tag> .uss/record.yaml`.
9. If USS has no translation of a text, translate it yourself. Say that your
   translation is informative.
10. Write each date as `YYYY-MM-DD`.
```

Add to "Questions to the developer", step 1: "Give each fact ID in English. Ask
its question in the language of the developer." Add after "Start the record",
step 2: "Ask the developer for the language of the evidence and the reports.
Write its BCP 47 tag in `project.language`, such as `de` or `pt-BR`. Leave out
the key for English." Add to "With no person present": "Write the evidence in
the language of the record." Step 8 of the new subsection waits for `--lang`
(8C); until then the agent translates and says so (step 9).

```json
"language": {
  "type": "string",
  "pattern": "^[a-z]{2,3}(-[A-Z][a-z]{3})?(-([A-Z]{2}|[0-9]{3}))?$"
}
```

The pattern under `project.properties` in `schema/record.schema.json` admits a
language, an optional script and an optional region, the short forms that W3C
advises. Add a commented `language` line to `templates/record.yaml`, a row to
conventions section 11 ("The language of the evidence, the reasons and the
report, as a BCP 47 tag. The default is `en`."), and to conventions section 8:
"When the language of the report is not English, the report shows the statement
in that language, then the English statement. The English statement is
normative." **Effort** S for the workflow, schema and conventions; M for the
Fluent move of all messages and the report, which also closes USS-I18N-002
(4.2). **Alternatives:** everything in English with only the conversation
translated (developers cannot read their own evidence); the language from `LANG`
(CI would write a different report); one template per language (the layouts
drift); gettext PO for `uss` (no native plural variants per language).

### 8C. Translations of USS itself: English stays normative

Comparable standards agree that one language is normative and translations are
best-effort, but they differ in keywords and in how they show lag. W3C keeps the
English version authoritative "in all cases and in case of dispute" and gives
volunteer translations no official status ([W3C translation
policy](https://www.w3.org/2005/02/TranslationPolicy)). SemVer shows three
keyword styles: Spanish translates the keywords and cites a Spanish RFC 2119
([es](https://semver.org/lang/es/)), while German and Japanese keep the English
keyword in parentheses after the verb, as in "muss (MUST)"
([de](https://semver.org/lang/de/), [ja](https://semver.org/lang/ja/)).
Whole-file translations go stale: the Contributor Covenant's 41 translations
spread across four versions
([translations](https://www.contributor-covenant.org/translations/)), and Keep a
Changelog marks a language that lacks the current version with "(n/a)".

**Recommendation.** English is the only normative text. A translated rule keeps
its keyword in English capitals in parentheses after the translated verb, which
`uss lint` can check: the same IDs, the same fields and the same keyword in each
body, and no other. po4a makes one PO file per English Markdown file under
`i18n/<tag>/po/`, and fills each missing or changed paragraph with the English
text ([po4a](https://po4a.org/)), so a translation never shows text of another
version; the generated Markdown is committed under `i18n/<tag>/text/`, and CI
fails when it is out of date, as with a lockfile. Any PO editor or Weblate can
front it. `build.rs` embeds `i18n/`; `uss show --lang <tag> <path>` prints a
translation with a notice (version, share translated, English normative), `uss
show --lang <tag>` lists shares, and `uss applicable --full --lang <tag>` prints
translated rules, with RFC 4647 fallback. A translation is built from the
English text of the same commit, so it always pins the same USS version, and a
lag shows as a lower share; updates are not normative, so they ship in PATCH
releases, and no release waits for one. `CHANGELOG.md` lists the share per
language. Each language has a maintainer and a reviewer in CODEOWNERS for
`i18n/<tag>/`; machine translation may draft, but a unit counts only after a
second person who reads the language approves it. A translation is complete for
a version when every unit is translated and approved and the lint passes.

Translate in the order of who reads it: the report and `uss` messages; each
fact's question and definition; rule titles; the Standard Deviation template,
the record template comments and the AGENTS.md snippet; rule bodies, one area at
a time; the README and conventions. Leave AGENTS.md in English at first: agents
apply the English workflows and talk to the developer in their language, and the
ASD-STE100 style keeps a machine translation reliable for a person who wants
one.

```markdown
## 14. Languages and translations

The English text of the standard is the only normative text. A translation
is informative. In a dispute, the English text of the same version decides.

- A translation pins the USS version of its English source. It MUST say at
  its top that the English text is normative.
- A translation MUST keep rule IDs, fact IDs, field names, record keys,
  outcomes and stack identifiers in English.
- A translated rule MUST keep its keyword in English capitals, in
  parentheses after the translated verb, such as `muss (MUST)`.
- A translated rule MUST have the same Level, Applies when, Category and
  References as its English rule.
- A translation that lacks a paragraph shows the English paragraph. It states
  the share of the text that it translates.
- Records, results files and reports write dates as `YYYY-MM-DD` in every
  language.

`uss lint` checks each translation against its English file: the same rule
IDs, the same fields, and the same keyword in each rule body, and no other.

A translation is complete for a version when every paragraph and message is
translated, a second person who reads the language approved each one, and
`uss lint` passes.
```

**Effort** L: conventions section 14, the `i18n/` layout, the po4a
configuration, the translation lint, `--lang` for `show` and `applicable`,
embedding, a governance page in `CONTRIBUTING.md`, and one pilot language.
**Alternatives:** whole-file copies per version (stale, untracked paragraphs);
Weblate's own Markdown support, which makes Weblate the source of truth and does
not import edits made in the repository
([Weblate](https://docs.weblate.org/en/latest/formats/markdown.html)); Crowdin,
free for open source but proprietary, with a translation-memory condition (not
checked); W3C-style authorised translations with a 30-day review (too heavy
now); translated keywords (no program can check them); translation by the agent
alone (no reviewed text, and two agents give two texts). Not checked: po4a's
default threshold for partial files (set it to 0), whether Weblate reads Fluent
files, and the licence terms for contributed translations.

### The order of the locale work

| When | Work | Effort |
|---|---|---|
| Before 0.1.0 | Change USS-I18N-006 and add `offers-multiple-languages`; optionally widen I18N-003 to input. | S |
| Before 0.1.0 | Add `project.language` to the schema, the template, `uss check` and conventions §11; add "Language of the audit" and the three step changes to AGENTS.md; add the first paragraph of conventions §14. | S |
| Before 0.1.0 | **Decide** USS-I18N-002 for `uss`: the Fluent move (M; better if the release can wait) or a Standard Deviation that names this plan (S). | S or M |
| 0.2.0 | I18N-007 to I18N-019, with each reference read and pinned, released with the metadata rules. | M |
| After 0.2.0 | `uss report --lang`, `release-check --lang`, the bilingual statement and one pilot report language. | M |
| After 0.2.0 | The translation pipeline: po4a, `i18n/`, the lint, `show --lang`, `applicable --full --lang`, the governance page, and a pilot of the report, facts and rule titles. | L |
| Then | Further languages and rule bodies, one area at a time, in PATCH releases. | S per area and language |

Not checked for this design: the revisions of UAX #9, UAX #29 and UTS #10 for
Unicode 18.0.0; a source on invariant machine formats; the gettext manual
(`LANGUAGE` before `LC_ALL`, `msgctxt`, `msgfmt --check`), which returned HTTP
429; RFC 9110's `Accept-Language` section; whether clap's derive API takes
translated help at run time; and whether the plugin skills repeat workflow text
that would need the language steps.

## Conclusion

The R8 reversal is less about a ruling than about how USS audits releases at
all. Every release audit precedes the tag that it certifies, so any Verify
phrased as "each release has…" can only ever examine the previous release;
judging the release process at the release commit is the reading that makes `uss
release-check --require-compliant` meaningful, and the maintainer should decide
whether to apply it to every release, not just the first. One principle explains
why the metadata and locale designs hold up: both defer to an outside authority
judged at the audit date (a host's own validator; CLDR, BCP 47 and the tz
database) and keep what programs read fixed in English, which lets the rules
survive changes they cannot predict.

The real deadline in this list is not the release date but the 0.1.0 freeze.
Narrowing `stores-persistent-data`, aligning DEP-006 with its own body, scoping
TST-003 to code, the `distributed-as-package` and I18N-006 changes, and the Plan
3 refinements all cost one edit now and a retired rule ID later; and after 1.0.0
a MINOR release can no longer change the record format, so `project.language`
and the results `questions:` list belong in the 0.x line. Making the repository
public converts most of the remaining work from "pay or deviate" into
configuration, which leaves the trial as the one input that no amount of
research can replace.
