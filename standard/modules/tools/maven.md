# Maven

- **Prefix:** MVN
- **Kind:** module
- **Applies when:** tool.maven

This module covers projects that build with Apache Maven: the pinned Maven, the
plugins and the repositories, and how the project declares, locks and checks
its dependencies. Maven is the module's subject, so the module names plugin
goals and configuration elements directly; the compiler and the language are
in the Java module (`standard/modules/languages/java.md`). It uses one vendor
profile: `standard/vendors/google/osv-scanner.md`. It names four tools without
a profile, because none has a rule set: the Maven Wrapper, the Maven Enforcer
plugin, the Maven Dependency plugin and the maven-lockfile plugin. The rules
were checked against Maven 3.10.2, the current release; Maven 4 is still a
release candidate. The core rules that this module does not refine apply as
written.

## Build

### USS-MVN-001: Pin Maven with the wrapper

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-001
- **Verify:** The wrapper scripts (`mvnw` and `mvnw.cmd`) and
  `.mvn/wrapper/maven-wrapper.properties` are committed. The `distributionUrl`
  property names the Maven distribution of one exact release (such as
  `.../apache-maven/3.10.2/apache-maven-3.10.2-bin.zip`), and the
  `distributionSha256Sum` property holds the SHA-256 checksum of that
  distribution file in hexadecimal. A `distributionUrl` that names no exact
  release, or a missing `distributionSha256Sum`, fails. Each CI step that runs
  Maven calls the wrapper (`./mvnw` or `mvnw.cmd`) and not a `mvn` that the CI
  image provides. The `mvnw` script of the default `only-script` type checks the
  checksum and exits with status 1 on a mismatch.
- **References:** Apache Maven Wrapper 3.3.4, documentation (read 2026-11-24);
  Apache Maven Wrapper 3.3.4, `only-mvnw` script (read 2026-11-24)

The project MUST build through the Maven Wrapper, which names an exact Maven
version and the checksum of the distribution.

**Rationale:** A build that calls whichever `mvn` is installed gives different
results on different machines. The wrapper downloads the pinned Maven, and the
checksum stops a changed or replaced distribution from running.

### USS-MVN-002: Pin every plugin version

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-001
- **Verify:** The build runs the `enforce` goal of `maven-enforcer-plugin` (in
  the POM or in a parent POM, under `plugins` or `pluginManagement`) with the
  rule `requirePluginVersions`. The rule has no entry in `unCheckedPluginList`,
  `banLatest` and `banRelease` are `true` (their default) and the rule's level
  is not `WARN` (its default is `ERROR`). The execution does not set `fail` to
  `false` (its default is `true`), and CI does not set `enforcer.skip`. The
  build passes on the default branch, which shows that each plugin that the
  rule checks has an explicit version.
- **References:** Apache Maven Enforcer 3.7.0, `requirePluginVersions` (read
  2026-11-24); Apache Maven Enforcer 3.7.0, `enforce` goal (read 2026-11-24);
  Apache Maven Enforcer 3.7.0, `AbstractEnforcerRule` source (read 2026-11-24)

Each plugin MUST have an explicit version, enforced with the Enforcer rule
`requirePluginVersions`.

**Rationale:** A plugin without a version takes the version that Maven or the
latest release supplies, so the build changes with no change to the project.
The rule fails the build on a plugin with no version, so a new plugin cannot
slip in unpinned.

### USS-MVN-003: Resolve only over HTTPS

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-005
- **Verify:** Each repository and each plugin repository that the build
  resolves from uses a `https` URL: those declared in the POMs
  (`repositories` and `pluginRepositories`), those in a settings file that the
  build uses, and the mirror that replaces one of them (its URL is then the
  one that counts). A URL with `http` fails. Checksum checks fail the build:
  the Maven command (in CI or in `.mvn/maven.config`) has `-C`
  (`--strict-checksums`, "Fail the build if checksums don't match"), or each
  repository that the build uses, Maven Central included, has
  `<checksumPolicy>fail</checksumPolicy>` in its `releases` and `snapshots`
  blocks. `-c` (`--lax-checksums`) and a policy of `warn` or `ignore` fail; the
  `checksumPolicy` of a repository is `warn` when it sets none.
