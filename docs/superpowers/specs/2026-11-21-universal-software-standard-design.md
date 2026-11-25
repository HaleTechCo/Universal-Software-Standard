# Universal Software Standard (USS) — Design

Date: 2026-11-21
Status: Draft for approval

## 1. Intent

A real, adoptable engineering standard for all software, written in RFC 2119
terms. The practices in it are ones human developers and architects should
always have followed; coding agents now make following all of them practical.

The promise: **if a project follows every applicable rule, including adopting
every applicable MAY, it is as good as software can be made.** The standard
defines what that means, rule by rule, in a form an agent can apply, and scores
projects by category so they can see how close they are (§5.5).

Two ways it gets used:

1. **New project ("adopt").** A developer tells their agent to comply with this
   standard. The agent profiles the project, asks the developer about every
   applicable SHOULD, SHOULD NOT and MAY item, records the answers, and builds
   to the result.
2. **Existing codebase ("audit").** The agent does a quick scan of the code
   first, then asks the same SHOULD / SHOULD NOT / MAY questions, informed by
   what it found. It then audits every applicable rule, reports, and fixes the
   codebase toward compliance.

It is a growing project. The core is language-neutral; languages, frameworks,
platforms and tools with their own quirks get **modules** over time.

## 2. Goals, non-goals, anti-goals

**Goals**

- One standard any project in any language can adopt or be audited against.
- Agent-operable: stable rule IDs, machine-checkable applicability, a per-project
  record of decisions, and written workflows an agent can follow unaided.
- Verifiable rules: an auditor (human or agent) can decide pass/fail from the
  code, config, CI and docs.
- Grows by modules without changing the core's shape.

**Non-goals**

- Mandating languages, frameworks or vendors. The core never names a required
  language. (The Go / React / PostgreSQL / Rust table in `Initial_Idea.md` is
  superseded by this.)
- Rewriting established standards. Where OWASP ASVS, SLSA, WCAG, CIS etc. already
  define something, USS rules reference them and say which level applies.
- An automated code scanner. Auditing is agent + human judgment; the tooling
  only handles the deterministic parts (format, applicability, record checks).
- A certification body or badge program (v1).
- An MCP server for the standard (later; see §11).

**Anti-goals** (failure even if it technically works)

- Ceremony that makes small projects skip the standard. Applicability must prune
  hard: a single-file CLI with no UI, data or network should see a small
  fraction of the rules.
- Vague, unverifiable rules ("write clean code").
- Rules that force rewrites of existing codebases (e.g. "MUST use language X").
- Interviews that walk a developer through 150 questions one at a time.
- Agents marking rules "pass" without evidence.

## 3. Decisions made with the user

- **Language-neutral core.** No rule picks a language. Instead, whatever
  language is used MUST have its safety features, strict modes and hardening
  flags enabled, and memory, auth and other safety MUST be properly
  implemented. Language modules spell out the exact flags and checks.
- **Pass/fail with applicability**, not tiered levels. Each rule says when it
  applies; compliance is judged only against applicable rules.
- **Standard Deviation.** Anyone who chooses not to follow a rule documents it
  in a Standard Deviation; who approves it is up to their own organization's or
  stakeholders' policy, not USS.
- **Stable rule IDs** and a **per-project record file**.
- **Existing codebases** get the same questions as new ones, asked after a quick
  scan so the questions reflect what the code actually does.
- **Approach 2** (see §4).
- **Plugin interface and marketplace** are MAY items, with a section defining
  what an ideal plugin interface and an ideal marketplace must have.
- **Upgrades must be trivial** (from `todo.txt`): install over existing
  installs, detect existing installs, etc.
- **Modules** for languages and build tools grow over time (`todo.txt` names
  Python and Maven).
- **Levelling policy** (§6.4): MUST when the absence is a defect anywhere the
  rule applies and any team can meet it; SHOULD when there are legitimate
  reasons to skip; small projects stay light through applicability, not weaker
  levels; external standards' levels (ASVS, SLSA, CIS) scale with facts.
- **Reference, don't reinvent** (§6.4): where an established standard exists
  (SemVer and the like), USS requires and cites it instead of restating it.
- **Best-in-class tools** are named in modules (e.g. Ruff for Python linting),
  rather than USS defining its own lint rules (§6.3).
- **Scores by category** (safety, compatibility, features, …) alongside
  pass/fail compliance. MAYs count, so a project can score 100% on Safety and
  low on Features because it has no plugin interface or marketplace (§5.5).
- **Review cadence** (§13): structure and checker first, then Security and
  Supply chain drafted and reviewed by the user, then the rest in batches, with
  the user reviewing the finished 0.1.0.
- **Changelogs** follow Common Changelog, with a few USS additions; USS does
  not define its own changelog format (§6.1 REL).
- **Breaking changes** SHOULD NOT happen; when unavoidable they MUST be
  handled properly: MAJOR version, flagged, migration guide (§6.1 REL).
- **Audits run at any time** in a project's life, and **every release** MUST
  be audited with its report (compliance status and scores) published with it
  (§8.3).
- **Dependencies** area: bounded "soft pin" ranges in every manifest, exact
  locked resolution, direct dependencies declared and documented (§6.1 DEP).
- **The validator is written in Rust** (chosen over Go after weighing both), so
  a future VS Code extension can use it directly (§9).
- **Contributability** area: how easy it is to contribute (§6.1 CTB).
- **Backup** area (§6.1 BAK).
- **Vendor profiles**: tool vendors add their own sections mapping their
  tool's rules and configuration to USS rules, grouped by score category, with
  the tool's baseline config as the floor (§6.6).
- **Instructions** that a human or agent follows SHOULD be written in ASD-STE100
  Simplified Technical English (§6.1 DOC).
- **Check integrity**: "zero violations" cannot be reached by disabling
  checks; floors, justified and specific suppressions, and audited
  suppression inventories (§6.5).
- **New areas:** Audit logging [AUD], with DoD and NIST requirements
  referenced rather than rewritten; AI features [AI]; Performance & efficiency
  [PRF]; Usability [UX]; Operations [OPS]; Fair dealing [TRU], including "no
  rug-pulls" of features that were once free (§6.1).
- **Additions to existing areas:** test effectiveness (mutation,
  property-based and fuzz testing); source and CI protection; no end-of-life
  software; vulnerability handling (disclosure, security.txt, advisories, VEX);
  project continuity; AGENTS.md for agent contributors (§6.1).
- **Regulation modules**, starting with the EU Cyber Resilience Act, the
  European Accessibility Act and GDPR (§6.7).
- **Independence** SHOULDs: self-hostable; FOSS software does not require
  commercial products or subscriptions to work; support for the major
  databases (SQLite, PostgreSQL, MariaDB/MySQL) rather than one (§6.1 IND).
- **Commercial licensing** for COTS software: transparent published pricing,
  à la carte features, capped price increases, offline-file and self-hostable
  licence-server delivery, and many licence models including a CI/CD
  service-account licence (§6.1 COM).

