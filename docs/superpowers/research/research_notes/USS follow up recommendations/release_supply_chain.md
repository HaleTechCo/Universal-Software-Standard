# USS 0.1.0 release, install steps and trial: supply-chain and workflow recommendations

Read 2026-11-25. Tool-call budget was reached before every item was covered. Items marked "(not checked)" rest on my prior knowledge, not on a source read today, and need a check before the release plan relies on them. Source reads are by WebFetch (summaries from a small model; quotes below are as returned). Tool versions come from `gh api repos/<tool>/releases/latest` on 2026-11-25.

Facts about this repository (read from files / `gh api -X GET repos/:owner/:repo`): the repository is private, owned by an Organization. `Cargo.toml` has `rust-version = "1.100"`, `rust-toolchain.toml` has `channel = "1.100.0"`, `templates/ci/uss-release.yml` has `USS_RUST_TOOLCHAIN: "1.100.0"`. CI matrix is `ubuntu-latest, windows-latest, macos-latest`. `CHANGELOG.md` holds only the line `# Changelog`.

Current tool versions (GitHub releases API, 2026-11-25): dist (cargo-dist) v0.34.0 (2026-11-02); cosign v3.1.6; cosign-installer v4.1.4; cargo-cyclonedx 0.5.10; cargo-about 0.9.3; cargo-auditable v0.7.8; syft v1.57.2; actions/attest v4.2.2 and actions/attest-build-provenance v4.2.2 (2026-08); slsa-github-generator v2.1.0 (2025-02-24, no release since); minisign 0.12 (2025-01); claude-code-action tag `v1` (floating, last release object 2025-08-26); cargo-binstall v1.28.0.

## 0. Headline: the private-repository constraint decides SUP-014, SUP-015 and the ARM builds

### Takeaway
GitHub artifact attestations on a private repository need GitHub Enterprise Cloud. The organisation is on the Free plan, so attestations (the cheapest route to USS-SUP-014 and USS-SUP-015) work only if the repository becomes public or the plan is upgraded. ARM64 GitHub-hosted runners are free only for public repositories. Making the repository public removes three blockers at once. If it stays private, use cosign keyless signing for SUP-014 and a hand-built matrix with cross-compilation or paid ARM minutes for DST-014; SUP-015 then has no cheap route.

