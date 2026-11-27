# Universal Software Standard (USS) for agents

USS is an engineering standard for all software. Its rules use the RFC 2119
levels, and the `uss` tool calculates which rules apply, the scores and
compliance. This file is for two readers:

- An agent that applies USS to a project reads "Use USS on a project" and
  "Agent conduct".
- An agent that changes USS itself reads "Agent conduct" and "Work on this
  repository".

## Use USS on a project

These workflows apply USS to one project. They use these terms:

- The **record** is the file `.uss/record.yaml` in the project root. It holds
  the stack, facts, decisions, approved Standard Deviations and audits of the
  project.
- A **Standard Deviation** is a document that gives the reason why the
  project does not follow a rule. The record lists each approved one in
  `deviations`.
- A **results file** is `.uss/audits/<date>.yaml`. It holds the result and the
  evidence for each rule in one audit.
- The **audited commit** is the commit that an audit examines. The key
  `commit` in the results file holds its SHA.
- A **report** is `.uss/audits/<date>.md`. `uss report` writes it from a
  results file.
- A **rule that applies** is a rule that `uss applicable .uss/record.yaml`
  lists.
- The **developer** is the person who owns the project.
- The **releases page** is
  <https://github.com/HaleTechCo/Universal-Software-Standard/releases>. It
  lists the USS releases.

### Before every workflow

1. Go to the project root. Run all commands in these workflows from there.
2. Write each file in UTF-8. `uss` refuses UTF-16, which a redirection (`>`)
   writes in Windows PowerShell 5.1.
3. If the project is a checkout of USS, use
   `cargo run --locked -q -p uss --` in place of `uss`. Do not install `uss`.
4. Find the USS version:
   - In an upgrade, it is the new version.
   - Otherwise, it is `standard.version` in the record.
   - With no record, it is the version of the installed `uss`.
   - With no record and no `uss`, it is the latest release on the releases
     page.
5. Run `uss version`. If it prints the version that you found, skip the rest
   of this step. Otherwise, install `uss` at that version:
   1. Look for the release on the releases page. Its tag is `v<version>`.
   2. The install needs a Rust toolchain at the `rust-version` of `uss` or
      later. A `rust-toolchain.toml` file in the project can select an older
      toolchain.
   3. If the releases page lists the release, install it with these commands.
      `+1.100.0` selects that toolchain in place of the toolchain of the
      project. The commands do not update rustup or the `stable` toolchain.

      ```sh
      rustup toolchain install 1.100.0 --profile minimal --no-self-update
      cargo +1.100.0 install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v<version> uss
      ```

   4. The commands name the toolchain of this version of USS. For another
      version, use the `rust-version` of `Cargo.toml` at its release tag. Add
      `.0`: `1.100` installs as `1.100.0`.
   5. If the machine has no rustup, ask the developer for permission to do
      one of these:
      - Install rustup from <https://rustup.rs> with
        `--default-toolchain none`.
      - Use an existing `cargo` at the `rust-version` or later.
        `cargo --version` prints its version.
   6. Do not install a Rust package of the system without asking. If neither
      way works, stop, as step 10 says.
   7. If the releases page does not list the release, install `uss` from a
      USS checkout at that version. Ask the developer for the path of the
      checkout. With no version to match, the developer can give any
      checkout.
   8. In the root of the checkout, run
      `cargo install --locked --path tools/uss`. Then go back to the project
      root.
6. Read the workflows of that version with `uss show AGENTS.md`. If they are
   different from the copy that you read, follow them.
7. Some outputs are too long to read at once, such as those of
   `uss show standard/conventions.md` and `uss applicable --full`. Save each
   such output to a file in a temporary directory outside the project. Read
   the file in parts. Read the rules one area at a time.
8. Read sections 5, 8, 9, 11 and 12 of the conventions with
   `uss show standard/conventions.md`. Section 12 tells you how to examine a
   tool at a version outside its vendor profile.
9. If the record pins that version, read the rules that apply with
   `uss applicable --full .uss/record.yaml`. The output gives the rules in
   the order of their IDs, so the rules of each area are together.
10. Do not calculate applicability, scores or compliance yourself. Only `uss`
    calculates them. If you cannot install `uss`, stop. Tell the developer
    why.

