# USS Security, Supply Chain and Dependencies Implementation Plan (Plan 2 of 5)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Draft the first three core areas of the standard, Security [SEC],
Supply chain [SUP] and Dependencies [DEP], as rule files that pass `uss lint`.
Then stop, so the user can calibrate levels and tone before any other area is
written (spec §13 step 2).

> **Calibrated 2026-11-24.** The user's calibration renumbered the drafted IDs
> before release, so the IDs below are the draft's. New rules: USS-SEC-008
> and USS-SEC-025. Draft SEC-008 to SEC-023 are now SEC-009 to SEC-024, and
> draft SEC-024 to SEC-031 are now SEC-026 to SEC-033. Draft SUP-002 moved to
> Contributability; draft SUP-003 to SUP-017 are now SUP-002 to SUP-016. DEP
> IDs are unchanged. `standard/core/` is the current text.

**Architecture:** Mostly content. Each area is one rule file under
`standard/core/`, in the format that `standard/conventions.md` defines and
`uss lint` enforces. Six new asked facts go into `standard/applicability.md`
first, because the rules' `Applies when` expressions use them. One small task
clears the Plan 1 follow-ups. No validator feature changes.

**Tech Stack:** Markdown rule files; the `uss` validator from Plan 1 (Rust
1.100.0, `cargo xtask check`). No new dependencies.

**Spec:** `docs/superpowers/specs/2026-11-21-universal-software-standard-design.md`
(§5 format and levels, §6.1 the SEC, SUP and DEP rows, §6.4 rule-writing bar,
§6.5 check integrity). The normative rulebook is `standard/conventions.md`:
read sections 1–7 before writing any rule.

**Roadmap:** Plan 1 (foundation, `uss`) is merged. This is Plan 2. Plan 3
writes the remaining core areas after the user's review of this plan's output.

## Global Constraints

- Keywords: exactly `MUST`, `MUST NOT`, `SHOULD`, `SHOULD NOT`, `MAY` (RFC 2119
  as clarified by RFC 8174). `SHALL`, `SHALL NOT`, `REQUIRED`, `RECOMMENDED`,
  `NOT RECOMMENDED`, `OPTIONAL` are never used. One rule, one level.
- A rule body contains its own keyword and no other keyword, rationale
  included (conventions §3 "Keyword scan"). A keyword wrapped across a line
  break still counts. Put any other keyword only inside a fenced code block.
- Rule IDs: `USS-<PREFIX>-<NNN>`. In each new file the IDs run from 001 in
  file order, with no gaps, exactly as the tables below give them.
- Every rule has `Level`, `Applies when`, `Category`, `Verify` and
  `References`, and a body. `Category` is the category name, exactly as
  `standard/registry.yaml` writes it (`Safety`, `Maintainability`).
- The rule-writing bar (spec §6.4, conventions §7): one requirement per rule;
  verifiable from the project's artifacts; language-neutral (tools appear
  only as examples, "such as …"); a `**Rationale:**` paragraph when the
  reason is not self-evident.
- Reference, don't reinvent: where an external standard defines the content
  (OWASP ASVS, SLSA, SPDX/CycloneDX, RFC 9116, FIPS 140-3, NIST SP 800-52,
  NIST SP 800-131A, NIST SSDF, CISA Secure by Design, OpenSSF guidance), the
  rule requires conformance and cites it, and never restates it.
- References MUST be checked against the current source when the rule is
  written, not cited from memory, and MUST pin a version (conventions §7).
  The tables below name what to cite; the implementer confirms the current
  version and the exact section from the source itself.
- Verify text says what an auditor looks at. For a requirement that lives
  in repository-host settings (branch protection, review rules, token
  defaults), Verify names the host settings or API and the OpenSSF Scorecard
  check as evidence; without access the auditor records `unverified`.
- The standard's version stays `0.1.0`. It is unreleased; Plan 5 releases
  0.1.0.
- Code changes (Tasks 1–2 only) follow the Plan 1 lint policy: Clippy `all`
  and `pedantic` deny, `missing_docs` deny, no `unwrap`/`expect`/`panic`
  outside tests, no `#[allow]`. Every cargo command in xtask and CI uses
  `--locked`.
- The content of `Initial_Idea.md` and `todo.txt` is never modified.
- Files are UTF-8 with LF line endings. Wrap prose at about 80 columns, as
  the existing standard files do.

## Plan-level decisions (refining the spec, for the user's review)

