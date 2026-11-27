# USS follow-ups: open questions about the standard, and the rulings to confirm

Scope: `docs/superpowers/follow-ups.md` section 2.5 (R1 to R10, the three
"other readings", the collapsed Plan 3 and Plan 4 rulings), section 3, section
4.7, section 5, and USS-DEP-006 from section 2.3. Repository state: `main` at
42c3280; audit `.uss/audits/2026-11-25.yaml` of commit 51435cb. All web and
API sources were read on 2026-11-25. Repository links are relative to this
file. Nothing in the repository was changed except this file; experiments ran
in a scratch copy outside the repository.

Note on editing rules: conventions section 4 says a change of meaning needs a
new rule ID ([conventions.md:183](../../../../../standard/conventions.md)).
USS 0.1.0 is not released, and earlier plans changed rules in place (such as
the TRU-004 level in Plan 3). The wording changes below assume an in-place
edit before 0.1.0. After a release, each one would retire the rule and add a
new ID.

## Section 3: `stores-persistent-data` (does it read true for a tool that only writes report files?)

### Takeaway
**Change: narrow the definition.** As written ("data it writes is still there
after it stops and starts again"), the fact is true for any program that writes
a file, including every compiler, formatter and report generator. For USS it
adds 10 rules that make no sense for a tool that only writes reports. Narrow it
to data that the software needs again after a restart, and exclude output that
it only hands over.

### Cited Findings
- Current text: Question "Does it keep data across restarts?"; Definition "Yes
  when data it writes is still there after it stops and starts again." —
  [standard/applicability.md:36-39](../../../../../standard/applicability.md)
- `stores-user-data` and `uses-database` both imply it — [standard/applicability.md:45, :51](../../../../../standard/applicability.md)
- Rules gated by the fact: BAK-001 to BAK-005 and BAK-014 (with
  `publishes-releases`), BAK-006 to BAK-013 (with `operates-service`), the
  whole DAT area (area-level `Applies when`), I18N-001 (as one of four
  alternatives), I18N-004, PRV-003, RES-006, TST-007, PG-001 to PG-003, and
  CRA-009 — `grep -rn stores-persistent-data standard` at 42c3280;
  [standard/core/data.md:5](../../../../../standard/core/data.md)
- Experiment: copied `.uss/record.yaml` to a scratch directory (with
  `audits: []`), set `stores-persistent-data: true`, and ran
  `cargo run --locked -q -p uss -- applicable <copy>`. False: 145 rules. True:
  155 rules. The 10 extra rules: USS-BAK-001 (MUST), USS-BAK-002 (MUST),
  USS-BAK-003 (SHOULD), USS-BAK-004 (MUST), USS-DAT-001 (MUST), USS-DAT-002
  (MUST NOT), USS-DAT-003 (SHOULD), USS-I18N-004 (MUST), USS-RES-006 (MUST),
  USS-TST-007 (MUST). — local run, 2026-11-25
- What `uss` writes: only the report (`std::fs::write(&target, report)`) and
  the combined report of `release-check` (`std::fs::write(out_path, …)`); it
  reads records, results and standard files that people or agents write —
  [tools/uss/src/cli/mod.rs:401, :543](../../../../../tools/uss/src/cli/mod.rs)
- The extra rules assume stored state: BAK-004 needs a CI job that restores a
  backup into a fresh install; DAT-001 needs versioned migrations that run on
  upgrade; TST-007 needs a test per migration — [standard/core/backup.md](../../../../../standard/core/backup.md);
  [standard/core/data.md](../../../../../standard/core/data.md);
  [standard/core/testing.md](../../../../../standard/core/testing.md)
- Precedent for exclusions inside a definition: `parses-untrusted-input`
  ("Configuration that only its operator writes does not count"),
  `exposes-web-api` (health endpoints and loopback listeners do not count),
  `uses-concurrency` (single-thread async does not count) —
  [standard/applicability.md:68, :116, :121](../../../../../standard/applicability.md)

### Inferences
- **Reason.** Every gated rule protects data that someone would lose: backups,
  migrations, atomic writes, UTC timestamps in stored records. A report that
  `uss report` can write again from the results file is not such data. The
  literal definition makes the fact true for nearly all software, so it no
  longer selects anything.
- **Exact change** (`standard/applicability.md`):

  ```markdown
  ### stores-persistent-data

  - **Question:** Does it keep data that it needs again after a restart?
  - **Definition:** Yes when it writes data that it, or a later version of it, reads back after it stops and starts again, as its own state or as its users' data, such as a database, a data directory, uploaded files, keys, or saved settings. Output that it only hands to people or to other programs, such as reports, logs, build outputs and exported files, does not count, and neither does a cache that it can rebuild.
  ```

  Also change the comment in `templates/record.yaml:48` and the record of this
  repository to the new question. USS keeps `false`.
- **Effect on other projects.** Compilers, linters, formatters, code
  generators, converters and report tools become `false` and lose about 10
  rules that they could only fail or pass emptily. A CLI that saves settings
  or tokens, a desktop app that saves documents, and every service with a
  database stay `true`; CRA-009 ("remove all their data and settings") still
  applies to software that saves settings. Logs stay covered by PRV, OBS and
  AUD, which have their own facts.
- **Alternatives.**
  - Answer `true` for USS: 10 extra rules (7 MUST or MUST NOT) that `uss`
    could not meet in any useful way, such as a CI restore test for reports.
  - Keep the text and add guidance only in AGENTS.md: the fact text stays
    wrong for every other reader of `applicability.md`.
  - Split into two facts (`keeps-state` and `stores-user-data`): more
    questions for every project, for no rule that needs the split.

### Gaps
- I did not check every module rule for a hidden dependence on the broad
  reading. Only the rules listed above name the fact.

## Section 3: USS-PRF-002 passes when no budget exists

### Takeaway
**Confirm (USS-PRF-001 is enough), and make the empty case explicit.** PRF-002
judges each budgeted measure; with no budget there is nothing to judge, and
PRF-001 already fails. Failing PRF-002 too would count one gap twice and would
need two Standard Deviations for one decision. Add one sentence to the Verify,
and state the empty-set reading once in the conventions.

### Cited Findings
- PRF-002 Verify: "Take the measures that the project has budgeted. … A job
  that only records the numbers, or a measure with no job, fails." It has no
  sentence for the case of no measure — [standard/core/performance.md](../../../../../standard/core/performance.md)
  (rule USS-PRF-002)
- PRF-001 fails when a kind of software has no budgeted measure ("A kind with
  no budgeted measure, or a budget with no number, fails") — same file, USS-PRF-001
- This repository: PRF-001 `fail` (no budget), PRF-002 `pass` with "This pass
  follows the Verify text, which has nothing to judge when there is no
  budget"; Efficiency shows 66% (2 of 3) — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml);
  [docs/superpowers/follow-ups.md:20](../../../follow-ups.md)
- Other USS rules already pass on an empty set: DOC-006 passed because no bug
  report was closed in 12 months; DST-015 judges only the kinds of artifact
  that carry a signature — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml);
  [standard/core/distribution.md](../../../../../standard/core/distribution.md)
- OpenSSF Best Practices criteria mark dependent criteria such as
  `static_analysis_fixed` and `dynamic_analysis_fixed` "{N/A allowed}", while
  the parent practice (`static_analysis`) is scored on its own — [OpenSSF Best Practices criteria, passing level](https://www.bestpractices.dev/en/criteria/0)
- Score rule: "`deviation`, `declined`, `fail` and `unverified` all score 0";
  compliance needs each SHOULD to be `pass` or `deviation` —
  [standard/conventions.md:448-484](../../../../../standard/conventions.md)

### Inferences
- **Reason.** One missing decision (no budget) should give one finding. A
  project that records a Standard Deviation for PRF-001 ("this CLI has no
  budget, because …") has also, in effect, decided that there is nothing to
  enforce. If PRF-002 failed, it would need a second deviation with the same
  reason. Compliance is already blocked by PRF-001.
- The cost of the reading is a score that is one rule too kind (Efficiency
  66% in place of 33% here). That is the same trade that DOC-006 and DST-015
  make, so changing PRF-002 alone would make the standard inconsistent.
- **Exact change** (append to the PRF-002 Verify): "A project with no budgeted
  measure has nothing to judge here; USS-PRF-001 judges the missing budget."
- **Exact change** (conventions section 8, Outcomes, new bullet): "When a
  Verify judges each item of a set (each budget, each signed artifact, each
  closed report) and the set is empty, the rule is `pass`, unless the Verify
  says otherwise. The rule that requires the set judges its absence."
- **Effect on other projects.** None on results; auditors stop guessing.
- **Alternatives.**
  - Fail PRF-002 with no budget: a truer score, but double counting and two
    deviations for one decision.
  - Gate PRF-002 on a fact such as `has-performance-budget`: a fact that
    depends on another rule's result lets a project leave the rule by not
    writing a budget, which is the current outcome with more machinery.

### Gaps
- No outside standard defines "empty set passes" for audit rules in so many
  words; the Best Practices "N/A allowed" precedent is the closest found.

## Section 3 and 2.3: USS-DEP-006 and archived crates that are finished (nohash-hasher, cfg-if)

### Takeaway
**Change both ways: replace nohash-hasher by a one-line feature change, and
narrow the Verify so an archive notice that promises security fixes (cfg-if)
is not end-of-life evidence.** Do not write a Standard Deviation: DEP-006 is a
MUST NOT, so a deviation makes the project "not compliant". Do not add a
general "finished crate" exemption.

### Cited Findings
- DEP-006 body: a version is end-of-life "when evidence shows that its release
  line … no longer receives security fixes"; the Verify counts "a deprecation
  or archive notice for the package or its release line" as evidence —
  [standard/core/dependencies.md](../../../../../standard/core/dependencies.md) (USS-DEP-006)
- Audit evidence: nohash-hasher 0.2.0 archived, last push 2020-09-08; cfg-if
  1.0.5 archived; cargo-deny 0.22.0 advisories ok — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml)
  (USS-DEP-006)
- `cargo tree --offline -i nohash-hasher`: only through serde-saphyr 2.2.0.
  `cargo tree --offline -i cfg-if`: through ahash 0.8.12, encoding_rs 0.8.44,
  **getrandom 0.3.4, getrandom 0.4.3 (via tempfile) and parking_lot_core
  0.9.12** — local run at 42c3280. The audit evidence names only ahash and
  encoding_rs, so it under-reports the paths.
- serde-saphyr pulls nohash-hasher only through its `serialize` feature:
  `serialize = ["dep:base64", "dep:num-traits", "dep:zmij", "dep:nohash-hasher"]`,
  default `["serialize", "deserialize", "comments"]`; same in 2.2.0
  (2026-11-17), 3.0.0-rc.1 (2026-11-18, pre-release) and the default branch —
  [crates.io sparse index, serde-saphyr](https://index.crates.io/se/rd/serde-saphyr);
  [serde-saphyr releases](https://github.com/bourumir-wyngs/serde-saphyr/releases);
  [serde-saphyr Cargo.toml](https://github.com/bourumir-wyngs/serde-saphyr/blob/master/Cargo.toml)
- `uss` uses only `serde_saphyr::from_str` (deserialisation) —
  [tools/uss/src/yaml.rs:19](../../../../../tools/uss/src/yaml.rs)
- Experiment in a scratch copy (`git archive HEAD`): changing
  `tools/uss/Cargo.toml:25` to
  `serde-saphyr = { version = "2.2", default-features = false, features = ["deserialize"] }`
  removed `nohash-hasher` and `base64` from `Cargo.lock` (15 lines), and
  `cargo test --offline -q -p uss` passed: 159 passed, 0 failed — local run, 2026-11-25
- Latest versions still depend on cfg-if: ahash 0.8.14 (`cfg-if ^1.0`),
  encoding_rs 0.8.44, getrandom 0.4.3 (`cfg-if ^1`), parking_lot_core 0.9.12
  (0.10.0 has none, but parking_lot 0.12 uses 0.9) — [crates.io sparse index](https://index.crates.io/ah/as/ahash);
  [encoding_rs](https://index.crates.io/en/co/encoding_rs); [getrandom](https://index.crates.io/ge/tr/getrandom);
  [parking_lot_core](https://index.crates.io/pa/rk/parking_lot_core). ahash
  and encoding_rs repositories are not archived (pushed 2026-11-25 and
  2026-11-18) — GitHub API, `gh api -X GET repos/tkaitchuck/aHash`, `repos/hsivonen/encoding_rs`
- getrandom closed "Removal of cfg_if in favor of cfg_select?" as not planned
  (2026-05-18): it keeps an old MSRV so that bug fixes reach older toolchains —
  [getrandom issue 847](https://github.com/rust-random/getrandom/issues/847)
- cfg-if README: "This crate has been "replaced" by the `cfg_select!` macro,
  which is stable since Rust 1.95.0 … Barring breakages and security fixes,
  this crate will no longer be updated." Repository archived, last push
  2026-09-16 — [rust-lang/cfg-if](https://github.com/rust-lang/cfg-if)
- nohash-hasher: archived, last push 2020-09-08, description "An implementation
  of `std::hash::Hasher` which does not hash at all"; no statement about fixes —
  [paritytech/nohash-hasher](https://github.com/paritytech/nohash-hasher)
- RustSec has no advisory directory for nohash-hasher, cfg-if, ahash,
  encoding_rs or serde-saphyr (HTTP 404 for `crates/<name>`; database last
  commit 2026-11-25T11:10:30Z) — [RustSec advisory-db, crates/](https://github.com/rustsec/advisory-db/tree/main/crates)
- RustSec policy: a crate is unmaintained when "a crate's author has declared
  that they are no longer maintaining" it, or the author is incommunicado
  (about 1 year of inactivity and 270 days without a reply); "If the author is
  responsive and declares that the project *is* maintained, then the RustSec
  organization considers it maintained" — [RustSec HOWTO_UNMAINTAINED.md](https://github.com/rustsec/advisory-db/blob/main/HOWTO_UNMAINTAINED.md)
- OpenSSF Concise Guide (byline 2025-03-28, history entry 2025-04-23): "Confirm
  significant recent activity (e.g., commits) has occurred within the previous
  12 months"; "most software needs continuous maintenance"; "Unmaintained
  software is a risk". It has no "finished software" exception —
  [OpenSSF Concise Guide for Evaluating Open Source Software](https://best.openssf.org/Concise-Guide-for-Evaluating-Open-Source-Software)
- A Standard Deviation from a MUST or MUST NOT makes the project "not
  compliant" — [docs/superpowers/specs/2026-11-21-universal-software-standard-design.md](../../../specs/2026-11-21-universal-software-standard-design.md)
  §5.5; [AGENTS.md](../../../../../AGENTS.md) "Questions to the developer", step 9

### Inferences
- **cfg-if is not end-of-life by USS's own definition.** The body defines
  end-of-life as "no longer receives security fixes", and the supplier's
  notice says security fixes continue. The Verify treats any archive notice as
  evidence, so the Verify is stricter than the body. Aligning the Verify with
  the body is a clarification, not a new rule.
- cfg-if cannot be replaced: five crates in this lockfile need it, and the
  latest release of each still does. Without the change, almost every Rust
  project with a lockfile fails a MUST NOT from the day the repository was
  archived (2026-09-16 or earlier).
- **nohash-hasher is end-of-life** (archived 2020, no promise of fixes), and
  `uss` does not need it. Replacing it costs one manifest line and is proven
  by the scratch build. This is better than any exemption.
- **Exact change, Verify of USS-DEP-006** (insert after the evidence
  sentence): "A deprecation or archive notice that also says that the
  supplier still ships security fixes for the release line is not such
  evidence."
- **Exact change, body of USS-DEP-006** (append to the definition): "A notice
  that says that the supplier still ships security fixes for the release line
  does not make it end-of-life."
- **Exact change, project:** `tools/uss/Cargo.toml:25` →
  `serde-saphyr = { version = "2.2", default-features = false, features = ["deserialize"] }`;
  run `cargo update -p serde-saphyr --offline` or a normal build to refresh
  `Cargo.lock`; note the change in `docs/dependencies.md` and ADR 0002. Run
  `cargo xtask check` before committing.
- **Effect on other projects.** The Verify change exempts only packages whose
  own notice promises security fixes (cfg-if, and similar "superseded but
  patched" packages). Packages archived with no such promise still fail.
- **Alternatives.**
  - Standard Deviation for each crate: makes USS "not compliant" for two
    leaf crates; the worst option.
  - A general exemption for "finished" crates (no unsafe code, no
    dependencies, small): subjective, and the OpenSSF guide says the
    opposite; a flaw in a finished crate still has nobody to fix it.
  - Count only registry or scanner flags (RustSec-style): weaker than USS's
    chosen bar and ties the rule to one ecosystem's database.
  - Replace serde-saphyr or jsonschema: large change, and cfg-if stays through
    getrandom and parking_lot_core.

### Gaps
- The crates.io web API was not queried (its policy asks for contact details
  in the User-Agent); versions come from the sparse index and publication
  dates from GitHub releases. ahash and encoding_rs publication dates were not
  read.
- `uss` still parses comments correctly without the `comments` feature only
  as far as the 159 tests show; no other check was run.

## Section 3: USS-TST-003 and `fix` commits that change no code

### Takeaway
**Change the Verify to leave out fixes that change only prose, and adopt a
commit-type rule.** Conventional Commits ties `fix` to "a bug fix for your
application", and for USS the rule text is part of the product, so
`fix(standard)` is correct usage; relabelling all of it `docs` would misstate
PATCH-level fixes. A test is possible only where a program acts on the change.

### Cited Findings
- TST-003 Verify: "Take each change in the history window that the project
  labels as a bug fix (by its commit message, its issue label or its type in
  the changelog) … Each one adds or changes a test in the same change." —
  [standard/core/testing.md](../../../../../standard/core/testing.md) (USS-TST-003)
- Conventional Commits 1.0.0: "a commit of the type fix patches a bug in your
  codebase"; item 3: "The type fix MUST be used when a commit represents a bug
  fix for your application"; other types: "@commitlint/config-conventional …
  recommends build:, chore:, ci:, docs:, style:, refactor:, perf:, test:";
  FAQ on a commit that fits several types: "Go back and make multiple commits
  whenever possible" — [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/)
- Keep a Changelog 1.1.0: "Fixed for any bug fixes" — [Keep a Changelog 1.1.0](https://keepachangelog.com/en/1.1.0/)
- Standard versioning: PATCH = "wording, references or verification text with
  no normative change" — [standard/conventions.md:553-577](../../../../../standard/conventions.md)
- History at 42c3280 (149 commits): `fix` 71 (`fix(standard)` 49, `fix(uss)`
  8, bare `fix` 7, others 7), `feat` 41, `docs` 22, `chore` 9, `test` 2,
  `style`, `refactor`, `ci`, `build` 1 each — local `git log --format=%s`
- Of the 71 `fix` commits up to 51435cb: 22 change a `.rs` file; 47 change only
  Markdown; 2 change other files (d726138 `Cargo.toml`, 484209a
  `.github/workflows/ci.yml`). Of the 47, 4 change fields that `uss` reads
  (`Level`, `Applies when`, `Refines` and so on): f759fc9, 3757de9, baebc97,
  5ed257a — local `git show` classification
- Audit result: 49 of 71 fix commits change no test, so TST-003 fails —
  [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-TST-003)

### Inferences
- **Reason.** The rule's purpose is "a bug that was fixed once and has no test
  returns". A wording fix in prose has no behaviour that a test can show. A
  fix to an `Applies when` expression does: a test can assert that the rule
  now applies to the facts that it should. The line between the two is
  whether a program acts on the change.
- **Exact change, Verify of USS-TST-003** (insert after the first sentence):
  "Leave out each change that changes only text that people read, such as
  documentation, comments and prose, and no code, configuration or data that
  a program acts on."
- **Exact change, body:** "Each bug fix in code, configuration or data that a
  program acts on MUST come with a test that fails without the fix."
- **Commit-type rule for this repository** (CONTRIBUTING.md and AGENTS.md
  "Conventions"): use `fix` for a fix to code or to rule fields that `uss`
  reads, with its test in the same commit; `fix(standard)` for prose-only rule
  fixes (left out by the new Verify); `docs` for documents outside the
  product; `ci` and `build` for CI and build settings (484209a and d726138
  would then not be labelled as bug fixes).
- **Effect on this repository.** With the change, 43 of the 49 failing
  commits (the prose-only ones) leave the count. The 4 field changes and the
  2 configuration fixes stay (they need a test or a truthful type). The next audit's window starts
  after 51435cb anyway.
- **Effect on other projects.** Projects that label typo fixes `fix` stop
  failing a MUST for them. Code fixes are judged as before; the auditor reads
  the diff, so a code fix cannot hide behind the label.
- **Alternatives.**
  - Relabel only, no wording change: wrong for USS, where rule prose is the
    product and `fix` ↔ PATCH.
  - Exempt all non-code changes, configuration included: too broad;
    configuration bugs (such as a missing `overflow-checks`) can have tests.
  - Keep the rule: every prose fix fails a MUST, which rewards mislabelling.

### Gaps
- I did not run the before-and-after test runs for the 22 code fixes; the
  audit also skipped them.

## Section 4.7: a practice that keeps USS-DEP-011, USS-DEP-012 and USS-TST-003 passing

### Takeaway
**Put the checks where this repository's changes happen: in commits, not in a
PR template.** The repository has no pull requests and no merge commits, so
the PR template's dependency checkbox never runs. Add a commit template, a
`commit-msg` hook, and one unit test that ties `Cargo.toml` to
`docs/dependencies.md`.

### Cited Findings
- `gh api -X GET repos/{owner}/{repo}/pulls?state=all` → 0; `git log --merges`
  → 0 commits — local run, 2026-11-25
- The PR template already asks for the health check ("This change adds no
  dependency, or the description above holds the health check
  (USS-DEP-012)") — [.github/pull_request_template.md](../../../../../.github/pull_request_template.md)
- The only hook is `pre-commit` (`cargo fmt --check`, `uss lint`) —
  [.githooks/pre-commit](../../../../../.githooks/pre-commit)
- `docs/dependencies.md` gives each direct dependency's purpose and says "A
  new dependency adds its entry here in the same change", but has no health
  column — [docs/dependencies.md](../../../../../docs/dependencies.md)
- DEP-011 and DEP-012 judge "the change that added it"; DEP-012 needs a record
  of maintained, deprecated or archived, more than one maintainer, and the
  standard library — [standard/core/dependencies.md](../../../../../standard/core/dependencies.md)
- OpenSSF Concise Guide checks: activity in 12 months, last release in 12
  months, "more than one maintainer, ideally from different organizations" —
  [OpenSSF Concise Guide](https://best.openssf.org/Concise-Guide-for-Evaluating-Open-Source-Software)

### Inferences
- **Practice (three parts).**
  1. Add a "Health check" column to `docs/dependencies.md` (date; maintained:
     last release and activity in 12 months; deprecated or archived; number of
     maintainers; why the standard library is not enough). A commit that adds
     a dependency adds its row. The row is the record "in the change that
     adds it".
  2. Add a test, such as `every_direct_dependency_is_documented`, that reads
     each `[dependencies]`, `[dev-dependencies]` and `[build-dependencies]`
     table of `tools/*/Cargo.toml` and fails when a name has no row with a
     purpose and a filled health cell. CI then enforces DEP-011 and DEP-012.
  3. Add `.gitmessage` (set by the one setup command of CTB-001 with
     `git config commit.template .gitmessage`) and a `commit-msg` hook in
     `.githooks/`:

     ```text
     # type(scope): summary
     # fix: a bug in code or in rule fields that uss reads. Add the test that
     #      fails without the fix in this commit.
     # docs, ci, build: documents, CI files or build settings only.
     # New dependency: add its row (purpose and health check) to
     # docs/dependencies.md in this commit.
     ```

     The hook warns when a `fix` commit changes a `.rs` file under `src/`
     and no test, and when a `fix` commit outside `standard/` changes only
     Markdown.
- A hook can be skipped, so the unit test (part 2) is the part that CI
  enforces; the hook is a reminder for TST-003, which needs judgement.
- **Alternatives.**
  - Use pull requests for every change so the PR template runs: more process
    for a one-person repository.
  - A CI script that checks the commit range of each push for the same
    patterns: stronger than a hook, more code to keep.

### Gaps
- None found.

## Section 2.5: rulings R1 to R10

### Takeaway
**Confirm R1, R2, R3, R5, R6 and R7; confirm R4 for 0.1.0 with a format
change later; confirm R9 with one headless exception; reverse R8 (see the next
key question); reverse R10 (fix the install defect before the trial or the
release).**

### Cited Findings
- R7: the Rust module requires `overflow-checks = true` in the release profile
  (USS-RUST-004) — [standard/modules/languages/rust.md:97-106](../../../../../standard/modules/languages/rust.md);
  the root manifest has it — [Cargo.toml:26-27](../../../../../Cargo.toml).
  Cargo: "Cargo only looks at the profile settings in the `Cargo.toml`
  manifest at the root of the workspace"; release default
  `overflow-checks = false` — [The Cargo Book, Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- R10: `rustup toolchain install --help` (rustup 1.29.3, 2026-11-02): "Install
  or update the given toolchains"; options `--no-self-update` and
  `--no-update` ("Don't try to update the installed toolchain") — local run.
  AGENTS.md runs `rustup toolchain install stable` and
  `cargo +stable install …` — [AGENTS.md:59-63](../../../../../AGENTS.md).
  The repository pins 1.100.0 and `rust-version = "1.100"` —
  [rust-toolchain.toml](../../../../../rust-toolchain.toml); [Cargo.toml:8](../../../../../Cargo.toml).
  USS-SUP-002: a script "MUST NOT install, upgrade or remove a system-wide
  runtime" — [standard/core/supply-chain.md:39-58](../../../../../standard/core/supply-chain.md);
  Agent conduct: "Do not change files, settings or systems outside the task" —
  [AGENTS.md](../../../../../AGENTS.md)
- R4: headless step 5 writes an open question with no dependent rule as
  `unverified` on USS-REL-011 — [AGENTS.md:364-381](../../../../../AGENTS.md);
  REL-011 is `always`, category Maintainability — [standard/core/release.md](../../../../../standard/core/release.md)
- R9: the Audit workflow adds the snippet when none exists (step 12) and
  commits it (step 14); "With no person present, do not offer" the CI template
  (step 13) — [AGENTS.md:186-196](../../../../../AGENTS.md). REL-011 accepts an
  audit "of a commit whose only later changes are under `.uss/`" —
  [standard/core/release.md](../../../../../standard/core/release.md)
- R3: xtask runs `uss check` as a CI step ("==> uss check") —
  [tools/xtask/src/main.rs:198-216](../../../../../tools/xtask/src/main.rs);
  USS-TST-002 already fails for untested xtask error paths —
  [docs/superpowers/follow-ups.md:236](../../../follow-ups.md)
- R5: upgrade step 13 — [AGENTS.md:408-410](../../../../../AGENTS.md)

### Inferences
- **R1, confirm.** A link that was broken for one task and is now fixed has
  no lasting effect.
- **R2, confirm.** Printing the fields in the order of the rule files matches
  what a reader sees in the source; no rule or workflow depends on another
  order. Low stakes.
- **R3, confirm.** CI runs the step on every push, so a broken step fails CI.
  The missing tests of xtask error paths are already an open USS-TST-002
  failure in the release plan (section 4.2).
- **R4, confirm for 0.1.0; add a `questions` list to the results format in a
  later 0.x release.** REL-011 is the rule about the audit itself, it always
  applies, and `unverified` blocks compliance until a person answers, which
  is the intent. The cost is a one-rule dent in Maintainability and an odd
  place for the question. A results-file list `questions:` that `uss report`
  counts as "not compliant" would be cleaner; the record format may change in
  a 0.x MINOR release (conventions section 10).
- **R5, confirm, with one addition:** "Show the developer each rule that
  passed before the upgrade and fails now." Developer acceptance is the only
  workable gate when the project was not compliant before.
- **R6, confirm.** The Definition of Done says each document that names a
  command changes with it; a stricter test enforces that.
- **R7, confirm.** USS-RUST-004 requires it, and Cargo reads profiles only
  from the workspace root.
- **R8, reverse.** See the next key question.
- **R9, confirm for audits with a person present; add a headless exception.**
  With no person present, adding and committing the snippet changes a file
  outside `.uss/`, so the audited commit is no longer the release commit, and
  `uss release-check` fails. Exact change, AGENTS.md audit step 12: add "With
  no person present, do not add the snippet. Report that it is missing."
- **R10, reverse: fix it now.** `rustup toolchain install stable` updates an
  installed stable toolchain (rustup's own help says "Install or update"), so
  the workflow changes the developer's machine outside the task, against
  USS-SUP-002's intent and the Agent conduct rule. The fix in follow-ups
  section 4.1 is right: `rustup toolchain install 1.100.0 --profile minimal
  --no-self-update` and `cargo +1.100.0 install --locked …`, with a test that
  ties the number to `rust-version`. Effect on adopters: their default and
  stable toolchains stay as they were; one pinned toolchain is added.
- **Alternatives.**
  - R4: a dedicated rule for open questions: a new rule only for audit
    bookkeeping.
  - R9: never add the snippet during an audit, only offer it: weaker uptake of
    the agent instructions.
  - R10: `rustup toolchain install stable --no-update --no-self-update`:
    leaves an existing stable alone, but an old stable then fails
    `rust-version` and the install is not repeatable.

### Gaps
- R2 and R6 were checked from the follow-ups text and a short look at the
  code, not by reading the full functions.

## Section 2.5: the reading "before the first release" (plan decision 12, R8)

### Takeaway
**Change: before the first release, judge the release process at the audited
commit in place of the release.** The current split is inconsistent, and it
makes the first release's own audit unable to pass: the release audit runs
before the tag, so no published artifact can exist when it runs.

### Cited Findings
- Decision 12: when a Verify judges each release and none exists, record
  `pass` "no release yet"; when it needs a published artifact, record `fail` —
  [docs/superpowers/plans/2026-11-25-uss-workflows-skills-self-audit.md:183-187](../../../plans/2026-11-25-uss-workflows-skills-self-audit.md)
- AGENTS.md step 16 carries the same split, and treats "a document about a
  release" as part of the release — [AGENTS.md:260-272](../../../../../AGENTS.md)
- Results under it: LIC-004, DST-014, DST-015, REL-003, REL-011 `pass` "no
  release yet"; DST-002, SUP-013, SUP-014, SUP-015 `fail` — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml).
  LIC-004's evidence also says "The uss binary embeds no notices file for its
  dependencies yet"; REL-003's says "No document gives rebuild steps yet".
- Rule texts are parallel: SUP-013 "Each release has an SBOM"; DST-014 "Each
  has a build for x86-64 and a build for ARM64, in the latest release";
  LIC-004 "Each release artifact MUST include the licence texts" —
  [standard/core/supply-chain.md](../../../../../standard/core/supply-chain.md);
  [standard/core/distribution.md](../../../../../standard/core/distribution.md);
  [standard/core/licensing.md](../../../../../standard/core/licensing.md)
- Release audit order: audit the commit to release, commit the results, then
  "Put the release tag on the commit that holds the results" —
  [AGENTS.md](../../../../../AGENTS.md) "Release audit", steps 2 and 5
- OpenSSF Scorecard (v5.7.0, released 2026-11-19): with no releases, the probe
  `releasesAreSigned` returns `OutcomeNotApplicable` ("no GitHub/GitLab
  releases found"); the check looks at the last releases' assets only —
  [scorecard probes/releasesAreSigned/impl.go](https://github.com/ossf/scorecard/blob/main/probes/releasesAreSigned/impl.go);
  [Scorecard checks.md, Signed-Releases](https://github.com/ossf/scorecard/blob/main/docs/checks.md)
- AGENTS.md already allows a nearest substitute: "use a fresh clone in place
  of a clean machine. Name the substitute in the evidence" —
  [AGENTS.md](../../../../../AGENTS.md) "Judge each rule", step 9

### Inferences
- **Why the split fails.** "Each release has an SBOM" fails while "each
  artifact has an ARM64 build" passes, though both say what a release must
  have. Worse, under decision 12 the audit of the 0.1.0 release commit still
  sees no release, so SUP-013, SUP-014 and SUP-015 fail even when the release
  workflow signs and attests everything. `uss release-check
  --require-compliant` could never pass on a first release.
- **Exact change, AGENTS.md step 16** (replace the step):

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

  Change plan decision 12 and spec §8 to match.
- **Effect on USS's own results.** LIC-004 (MUST) becomes `fail` (no notices
  file), DST-014 `fail` (no release workflow), REL-003 `fail` (no rebuild
  steps), REL-011 `fail` until a release process runs `uss release-check`;
  DST-015 stays `pass` (nothing is signed, SUP-014 judges that). These are
  items the release plan already lists (sections 4.6), so the audit stops
  hiding them. The 0.1.0 release audit can then pass once the workflow exists.
- **Effect on other projects.** A new project that declares
  `publishes-releases` sees its release gaps before it ships, and can be
  compliant at its first release.
- **Wider flag (not only before the first release).** In every release audit
  the "latest release" is the previous one, because the audit precedes the
  tag. Consider the same rule for all release audits: judge the release
  process at the release commit, and check the published artifacts of the
  previous release as well.
- **Alternatives.**
  - Keep decision 12: inconsistent, and blocks first-release compliance.
  - Pass all "no release yet": scores overstate, and nothing checks the
    release process until a release has shipped without its SBOM.
  - Write `unverified` (Scorecard's "not applicable" analogue): blocks
    compliance until a release exists, which the release audit can never see.

### Gaps
- No outside standard was found that defines how to audit release
  requirements before any release; Scorecard's choice (not applicable) is the
  nearest precedent.

## Section 2.5: an adopted MAY rule that fails lowers the score but not compliance

### Takeaway
**Confirm.** It is what spec §5.5 and conventions section 8 say, it matches
RFC 2119's "truly optional", and the alternative would reward declining MAYs.
Optionally, list adopted MAY rules that fail in their own line of the report.

### Cited Findings
- Spec §5.5: compliant when every MUST passes, every SHOULD is `pass` or
  `deviation`, "every applicable MAY has a recorded decision", and nothing is
  `unverified` — [design spec §5.5](../../../specs/2026-11-21-universal-software-standard-design.md)
- Conventions: "An adopted MAY rule that fails lowers the score of its
  category, but it does not affect compliance." — [standard/conventions.md:459-460](../../../../../standard/conventions.md)
- `declined` and `fail` both score 0 — [standard/conventions.md:462-484](../../../../../standard/conventions.md)
- RFC 2119: "MAY … mean that an item is truly optional" — [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119)
  (text known; not re-read this session)
- USS adopts DST-002 and it fails (no binaries yet) — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml);
  [.uss/record.yaml](../../../../../.uss/record.yaml) (`decisions`)

### Inferences
- If an adopted but failing MAY made a project non-compliant, a project could
  regain compliance by switching `adopt: true` to `adopt: false`, with the
  same score (both 0). The rule would only punish honesty about intent.
- Rules switched on by an adopted MAY (`Enables`) are MUST and SHOULD rules
  and do affect compliance, so adopting an extension still binds.
- **Optional change:** the report lists "Adopted MAY rules not met: …", so a
  reader sees the gap between intent and state. No standard change.
- **Alternative.** Treat `adopt: true` as a commitment that affects
  compliance: perverse incentive to decline.

### Gaps
- None found.

## Section 2.5: the Plan 3 numbers (CTB-005, DOC-006, PRF-005, SAF-004, BAK-007)

### Takeaway
**Confirm all five numbers; refine three Verify texts.** BAK-007: confirm and
cite the US-CERT paper that says "1 primary and 2 backups". CTB-005: keep 25%,
but compare like with like. PRF-005: keep 1%, lengthen the window to five
minutes after start-up ends. SAF-004: keep weekly, apply the same bound to the
fuzzing rules, and check that the schedule really ran. DOC-006: confirm.

### Cited Findings
- **BAK-007.** US-CERT "Data Backup Options" (Paul Ruggiero and Matthew A.
  Heckathorn, Carnegie Mellon University for US-CERT, 2012): "3 – Keep 3
  copies of any important file: 1 primary and 2 backups. 2 – Keep the files
  on 2 different media types … 1 – Store 1 copy offsite" — [CISA-hosted PDF](https://www.cisa.gov/sites/default/files/publications/data_backup_options.pdf).
  CISA's current page lists "3 copies of important files", "2 different types
  of storage media", "1 copy stored off-site", does not say whether the
  original counts, and mentions "offline copies" — [CISA, Back Up Business Data](https://www.cisa.gov/audiences/small-and-medium-businesses/secure-your-business/back-up-business-data)
  (no date shown). BAK-007's rationale says "Counting the production data as
  one of the three copies is USS's own reading; the CISA page does not say
  whether it counts" — [standard/core/backup.md](../../../../../standard/core/backup.md) (USS-BAK-007)
- **CTB-005.** Verify: "Run the command, or read the duration of the matching
  CI job … the time is not more than 25% above the stated time" — [standard/core/contributability.md](../../../../../standard/core/contributability.md)
  (USS-CTB-005). GitHub-hosted runners: `ubuntu-latest` and `windows-latest`
  have 4 CPUs / 16 GB on public repositories but 2 CPUs / 8 GB on private
  ones — [GitHub Docs, GitHub-hosted runners reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).
  USS states "about 9 seconds" warm and "about 140 seconds" cold; warm runs
  measured 6.2 s, a cold fresh clone 301 s — [.uss/audits/2026-11-25.yaml](../../../../../.uss/audits/2026-11-25.yaml) (USS-CTB-005)
- **PRF-005.** Verify: idle 60 seconds after start-up, below 1% of one core —
  [standard/core/performance.md](../../../../../standard/core/performance.md) (USS-PRF-005).
  Apple: "When your app isn't active, the number in this column [% CPU] should
  be 0.0" (updated 2016-09-13) — [Apple, Energy Efficiency Guide for Mac Apps, Monitoring Energy Usage](https://developer.apple.com/library/archive/documentation/Performance/Conceptual/power_efficiency_guidelines_osx/MonitoringEnergyUsage.html).
  Microsoft (ms.date 2026-07-22): in the background an app should not "use
  system resources, wake the CPU through timers, wake the CPU by waiting for
  vsync events"; the test procedure leaves the device idle "for 5 minutes",
  and CPU use and context switches should be "as close to 0 as possible" —
  [Microsoft Learn, Power consumption improvements](https://learn.microsoft.com/en-us/windows/apps/performance/power)
- **SAF-004.** Verify: "on each pull request or on a schedule of at most a
  week" — [standard/core/runtime-safety.md:86](../../../../../standard/core/runtime-safety.md);
  SAF-007 uses the same weekly bound — [standard/core/runtime-safety.md:140-159](../../../../../standard/core/runtime-safety.md).
  TST-011 and TST-012 say only "on a schedule" and "A recent run exists",
  with no bound — [standard/core/testing.md:225-262](../../../../../standard/core/testing.md).
  ClusterFuzzLite examples: PR fuzzing for 600 s; batch fuzzing every 6 hours
  ("Change this to whatever is suitable"); corpus pruning and coverage once a
  day; `address` sanitizer on by default — [ClusterFuzzLite, GitHub Actions](https://google.github.io/clusterfuzzlite/running-clusterfuzzlite/github-actions/).
  GitHub: "In a public repository, scheduled workflows are automatically
  disabled when no repository activity has occurred in 60 days" —
  [GitHub Docs, Disabling and enabling a workflow](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/disable-and-enable-workflows)
- **DOC-006.** Verify: each error that closed reports "name more than once"
  has an entry — [standard/core/documentation.md](../../../../../standard/core/documentation.md) (USS-DOC-006)

### Inferences
- **BAK-007, confirm.** The 2012 US-CERT paper, hosted by CISA, says the three
  copies are one primary and two backups. Exact change: replace the
  rationale's last sentence with "The production data counts as one of the
  three copies, as US-CERT's Data Backup Options (2012) says: '1 primary and
  2 backups'." and add that paper to References. Effect on others: none; it
  removes a USS-invented reading.
- **CTB-005, confirm 25%; change the comparison.** A guide that states a time
  "on a typical machine" and an auditor who reads a CI job on a 2-CPU private
  runner compare different machines; cold CI runs also download toolchains.
  Exact change: "The contributing guide states how long the one-command
  checks take, and on what (a named machine class, or the CI job). Compare
  with a run on the same kind of machine, warm or cold as the guide states."
  Effect: fewer false failures for private repositories.
- **PRF-005, confirm 1%; lengthen the window.** Apple's target is 0.0 and
  Microsoft's procedure idles for 5 minutes; 1% of one core is a lenient,
  measurable bar. Sixty seconds right after start-up can catch start-up work
  (caches, JIT, first GC) and miss a timer that fires every few minutes.
  Exact change: "Start the software and wait until start-up ends (it reports
  ready, or 30 seconds pass). Then measure its processor time for 5 minutes,
  with nothing to do: it stays below 1 percent of one core." Effect: audits
  take 4 more minutes per idle test.
- **SAF-004, confirm weekly; align and check.** Weekly is lenient against
  ClusterFuzzLite's daily and six-hourly examples, but PR runs are the main
  path and weekly bounds the exposure. Exact changes: in TST-011 and TST-012,
  replace "on a schedule … A recent run exists" with "on a schedule of at most
  a week … The latest scheduled run is less than a week old."; in SAF-004 and
  SAF-007, add "The latest scheduled run is less than a week old." The second
  clause catches schedules that the host has turned off.
- **DOC-006, confirm.** "More than once" is the smallest count that means
  "recurs". No outside threshold was found; any other number is as arbitrary.
- **Alternatives (one line each).** BAK-007: count only backup copies (four
  copies in all), stricter than the source. CTB-005: 50% margin, looser
  without fixing the machine mismatch. PRF-005: keep 60 s, faster audits but
  more false results. SAF-004: daily, closer to fuzzing practice but more CI
  cost. DOC-006: three reports, fewer entries required.

### Gaps
- No outside source was found that names a margin for stated check durations
  (CTB-005) or a count for "common" errors (DOC-006).
- The US-CERT paper is from 2012 and is not on CISA's current backup page.

## The collapsed list of Plan 3 and Plan 4 rulings

### Takeaway
**Confirm all, with two flags: the USS-K8S-007 body states the unset case
wrongly, and the Python end-of-life ruling (USS-PY-007) lets a library declare
an untested minimum.** Neither flag reverses a ruling.

### Cited Findings
- K8S-007 body: "The Pod spec's value wins over the ServiceAccount's, and an
  unset value mounts the token" — [standard/modules/platforms/kubernetes.md:218-221](../../../../../standard/modules/platforms/kubernetes.md).
  Kubernetes docs: "If both the ServiceAccount and the Pod's `.spec` specify a
  value for `automountServiceAccountToken`, the Pod spec takes precedence";
  the ServiceAccount itself can set `automountServiceAccountToken: false` —
  [Kubernetes, Configure Service Accounts for Pods](https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/)
- HRD-008 accepts the setting "on the pod or on its service account" —
  [standard/core/deployment-hardening.md](../../../../../standard/core/deployment-hardening.md) (USS-HRD-008)
- PY-007: the pinned version and "each Python version in the CI test matrix"
  must not be end-of-life; "`requires-python` … is not judged by this rule" —
  [standard/modules/languages/python.md:147-158](../../../../../standard/modules/languages/python.md)
- ESLint profile: baseline ESLint "is not the SAST tool of the rule …
  because its core and typescript-eslint rules are not a security rule set";
  Ruff `S`, gosec and Find Security Bugs are — [standard/vendors/eslint/eslint.md:91](../../../../../standard/vendors/eslint/eslint.md);
  [ruff.md:48](../../../../../standard/vendors/astral-sh/ruff.md);
  [golangci-lint.md:85](../../../../../standard/vendors/golangci/golangci-lint.md);
  [spotbugs.md:89](../../../../../standard/vendors/spotbugs/spotbugs.md)
- MVN-004 accepts bounded ranges; MVN-006 requires the maven-lockfile plugin
  and its `validate` goal, so ranges still resolve to locked versions —
  [standard/modules/tools/maven.md:118-201](../../../../../standard/modules/tools/maven.md)
- The other rule texts read for this list: QLT-006/007/008/012, TRU-001/004,
  REL-001/007, OPS-002, AUD-004/006, DAT-005, EXT-002, HRD-003, SAF-001,
  SEC-028, GO-006/010, RUST-010, SUP-005, TS-004/015, JAVA-001, REACT-004/005 —
  `standard/core/*.md`, `standard/modules/**/*.md` at 42c3280

### Inferences
Plan 3:
- QLT-006 met by the baseline; QLT-007/008 judge a rule turned off: **confirm**
  (no double judging; matches spec §6.5).
- TRU-004 a MUST: **confirm** (spec §6.1 TRU: data "MUST NOT be held hostage";
  the positive form is easier to verify).
- TRU-001 paywall moves only, REL-007 removal from every edition: **confirm**;
  the known gap (an operated service with no releases) is written in the spec.
- Overlaps AI-001 with SEC-009/010/013/014, HRD-003 with HRD-002: **confirm**
  (different scopes; overlap costs one extra result).
- OPS-002 leaves out alerts other rules require: **confirm** (BAK-010 and the
  others must page on a cause by design).
- AUD-004/006 removal only at expiry; DAT-005 keeps audit records: **confirm**
  (consistent across the three rules).
- REL-001 and EXT-002 SemVer as written, no 0.x rule: **confirm** (SemVer
  item 4 already covers 0.x; USS itself is 0.x).
- Unreadable sources not cited; registration-gated documents cited by the
  publisher's version: **confirm** (spec §6.4 "checked at writing time").

Plan 4:
- GO-006 exact Go release used by CI: **confirm**.
- GO-010 without GONOSUMCHECK: **confirm** (the remaining settings are the
  ones that turn checksum checks off).
- RUST-010 no longer refines SUP-005: **confirm** (cargo-deny checks
  lockfile dependencies, SUP-005 judges downloads outside a lockfile).
- TS-015 pnpm `allowBuilds`; TS-004 the effective tsconfig: **confirm**.
- JAVA-001 also refines SAF-001: **confirm** (makes `-Xlint:all -Werror`
  binding through SAF-001).
- Oxlint replaces eslint-plugin-jsx-a11y (REACT-004): **confirm**; it is a
  SHOULD, so ESLint users record a deviation and still meet ACC-002.
- Squawk default rules; kube-linter without organisation-specific checks:
  **confirm**.
- K8S-007 stricter than HRD-008: **confirm the strictness, fix the wording.**
  A ServiceAccount can be shared or not shipped by the chart, so setting the
  value on the Pod is the safer rule. But "an unset value mounts the token" is
  true only when the ServiceAccount also leaves it unset. Exact change:
  "An unset value leaves the choice to the ServiceAccount, whose default
  mounts the token." Also state the reason for the stricter rule in its
  rationale, because spec §6.3 says a module never "silently raises" a core
  rule.
- MVN-004 bounded ranges: **confirm** (MVN-006 locks the resolution).
- Bandit-class rule sets count as SAST, ESLint does not: **confirm** (the
  ESLint baseline has no security rule set; an ESLint security plugin in a
  future profile could change this).
- Module end-of-life rules judge what is built, tested and run: **confirm,
  with a flag.** Under PY-007 a library that declares `requires-python >=3.9`
  cannot test 3.9 in CI without failing, so it may ship an untested claim.
  Consider a SHOULD that the declared minimum is a supported version, or let a
  library's CI test its declared minimum as long as it is the only
  end-of-life entry.
- REACT-005 exempts plain string literals: **confirm** (static markup the
  project wrote is not untrusted input).
- QLT-012 inline suppressions only: **confirm** (config-level stale entries
  are handled by profiles, such as cargo-deny `unused-ignored-advisory`).
- SAF-001 binds the module MUST rules that refine it: **confirm**.

### Gaps
- I did not re-read the sources behind the Plan 4 rulings (pnpm, Oxlint,
  Squawk, kube-linter, Go toolchain docs); the confirmations rest on the rule
  texts and their recorded sources.

## Section 5: removing the four locked worktrees and their branches

### Takeaway
**Remove them only after the Claude Code session that locked them has ended,
then unlock, remove without `--force`, and delete each branch with `-d`.** All
four are clean, fully merged into `main`, and hold about 8.9 GB of build output.

### Cited Findings
- `git worktree list --porcelain`: the four worktrees at 9a07f2d, d8eb4ae,
  f414a61 and 42c3280, each `locked claude session <name> (pid 14180)` — local
  run, 2026-11-25
- `tasklist /FI "PID eq 14180"`: `claude.exe` is running (pid 14180) — local
  run, 2026-11-25. The project instructions loaded in this session come from
  `.claude/worktrees/uss-plan5-workflows/CLAUDE.md`, so a live session may use
  that worktree.
- `git branch --merged main` lists all four `worktree-*` branches; none has an
  upstream — local run
- `git status --porcelain --untracked-files=all` is empty in each worktree;
  ignored content: `.superpowers/` (only `sdd/.gitignore`), `.tools/`
  (about 8.8 MB) and `target/` (2.1 to 2.5 GB each) — local run
- `.claude/worktrees/` is excluded in `.git/info/exclude:7` — local run
- git 2.56.0 docs: "Only clean worktrees (no untracked files and no
  modification in tracked files) can be removed"; "To remove a locked
  worktree, specify `--force` twice"; `unlock`: "Unlock a worktree, allowing it
  to be pruned, moved or deleted"; `prune` removes administrative files of
  worktrees whose directories are missing — [git-worktree documentation](https://git-scm.com/docs/git-worktree)

### Inferences
- **Steps** (run from the main checkout, after closing the Claude Code
  session with pid 14180 and any editor or rust-analyzer that holds files in
  `target/`; Windows refuses to delete open files):

  ```sh
  git worktree list --porcelain                 # confirm paths, locks, heads
  git -C .claude/worktrees/uss-plan2-security status --porcelain --untracked-files=all   # expect nothing; repeat for each
  git worktree unlock .claude/worktrees/uss-plan2-security
  git worktree remove .claude/worktrees/uss-plan2-security   # no --force: it refuses if anything is unclean
  git branch -d worktree-uss-plan2-security                  # -d refuses an unmerged branch
  # repeat for uss-plan3-core-areas, uss-plan4-modules, uss-plan5-workflows (last)
  git worktree prune --dry-run -v
  git worktree list
  ```

- Remove `uss-plan5-workflows` last, and not from a session whose working
  directory is inside it.
- Ignored files (`target/`, `.tools/`, `.superpowers/`) do not block
  `remove` and are deleted with the directory; `.superpowers/` holds only a
  `.gitignore`, so nothing is lost. Removal frees about 8.9 GB.
- Do not use `remove -f -f`: it skips the lock and the cleanliness check
  together.
- If the session that owns a worktree is still open, Claude Code's own exit
  from the worktree (its ExitWorktree action with removal) is the intended
  path, because the harness created the lock.

### Gaps
- I did not confirm that `git worktree remove` deletes ignored files on this
  git build; the documentation defines "clean" without them, which implies it.
- I did not run any removal command, as instructed.
