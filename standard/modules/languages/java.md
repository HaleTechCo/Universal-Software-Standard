# Java

- **Prefix:** JAVA
- **Kind:** module
- **Applies when:** lang.java

This module covers Java projects: the compiler's warnings, the checks that read
the code for bug patterns, nullness and security flaws, the formatter, the
pinned JDK and the language's two classic hazards, deserialization and XML
parsers. Build, dependency and plugin rules are in the Maven module
(`standard/modules/tools/maven.md`). The module uses three vendor profiles:
`standard/vendors/google/error-prone.md`, `standard/vendors/uber/nullaway.md`
and `standard/vendors/spotbugs/spotbugs.md`. It names google-java-format
without a profile, because a formatter has no rule set. USS-JAVA-001 is the
Java safety check that USS-SAF-001 makes binding. In this module, a release is
one exact JDK version (such as `25.0.1`), and a feature release is its Java
number (such as `25`). The core rules that this module does not refine apply as
written.

## Checks

### USS-JAVA-001: Compile with all warnings as errors

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-003, USS-SAF-001
- **Verify:** Each `javac` run of the build, test compilation included, has the
  options `-Xlint:all` and `-Werror` (in Maven, `compilerArgs` of
  `maven-compiler-plugin`; another build tool passes the same options), and
  no option switches the categories off again (such as a later
  `-Xlint:none`). Each category that the options leave out with a minus sign
  (such as `-Xlint:all,-serial` or a later `-Xlint:-serial`) has a reason in a
  comment next to it, and a left-out category with no reason fails. Under
  `-Werror`, `javac` terminates the compilation when a warning occurs, so the
  compile step fails. A build with these options reports no warning on the
  default branch.
- **References:** The `javac` documentation, JDK 27 (read 2026-11-24):
  `-Werror`, `-Xlint`; The `javac` documentation, JDK 25 (read 2026-11-24): the
  `-Xlint` keys

The `javac` options MUST include `-Xlint:all` and `-Werror`, and each lint
category that the options leave out MUST have a reason.

**Rationale:** `-Xlint:all` turns on every warning that `javac` can give, and
`-Werror` makes a warning end the compilation, so a new warning cannot hide
among old ones. A category that is left out names the defects that `javac` no
longer reports, so its reason is written next to it.

### USS-JAVA-002: Check for bug patterns when compiling

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001
- **Verify:** CI runs a bug-pattern checker on all of the project's Java code,
  test code included (such as Error Prone, which runs inside `javac`, or
  SpotBugs, which reads the compiled classes), and a finding fails the build.
  The job fails when the checker or the build tool exits with a non-zero
  status. A finding that is only a warning fails the build only when the
  compiler turns warnings into errors (USS-JAVA-001) or when the checker's
  configuration raises its severity to error. A run on the default branch
  reports no finding.
- **References:** Error Prone documentation, Installation (read 2026-11-24);
  SpotBugs Maven Plugin 4.10.5.0, `check` goal (read 2026-11-24)

CI MUST run a bug-pattern checker on all Java code, and a finding MUST fail the
build.

**Rationale:** The compiler accepts code that is wrong in ways that a checker
for known bug patterns can see, such as a comparison that is always false or a
result that nobody reads. A finding that does not fail the build is easy to
ignore.

### USS-JAVA-003: Check with Error Prone

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-001
- **Verify:** The build runs Error Prone as a `javac` plugin
  (`-Xplugin:ErrorProne` in the compiler arguments, with `error_prone_core` on
  the annotation processor path), CI builds with it, and the build fails on a
  finding (an Error Prone check at ERROR is a compile error). The compiler
  arguments include the baseline of the Error Prone profile
  (`standard/vendors/google/error-prone.md`). A project that uses another tool
  fails this rule (it records a Standard Deviation) and meets the outcome rule
  through that tool, configured to at least the baseline that USS-QLT-006
  sets.
- **References:** Error Prone documentation, Installation and Flags (read
  2026-11-24); Error Prone 2.53.0, Bug Patterns (read 2026-11-24)

The build SHOULD run Error Prone, configured per its vendor profile
(`standard/vendors/google/error-prone.md`).

**Rationale:** Error Prone runs inside the compiler, so each finding appears
with the compile error that the author is already reading, and its checks use
the compiler's types, which a check on the bytecode cannot always see.