- **Vendoring is an asked fact plus a MUST, not a MAY rule.** Spec §6.1 says
  projects "MAY vendor". A MAY rule would count toward the Safety score, and a
  declined MAY scores 0, so every project that does not vendor would lose
  Safety points for a neutral choice. Instead, the asked fact
  `vendors-dependencies` makes DEP-010 apply: vendored code is pinned,
  documented and updated like any other dependency.
- **OWASP ASVS is scoped to web UIs and network APIs at every level.**
  Conventions §7 gives "Level 2 is a MUST when `handles-authentication or
  handles-sensitive-data`" as an example. ASVS covers web applications and
  web services, so SEC-019 and SEC-020 AND that condition with
  `has-web-ui or exposes-network-api`. Task 3 updates the conventions example
  to match.
- **Signing, SBOM and provenance live in SUP.** Spec §6.1 also mentions signed
  artifacts under DST and REL. Plan 3 references SUP-014 to SUP-017 from those
  areas instead of adding duplicate rules.
- **CI is assumed.** The Testing area (Plan 3) will require tests in CI, so
  the CI rules here apply `always`.
- **Each requirement lives in exactly one rule.** DEP-008 (fix or assess each
  known vulnerability) and SEC-030 (publish VEX statements to users) are
  different requirements. SUP-010 (review before merge) covers dependency
  update PRs too, so DEP-014 does not repeat it.

## Review Focus

1. **A small project sees only the rules that fit it** (spec §2 anti-goal).
   A record with every fact `false` must get exactly the 23 `always` rules
   listed in Task 6, and nothing that needs a network, a database or
   dependencies. Pinned in Task 6 Step 1.
2. **A rule that cannot be judged from artifacts.** Host-setting rules
   (SUP-008 to SUP-011) say how to verify them, and that the auditor records
   `unverified` without access. Checked in the Task 4 review.
3. **The same requirement written twice.** Within this plan and against the
   spec's other areas (DST, REL, QLT, CTB in Plan 3), each requirement has
   one home. Checked in each area task's review against the decisions above.
4. **Language-neutral wording.** A core rule never requires one language's
   tool. Ecosystem syntax and tool names appear only as examples. Checked in
   each area task's review.
5. **Keyword discipline in rationale text.** For example, a SHOULD rule whose
   rationale says "MUST" fails lint. `uss lint` pins it in every area task.

---

### Task 1: Plan 1 follow-ups

**Files:**
- Modify: `standard/conventions.md` (section 11, "The report" and "To record an audit")
- Modify: `tools/uss/src/catalog/registry.rs` (test `load_registry_valid`)
- Modify: `tools/uss/src/cli/mod.rs` (two test comments, near the tests `run_report_rejects_invalid_record` and `release_check_rejects_invalid_record`)

**Interfaces:** none new.

- [ ] **Step 1: Correct the audit steps in `standard/conventions.md` §11**

In "The report", add after the first sentence: `uss report` and
`uss release-check` first run the checks of `uss check` on the record, and
refuse a record that fails them. Rewrite "To record an audit" in this order
(ASD-STE100 style, one instruction per step):

1. Write the results to `.uss/audits/<date>.yaml`.
2. Add the audit to `audits` in `.uss/record.yaml`.
3. Go to the project root.
4. Run `uss check .uss/record.yaml`. Correct each problem that it reports.
5. Run `uss report .uss/audits/<date>.yaml`. It writes
   `.uss/audits/<date>.md`, then prints the statement and the score line.
6. Commit the record, the results and the report together.

- [ ] **Step 2: Stop pinning the release version in `load_registry_valid`**

Replace `assert_eq!(r.version, "0.1.0");` with
`assert_eq!(r.version, env!("CARGO_PKG_VERSION"));`.

- [ ] **Step 3: Correct the two test comments**

Both say "A misspelled fact leaves `uses-database` false, so USS-SEC-002 drops
out." Replace each with: "A misspelled fact is an unknown fact, and the fact
it was meant to answer is left unanswered, so the record checks reject the
record."

- [ ] **Step 4: Run all checks**

Run: `cargo xtask check`
Expected: exit 0, all seven steps pass, `uss lint: ok (0 rules, 0 files)`.

- [ ] **Step 5: Commit**

```bash
git add standard/conventions.md tools/uss/src/catalog/registry.rs tools/uss/src/cli/mod.rs
git commit -m "docs: run uss check before uss report; fix Plan 1 test leftovers"
```

---

### Task 2: New asked facts

**Files:**
- Modify: `standard/applicability.md`
- Modify: `templates/record.yaml`
- Modify: `tools/uss/src/catalog/facts.rs` (test `applicability_file_is_valid`)

**Interfaces:**
- Produces these asked facts, used by Tasks 3–5: `uses-network`,
  `uses-secrets`, `uses-container-images`, `has-dependencies`,
  `uses-private-packages`, `vendors-dependencies`.

New and changed facts (Question, Definition, Implies). Place each new fact
after the fact named in "After", so related facts sit together:

| Fact | After | Question | Yes when | Implies |
|---|---|---|---|---|
| uses-network | has-cli | Does it send or receive data over a network? | it opens network connections or accepts them | |
| uses-secrets | handles-sensitive-data | Does it need secrets to run? | it needs passwords, API keys, tokens, private keys or similar secrets at run time | |
| uses-container-images | deploys-to-kubernetes | Does the repository use container images? | its Dockerfiles, Compose files, Kubernetes manifests, Helm charts or CI jobs name a container image | |
| has-dependencies | distributed-as-package | Does it use third-party code? | it uses code that it does not maintain itself, such as packages from a registry, libraries from other repositories, or vendored code | |
| uses-private-packages | has-dependencies | Does it use packages from a private registry or repository? | any of its dependencies comes from a package registry or repository that is not public | has-dependencies |
| vendors-dependencies | uses-private-packages | Does it keep copies of third-party code in its repository? | its repository holds third-party source or binaries that it builds or ships | has-dependencies |

Add `Implies` to existing facts (keep any `Implies` they already have,
comma-separated):

| Existing fact | Add to Implies |
|---|---|
| has-web-ui | uses-network |
| exposes-network-api | uses-network |
| ships-container-image | uses-container-images |
| deploys-to-kubernetes | uses-container-images |

- [ ] **Step 1: Update the test first**

In `applicability_file_is_valid`, add the six new IDs to `want`, and change
`assert_eq!(want.len(), 27);` to `assert_eq!(want.len(), 33);`.

- [ ] **Step 2: Run the test to verify it fails**

Run: `cargo test --locked -p uss applicability_file_is_valid`
Expected: FAIL (the six facts do not exist yet).

- [ ] **Step 3: Add the facts and the `Implies` lines to `standard/applicability.md`**

Follow the existing style: `### <fact-id>`, then `- **Question:**`,
`- **Definition:** Yes when …`, and `- **Implies:**` where the tables say so.
No RFC 2119 keywords in this file.

