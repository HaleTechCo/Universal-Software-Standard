# OpenSSF Scorecard

- **Kind:** profile
- **Product:** OpenSSF Scorecard
- **Vendor:** OpenSSF
- **Maintainer:** community
- **Versions:** >=5.7.0, <6.0.0

OpenSSF Scorecard scores the practices of a repository, not its code. Each of
its checks (such as Pinned-Dependencies, Branch-Protection and
Token-Permissions) gives a score from 0 to 10, mostly from the repository host's
API and from the files in the repository. No module recommends it: the
specification names it (section 6.6), and the Supply chain rules cite its checks
as evidence. The checks and scores that the mappings name are those of Scorecard
5.7.0, from its check documentation (`docs/checks.md`) and its source, and the
action's inputs are those of `ossf/scorecard-action` 2.6.0 (README and
`action.yaml`), all read 2026-11-24. The baseline below runs the Scorecard
GitHub Action, pinned by commit (USS-SUP-003), on pushes to the default branch
and on a weekly schedule, and publishes the results. Put it in
`.github/workflows/scorecard.yml`, and set `branches` to the default branch. The
baseline has no `branch_protection_rule` trigger, which the action's example has
for the Branch-Protection check: a change to branch protection is then scored at
the next push or weekly run. The action's README lists only `push` and
`schedule` as supported triggers, so the baseline keeps to those.