### USS-JAVA-004: Analyse with SpotBugs and Find Security Bugs

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-QLT-001, USS-SEC-028
- **Verify:** CI runs SpotBugs with the Find Security Bugs plugin on the
  project's compiled classes, and the job fails on a finding (the `check` goal
  of the Maven plugin ends the build with an error when SpotBugs reports a
  bug). The configuration includes the baseline of the SpotBugs profile
  (`standard/vendors/spotbugs/spotbugs.md`); another build tool sets the same
  effort, threshold, plugin and failure options under its own names. A project
  that uses another tool fails this rule (it records a Standard Deviation) and
  meets the outcome rule through that tool, configured to at least the
  baseline that USS-QLT-006 sets.
- **References:** SpotBugs Maven Plugin 4.10.5.0, `spotbugs` and `check` goals
  (read 2026-11-24); Find Security Bugs 1.14.0, Bug Patterns (read 2026-11-24)

The project SHOULD analyse its code with SpotBugs and the Find Security Bugs
plugin in CI, configured per their vendor profile
(`standard/vendors/spotbugs/spotbugs.md`).

**Rationale:** SpotBugs reads the bytecode and finds bug patterns that the
compiler does not report, and Find Security Bugs adds detectors for injection,
unsafe deserialization, weak cryptography and unsafe XML parsing, the flaws
that a static security analysis exists to find.

### USS-JAVA-005: Check nullness with NullAway

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Safety
- **Verify:** The project's Java code marks nullness with JSpecify annotations
  (`org.jspecify.annotations`): `@NullMarked` on each module, package or class
  of the project's own code, and `@Nullable` where a value can be null. The
  build runs NullAway as an Error Prone check, CI builds with it, and the build
  fails on a finding (NullAway at ERROR is a compile error). The compiler
  arguments include the baseline of the NullAway profile
  (`standard/vendors/uber/nullaway.md`). A project that uses another tool
  fails this rule (it records a Standard Deviation) and meets the outcome rule
  through that tool, configured to at least the baseline that USS-QLT-006
  sets.
- **References:** JSpecify 1.0.1, User Guide (read 2026-11-24); NullAway 0.14.4,
  Configuration and JSpecify Support (read 2026-11-24)

The project's code SHOULD mark nullness with JSpecify annotations, and the
build SHOULD check it with NullAway, configured per its vendor profile
(`standard/vendors/uber/nullaway.md`).

**Rationale:** A null value that reaches a dereference is one of the most
common causes of a crash in Java. JSpecify gives the annotations one meaning
across tools, and NullAway checks each use against them at compile time.

### USS-JAVA-006: Format with google-java-format

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Refines:** USS-QLT-002
- **Verify:** CI runs google-java-format over all of the project's Java files
  in a mode that fails when it would change a file: the command line with
  `--dry-run` and `--set-exit-if-changed` (which exits with status 1 when
  there are formatting changes; `--dry-run` alone prints the paths and does
  not fail), or a build plugin that runs google-java-format and fails on a
  difference. A project that uses another tool fails this rule (it records a
  Standard Deviation) and meets the outcome rule through that tool, configured
  to at least the baseline that USS-QLT-006 sets.
- **References:** google-java-format 2.0.0, README and usage text (read
  2026-11-24)

The project SHOULD format with google-java-format, and CI SHOULD fail on a file
that it would change.

**Rationale:** google-java-format fixes one format for the code, so a format
check ends style arguments in review and keeps each diff to the change itself.

## Toolchain

### USS-JAVA-007: Pin the JDK

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-001
- **Verify:** A file in the repository (such as a tool-version file, a dev
  container definition or a pinned base image) names one JDK distribution (a
  vendor's build of the JDK, such as Eclipse Temurin) and one exact release of
  it, written as that release's version number (JEP 223: the version number
  has no trailing zero element, so the first update of release 25 is `25.0.1`).
  Local builds and CI both read that file or name the same distribution and
  release. A pin that moves to newer updates (a range, `latest`, `lts`, or a
  name that the tool resolves to the newest update) fails, and so does a
  build that uses the JDK that happens to be installed.
- **References:** JEP 223, New Version-String Scheme (read 2026-11-24)

The repository MUST pin the exact JDK distribution and release for development
and CI.