### Questions to the developer

Ask questions in batches, not one at a time. A batch is the table of facts,
or the rules of one area. The area is the prefix in the rule ID, such as `SEC`
in `USS-SEC-013`.

1. Ask about the facts in one table. Give each fact, its question, your answer
   and the evidence for that answer. The developer confirms or corrects the
   table in one reply.
2. Ask about the SHOULD and SHOULD NOT rules, one area at a time. Recommend
   "comply" for each rule. The developer can accept a whole area in one reply.
3. Ask about the MAY rules, one area at a time. Give your recommended answer
   and the score category of each rule. The developer answers each area in
   one reply.
4. In an audit, add what the scan found to each question. For example, say
   if the project meets the rule now.
5. For each SHOULD or SHOULD NOT rule that the developer declines, ask for the
   reason.
6. Write a draft Standard Deviation with that reason at
   `docs/standard-deviations/<RULE-ID>.md`. Start from
   `uss show templates/standard-deviation.md`.
7. Do not write the approver in the Standard Deviation or `approved-by` in
   the record. The developer gets the approval under the policy of their
   organisation.
8. After the approval, the developer adds the Standard Deviation to
   `deviations` in the record. Until then, the rule has no Standard Deviation.
9. If the developer wants a Standard Deviation from a MUST or MUST NOT rule,
   write it in the same way. Tell the developer that it makes the project
   "not compliant".

### Adopt a new project

1. Do the steps in "Before every workflow".
2. Do the steps in "Start the record", below.
3. Add the output of `uss show templates/AGENTS.snippet.md` to `AGENTS.md` in
   the project root. If the project has a `CLAUDE.md` and no `AGENTS.md`, add
   it to `CLAUDE.md`.
4. If the project publishes releases on GitHub, copy the output of
   `uss show templates/ci/uss-release.yml` to
   `.github/workflows/uss-release.yml`. Follow the comments in the file.
5. Commit the record, the draft Standard Deviations and the files that you
   changed.
6. Read the rules that apply with `uss applicable --full .uss/record.yaml`.
   Build the project to these rules.
7. Meet every MUST and MUST NOT rule. Only a Standard Deviation excuses one,
   and the project is then "not compliant".
8. To get a report, do the steps in "Audit an existing project".

#### Start the record

1. Copy the output of `uss show templates/record.yaml` to `.uss/record.yaml`.
2. Set `project.name`.
3. List the stack. Use the identifiers that
   `uss show standard/registry.yaml` lists. No module covers an identifier
   that the registry does not list.
4. Read the definition of each fact in `uss show standard/applicability.md`.
5. Answer every fact. In an audit, take the answers from the scan. In an
   adoption, take them from the description of the project.
6. The `false` values of the template are placeholders, not answers. Do not
   add or remove a fact.
7. Remove the comment "Answer this. false is not a default." from each fact
   that you answer. Keep the other comments.
8. Ask the developer to confirm the facts, as "Questions to the developer"
   says. If a definition fits the project in two ways, give both readings.
   The developer chooses.
9. Run `uss applicable .uss/record.yaml`.
10. Ask about the SHOULD, SHOULD NOT and MAY rules that apply.
11. Write each MAY answer in `decisions`, as `<RULE-ID>: { adopt: true }` or
    `<RULE-ID>: { adopt: false }`. Write no decision for a rule of another
    level.
12. If the developer adopts a MAY rule, run `uss applicable .uss/record.yaml`
    again. Ask about the new rules only. Do this until no new rules apply.
13. Run `uss check .uss/record.yaml`. Correct each problem that it reports.
    Run it again until it reports no problems.

### Audit an existing project

1. Do the steps in "Before every workflow".
2. Scan the project. Find its languages, frameworks, build tools, CI,
   containers, UI, databases, authentication, network use, releases and
   documents. Write down the obvious findings.
3. Run `git status --porcelain --untracked-files=all`. If it lists a file
   outside `.uss/`, ask the developer to commit it. Do not continue until
   it lists no such file.
4. Look for a stopped audit. It is a results file in `.uss/audits/` that has
   no report, or that `git status` lists.
5. If you found a stopped audit, run `git diff --name-only <commit> HEAD`.
   Use its audited commit as `<commit>`.