### Cited Findings
- Attestations in private repos: "To use artifact attestations in private or internal repositories, you must be on a GitHub Enterprise Cloud plan." Also "not available on legacy plans". As of v4, attest-build-provenance "is simply a wrapper on top of `actions/attest`"; new work should use `actions/attest`. — [actions/attest-build-provenance README](https://raw.githubusercontent.com/actions/attest-build-provenance/main/README.md)
- A search result shows a newer docs page that says attestations are "available for repositories in all current GitHub plans" without the private-repo restriction. This conflicts with the README. The Enterprise Cloud statement is the more specific and is repeated by the dist docs and the `@actions/attest` package docs (via search summary). Treat as: not available on Free private; confirm on the page below before planning. — [GitHub docs, use artifact attestations](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations); [dist GitHub attestations page](https://raw.githubusercontent.com/axodotdev/cargo-dist/main/book/src/supplychain-security/attestations/github.md) ("only supports public repositories and private repositories of an organization with the GitHub Enterprise plan")
- Public repos use the Sigstore Public Good Instance (bundles go to a public transparency log). Private repos use GitHub's own Sigstore instance, which "does not have a transparency log and only federates with GitHub Actions". Attestations alone give "SLSA v1.0 Build Level 2"; Level 3 needs a reusable workflow. — [GitHub docs, artifact attestations concepts](https://docs.github.com/en/actions/concepts/security/artifact-attestations)
- ARM64 standard runners: `ubuntu-24.04-arm`, `ubuntu-22.04-arm`, `ubuntu-26.04-arm`, `windows-11-arm`, `windows-11-vs2026-arm`, `macos-latest`/`macos-15`/`macos-26` (Apple silicon). `macos-15-intel` and `macos-26-intel` are x64. Standard runners are "free and unlimited on public repositories"; for private repositories they "draw from your GitHub account's included compute usage, and are then charged at the per-minute rates." — [GitHub-hosted runners reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners). Note: the page text does not say whether Linux/Windows ARM runners are billed differently for private repositories; I did not find a statement either way.

### Inferences
- Simplest overall path for 0.1.0: make the repository public before the release (the standard is meant to be open; this also helps the Claude Code marketplace, item 14). Then attestations, free ARM runners and the public Sigstore instance all work. Effort S (a setting), but it is a decision for the maintainer and is irreversible in effect.
- Free-plan private fallback: SUP-015 would have no route that meets "provenance signed by the build platform" cheaply (see item 5).

### Gaps
- Exact plan-comparison page for attestations was not found; one docs page conflicts with the README. Verify by running a throwaway attestation in a private test repo, or read GitHub's pricing page.

## 1. Install defect (R10): AGENTS.md "Before every workflow" step 5

### Takeaway
Recommended fix: `rustup toolchain install 1.100.0 --profile minimal --no-self-update`, then `cargo +1.100.0 install --locked --git <repo> --tag v<version> uss`. `+toolchain` beats `rust-toolchain.toml`, a numeric version is a valid toolchain name, and installing a named version does not replace the stable toolchain. Both flags exist (the `--profile` flag was confirmed in the docs; `--no-self-update` was NOT confirmed in a page I could read, see Gaps).

### Cited Findings
- Toolchain names may be "a fully specified version number, such as `1.42.0`." — [rustup, Toolchains](https://rust-lang.github.io/rustup/concepts/toolchains.html)
- `--profile` on install: "You can also directly select the profile used when installing a toolchain with: `rustup install --profile <name>`." Minimal profile = "as few components as possible to get a working compiler (`rustc`, `rust-std`, and `cargo`)"; recommended for CI. "Profile choices affect only newly installed toolchains." — [rustup, Profiles](https://rust-lang.github.io/rustup/concepts/profiles.html)
- Precedence: 1 `+toolchain`, 2 `RUSTUP_TOOLCHAIN`, 3 directory override, 4 `rust-toolchain.toml`, 5 default. "The toolchain is chosen in the order listed above, using the first one that is specified." — [rustup, Overrides](https://rust-lang.github.io/rustup/overrides.html). Caution: `RUSTUP_TOOLCHAIN` set in the environment would beat nothing above `+`, so `+1.100.0` still wins; but a directory override cannot beat it either.
- The rustup installation page does not mention distro packages or `--no-self-update`. — [rustup, Installation](https://raw.githubusercontent.com/rust-lang/rustup/master/doc/user-guide/src/installation/index.md)
- Current repository state: `docs/troubleshooting.md` lines 79-86 repeat the `rustup toolchain install stable` / `+stable` advice and need the same change. The test `ci_template_is_hardened` (tools/uss/src/cli/mod.rs ~line 1840-1895) checks only that the template's `USS_RUST_TOOLCHAIN` is `>=` the crate's `rust-version` (compares numeric vectors: `[1,100,0] >= [1,100]`) and that each `cargo` line starts with `cargo "+${USS_RUST_TOOLCHAIN}" install `. It does not read AGENTS.md or `rust-toolchain.toml`. `tools/xtask/src/main.rs` also hard-codes 1.100.0 in test strings (those are fixtures, not the pin).

### Inferences
- Recommended approach (effort S, files: AGENTS.md step 5, docs/troubleshooting.md "`cargo install` fails", README if it names the command; one new test in tools/uss/src/cli/mod.rs): 
  1. `rustup toolchain install 1.100.0 --profile minimal --no-self-update`
  2. `cargo +1.100.0 install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v<version> uss`
  This leaves `stable` and the default untouched on a machine that already has a default. (Behaviour of "first toolchain installed becomes default when none exists" is rustup behaviour I know of but did not verify today; it only matters on a machine with no default, which is harmless.) The number must be "the `rust-version` of `uss` at the pinned USS version, or later"; the text should say: use the `rust-version` printed by the release's `Cargo.toml` and add `.0` (a `rust-version` like `1.100` is installed as `1.100.0`).
- Keeping the numbers equal: make `rust-version` (`1.100` in the root Cargo.toml) the source; add one test that reads, with `include_str!`, `rust-toolchain.toml`, `templates/ci/uss-release.yml`, AGENTS.md and docs/troubleshooting.md and asserts that each toolchain number it names is `rust-version + ".0"` (or `==` for the pin in AGENTS.md; keep `>=` for the CI template only if the maintainers want to allow a newer CI toolchain, otherwise `==`). Extend the existing `ci_template_is_hardened` test rather than adding a file. Match with a regex like `rustup toolchain install (\d+\.\d+\.\d+)` and `cargo \+(\d+\.\d+\.\d+) install`.
- Machine without rustup: the workflow already stops if it cannot install `uss`. Recommended text: if `rustup` is missing, ask the developer for permission to install it from https://rustup.rs with `--default-toolchain none` (so no default toolchain is set, nothing else is changed) or to use an existing `cargo` at `rust-version` or later (`cargo --version`); never run `apt`/`brew` `rustc` installs without asking; if neither is possible, stop (matches step 10). `--default-toolchain none` is documented on the rustup install page for the nightly flow ("install `rustup` without a toolchain, `--default-toolchain none -y`").
- Alternative (one line each): `cargo binstall uss` after release binaries exist (needs DST-002, no toolchain at all, cargo-binstall v1.28.0); download the release binary directly (best long term for agents, removes the whole step); `rustup run 1.100.0 cargo install ...` (same effect, longer).

### Gaps
- `--no-self-update` for `rustup toolchain install`: the docs page I tried (`cli-reference.html`) returned 404 and the other pages did not mention it. I believe the flag exists (it is a documented option of `rustup toolchain install`, and `rustup update`), but verify with `rustup toolchain install --help` on a machine with rustup before editing AGENTS.md. Also verify the default-toolchain behaviour with that same command.

## 2. The trial: README "Try USS before its first release"

### Takeaway
No step is likely to fail on a documented basis, but three need small edits: the marketplace command should be run from the shell (`claude plugin marketplace add <path>`) or in-session with a path that works on Windows; the install id `uss@universal-software-standard` must equal marketplace `name`; `claude plugin validate --strict` should be added as a pre-step.

### Cited Findings
- Local path marketplace: `claude plugin marketplace add ./my-marketplace` (shell) and `/plugin marketplace add ./my-marketplace` (in session) both register a directory. For a local directory with a relative-path `source`, "Claude Code reads the plugin's files directly"; edits "take effect at the next session start or when you run `/reload-plugins` in a session, with no change to the plugin's `version`." — [Create a marketplace](https://code.claude.com/docs/en/plugin-marketplaces)
- Install id is `<entry name>@<marketplace name>`; entry name must equal the `name` in `plugin.json`, otherwise `Plugin "<manifest-name>" not found in marketplace`. In-session `/plugin install x@y` "opens the plugin's details in the `/plugin` panel, where you install it" (so it is a two-step action, not a silent install). The shell form `claude plugin install x@y` installs directly. — same page
- Validation: `claude plugin validate ./path` and `--strict` (warnings fail, for CI). — [Publish and distribute](https://code.claude.com/docs/en/plugins/publish.md)
- Marketplace name is checked: a name reserved for Anthropic's official marketplaces (e.g. `claude-plugins-official`) is refused. — Create a marketplace page.
- Local relative `source` must start inside the marketplace root, no `..`; a source directory that does not exist fails at install with `Source path does not exist`. — same page.

### Inferences
- Windows: I found no Windows-specific path caveat in the docs; a quoted absolute path (`C:\...`) or `.` from the checkout is the safe form. I could not test it here (not checked on macOS/Linux either). `cargo install --locked --path tools/uss --force` is standard Cargo behaviour (not re-read today). On Windows, a running `uss.exe` cannot be replaced; close agent sessions that run `uss` before `--force` (not checked, widely seen). Add that sentence to the README.
- Maintainer measurements for the trial (recommend writing these down in a short log; effort S): (1) wall time and number of questions in `/uss:audit` on one repository, and how many were answered "comply" by default; (2) the number of `unverified` results and why (missing tool, host setting, no person); (3) every command in AGENTS.md that failed or needed a correction (counts toward USS-CTB-007); (4) install time of `uss` on a clean machine and on the maintainer's (compile minutes; this is the case for shipping binaries); (5) whether the agent invented any applicability or score instead of calling `uss` (agent conduct); (6) token/cost for one full audit, and for one resumed audit; (7) false fails (rule judged `fail` but the maintainer disagrees) and Verify texts that were ambiguous; (8) records/reports produced: does `uss check` and `uss report` pass first time; (9) the maintainer's own judgement of the report's usefulness. Each failed step becomes an issue in follow-ups section 1.

### Gaps
- I did not test any step on a real machine; the claim "works on Windows, Linux and macOS" in the README is untested here.
- `/reload-plugins` and `--force` were not separately re-verified; only the marketplace page sentence about `/reload-plugins` was.

## 3. USS-DST-002 and USS-DST-014: binaries for x86-64 and ARM64

### Takeaway
Recommended: dist v0.34.0 (project now named "dist", config key `cargo-dist-version` still used) with six targets, because it also gives checksums, the install scripts, `github-attestations`, `cargo-auditable` and `cargo-cyclonedx` in one generated workflow. Effort M. It needs a public repository for free ARM runners and for attestations.

### Cited Findings
- dist describes itself as "dist distributes your binaries"; installers include shell, PowerShell, npm, Homebrew, MSI, updater; supply-chain section includes GitHub Attestations; checksums supported. — [dist book](https://axodotdev.github.io/cargo-dist/book/)
- Config keys: `targets` ("list of target platforms"), `github-attestations` (experimental), `cargo-auditable`, `cargo-cyclonedx`, `github-custom-runners` (map a target to a runner, example is ARM Linux), `checksum` (sha256 default options), `github-build-setup`, `allow-dirty` (advised against), `cargo-dist-version` (mandatory, written by `dist init`). — [dist config reference](https://raw.githubusercontent.com/axodotdev/cargo-dist/main/book/src/reference/config.md)
- Latest dist release v0.34.0 (2026-11-02). — GitHub releases API.
- ARM runner labels: see item 0.

### Inferences
- Target triples (recommended set, six): `x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc` (runner `windows-11-arm`), `x86_64-apple-darwin` (runner `macos-15-intel`), `aarch64-apple-darwin` (`macos-latest`), `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`. musl is chosen because DST-002 requires "no language runtime or library to install first (except those of the operating system)": a statically linked musl binary has no glibc version floor. gnu targets would need a documented minimum glibc. Check that `uss` dependencies build on musl (they are pure Rust: clap, jsonschema, minijinja, regex, serde, serde-saphyr; no C library seen in Cargo.toml) (musl link not tested). dist's own default for Linux is gnu; musl needs an explicit `targets` entry and, for aarch64-musl, cross tooling or an ARM runner (not checked).
- DST-014 Verify: for each OS the documentation says is supported, the release must hold both x86-64 and ARM64 artifacts. List exactly the six above in the README "supported platforms" and nothing else; an unlisted OS cannot fail.
- Private-repo cost: ARM runners and macOS runners bill against the plan's included minutes on a private repository. Alternative if private: cross-compile `aarch64` Linux/Windows on x64 runners (`cargo-zigbuild` or `cross`; not checked), and use `macos-latest` (ARM) for both mac targets with `rustup target add x86_64-apple-darwin` (Apple's toolchain cross-builds both; not checked).
- Alternatives: hand-written matrix workflow with `cargo build --release --locked --target` and `tar`/`zip` plus `sha256sum` (effort M, more lines to maintain, full control; best if you want the build to be simple to audit for USS-REL-003); `cargo-binstall` metadata only (needs release archives anyway); `cross`/`zigbuild` (build-side tool, not a release tool).
- dist generates `.github/workflows/release.yml` and must be re-run by `dist init`/`dist generate` when its version changes; its generated actions are pinned by tag, not SHA (not checked), which conflicts with the repo's own SHA-pin rule (USS-SUP-*). Check the generated file against USS-SUP rules (and `allow-dirty`, which dist advises against, would be needed to patch it).

### Gaps
- I did not read dist's supported-target list page or its private-repo behaviour; the book front page says nothing of either.
- dist plus musl-aarch64 support was not confirmed.

## 4. USS-SUP-013 SBOM

### Takeaway
Recommended: cargo-cyclonedx 0.5.10, one CycloneDX JSON file per release, published with the release (dist's `cargo-cyclonedx = true` does it, or run `cargo cyclonedx --format json` in the build job). Verify requires SPDX or CycloneDX, published with the release, listing components, with dependency relationships showing the direct dependencies of the release.

### Cited Findings
- cargo-cyclonedx options: `-f, --format json|xml`; `--spec-version` accepts "1.3, 1.4 or 1.5", default 1.3 (README as fetched; 1.6 and 1.7 not listed); `--all` (all dependencies, default); `--describe crate|binaries|all-cargo-targets`; `--target` (default host; `all` allowed); output `bom.xml` beside each Cargo.toml, `--override-filename`. — [cargo-cyclonedx README](https://raw.githubusercontent.com/CycloneDX/cyclonedx-rust-cargo/main/cargo-cyclonedx/README.md)
- The rule's references name SPDX 3.0.1 and CycloneDX 1.7, but cargo-cyclonedx (README) lists up to 1.5. The rule's Verify text does not require a specific version, only "SPDX or CycloneDX format". Note the mismatch.
- dist `cargo-cyclonedx` key generates and uploads an SBOM per project in a workspace. — dist config reference.

### Inferences
- Content: CycloneDX output includes a `dependencies` array (the dependency graph). The component for the `uss` crate has the direct dependencies listed in its `dependsOn`, which meets "direct dependencies of the release" (the graph content was not inspected here; test by running the tool once and reading the JSON).
- Use `--describe binaries --target <triple>` per target if the SBOM should match one binary; one SBOM from the workspace root with `--target all` is simpler and sufficient for the rule ("the components of the release"). Attach as `uss-<version>.cdx.json`.
- cargo-auditable v0.7.8 embeds the dependency list in the binary (readable by `cargo audit bin`). It is a useful supplement, not an SBOM file in SPDX/CycloneDX; do not use it alone.
- syft v1.57.2: scans a binary or directory and emits CycloneDX or SPDX; works but reads auditable data only if embedded and gives a less exact graph than the lockfile; use as a cross-check. cargo-sbom: not checked.
- Effort S (one workflow step, one file in the release); no plan dependency.

### Gaps
- Did not verify SPDX/CycloneDX 1.7 support in any Rust generator.

## 5. USS-SUP-014 signatures, USS-DST-015 verify instructions, USS-SUP-015 SLSA Build L2

### Takeaway
Recommended (public repository): `actions/attest@v4` (4.2.2) pins by SHA in the release job, with `permissions: id-token: write, contents: read, attestations: write`, over every archive, the SBOM and the notices file; users run `gh attestation verify <file> -R <owner>/<repo>`. One attestation covers SUP-014 (a signature in a registry) and SUP-015 (build-platform provenance, SLSA v1.0 Build L2 per GitHub). Effort S-M. For a private Free-plan repository this route is unavailable.

### Cited Findings
- Permissions and step as documented: `permissions: id-token: write, contents: read, attestations: write`; `uses: actions/attest@v4` with `subject-path`; verify with `gh attestation verify PATH -R ORGANIZATION_NAME/REPOSITORY_NAME`. The page uses `actions/attest`, not attest-build-provenance. — [GitHub docs, use artifact attestations](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations)
- Plan restriction: see item 0.
- SLSA: Build L2 needs provenance that is "authentic" and a build on a hosted platform; Build L3 adds unforgeable provenance (signing secrets that user build steps cannot reach) and isolated builds. — [SLSA v1.2 build requirements](https://slsa.dev/spec/v1.2/build-requirements). GitHub says attestations alone give "SLSA v1.0 Build Level 2" and L3 needs a reusable workflow. — GitHub concepts page. The rule cites SLSA v1.2 (the page I read is v1.2).
- cosign v3.1.6 and cosign-installer v4.1.4 are current. Sigstore signing overview: signing events "are logged in Rekor... so that the events are publicly auditable"; identity is bound through the Fulcio certificate. The page does not document `sign-blob --bundle` or `verify-blob --certificate-identity/--certificate-oidc-issuer` and does not mention private repositories. — [Sigstore signing overview](https://docs.sigstore.dev/cosign/signing/overview/)
- slsa-github-generator: last release v2.1.0 (2025-02-24), nothing newer. — GitHub releases API.

### Inferences
- Alternative for a private Free repo, SUP-014 only: cosign keyless via GitHub OIDC: `cosign sign-blob --bundle <file>.sigstore.json <file>` in the release job (needs `id-token: write`), verified with `cosign verify-blob --bundle ... --certificate-identity-regexp '^https://github.com/<owner>/<repo>/\.github/workflows/release\.yml@refs/tags/v' --certificate-oidc-issuer https://token.actions.githubusercontent.com <file>`. Privacy consequence: the signing certificate carries the repository identity and the event goes to the public Rekor log (inferred from the Sigstore page; the public-good instance logs publicly), so the private repository's name and workflow path become public. Consider that, or make the repository public. DST-015 Verify needs the command and the signer identity in the docs: the `--certificate-identity-regexp` line above is exactly that.
- Minisign (0.12, last release 2025-01): works anywhere, no plan limits, but you must keep a private key as a secret and publish the public key. It is a valid "signature"; it does not give provenance. Lower assurance (secret in CI) but the cheapest fully-private option.
- SUP-015 Alternative: slsa-github-generator (v2.1.0, stale) reaches Build L3 via reusable workflows for public repositories; for private repositories it publishes to the public Rekor log by default (not checked); not recommended with a stale release. A hand-made provenance file signed by cosign in the same job does not meet "signed by the build platform" (the job's own steps hold the signing identity, which is L2 by SLSA's text only if the platform produced it; do not claim it). So on private Free: report SUP-015 as `fail` / declined by Standard Deviation, or go public.
- DST-015 Verify: the README (or release notes) must list each signed kind of artifact (archive per target, SBOM, notices file, checksum file) and the command with the signer identity. Write one table: kind, command, identity. Effort S.
- File changes: `.github/workflows/release.yml` (new or dist-generated), README "Verify a download" section, CHANGELOG, `.uss/audits/*`; one test could assert the README shows `gh attestation verify` or `cosign verify-blob` (optional).

### Gaps
- Did not confirm the exact `cosign` flags in v3 docs (v3 changed defaults, e.g. bundle format); verify with `cosign sign-blob --help` before writing the README.
- Did not verify slsa-github-generator's private-repo behaviour.

## 6. USS-LIC-004 notices file

### Takeaway
Recommended: cargo-about 0.9.3 generating a `THIRD_PARTY_NOTICES.md` (or `.txt`) at release time from `about.toml` and `about.hbs`, included in each archive and embedded in `uss` with `include_str!`, shown with `uss show NOTICES` (or `uss licenses`). Effort M. The Verify says a generated notices file that lists every included dependency passes.

### Cited Findings
- cargo-about: "Cargo plugin for generating a license listing for all dependencies of a crate"; `cargo about init` writes `about.toml` and `about.hbs`; `cargo about generate about.hbs > license.html`. — [cargo-about README](https://raw.githubusercontent.com/EmbarkStudios/cargo-about/main/README.md). The page did not explain accepted licences, `--workspace` or target selection; details are in its book (not read).
- cargo-about 0.9.3 is current (2026-10-26).

### Inferences
- `uss` is built from `tools/uss` with `--locked`; run `cargo about generate --locked about.hbs > NOTICES.md` in the release job and in `cargo xtask` so a test compares the committed notices with the generated output (needs `cargo-about` installed under `.tools` like cargo-deny; xtask already installs cargo-deny there). Commit the file so a tag checkout contains it: then `include_str!` embeds it and no build-time network or tool is needed. A test in `tools/uss` asserts the embedded notices name every crate in `Cargo.lock` that is in the `uss` normal dependency tree.
- Embedding: `uss show NOTICES.md` already exists for embedded files (`uss show AGENTS.md`, `uss show standard/...`); add the notices to the embed list so no new command is needed; add `uss version --licenses` only if the maintainers want a separate command (not needed for the Verify). Update every document that names the command in the same commit (Definition of Done).
- Licence of the project itself: MIT, with text in `LICENSE`; the archive should carry `LICENSE` and the notices file.
- Alternative: cargo-bundle-licenses (not read, not checked): outputs a single YAML/JSON/TOML of dependencies and licence texts; simpler, no template, less readable; good if no human-readable file is needed. cargo-deny already in the repo does licence allow-listing but does not output texts.

### Gaps
- Did not read the cargo-about book, so `about.toml` accepted-list details and `--workspace`/target handling are unverified.
- Did not read cargo-bundle-licenses at all.

## 7. USS-REL-003 rebuild steps from a tag

### Takeaway
Document the rebuild as: pinned toolchain (`rust-toolchain.toml`, 1.100.0), `cargo build --release --locked --target <triple>` on the named runner image, with `--remap-path-prefix` and `SOURCE_DATE_EPOCH`. Bit-for-bit identity is realistic only on Linux musl targets built in a pinned container; Windows and macOS binaries are likely to differ (linker timestamps, code-signing). Be honest in the document and in the audit.

### Cited Findings
- I did not fetch a source for this item in this session. (Reproducible Builds definitions are the rule's own reference.)

### Inferences (not checked; from prior knowledge)
- Rust reproducibility levers: `--locked`; exact toolchain; `RUSTFLAGS="--remap-path-prefix=$PWD=/build --remap-path-prefix=$CARGO_HOME=/cargo"`; `SOURCE_DATE_EPOCH` set to the tag commit time (`git log -1 --format=%ct`); `CARGO_INCREMENTAL=0`; same linker/libc (musl static helps); `strip` setting fixed in the profile. MSVC (Windows) builds embed a PDB path and timestamps unless `/Brepro` (`-C link-arg=/Brepro`) is used; macOS adds an ad-hoc signature and UUIDs; cross-OS identical output is not realistic.
- The document (new `docs/release.md` or section in CONTRIBUTING; under the R8 ruling it waits for the release) must give: tag, runner image or container digest, toolchain version, environment variables, exact commands, and the `sha256sum` comparison. Include a statement per artifact "reproducible" or "not reproducible, because X". The Verify says "all are identical", so list only the targets that were proven; a non-identical artifact fails the rule, so an honest Standard Deviation is better than claiming it.
- Plan: build the Linux musl archives twice (in CI and on a clean machine) before the release and compare. Effort M-L. Alternative: declare a Standard Deviation for USS-REL-003 and document best-effort.

### Gaps
- No source read; every sentence above needs a check against the Reproducible Builds site and the Cargo/rustc docs.

## 8. USS-CTB-007: make the documented install from a tag runnable

### Takeaway
The install command `cargo install --locked --git <repo> --tag v0.1.0 uss` cannot run until the tag exists and, for a private repository, cannot run for anyone without credentials. Making the repository public or shipping binaries (items 0 and 3) fixes both. The Verify says each command "runs as written in a fresh clone"; a command that needs a tag that does not exist fails it.

### Cited Findings
- AGENTS.md "Before every workflow" and the CI template both use `cargo install --locked --git https://github.com/HaleTechCo/Universal-Software-Standard --tag v<version> uss` (read from files).
- `gh api` shows the repository is private (read-only call).

### Inferences
- Order that works: (1) public repo or credentials, (2) commit CHANGELOG/notes, (3) release audit and `uss release-check`, (4) tag, (5) run the install command from a clean machine as a test, (6) re-audit CTB-007 with that run as evidence. Until the tag exists, before-release text in AGENTS.md ("If the release is not listed ... install from a checkout") stays valid and the audit can use the checkout form (that is the R8-style approach already used).
- Effort S once the repository is public.

### Gaps
- None for the logic; the Verify's "fresh clone" run needs a real tag.

## 9. The 0.1.0 entry in CHANGELOG.md, and the order of audit, `uss release-check` and tag

### Takeaway
Use Keep a Changelog headings under `## [0.1.0] - YYYY-MM-DD`, with the rule IDs added, and write the entry before the release audit, because the audit examines the release commit and the changelog is part of it. Order from AGENTS.md "Release audit": audit the commit, commit record/results/report, run `uss release-check`, delete `uss-report.md`, tag that commit.

### Cited Findings
- AGENTS.md "Release audit" (read from the file): audit the commit to release; commit the record, results file and report; run `uss release-check` (it examines the latest audit of each record and writes `uss-report.md`); delete `uss-report.md`, do not commit it; "Put the release tag on the commit that holds the results." CI runs `uss release-check` on `release: published` and attaches the report.
- `CHANGELOG.md` currently holds only `# Changelog`. `uss show CHANGELOG.md` is used by the upgrade workflow to list "the rule IDs that each version after the old one added, changed or retired" (AGENTS.md "Upgrade the pinned version"); that is the content requirement the entry must meet. For 0.1.0 there is no earlier version, so list the initial set (443 rules per memory note; confirm with `uss applicable`/`lint` output).
- Keep a Changelog format: not fetched today (not checked).

### Inferences
- Because the changelog must be in the audited commit, and the tag must sit on the commit that holds the audit results, the results commit itself comes after the changelog commit. The audit's `commit` field names the audited commit; the tag goes on the results commit (AGENTS.md); `uss release-check` accepts later commits that touch only `.uss/` (see troubleshooting "reports changed files").
- Release date: set the date in the heading to the audit/tag day; if the tag moves to another day, change both.
- Attach to the GitHub release: archives, checksums, SBOM, notices, signatures/attestations, and `uss-report.md` (the CI template does the last one).

### Gaps
- Exact Keep a Changelog 1.1.0 text not re-read.

## 10. CI template `templates/ci/uss-release.yml`

### Takeaway
Split into two jobs: `release-check` (permissions `contents: read`, produces the report as a workflow artifact) and `upload` (`needs: release-check`, `contents: write`, only `gh release upload`). This keeps the write token away from the step that installs and runs third-party-built code. Use `cargo install --git ... --rev <sha>` pinned to the release commit SHA plus `--tag` check in text; `--tag` can be moved by whoever can move tags.

### Cited Findings
- Current template (read from file): one job `release-check` with `contents: write`, installs `uss` from `--tag "v${USS_VERSION}"`, runs `uss release-check --out uss-report.md`, then `gh release upload "$TAG" uss-report.md --clobber`; actions pinned by SHA (`actions/checkout@3d3c42e...  # v7.0.1`). The test `ci_template_is_hardened` asserts `on.release.types == [published]`, top-level `permissions == {contents: read}`, every `uses:` is `name@<40 hex>`, checkout has `fetch-depth: 0` and `persist-credentials: false`, no `${{` in `run`, and each `cargo` line is `cargo "+${USS_RUST_TOOLCHAIN}" install `.
- Dependabot (`github-actions` ecosystem) is listed as a supported ecosystem. — [Dependabot supported ecosystems](https://docs.github.com/en/code-security/dependabot/ecosystems-supported-by-dependabot/supported-ecosystems-and-repositories). The page text I read did not answer whether it updates SHA-pinned `uses:` lines with version comments, or whether Dependabot version updates are free for private repos.

### Inferences
- Split: job 1 `release-check` with `permissions: contents: read`; ends with `actions/upload-artifact` (SHA-pinned) of `uss-report.md`; job 2 `attach` `needs: release-check`, `permissions: contents: write`, steps: `actions/download-artifact` (SHA-pinned), then `gh release upload`. Update the test: iterate both jobs (it already loops over all jobs), assert the first job has no `contents: write`, and `uses` still pinned. Effort S.
- `--tag` versus `--rev`: with `--tag`, a retargeted tag changes what is installed. `--rev <40-hex>` is immutable; add a second env `USS_REV` and a test that checks the SHA is 40 hex. But the template text would then carry a value the adopter must look up (`git rev-parse v0.1.0` in the USS repo) and cannot know before the release; the release must therefore update the template file in a later commit (the template lives inside the release's own commit, so the SHA of that commit cannot be inside itself). Recommend: keep `--tag` in the template, add `--locked`, and rely on tag protection rules for `v*` in the USS repository; document it. (Tag protection for private repos on Free: not checked.) Once binaries exist, the stronger fix is to download the release binary and verify its attestation/signature, with no `cargo install` at all.
- Dependabot and a copied template: Dependabot reads workflow files in the adopter's `.github/workflows/` when `package-ecosystem: github-actions` is configured in their `dependabot.yml`; it updates action versions, and for SHA pins it updates the SHA and the trailing version comment when the comment holds a version tag (my prior knowledge; not confirmed from the page I read). Put a sentence in the template header: "Add `github-actions` to `.github/dependabot.yml` so the SHA pins stay current", and add an example `dependabot.yml` snippet in the README. The adopter's repository settings decide availability; the USS repo itself does not control it.

### Gaps
- Dependabot behaviour on SHA pins and on private Free repositories: unconfirmed.
- Tag-protection availability on a private Free repository: unconfirmed.

## 11. CI template that runs the agent audit with no person present

### Takeaway
Feasible with `anthropics/claude-code-action@v1`: automation mode (a `prompt` input), plugin installed by the `plugin_marketplaces` and `plugins` inputs, secret `ANTHROPIC_API_KEY` or `CLAUDE_CODE_OAUTH_TOKEN`. Requirements below. For a repository whose marketplace is private, the plugin cannot be fetched without extra credentials (not documented; see Gaps), so the marketplace should be public first.

### Cited Findings
- Automation mode: "when the workflow provides a `prompt` input, Claude runs without waiting for a mention"; results go to the run log unless the prompt tells Claude to post and a tool allows it. — [Claude Code GitHub Actions](https://code.claude.com/docs/en/github-actions)
- Plugins: "install the plugin with the `plugin_marketplaces` and `plugins` inputs, then pass the namespaced `/plugin-name:skill-name` as the `prompt`. The `plugins` input takes `plugin-name@marketplace-name`, where the marketplace name comes from the marketplace's own manifest rather than its repository URL." Example uses `plugin_marketplaces: "https://github.com/anthropics/claude-plugins-official.git"` and `plugins: "code-review@claude-plugins-official"`. — same page
- Secrets: `ANTHROPIC_API_KEY` or `CLAUDE_CODE_OAUTH_TOKEN` (subscription tokens, from `claude setup-token`; tied to one person's subscription, so for shared use an API key is recommended); or workload identity federation with `id-token: write`. — same page
- Permissions in the doc's examples: `contents: read`, `pull-requests: read`, `issues: read`, `id-token: write`; `claude_args` carries `--allowedTools`; "For a plain-text prompt, Claude has no shell or GitHub API access until you grant the tools" with `--allowedTools` or `permissions.allow`; with a skill, the skill's `allowed-tools` frontmatter applies. Triggers: the actor must have write access on issue/PR events; scheduled runs are attributed to a repo user and bot actors are rejected unless in `allowed_bots`. — same page
- The Claude GitHub App is needed only for the default authentication (`github_token` omitted); the Contents/Issues/PR permissions apply. — same page
- Releases: the action has a floating tag `v1`; the releases API shows `v1` published 2025-08-26 (the repository moves the tag). The doc's own examples use `@v1`, not a SHA. — GitHub releases API.

### Inferences
- Template design (effort M; new file `templates/ci/uss-audit.yml`; update `tools/uss/src/cli/mod.rs` test to cover it; update AGENTS.md "With no person present"): trigger `workflow_dispatch` and `schedule` (the AGENTS.md rules for no person present already say: ask nothing, do not change `facts/decisions/deviations`, write unverified with evidence). Step list: checkout (SHA pinned, `fetch-depth: 0`), `anthropics/claude-code-action` pinned to a full SHA (the repo's hardening test requires SHA pins; resolve the SHA of the latest `v1.x` tag, which I did not do), `plugin_marketplaces: https://github.com/<owner>/<repo>.git`, `plugins: uss@universal-software-standard`, `prompt: /uss:audit`, `claude_args` allowing `Bash`, `Read`, `Write`, `Edit` narrowly (e.g. `Bash(uss:*,git:*,cargo:*)`), `permissions: contents: write` only if the job commits results (prefer opening a PR from a second job, as in the split design).
- The audit must install `uss` first (a step before the action, using the same pinned-toolchain approach as item 1, or the release binary).
- Cost control: `--max-turns`, workflow `timeout-minutes`, `concurrency`.
- Private marketplace in CI: the docs page I read shows only public Git URLs; I found no statement about passing a token for a private marketplace in the action (host-marketplace page on private repos was not read).

### Gaps
- Private marketplace auth in the action; the action's version SHA; the cost of a full audit run.

## 12. Making the plugin marketplace public

### Takeaway
Two routes. (a) Your own marketplace: push to a public GitHub repository; users run `claude plugin marketplace add <owner>/<repo>` then `claude plugin install uss@universal-software-standard`. No submission form. (b) Anthropic's directory (claude.ai/directory): submit from `claude.ai/directory/manage`; needs a paid claude.ai plan and a GitHub repository; the official marketplace `claude-plugins-official` does not accept portal submissions.

### Cited Findings
- Own marketplace: "Once the file is in the repository, the plugin is published, with no submission form." Anyone who can clone the repository can install; if private, the marketplace is private. Users add with `claude plugin marketplace add your-org/your-marketplace` ("a GitHub `owner/repo` shorthand, a URL, or a path"), install with `claude plugin install name@marketplace`, or in one command `claude plugin install name --marketplace owner/repo` (needs Claude Code v2.1.292 or later); in session `/plugin install name --marketplace owner/repo` needs v2.1.275 or later. — [Publish and distribute a plugin](https://code.claude.com/docs/en/plugins/publish.md)
- Versioning: if `plugin.json` has `version` and later commits do not change it, users keep the old copy; "either increment `version` on every release, or omit it in a git-hosted marketplace so Claude Code uses the commit SHA." Auto-update is off by default for a third-party marketplace. `claude plugin tag` creates `{name}--v{version}`; `claude plugin validate --strict` for CI. — same page
- Directory: "Submitting requires a paid claude.ai plan"; "the portal applies additional directory rules that the CLI doesn't check"; checklists at claude.com/docs/plugins/pre-submission-checklist and /submit. "Anthropic's official marketplace, `claude-plugins-official`, doesn't take submissions through the directory portal." — same page
- A marketplace needs `name`, `owner`, `plugins[]`; choose a `name` others are unlikely to pick; reserved official names are refused. — [Create a marketplace](https://code.claude.com/docs/en/plugin-marketplaces)

### Inferences
- Recommended: route (a) first (effort S: set repository public, run `claude plugin validate --strict .`, decide `version` policy: omit `version` to use commit SHA, or set it equal to the USS version and bump it with each release so a plugin update goes with a `uss` install), add the two user commands to README. Route (b) later if there is demand; it needs a paid claude.ai plan (not known whether the maintainer has one) and a review whose requirements are on pages I did not read (`claude.com/docs/plugins/pre-submission-checklist`).
- Version coupling: the plugin's skills call `uss`; `uss` embeds the workflows. Pin the plugin version to the USS version, and have the skill check `uss version` against its own, so a stale plugin and a new binary are caught (inferred design).
- Community directories: I did not search for community catalogues; none is cited.

### Gaps
- Directory review criteria not read.
- Whether a private marketplace can be added with GitHub credentials (host-marketplace page) not read; the publish page says only that private repository means private marketplace.

## Summary of recommended order (effort)
1. Decide public vs private repository (the decision gates items 3-5, 11, 12).
2. Fix the install defect (S) with the equality test (S), docs/troubleshooting.md, README.
3. Run the trial, measure items listed in section 2.
4. Release workflow with dist v0.34.0 or a hand matrix (M); SBOM (S); attestations or cosign (S-M); notices (M); rebuild document (M-L, with honest per-artifact claims).
5. Split the CI template (S); optional audit template (M).
6. CHANGELOG entry, release audit, `uss release-check`, tag, then the clean-machine install test (S).
