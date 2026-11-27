# Metadata rules for packages, plugins and projects

Research for the `todo.txt` item "metadata": "there should be MUST for
certain metadata (like what the parent tool/marketplace requires) and SHOULDs
for anything that increases compatibility/QoL". Every web source below was
read on 2026-11-25 unless a line says otherwise. Repository files were read at
commit dd5ab83 on `main`. Paths to repository files are relative to the
repository root.

## Q1. What does USS already require about metadata, and where would a new rule overlap?

### Takeaway

USS already covers the licence field (USS-LIC-001), the README (USS-DOC-001),
version numbers and tags (USS-REL-001, USS-REL-002) and the manifest of plugins
for the project's *own* plugin host (USS-EXT-003). Nothing requires a published
package, plugin, extension, action, chart or image to carry the metadata that
its registry or host tool requires, nothing requires a declared minimum
runtime or host version, and nothing covers listing, image or repository-host
metadata. The fact `distributed-as-package` exists but no rule uses it yet.

### Cited Findings

Existing rules that touch metadata:

- USS-LIC-001 (MUST, always, Compatibility): the repository root has a licence
  file, and "each package manifest whose format has a licence field MUST
  declare the licence there as an SPDX licence expression". Its Verify names
  `Cargo.toml`, `package.json` and `pyproject.toml`, fails a manifest that
  names a licence file instead of an expression, and does not judge a format
  with no licence field (such as `go.mod`). It cites SPDX License List
  3.29.0. — [standard/core/licensing.md](../../../../../standard/core/licensing.md)
- USS-EXT-003 (MUST, `has-plugin-interface`): each plugin of the project's own
  plugin interface carries a manifest that states its identity, version,
  compatible host versions, requested permissions and licence. It cites the VS
  Code extension manifest as an example. —
  [standard/core/extensibility.md](../../../../../standard/core/extensibility.md)
- USS-EXT-015 (MUST, `has-plugin-marketplace`): each listing in the project's
  own marketplace shows the plugin's licence as an SPDX expression. —
  [standard/core/extensibility.md](../../../../../standard/core/extensibility.md)
- USS-DST-006 (MAY, `is-developer-tool`): once adopted, the IDE extension has
  "the manifest that the IDE's extension format needs" and loads without
  error. USS-DST-008 (MAY): once adopted, "the agent that it targets loads it
  without error". USS-DST-005 (MAY): once adopted, `helm lint` passes on the
  chart. These judge metadata only when the MAY is adopted. —
  [standard/core/distribution.md](../../../../../standard/core/distribution.md)
- USS-K8S-004 (MUST, `platform.helm and deploys-to-kubernetes`): CI runs
  `helm lint --strict` on each chart. —
  [standard/modules/platforms/kubernetes.md](../../../../../standard/modules/platforms/kubernetes.md)
- USS-DOC-001 (MUST, always): the repository root has a README that says what
  the software does, how to install it and how to start. —
  [standard/core/documentation.md](../../../../../standard/core/documentation.md)
- USS-REL-001 (MUST): releases follow SemVer 2.0.0; USS-REL-002 (MUST): each
  release has a tag named after its version. —
  [standard/core/release.md](../../../../../standard/core/release.md)
- USS-PY-008 (MUST): direct dependencies are declared in `pyproject.toml`
  (PEP 621, PEP 735); its rationale calls `pyproject.toml` "the one place that
  every Python tool reads for the project's metadata". —
  [standard/modules/languages/python.md](../../../../../standard/modules/languages/python.md)
- USS-PY-007, USS-TS-008 and USS-GO-007 judge only the pinned and CI-tested
  runtime. Each says the declared minimum (`requires-python`, `engines.node`,
  the `go` directive) "is not judged by this rule" and calls it "a
  compatibility promise (USS-DEP-016)". USS-DEP-016 (SHOULD, `is-library`)
  is about the declared ranges of *dependencies*, not of the runtime. —
  [standard/modules/languages/python.md](../../../../../standard/modules/languages/python.md);
  [standard/modules/languages/typescript.md](../../../../../standard/modules/languages/typescript.md);
  [standard/modules/languages/go.md](../../../../../standard/modules/languages/go.md);
  [standard/core/dependencies.md](../../../../../standard/core/dependencies.md)
- USS-TS-007 pins the *development* runtime through `devEngines`, not
  `engines`. —
  [standard/modules/languages/typescript.md](../../../../../standard/modules/languages/typescript.md)
- The CRA module maps "Information for users" (Annex II) to USS-CRA-004,
  USS-SEC-029 and USS-DOC-001 with "partial: the other items of Annex II". —
  [standard/modules/regulations/cra.md](../../../../../standard/modules/regulations/cra.md)
- The Rust module has no rule on `rust-version`, `description`, `keywords` or
  `categories` (a search of the file for `rust-version`, `msrv` and
  `crates.io` finds nothing). —
  [standard/modules/languages/rust.md](../../../../../standard/modules/languages/rust.md)

Facts:

- `distributed-as-package` ("Is it released as an installable package or
  binary?") implies `publishes-releases`. A search of `standard/` finds it only
  in `applicability.md`: no rule's `Applies when` uses it. —
  [standard/applicability.md](../../../../../standard/applicability.md)
- `has-plugin-interface` and `has-plugin-marketplace` are enabled by
  USS-EXT-001 and USS-EXT-008: they describe a project that *hosts* plugins,
  not one that *is* a plugin of another tool. —
  [standard/applicability.md](../../../../../standard/applicability.md)
- The DST area header is `Applies when: publishes-releases`, and its default
  category is Compatibility (install methods count toward Features). —
  [standard/core/distribution.md](../../../../../standard/core/distribution.md)
- The spec's budget: "a project whose facts are all false sees at most a
  quarter of the core rules by the 0.1.0 release". —
  [docs/superpowers/specs/2026-11-21-universal-software-standard-design.md](../../../../../docs/superpowers/specs/2026-11-21-universal-software-standard-design.md)
  (section 6.4)

### Inferences

- Gaps, in order of value: (1) required host metadata for anything the project
  publishes to a registry, a marketplace or a host tool; (2) a declared,
  tested minimum version of the runtime, toolchain or host; (3) listing
  metadata (description, source link, issue link, keywords, README on the
  registry); (4) OCI image annotations; (5) the repository-host description
  and topics; (6) citation metadata.
- Overlaps to avoid: the licence field stays with USS-LIC-001 (new rules say
  "USS-LIC-001 judges the licence field, not this rule"); the README file in
  the repository root stays with USS-DOC-001 (a new listing rule judges the
  README that the *package* ships); chart linting stays with USS-K8S-004; the
  project's own plugin host stays with EXT.
- Putting the new rules in DST keeps them behind `publishes-releases`, so the
  all-false budget does not change.
- Because no rule uses `distributed-as-package`, it can gate the new rules
  and its Definition can be made more precise before 0.1.0 without a ripple.
- Conflict found: crates.io reads the licence against SPDX License List 3.20
  (Q2), while USS-LIC-001 cites 3.29.0. An identifier added after 3.20 passes
  USS-LIC-001 and can still be refused by crates.io. The proposed MUST
  (USS-DST-017) would catch this through the registry's own rule.

### Gaps

- I did not read every area file in full. I searched all of `standard/` for
  metadata terms (SPDX, `rust-version`, `engines`, `requires-python`, keywords,
  CITATION, FUNDING, topics, annotations, `Chart.yaml`, `plugin.json`,
  homepage, publiccode, codemeta) and read the rules that matched.

## Q2. What does each registry, marketplace or host require, and what does it recommend?

### Takeaway

Each host splits its metadata into a small required set, which it enforces by
refusing the upload or by failing to load, and a larger recommended set that it
shows or searches. Most hosts have a validator (a dry-run publish, a validate
command or an audit). USS has modules for crates.io (Rust), npm (TypeScript),
PyPI (Python), Go, Maven Central (Maven, Java), Helm and Kubernetes. It has no
module for OCI images, Claude Code plugins, VS Code or Open VSX, JetBrains,
GitHub Actions, Homebrew or winget.

### Cited Findings

#### Summary table

| Host | USS module | Required (host refuses or fails without it) | Recommended (compatibility, discovery, QoL) | Validator |
|---|---|---|---|---|
| crates.io | Rust | `name`; `version` to publish; `description`; `license` or `license-file` | `repository`, `readme`, `homepage` (only if distinct), `documentation`, `keywords` (max 5), `categories` (max 5, exact slugs), `rust-version` | `cargo publish --dry-run`; Clippy `cargo_common_metadata` |
| npm | TypeScript | `name`, `version` | `description`, `keywords`, `homepage`, `bugs`, `license`, `repository`, `engines`, `os`, `cpu`, `exports`, `files`, `funding`; `types` | dry-run publish (not read); publint |
| PyPI | Python | `Metadata-Version`, `Name`, `Version` (`name` static, `version` static or dynamic); no `Private ::` classifier | `description`, `readme`, `requires-python`, `license` (SPDX), `license-files`, `authors`, `keywords`, `classifiers`, `urls` | `twine check --strict` |
| Go modules, pkg.go.dev | Go | exactly one `module` directive; a detectable redistributable licence for full pkg.go.dev display | `go` directive, tagged and stable versions, package doc comment, README | the `go` command; pkg.go.dev licence policy |
| Maven Central | Maven, Java | coordinates (no `-SNAPSHOT`), `name`, `description`, `url`, `licenses`, `developers`, `scm`; sources and javadoc JARs; checksums; signatures | none stated | Central Portal validation |
| OCI image (any registry) | none | none (annotations are optional) | `org.opencontainers.image.*` keys | none |
| GitHub Container Registry | none | none | `source`, `description` (512 chars), `licenses` (256 chars) | none |
| Helm chart | Kubernetes (`platform.helm`) | `apiVersion`, `name`, `version` | `kubeVersion`, `description`, `type`, `keywords`, `home`, `sources`, `maintainers`, `icon`, `appVersion`, `annotations` | `helm lint` |
| Artifact Hub | Kubernetes | none found | `artifacthub.io/*` annotations | not read |
| Kubernetes objects | Kubernetes | none | `app.kubernetes.io/*` labels | none |
| Claude Code plugin | none | `name` (manifest optional) | `version`, `description`, `author`, `homepage`, `repository`, `license`, `keywords`, `engines` | `claude plugin validate [--strict]` |
| Claude Code marketplace | none | `name`, `owner.name`, `plugins[].name`, `plugins[].source` | `description` and per-entry display fields | `claude plugin validate` |
| VS Code Marketplace | none | `name`, `version`, `publisher`, `engines.vscode` | `displayName`, `description`, `categories`, `keywords` (max 30), `icon`, `license`, `repository`, `bugs`, `homepage`, `galleryBanner`, `pricing`, `qna`, `sponsor` | `vsce package` |
| Open VSX | none | `publisher` (namespace) | not stated | `ovsx` (not read) |
| JetBrains Marketplace | none | `name`, `version`, `idea-version`, `vendor`, `description` | `id` ("highly recommended"), `change-notes` | Plugin Verifier |
| GitHub Actions | none | `name`, `description`, `runs`; for the Marketplace, a unique name and one `action.yml` at the root | `author`, `inputs`, `outputs`, `branding` | not found |
| Homebrew (homebrew-core) | none | `homepage`; `license` for new formulae | `desc` | `brew audit` |
| winget | none | `PackageIdentifier`, `PackageVersion`, `PackageLocale`, `Publisher`, `PackageName`, `License`, `ShortDescription`, `Installers`, `ManifestType`, `ManifestVersion` | `PublisherUrl`, `PackageUrl`, `LicenseUrl`, `Tags`, `MinimumOSVersion` and more | `winget validate` (not read) |

The rows are compiled from the findings below.

#### crates.io (Cargo) — USS module: Rust

- "The only field required by Cargo is `name`." `version` "is optional and
  defaults to `0.0.0`. The field is required for publishing packages." —
  [The Cargo Book, The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- crates.io "requires the `description` to be set" and "requires either
  `license` or `license-file` to be set". `description` "should be plain text
  (not Markdown)". —
  [The Cargo Book, The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- crates.io reads `license` as an SPDX 2.3 expression, and "The name must be a
  known license from the SPDX license list 3.20". —
  [The Cargo Book, The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- crates.io name rules: ASCII only, no reserved names, no special Windows
  names, at most 64 characters. —
  [The Cargo Book, The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- `keywords`: crates.io "allows a maximum of 5 keywords. Each keyword must be
  ASCII text, have at most 20 characters, start with an alphanumeric
  character, and only contain letters, numbers, `_`, `-` or `+`".
  `categories`: "a maximum of 5 categories. Each category should match one of
  the strings available at https://crates.io/category_slugs, and must match
  exactly". —
  [The Cargo Book, The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- `readme`: crates.io "will interpret it as Markdown and render it on the
  crate's page"; without the field, Cargo uses a `README.md`, `README.txt` or
  `README` in the package root. `homepage` should be set only for a dedicated
  website, not one redundant with `documentation` or `repository`. Without
  `documentation`, crates.io links to docs.rs. —
  [The Cargo Book, The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- The publishing checklist: "Before publishing, make sure you have filled out
  the following fields: `license` or `license-file`, `description`,
  `homepage`, `repository`, `readme`". Keywords and categories are "a good
  idea ... though they are not required". `cargo publish --dry-run` (or
  `cargo package`) verifies, packages and compiles the crate; the `.crate` file
  has a 10 MB limit. —
  [The Cargo Book, Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
- `rust-version` is optional. "When your package is compiled on an unsupported
  toolchain, Cargo will report that as an error"; `cargo add` picks
  dependency versions compatible with it; the resolver "may take Rust version
  into account"; Clippy's `incompatible_msrv` lint uses it. The page expects
  support to be "Verified: A package's functionality is verified on its
  supported Rust versions, including automated testing", and treats a change
  of `rust-version` as "a minor incompatibility". —
  [The Cargo Book, Rust Version](https://doc.rust-lang.org/cargo/reference/rust-version.html)
- Clippy's `cargo_common_metadata` lint "Checks to see if common metadata is
  defined in `Cargo.toml`"; group `cargo`, default level allow; option
  `cargo-ignore-publish` (internal testing) ignores the `publish` setting.
  "Why is this bad: It will be more difficult for users to discover the
  purpose of the crate". The page did not list the fields. —
  [Clippy lint list](https://clippy.rust-lang.org/master/index.html)
- A secondary source says the lint covers description, license or
  license_file, repository, readme, keywords and categories. —
  [Clippy source, common_metadata.rs](https://rust.googlesource.com/rust/+/cc7fb2da97f7701e1bd6ec87b5aefc162a64d310/clippy_lints/src/cargo/common_metadata.rs)
  (search-result summary, not read in full)

#### npm (`package.json`) — USS module: TypeScript and JavaScript

- The docs are for "npm CLI Version 11.21.3 (Legacy)". "If you plan to publish
  your package, the most important things in your package.json are the name
  and version fields as they will be required". Name: at most 214 characters,
  no uppercase for new packages, URL-safe. —
  [npm Docs, package.json](https://docs.npmjs.com/cli/v11/configuring-npm/package-json)
- `license`: "You should specify a license"; SPDX expression syntax;
  `UNLICENSED` for a package that grants no rights. `engines` is "advisory
  only and will only produce warnings" unless `engine-strict` is set. `os` and
  `cpu` limit the platforms. `exports` defines entry points. `funding` feeds
  `npm fund`. `repository` "is helpful for people who want to contribute". —
  [npm Docs, package.json](https://docs.npmjs.com/cli/v11/configuring-npm/package-json)
- TypeScript: "If your package has a main `.js` file, you will need to
  indicate the main declaration file in your `package.json` file as well. Set
  the `types` property"; `typings` is a synonym; `typesVersions` is read
  first. The page does not cover `exports` conditions. —
  [TypeScript Handbook, Publishing](https://www.typescriptlang.org/docs/handbook/declaration-files/publishing.html)
- publint rules on metadata: `EXPORTS_TYPES_SHOULD_BE_FIRST`,
  `EXPORTS_TYPES_INVALID_FORMAT`, `TYPES_NOT_EXPORTED` ("When `exports`
  exists, the root `types` field is ignored"), `INVALID_REPOSITORY_VALUE`,
  `USE_LICENSE`, `USE_FILES`, `USE_TYPE`, `USE_ENGINES_NODE` ("A package that
  targets Node.js should declare `engines.node`"). —
  [publint rules](https://publint.dev/rules)

#### PyPI (core metadata, `pyproject.toml`) — USS module: Python

- Core metadata 2.6, "approved in May 2026". "The required fields are:"
  `Metadata-Version`, `Name`, `Version`; "All the other fields are
  optional." —
  [PyPA, Core metadata specifications](https://packaging.python.org/en/latest/specifications/core-metadata/)
- On `Description-Content-Type`: "PyPI will probably reject anything with an
  unrecognized value". If both the deprecated `License` and
  `License-Expression` are present, "PyPI will reject uploads". `License ::`
  classifiers are deprecated as of Metadata 2.4. `Requires-Python`: installers
  may use it to choose a version. `Project-URL` labels are at most 32
  characters; PEP 753 well-known labels get special display. —
  [PyPA, Core metadata specifications](https://packaging.python.org/en/latest/specifications/core-metadata/)
- `pyproject.toml`: only `name` must be static; `version` is required "either
  statically or listed as dynamic". `license` is "a valid SPDX license
  expression"; `license-files` takes glob patterns; build tools "MAY raise an
  error" when a `license` string and `License ::` classifiers are both used. —
  [PyPA, pyproject.toml specification](https://packaging.python.org/en/latest/specifications/pyproject-toml/)
- "PyPI will always reject packages with classifiers beginning with
  `Private ::`." The page says nothing about unknown classifiers in the part
  read. —
  [PyPI classifiers](https://pypi.org/classifiers/)
- twine 7.0.0: `twine check` "Checks whether your distribution's long
  description will render correctly on PyPI"; `--strict` means "Fail on
  warnings". —
  [twine documentation](https://twine.readthedocs.io/en/stable/)

#### Go modules and pkg.go.dev — USS module: Go

- "A `go.mod` file must contain exactly one `module` directive." The `go`
  directive "sets the minimum version of Go required to use this module.
  Before Go 1.21, the directive was advisory only; now it is a mandatory
  requirement: Go toolchains refuse to use modules declaring newer Go
  versions." `retract` and a `Deprecated:` comment on the `module` directive
  are reported by `go list -m -u` and `go get`. —
  [Go Modules Reference](https://go.dev/ref/mod)
- pkg.go.dev detects licences "using heuristics based on their file name and
  contents" with `github.com/google/licensecheck`; without a detectable
  licence "only limited package and module information will be made
  available". —
  [pkg.go.dev License Policy](https://pkg.go.dev/license-policy)
- pkg.go.dev's best-practice details are "Has go.mod file", "Redistributable
  license", "Tagged version" and "Stable version" (v1 or higher). "It's
  important to write a good summary of the package in the first sentence of
  the package comment." —
  [pkg.go.dev About](https://pkg.go.dev/about)

#### Maven Central — USS modules: Maven, Java

- Required: a POM with coordinates (version "cannot end in `-SNAPSHOT`"),
  "We require the presence of" `name`, `description` and `url`; licences;
  "Developer information is also required"; SCM "is another required
  element". A `-sources.jar` and a `-javadoc.jar` for each JAR (placeholders
  allowed "to pass validation"); `.md5` and `.sha1` checksums; every file
  signed with GPG (`.asc`). The page shows no date. —
  [Sonatype Central, Requirements](https://central.sonatype.org/publish/requirements/)

#### OCI images, GitHub Container Registry and Docker Hub — no USS module (core USS-DST-003, USS-DST-016)

- The `annotations` property is optional ("This property contains arbitrary
  metadata"); "Consumers MUST NOT generate an error if they encounter an
  unknown annotation key". Pre-defined keys: `created`, `authors`, `url`,
  `documentation`, `source`, `version`, `revision`, `vendor`, `licenses`
  ("as an SPDX License Expression"), `ref.name`, `title`, `description`,
  `base.digest`, `base.name`. No key is mandatory; `base.*` use SHOULD. I read
  the `main` branch, which states no version. —
  [OCI image-spec, annotations.md](https://github.com/opencontainers/image-spec/blob/main/annotations.md)
- GitHub's registry reads `org.opencontainers.image.source` ("The URL of the
  repository associated with the package"), `description` ("limited to 512
  characters") and `licenses` ("limited to 256 characters"). They are Docker
  labels for most images; "For multi-arch images, the description instead
  comes from the `annotations` field in the image's manifest". —
  [GitHub Docs, Working with the Container registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)

#### Helm charts and Artifact Hub — USS module: Kubernetes and Helm

- `Chart.yaml`: `apiVersion`, `name` and `version` are "(required)"; all
  others optional: `kubeVersion`, `description`, `type`, `keywords`, `home`,
  `sources`, `dependencies`, `maintainers`, `icon`, `appVersion`,
  `deprecated`, `annotations`. "Every chart must have a version number." Helm
  validates `kubeVersion` at install "and fail[s] if the cluster runs an
  unsupported" Kubernetes version. The page is labelled Helm 4.3.1 but says
  it "has not yet been updated for Helm 4". —
  [Helm Docs, Charts](https://helm.sh/docs/topics/charts/)
- Artifact Hub reads `artifacthub.io/*` annotations in `Chart.yaml`, such as
  `license` (an SPDX identifier that overrides the `LICENSE` file), `links`
  (a link named `support` gets a highlighted issue link), `changes`,
  `containsSecurityUpdates`, `images`, `prerelease`, `signKey` (its `url` "is
  mandatory when a sign key entry is provided"), `category`, `maintainers`,
  `screenshots`. —
  [Artifact Hub, Helm annotations](https://artifacthub.io/docs/topics/annotations/helm/)
- Helm's standard labels: REC `app.kubernetes.io/name`, `helm.sh/chart`,
  `app.kubernetes.io/managed-by`, `app.kubernetes.io/instance`; OPT
  `app.kubernetes.io/version`, `component`, `part-of`. "Helm itself never
  requires that a particular label be present." —
  [Helm Docs 4.3.1, Labels and Annotations](https://helm.sh/docs/chart_best_practices/labels/)
- Kubernetes recommended labels: `app.kubernetes.io/name`, `instance`,
  `version`, `component`, `part-of`, `managed-by`. "These are recommended
  labels. They make it easier to manage applications but aren't required for
  any core tooling." They "should be applied on every resource object". —
  [Kubernetes Docs, Recommended Labels](https://kubernetes.io/docs/concepts/overview/working-with-objects/common-labels/)

#### Claude Code plugins and marketplaces — no USS module (core USS-DST-008 example)

- "The manifest is optional." "`name` is the only required key"; kebab-case;
  names that pass as Anthropic's own are errors. Optional metadata:
  `displayName`, `version` ("Setting it keeps users on that version until you
  change it"), `description`, `author` (`name` required inside it),
  `homepage` ("Must parse as a URL"), `repository` ("Not validated"),
  `license` (SPDX identifier), `keywords`. —
  [Claude Code Docs, Plugin manifest reference](https://code.claude.com/docs/en/plugins-reference)
- `engines`: "The Claude Code versions that the plugin works on, as a semver
  range under the `claude-code` key ... Requires Claude Code v2.1.323 or
  later. To keep the plugin working on v2.0.15 through v2.1.75, omit
  `engines`. Those versions reject the manifest". —
  [Claude Code Docs, Plugin manifest reference](https://code.claude.com/docs/en/plugins-reference)
- `icon`, `documentationUrl`, `supportUrl`, `privacyPolicyUrl` and
  `termsOfServiceUrl` are read only by Anthropic's directory; "Claude Code
  ignores them at load time". —
  [Claude Code Docs, Plugin manifest reference](https://code.claude.com/docs/en/plugins-reference)
- "`claude plugin validate` is the authoritative check for a manifest."
  "Validation passed with warnings" covers an unknown top-level field, a
  non-kebab-case `name`, "or a missing `version`, `description`, or
  `author`. Pass `--strict` to turn warnings into failures in CI". —
  [Claude Code Docs, Plugin manifest reference](https://code.claude.com/docs/en/plugins-reference)
- `marketplace.json`: "`name`, `owner`, and `plugins` are required"; `owner`
  needs `name`; each entry needs `name` and `source`. `description` is
  optional, and "`claude plugin validate` warns when it's missing";
  `metadata.description` and `metadata.version` are an "Alternate location".
  The page lists reserved marketplace names. —
  [Claude Code Docs, Marketplace reference](https://code.claude.com/docs/en/plugins/marketplace-reference.md)

#### VS Code Marketplace and Open VSX — no USS module (core USS-DST-006; USS-EXT-003 cites the VS Code manifest)

- Required: `name` ("must be unique to the Marketplace"), `version` (SemVer),
  `publisher`, `engines` ("at least the `vscode` key ... Cannot be `*`").
  Optional: `displayName`, `description`, `categories` (a fixed list),
  `keywords` ("limited to 30"), `galleryBanner`, `icon` ("at least 128x128
  pixels"), `license`, `repository`, `bugs`, `homepage` (shown under
  Resources), `pricing`, `qna`, `sponsor`. Badges only "from trusted
  services". —
  [VS Code, Extension Manifest](https://code.visualstudio.com/api/references/extension-manifest)
- "`vsce` will not publish extensions that contain user-provided SVG images";
  the icon "may not be an SVG"; "Image URLs in `README.md` and `CHANGELOG.md`
  need to resolve to `https` URLs". "The Marketplace requires the extension
  name to be unique"; a removed name "is permanently reserved". Versions use
  `major.minor.patch` only. —
  [VS Code, Publishing Extensions](https://code.visualstudio.com/api/working-with-extensions/publishing-extension)
- `vsce` source: it throws on "Manifest missing field: engines", "Failed to
  parse semver of engines.vscode", a README with template text, SVG images,
  non-HTTPS images and an SVG or missing icon. It warns and prompts for "A
  'repository' field is missing" (`--allow-missing-repository`) and a missing
  `LICENSE` file. —
  [vscode-vsce, src/package.ts](https://raw.githubusercontent.com/microsoft/vscode-vsce/main/src/package.ts)
- Open VSX: the `publisher` field "defines the namespace", which must exist
  before publishing; publishing needs a signed Eclipse publisher agreement.
  The page says nothing about licences. —
  [Open VSX wiki, Publishing Extensions](https://github.com/eclipse/openvsx/wiki/Publishing-Extensions)

#### JetBrains Marketplace — no USS module

- `plugin.xml`: `name`, `version`, `idea-version`, `vendor` and `description`
  are required; `id` is not required but "highly recommended" and "cannot be
  changed later after public release"; `change-notes` is optional. "Plugins
  uploaded to the JetBrains Marketplace must follow semantic versioning."
  `since-build` is "The lowest IDE version compatible with the plugin"; for
  `until-build`, "It's highly recommended not to set this attribute". —
  [IntelliJ Platform SDK, Plugin Configuration File](https://plugins.jetbrains.com/docs/intellij/plugin-configuration-file.html)
- Approval guidelines version 1.3, effective 31 March 2026: name at most 30
  Latin characters without "Plugin", "IntelliJ", "JetBrains" or a product
  name; description in English; vendor website and email "valid, and
  functional"; a Developer EULA; a source link for open-source plugins; no
  placeholder change notes; a 40 px SVG icon unlike the template; and "During
  each upload ... compatibility must be verified" with the Plugin Verifier. —
  [JetBrains Marketplace Approval Guidelines](https://plugins.jetbrains.com/docs/marketplace/jetbrains-marketplace-approval-guidelines.html)

#### GitHub Actions and GitHub Marketplace — no USS module (USS-SUP-003 covers using actions)

- `action.yml`: `name`, `description` and `runs` are "**Required**"; `author`,
  `inputs`, `outputs` and `branding` are optional. `branding` sets a badge
  "shown next to your action name in GitHub Marketplace". —
  [GitHub Docs, Metadata syntax](https://docs.github.com/en/actions/reference/workflows-and-actions/metadata-syntax)
- Marketplace: "The action must be in a public repository"; "a single action
  metadata file (`action.yml` or `action.yaml`) at the root"; the `name` must
  be unique and must not match an existing action, a user or organisation
  (unless it is the owner), a Marketplace category or a reserved GitHub
  feature name; terms acceptance and two-factor authentication; actions are
  published on a release tag and "aren't reviewed by GitHub". —
  [GitHub Docs, Publishing actions in GitHub Marketplace](https://docs.github.com/en/actions/how-tos/create-and-publish-actions/publish-in-github-marketplace)

#### Homebrew and winget — no USS module (core USS-DST-001 names both)

- Homebrew: "We don't accept formulae without a `homepage`!" and "We don't
  accept new formulae into Homebrew/homebrew-core without a license!", using
  "the licence identifier from the SPDX License List". `desc` is one line,
  following the Description Guidelines. `brew audit --strict --online` checks
  house style; `brew audit --new --formula foo` "highlights more potential
  issues". —
  [Homebrew, Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- winget (page dated 2026-09-14, schema 1.12.0): the minimal required schema
  is `PackageIdentifier`, `PackageVersion`, `PackageLocale`, `Publisher`,
  `PackageName`, `License`, `ShortDescription`, `Installers` (`Architecture`,
  `InstallerType`, `InstallerUrl`, `InstallerSha256`), `ManifestType` and
  `ManifestVersion`. "Provide as many fields as possible. The more meta-data
  you provide the better the user experience will be." `PackageName` and
  `Publisher` should match Add/Remove Programs "to support export, and
  upgrade". —
  [Microsoft Learn, Create your package manifest](https://learn.microsoft.com/en-us/windows/package-manager/package/manifest)

### Inferences

- The required set is small and stable everywhere: an identity (name), a
  version, and often a description, a licence and a publisher or owner. A core
  MUST can name the outcome and leave the field list to the host's own
  documentation and validator.
- Two kinds of "parent" exist. A *registry or marketplace* refuses the upload
  (crates.io, PyPI, Maven Central, VS Code Marketplace, JetBrains). A *host
  tool* that loads the item straight from a repository refuses to load it
  (Claude Code from a git marketplace, GitHub Actions by `uses:`, Go by
  module path, Helm from a chart repository). With no upload gate, the user
  meets the failure, which is why the MUST is useful.
- Validators separate the two levels: errors map to required metadata (the
  MUST), and warnings or strict mode map to recommended metadata (the
  SHOULD). Examples: `claude plugin validate` and `--strict`; `vsce` throws
  and prompts; `twine check` and `--strict`.
- Host-specific field lists belong in modules where USS has a module (Rust,
  TypeScript, Python, Go, Maven, Kubernetes). The other hosts can rely on the
  core rule's Verify ("the host's documentation and validator at the audit
  date") until a module exists.

### Gaps

- I did not read the docs for `npm publish --dry-run`, `winget validate`,
  `ovsx publish`, Artifact Hub's linter, the IntelliJ Platform Gradle Plugin's
  verification tasks, or Central Portal "validate without publishing". The
  notes name them as candidate validators only.
- I found no statement that PyPI rejects unknown classifiers, that Open VSX
  requires a licence, that `brew audit` limits the length of `desc`, or that
  JetBrains sets a minimum description length.
- I did not research Docker Hub. I know of no required metadata there beyond
  the repository name and tag, and I did not confirm that.
- The OCI annotations page was read on `main`. USS-DST-003 cites v1.1.1, so the
  key list needs a check against the v1.1.1 tag before a rule cites it.
- The GitHub Marketplace page no longer says whether workflow files are
  forbidden in an action repository.

## Q3. Which compatibility metadata matters most to users?

### Takeaway

The declared minimum version of the runtime, toolchain or host matters most,
because tools act on it (Cargo errors, the Go toolchain refuses, Helm fails the
install, VS Code requires it). The SPDX licence (already USS-LIC-001), type
declarations for TypeScript libraries, and source and issue links come next.
Keywords and categories help discovery. Platform fields are mostly automatic.

### Cited Findings

- Cargo reports an unsupported toolchain as an error, `cargo add` picks
  compatible dependency versions and the resolver may use `rust-version`. —
  [The Cargo Book, Rust Version](https://doc.rust-lang.org/cargo/reference/rust-version.html)
- Go toolchains "refuse to use modules declaring newer Go versions" than
  themselves. — [Go Modules Reference](https://go.dev/ref/mod)
- Helm validates `kubeVersion` at install and fails on an unsupported
  cluster. — [Helm Docs, Charts](https://helm.sh/docs/topics/charts/)
- `engines.vscode` is required; `engines.node` is advisory unless
  `engine-strict` is set. —
  [VS Code, Extension Manifest](https://code.visualstudio.com/api/references/extension-manifest);
  [npm Docs, package.json](https://docs.npmjs.com/cli/v11/configuring-npm/package-json)
- `Requires-Python` lets installers choose a version. —
  [PyPA, Core metadata specifications](https://packaging.python.org/en/latest/specifications/core-metadata/)
- `idea-version since-build` is required and states the lowest compatible IDE
  build. —
  [IntelliJ Platform SDK, Plugin Configuration File](https://plugins.jetbrains.com/docs/intellij/plugin-configuration-file.html)
- `engines.claude-code` exists, and Claude Code v2.0.15 to v2.1.75 reject a
  manifest that has it. —
  [Claude Code Docs, Plugin manifest reference](https://code.claude.com/docs/en/plugins-reference)
- When `exports` exists, the root `types` field is ignored, so types must be
  declared in `exports`. — [publint rules](https://publint.dev/rules)
- GitHub's registry links an image to its repository through
  `org.opencontainers.image.source`. —
  [GitHub Docs, Working with the Container registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
- VS Code shows `bugs`, `homepage` and `repository` under Resources;
  crates.io renders the README. —
  [VS Code, Extension Manifest](https://code.visualstudio.com/api/references/extension-manifest);
  [The Cargo Book, The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html)
- The spec already holds that "support that CI does not prove is only a
  claim" (Independence row, section 6.1). —
  [docs/superpowers/specs/2026-11-21-universal-software-standard-design.md](../../../../../docs/superpowers/specs/2026-11-21-universal-software-standard-design.md)

### Inferences

- Ranking (my judgement from the findings): (1) the declared minimum
  runtime, toolchain or host version; (2) the SPDX licence, already
  USS-LIC-001; (3) TypeScript type declarations reached through `exports`;
  (4) source repository and issue-tracker links; (5) the README shown on the
  registry; (6) keywords and categories; (7) platform fields (`os`, `cpu`,
  VS Code targets, winget `Architecture`). Wheel tags, VS Code targets and OCI
  indexes mostly set platforms automatically, and USS-DST-001, USS-DST-002 and
  USS-DST-014 already cover documented platforms.
- A declared minimum must be tested to be worth anything. That is the same
  idea as USS-DEP-016 (the widest *tested* range) and the spec's "support that
  CI does not prove is only a claim".
- Interaction to flag: USS-PY-007, USS-TS-008 and USS-GO-007 forbid
  end-of-life versions in the CI matrix. A rule that requires the declared
  minimum to be tested therefore stops a project from claiming an end-of-life
  minimum. I think that is the right outcome for USS, but the module
  rationales today leave a declared minimum unjudged, so the maintainer has to
  choose (see Q6).

### Gaps

- I found no user survey that ranks these fields. The ranking is a judgement
  from how tools act on each field.

## Q4. Which repository-level metadata deserves a rule?

### Takeaway

The repository host's description and topics deserve a SHOULD. `CITATION.cff`
deserves at most a MAY. `FUNDING.yml`, `publiccode.yml` and `codemeta.json`
do not deserve a rule now.

### Cited Findings

- GitHub topics: "To help other people find and contribute to your project,
  you can add topics to your repository." Topics use lowercase letters,
  numbers and hyphens, at most 50 characters each and 20 per repository, and
  are set in the About section. —
  [GitHub Docs, Classifying your repository with topics](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/classifying-your-repository-with-topics)
- `CITATION.cff` in the default branch root adds "Cite this repository" to the
  sidebar, with APA and BibTeX. —
  [GitHub Docs, About CITATION files](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-citation-files)
- Citation File Format schema 1.2.0; the required keys are `cff-version`,
  `message`, `title` and `authors`; the guide links a JSON Schema. —
  [CFF schema guide](https://raw.githubusercontent.com/citation-file-format/citation-file-format/main/schema-guide.md)
- `FUNDING.yml` in the `.github` folder shows a sponsor button: one account
  per external platform, up to four custom URLs, one organisation and up to
  four developers on GitHub Sponsors. —
  [GitHub Docs, Displaying a sponsor button](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/displaying-a-sponsor-button-in-your-repository)
- publiccode.yml is a metadata standard for software "developed or acquired
  by the Public Administration", read by national catalogues; v0.8.0 is the
  latest, with a 1.0-rc; the file is mandatory for public software developed
  in Italy. —
  [publiccode.yml standard](https://publiccodeyml.github.io/v0/)
- CodeMeta is "a shared, minimal metadata vocabulary for software" built on
  Schema.org and JSON-LD, with crosswalks between formats. The page states no
  version. — [CodeMeta](https://codemeta.github.io/)

### Inferences

- Description and topics: SHOULD. Any project that publishes releases gains
  from them, they cost a minute, and an auditor can read them from the host.
  AGENTS.md already lets a Verify read a repository-host setting.
- `CITATION.cff`: MAY (category Features), behind `publishes-releases`. It is
  cheap and useful for research users, but most software is not cited, so a
  SHOULD would make most projects record a Standard Deviation for nothing.
  Leaving it out is also defensible.
- `FUNDING.yml`: no rule. Asking for money is a business choice, not an
  engineering quality, and commercial projects have no use for it.
- `publiccode.yml`: no rule until USS has a public-sector fact. It could then
  be a MUST in a regulation-style module where a government mandates it, as in
  Italy.
- `codemeta.json`: no rule. It overlaps `CITATION.cff`, and the sources I read
  show no host that reads it the way GitHub reads `CITATION.cff`.

### Gaps

- I did not confirm on a source that GitHub shows the repository description
  in search results and link previews. The GitHub topics page does not
  mention the description or website fields.
- I did not find a list of who reads `codemeta.json`; the CodeMeta page shows
  Software Heritage and Zenodo only as supporters.

## Q5. How should a Verify handle a host rule that changes over time?

### Takeaway

Judge the audited commit against the host's documentation and validator at
the audit date. Prefer the host's own validator and give its version and output
as evidence. Pin the documents read when the rule is written with
`(read YYYY-MM-DD)`. Keep host-specific field lists in modules, and review them
at each USS release.

### Cited Findings

- Conventions: "A document without a version is pinned by its publication
  date or, for a page that changes in place, by the date it was read, written
  `(read YYYY-MM-DD)`", and "Each release of the standard MUST review whether
  to adopt newer versions of the standards that its rules reference". —
  [standard/conventions.md](../../../../../standard/conventions.md) (section 7)
- Precedent for "at the audit date": USS-PY-007 ("Take the status table of the
  Python Developer's Guide at the audit date"), USS-TS-008 ("Take the release
  schedule of Node.js at the audit date") and USS-GO-007 ("Take the Go release
  policy at the audit date"). —
  [standard/modules/languages/python.md](../../../../../standard/modules/languages/python.md);
  [standard/modules/languages/typescript.md](../../../../../standard/modules/languages/typescript.md);
  [standard/modules/languages/go.md](../../../../../standard/modules/languages/go.md)
- Precedent for a tool outside its checked versions: the auditor checks that
  each rule or setting exists in that version (conventions section 12). —
  [standard/conventions.md](../../../../../standard/conventions.md)
- Drift seen on 2026-11-25: crates.io uses SPDX License List 3.20 while USS
  cites 3.29.0
  ([The Cargo Book, The Manifest Format](https://doc.rust-lang.org/cargo/reference/manifest.html));
  the npm page is labelled "11.21.3 (Legacy)"
  ([npm Docs, package.json](https://docs.npmjs.com/cli/v11/configuring-npm/package-json));
  the Helm 4.3.1 page "has not yet been updated for Helm 4"
  ([Helm Docs, Charts](https://helm.sh/docs/topics/charts/)); `claude plugin
  validate` results depend on the Claude Code version (directory-listing
  fields warn before v2.1.281, and `engines` needs v2.1.323)
  ([Claude Code Docs, Plugin manifest reference](https://code.claude.com/docs/en/plugins-reference)).

### Inferences

- Recommended Verify pattern:
  1. List the hosts.
  2. For each, take its documentation of required metadata at the audit date.
  3. Run its validator on the audited commit where one exists. Give the
     command, the validator's version and its output as evidence.
  4. Where no validator exists, check each required field by hand.
- A host that adds a requirement after a release does not fail that release:
  the host accepted it. The next audit judges the manifests of the audited
  commit, which are what the next release publishes.
- Do not copy volatile limits (5 keywords on crates.io, 30 on VS Code, 512
  characters on GHCR) into rules. The validator enforces them, and modules
  can name them as examples with a read date.
- MUST covers validator errors (required). SHOULD covers validator warnings
  or strict mode (recommended). Some strict modes warn about fields that only
  a host the project does not use reads, such as the Anthropic directory icon
  (Q7). The SHOULD's Verify must not judge those.

### Gaps

- None beyond Q2's unread validator docs.

## Q6. What rule design is recommended?

### Takeaway

Add a `## Metadata` section at the end of the Distribution and install (DST)
area with one MUST and five SHOULD or MAY rules (USS-DST-017 to USS-DST-022).
Gate them on the existing fact `distributed-as-package` (with a clearer
Definition), `ships-container-image`, or the area header alone. Refine the
MUST and the two package SHOULDs in the Rust, TypeScript, Python, Go, Maven and
Kubernetes modules, and add a Clippy profile mapping. Add no new fact.

### Cited Findings

- DST's scope is "the ways in which the software reaches its users and stays
  current", its header is `Applies when: publishes-releases`, and its last rule
  is USS-DST-016. —
  [standard/core/distribution.md](../../../../../standard/core/distribution.md)
- A module rule that refines core rules "MUST apply only where each of them
  applies", has the strength of the strongest of them, and "A SHOULD rule that
  is not a tool rule MUST NOT refine a MUST rule". A checking tool's
  configuration belongs in its vendor profile. —
  [standard/conventions.md](../../../../../standard/conventions.md) (section 7)
- "Use MUST or MUST NOT when the absence is a defect in every project the rule
  applies to, and any team of any size can meet the rule"; "Use SHOULD ...
  when the practice is valuable but legitimate reasons to skip it exist". —
  [standard/conventions.md](../../../../../standard/conventions.md) (section 7)
- USS-EXT-003 is one MUST that lists five manifest fields, so one rule can
  name a set of fields for one outcome. —
  [standard/core/extensibility.md](../../../../../standard/core/extensibility.md)
- Last rule numbers: RUST-011, TS-017, PY-016, GO-014, MVN-009, K8S-008. The
  Kubernetes module header is `platform.kubernetes or platform.helm`. —
  [standard/modules/](../../../../../standard/modules/)
- Clippy's `cargo_common_metadata` is in the `cargo` group, allow by
  default. —
  [Clippy lint list](https://clippy.rust-lang.org/master/index.html)

### Inferences

#### Area, levels and categories

| ID | Title | Level | Applies when (and DST's `publishes-releases`) | Category |
|---|---|---|---|---|
| USS-DST-017 | Carry the metadata that each host requires | MUST | `distributed-as-package` | Compatibility |
| USS-DST-018 | Declare the tested runtime and host versions | SHOULD | `distributed-as-package` | Compatibility |
| USS-DST-019 | Fill the listing metadata | SHOULD | `distributed-as-package` | Accessibility & Usability |
| USS-DST-020 | Annotate published images | SHOULD | `ships-container-image` | Compatibility |
| USS-DST-021 | Describe the repository on its host | SHOULD | `always` | Accessibility & Usability |
| USS-DST-022 | Provide citation metadata | MAY | `always` | Features |

- Why DST: publishing to a host is how the software reaches users, and the
  header keeps the all-false budget unchanged. LIC keeps the licence field,
  DOC keeps the root README, EXT keeps the project's own plugin host.
- Why Accessibility & Usability for USS-DST-019 and USS-DST-021: their effect
  is that users find and understand the software. Compatibility is the
  alternative if the maintainer wants all of DST's metadata rules in one
  category.
- USS-DST-020 is separate from USS-DST-019 because images have no USS module
  and a different mechanism (annotations), as USS-DST-016 is separate for
  images.
- Optional: USS-DST-022 can be dropped (Q4).

#### Facts

Reuse `distributed-as-package`. No rule uses it today, so its Definition can be
made precise before 0.1.0. Proposed text (`standard/applicability.md`):

```markdown
### distributed-as-package

- **Question:** Is it released as a package, a binary, a chart, or a plugin or extension of another tool?
- **Definition:** Yes when it is released as something that people install or that another tool fetches or loads: a package in a registry (such as crates.io, npm, PyPI or Maven Central), a library module that others fetch, a binary, a Helm chart, or a plugin, extension or action for another tool (such as a Claude Code plugin, a VS Code extension or a GitHub Action). A container image alone does not count: `ships-container-image` covers it.
- **Implies:** publishes-releases
```

A new fact such as `publishes-to-registry` was considered and rejected: it
would add a question to every record and mostly duplicate
`distributed-as-package`. Records that answer `distributed-as-package` today
keep their answer unless they ship only a plugin, extension or action; those
change from `false` to `true`.

#### Core drafts

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

Notes on the core drafts:

- USS-DST-018 uses equality (lowest declared equals lowest tested), as
  USS-DEP-016 does for dependency ranges. Because USS-PY-007, USS-TS-008 and
  USS-GO-007 keep end-of-life versions out of the CI matrix, the declared
  minimum can no longer be an end-of-life version. If the maintainer wants to
  allow that, change the last two Verify sentences to "A lowest version that
  CI does not test fails, unless it is end-of-life", and accept untested
  claims. I recommend the drafted text.
- A host-tool minimum (`engines.claude-code`, VS Code `engines.vscode`,
  `kubeVersion`) needs a CI job on the oldest host version. That is real work
  for plugin authors. It is a SHOULD, so a Standard Deviation is the escape.
- Platform fields (`os`, `cpu`, `MinimumOSVersion`) are left out of
  USS-DST-018 on purpose (Q3). The TypeScript module can add one for native
  packages later.

#### Module refinements

Rust (`standard/modules/languages/rust.md`):

```markdown
### USS-RUST-012: Set the metadata that crates.io requires

- **Level:** MUST
- **Applies when:** distributed-as-package
- **Category:** Compatibility
- **Refines:** USS-DST-017
- **Verify:** Take each package that the project publishes to crates.io. Its
  `Cargo.toml` sets `version` and a plain-text `description`.
  `cargo publish --dry-run` exits with status 0 for it. A missing field or a
  failed dry run fails. USS-LIC-001 judges the `license` field.
- **References:** The Cargo Book, The Manifest Format and Publishing on
  crates.io (read 2026-11-25)

Each crate that the project publishes to crates.io MUST set `version` and
`description` in `Cargo.toml`.

**Rationale:** crates.io refuses a crate without a description or a licence,
and Cargo needs a version to publish. The dry run finds the other problems
before the release.

### USS-RUST-013: Declare and test rust-version

- **Level:** SHOULD
- **Applies when:** distributed-as-package
- **Category:** Compatibility
- **Refines:** USS-DST-018
- **Verify:** Each package that the release publishes, or that users build
  with `cargo install`, sets `rust-version` in `Cargo.toml`. CI builds and
  tests it with the Rust toolchain of that version. A toolchain that
  USS-RUST-007 pins at that version counts. A missing `rust-version`, or a
  version that CI does not build and test, fails.
- **References:** The Cargo Book, Rust Version (read 2026-11-25)

Each published package SHOULD set `rust-version` to the Rust version that CI
builds and tests it with.

**Rationale:** Cargo stops with a clear error on an older toolchain, and its
resolver can choose dependencies that fit. The Cargo Book expects support for
a Rust version to be verified by automated tests.

### USS-RUST-014: Fill the crates.io listing

- **Level:** SHOULD
- **Applies when:** distributed-as-package
- **Category:** Accessibility & Usability
- **Refines:** USS-DST-019
- **Verify:** Take each package that the project publishes to crates.io. Its
  `Cargo.toml` sets `repository`, `keywords` and `categories`. Cargo finds a
  README for it, through `readme` or a `README.md` beside the manifest. Each
  category is a slug on crates.io's category list. A missing field, or a
  category that is not on the list, fails.
- **References:** The Cargo Book, The Manifest Format (read 2026-11-25)

Each crate that the project publishes to crates.io SHOULD set `repository`,
`keywords` and `categories`, and SHOULD have a README that Cargo finds.
```

Clippy profile (`standard/vendors/rust-lang/clippy.md`): add
`cargo_common_metadata = "deny"` to the baseline's `[workspace.lints.clippy]`
table, and add a mapping row in the Accessibility & Usability block:
`| USS-RUST-014 | Accessibility & Usability | clippy::cargo_common_metadata | partial: the lint checks that the common fields are set, not that each category is a valid slug | Run cargo clippy; compare each category with crates.io's list. |`.
The lint's option `cargo-ignore-publish` exists to ignore the `publish`
setting, which suggests that by default it skips crates with
`publish = false`. Confirm that with a run before the baseline changes.

TypeScript and JavaScript (`standard/modules/languages/typescript.md`):

```markdown
### USS-TS-018: Set the metadata that npm requires

- **Level:** MUST
- **Applies when:** distributed-as-package
- **Category:** Compatibility
- **Refines:** USS-DST-017
- **Verify:** Take each package that the project publishes to an npm
  registry. Its `package.json` sets `name` and `version`. A dry-run publish
  with the project's package manager, such as `npm publish --dry-run`, exits
  with status 0. A missing field or a failed dry run fails.
- **References:** npm Docs, package.json (npm 11.21.3, read 2026-11-25)

Each package that the project publishes to an npm registry MUST set `name`
and `version` in `package.json`.

### USS-TS-019: Declare and test engines.node

- **Level:** SHOULD
- **Applies when:** distributed-as-package
- **Category:** Compatibility
- **Refines:** USS-DST-018
- **Verify:** Each package that the release publishes for Node.js sets
  `engines.node`. The lowest Node.js release line that the range admits is
  the lowest line in the CI test matrix. A missing field, or a lowest line
  that CI does not test, fails. USS-TS-008 judges end-of-life lines.
- **References:** npm Docs, package.json (npm 11.21.3, read 2026-11-25);
  publint rules, `USE_ENGINES_NODE` (read 2026-11-25)

Each published package for Node.js SHOULD set `engines.node` to a range whose
lowest release line CI tests.

### USS-TS-020: Fill the npm listing

- **Level:** SHOULD
- **Applies when:** distributed-as-package
- **Category:** Accessibility & Usability
- **Refines:** USS-DST-019
- **Verify:** Take each package that the project publishes to an npm
  registry. Its `package.json` sets `description`, `keywords`, `repository`
  and `bugs`, and the package holds a README. publint reports no
  `INVALID_REPOSITORY_VALUE` for it. A missing field fails.
- **References:** npm Docs, package.json (npm 11.21.3, read 2026-11-25);
  publint rules (read 2026-11-25)

Each package that the project publishes to an npm registry SHOULD set
`description`, `keywords`, `repository` and `bugs`, and SHOULD ship a README.

### USS-TS-021: Publish type declarations through exports

- **Level:** SHOULD
- **Applies when:** is-library and distributed-as-package
- **Category:** Compatibility
- **Verify:** Take each library package that the release publishes. Each
  entry point in `exports` has a `types` condition, listed first. A package
  without `exports` sets `types`. publint reports none of
  `EXPORTS_TYPES_SHOULD_BE_FIRST`, `EXPORTS_TYPES_INVALID_FORMAT` and
  `TYPES_NOT_EXPORTED`. An entry point without types fails.
- **References:** TypeScript Handbook, Publishing (read 2026-11-25); publint
  rules (read 2026-11-25)

Each published library SHOULD declare its types for each entry point that its
`exports` field names.

**Rationale:** When `exports` exists, TypeScript ignores the root `types`
field, so a library without a `types` condition looks untyped to its users.
```

Python (`standard/modules/languages/python.md`):

```markdown
### USS-PY-017: Set the metadata that PyPI requires

- **Level:** MUST
- **Applies when:** distributed-as-package
- **Category:** Compatibility
- **Refines:** USS-DST-017
- **Verify:** Take each distribution that the project publishes to PyPI. Its
  `[project]` table sets `name`, and sets `version` or lists it in
  `dynamic`. No classifier starts with `Private ::`. Build the
  distributions. `twine check --strict` on them exits with status 0. A missing
  field, a `Private ::` classifier or a failed check fails.
- **References:** PyPA, Core metadata specifications 2.6; PyPA,
  pyproject.toml specification (read 2026-11-25); PyPI classifiers (read
  2026-11-25); twine 7.0.0

Each distribution that the project publishes to PyPI MUST carry the core
metadata that PyPI requires, and its description MUST render on PyPI.
```

The PY-017 body states two requirements. Split it if the maintainer prefers
strict one-requirement bodies; the second part can move to PY-019.

```markdown
### USS-PY-018: Declare and test requires-python

- **Level:** SHOULD
- **Applies when:** distributed-as-package
- **Category:** Compatibility
- **Refines:** USS-DST-018
- **Verify:** Each distribution that the release publishes sets
  `requires-python`. The lowest Python version that it admits is the lowest
  version in the CI test matrix. A missing field, or a lowest version that CI
  does not test, fails. USS-PY-007 judges end-of-life versions.
- **References:** PyPA, pyproject.toml specification (read 2026-11-25)

Each published distribution SHOULD set `requires-python` to a range whose
lowest version CI tests.

### USS-PY-019: Fill the PyPI listing

- **Level:** SHOULD
- **Applies when:** distributed-as-package
- **Category:** Accessibility & Usability
- **Refines:** USS-DST-019
- **Verify:** Take each distribution that the project publishes to PyPI. Its
  `[project]` table sets `description`, `readme`, `keywords` and
  `classifiers`, and `[project.urls]` has a link to the source and a link to
  the issue tracker. A missing field fails.
- **References:** PyPA, pyproject.toml specification (read 2026-11-25); PyPA,
  Core metadata specifications 2.6 (`Project-URL`, PEP 753)

Each distribution that the project publishes to PyPI SHOULD set
`description`, `readme`, `keywords`, `classifiers`, and links to its source and
its issue tracker.
```

Go (`standard/modules/languages/go.md`):

```markdown
### USS-GO-015: Set the go directive to a tested version

- **Level:** SHOULD
- **Applies when:** distributed-as-package
- **Category:** Compatibility
- **Refines:** USS-DST-018
- **Verify:** The `go` directive of each published module's `go.mod` names
  the lowest Go release in the CI test matrix. A directive above or below
  that release fails. USS-GO-007 judges supported releases.
- **References:** Go Modules Reference (read 2026-11-25)

The `go` directive of each published module SHOULD name the lowest Go release
that CI tests.

**Rationale:** Since Go 1.21 the directive is a mandatory minimum. `go mod
init` writes the developer's own release, which can lock out users who could
build the module.
```

The claim that `go mod init` writes the current release is from memory, not a
source read today; confirm it before the rationale uses it. A Go refinement of
USS-DST-017 (module path and major-version suffix) is a candidate, but I did
not read the major-version section of the reference today.

Maven (`standard/modules/tools/maven.md`):

```markdown
### USS-MVN-010: Meet the Maven Central POM requirements

- **Level:** MUST
- **Applies when:** distributed-as-package
- **Category:** Compatibility
- **Refines:** USS-DST-017
- **Verify:** Take each artifact that the project publishes to Maven Central.
  Its POM has `groupId`, `artifactId`, a `version` that does not end in
  `-SNAPSHOT`, `name`, `description`, `url`, `licenses`, `developers` and
  `scm`. The latest release deployment passed the Central Portal's
  validation. A missing element fails. USS-SUP-014 judges the signatures.
- **References:** Sonatype Central, Requirements (read 2026-11-25)

Each artifact that the project publishes to Maven Central MUST have a POM
with the elements that Maven Central requires.
```

Kubernetes and Helm (`standard/modules/platforms/kubernetes.md`):

```markdown
### USS-K8S-009: Set the metadata that Helm requires

- **Level:** MUST
- **Applies when:** platform.helm and distributed-as-package
- **Category:** Compatibility
- **Refines:** USS-DST-017
- **Verify:** Each published chart's `Chart.yaml` sets `apiVersion: v2`,
  `name` and a SemVer `version`. `helm lint --strict` (USS-K8S-004) and
  `helm package` succeed on it. A missing field or a failed command fails.
- **References:** Helm 4.3.1 documentation, Charts (read 2026-11-25)

Each published chart MUST set `apiVersion`, `name` and `version` in
`Chart.yaml`.

### USS-K8S-010: Declare and test kubeVersion

- **Level:** SHOULD
- **Applies when:** platform.helm and distributed-as-package
- **Category:** Compatibility
- **Refines:** USS-DST-018
- **Verify:** Each published chart sets `kubeVersion`. The lowest Kubernetes
  minor version that it admits is the lowest version on which CI installs
  the chart. A missing field, or a lowest version that CI does not install
  on, fails.
- **References:** Helm 4.3.1 documentation, Charts (read 2026-11-25)

Each published chart SHOULD set `kubeVersion` to a range whose lowest version
CI installs the chart on.

### USS-K8S-011: Fill the chart listing

- **Level:** SHOULD
- **Applies when:** platform.helm and distributed-as-package
- **Category:** Accessibility & Usability
- **Refines:** USS-DST-019
- **Verify:** Each published chart's `Chart.yaml` sets `description`,
  `home` or `sources`, `maintainers`, `keywords` and `icon`. A missing field
  fails.
- **References:** Helm 4.3.1 documentation, Charts (read 2026-11-25);
  Artifact Hub, Helm annotations (read 2026-11-25)

Each published chart SHOULD set `description`, a source or home link,
`maintainers`, `keywords` and `icon` in `Chart.yaml`.

### USS-K8S-012: Label each object with the recommended labels

- **Level:** SHOULD
- **Applies when:** deploys-to-kubernetes
- **Category:** Compatibility
- **Verify:** Render each chart with its default values (`helm template`),
  and take each manifest. Each object has `app.kubernetes.io/name`,
  `app.kubernetes.io/instance` and `app.kubernetes.io/managed-by`. Each object
  that a chart renders also has `helm.sh/chart`. An object without one of
  them fails.
- **References:** Kubernetes Documentation, Recommended Labels (read
  2026-11-25); Helm 4.3.1 documentation, Labels and Annotations (read
  2026-11-25)

Each object that the project ships SHOULD carry the recommended
`app.kubernetes.io` labels.

**Rationale:** Tools and people select, group and trace objects by these
labels. Kubernetes asks for them on every object to get their full benefit.
```

Hosts without a USS module (Claude Code, VS Code and Open VSX, JetBrains,
GitHub Actions, Homebrew, winget, OCI): no module now. The core Verify of
USS-DST-017 and USS-DST-019 points the auditor to each host's documentation
and validator, and the Q2 tables give the fields as read today. A module is
worth writing when a trial shows auditors stumble. GitHub Actions and VS Code
extensions are the likeliest first candidates.

#### Verify notes for auditors

- Evidence for USS-DST-017 and USS-DST-019 is the validator command, its
  version and its output, as AGENTS.md asks for a search.
- Validators by host: `cargo publish --dry-run`; Clippy
  `cargo_common_metadata`; a dry-run publish and publint for npm;
  `twine check --strict`; `helm lint --strict`; `claude plugin validate` and
  `--strict`; `vsce package`; `brew audit --strict --new`; the Plugin
  Verifier; the Central Portal's validation.

### Gaps

- I did not run `uss lint` on the drafts: the task allows no change to the
  standard files. The drafts need a lint run, a keyword scan and a check of
  each reference before they go in.
- USS-RUST-014, USS-TS-020 and the PY-017 body each hold more than one
  `SHOULD` or `MUST` clause. The keyword scan accepts that, but the maintainer
  may want one clause per body.
- The Central Portal's "validate without publishing" flow, which
  USS-MVN-010 would use, was not read.

## Q7. Does this repository meet the proposed rules?

### Takeaway

Mostly yes. The Claude Code plugin and marketplace pass `claude plugin
validate`, `rust-version` matches the CI toolchain, and `publish = false` keeps
the crate off crates.io. The open points are `engines.claude-code` under
USS-DST-018 (it would fail, or need a Standard Deviation), the repository
description and topics (not checked), and a decision on the citation MAY.

### Cited Findings

- The record answers `distributed-as-package: true`, `publishes-releases:
  true`, `ships-container-image: false`, `is-developer-tool: true`, and
  adopts USS-DST-002 and USS-DST-008. —
  [.uss/record.yaml](../../../../../.uss/record.yaml)
- `Cargo.toml` sets `rust-version = "1.100"`, `license = "MIT"`,
  `repository` and `publish = false` in `[workspace.package]`; it has no
  `description`, `readme`, `keywords` or `categories`. `tools/uss/Cargo.toml`
  inherits them. — [Cargo.toml](../../../../../Cargo.toml);
  [tools/uss/Cargo.toml](../../../../../tools/uss/Cargo.toml)
- `rust-toolchain.toml` pins `channel = "1.100.0"`, and CI runs `rustup
  toolchain install` (`.github/workflows/ci.yml` line 25). —
  [rust-toolchain.toml](../../../../../rust-toolchain.toml);
  [.github/workflows/ci.yml](../../../../../.github/workflows/ci.yml)
- `.claude-plugin/plugin.json` sets `name`, `version`, `description`,
  `author`, `homepage`, `repository`, `license` (MIT) and `keywords`, and no
  `engines`. `.claude-plugin/marketplace.json` sets `name`, `owner.name`,
  `metadata.description` and one entry with `name`, `source` and
  `description`. —
  [.claude-plugin/plugin.json](../../../../../.claude-plugin/plugin.json);
  [.claude-plugin/marketplace.json](../../../../../.claude-plugin/marketplace.json)
- Commands run on 2026-11-25 with Claude Code 2.1.330 from the repository
  root:
  - `claude plugin validate .` printed one warning, `.claude-plugin/plugin.json
    icon: No icon (ICON_MISSING)`, then `Validation passed with warnings`.
  - `claude plugin validate --strict .` printed the same warning, then
    `Validation failed (--strict treats warnings as errors)`.
  - `claude plugin validate .claude-plugin/marketplace.json` printed
    `Validation passed`.
- The docs say the directory-listing `icon` is read by Anthropic's directory
  and ignored by Claude Code. —
  [Claude Code Docs, Plugin manifest reference](https://code.claude.com/docs/en/plugins-reference)
- `metadata.description` is a documented alternate location for the
  marketplace description. —
  [Claude Code Docs, Marketplace reference](https://code.claude.com/docs/en/plugins/marketplace-reference.md)
- The repository has no `CITATION.cff`, `codemeta.json`, `publiccode.yml` or
  `.github/FUNDING.yml` (`ls` on 2026-11-25).

### Inferences

Per proposed rule:

- USS-DST-017 (MUST): pass. The hosts are GitHub Releases (binaries), Claude
  Code (plugin and marketplace) and `cargo install --git` (Cargo reads
  `name` and `version`). The validators report no error. crates.io is not a
  host because of `publish = false`.
- USS-DST-018 (SHOULD): pass for Cargo (1.100 declared and tested). Fail for
  the plugin: `plugin.json` has a host-version field (`engines.claude-code`),
  and it is not set. Setting it locks out Claude Code v2.0.15 to v2.1.75, and
  CI does not run Claude Code at any version. Choices: set it and add a CI
  job that runs `claude plugin validate` on the declared lowest version, or
  record a Standard Deviation. The reason would be that older hosts reject
  the field and that no CI job can install a pinned host version cheaply. I
  have not confirmed the second point.
- USS-DST-019 (SHOULD): pass under the drafted Verify, because the only strict
  warning is the Anthropic-directory icon and the project does not publish to
  that directory. Under a plain "strict mode passes" Verify it would fail,
  which is why the draft excludes such warnings.
- USS-DST-020: does not apply (`ships-container-image: false`).
- USS-DST-021 (SHOULD): unverified. I did not query the repository host,
  because the task forbids requests that name the repository. An audit reads
  it with the host's API or web page.
- USS-DST-022 (MAY): needs a decision in `decisions`. There is no
  `CITATION.cff` today.
- RUST-012 and RUST-014 do not apply in practice (nothing goes to crates.io).
  If the project later publishes `uss` to crates.io, it needs `description`,
  plus `keywords`, `categories` and a README for the SHOULD.
- USS-RUST-013: pass. The crate is built with `cargo install --git`, and its
  `rust-version` equals the pinned CI toolchain.
- The fact change makes no difference here: the record already answers
  `distributed-as-package: true`.

### Gaps

- The repository-host description and topics were not checked (constraint on
  outside requests that name the repository).
- I did not check whether CI could run a pinned old Claude Code version for
  `engines.claude-code`.