6. If that command lists only files under `.uss/`, continue the stopped
   audit. Go to "Judge each rule".
7. If it lists another file, the stopped audit is out of date. Delete its
   results file. Then do the next step.
8. If the project has no record, do the steps in "Start the record", under
   "Adopt a new project".
9. If the project has a record, compare its facts with the scan. Ask the
   developer about each fact that the scan does not agree with.
10. If a fact changes, run `uss applicable .uss/record.yaml`. Ask about the
    new rules only.
11. Look for the USS snippet in `AGENTS.md` and `CLAUDE.md` in the project
    root. The snippet starts with the heading "Universal Software Standard
    (USS)".
12. If the project has no USS snippet, add it as "Adopt a new project" says.
    In that case, also offer the CI template if the project publishes
    releases on GitHub.
13. If the developer accepts the CI template, add it as "Adopt a new project"
    says. With no person present, do not offer it.
14. Commit the record, the draft Standard Deviations, the snippet and the CI
    template, if you changed or added them.
15. If you started or changed the record, read the rules that apply with
    `uss applicable --full .uss/record.yaml`.
16. Start the results file `.uss/audits/<date>.yaml`, with the date of today.
    Set `standard.version` to the version that the record pins.
17. If that file exists and is committed, write the new audit in it. Remove
    its results and its suppressions. Keep its entry in `audits`.
18. Set `commit` to the full 40-character SHA that `git rev-parse HEAD`
    prints. Set `auditor` to who or what does the audit, such as `claude`.
    For example:

    ```yaml
    standard:
      version: 0.1.0
    commit: 0123456789abcdef0123456789abcdef01234567
    date: 2026-11-25
    auditor: claude
    results:
      USS-SEC-013:
        result: pass
        evidence: src/db/query.go:42
    ```

#### Judge each rule

1. Audit the rules that apply, one area at a time. Start at the first rule
   that has no result.
2. For each rule, apply its Verify exactly as written. Add nothing to it.
   Leave nothing out.
3. Give each rule one result:
   - `pass`: the project meets the rule.
   - `fail`: the project does not meet the rule, and the record lists no
     Standard Deviation for it.
   - `deviation`: the project does not meet the rule, and the record lists a
     Standard Deviation for it.
   - `declined`: the record declines the MAY rule (`adopt: false`).
   - `unverified`: you cannot decide. Do not guess. Write the reason as the
     evidence.
4. If the parts of a Verify give different results, combine them. If a part
   fails, write `fail`. Otherwise, if a part is unverified, write
   `unverified`. Otherwise, write `pass`.
5. Give evidence for each `pass`, `fail` and `deviation` result. Name a
   `file:line`, a configuration key, a CI job, or a command and its output.
   For `deviation`, name the Standard Deviation.
6. For a search, give the full command, with its revision and its paths. Give
   its matches. If you leave out matches, say which and why.
7. Write results only for the rules that apply. Do not write
   `not-applicable` for a rule that applies.
8. If a Verify needs a tool that is not installed, ask the developer for
   permission to install it. Without permission, write `unverified`. Name
   the tool in the evidence.
9. If a Verify needs an environment that you do not have, use the nearest
   substitute. For example, use a fresh clone in place of a clean machine.
   Name the substitute in the evidence.
10. If no substitute can show the result, write `unverified`.
11. Make test instances and fresh clones in a temporary directory outside the
    project. Delete them after the audit.
12. Some Verify texts read a setting of the repository host. If the host says
    that the feature is not available to the project, the setting is off.
13. Give the answer of the host as the evidence. Write `unverified` for a host
    setting only when you cannot read it.
14. If a Verify examines a CI run on the default branch, use the latest run
    there. The run on the default branch decides.
15. If the audited commit has no CI run of its own, also run the check on the
    audited commit. Give both results in the evidence.
16. Before the first release, some Verify texts have no release to examine.
    Show with a command that no release exists, such as `git tag`. Then
    judge the release process at the audited commit in place of the release.
    The release process is the CI workflow or the documented steps that build
    and publish a release.
    - If the process makes what the rule requires, write `pass`. Examples are
      binaries, an SBOM, signatures, provenance and a notices file.
    - If it does not, or the project has no release process, write `fail`.
    - Examine each document now. Examples are the rebuild steps, the
      instructions to verify signatures and the definition of the public API.
    - If the Verify compares with a published artifact, use a substitute. For
      example, build twice from the audited commit. Compare the checksums.
      Name the substitute in the evidence.