- [ ] **Step 4: Add the six facts to `templates/record.yaml`**

Use the same order as `applicability.md`. Each one has the template's existing
form: `# <Question>`, `# Answer this. false is not a default.`, then
`<fact>: false`.

- [ ] **Step 5: Run all checks**

Run: `cargo xtask check`
Expected: exit 0. `applicability_file_is_valid` and
`record_template_matches_standard` pass; `uss lint: ok (0 rules, 0 files)`.

- [ ] **Step 6: Commit**

```bash
git add standard/applicability.md templates/record.yaml tools/uss/src/catalog/facts.rs
git commit -m "feat(standard): add facts for network, secrets, container images and dependencies"
```

---

## How to write each area (Tasks 3–5)

These steps apply to each area task. The tables fix each rule's ID, title,
level, `Applies when` and requirement. The implementer writes the body, the
Verify text and the rationale, and confirms each reference.

- Header: `# <Area name>`, then `- **Prefix:** <PREFIX>` and
  `- **Kind:** area` (DEP also has `- **Applies when:** has-dependencies`).
- One short free-text paragraph after the header: what the area covers and
  its default category.
- `##` group headings exactly as the tables group the rules.
- Each rule: the heading `### USS-<PREFIX>-<NNN>: <title>` with the table's
  title; the five fields; a body whose first sentence states the table's
  requirement with the rule's keyword; a `**Rationale:**` paragraph unless the
  reason is self-evident. `Category` is `Safety` unless the table says
  otherwise.
- References: confirm each one from its source (web fetch). Pin the version
  and name the section where one exists, such as
  `OWASP ASVS 5.0 (V1 Encoding and Sanitization)`. Add CWE IDs where the
  table names them. If a source does not say what the table expects, report
  it instead of citing it.
- Check: after writing, run `cargo run --locked -p uss -- export --json` and
  compare each rule of the area with its table row: `id`, `title`, `level`,
  `appliesWhen` (the effective expression) and `category`.

### Task 3: Security area

**Files:**
- Create: `standard/core/security.md`
- Modify: `standard/conventions.md` (section 7, "Choosing a level", the OWASP ASVS example)

