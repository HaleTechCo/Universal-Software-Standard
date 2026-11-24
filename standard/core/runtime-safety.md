# Language and runtime safety

- **Prefix:** SAF
- **Kind:** area

This area covers the safety options of a language's toolchain, and the extra
care that code without memory safety needs. Its rules count toward Safety by
default. The language modules name the exact flags and tools, so these rules
stay language-neutral. Fuzzing is in the Testing area. The check-integrity
rules for the checks that this area turns on are in the Code quality (QLT)
area.

### USS-SAF-001: Turn on the toolchain's safety checks

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** For each language of the project's own code, take the list of
  safety checks, strict modes and hardening options that its USS module
  names. For a language with no module, take the options that the
  toolchain's documentation describes as safety or strict options (such as
  integer-overflow checks in release builds, a type checker's strict mode,
  or the warnings that the documentation marks as safety-relevant). Each one
  is on in the project's build configuration and in the CI build.
- **References:** CWE-190

Every safety check, strict mode and hardening option that the language's
module names MUST be on. For a language with no module, the same holds for
every option that the toolchain documents as a safety or strict option.

**Rationale:** Toolchains ship with some safety options off, to keep old code
building or to keep the build fast. A project that turns them on finds the
defects at build time, not in production.

### USS-SAF-002: Harden native binaries

- **Level:** MUST
- **Applies when:** has-memory-unsafe-code
- **Category:** Safety
- **Verify:** The build configuration of each native binary (Makefile, CMake,
  Bazel, Cargo or other build files) passes the compiler and linker options
  that the OpenSSF Compiler Options Hardening Guide for C and C++ recommends,
  or those that the USS module of the language names. Where the toolchain is
  not covered by the guide, the module's list applies. A check of a built
  binary (such as a hardening checker) confirms the options took effect.
- **References:** OpenSSF Compiler Options Hardening Guide for C and C++ (read
  2026-11-24); CWE-119; CWE-787

Native code MUST be built with the options that the OpenSSF Compiler Options
Hardening Guide for C and C++ recommends, or that the language module names.

**Rationale:** Hardening options make a memory error much harder to turn into
an exploit, at a small cost in speed.

### USS-SAF-003: Justify each use of unsafe code

- **Level:** MUST
- **Applies when:** has-memory-unsafe-code
- **Category:** Safety
- **Verify:** In a memory-safe language, search the project's own code for
  each use of an unsafe feature (such as a Rust `unsafe` block, Go's `unsafe`
  package, C# `unsafe` code). Each one has a comment on or directly above it
  that says why it is sound: the condition that makes the operation safe and
  why that condition holds. Each one also wraps only the operation that needs
  it, and no unrelated safe statements. Code in C, C++ and assembly is not
  judged by this rule.
- **References:** CWE-119; CWE-787; CWE-416

In a memory-safe language, each use of an unsafe feature MUST be as small as
possible and have a comment that says why it is sound.

**Rationale:** The compiler cannot check unsafe code, so the reviewer has to.
A small block with a stated reason is a short thing to review, and a
wrong reason is visible.

### USS-SAF-004: Run sanitizers in CI

- **Level:** MUST
- **Applies when:** has-memory-unsafe-code
- **Category:** Safety
- **Verify:** The CI configuration runs the tests with a memory-error
  sanitizer and an undefined-behaviour sanitizer that the toolchain offers
  (such as AddressSanitizer and UndefinedBehaviorSanitizer for C and C++, or
  Miri for Rust), on each pull request or on a schedule of at most a week,
  and a finding fails the job. Where the toolchain offers none, the job file
  says so and the auditor accepts that.
- **References:** Clang AddressSanitizer documentation (read 2026-11-24);
  Miri README (read 2026-11-24); CWE-119; CWE-787; CWE-416

CI MUST run the tests with the memory-error and undefined-behaviour
sanitizers that the toolchain offers (such as AddressSanitizer and
UndefinedBehaviorSanitizer, or Miri for Rust).

**Rationale:** A memory error often does not fail a test: it corrupts memory
and the program goes on. A sanitizer turns it into a failure at the line
where it happens.

### USS-SAF-005: Publish a memory-safety roadmap

- **Level:** SHOULD
- **Applies when:** has-memory-unsafe-code
- **Category:** Safety
- **Verify:** A published document (in the repository, the documentation or
  the project's website) names each memory-unsafe component of the project,
  and for each one the plan and the dates to move it to a memory-safe
  language or to isolate it. A roadmap that says only "we will adopt Rust" or
  that has no dates fails. A project that sets an end-of-support date for
  the component records that date and passes.
- **References:** CISA and partners, The Case for Memory Safe Roadmaps
  (2023-12-06); CISA and FBI, Product Security Bad Practices (Version 2.0,
  2025-01-17); CWE-119; CWE-787; CWE-416

A published roadmap SHOULD name the memory-unsafe components, and the plan and
dates to move them to a memory-safe language or to isolate them.

**Rationale:** Memory-safety flaws are a large share of serious
vulnerabilities, and moving code takes years. A dated plan lets users and
reviewers see that the risk is being reduced.

### USS-SAF-006: Write new components in a memory-safe language

- **Level:** SHOULD
- **Applies when:** has-memory-unsafe-code
- **Category:** Safety
- **Verify:** List the components added since the last audit, or the last 12
  months when there is none (a new directory, package, crate or service with its
  own build). Each one is written in a memory-safe language. A component that is
  not has a written reason (such as a platform interface that has no memory-safe
  binding) in the repository.
- **References:** CISA and FBI, Product Security Bad Practices (Version 2.0,
  2025-01-17); CWE-119; CWE-787; CWE-416

Each new component SHOULD be written in a memory-safe language.

**Rationale:** New code in a memory-unsafe language adds to the roadmap that
the project has to carry. New code in a memory-safe language does not.

### USS-SAF-007: Run a race detector in CI

- **Level:** SHOULD
- **Applies when:** uses-concurrency
- **Category:** Safety
- **Verify:** Where the toolchain offers a data-race detector (such as Go's
  `-race` or ThreadSanitizer), the CI configuration runs the tests of the
  concurrent code with it, on each pull request or on a schedule of at most a
  week, and a finding fails the job. Where the toolchain offers none, the job
  file says so and the auditor accepts that.
- **References:** Clang ThreadSanitizer documentation (read 2026-11-24);
  CWE-362

CI SHOULD run the tests of concurrent code with a data-race detector where the
toolchain offers one (such as Go's `-race` or ThreadSanitizer).

**Rationale:** A data race shows up only under some timings, so a test can
pass a hundred times and fail in production. A detector finds the race that
did not happen to hurt.