- **References:** Apache Maven 3.10.2, POM Reference, Repositories (read
  2026-11-24); Apache Maven 3.10.2, Command-Line Options (read 2026-11-24);
  Apache Maven, Configuring Maven, `.mvn/maven.config` (read 2026-11-24);
  Apache Maven 3.8.1, Release Notes (read 2026-11-24)

Each repository and plugin repository MUST use HTTPS, and checksum checks MUST
fail the build.

**Rationale:** An artifact over plain HTTP can be replaced on the way, and a
checksum that only warns lets a changed artifact into the build. Maven 3.8.1
and later block external HTTP repositories by default for this reason, so the
rule keeps a project from switching that off.

## Dependencies

### USS-MVN-004: Pin release versions

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-001
- **Verify:** In each POM of the project, each dependency (the `dependencies`
  and `dependencyManagement` tables, a BOM that the project imports with scope
  `import`, and the dependencies in profiles) has an effective version that
  names one release: a version written directly, set by a property, or taken
  from `dependencyManagement` or an imported BOM. A soft requirement (`1.2.3`)
  and a hard requirement (`[1.2.3]`) pass. A range (`[1.0,2.0)`, `(,1.0]` or
  `[1.5,)`), `LATEST` and `RELEASE` fail. A version that ends in `-SNAPSHOT`
  fails in a released build, which is a build whose project version has no
  `-SNAPSHOT`, such as the build of a release tag.
- **References:** Apache Maven 3.10.2, POM Reference, Dependency Version
  Requirement Specification (read 2026-11-24); Apache Maven Enforcer 3.7.0,
  `requireReleaseDeps` (read 2026-11-24)

Each dependency MUST have a release version, directly or through
`dependencyManagement` or a BOM, and never `LATEST`, `RELEASE` or a
`-SNAPSHOT` version in a released build.

**Rationale:** A range, `LATEST` and `RELEASE` let the build pick a version
that nobody has tested, and a snapshot changes under the same version number.
A release version names one set of files, so the same POM builds the same
dependencies.

### USS-MVN-005: Fail on dependency version conflicts

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-001
- **Verify:** The build runs the `enforce` goal of `maven-enforcer-plugin` (in
  the POM or in a parent POM) with the rule `dependencyConvergence` or the rule
  `requireUpperBoundDeps`. The rule has none of `includes`, `excludes` or an
  `excludedScopes` list wider than its default (`test` and `provided`), and its
  level is not `WARN` (its default is `ERROR`). The execution does not set
  `fail` to `false` (its default is `true`), and CI does not set
  `enforcer.skip`. The build passes on the default branch.
  `dependencyConvergence` fails the build when two paths of the dependency tree
  need different versions of an artifact; `requireUpperBoundDeps` fails it when
  the resolved version is lower than the highest version that the tree
  requests.
- **References:** Apache Maven Enforcer 3.7.0, `dependencyConvergence` (read
  2026-11-24); Apache Maven Enforcer 3.7.0, `requireUpperBoundDeps` (read
  2026-11-24); Apache Maven Enforcer 3.7.0, `enforce` goal (read 2026-11-24)

The build MUST run the Enforcer rule `dependencyConvergence` or
`requireUpperBoundDeps`, so that conflict mediation never picks a version
silently.

**Rationale:** When two dependencies need different versions of a third,
Maven picks the one nearest to the project and says nothing, so the project
can run on a version that one dependency was never tested with. The rule turns
that choice into a build failure, and the project then pins the version it
wants in `dependencyManagement`.

### USS-MVN-006: Lock and verify every resolved artifact

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-002, USS-DEP-003
- **Verify:** A lock file of the maven-lockfile plugin (`lockfile.json`, or the
  name that `lockfileName` sets; one per module) is committed. It records a
  checksum for each artifact of the dependency tree, transitive ones
  included, and an entry with an empty `checksum` fails. The plugin's
  `includeMavenPlugins` is not `false` (its default is `true`), so the plugins
  are recorded too. A CI step runs the `validate` goal
  (`io.github.chains-project:maven-lockfile:validate`), which ends the build
  with an error when a dependency differs from the lock file, and the job
  fails when Maven exits with a non-zero status (`--fail-never` hides a
  failure, so the job does not use it). None of `allowValidationFailure`,
  `allowPomValidationFailure`, `allowMavenPluginValidationFailure`,
  `allowBomValidationFailure`, `allowParentPomValidationFailure` and
  `allowMavenExtensionsValidationFailure` is `true`, and `skip` is not `true`.
  A lock file generated with `reduced` set holds the artifacts that remain after
  conflict resolution and passes. Another tool that records the checksum of
  each resolved artifact and fails the build on a difference meets the rule.