**Interfaces:**
- Consumes: the facts from Task 2 and the existing facts in `standard/applicability.md`.
- Produces: rules USS-SEC-001 to USS-SEC-031. USS-SEC-012 is the spec's and
  conventions' parameterized-query example; keep its fields exactly as
  conventions §3 shows them. If the ASVS 5.0 source shows that "injection
  prevention" is not the right section name, change the References of
  SEC-012 and of the conventions §3 example together, so they stay identical.

Header: `# Security`, `Prefix: SEC`, `Kind: area`. Default category Safety.

| ID | Title | Level | Applies when | Requirement | Cite |
|---|---|---|---|---|---|
| **## Design and secrets** | | | | | |
| SEC-001 | Keep a threat model | MUST | uses-network or handles-sensitive-data or stores-user-data | A written threat model covers the current architecture: assets, trust boundaries, threats and the controls for each. | NIST SSDF (SP 800-218) PW.1.1; OWASP Threat Modeling Cheat Sheet |
| SEC-002 | Secure by default | MUST | always | Every security feature is on in the default configuration; turning one off is an explicit, documented setting. | CISA Secure by Design |
| SEC-003 | Run without elevated privileges | MUST NOT | always | The software does not need administrator or root rights to run, except for documented operations that need them. | CWE-250 |
| SEC-004 | Keep secrets out of the repository | MUST NOT | always | No secret is committed to the repository or its history. | CWE-798 |
| SEC-005 | Scan for secrets in CI | MUST | always | CI runs a secret scanner on every change, and a finding blocks the merge until it is removed or suppressed with a reason. | OpenSSF guidance on secret scanning (confirm source) |
| SEC-006 | Load secrets at run time | MUST | uses-secrets | Secrets come from the environment, a secret store or a protected file outside the build, never from code or built artifacts. | CWE-798; OWASP Secrets Management Cheat Sheet |
| SEC-007 | Scope each credential | MUST | uses-secrets | Each credential the software uses grants only the permissions it needs. | NIST SP 800-53 AC-6 (least privilege) |
| **## Input and output** | | | | | |
| SEC-008 | Validate untrusted input | MUST | always | Input from outside the trust boundary is validated against an allow-list of expected form before use. | CWE-20; OWASP Input Validation Cheat Sheet |
| SEC-009 | Keep untrusted input out of shells | MUST NOT | always | Untrusted input is never passed to a shell or command interpreter; commands use argument arrays. | CWE-78 |
| SEC-010 | Do not deserialize untrusted data into arbitrary types | MUST NOT | always | Untrusted data is never deserialized with a format or library that can create arbitrary types or run code. | CWE-502; OWASP Deserialization Cheat Sheet |
| SEC-011 | Fail closed | MUST | uses-network or handles-authentication or uses-cryptography | When a security check cannot complete (an error, a timeout, a missing value), access is denied. | CWE-636 |
| SEC-012 | Use parameterized queries | MUST | uses-database | (exactly as conventions §3: Verify, References `OWASP ASVS 5.0 (injection prevention); CWE-89`, body and rationale; confirm the ASVS section name) | as conventions §3 |
| SEC-013 | Encode output for its context | MUST | always | Data written into another interpreter or format (HTML, SQL, shell, logs, terminals) is encoded or escaped for that context. | CWE-116; CWE-117 |
| **## Authentication and authorization** | | | | | |
| SEC-014 | Use standard authentication | MUST | handles-authentication | Authentication uses an established protocol (such as OIDC, SAML 2.0, WebAuthn or mutual TLS) through a maintained library, never a scheme of the project's own. | CWE-287; NIST SP 800-63B |
| SEC-015 | No default credentials | MUST NOT | handles-authentication | The software never ships with a default or shared password, key or token; the first run makes the operator set unique credentials. | CISA Secure by Design; CWE-1392; CWE-798 |
| SEC-016 | Offer multi-factor authentication | MUST | handles-authentication and has-ui | Every account that signs in interactively can use multi-factor authentication (directly, or through the identity provider). | CISA Secure by Design; NIST SP 800-63B |
| SEC-017 | Check authorization on every access | MUST | handles-authentication | Every access to a protected resource or operation is checked against the caller's permissions, in a component the caller does not control, and anything not explicitly allowed is denied. | CWE-862; CWE-863; OWASP Authorization Cheat Sheet; NIST SP 800-53 AC-3 |
| **## Web applications and APIs** | | | | | |
| SEC-018 | Meet OWASP ASVS Level 1 | MUST | has-web-ui or exposes-network-api | The software meets every ASVS Level 1 requirement. | OWASP ASVS 5.0 |
| SEC-019 | Meet OWASP ASVS Level 2 | MUST | (has-web-ui or exposes-network-api) and (handles-authentication or handles-sensitive-data) | The software meets every ASVS Level 2 requirement. | OWASP ASVS 5.0 |
| SEC-020 | Meet OWASP ASVS Level 3 | SHOULD | (has-web-ui or exposes-network-api) and high-assurance | The software meets every ASVS Level 3 requirement. | OWASP ASVS 5.0 |
| **## Transport and cryptography** | | | | | |
| SEC-021 | Encrypt network traffic | MUST | uses-network | Traffic that leaves the host uses TLS 1.2 or later, or another protocol with equal protection such as SSH. | NIST SP 800-52; RFC 9325 (BCP 195); RFC 8446 |
| SEC-022 | Keep certificate validation on | MUST NOT | uses-network | Certificate and host-name validation is never turned off, including in tests that reach real hosts. | CWE-295 |
| SEC-023 | Use vetted cryptography | MUST NOT | uses-cryptography | The project never implements its own cryptographic primitives or protocols; it uses a maintained, widely reviewed library. | CWE-327 |
| SEC-024 | No disallowed algorithms | MUST NOT | uses-cryptography | No algorithm or key size that NIST SP 800-131A disallows is used for a security purpose. | NIST SP 800-131A; CWE-327; CWE-328 |
| SEC-025 | Use FIPS 140-3 validated modules | MUST | requires-fips | All cryptography runs in modules validated under FIPS 140-3, in their approved mode. | FIPS 140-3; NIST CMVP |
| **## Security testing** | | | | | |
| SEC-026 | Run static security analysis in CI | MUST | always | CI runs a static application security testing (SAST) tool on every change, and a finding blocks the merge until it is fixed or suppressed with a reason. | NIST SSDF (SP 800-218) PW.7, PW.8 |
| **## Vulnerability handling** | | | | | |
| SEC-027 | Publish a vulnerability disclosure policy | MUST | always | The repository has a security policy (such as `SECURITY.md`) that says how to report a vulnerability privately and what happens next. | ISO/IEC 29147; CISA vulnerability disclosure guidance |
| SEC-028 | State response times | SHOULD | always | The security policy states target times to acknowledge a report, assess it and release a fix. | ISO/IEC 30111 |
| SEC-029 | Publish security advisories | MUST | distributed-as-package or ships-container-image | Each fixed vulnerability in a released version gets a published advisory that names the affected and fixed versions and a CVE or other public ID. | OSV schema; CSAF 2.0; CVE Program |
| SEC-030 | Publish VEX statements | SHOULD | distributed-as-package or ships-container-image | When a known vulnerability in a dependency does not affect a release, the project publishes a VEX statement for it with that release. | CISA VEX minimum requirements; OpenVEX or CSAF VEX or CycloneDX VEX |
| SEC-031 | Serve security.txt | SHOULD | operates-service and (has-web-ui or exposes-network-api) | Each web service the project operates serves `/.well-known/security.txt`. | RFC 9116 |

