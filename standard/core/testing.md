# Testing

- **Prefix:** TST
- **Kind:** area

This area covers the automated tests of the software: that they run, what kinds
of test the project has and how well the tests find faults. Its rules count
toward Reliability by default. The Code quality (QLT) area keeps the tests
honest, with its rules on skipped tests and coverage exclusions, the Language
and runtime safety (SAF) area holds sanitizer and race-detector runs, and the
Accessibility (ACC) area holds accessibility testing.

## Tests in CI

### USS-TST-001: Run the tests in CI

- **Level:** MUST
- **Applies when:** always
- **Category:** Reliability
- **Verify:** The CI configuration has a job that runs the whole automated test
  suite (each kind of test that the repository holds) on each pull request and
  each push to the default branch. A failing test fails the job, and the
  branch protection or merge rules require the job, where the host allows
  that. Without access to those settings, the auditor records `unverified`
  for the second part. A recent run on the default branch passes, and a test
  suite that the CI does not run, or whose failures the job ignores, fails.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.8.2

CI MUST run the whole automated test suite on every change, and a failing test
MUST fail the build.

**Rationale:** A test that nobody runs protects nothing, and a failure that
does not stop the change is soon ignored.

## Kinds of test

### USS-TST-002: Unit-test the logic

- **Level:** MUST
- **Applies when:** always
- **Category:** Reliability
- **Verify:** List the modules (files, packages or classes) that hold logic: a
  branch, a loop, a calculation or a parse. Each one has automated tests that
  call it, and the tests cover its error cases (invalid input, a failed
  dependency), not only the normal case. A module with logic and no test
  fails. The project's coverage report, where it has one, is a guide to which
  modules have none.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.8.2

Each module that contains logic MUST have automated tests of its behaviour,
its error cases included.

**Rationale:** A unit test finds a fault where it is cheapest to fix, and it
lets a later change to the module show at once what it breaks.

### USS-TST-003: Add a regression test for each bug fix

- **Level:** MUST
- **Applies when:** always
- **Category:** Reliability
- **Verify:** Take each change in the history window that the project labels
  as a bug fix (by its commit message, its issue label or its type in the
  changelog): the changes since the last audit, or the last 12 months when
  there is none. Each one adds or changes a test in the same change. Run the
  test against the code before the fix: it fails, and with the fix it passes.
  A fix with no test, or whose test also passes without the fix, fails.
- **References:** none

Each bug fix MUST come with a test that fails without the fix.

**Rationale:** A bug that was fixed once and has no test returns with the next
change near it. The test keeps the fix.

### USS-TST-004: Test integrations against real services

- **Level:** SHOULD
- **Applies when:** uses-database or uses-network
- **Category:** Reliability
- **Verify:** List the code that talks to a database, a message broker or
  another service. Its tests run against a real instance of the service (such
  as a container that CI starts with Testcontainers or a service container of
  the CI host), in at least one test for each such integration. Tests that
  use only mocks, fakes or an in-memory substitute of a different kind of
  database fail. A service that cannot run in CI (a paid third-party API) may
  have a recorded contract test, and the documentation says so.
- **References:** Testcontainers (testcontainers.com, read 2026-11-24), as an
  example

Tests of code that talks to a database, a message broker or another service
SHOULD run against a real instance of it, such as a container in CI, and not
only against mocks.

**Rationale:** A mock does what its author thought the service does. A real
instance shows what the service does, including the differences in query
behaviour, transactions and errors that cause faults in production.

### USS-TST-005: Test user workflows end to end

- **Level:** MUST
- **Applies when:** has-ui or has-cli
- **Category:** Reliability
- **Verify:** List the workflows that the user documentation describes (each
  task that a guide, a tutorial or a getting-started page walks through). For
  each one, an automated end-to-end test drives the real interface (the
  browser or the application for a UI, the built command for a CLI) through
  the steps in the documentation and checks the result. A workflow that the
  documentation describes and no end-to-end test covers fails. A test that
  calls internal functions in place of the interface does not count.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.8.2

Each workflow that the user documentation describes MUST have an end-to-end
test that drives the real interface.

**Rationale:** Unit tests pass while the pieces still fail to work together.
The documented workflows are the ones that users try first.

### USS-TST-006: Test the API against its contract

- **Level:** MUST
- **Applies when:** exposes-network-api
- **Category:** Reliability
- **Verify:** CI runs tests (or a contract-test tool) against the running API,
  such as a test instance, that send requests and check each response against
  the contract (USS-API-001): the status, the fields and their types, and the
  error format. Each operation in the contract has at least one such test.
  The tests fail when the API and the contract differ. A test that checks
  only that the service answers fails.
- **References:** OpenAPI Specification 3.2.1

Tests MUST check that the running API's requests and responses match its
contract (USS-API-001).

**Rationale:** A contract only helps while it is true. A test that compares
the two finds the drift the day it happens, not when a client breaks.

### USS-TST-007: Test each data migration

