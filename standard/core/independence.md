# Independence

- **Prefix:** IND
- **Kind:** area

This area covers whether users can run the software on their own terms: on
their own infrastructure, without a commercial product, without one cloud
provider, and with the database they already have. Its rules count toward
Compatibility. The Distribution (DST) area holds installing without internet
access (USS-DST-013); this area holds running without it. The Testing (TST)
area holds tests in CI (USS-TST-001) and tests against real services
(USS-TST-004), and the Data (DAT) area holds versioned migrations
(USS-DAT-001). The Architecture (ARC) area holds the rule that configuration
stays out of code (USS-ARC-002), which is what lets a deployment swap one
service for another.

## Self-hosting

### USS-IND-001: Support self-hosting

- **Level:** SHOULD
- **Applies when:** is-network-service
- **Category:** Compatibility
- **Verify:** Find the self-hosting guide in the documentation. It lists
  everything that a self-hosted installation needs, and each item is something
  that the operator runs or controls (the software's own components, a
  database, a proxy). Install the software by the guide on infrastructure
  that you control. Then run it with no connection to a service that the
  project or a vendor runs: it starts and does its main job as the guide
  describes. Search the code and the configuration for such
  connections (a licence server, a phone-home call, a required sign-in to a
  hosted service): each is optional and off by default, or the guide names it
  as an optional integration. A required connection to a service outside the
  operator's control while running, or a missing guide, fails.
- **References:** none

The software SHOULD run entirely on the user's own infrastructure, with a
documented self-hosting guide, and need no connection to a service outside it.

**Rationale:** A user who cannot run the software themselves depends on its
maker for the service, the data and the price. A guide matters as much as the
ability: software that only the maker knows how to run is not self-hostable.

### USS-IND-002: Do not cut down the self-hosted edition

- **Level:** SHOULD NOT
- **Applies when:** is-network-service and operates-service
- **Category:** Compatibility
- **Verify:** The project runs a hosted edition. Compare the feature list of
  the hosted edition (its documentation, pricing page or settings) with that of
  the self-hosted edition. Every feature of the hosted edition is in the
  self-hosted edition. A hosted feature that is not in
  the self-hosted edition fails. The comparison is of what the software does;
  the operator's own capacity, support and the hosting itself are not
  features.
- **References:** none

The self-hosted edition SHOULD NOT lack features of the software that the
hosted edition has.

**Rationale:** A self-hosted edition with fewer features turns self-hosting
into a lesser product, and pushes users toward the hosted one.

### USS-IND-003: Run without internet access

- **Level:** SHOULD
- **Applies when:** is-network-service
- **Category:** Compatibility
- **Verify:** List each feature that does not inherently need the internet (a
  feature that needs it by its nature, such as fetching a remote web page or
  sending email to an outside address, is not on the list). Run the software
  in a test environment with all outbound traffic blocked, after its install
  from local media (USS-DST-013 covers that install): it starts, and each
  feature on the list works. Read the logs and the traffic: no feature on the
  list waits for an outside host, or fails because of one (such as an update
  check, a font or script from a CDN, a licence check or telemetry). A listed
  feature that fails or hangs without internet access fails.
- **References:** none

Every feature of the software that does not inherently need the internet
SHOULD work where outbound traffic is restricted, or where there is no
internet access.

**Rationale:** Many sites restrict outbound traffic, or have none at all. A
feature that quietly reaches out for a font, an update check or a licence
fails there, often in a way that is hard to find.

## Open dependencies

### USS-IND-004: Need no commercial product

- **Level:** SHOULD NOT
- **Applies when:** is-foss
- **Category:** Compatibility
- **Verify:** List every service, tool and product that the software needs to
  run its documented features (from the install guide, the dependency
  manifests, the container and deployment files, and the configuration). For
  each, find a FOSS option (a service or tool released under an OSI-approved
  licence) that the documentation names or that the code supports through a
  setting or an interface, and that gives the same feature. A proprietary
  product, service or subscription that is not optional (the software does not
  work, or a documented feature does not work, without it) and has no FOSS
  option fails. Proprietary services that are optional integrations pass.
- **References:** none

The software SHOULD NOT require a commercial product or subscription to work:
every required service or tool has a FOSS option, and proprietary ones are
optional integrations.

**Rationale:** Software released as FOSS that works only with a paid service is
free in name. An AI feature that calls a hosted model is the usual case: it
stays independent when it can also point at a model that the user runs.

### USS-IND-005: Need no single cloud provider