(The `## …` rows are group headings, not rules.)

SEC-005 and SEC-026 set a "no open findings" target, so their bodies say that
suppressions follow the check-integrity rules (spec §6.5; the Code quality
area in Plan 3 will hold them). Do not cite IDs that do not exist yet.

- [ ] **Step 1: Confirm the references**

For each row, fetch the source and note the current version and section.
Write the list (rule ID, reference as it will be cited, source URL) into the
task report. Flag any row whose source does not support the requirement.

- [ ] **Step 2: Write `standard/core/security.md`**

Follow "How to write each area" and the table.

- [ ] **Step 3: Update the ASVS example in `standard/conventions.md` §7**

In "Choosing a level", change the ASVS sentence so it matches SEC-018 to
SEC-020: Level 1 is a MUST when `has-web-ui or exposes-network-api`; Level 2
is a MUST when, in addition, `handles-authentication or
handles-sensitive-data`; Level 3 is a SHOULD when, in addition,
`high-assurance`.

- [ ] **Step 4: Lint**

Run: `cargo run --locked -p uss -- lint`
Expected: `uss lint: ok (31 rules, 1 files)`.

- [ ] **Step 5: Compare the export with the table**

Run: `cargo run --locked -p uss -- export --json`
Expected: 31 rules with prefix SEC whose `id`, `title`, `level`,
`appliesWhen` and `category` equal the table (category `Safety` for all).

- [ ] **Step 6: Run all checks**

Run: `cargo xtask check`
Expected: exit 0; the lint step prints `uss lint: ok (31 rules, 1 files)`.

- [ ] **Step 7: Commit**

