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
  the stack, facts, decisions, deviations and audits of the project.
- A **results file** is `.uss/audits/<date>.yaml`. It holds the result and the
  evidence for each rule in one audit.
- A **report** is `.uss/audits/<date>.md`. `uss report` writes it from a
  results file.
- A **rule that applies** is a rule that `uss applicable .uss/record.yaml`
  lists.
- The **developer** is the person who owns the project.

### Before every workflow

1. Go to the project root. Run all commands in these workflows from there.
2. Find the USS version. It is `standard.version` in the record. With no
   record, it is the version of the installed `uss`. With no record and no
   `uss`, use the latest USS release.
3. Run `uss version`. Compare the version that it prints with the version
   that you found.
4. If `uss` is not installed, or the versions are different, install that
   version:

   ```sh
   cargo install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v<version> uss
   ```

   In the root of a USS checkout at that version, you can run
   `cargo install --locked --path tools/uss` instead.
5. Read the workflows of that version with `uss show AGENTS.md`. If they are
   different from the copy that you read, follow them.
6. Read sections 5, 8, 9 and 11 of the conventions with
   `uss show standard/conventions.md`.
7. If the project has a record, read the rules that apply with
   `uss applicable --full .uss/record.yaml`.
8. Do not calculate applicability, scores or compliance yourself. Only `uss`
   calculates them. If you cannot install `uss`, stop. Tell the developer why.

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
4. In an audit, tell the developer what the scan found for each rule.
5. For each SHOULD or SHOULD NOT rule that the developer declines, ask for the
   reason.
6. Write a draft Standard Deviation with that reason at
   `docs/standard-deviations/<RULE-ID>.md`. Start from
   `uss show templates/standard-deviation.md`.
7. Do not write the approver in the document or `approved-by` in the record.
   The developer gets the approval under the policy of their organisation.
8. After the approval, the developer adds the deviation to `deviations` in the
   record. Until then, the rule has no deviation.
9. If the developer wants a deviation from a MUST or MUST NOT rule, write it
   in the same way. Tell the developer that a MUST deviation makes the
   project "not compliant".

### Adopt a new project

1. Do the steps in "Before every workflow".
2. Do the steps in "Start the record", below.
3. Add the output of `uss show templates/AGENTS.snippet.md` to `AGENTS.md` in
   the project root. If the project has a `CLAUDE.md` and no `AGENTS.md`, add
   it to `CLAUDE.md`.
4. If the project publishes releases on GitHub, copy the output of
   `uss show templates/ci/uss-release.yml` to
   `.github/workflows/uss-release.yml`. Follow the comments in the file.
5. Commit the record, the draft deviations and the files that you changed.
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
4. Answer every fact from the project description. The `false` values of the
   template are placeholders, not answers. Do not add or remove a fact.
5. Ask the developer to confirm the facts, as "Questions to the developer"
   says.
6. Run `uss applicable .uss/record.yaml`.
7. Ask about the SHOULD, SHOULD NOT and MAY rules that apply.
8. Write each MAY answer in `decisions`, as `<RULE-ID>: { adopt: true }` or
   `<RULE-ID>: { adopt: false }`. Write no decision for a rule of another
   level.
9. If the developer adopts a MAY rule, run `uss applicable .uss/record.yaml`
   again. Ask about the new rules only. Do this until no new rules apply.
10. Run `uss check .uss/record.yaml`. Correct each problem that it reports.
    Run it again until it reports no problems.

### Audit an existing project

1. Do the steps in "Before every workflow".
2. Scan the project. Find its languages, frameworks, build tools, CI,
   containers, UI, databases, authentication, network use, releases and
   documents. Write down the obvious findings.
3. If the project has no record, do the steps in "Start the record", under
   "Adopt a new project". Take the answers from the scan.
4. If the project has a record, compare its facts with the scan. Ask the
   developer about each fact that the scan does not agree with.
5. If a fact changes, run `uss applicable .uss/record.yaml`. Ask about the
   new rules only.
6. Commit the record if you changed it.
7. Make sure that `git status --porcelain` prints nothing. If it prints
   changes, ask the developer to commit them.
8. Start the results file `.uss/audits/<date>.yaml`, with the date of today.
   Set `standard.version` to the version that the record pins.
9. Set `commit` to the full 40-character SHA that `git rev-parse HEAD`
   prints. Set `auditor` to your name. For example:

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

10. Audit the rules that apply, one area at a time. For each rule, apply its
    Verify exactly as written. Add nothing to it. Leave nothing out.
    `uss applicable --full .uss/record.yaml` prints the Verify of each rule.