- **Level:** SHOULD NOT
- **Applies when:** uses-network
- **Category:** Compatibility
- **Verify:** List the external services that the software calls or deploys
  to (object storage, queues, identity, databases, metrics and tracing, and
  similar), from the code, the deployment files and the configuration. For
  each, find the interface that the software uses to reach it: an open or
  widely implemented protocol (such as an S3-compatible API, OpenID Connect,
  OpenTelemetry, SMTP or SQL), or an interface that is proprietary to one
  provider. A service that the software reaches only through one provider's
  proprietary interface, with no alternative that the code supports or the
  documentation describes, fails. An optional integration that is off by
  default passes.
- **References:** The Twelve-Factor App, IV. Backing services (read
  2026-11-24)

The software SHOULD NOT require one cloud provider's proprietary services; it
uses open or widely implemented protocols, such as S3-compatible storage, OIDC
or OpenTelemetry.

**Rationale:** A required service of one provider ties the software, and its
users, to that provider's prices, regions and terms. A common protocol lets the
user choose the provider, or run the service themselves.

## Databases

### USS-IND-006: Support SQLite, PostgreSQL and MariaDB or MySQL

- **Level:** SHOULD
- **Applies when:** uses-relational-database
- **Category:** Compatibility
- **Verify:** The documentation lists the supported databases, and the list
  has SQLite, PostgreSQL, and MariaDB or MySQL (one of the two, or both). Each
  listed database has installation or configuration instructions, and a
  setting that selects it. Start the software against each of the three, with
  the instructions: it starts and runs its migrations. A listed database that
  the software does not start against fails, and so does a list that lacks one
  of the three.
- **References:** none

The software SHOULD support SQLite, PostgreSQL, and MariaDB or MySQL.

**Rationale:** Users run the database that they already know and operate.
Software for one database only forces them to run a second one. A project that
depends on one database's distinctive features, such as PostGIS, JSONB
operators or LISTEN/NOTIFY, records a Standard Deviation (spec §6.1); the
deviation states the feature and why it cannot be had elsewhere.

### USS-IND-007: Test every supported database in CI

- **Level:** MUST
- **Applies when:** uses-relational-database
- **Category:** Compatibility
- **Verify:** List the databases that the documentation, the release notes and
  the configuration claim to support. Open the CI configuration: its test job
  runs the full test suite (the same suite that the project runs against its
  main database; USS-TST-001 has the tests in CI, and USS-TST-004 has the tests
  against real services) once for each listed database, with a real engine of
  that kind (a container or a service, not a mock). In the CI runs since the
  last audit, or the last 12 months when there is none, the matrix ran for each
  database. A claimed database that is missing from the matrix, or whose runs
  skip tests that the others run, fails.
- **References:** none

The full test suite MUST run in CI against every database that the software
claims to support.

**Rationale:** Support that no test proves is only a claim. Databases differ
in types, locking, case rules and SQL dialect, and a bug that shows on one is
easy to miss on another.

### USS-IND-008: Access databases through an abstraction layer

- **Level:** SHOULD
- **Applies when:** uses-relational-database
- **Category:** Compatibility
- **Verify:** Search the code for database access (queries, connections, and
  driver imports). All of it is in one layer that hides the database: an ORM,
  a query builder, or repository interfaces with one adapter for each
  supported database. Code outside that layer (business logic, handlers, and
  views) has no SQL text and no import of a database driver. A query outside
  the layer, or a layer with no adapter for a supported database, fails.
- **References:** none

Database access SHOULD go through an abstraction layer: an ORM, a query
builder, or repository interfaces with an adapter for each database.

**Rationale:** The layer is where the differences between databases are
handled, so that the rest of the code does not need to know them. A layer does
not have to be an ORM; a set of repository interfaces does the same work.

### USS-IND-009: Write migrations for each supported database

- **Level:** SHOULD
- **Applies when:** uses-relational-database
- **Category:** Compatibility
- **Verify:** Open the migrations (versioned under USS-DAT-001). For each
  supported database, each migration exists in a form that runs on it: a
  separate script for each database, or one definition from which the migration
  tool generates the statements for each. For the migrations added since the
  last audit, or the last 12 months when there is none, each has a form for
  every supported database. A migration that runs on only some of them fails.
- **References:** none

Each migration SHOULD be written or generated for every supported database.

**Rationale:** A migration is the point where the databases differ most, in
column types, alter-table support and locking. One that runs on only one
database leaves users of the others unable to upgrade.