```bash
git add standard/core/security.md standard/conventions.md
git commit -m "feat(standard): draft the Security area"
```

---

### Task 4: Supply chain area

**Files:**
- Create: `standard/core/supply-chain.md`

**Interfaces:**
- Consumes: `uses-container-images` (Task 2), `distributed-as-package`,
  `ships-container-image`, `high-assurance`.
- Produces: rules USS-SUP-001 to USS-SUP-017. Plan 3's DST and REL areas
  reference SUP-014 (SBOM), SUP-015 (signing) and SUP-016/017 (provenance)
  instead of duplicating them.

Header: `# Supply chain`, `Prefix: SUP`, `Kind: area`. Default category Safety.
Library and package dependencies are the Dependencies area (DEP); say so in
the free-text paragraph.

| ID | Title | Level | Applies when | Requirement | Cite |
|---|---|---|---|---|---|
| **## Toolchains and build inputs** | | | | | |
| SUP-001 | Pin the toolchain | MUST | always | The version of every compiler, runtime and build tool the build uses is pinned in a file in the repository, and local builds and CI use that version. | NIST SSDF (SP 800-218) PO.3 |
| SUP-002 | Install toolchains per project | SHOULD | always | Toolchains and build tools are installed per project (such as through a version manager, a wrapper script or a dev container), not system-wide. | none, or a source confirmed at writing |
| SUP-003 | Leave system-wide installs alone | MUST NOT | always | No build, setup or test script installs, upgrades or removes system-wide runtimes, package managers or global packages. | none, or a source confirmed at writing |
| SUP-004 | Pin CI components by commit | MUST | always | Every third-party CI component (such as a GitHub Action, a GitLab CI include or a CircleCI orb) is referenced by a full commit SHA or content digest, with its version in a comment. | GitHub Actions security hardening guide; OpenSSF Scorecard (Pinned-Dependencies) |
| SUP-005 | Pin container images by digest | MUST | uses-container-images | Every container image reference in the repository includes its content digest. | OpenSSF Scorecard (Pinned-Dependencies) |
| SUP-006 | Verify downloaded build inputs | MUST | always | Every file the build or CI downloads outside a package manager's lockfile (installers, scripts, binaries) is pinned to a version and checked against a checksum or signature. | NIST SSDF (SP 800-218) PW.4 |
| SUP-007 | No end-of-life platforms | MUST NOT | always | The project does not build with or run on a runtime, compiler, build tool or base image that its supplier no longer supports with security fixes. | the supplier's support policy; CWE-1104 |
| **## Source and CI protection** | | | | | |
| SUP-008 | Protect the default branch | MUST | always | The default branch and release tags are protected against force-pushes and deletion. | OpenSSF Scorecard (Branch-Protection) |
| SUP-009 | Require CI checks before merge | MUST | always | A change reaches the default branch only after the required CI checks pass. | OpenSSF Scorecard (Branch-Protection) |
| SUP-010 | Review changes before merge | SHOULD | always | Each change is reviewed and approved by someone other than its author before it merges. | OpenSSF Scorecard (Code-Review) |
| SUP-011 | Limit CI token permissions | MUST | always | Each CI job runs with the least token permissions it needs, read-only by default. | OpenSSF Scorecard (Token-Permissions); GitHub Actions security hardening guide |
| SUP-012 | Keep untrusted code away from secrets | MUST NOT | always | CI never runs code from an untrusted change (such as a pull request from a fork) in a job that has secrets or a write token. | OpenSSF Scorecard (Dangerous-Workflow); GitHub Security Lab guidance on `pull_request_target` |
| SUP-013 | Keep untrusted data out of CI scripts | MUST NOT | always | CI never inserts untrusted event data (titles, branch names, commit messages, comments) into a script it runs; it passes such data through environment variables or files. | GitHub Actions security hardening guide (script injection); CWE-78 |
| **## Release artifacts** | | | | | |
| SUP-014 | Publish an SBOM | SHOULD | distributed-as-package or ships-container-image | Each release publishes an SBOM in SPDX or CycloneDX format that lists every component and marks each as direct or transitive. | SPDX; CycloneDX; NTIA or CISA SBOM minimum elements |
| SUP-015 | Sign release artifacts | SHOULD | distributed-as-package or ships-container-image | Release artifacts are signed, and the release documentation says how to verify the signature. | Sigstore; SLSA |
| SUP-016 | Meet SLSA Build Level 2 | SHOULD | distributed-as-package or ships-container-image | Releases are built on a hosted build platform that produces signed provenance, meeting SLSA Build Level 2. | SLSA (Build track) |
| SUP-017 | Meet SLSA Build Level 3 | SHOULD | (distributed-as-package or ships-container-image) and high-assurance | Releases meet SLSA Build Level 3. | SLSA (Build track) |