- **References:** maven-lockfile 5.19.0, README (read 2026-11-24);
  maven-lockfile 5.19.0, `ValidateMojo` source (read 2026-11-24); Apache Maven
  3.10.2, Command-Line Options (read 2026-11-24)

A committed lock file MUST record the checksum of every resolved artifact,
transitive ones included, and CI MUST fail when the resolved set differs from
it.

**Rationale:** A POM names versions, not files, so two builds of the same
version can fetch different bytes if a repository or a mirror serves them. The
lock file fixes the checksum of each artifact, and the check turns a difference
into a failed build before the code runs.

### USS-MVN-007: Check undeclared and unused dependencies

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-004, USS-DEP-013
- **Verify:** CI runs the goal `dependency:analyze` (or
  `dependency:analyze-only`, which Maven attaches to the `verify` phase) of
  `maven-dependency-plugin` with `failOnWarning` set to `true` (in the POM, or
  with `-DfailOnWarning=true`),
  and the job fails when Maven exits with a non-zero status (`--fail-never`
  hides a failure, so the job does not use it). The goal fails the build on a
  dependency that the code uses and no POM declares and on one that a POM
  declares and the code does not use. `skip` is not `true`.
- **References:** Apache Maven Dependency Plugin 3.11.0, `analyze` goal (read
  2026-11-24); Apache Maven Dependency Plugin 3.11.0, `analyze-only` goal (read
  2026-11-24); Apache Maven 3.10.2, Command-Line Options (read 2026-11-24)

CI MUST run `dependency:analyze` with `failOnWarning` on.

**Rationale:** The analysis reads the compiled classes. It reports a
dependency that the code uses and no POM declares, which only works while
another dependency happens to bring it in, and a declared dependency that
nothing uses, which adds code to build, scan and update. It reads bytecode, so
a dependency that the program loads by reflection can show as unused.

### USS-MVN-008: Scope non-runtime dependencies

- **Level:** MUST
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-005
- **Verify:** In each POM (the `dependencies` and `dependencyManagement` tables
  and the dependencies in profiles), each dependency that only the test code
  uses has `<scope>test</scope>`, and each dependency that the JDK or the
  runtime container provides when the program runs has
  `<scope>provided</scope>`. A scope can come from `dependencyManagement`. A
  dependency of the first kind with `compile` scope (the default), or of the
  second kind with `compile` or `runtime` scope, fails. As optional evidence
  that cannot fail the rule, the `dependency:analyze` report lists the
  dependencies that only tests use and that have no test scope.
- **References:** Apache Maven 3.10.2, POM Reference, Dependencies (read
  2026-11-24); Apache Maven Dependency Plugin 3.11.0, `analyze` goal (read
  2026-11-24)

Test-only dependencies MUST have `test` scope, and dependencies that the runtime
provides MUST have `provided` scope.

**Rationale:** A dependency with `compile` scope is packaged with the program
and passed on to the projects that depend on it. Test libraries and container
APIs in the shipped artifact are extra code to attack and to keep updated.

### USS-MVN-009: Scan with OSV-Scanner

- **Level:** SHOULD
- **Applies when:** has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-007
- **Verify:** CI runs OSV-Scanner over the project's `pom.xml` files with
  transitive dependency resolution on (OSV-Scanner resolves them by default;
  `--no-resolve` and offline mode turn it off, and a scan that sets either
  fails), with a configuration that includes the baseline of the OSV-Scanner
  profile (`standard/vendors/google/osv-scanner.md`), and the job fails on a
  finding. A project on another scanner fails this rule (it records a Standard
  Deviation) and meets USS-DEP-007 through that tool's own profile or
  settings.
- **References:** OSV-Scanner Docs, Supported Languages and Lockfiles (read
  2026-11-24)

The project SHOULD scan its resolved dependencies with OSV-Scanner, configured
per its vendor profile (`standard/vendors/google/osv-scanner.md`).

**Rationale:** OSV-Scanner resolves the transitive dependencies of a
`pom.xml` and reads the same vulnerability data for Python, TypeScript and
Maven projects. It does not yet include test dependencies in the dependency
graph that it computes for a `pom.xml`.