11. Give each rule one result:
    - `pass`: the project meets the rule.
    - `fail`: the project does not meet the rule, and the record has no
      deviation for it.
    - `deviation`: the project does not meet the rule, and the record has a
      deviation for it.
    - `declined`: the record declines the MAY rule (`adopt: false`).
    - `unverified`: you cannot decide. Do not guess. Write the reason as the
      evidence.
12. Give evidence for each `pass`, `fail` and `deviation` result. Name a
    `file:line`, a configuration key, a CI job, or a command and its output.
    For `deviation`, name the deviation document.
13. Write results only for the rules that apply. Do not write
    `not-applicable` for a rule that applies.
14. If a Verify examines each release, and no release exists yet, write
    `pass`. Give the evidence "no release yet". If the Verify needs a
    published artifact, such as binaries, an SBOM, signatures or provenance,
    write `fail`.
15. Save the results file after each area. A rule that applies with no result
    counts as `unverified`. After an interruption, continue at the first area
    with no results.
16. Add each suppression in the checks of the project to `suppressions`, with
    `file`, `line`, `rule` and `reason`. The `rule` is the rule of the check,
    such as `clippy::unwrap_used`, not a USS rule ID.
17. If the code gives no reason for a suppression, write "no reason given".
18. Add the audit to `audits` in the record, with `date`, `results` and
    `report`.
19. Run `uss check .uss/record.yaml`. Correct each problem that it reports.
    `uss report` refuses a record with problems.
20. Run `uss report .uss/audits/<date>.yaml`. It writes the report, then
    prints the compliance statement and the scores. Do not edit the report.
21. Commit the record, the results file and the report together. Show the
    statement and the scores to the developer.
22. Fix the project only when the developer tells you to. Fix the MUST and
    MUST NOT failures first.
23. Then fix the SHOULD and SHOULD NOT failures that the developer chose to
    comply with. Fix one area at a time.
24. Apply the Verify of the rule to each fix. Change the result to `pass`
    only when the Verify passes.
25. After each area, commit the fixes. Set `commit` to the new SHA that
    `git rev-parse HEAD` prints.
26. Audit again each rule whose evidence the fixes changed.
27. Run `uss report .uss/audits/<date>.yaml` again. Commit the record, the
    results file and the report together.

### Projects with several released units

A repository can hold units that the project releases separately. Each unit
has its own record (conventions section 11, "Components").

1. Write a record for each unit at `<unit>/.uss/record.yaml`, with its own
   stack, facts, decisions, deviations and audits.
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
3. Run `uss release-check`. It examines the latest audit of each record. Do
   not commit `uss-report.md`, the file that it writes.
4. Put the release tag on the commit that holds the results.

In CI, the workflow from `templates/ci/uss-release.yml` runs
`uss release-check` when the project publishes a release. It attaches the
report to the release. A CI job that runs `uss release-check` needs the full
Git history: with `actions/checkout`, set `fetch-depth: 0`.
`uss release-check --require-compliant` also fails when a record is not
compliant.

**With no person present**, such as an agent in CI, use these rules in each
audit:

1. Ask nothing.
2. Do not change a fact, a decision or a deviation in the record.
3. If `uss check` reports a problem that needs such a change, stop. Report
   the problem.
4. Write each open question as an `unverified` result on each rule that
   applies and depends on it. Write the question as the evidence.
5. If no rule that applies depends on an open question, write `unverified`
   on USS-REL-011, "Audit each release". Write the question as the evidence.
   The audit is not complete until a person answers the question.
6. If USS-REL-011 does not apply to the record, stop. Report the question.

The report then says "Not compliant" until a person answers each question.

### Upgrade the pinned version

Do the upgrade as in "Audit an existing project", with these differences:

1. Install `uss` at the new version, as "Before every workflow" says.
2. Read `uss show CHANGELOG.md`. List the rule IDs that each version after the
   old one added, changed or retired, up to the new version.
3. Make a branch. Do the next steps on that branch. `uss` refuses a record
   that pins another version, so the upgrade happens on a branch.
4. Change `standard.version` to the new version in every record.
5. If the project has `.github/workflows/uss-release.yml`, change its
   `USS_VERSION` to the new version.
6. Run `uss check .uss/record.yaml`. It names each new fact that the record
   does not answer. It also names each fact that the new version does not
   have.
7. Answer each new fact. Remove each fact that the new version does not have.
   Ask the developer to confirm the new answers.
8. Remove each decision and each deviation for a retired rule.
9. Ask questions only about the added and changed rules that apply.
10. Run `uss check .uss/record.yaml`. Correct each problem that it reports.
11. Do a full audit.
12. Merge the branch when the project is compliant with the new version. If
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
4. Run `cargo xtask check`. The first run also installs cargo-deny under
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
- Do not change `Initial_Idea.md` or `todo.txt`.
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