Verify for SUP-008 to SUP-011 names the repository-host settings or API and
the matching OpenSSF Scorecard check as evidence, and says the auditor records
`unverified` without access. SUP-010's rationale says that a project with one
maintainer cannot meet it and records a Standard Deviation.

- [ ] **Step 1: Confirm the references** (as in Task 3 Step 1)
- [ ] **Step 2: Write `standard/core/supply-chain.md`**
- [ ] **Step 3: Lint**

Run: `cargo run --locked -p uss -- lint`
Expected: `uss lint: ok (48 rules, 2 files)`.

- [ ] **Step 4: Compare the export with the table** (17 SUP rules, all `Safety`)
- [ ] **Step 5: Run all checks**

Run: `cargo xtask check`
Expected: exit 0; `uss lint: ok (48 rules, 2 files)`.

- [ ] **Step 6: Commit**

```bash
git add standard/core/supply-chain.md
git commit -m "feat(standard): draft the Supply chain area"
```

---

### Task 5: Dependencies area

**Files:**
- Create: `standard/core/dependencies.md`

**Interfaces:**
- Consumes: `has-dependencies`, `uses-private-packages`,
  `vendors-dependencies` (Task 2), `is-library`.
- Produces: rules USS-DEP-001 to USS-DEP-017. The file header has
  `- **Applies when:** has-dependencies`, so every DEP rule's effective
  `appliesWhen` is `has-dependencies` ANDed with the rule's own condition
  (for `always`, just `has-dependencies`).

Header: `# Dependencies`, `Prefix: DEP`, `Kind: area`,
`Applies when: has-dependencies`. Default category Safety. The free-text
paragraph says that language modules and vendor profiles name each
ecosystem's syntax and tools.

| ID | Title | Level | Applies when | Requirement | Category |
|---|---|---|---|---|---|
| **## Declaring and resolving** | | | | | |
| DEP-001 | Bound every version range | MUST | always | Every dependency in every manifest is limited to a bounded range no wider than one major version (a "soft pin"), never `*`, `latest`, a branch or an unbounded `>=`. | Safety |
| DEP-002 | Commit a lockfile | MUST | always | A lockfile with an integrity hash for every resolved package is committed. | Safety |
| DEP-003 | Install only from the lockfile | MUST | always | Builds and CI install strictly from the lockfile, and fail when it is out of date. | Safety |
| DEP-004 | Declare direct dependencies | MUST | always | Every package the code uses directly is declared as a direct dependency, never reached through another package's dependencies. | Safety |
| DEP-005 | Separate runtime, build and development dependencies | MUST | always | Runtime, build and development dependencies are declared separately, so shipped artifacts carry only runtime ones. | Safety |
| DEP-006 | No end-of-life dependencies | MUST NOT | always | No dependency is at a version that no longer receives security fixes. | Safety |
| **## Vulnerabilities** | | | | | |
| DEP-007 | Scan dependencies for known vulnerabilities | MUST | always | CI scans all dependencies, direct and transitive, for known vulnerabilities on every change and on a schedule. | Safety |
| DEP-008 | Fix or assess each known vulnerability | MUST | always | Each known vulnerability that the scan reports is fixed, or assessed as not affecting the product with the reason recorded where the scanner reads it. | Safety |
| **## Sources** | | | | | |
| DEP-009 | Protect private package names | MUST | uses-private-packages | Private packages are scoped or their registry is configured so that a public package with the same name cannot replace them. | Safety |
| DEP-010 | Treat vendored code as a dependency | MUST | vendors-dependencies | Vendored code is pinned to an upstream version, its source, version and licence are documented, and it is scanned and updated like any other dependency. | Safety |
| **## Choosing and maintaining** | | | | | |
| DEP-011 | Document each direct dependency | SHOULD | always | The purpose of each direct dependency is recorded, in the manifest or a dependency list, when it is added. | Safety |
| DEP-012 | Check health before adding | SHOULD | always | Before a dependency is added, it is checked to be maintained, not deprecated or archived, licence-compatible and with more than one maintainer, and the standard library is preferred when it is enough. | Safety |
| DEP-013 | Remove unused dependencies | SHOULD | always | No declared dependency is unused. | Maintainability |
| DEP-014 | Automate dependency updates | SHOULD | always | Dependency updates arrive as automated pull requests (such as from Renovate or Dependabot) that pass CI before merge. | Safety |
| DEP-015 | Wait before adopting a new release | SHOULD | always | A newly published dependency release is adopted only after a minimum age of a few days, set in the update tool. | Safety |
| **## Libraries** | | | | | |
| DEP-016 | Declare the widest tested range | SHOULD | is-library | A library declares the widest range of each dependency that it is tested against, within the bound of DEP-001. | Safety |
| DEP-017 | Test the lowest versions | SHOULD | is-library | CI tests the library against the lowest version in each dependency range, as well as the newest. | Safety |