A check supports a mapped rule only at the score that its mapping row names; a
lower score leaves the rule to be judged as the rule says. A score is also only
as good as what the check can see, so each row says what the check does not see.
Pinned-Dependencies also counts download-then-run findings (such as `curl |
sh`), which lower the score that the USS-SUP-003, USS-SUP-004, USS-DEP-003 and
USS-TS-010 rows read; USS-SUP-005 has no row, because the check never sees a
checksum. Scorecard is not a gate: the action has no input that fails the job on
a low score, so the score is read in the viewer
(`https://scorecard.dev/viewer/?uri=github.com/<owner>/<repo>`, which needs
`publish_results: true`) or in the SARIF file in code scanning. Several checks
read GitHub only (the documentation says so for Pinned-Dependencies,
Token-Permissions, SAST, CI-Tests and Signed-Releases), so a project on another
host judges the rules below as they say. Scorecard reads Repository Rules with
the default token, and classic branch protection only with the extra token
permissions that the action's authentication document describes (`repo_token`).
A private repository adds the read permissions that the action's README lists.
The checks Binary-Artifacts, CII-Best-Practices, Contributors, Maintained,
Packaging and Webhooks have no row, because none of them tests what a USS rule
requires.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```yaml
# .github/workflows/scorecard.yml
# A check supports a mapped USS rule only at the score that the rule's row in
# the profile names. Set `branches` to the default branch.
name: Scorecard supply-chain security

'on':
  push:
    branches: ['main']
  schedule:
    - cron: '15 14 * * 0'

permissions: {}

jobs:
  analysis:
    name: Scorecard analysis
    runs-on: ubuntu-latest
    # Scorecard supports only the default branch.
    if: ${{ github.event.repository.default_branch == github.ref_name }}
    permissions:
      security-events: write  # Upload the SARIF file to code scanning.
      id-token: write  # Sign the results that are published.
    steps:
      - name: Checkout code
        uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1  # v7.0.1
        with:
          persist-credentials: false

      - name: Run analysis
        uses: ossf/scorecard-action@23f22cd43608bae7a1f1fa2603d7314dee9538a5  # v2.6.0
        with:
          results_file: results.sarif
          results_format: sarif
          publish_results: true

      - name: Upload artifact
        if: ${{ always() }}
        uses: actions/upload-artifact@cf430e030ddbb5b0abf93d22962f4752f3646cd9  # v7.0.2
        with:
          name: SARIF file
          path: results.sarif
          retention-days: 5

      - name: Upload to code scanning
        if: ${{ always() }}
        uses: github/codeql-action/upload-sarif@33d7b85da1a04171e211befe2c739303b9a4c066  # v4.39.3
        with:
          sarif_file: results.sarif
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-DEP-003 | Safety | Pinned-Dependencies at 10 (probe `pinsDependencies`, dependency types `npmCommand` and `pipCommand`: it reports `npm install` and `npm update`, and a `pip install` without `--require-hashes`, and it accepts `npm ci` and `pip install --require-hashes`) | partial: the check reads npm, pip and Go commands, in workflows, shell scripts and Dockerfiles, so a pnpm, Yarn, Bun, uv, Cargo or Maven install is not seen; for `go get` and `go install` it reports a package that has no commit hash or full semantic version (`@latest` or no version), and it accepts one with a full version, which is a pinned version and not an install from `go.sum`, so it does not show a lockfile install; it does not show that CI fails when a pip lockfile is out of date; its score also counts every other dependency type, so a score under 10 can come from them | Read the Pinned-Dependencies score and its findings for npm, pip and Go commands in the viewer, and expect no such finding; judge each install step that the check does not read as the rule says. |
| USS-DEP-007 | Safety | Vulnerabilities at 10 (probe `hasOSVVulnerabilities`: Scorecard scans the repository recursively, and its commit, with the OSV-Scanner library and reports each open vulnerability that OSV knows, and the score is 10 minus their number) | partial: the baseline workflow scans on pushes to the default branch and weekly, not on each pull request; the check does not show that vendored code is covered through the record of USS-DEP-010, and it reads Python `requirements.txt` files without their transitive dependencies; a finding does not fail the job | Read the Vulnerabilities score in the viewer and expect 10; read the CI configuration for a dependency scan on each pull request, each push and a schedule. |
| USS-DEP-008 | Safety | Vulnerabilities at 10 (an open vulnerability lowers the score, and one that an `osv-scanner.toml` file next to the manifest ignores by ID is not reported) | partial: the check does not require a reason, an expiry date or an issue link for an ignore entry, and it reads no VEX statement, so the auditor reads each entry | Read the Vulnerabilities score in the viewer and expect 10; read each ignore entry for the reason, the expiry date and the issue link that the rule requires. |
| USS-DEP-014 | Safety | Dependency-Update-Tool at 10 (probe `dependencyUpdateToolConfigured`: it finds the configuration of Dependabot or Renovate, and scores 10 when it finds one and 0 when it finds none) | partial: the check does not show that the configuration covers every manifest, or that the tool opened the pull requests that were merged | Read the Dependency-Update-Tool score in the viewer and expect 10; read the configuration for each manifest and the merged update pull requests. |
| USS-SEC-028 | Safety | SAST at 10 (probes `sastToolConfigured` and `sastToolRunsOnAllCommits`: a workflow that uses `github/codeql-action/analyze`, Snyk, Pysa, Qodana or SonarCloud, or a check run of GitHub code scanning or SonarCloud on the recent merged pull requests) | partial: the check sees only those tools, so a project that runs another tool (such as Semgrep or gosec) scores low; a workflow that uses CodeQL can give 10 without showing that it runs on each pull request and each push, and the check does not see that a finding fails the CI check or that each suppression has a reason | Read the SAST score in the viewer and expect 10; read the CI configuration for the tool, its triggers and the way a finding fails the check, and read each suppression. |
| USS-SEC-029 | Safety | Security-Policy at 10 (a `SECURITY.md` file in a well-known place: 6 points for an email address or an http or https link, 3 for free text beyond links, 1 for text that matches `vuln` and `disclos` or a time such as 30 days) | partial: the check does not show that the reporting channel is private, and it judges what happens after a report only by the words in the file | Read the Security-Policy score in the viewer and expect 10; read the file for a private reporting channel and for what happens after a report. |
| USS-SUP-003 | Safety | Pinned-Dependencies at 10 (probe `pinsDependencies`, dependency type `GitHubAction`: it reports a `uses:` reference that is not a full commit SHA, and a `docker://` reference without a sha256 digest) | partial: GitHub workflows only, so a GitLab include or component is not seen; the check counts a tag that is backed by an immutable release as pinned, which the rule treats as a failure; it does not read `.github/workflows/actions.lock`, so a tag reference that the lockfile pins is reported as unpinned; its score also counts other dependency types | Read the Pinned-Dependencies score and its GitHubAction findings in the viewer; for each reported or accepted tag reference, check the SHA or the lockfile entry as the rule says. |
| USS-SUP-004 | Safety | Pinned-Dependencies at 10 (probe `pinsDependencies`, dependency type `containerImage`: it reports a `FROM` line in a Dockerfile whose image has no digest) | partial: the check reads Dockerfiles and `docker://` action references only, so Compose files, Kubernetes manifests, Helm charts and other CI job images are not seen, and it does not know which images the project builds itself, which the rule leaves out | Read the Pinned-Dependencies score and its containerImage findings in the viewer; search the other files for image references and read each digest. |
| USS-SUP-007 | Safety | Branch-Protection at 3 or more (tier 1: force pushes and branch deletion are prevented, 3 points) | partial: the check reads the default branch and the release branches, not release tags; classic branch protection needs the extra token permissions of the action's authentication document, while Repository Rules are readable with the default token | Read the Branch-Protection score in the viewer and expect at least 3; read the host's tag protection or tag ruleset for the release tags. |
| USS-SUP-008 | Safety | Branch-Protection at 8 or more (tier 3, 8 points: the branch requires at least one passing status check; tiers 1 and 2 must be met first) | partial: the check does not say which status checks are required, so the auditor reads whether they include each CI check that the standard requires; tier 2 needs a required reviewer, so a project with no reviewer requirement scores under 8 whatever its status checks | Read the Branch-Protection score and its required-status-check line in the viewer; read the required checks against the CI jobs that the standard's rules require. |
| USS-SUP-009 | Safety | Code-Review at 10 (probe `codeApproved`: each recent changeset on the default branch has an approving review from someone other than its author, or a merger other than its author, and the score is the share of such changesets) | partial: the check samples about the last 30 commits, so it does not show the host setting that requires an approval before merge; a merger who is not the author counts as an approval | Read the Code-Review score in the viewer and expect 10; read the host setting that requires an approval, and, without one, the approvals of every change since the last audit, or the last 12 months when there is none. |
| USS-SUP-010 | Safety | Token-Permissions at 10 (probes `topLevelPermissions` and `jobLevelPermissions`: each workflow sets read-only permissions at the top level and declares write permissions only in the jobs that need them; no top-level definition costs 1 point) | partial: the check cannot see the repository's default workflow permissions setting, and a job-level write permission (such as `contents` or `packages`) only gives a warning, so it does not show that a job declares only the permissions that it needs | Read the Token-Permissions score and its warnings in the viewer and expect 10; read the repository's default workflow permissions setting and each job's permissions. |
| USS-SUP-011 | Safety | Dangerous-Workflow at 10 (probe `hasDangerousWorkflowUntrustedCheckout`: it reports a `pull_request_target` or `workflow_run` workflow that checks out a ref that holds `github.event.pull_request` or `github.event.workflow_run`; the score is 10 or 0) | partial: the check does not decide whether a reported checkout is safe, and it sees only an explicit `ref` of those forms | Read the Dangerous-Workflow score in the viewer and expect 10; read each privileged trigger for a checkout of the change's code. |
| USS-SUP-012 | Safety | Dangerous-Workflow at 10 (probe `hasDangerousWorkflowScriptInjection`: it reports a script that holds an untrusted context expression, such as `github.head_ref` or a listed `github.event.*` field; the score is 10 or 0) | partial: the check knows a fixed list of expressions, so another untrusted value in a script is not seen, and it does not show that such data reaches a script only through an environment variable or a file | Read the Dangerous-Workflow score in the viewer and expect 10; read each script for untrusted event data that is not on the list. |
| USS-SUP-013 | Safety | SBOM at 10 (probes `hasSBOM` and `hasReleaseSBOM`: a file whose name ends in `.cdx.json`, `.cdx.xml`, `.spdx`, `.spdx.json`, `.spdx.xml`, `.spdx.yaml`, `.spdx.rdf` or `.spdx.rdf.xml`, among the assets of the five latest releases) | partial: the check matches the file name and does not read the content, so it does not show the components or the dependency relationships, and one such file in one of the five releases gives 10, so the auditor checks each release | Read the SBOM score in the viewer and expect 10; open the SBOM of each release for the components and the direct dependencies. |
| USS-SUP-014 | Safety | Signed-Releases at 8 or more (probes `releasesAreSigned` and `releasesHaveProvenance`: a release asset that ends in `.asc`, `.minisig`, `.sig`, `.sign`, `.sigstore` or `.sigstore.json` gives 8 for that release, and SLSA provenance gives 10; the score is the mean over the five latest releases) | partial: the check looks for one signature file per release and not one for each artifact, it does not verify a signature, it sees no signature that is held in a registry, and it counts a GitHub immutable release as provenance, so a 10 can rest on that alone; because the score is a mean, one unsigned release can hide among higher ones | Read the Signed-Releases score and its per-release lines in the viewer; a score of 8 or more counts only when each release's line shows a signature file (an immutable release scores 10 without one); check that each artifact of each release has a signature and read how it is verified. |
| USS-TS-010 | Safety | Pinned-Dependencies at 10 (probe `pinsDependencies`, dependency type `npmCommand`: it reports `npm install` and `npm update`, and it accepts `npm ci`) | partial: the check reads npm commands only, so a pnpm, Yarn or Bun install is not seen, and it does not read the lock file or show that it is current | Read the Pinned-Dependencies score and its npmCommand findings in the viewer, and expect no such finding; judge each other install step as the rule says. |
| USS-TST-001 | Reliability | CI-Tests at 10 (probe `testsRunInCI`: each of the recent merged pull requests has a successful check run or status whose name holds one of appveyor, buildkite, circleci, e2e, github-actions, jenkins, mergeable, test, travis-ci or woodpecker) | partial: the check reads names, not what a job runs, so it does not show that the whole suite runs, that a failing test fails the job, or that the job runs on each push to the default branch; a project whose job names are not on the list can score low | Read the CI-Tests score in the viewer and expect 10; read the CI configuration for the whole suite and for the failure of the job on a failing test. |
| USS-TST-011 | Reliability | Fuzzing at 10 (probe `fuzzed`: the repository is in the OSS-Fuzz project list, it deploys ClusterFuzzLite, or it holds Go fuzz functions or a listed property-based test library; the score is 10 or 0) | partial: one fuzzer of any parser gives 10, so the check does not show a target for each parser or that sanitizers are on, and it does not see a libFuzzer, AFL++ or cargo-fuzz target that is not run through OSS-Fuzz or ClusterFuzzLite | Read the Fuzzing score in the viewer; list the parsers of untrusted input and read the fuzz target, the sanitizers and a recent run for each. |
| USS-TST-012 | Reliability | Fuzzing at 10 (probe `fuzzed`, as for USS-TST-011) | partial: one fuzzer of any parser gives 10, so the check does not show a target for each parser, and it does not see a fuzz target that is not run through OSS-Fuzz or ClusterFuzzLite or written with Go fuzzing or a listed property-based library | Read the Fuzzing score in the viewer; list the parsers of untrusted input and read the fuzz target and a recent run for each. |
| USS-LIC-001 | Compatibility | License at 9 or more (a file named `LICENSE`, `LICENCE`, `COPYING` or `COPYRIGHT`, or files in a `LICENSES` directory, gives 6 points, the top-level directory gives 3, and an FSF or OSI licence gives 1) | partial: the check does not read the licence field of any package manifest or show that it is a valid SPDX expression, and the licence text is recognised through the GitHub licence API or the file name | Read the License score in the viewer and expect at least 9; read the licence field of each package manifest for an SPDX expression. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| Every check runs | The action has no input that selects a check or turns one off (the inputs of `action.yaml` in 2.6.0 are `results_file`, `results_format`, `repo_token`, `publish_results`, `file_mode`, `fail_on_publish_error` and two internal ones), and the baseline sets none | USS-QLT-006 |
| A suppression names its checks and gives a reason | Each `annotations` entry in `scorecard.yml` (the tool also reads `scorecard.yaml`, `.scorecard.yml`, `.scorecard.yaml`, `.github/scorecard.yml` and `.github/scorecard.yaml`, and uses the first that it can open) holds `checks`, the lower-case check names, and `reasons`, each with a `reason` that is `test-data`, `remediated`, `not-applicable`, `not-supported` or `not-detected`; a file with another check name or reason is ignored whole, with the log line "couldn't parse maintainer annotations" | USS-QLT-009 |
| No blanket suppression | No annotation entry lists every check, or a check that the reason does not explain; the tool accepts such an entry (its test file `all_checks.yml`) | USS-QLT-010 |

An annotation adds context that is displayed beside the result (the
annotations README says so), and its documentation does not say that it
changes the score, so the auditor still reads the score, and an annotation
does not lift a check to the score that a mapping row names. The
documentation describes no setting that reports an annotation that no longer
applies, so the auditor reads each annotation against the check's current
finding.