**Calls made by Claude** (for the user to check): the USS name and ID prefix;
Privacy and Internationalization areas; Contributability as its own score
category; Backup's three scopes (what the software offers operators, the
project's own deployments, mirroring the source) (§6.1); "auditing" read as
audit logging (accountability trails), with "audit" alone reserved for USS
compliance audits; holding user data hostage is a MUST NOT while the other
fair-dealing rules are SHOULD NOT; grouping self-hosting, FOSS independence and
multi-database support into an Independence area, adding a "no single-cloud
lock-in" SHOULD to it, and requiring multi-database support to be proven by a
CI test matrix, with an abstraction layer (not specifically an ORM) as the
means; the COM specifics (price cap of inflation index + 3 points a year,
90 days' notice, renewal only; no penalties for virtualisation; in-product
licence usage visibility; grace periods and graceful expiry; perpetual
fallback); a Fairness score category for TRU and COM; the nine score categories, with Efficiency
and Fairness added and Accessibility widened to "Accessibility & Usability"
(§5.5); MUST deviations make a project "not
compliant" (§5.5); the score categories, equal weight per rule, deviations and
declined MAYs scoring 0, and no single overall score (§5.5); audit results
stored as data so scores are computed by the tool, not by hand (§7); 0.x
releases until the user declares 1.0.0 (§5.7); module rules make the outcome a
MUST and the recommended tool a SHOULD (§6.3); conduct behaviour that leaves no
artifact moves to agent instructions instead of audited rules (§6.1); one
record per released unit for monorepos (§7); release audits can run locally
or headless in CI, with `uss release-check` tying the audit to the release
commit (§8.3); vendor profiles live in this repo, are reviewed by USS
maintainers, and v1 ships community-maintained ones for the recommended tools
(§6.6); the Rust validator split into an I/O-free library and a thin CLI (§9);
Dependencies split out of Supply chain as its own area, taking the `DEP`
prefix, with Deployment hardening renamed to `HRD`; soft pins bounded above by
the next major version; declaring direct dependencies a MUST but documenting
their purpose a SHOULD; minimum release age for updates; testing libraries
against the lowest versions in their range (§6.1);
MIT licence for this repo; packaging as a Claude plugin plus a tool-agnostic
AGENTS.md.

## 4. Approaches considered

1. **Plain Markdown.** Rules as prose with inline IDs; AGENTS.md describes the
   workflows. Easiest to read and contribute to, but nothing enforces unique
   IDs, well-formed applicability, or record files that match the standard; it
   drifts as it grows.
2. **Markdown with a fixed rule format, machine-checked.** *(Chosen.)* Each rule
   is a Markdown section with a small structured header. A validator checks the
   standard; a JSON Schema plus the validator checks project record files.
   Agent entry points: AGENTS.md (any agent) and a Claude skill. Stays pleasant
   for humans to read and contribute to, while machines can rely on it.
3. **YAML catalog generating docs.** Most machine-friendly, easy diffs, but long
   normative prose in YAML is awkward and contributors edit data files instead
   of a document — works against an open standard people should adopt.

## 5. The standard's structure

### 5.1 Keywords and conformance language

- Keywords per **RFC 2119** as clarified by **RFC 8174**: only UPPERCASE forms
  carry normative meaning.
- Each rule has exactly **one level**: `MUST`, `MUST NOT`, `SHOULD`,
  `SHOULD NOT` or `MAY`. (The synonyms SHALL, REQUIRED, RECOMMENDED, OPTIONAL
  are not used, so an audit never has to interpret them.) One rule, one
  requirement, one level.

### 5.2 Rule format

Every rule is a level-3 heading with a fixed header, then normative text, then
an optional rationale:

```markdown
### USS-SEC-013: Use parameterized queries

- **Level:** MUST
- **Applies when:** uses-database
- **Category:** Safety
- **Verify:** No query is built by concatenating external input; all queries
  use bound parameters, or a query builder that binds them.
- **References:** OWASP ASVS 5.0 (injection prevention); CWE-89

Every database query that includes externally supplied values MUST pass those
values as bound parameters.

**Rationale:** ...
```

Header fields:

| Field | Required | Meaning |
|---|---|---|
| Level | yes | One of the five keywords |
| Applies when | yes | `always` or an applicability expression (§5.3) |
| Category | yes | Score category (§5.5); normally the area's default category |
| Verify | yes | How an auditor determines pass/fail |
| References | yes (may be `none`) | External standards, CWEs, specs |
| Enables | MAY rules only, optional | Fact(s) set to true when the project adopts this MAY |
| Refines | module and profile rules, optional | Core rule ID(s) this module or profile rule makes concrete for its technology |
| Status | optional | `Retired`, with `Retired in` and `Replaced by` |

### 5.3 Applicability

- Facts are boolean, kebab-case, and defined in one controlled vocabulary file
  (`standard/applicability.md`). Each fact has a definition and the question an
  agent asks (or infers from the code) to set it. Examples: `has-ui`,
  `has-web-ui`, `has-cli`, `stores-user-data`, `uses-database`,
  `exposes-network-api`, `handles-authentication`, `uses-cryptography`,
  `requires-fips`, `ships-container-image`, `deploys-to-kubernetes`,
  `is-library`, `distributed-as-package`, `has-plugin-interface`,
  `has-plugin-marketplace`, `stores-persistent-data`, `operates-service`,
  `accepts-external-contributions`, `uses-ai-models`, `exposes-mcp-server`,
  `has-multiple-users`, `is-commercial`, `sold-in-eu`,
  `covered-by-eaa`, `processes-eu-personal-data`, `is-foss`,
  `is-network-service`, `uses-relational-database`.
- Stack facts are derived from the record's stack lists: `lang.<id>`,
  `framework.<id>`, `platform.<id>`, `tool.<id>` (e.g. `lang.go`, `tool.maven`).
- Expression grammar: `always`, a fact, `not X`, `X and Y`, `X or Y`,
  parentheses.
- MAY rules with an `Enables` field switch facts on when adopted. Example:
  `USS-EXT-001` "MAY provide a plugin interface" enables `has-plugin-interface`,
  which makes the plugin-interface MUST/SHOULD rules applicable.
- A module declares a module-level `Applies when` (e.g. `lang.python`) that is
  ANDed with each of its rules.

### 5.4 Rule IDs

- Format: `USS-<AREA>-<NNN>`, e.g. `USS-SEC-012`. Modules use their own
  registered prefix: `USS-GO-004`, `USS-PY-010`, `USS-MVN-002`.
- Area and module prefixes are registered in `standard/conventions.md`.
- IDs are **never reused or renumbered**. A rule is never deleted: it is marked
  `Status: Retired` in place (with what replaced it), so links and old records
  stay valid. Changing a rule's meaning means retiring it and adding a new ID.

### 5.5 Compliance and scores

For a given USS version and a project's record, each applicable rule gets one
audit result: `pass`, `fail`, `deviation` (covered by a Standard Deviation),
`declined` (a MAY the project chose not to adopt), `unverified` (needs a human
to confirm), or `not-applicable`.

**Compliance** answers "are all gaps justified?" It is pass/fail:

- **Compliant**: every applicable MUST / MUST NOT is `pass`; every applicable
  SHOULD / SHOULD NOT is `pass` or `deviation`; every applicable MAY has a
  recorded decision; nothing is `unverified`.
- A MUST / MUST NOT can still be deviated from (organizations do what they
  must, and the Standard Deviation records it honestly), but the project is
  then **not compliant**, reported as "Not compliant: N documented MUST
  deviations". The word "compliant" keeps its meaning.
- Declining a MAY needs no justification and does not affect compliance.

**Scores** answer "how much of the ideal is actually there?" Every rule has
one score category. v1 categories (registered in `standard/conventions.md`;
each area has a default, and an individual rule may override it, e.g. signed
release artifacts in DST count toward Safety):

| Category | Measures | Mostly from |
|---|---|---|
| Safety | Security, memory and runtime safety, supply chain, dependencies, hardening, privacy, accountability, safe AI use | SAF, SEC, SUP, DEP, HRD, PRV, AUD, AI |
| Reliability | Keeps working, fails safely, recovers, is tested, can be restored, is operated well | RES, OBS, TST, DAT, BAK, OPS |
| Efficiency | Fast, lean, and sparing with energy and resources | PRF |
| Maintainability | Easy to change correctly | QLT, ARC, DOC, CON, REL |
| Contributability | Easy for someone new (human or agent) to start contributing, and the project outlives any one person | CTB |
| Compatibility | Runs where users need it, on their own terms; upgrades and integrates cleanly | DST, API, IND, LIC |
| Fairness | Keeps its promises to users; honest, flexible, non-predatory commercial terms | TRU, COM |
| Accessibility & Usability | Usable by everyone, in any language, without confusion | ACC, I18N, UX |
| Features | Optional capabilities that make software complete | EXT, install-method and data-portability MAYs |