17. Save the results file after each area, so that a stopped audit can
    continue.
18. Add each suppression in the checks of the project to `suppressions`, with
    `file`, `rule` and `reason`. A suppression turns off or relaxes a check
    for part of the code. It is a comment, an attribute or a setting.
19. A setting for the whole project is not a suppression. USS-QLT-006 and
    USS-QLT-007 examine it.
20. Add `line` when you know it. It is the line of the comment, the attribute
    or the setting.
21. Write the rule of the check as `rule`, such as `clippy::unwrap_used`. Do
    not write a USS rule ID.
22. If the project gives no reason for a suppression, write
    "no reason given".

#### Finish the audit

1. Compare the rule IDs in the results file with the IDs that
   `uss applicable .uss/record.yaml` lists. Give each rule that applies one
   result.
2. A rule that applies with no result counts as `unverified`. The exception
   is a MAY rule that the record declines: it counts as `declined`.
3. Add the audit to `audits` in the record, with `date`, `results` and
   `report`. If `audits` already lists that results file, keep its one entry.
4. Run `uss check .uss/record.yaml`. Correct each problem that it reports.
   `uss check` does not examine the content of the results file.
5. Run `uss report .uss/audits/<date>.yaml`. It examines the results file. If
   it reports a problem, correct the results file. Run it again.
6. `uss report` writes the report, then prints the compliance statement and
   the scores. Do not edit the report.
7. Commit the record, the results file and the report together. Show the
   statement and the scores to the developer.

#### Fix the project

1. Fix the project only when the developer tells you to. Fix the MUST and
   MUST NOT failures first.
2. Then fix the SHOULD and SHOULD NOT failures that the developer chose to
   comply with. Fix one area at a time.
3. Apply the Verify of the rule to each fix. Change the result to `pass`
   only when the Verify passes.
4. After each area, commit the fixes. Set `commit` to the new SHA that
   `git rev-parse HEAD` prints.
5. Set `date` to the date of today. If the date changed, rename the results
   file and the report. Change their entry in `audits` to match.
6. Audit again each rule that you fixed, and each rule whose evidence the
   fixes changed. Correct each `file:line` that a fix moved.
7. Run `uss report .uss/audits/<date>.yaml` again. Commit the record, the
   results file and the report together.

### Projects with several released units

A repository can hold units that the project releases separately. Each unit
has its own record (conventions section 11, "Components").

1. Write a record for each unit at `<unit>/.uss/record.yaml`, with its own
   stack, facts, decisions, Standard Deviations and audits.
2. Write the paths in the record of a unit relative to the directory of the
   unit.
3. Pin the same USS version in every record.
4. In the root record, list the directory of each unit in `components`, such
   as `components: [libs/core, cli]`.
5. Adopt or audit each record on its own. Run the workflow from the directory
   of the unit.
6. Run `uss check .uss/record.yaml` from the repository root. It also checks
   the record of each unit.

The root record covers the rules for the whole repository, such as CI, the
supply chain, `SECURITY.md` and the licence. The repository is compliant when
the root record and the record of each unit are compliant. `uss release-check`
examines the latest audit of every record.

### Release audit

Each release needs an audit of the release commit, and its report goes with
the release. Do the audit as in "Audit an existing project", with these
differences:

1. If the project has a record and the scan agrees with every fact in it, ask
   nothing.
2. To release from your computer, audit the commit to release. Commit the
   record, the results file and the report.
3. Run `uss release-check`. It examines the latest audit of each record. It
   writes the reports to `uss-report.md`.
4. Delete `uss-report.md` after you read it. Do not commit it.
5. Put the release tag on the commit that holds the results.

In CI, the workflow from `templates/ci/uss-release.yml` runs
`uss release-check` when the project publishes a release. It attaches the
report to the release. A CI job that runs `uss release-check` needs the full
Git history: with `actions/checkout`, set `fetch-depth: 0`.
`uss release-check --require-compliant` also fails when a record is not
compliant.

