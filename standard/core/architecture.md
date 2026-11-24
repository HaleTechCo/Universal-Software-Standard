# Architecture

- **Prefix:** ARC
- **Kind:** area

This area covers how a project records and keeps its structure: decision
records, configuration kept out of code, stateless service instances, service
boundaries and module boundaries. Its rules count toward Maintainability by
default. The Documentation (DOC) area holds the architecture document that
describes the result, and the Hardening and Operations areas hold how a
deployed service runs.

### USS-ARC-001: Record significant decisions

- **Level:** MUST
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** Find the architecture decision records (ADRs) in the repository
  (such as a `docs/adr` or `docs/decisions` directory). A decision is
  significant when it is one of: the choice of a language or main framework,
  a data store, an external service that the software depends on, the split
  into deployable parts, or any other choice that the project names as costly
  to reverse. For each significant decision that the project has made, an
  ADR exists, and it states the context, the options considered and the
  reason for the choice. A decision made before the first ADR passes when a
  later ADR records it. A record that lists no option other than the one
  chosen fails.
- **References:** MADR 4.0.0; Michael Nygard, Documenting Architecture
  Decisions (2011-11-15)

Each significant technical decision MUST have an architecture decision record
(ADR) in the repository that states the context, the options considered and
the reason for the choice.

**Rationale:** A decision is easy to see in the code and hard to understand.
Without the reason, a later maintainer either keeps a choice that no longer
fits or reverses it and repeats the mistake that it avoided.

### USS-ARC-002: Keep configuration out of the code

- **Level:** MUST
- **Applies when:** is-network-service or operates-service or
  ships-container-image
- **Category:** Maintainability
- **Verify:** List the settings that differ between deployments: addresses,
  credentials, limits and feature switches. Each one is read from the
  environment or from a configuration file at start-up, and the code holds
  no value for it other than a default that suits a local run. A search of
  the code for host names, ports, URLs and credentials of a deployment finds
  none. The same build artifact runs in each deployment without a rebuild.
- **References:** The Twelve-Factor App, III Config (read 2026-11-24)

Every setting that differs between deployments (such as addresses,
credentials, limits and feature switches) MUST be read from the environment
or a configuration file, not set in code.

**Rationale:** If a deployment's settings are in the code, each deployment
needs its own build, and a credential in the code is also in every copy of
the repository. Secrets handling is the subject of the Security (SEC) area.

### USS-ARC-003: Keep service instances stateless

- **Level:** SHOULD
- **Applies when:** is-network-service
- **Category:** Maintainability
- **Verify:** Look for state that a later request from another instance could
  need and that an instance keeps in its own memory or on its own disk:
  session data, uploaded files that are not yet stored elsewhere, job queues,
  caches that hold the only copy of data. Each such state is in a backing
  service (such as a database, a cache server or an object store). A cache
  that can be rebuilt from a backing service, or a file that exists only
  during one request, passes.
- **References:** The Twelve-Factor App, VI Processes (read 2026-11-24)

A service instance SHOULD keep no state that another instance needs in its
own memory or disk. Shared state lives in a backing service, so any instance
can serve any request.

**Rationale:** If any instance can serve any request, instances can be added,
removed and restarted at any time, and a failed instance loses nothing.

### USS-ARC-004: Do not split into services without a reason

- **Level:** SHOULD NOT
- **Applies when:** is-network-service
- **Category:** Maintainability
- **Verify:** List the separately deployed services of the project (each
  deployable unit with its own process, image or manifest). For each one
  beyond the first, an ADR (USS-ARC-001) records why the split is needed,
  with the options considered. A reason such as "microservices are best
  practice" with no cause that applies to this project fails.
- **References:** none

The software SHOULD NOT be split into separately deployed services unless an
ADR records why the split is needed.

**Rationale:** Each service adds a network boundary, a deployment, and a
failure mode that a function call does not have. A project that has no need
for the split carries the cost without the benefit.

### USS-ARC-005: Check module boundaries in CI

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Maintainability
- **Verify:** A file in the repository states which dependencies between the
  project's modules are allowed (such as an ArchUnit test, an import-linter
  contract or a dependency-cruiser configuration). A CI job checks that file
  against the code and fails when a change adds a dependency that it does not
  allow. A project with a single module and no internal layers passes when
  its architecture document says so.
- **References:** none

The allowed dependencies between the project's modules SHOULD be written down,
and a CI check SHOULD fail a change that breaks them.

**Rationale:** A boundary that nothing checks erodes one convenient import at
a time, until a change to one module needs a change to all of them.