Cite: DEP-001 and DEP-002 the spec's ecosystem examples as examples in the
body (Cargo `1.2`, npm `^1.2.0`, Python `~=1.2`, Maven `[1.2,2.0)`;
`cargo build --locked`, `npm ci`, `uv sync --locked`); DEP-004 tools as
examples (deptry, knip, Maven `dependency:analyze`, `go mod tidy`); DEP-006
CWE-1104; DEP-007/008 OSV and NIST SSDF (SP 800-218) RV.1; DEP-009 the
"dependency confusion" write-ups confirmed at writing; DEP-014/015 the update
tools' documentation (Renovate `minimumReleaseAge`, Dependabot cooldown,
uv `--exclude-newer`) as examples. Use `none` only where no source applies.

DEP-008 sets a "no open findings" target: its body says that each recorded
assessment gives a reason (and, where the tool supports it, an expiry date),
per the check-integrity rules (spec §6.5).

- [ ] **Step 1: Confirm the references** (as in Task 3 Step 1)
- [ ] **Step 2: Write `standard/core/dependencies.md`**
- [ ] **Step 3: Lint**

Run: `cargo run --locked -p uss -- lint`
Expected: `uss lint: ok (65 rules, 3 files)`.

- [ ] **Step 4: Compare the export with the table**

Expected: 17 DEP rules; `appliesWhen` is `has-dependencies` for the `always`
rows and `has-dependencies and <condition>` for the others; DEP-013 is
`Maintainability`, the rest `Safety`.

- [ ] **Step 5: Run all checks**

Run: `cargo xtask check`
Expected: exit 0; `uss lint: ok (65 rules, 3 files)`.

- [ ] **Step 6: Commit**

```bash
git add standard/core/dependencies.md
git commit -m "feat(standard): draft the Dependencies area"
```

---

### Task 6: Review package, then stop

**Files:** none committed.

- [ ] **Step 1: Check what a minimal project sees**

The template record answers every fact `false`.
Run: `cargo run --locked -p uss -- applicable templates/record.yaml`
Expected: exactly these 23 rules, and no others:
SEC-002, SEC-003, SEC-004, SEC-005, SEC-008, SEC-009, SEC-010, SEC-013,
SEC-026, SEC-027, SEC-028, SUP-001, SUP-002, SUP-003, SUP-004, SUP-006,
SUP-007, SUP-008, SUP-009, SUP-010, SUP-011, SUP-012, SUP-013.

Run: `cargo run --locked -p uss -- check templates/record.yaml`
Expected: `uss check: ok` (no applicable MAY rules, so no decisions needed).

- [ ] **Step 2: Check a typical small project**

Copy the template to a scratch directory, set `has-cli`, `uses-network` and
`has-dependencies` to `true`, and run `uss applicable` on it.
Expected: the 23 rules above, plus SEC-001, SEC-011, SEC-021, SEC-022, and
DEP-001 to DEP-008 and DEP-011 to DEP-015 (40 rules in all).

- [ ] **Step 3: Hand over for the user's review, and stop**

Give the user: the three area files; a table of all 65 rules (ID, title,
level, applies when, category); the two counts above; and the calibration
questions below. Do not start Plan 3. The user's answers drive any changes,
which are made before Plan 3 is written.

Calibration questions for the user:

1. **Levels on the edge:** SEC-001 threat model (MUST), SEC-016 MFA (MUST),
   SEC-029 advisories (MUST), SUP-010 review before merge (SHOULD, because a
   solo maintainer cannot meet it), DEP-014 automated updates (SHOULD),
   SEC-031 security.txt (SHOULD).
2. **Applicability:** is 23 rules for a project with every fact `false` the
   right floor, given that most of them are CI and repository hygiene?
3. **Vendoring:** an asked fact plus a MUST (DEP-010), not a scored MAY (see
   plan-level decisions).
4. **ASVS scope:** Levels 2 and 3 apply only to web UIs and network APIs.
5. **Tone:** are the bodies short and plain enough, and is the
   rationale/Verify split right?