- **Level:** MUST
- **Applies when:** stores-persistent-data
- **Category:** Reliability
- **Verify:** Take each migration (USS-DAT-001) in the history window: the
  changes since the last audit, or the last 12 months when there is none.
  Each one has a test that builds data in the format it migrates from (a
  fixture or a database dump of the previous version), runs the migration and
  checks the result: the structure is as expected, and the data is kept. A
  migration with no such test fails. A test that migrates an empty database
  does not count.
- **References:** none

Each migration (USS-DAT-001) MUST be tested against data in the format that it
migrates from.

**Rationale:** A migration that works on an empty database can still fail on
real data, and it fails on the day of the upgrade, when going back is hard.

### USS-TST-008: Test that access control denies

- **Level:** MUST
- **Applies when:** handles-authentication
- **Category:** Reliability
- **Verify:** List each protected operation (each route, command or function
  that USS-SEC-018 puts behind an authorization check). For each one, an
  automated test calls it as a caller without the permission (not signed in,
  or with a different role or another user's identity) and checks that it is
  refused. A protected operation with no such test fails. A test that checks
  only the permitted caller does not count.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.8.2; CWE-862

For each protected operation, a test MUST show that a caller without the
permission is refused.

**Rationale:** An authorization check that breaks does not show itself, as
every permitted use still works. Only a test of the refusal finds that the
door has opened.

## Test effectiveness

### USS-TST-009: Run mutation testing on security-relevant code

- **Level:** SHOULD
- **Applies when:** handles-authentication or high-assurance
- **Category:** Reliability
- **Verify:** The scope is the authentication and authorization code, the code
  that validates input or does cryptography, and the code that the project
  documents as critical (its failure harms users or the business). A mutation
  testing tool (such as Stryker, PIT, mutmut or cargo-mutants) runs on that
  code, in CI or on a schedule, and its latest report shows each surviving
  mutant either fixed by a new test or recorded with a reason (such as an
  equivalent mutant). A survivor that has neither a test nor a reason fails.
  No report fails.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.8.2; Stryker Mutator,
  PIT, mutmut and cargo-mutants as examples (read 2026-11-24)

Mutation testing SHOULD run on the authentication, authorization and other
security-relevant code, and on the code whose failure the project names as
critical, and each surviving mutant SHOULD be fixed or recorded with a reason.

**Rationale:** Coverage shows that a test ran the code, not that it would
notice a fault. A mutation survivor shows a change that no test catches.

### USS-TST-010: Use property-based tests for wide input ranges

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Reliability
- **Verify:** List the code that accepts a wide range of inputs: parsers,
  encoders and decoders, numeric calculations and data transformations. Each
  one has at least one property-based test (such as with Hypothesis,
  fast-check or proptest) that states a property (for example, that decoding
  an encoded value gives the original) and checks it against generated inputs.
  Code of that kind with only fixed examples fails. A project with no such
  code passes.
- **References:** Hypothesis, fast-check and proptest as examples (read
  2026-11-24)

Code that accepts a wide range of inputs, such as parsers, encoders, numeric
calculations and data transformations, SHOULD have property-based tests.

**Rationale:** Hand-written examples cover the cases that their author thought
of. Generated inputs find the ones that nobody did, such as an empty value or
the largest number.

### USS-TST-011: Fuzz parsers in memory-unsafe code

- **Level:** MUST
- **Applies when:** parses-untrusted-input and has-memory-unsafe-code
- **Category:** Reliability
- **Verify:** List each parser of untrusted input in the code (a file format,
  a protocol, a request body). Each one has a fuzz target (such as for
  libFuzzer, AFL++ or cargo-fuzz) that runs with sanitizers on (USS-SAF-004),
  and CI runs it (for a fixed time on each change, or on a schedule), or a
  continuous fuzzing service such as OSS-Fuzz runs it. A recent run exists.
  Crashes that it found are fixed or tracked as issues. A parser with no
  fuzz target fails.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.8.2; CWE-20; OSS-Fuzz (read
  2026-11-24)

Each parser of untrusted input MUST be fuzzed, with sanitizers on, in CI or on
a continuous fuzzing service.

**Rationale:** In memory-unsafe code a malformed input can become a memory
fault that an attacker uses. Fuzzing finds such inputs faster than review, and
the sanitizers turn a silent corruption into a crash.

### USS-TST-012: Fuzz other parsers of untrusted input

- **Level:** SHOULD
- **Applies when:** parses-untrusted-input and not has-memory-unsafe-code
- **Category:** Reliability
- **Verify:** List each parser of untrusted input in the code (a file format,
  a protocol, a request body). Each one has a fuzz target (such as for a
  libFuzzer-style engine, cargo-fuzz or the fuzzing support of the language's
  test tools) that CI runs, for a fixed time on each change or on a schedule,
  or that a continuous fuzzing service such as OSS-Fuzz runs. A recent run
  exists. A parser with no fuzz target fails.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.8.2; CWE-20; OSS-Fuzz (read
  2026-11-24)

Each parser of untrusted input SHOULD be fuzzed, in CI or on a continuous
fuzzing service.

**Rationale:** A safe language turns many faults into errors, but a parser can
still hang, use too much memory or give a wrong result on a crafted input.