**With no person present**, such as an agent in CI, use these rules in each
audit:

1. Ask nothing.
2. Do not change `facts`, `decisions` or `deviations` in the record.
3. If `uss check` reports a problem that needs such a change, stop. Report
   the problem.
4. Write each open question as an `unverified` result on each rule that
   applies and depends on it. Write the question as the evidence.
5. If no rule that applies depends on an open question, write `unverified`
   on USS-REL-011, "Audit each release". Write the question as the evidence.
   The audit is not complete until a person answers the question.
6. If USS-REL-011 does not apply to the record, stop. Report the question.

Each `unverified` result makes the report say "Not compliant", until a
person answers its question.

### Upgrade the pinned version

Do the upgrade with these steps. They use "Audit an existing project" for the
audit.

1. Do the steps in "Before every workflow". Use the new version as the USS
   version.
2. Read `uss show CHANGELOG.md`. List the rule IDs that each version after the
   old one added, changed or retired, up to the new version.
3. Make a branch. Do the next steps on that branch. `uss` refuses a record
   that pins another version, so the upgrade happens on a branch.
4. Change `standard.version` to the new version in every record.
5. If the project has `.github/workflows/uss-release.yml`, change its
   `USS_VERSION` to the new version. Then compare its `USS_RUST_TOOLCHAIN`
   with the value in `uss show templates/ci/uss-release.yml`. Keep the newer
   of the two values.
6. Run `uss check .uss/record.yaml`. It names each new fact that the record
   does not answer. It also names each fact that the new version does not
   have.
7. Answer each new fact. Remove each fact that the new version does not have.
   Ask the developer to confirm the new answers.
8. Remove each decision and each entry in `deviations` for a retired rule.
9. Ask questions only about the added and changed rules that apply.
10. Run `uss check .uss/record.yaml`. Correct each problem that it reports.
11. Commit the records and the files that you changed.
12. Do the steps in "Audit an existing project". Skip "Before every workflow",
    because you did it at the start. Audit every rule that applies.
13. Merge the branch when the project is compliant with the new version. If
    the project was not compliant before the upgrade, merge the branch when
    the developer accepts the new report.

## Agent conduct

These rules bind every agent that follows these workflows. Audits do not count
them, because they leave nothing in the repository to examine.

- Do not reduce the scope of a task without telling the developer.
- Report only results that you saw from commands that you ran. The result in
  CI decides whether a check passes.
- Do not change files, settings or systems outside the task.
- Do not turn off a check or skip a test to make it pass.

## Work on this repository

This section is for agents that change USS: the standard, the schemas, the
templates and the `uss` tool.

### Set up

1. Install rustup from <https://rustup.rs>.
2. Clone the repository. Run all commands from its root.
3. Run `rustup toolchain install`. It installs the toolchain that
   `rust-toolchain.toml` pins.
4. Run `git config core.hooksPath .githooks`. It turns on the pre-commit hook.
5. Run `cargo xtask check`. The first run also installs cargo-deny under
   `.tools`.

### Commands

- `cargo xtask check` sets up and runs every check that CI runs.
- `cargo build --locked` builds the workspace.
- `cargo test --locked -p uss` runs the tests of `uss`.
- `cargo run --locked -p uss -- lint` checks the standard files.
- To follow a workflow on this repository, use `cargo run --locked -q -p uss --`
  in place of `uss`.

### Conventions

- Follow `standard/conventions.md` when you write or change a rule.
- Do not reuse or renumber a rule ID. A new rule takes the next free number at
  the end of its file.
- Keep the version at 0.1.0 until the first release.
- Do not change `Initial_Idea.md` or `todo.txt`. They are the maintainer's
  local notes, and Git ignores them.
- Write commit messages as `type(scope): summary`, as in `git log`.
- Write steps in the ASD-STE100 style (USS-DOC-007). Use one instruction in
  each sentence, the imperative and the active voice. Use at most 20 words in
  a sentence of a step, and 25 in other sentences.
- Use UTF-8 and LF line endings. Wrap prose at 80 columns.

### Definition of Done

A change is done when all of these are true:

- `cargo xtask check` passes.
- Each change in behaviour has tests.
- When a command or an option changes, each document that names it changes in
  the same commit.
- You read the source of each reference that you added or changed.