- Score per category = applicable rules with result `pass` ÷ all applicable
  rules in that category, as a percentage. Every rule has equal weight, so a
  score reads plainly as "share of the ideal achieved".
- `deviation`, `declined`, `fail` and `unverified` all score 0. A score measures
  what is there; compliance measures whether the gaps are justified. A
  declined plugin marketplace is why a project can be 100% Safety and 40%
  Features.
- Rules switched on by an adopted MAY count from then on.
- A category with no applicable rules shows `n/a`.
- There is **no single overall score**: one number would hide exactly the
  differences the categories exist to show.

**"As perfect as it can be"** = compliant, with 100% in every applicable
category (which implies zero deviations and every applicable MAY adopted).

Statements always name the version and date: "Compliant with USS 1.0.0 as of
2026-11-21. Safety 100%, Reliability 94%, Efficiency 88%, Maintainability 97%,
Contributability 90%, Compatibility 80%, Fairness 100%, Accessibility &
Usability n/a, Features 40%."

### 5.6 Standard Deviation

A Markdown document per deviation, from `templates/standard-deviation.md`:
rule ID and level, USS version, scope (what exactly is not followed), reason,
risks and impact, compensating controls, approval (approver and date — process
per the adopter's own policy), review-by date, and remediation plan if any.

### 5.7 Versioning of the standard

SemVer, applied to the standard as a document. Projects pin a version in their
record, and upgrading is a deliberate act (just like pinned dependencies), so:

- **MAJOR**: changes to the conventions, rule format, record format or
  compliance definition; raising a rule's level; retiring rules en masse.
- **MINOR**: new rules, areas, modules or facts.
- **PATCH**: wording, references or verification text with no normative change.

Until the user declares 1.0.0, releases are **0.x**: the rule format, record
format and compliance definition may still change in a MINOR release. 1.0.0
does not require every module in §6.3; modules added after 1.0.0 are MINOR
releases.

`CHANGELOG.md` lists added, changed and retired rule IDs per release, so an
agent upgrading a project's pinned version only asks about what changed.

## 6. Content scope for v1

### 6.1 Core areas (language-neutral)

Seeded from `Initial_Idea.md` and `todo.txt`. Each file under
`standard/core/`, prefix in brackets.

| Area | Covers |
|---|---|
| Conduct [CON] | Delivery integrity, written as rules checkable from the repository: no placeholders/stubs in production code paths, no disabled checks or skipped tests without a tracked reason (the check-integrity rules that enforce this are QLT's), CI (not a person's or agent's claim) is the source of truth for pass/fail, Definition of Done is documented and enforced in CI where possible. Behaviour that leaves no artifact (no silent scope reduction, no fabricated success reports, no out-of-scope system changes) is **not** a compliance rule: it goes in the "Agent conduct" section of `AGENTS.md` and the adopter snippet, which bind any agent following the workflows but are not counted in audits |
| Architecture [ARC] | Modularity, clear interfaces, externalized config, statelessness where practical, ADRs for significant decisions (including stack choice: "best tool for the job" documented, not dictated), no unnecessary distribution |
| Code quality [QLT] | Strict linting and static analysis, enforced formatting, warnings as errors, no dead code, explicit error handling, no silent failure, and **check integrity** (§6.5) so "zero violations" cannot be reached by switching checks off |
| Language & runtime safety [SAF] | Enable every safety feature, strict mode and hardening flag the language/toolchain offers; memory safety; isolated and justified unsafe code; sanitizers where the language permits memory errors (fuzzing is TST's); integer and concurrency safety (integer safety comes through the language modules' safety checks; race detection applies when `uses-concurrency`) |
| Security [SEC] | Threat model, least privilege, secrets handling, standards-based authN/authZ, input validation and output encoding, parameterized queries, TLS minimums, crypto and FIPS 140-3 (when `requires-fips`), SAST and secret scanning, never fail open, OWASP ASVS level for web apps; secure-by-default per CISA Secure by Design (no default passwords, MFA available). **Vulnerability handling:** coordinated disclosure policy, `security.txt` (RFC 9116) for web services, published security advisories, VEX statements alongside the SBOM (does a known vulnerability actually affect this product), documented response-time targets; the advisory and VEX rules apply to every project that publishes releases |
| Supply chain [SUP] | Pin toolchains, CI actions by SHA (or a host lockfile such as GitHub's `actions.lock`) and third-party container images by digest (library and package dependencies: see DEP); no floating tags; SBOM (SPDX or CycloneDX) marking direct versus transitive dependencies; signing and provenance (SLSA), with SUP as the one home for signing release artifacts (the SBOM, signing and SLSA rules apply to every project that publishes releases); never modify system-level runtimes or package managers (installing toolchains per project is CTB's). **No end-of-life platforms:** no unsupported runtimes, base images or toolchains. **Source and CI protection:** branch protection, required review (when `has-multiple-maintainers`), CI tokens with least privilege, no dangerous workflow patterns (e.g. untrusted code running with secrets). OpenSSF Scorecard checks much of this automatically and has a vendor profile (§6.6) |
| Dependencies [DEP] | Default category Safety. **Bounded version ranges (MUST):** every dependency in a manifest is constrained to a bounded range: at least a "soft pin" within one major version (e.g. `>=1.2, <2`, written in the ecosystem's own syntax: Cargo `1.2`, npm `^1.2.0`, Python `~=1.2`, Maven `[1.2,2.0)`); never `*`, `latest`, a branch, or an unbounded `>=` (which admits the next major version, and with it breaking changes). Language modules name the syntax. **Exact, verified resolution (MUST):** a lockfile with integrity hashes is committed, and builds and CI install strictly from it, failing when it is out of date (e.g. `cargo build --locked`, `npm ci`, `uv sync --locked`). **Libraries:** SHOULD declare the widest range they are tested against (tight pins in a library cause conflicts for everyone downstream) and SHOULD test the lowest versions in that range, not only the newest. **Direct dependencies declared (MUST):** every package the code uses directly is declared as a direct dependency, never reached through another package's dependencies (such "phantom" dependencies break when that package changes); tools find these, e.g. deptry, knip, Maven `dependency:analyze`, `go mod tidy` (Rust's compiler already enforces it). **Direct dependencies documented (SHOULD):** each one has its purpose recorded (in the manifest or a dependency list), and adding one requires that justification. **No unused dependencies (SHOULD; category Maintainability).** **Runtime, build and dev dependencies kept separate (MUST)**, so shipped artifacts carry only runtime ones. **Healthy dependencies (SHOULD):** before adding one, check that it is maintained, not deprecated or archived, has more than one maintainer, and that the standard library is not enough, and record the check in the change that adds it, with a reason for any point it fails (licence compatibility is LIC's). **No end-of-life dependencies (MUST).** **Updates:** automated update PRs (e.g. Renovate, Dependabot), reviewed before merge; new releases SHOULD wait a minimum age (e.g. a few days) before adoption, because compromised releases are usually caught and pulled within days (e.g. Renovate `minimumReleaseAge`, Dependabot cooldown, uv `--exclude-newer`); releases that fix a known vulnerability affecting the project are exempt. **Vulnerability scanning** of all dependencies, with VEX for findings that do not affect the product (SEC). **Private packages:** names scoped or registries configured so a public package cannot take their place ("dependency confusion"). **Vendoring:** a project that vendors (fact `vendors-dependencies`) MUST record each vendored component's source, version and licence in a form scanners and update tools read, so it is scanned and updated like any other dependency. Tool-specific details (exact commands and configuration) live in language modules and vendor profiles |
| Testing [TST] | Unit, integration, end-to-end for user workflows, API, regression test per fixed bug, migration, accessibility (accessibility testing is ACC's); tests run in CI; no disabled tests without tracking (QLT's); coverage not a gate but critical and security logic tested. **Test effectiveness** (tests can be gamed too: a test that asserts nothing still passes): mutation testing on critical and security logic (e.g. Stryker, PIT, mutmut, cargo-mutants) to prove the tests catch real faults; property-based testing for logic with wide input ranges; fuzzing for anything that parses untrusted input. Performance testing lives in PRF |
| Accessibility [ACC] | WCAG 2.2 AA baseline, keyboard, screen readers, focus, contrast, no colour-only meaning, reduced motion, responsive, automated (e.g. axe-core) plus manual testing |
| APIs [API] | Machine-readable contracts (OpenAPI / AsyncAPI / protobuf), versioning, consistent errors (RFC 9457), pagination, idempotency |
| Data [DAT] | Versioned migrations, no automatic destructive schema changes, export/import portability (MAY rules, category Features), documented schemas, deletion, encryption at rest for sensitive data (backups are their own area, BAK) |
| Backup [BAK] | Category Reliability. Three scopes: **(1) what the software offers its operators** (when `stores-persistent-data` and `publishes-releases`): built-in, documented backup and restore commands, consistent (application-aware) backups, automatic backup before every upgrade so rollback is always possible (ties to DST's trivial upgrades), restore that works on a fresh install. **(2) Deployments the project runs itself** (when `operates-service`): documented RPO/RTO, automated scheduled backups, the 3-2-1 rule with one immutable or offline copy, encryption, backup credentials separate from production, alerting on failed backups, **restore tested on a schedule** (a backup never restored is not a backup), retention aligned with PRV deletion rules. **(3) The source itself**: the repository SHOULD be mirrored to a second location independent of its primary host. References: CIS Controls (data recovery), NIST SP 800-34; confirmed at writing time |
| Privacy [PRV] | Data minimization, PII inventory, retention limits, no sensitive data in logs or telemetry, opt-in telemetry *(Claude's addition)* |
| Observability [OBS] | Structured logs, configurable levels, health and readiness, metrics, tracing (OpenTelemetry), graceful shutdown |
| Resilience [RES] | Timeouts, retries with backoff, graceful degradation, fail safe, resource limits, recovery when dependencies return |
| Distribution & install [DST] | Install methods offered as MAY items (native packages, standalone binaries, OCI images, Compose, Helm, IDE plugins, MCP servers, agent skills), which apply only to released software (`publishes-releases`) of a kind that the method fits; for each offered: install, upgrade and uninstall docs; **trivial upgrades** (install over the top, detect existing installs, migrate config/data automatically, rollback); offline install; multi-arch; instructions to verify artifact signatures (signing itself is SUP's); published charts and manifests name the project's own images by digest |
| Deployment hardening [HRD] | Non-root, drop ALL capabilities and add only what's needed, read-only root FS, no privileged mode, no host network/mounts without justification, resource limits, health checks, seccomp/AppArmor defaults, secure-by-default Compose and Kubernetes config |
| Contributability [CTB] | How easy it is for someone new to contribute correctly. Own score category. **Always:** one-command dev environment setup (e.g. a Dev Container or equivalent) using project-local toolchains (pinning them is SUP's); the same checks runnable locally as in CI, with one command; pre-commit hooks that mirror CI; fast feedback (documented time for the main checks); a CONTRIBUTING.md that covers setup, workflow, how to run checks and what makes a change acceptable. **When `accepts-external-contributions`:** code of conduct (Contributor Covenant, referenced not rewritten); DCO sign-off preferred over a CLA; issue and PR templates; labelled starter issues; CI that runs on forks (keeping secrets from untrusted PRs is SUP's); a documented review process and response-time expectation; GOVERNANCE.md and MAINTAINERS for projects with more than one maintainer; contributor recognition. **Agents as contributors:** an AGENTS.md with the exact setup, build, test and check commands and the project's conventions, so coding agents contribute correctly. **Continuity:** at least two people with release and admin access; a documented succession or archival plan so the project is never stranded if a maintainer leaves. References: OpenSSF Best Practices badge, CHAOSS metrics, containers.dev, the AGENTS.md convention; confirmed at writing time |
| Documentation [DOC] | README, SECURITY.md (its content is SEC's), CHANGELOG.md (its format is REL's), ARCHITECTURE.md, configuration reference (naming each setting that turns a security feature off), troubleshooting, upgrade guide (DST's, per install method), dev environment setup (CTB's), a public API reference for libraries. Any set of steps for a human or agent to follow (install, upgrade, runbooks, AGENTS.md workflows) SHOULD be written in **ASD-STE100 Simplified Technical English**; category Accessibility. The rule references the ASD-STE100 specification and does not reproduce it (it is copyrighted by ASD and, to be confirmed at writing time, distributed free on request). Audits without the specification check STE's writing rules (one instruction per sentence, imperative, active voice, sentence-length limits); full dictionary conformance is `unverified` unless the auditor has the specification or an STE checker |
| Licensing [LIC] | Default category Compatibility. Declared license; SHOULD be OSI-approved FOSS, preferring permissive (e.g. MIT) (the preference is guidance in the rationale, not a scored rule); third-party compatibility; attribution |
| Release [REL] | SemVer; tagged and reproducible releases (signing artifacts is SUP's); support/EOL policy. Changelog MUST follow **Common Changelog** (a stricter take on Keep a Changelog; its exact requirements confirmed at writing time) rather than a USS-defined format; USS adds only what it leaves open, such as security fixes citing the advisory or CVE ID. **Breaking changes** (category Compatibility): SHOULD NOT be introduced; when unavoidable they MUST come with a MAJOR version, be flagged as breaking in the changelog (both come from SemVer and Common Changelog, which REL requires), and ship a migration guide; they SHOULD be preceded by a deprecation period with warnings, and SHOULD come with automated migration (ties to DST's trivial upgrades). **Release audit**: every release MUST be audited against the project's pinned USS version, and the generated report (compliance status and category scores) MUST be published with the release (§8.3) |
| Internationalization [I18N] | Externalized strings, Unicode throughout, locale-aware formatting, timestamps stored in UTC and exchanged as RFC 3339 *(Claude's addition; mostly SHOULD/MAY)* |
| Extensibility [EXT] | See §6.2 |
| Audit logging [AUD] | Category Safety. When `handles-authentication or has-multiple-users or operates-service`. Accountability trail of who did what, when, from where, to what, with what outcome. Which events are recorded (authentication success and failure, authorization failures, privilege and role changes, configuration changes, admin actions, access to sensitive data, access to the audit log itself); record contents; audit records kept separate from debug logs; tamper-evident, append-only storage; synchronized, UTC timestamps; restricted access to the trail, itself audited; configurable retention aligned with PRV; alerting when audit logging fails; export to SIEMs in a standard format; available at no extra charge (TRU). References: NIST SP 800-53 Rev. 5 AU (Audit and Accountability) family, the DoD/DISA Application Security and Development STIG, OWASP ASVS logging requirements, NIST SP 800-92, OCSF or syslog (RFC 5424) for formats; confirmed at writing time. *Naming:* in USS, "audit" alone means a compliance audit against USS; this area is always "audit logging" |
| AI features [AI] | Category Safety (evaluation rules: Reliability). When `uses-ai-models` or `has-ai-tool-use` (which `exposes-mcp-server` implies). Defences against prompt injection; model output treated as untrusted input; least-privilege tool access for models and agents; a human confirms destructive or irreversible actions; pinned model versions; evaluation suites run like tests, with regressions failing the build; cost, rate and token limits; no sensitive data sent to models without a basis for it (PRV); users told when they are interacting with AI; MCP server security (authentication, scoped tools, input validation). References: OWASP Top 10 for LLM Applications, the MCP specification's security guidance, NIST AI RMF, EU AI Act transparency obligations; confirmed at writing time |
| Performance & efficiency [PRF] | Category Efficiency. Written performance budgets (e.g. startup time, latency, memory, binary or bundle size, Core Web Vitals for web UIs); benchmarks in CI that fail the build on regression beyond a stated threshold; load testing for operated services; efficient resource use (no busy-waiting, bounded memory, idle at near-zero cost); energy and carbon measured with the Software Carbon Intensity specification (ISO/IEC 21031) where practical. References confirmed at writing time |
| Usability [UX] | Category Accessibility & Usability. Error messages say what went wrong and how to fix it; destructive actions ask for confirmation or can be undone; sensible defaults with zero-config start where possible; no deceptive design ("dark patterns"; counts toward Fairness). **When `has-cli`:** Command Line Interface Guidelines (clig.dev), `--help` and `--version`, meaningful exit codes, machine-readable output option, `NO_COLOR` honoured, stdout for output and stderr for messages. **Config and data locations** follow platform conventions (XDG Base Directory on Linux, the platform equivalents elsewhere) |
| Operations [OPS] | Category Reliability. When `operates-service`. Service-level objectives (SLOs) with error budgets; alerts on symptoms users feel, not on causes; a runbook (in ASD-STE100) for every alert; a documented incident-response process; blameless postmortems with tracked follow-ups; progressive rollouts (canary or staged) with automatic rollback; status communication to users during incidents. References: Google SRE practices; confirmed at writing time |
| Fair dealing [TRU] | Promises to users kept across versions. Default category Fairness; individual rules override. Features once available free SHOULD NOT be taken out of the free edition or moved behind a payment while a paid edition keeps them (no "rug-pulls"), e.g. LDAP or OIDC login that was free must stay free. Removing a feature from every edition is a breaking change and belongs to REL; an operated service with no releases that removes a feature from every edition is judged by no rule; future versions SHOULD NOT be relicensed to more restrictive terms. **When `is-commercial`:** security essentials (SSO via OIDC/SAML/LDAP, MFA, audit logging) SHOULD NOT be paywalled at all (category Safety; ties to CISA Secure by Design); users' own data MUST NOT be held hostage: users MUST be able to export it in an open, documented format, and export works regardless of subscription state; pricing and licence changes are announced ahead of time in the changelog. References: CISA Secure by Design; confirmed at writing time |
| Commercial licensing [COM] | Category Fairness. When `is-commercial` (any edition, tier, feature or licence is sold). **Transparent:** licence models, tiers and prices SHOULD be published, with what each includes, so the full cost can be worked out in advance (no hidden fees, and every offering has a published list price rather than being "contact sales" only). Negotiated volume and corporate-wide agreements MAY be offered on top of the published prices, provided they follow every other COM rule (price-increase caps, notice, renewal-only increases, no virtualisation penalties, usage visibility, grace periods) and are never worse than the published price for the same thing. **À la carte:** each feature SHOULD be purchasable on its own; bundles MAY also be offered, but never priced so that buying à la carte is a penalty. **Not predatory:** every agreement SHOULD state a cap on price increases; increases SHOULD NOT exceed a named inflation index (e.g. CPI) plus 3 percentage points per year, apply only at renewal (never mid-term), and be announced at least 90 days ahead; licence metrics SHOULD NOT penalise virtualisation or containers (e.g. counting physical cores of a whole host); licence usage SHOULD be visible to the customer in the product, so there is never a surprise "true-up" audit. **Licence delivery types:** an offline licence file for air-gapped use SHOULD always be offered; a licence server MAY be offered, and if so SHOULD be self-hostable (IND); a lost connection to any licence service SHOULD start a generous grace period, never an immediate stop; an expired licence SHOULD degrade gracefully (e.g. read-only), and data export always works (TRU). **Licence models:** SHOULD offer several, such as named user, concurrent (floating), node-locked, subscription, perpetual (with optional paid maintenance, or a perpetual-fallback licence that keeps the last version paid for), and a **service-account licence for CI/CD**: for automation only, does not consume a human seat, works on ephemeral and containerised runners (no node lock or hardware fingerprint), activates without interaction from a secret token, and is priced for automation use |
| Independence [IND] | Users can run the software on their own terms. Category Compatibility. **When `is-network-service`:** SHOULD be self-hostable: runs entirely on the user's own infrastructure, with a documented self-hosting guide, no required phone-home or licence server, and the self-hosted edition not cut down relative to any hosted one; SHOULD run in restricted-egress and air-gapped environments (DST covers installing offline; this covers running offline). **When `is-foss`:** SHOULD NOT require any commercial product or subscription to work: proprietary services may be offered as optional integrations, but every required dependency has a FOSS option (e.g. an AI feature that calls a hosted model API can also point at a locally run model). **When `uses-network`:** SHOULD NOT require one cloud provider's proprietary services; use open or widely implemented protocols instead (e.g. S3-compatible storage, OIDC, OpenTelemetry). **When `uses-relational-database`:** SHOULD support SQLite, PostgreSQL and MariaDB/MySQL rather than one database only, with the full test suite running against every supported database in CI (support that CI does not prove is only a claim); database access SHOULD go through an abstraction layer (an ORM, a query builder, or repository interfaces with per-database adapters, as the language module recommends) and migrations SHOULD be written or generated per supported database. Projects that depend on one database's distinctive features (e.g. PostGIS, PostgreSQL JSONB operators, LISTEN/NOTIFY) record a Standard Deviation |

### 6.2 Extensibility: plugin interface and marketplace

- `USS-EXT-001` **MAY** provide a plugin interface (enables
  `has-plugin-interface`). If adopted, rules define the ideal interface:
  versioned and documented contract with SemVer compatibility guarantees;
  plugin manifest (identity, version, compatible host range, requested
  permissions, license); least-privilege permission model; isolation so a
  failing plugin cannot crash or compromise the host; lifecycle (install,
  enable, disable, upgrade, uninstall) with clean removal; SDK, example plugin
  and test harness.
- A further **MAY** provides a plugin marketplace (enables
  `has-plugin-marketplace`, requires `has-plugin-interface`). If adopted:
  signed packages and verified publisher identity; provenance; automated
  scanning and a review process; versioned index with dependency resolution;
  update channel; revocation / kill switch; licence metadata; search and
  discovery; self-hostable / mirrorable for offline environments.

### 6.3 v1 modules

Under `standard/modules/<category>/`:

- languages: Python (`PY`), Go (`GO`), Rust (`RUST`), TypeScript and
  JavaScript (`TS`), Java (`JAVA`)
- tools: Maven (`MVN`)
- platforms: Docker Compose (`COMPOSE`), Kubernetes and Helm (`K8S`),
  PostgreSQL (`PG`)
- frameworks: React (`REACT`)
- regulations: EU Cyber Resilience Act (`CRA`), European Accessibility Act
  (`EAA`), GDPR (`GDPR`) (see §6.7)

Each module is one file, `standard/modules/<category>/<name>.md`, with the
prefix in brackets.

Each module states the concrete flags, linters, checker configs and patterns
that satisfy the core rules for that technology (e.g. which compiler flags
satisfy SAF rules for Rust, which linters satisfy QLT for Python), plus rules
that exist only because of that technology's quirks.

Module rules are **binding**, and each one that makes a core rule concrete
names it in `Refines` and takes the strength of the core rules it refines (a
module never weakens a core rule or silently raises one). The required
**outcome** is a MUST ("MUST lint with zero violations against a rule set
covering correctness, style, imports and security"). The **best-in-class tool**
for that outcome, chosen from current sources when the module is written, is a
separate SHOULD ("SHOULD use Ruff, configured per its vendor profile"); this
tool rule refines the core rules whose outcome it serves and is exempt from
the strength rule. New projects simply get the recommended tool. An existing
project on another tool meets the MUST through that tool, configured to at
least the baseline that QLT-006 sets (the tool's vendor profile, §6.6, when it
has one), and records a Standard Deviation for the SHOULD.

A module states the technology's own flags and settings (the compiler, the
standard formatter, the package manager and, for Maven, Compose and Kubernetes,
the configuration of the technology itself); the configuration of a separate
checking tool whose rule set sets a floor (a linter, a type checker, a scanner
with a rule set) lives in that tool's vendor profile. A formatter or a scanner
without a configurable rule set (such as Prettier or `govulncheck`) is named by
its module rule and has no profile. Where one tool does not cover an outcome,
the module pairs tools: Ruff covers linting, formatting and Bandit-style
security checks but not type checking, so the Python module also names mypy as
the strict type checker.

A project in a language with no module
yet is audited against the core rules directly, using that toolchain's own
documentation, and the audit report lists the stack entry as "no module";
`uss check` does not reject it.

A Dockerfile module (with hadolint) is a candidate for a later release. In v1,
USS-HRD-001 covers container images through the CIS Docker Benchmark, and the
Compose module covers only Compose files.

### 6.4 Rule-writing bar

Every rule: one requirement; verifiable from artifacts; language-neutral in
core; has a rationale where not self-evident. References are cited from current
sources, checked at writing time (not from memory), with versions pinned
(WCAG 2.2, ASVS 5.0, SLSA 1.x, etc.); each release reviews whether newer
versions of referenced standards should be adopted.

**Reference, don't reinvent.** Where an established standard or specification
already defines something (SemVer, Keep a Changelog, RFC 9457, OpenAPI, WCAG,
CycloneDX / SPDX, OWASP ASVS, SLSA, CIS Benchmarks, OpenTelemetry, NIST SSDF,
the OpenSSF Compiler Hardening Guide, and so on), the USS rule requires
conformance to it and cites it, and never restates its content. USS adds only
what nothing else covers, plus the glue: which standard, which level, and when
it applies. The same goes for tools: when a widely adopted tool already encodes
best practice, modules point to it (§6.3).

**Choosing a level.**

- **MUST** when the absence is a defect in every project the rule applies to,
  and any team of any size can meet it (safety flags, input validation, secrets
  handling, pinned dependencies, tests in CI, a licence file).
- **SHOULD** when it is valuable but legitimate reasons to skip it exist (an
  SBOM for an internal tool, signed artifacts, ARM64 builds, offline install).
- **MAY** for optional capabilities; they count toward scores (§5.5).
- Small projects stay light through applicability, never by weakening a level.
  Budget: a project whose facts are all false sees at most a quarter of the
  core rules by the 0.1.0 release. A `uss` test pins the current count, and
  each plan's handover reports it.
- External standards' own levels scale with facts, not tiers. OWASP ASVS:
  Level 1 MUST when `has-web-ui or exposes-web-api` (ASVS covers web
  applications and services, so other network servers are out of its scope);
  Level 2 MUST when `handles-authentication or handles-sensitive-data`; Level 3
  SHOULD when `high-assurance`. SLSA build levels and CIS profiles follow the
  same pattern.
- Any rule that sets a "zero violations" or "all passing" target must be
  paired with check-integrity rules (§6.5); otherwise it can be met by
  switching the check off.

### 6.5 Check integrity

"Lint with zero violations", "zero compiler warnings", "all tests pass" and
"no scanner findings" can all be met by disabling the check. Core rules (QLT,
with CON) close that off for **every** kind of check: linters, compilers, type
checkers, security scanners, tests and coverage.

- **A floor.** Each vendor profile (§6.6) defines the baseline configuration
  for its tool (e.g. the Ruff rule groups that must be selected). A project
  MUST run checks at least as strict as the baseline; it may add rules, and it
  drops a rule of the baseline only with a written reason (next point).
- **Disabling a baseline rule project-wide** MUST have a reason written next to
  it in the configuration. Disabling one in a security-related group (e.g.
  Ruff's Bandit rules, gosec) is a SHOULD NOT and needs a Standard Deviation.
- **Inline suppressions** (`# noqa`, `//nolint`, `#[allow]`, `eslint-disable`,
  `@SuppressWarnings`, `# type: ignore`, `@ts-ignore`) MUST name the specific
  rule and give a reason on the same line. Blanket suppressions (all rules, or
  a whole file) MUST NOT be used. Where the language offers a form that fails
  when the problem goes away (`#[expect]` in Rust, `@ts-expect-error` in
  TypeScript), it SHOULD be used.
- **Stale suppressions** (ones that no longer suppress anything) MUST be
  reported as errors.
- **Skipped or disabled tests** MUST name a tracked issue. **Coverage
  exclusions** MUST give a reason.
- **Enforced by the tools themselves where possible**, per "reference, don't
  reinvent": modules turn on each tool's own guards, such as Ruff's
  blanket-`noqa` and unused-`noqa` rules, golangci-lint's `nolintlint`,
  ESLint's unused-disable-directive reporting and comment-description rules,
  and Clippy's `allow_attributes_without_reason`. Exact rule names are
  confirmed at writing time.
- **Audits list every suppression** with its rule and reason as evidence, so a
  reviewer can see what has been switched off, not just that the check is
  green.

### 6.6 Vendor profiles

Tool vendors (companies or open-source projects, e.g. Astral for Ruff, the
Rust project for Clippy) can add their own sections to this repository stating
exactly how their tool satisfies USS rules. USS names the best tools; vendors
define how those tools meet USS. Each side references the other, and neither
restates the other's content.

- **Location:** `standard/vendors/<vendor>/<product>.md`, contributed by pull
  request, owned via CODEOWNERS by the vendor's maintainers. Each profile is
  labelled **vendor-maintained** or **community-maintained** (written by USS
  contributors until the vendor takes it over).
- **Contents:**
  - Product and the tool version range the profile is valid for (tool rule
    sets change between versions).
  - The **baseline configuration**, as a copy-ready config fragment (e.g. the
    `[tool.ruff]` block), which is the floor in §6.5.
  - **Mappings, grouped by USS score category** (Safety, Compatibility,
    Accessibility, …): for each USS rule the tool satisfies, the rule ID, the
    tool rules or settings that satisfy it, and how an auditor verifies it.
    Example: Safety → `USS-SEC-0xx` ← Ruff `S` (Bandit) rules; Compatibility →
    `USS-…` ← Ruff `UP` rules with `target-version` set.
  - The tool's **check-integrity guards** (§6.5), which a profile MUST include
    when the tool has them.
- **Completeness:** a mapping claims a rule only if the configuration fully
  satisfies it; partial coverage is marked partial and names what else is
  needed. For every category the profile lists, it covers every applicable USS
  MUST that falls within what the tool can check; it cannot claim a category
  and leave such a MUST out.
- **Tool-specific rules:** a profile MAY add rules that exist only because of
  the tool (e.g. "MUST NOT use file-level `ruff: noqa`"), with a registered
  prefix (`USS-RUFF-NNN`), in the normal rule format.
- **Effect on audits:** when a project uses a tool with its profile's baseline,
  the mapped rules can be marked `pass` with the profile and the verified
  config as evidence. Audits get faster and more consistent the more profiles
  exist.
- **Review:** USS maintainers review every profile (and every change to one)
  for accurate claims before merging; the validator checks its format and that
  every mapped ID exists. Profiles ship with USS releases (adding one is a
  MINOR release).
- **v1:** seventeen community-maintained profiles, for the tools that the v1
  modules recommend and that have a rule set: Ruff, mypy, deptry, OSV-Scanner,
  golangci-lint, Clippy, cargo-deny, ESLint with typescript-eslint, knip, Error
  Prone, NullAway, SpotBugs with Find Security Bugs, kube-linter, Squawk,
  eslint-plugin-react-hooks, Oxlint (for its jsx-a11y rules;
  eslint-plugin-jsx-a11y was replaced because it no longer met the maintenance
  test and does not support ESLint 10) and OpenSSF Scorecard. Scorecard checks
  supply-chain, security, dependency, testing and licence rules automatically;
  it checks no Contributability rule. No CODEOWNERS entry exists yet: USS
  maintainers own the community profiles, and a vendor that takes one over adds
  its own entry.

### 6.7 Regulation modules

Laws and regulated-industry frameworks are modules under
`standard/modules/regulations/`, switched on by facts (e.g. `sold-in-eu`,
`processes-eu-personal-data`). Like vendor profiles, each one **maps** its
obligations to existing USS rule IDs, and adds rules only for obligations USS
does not already cover (e.g. the CRA's duty to report actively exploited
vulnerabilities to the authorities within set deadlines, or its duty to draw
up an SBOM, which core makes only a SHOULD). Each states the regulation's
version and the dates its obligations apply, all confirmed from the official
text at writing time.

Every regulation module carries a plain disclaimer: USS compliance is not legal
compliance and not legal advice. A regulation module shows where USS helps
meet the law and what is left over, for a qualified person to confirm.

v1: EU Cyber Resilience Act (`sold-in-eu`; as Article 71(2) states, Chapter IV,
Articles 35 to 51 on the notification of conformity assessment bodies, applies
from 11 June 2026, Article 14 on reporting from 11 September 2026, and the rest
from 11 December 2027), European Accessibility Act (switched on by the
`covered-by-eaa` fact; it entered into force in 2019 and applies from 28 June
2025), and GDPR (switched on by
`processes-eu-personal-data`). Later: PCI DSS, HIPAA, US federal (NIST SP
800-53, FedRAMP, DISA STIGs, secure-development attestation).

## 7. The project record file

Lives in the adopting project at `.uss/record.yaml`, validated by
`schema/record.schema.json`. It holds the **inputs** to compliance; audit
**results** live in reports.

```yaml
standard:
  version: 1.0.0
project:
  name: example-service
stack:
  languages: [go, typescript]
  frameworks: [react]
  platforms: [postgresql, docker-compose]
  tools: []
facts:
  has-web-ui: true
  stores-user-data: true
  requires-fips: false
decisions:                 # one per applicable MAY
  USS-EXT-001: { adopt: false }
  USS-DST-004: { adopt: true }
deviations:
  - rule: USS-SUP-009
    document: docs/standard-deviations/USS-SUP-009.md
    approved-by: "Jane Doe, CTO"
    date: 2026-11-21
    review-by: 2027-05-21
audits:
  - date: 2026-11-21
    results: .uss/audits/2026-11-21.yaml
    report: .uss/audits/2026-11-21.md
```

**Repositories with several components.** One record per independently
released unit. A repository's root `.uss/record.yaml` covers repository-wide
rules (CI, supply chain, SECURITY.md, licence) and MAY list child records
(`components: [libs/core, cli, web]`), each with its own `.uss/record.yaml`,
facts, stack and deviations. The repository is compliant when its root record
and every listed component record are compliant.

Audit results are recorded as data in `.uss/audits/<date>.yaml` (per rule ID:
result and evidence such as file:line, config or CI job), validated by
`schema/audit-results.schema.json`. The readable report `.uss/audits/<date>.md`
is generated from it by `uss report` (layout from `templates/audit-report.md`):
compliance statement and category scores first, then every applicable rule
with result and evidence, MUST failures first. Agents never compute scores by
hand.

## 8. Agent workflows

Written once in this repo's `AGENTS.md` (works with any coding agent) and
packaged as Claude skills (§9). The skills follow the AGENTS.md text, never a
separate copy of it. The workflows read the standard and the workflows
themselves only through `uss show` and `uss applicable --full` (§9), at the
version the record pins. The agent installs `uss` before it starts: it reads
the standard through `uss`, and scores are never computed by hand.

**Questions come in batches.** The facts come in one table: each fact with its
question, an answer filled in from the scan, and the evidence for it; the
developer confirms or corrects it in one reply. SHOULD and SHOULD NOT rules
come one area at a time, with "comply" as the recommended answer, and the
developer can accept a whole area at once. MAY rules also come one area at a
time, each with its score category, answered in one reply per area.

### 8.1 Adopt (new project)

1. Load the standard at the version being adopted, through `uss`.
2. **Profile.** Ask the applicability questions, inferring answers from the
   project description where possible and confirming them. Stack facts come
   from the chosen languages, frameworks, platforms and tools.
3. Compute the applicable rules.
4. **Decide SHOULD / SHOULD NOT items** grouped by area, each group with the
   recommended answer (comply). The developer can accept a group at once or go
   item by item. Each declined item gets a Standard Deviation drafted from the
   developer's own reasoning.
5. **Decide MAY items** individually (they are feature choices), still grouped
   by area, saying which score category each one counts toward. Adopted MAYs
   switch on their conditional rules, which go back through step 4.
6. Write `.uss/record.yaml`, the deviation documents, and a snippet in the
   project's AGENTS.md / CLAUDE.md (from `templates/AGENTS.snippet.md`) so that
   every future agent session works to the applicable rules.
7. Build. MUSTs are not negotiable without a Standard Deviation, and the
   developer is told that a MUST deviation means "not compliant".

### 8.2 Audit (existing codebase)

1. Load the standard at the version in the record (or the latest, if there is
   no record yet).
2. **Quick scan.** Detect languages, frameworks, build tools, CI, containers,
   UI, databases, auth, existing docs; note obvious findings.
3. **Profile with pre-filled answers** from the scan; developer confirms.
4. **Ask the SHOULD / SHOULD NOT / MAY questions as in 8.1**, informed by the
   scan ("there's no SBOM generation today; USS-SUP-0xx says SHOULD — comply or
   deviate?").
5. **Full audit** of every applicable rule: result plus evidence. Anything the
   agent cannot determine is `unverified`, never guessed.
6. Write the record and the audit results, and generate the report with its
   compliance statement and scores. A project whose AGENTS.md / CLAUDE.md has
   no USS snippet gets it, as in 8.1, and is offered the CI template when it
   publishes releases on GitHub.
7. **Remediate**, with the developer's go-ahead: MUST failures first, then the
   SHOULDs the developer chose to comply with; in batches by area; each fix
   verified before its result changes to `pass`; report updated as it goes.

**Before the first release.** Every new project meets this: a rule whose Verify
judges each release passes with the evidence "no release yet"; a rule that
needs a published artifact (binaries, an SBOM, signatures, provenance) fails; a
document about a release waits for the release; a document that does not depend
on a release is examined now.

### 8.3 Audits at any time, and at every release

An audit can run at any point in a project's life; it is the same workflow
each time. With an existing record and no rules added since, the agent asks
nothing and goes straight to the full audit. Every audit results file records
the commit it audited.

At release, an audit is required (REL). USS supports two ways to run it:

- **Locally:** run the audit skill before tagging, commit the results.
- **In CI:** run the agent audit headless (e.g. a coding agent in a release
  workflow). With no human present the agent asks nothing and changes no
  fact, decision or deviation. Each open question becomes `unverified` on the
  rules whose result depends on it, or on USS-REL-011 when none does, and the
  report says "not compliant" until someone answers it. When the record must
  change, the agent stops. Nothing is guessed.

Either way, the release CI job (template in `templates/ci/`) runs
`uss release-check`: the audit results must be for the release commit (or for
a commit whose only later changes are under `.uss/`), the report is
regenerated, and the report is attached to the release.

### 8.4 Upgrading a project's pinned USS version

Same as audit, but questions cover only rules added or changed between the two
versions (from the changelog), and the record's version is bumped once the
project is compliant with the new one (or, for a project that was not compliant
before, once the developer accepts the new report).

### 8.5 Projects with several released units

A repository can hold units that are released separately. Each unit has its own
record at `<unit>/.uss/record.yaml` (components, conventions §11), all pinning
the same version. The root record lists the units in `components` and covers
the rules for the whole repository: CI, the supply chain, `SECURITY.md` and the
licence. The repository is compliant when the root record and every unit's
record are. `uss check` also checks the units' records, and `uss release-check`
examines the latest audit of every record.

## 9. Repository layout and tooling

```
Universal-Software-Standard/
├── README.md                  what USS is; how to adopt; how to audit
├── AGENTS.md                  agent entry point: workflows in §8, agent
│                              conduct, contributor section
├── CLAUDE.md                  imports AGENTS.md
├── LICENSE                    MIT
├── CHANGELOG.md               per-release added / changed / retired IDs
├── CONTRIBUTING.md            how to add rules, areas, modules; ID policy
├── CODE_OF_CONDUCT.md
├── GOVERNANCE.md              who decides; succession
├── SECURITY.md
├── standard/
│   ├── conventions.md         keywords, rule format, IDs, prefixes registry,
│   │                          compliance, deviations, versioning
│   ├── applicability.md       the fact vocabulary and questions
│   ├── core/<area>.md         one file per area in §6.1
│   ├── modules/<category>/<name>.md
│   └── vendors/<vendor>/<product>.md   vendor profiles (§6.6)
├── schema/
│   ├── record.schema.json
│   └── audit-results.schema.json
├── templates/
│   ├── record.yaml
│   ├── standard-deviation.md
│   ├── audit-report.md.j2     report template used by `uss report`
│   ├── AGENTS.snippet.md      names no version; points to the record
│   └── ci/uss-release.yml     release-check workflow for adopters (§8.3)
├── skills/                    Claude skills: adopt, audit
├── .claude-plugin/            plugin + marketplace manifests
├── docs/                      cli, architecture, dependencies, troubleshooting,
│                              adr/
├── Cargo.toml, Cargo.lock     Rust workspace (lock file committed)
├── rust-toolchain.toml        pinned Rust toolchain
├── deny.toml                  cargo-deny policy (advisories, licences, sources)
├── tools/uss/                 validator: Rust library + `uss` CLI
├── tools/xtask/               `cargo xtask check`: every check in one command
├── .uss/                      this repo's own record and audits
├── .githooks/pre-commit       runs `cargo fmt --all --check` and `uss lint`
└── .github/                   CODEOWNERS (vendor profile ownership),
                               dependabot.yml, issue and PR templates,
                               workflows/ (CI)
```

**Validator `uss`**, in **Rust** (the user's call, made for a future VS Code
extension, in line with the plugin install methods in the original idea). It
is a **library plus a thin CLI**. The library holds all the logic (parsing,
linting, applicability, compliance, scores, report rendering) and does no
filesystem, network or process I/O: it works on file contents handed to it.
The CLI does the I/O (reading files, calling `git`). That keeps the library
able to run inside a VS Code extension or a web page through WebAssembly, or
to be embedded elsewhere, without a rewrite. The standard's files, `AGENTS.md`
and `CHANGELOG.md` are embedded in the binary, so a binary's version is always
the version of the standard and of the workflows.
Distribution: release binaries per OS, plus
`cargo install --git <repo> --tag vX.Y.Z uss` for people with Rust. Commands:

- `uss lint` — checks the standard: header format, valid levels, unique IDs,
  registered prefixes, applicability expressions reference defined facts,
  `Enables` only on MAY rules, retired rules carry replacement info, every rule
  ID named in the text exists, a rule applies only where each rule it refines
  applies, and the formats of vendor profiles and regulation modules.
- `uss check <record>` — validates a project record against the schema and the
  pinned standard version: known facts, every deviation names a real rule and
  an existing document, every applicable MAY has a decision.
- `uss applicable <record>` — prints the applicable rules for a record, so
  agents compute applicability deterministically instead of by reading;
  `--full` prints the full text of each one.
- `uss show [PATH]` — prints an embedded file (the standard, schemas,
  templates, `AGENTS.md`, `CHANGELOG.md`), or lists them. This is how an agent
  reads the standard at the version the record pins.
- `uss report <results>` — validates audit results, computes the compliance
  status and category scores, and writes the Markdown report.
- `uss release-check` — for release CI: confirms the audit results match the
  release commit (§8.3) and regenerates the report to attach to the release.
- `uss export --json` — the rule catalog as JSON, for future tools (MCP, etc.).

The tool is optional for adopters (agents can work from the Markdown), required
in this repo's CI.

**Plugin and skills.** The plugin is `uss`, at the repository root, in the
marketplace `universal-software-standard` (`.claude-plugin/plugin.json` and
`marketplace.json`, source `./`). It has two skills, `skills/adopt/` and
`skills/audit/`, which Claude Code names `/uss:adopt` and `/uss:audit` (the
plugin namespace is added to the skill names). The audit skill also takes
`release`, `upgrade <version>` and `headless`. Each skill tells Claude to
follow the workflow in `${CLAUDE_PLUGIN_ROOT}/AGENTS.md` and copies none of
it. A marketplace from a local path loads the plugin in place, so a trial
needs no release.

**Dogfooding.** This repo complies with its own standard: MIT LICENSE,
SECURITY.md, pinned Rust toolchain, CI actions pinned by SHA, strict Clippy
linting (with `allow_attributes_without_reason` for check integrity) and
cargo-deny for the tool, an ADR recording the choice of Rust, its own
`.uss/record.yaml` and audit report, a Common Changelog CHANGELOG.md, and the step-by-step parts of AGENTS.md and the skills written in
ASD-STE100. It is the first audit target and the reference example.

## 10. Constraints

- RFC 2119 + RFC 8174.
- Hosted on GitHub under HaleTechCo; CI on GitHub Actions.
- Repository: `HaleTechCo/Universal-Software-Standard` on GitHub, private for
  now, default branch `main`.
- Development machine is Windows; everything must also work on Linux and macOS.
- `Initial_Idea.md` and `todo.txt` are source material and stay untouched; the
  user decides what happens to them.

## 11. Later (not v1)

- MCP server serving rules and records.
- More modules (C/C++, C#/.NET, Kotlin, Swift, Gradle, Vue/Angular, CLI
  conventions, mobile, etc.).
- Dashboards over audit results across many projects.
- Regulation modules for PCI DSS, HIPAA and US federal requirements.
- VS Code extension (then other IDE plugins) using the `uss` library, through
  WebAssembly or a bundled binary: inline rule diagnostics in records, results
  and standard files.
- Cloud cost management, export controls on cryptography.
- Modules for mobile, embedded and games.
- Compliance badge.

## 12. Left to the builder

- Exact wording and full set of rules within each area and module, within the
  rule-writing bar in §6.4 and the area scopes in §6.1.
- The exact fact vocabulary beyond the examples.
- Validator internals, Markdown parsing approach, release mechanism for the
  binary.
- Claude plugin / skill packaging details. (Settled: a skill obtains the
  pinned standard version by installing the `uss` release of the record's
  version and reading the standard and the workflows through it, with
  `uss show` and `uss applicable --full`.)
- CI job layout.

## 13. Delivery order and review points

1. Repository skeleton, conventions, applicability vocabulary, schemas,
   templates, and the `uss` validator, with this repo's own CI.
2. Draft the **Security**, **Supply chain** and **Dependencies** areas. **Stop for the user's
   review**: this is where levelling and tone get calibrated.
3. Draft the remaining core areas, then the modules with their
   community-maintained vendor profiles, in batches.
4. AGENTS.md workflows, the Claude skills, and this repo's self-audit. Done.
5. Release **0.1.0** for the user's review. It waits for the user's trial of
   the plugin on one of their own repositories and for the release plan.
   1.0.0 comes when the user declares it.