**Rationale:** JDK builds from different vendors, and updates of one vendor,
differ in security fixes and in behaviour. A build that takes the JDK that is
installed gives different results on different machines and changes with no
change to the code.

### USS-JAVA-008: No end-of-life Java releases

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-006
- **Verify:** Take the feature release of each JDK that the project builds and
  tests with: the pin of USS-JAVA-007 (such as 25 for `25.0.1`) and each JDK in
  the CI test matrix. Take the support roadmap that the vendor of each of those
  JDKs publishes (such as the Eclipse Temurin support roadmap for a Temurin
  JDK). At the audit date, the roadmap shows that each of those feature
  releases still receives security updates. A feature release whose end date
  has passed fails. The release that the code targets (`--release`, or
  `maven.compiler.release` in Maven; `source` and `target` where it uses
  those) is not judged by this rule. Where neither the roadmap nor the vendor's
  release history shows it, the auditor records `unverified`.
- **References:** Eclipse Temurin, Support (Adoptium, read 2026-11-24)

The project MUST NOT build or test with a JDK whose feature release its vendor
no longer supports with security fixes.

**Rationale:** A flaw that is found in a release that the vendor no longer
updates is not fixed, so every build and every program on that release keeps
it. The vendor's own roadmap says which releases it still updates, and the
answer differs between vendors. A declared minimum such as the `--release`
target is a compatibility promise (USS-DEP-016), not a platform that the
project runs on.

## Language hazards

### USS-JAVA-009: No unfiltered deserialization

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SEC-011
- **Verify:** Search the project's own code for each `ObjectInputStream` (and
  subclass) whose input can come from outside the trust boundary. Each one has
  a deserialization filter that allows only the expected classes: set on the
  stream with `setObjectInputFilter`, or applied to it by a JVM-wide filter
  or filter factory (the `jdk.serialFilter` property, `Config.setSerialFilter`
  or `Config.setSerialFilterFactory`). The filter rejects every class that it
  does not allow: a pattern filter ends with `!*`, and a custom filter returns
  a rejection for a class that is not on its list, because a class that the
  filter leaves undecided is accepted. A stream with no such filter, or with a
  filter that accepts the classes that it does not list, fails.
- **References:** JEP 290, Filter Incoming Serialization Data (read
  2026-11-24); Java SE 25 Core Libraries, Serialization Filtering (read
  2026-11-24); Java SE 25 API, `java.io.ObjectInputFilter` (read 2026-11-24);
  CWE-502

Untrusted data MUST NOT reach an `ObjectInputStream` without a deserialization
filter that allows only the expected classes.

**Rationale:** Deserialization creates the classes that the stream names, so a
stream that an attacker writes can choose which classes the program builds and
run their code. A filter that allows only the expected classes stops the
others before they are created.

### USS-JAVA-010: Turn off external entities in XML parsers

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SEC-009
- **Verify:** Search the project's own code for each XML parser or processor
  that reads input from outside the trust boundary. Each one has the settings
  that the OWASP XML External Entity Prevention Cheat Sheet gives for its API,
  which turn off DTDs and external entities. For `DocumentBuilderFactory` and
  `SAXParserFactory` (and the `XMLReader` that it makes), the feature
  `http://apache.org/xml/features/disallow-doctype-decl` is `true`. For
  `XMLInputFactory`, `SUPPORT_DTD` and `IS_SUPPORTING_EXTERNAL_ENTITIES` are
  `false`. For `TransformerFactory`, `ACCESS_EXTERNAL_DTD` and
  `ACCESS_EXTERNAL_STYLESHEET` are `""`. For `SchemaFactory`,
  `ACCESS_EXTERNAL_DTD` and `ACCESS_EXTERNAL_SCHEMA` are `""`. The settings
  are made before the parser or processor is created. A parser with none of
  these settings fails.
- **References:** OWASP XML External Entity Prevention Cheat Sheet, Java
  (read 2026-11-24); CWE-611

Each XML parser that reads untrusted input MUST turn off DTDs and external
entities.

**Rationale:** An XML document can declare an entity that the parser replaces
with the content of a local file or of a URL, so a document from an attacker
can read files or reach internal hosts. The settings above stop the parser
from reading a DTD or resolving an external entity at all.
