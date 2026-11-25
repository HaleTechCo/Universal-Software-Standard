# PostgreSQL

- **Prefix:** PG
- **Kind:** module
- **Applies when:** platform.postgresql

This module covers projects that use PostgreSQL: how their migrations are
checked and set up, how the application connects, and which PostgreSQL major
versions they run on. PostgreSQL is the module's subject, so it names its
settings and statements directly. It uses one vendor profile,
`standard/vendors/sbdchd/squawk.md`, and names no tool without a profile. The
rules were checked against the PostgreSQL 19 documentation and Squawk 2.74.0,
the current releases on 2026-11-24. The core rules that this module does not
refine apply as written.

### USS-PG-001: Lint migrations for unsafe operations

- **Level:** MUST
- **Applies when:** stores-persistent-data
- **Category:** Reliability
- **Refines:** USS-DAT-002
- **Verify:** Read the CI configuration. A step runs a migration linter over
  each migration file that the change under test adds (it may run over all of
  the migration files). The linter's documentation names its checks for
  operations that lose data (such as dropping a table or a column) and for
  operations that hold a long lock (such as creating an index without
  `CONCURRENTLY`, or adding a constraint that scans the table while it blocks
  writes), and the configuration leaves each of those checks on. The job fails
  when the linter reports a finding, as the linter's documentation states for
  its exit status (Squawk exits with status 1). A migration file that the
  linter does not read, a linter that the job runs with failures tolerated and
  a check that the configuration turns off fail.
- **References:** Squawk 2.74.0, documentation, Rules (read 2026-11-24);
  Squawk 2.74.0, source of the linter and of the command line (read
  2026-11-24)

CI MUST check each new migration for operations that lose data or hold long
locks, and a finding MUST fail the build.

**Rationale:** A statement such as `DROP COLUMN` or `CREATE INDEX` without
`CONCURRENTLY` is valid and reads as harmless in a review, yet it destroys data
or blocks every query on the table while it runs. A linter reads each
statement before the migration reaches a database, and a finding that does not
fail the build is soon ignored. The linter finds the operations that it knows:
the rules of the core area still judge what a migration does to stored data.

### USS-PG-002: Lint migrations with Squawk

- **Level:** SHOULD
- **Applies when:** stores-persistent-data
- **Category:** Reliability
- **Refines:** USS-DAT-002
- **Verify:** CI runs `squawk` (or `squawk check`) over the migration files,
  and the job fails when Squawk exits with status 1, which it does when it
  reports a violation. Squawk reads `.squawk.toml` (from the current directory
  upwards) or the file that `--config` names, and the configuration includes
  the baseline of the profile `standard/vendors/sbdchd/squawk.md`: `pg_version`
  is set (or `--pg-version` gives it) to a version no higher than the oldest
  PostgreSQL major version that the project supports, `excluded_rules` lists
  no rule and `excluded_paths` lists no path. The job passes no `--exclude` or
  `--exclude-path` option (each replaces the list of the configuration file).
  A project that uses another linter fails this rule and records a Standard
  Deviation; it meets USS-PG-001 through that linter's own profile.
- **References:** Squawk 2.74.0, documentation, Squawk CLI and Rules (read
  2026-11-24); Squawk 2.74.0, source of `config.rs` and `reporter.rs` (read
  2026-11-24)

The project SHOULD lint its migrations with Squawk, configured as its vendor
profile says.

**Rationale:** Squawk reads PostgreSQL migration files without a database and
has checks for most of the operations that lose data or hold long locks, so a
project gets them without writing checks of its own. The profile fixes the
configuration that makes "no findings" mean the same in every project.

### USS-PG-003: Set a lock timeout in migrations

- **Level:** MUST
- **Applies when:** stores-persistent-data
- **Category:** Reliability
- **Refines:** USS-DAT-001
- **Verify:** Take each migration of the project. Either the migration file
  sets `lock_timeout` to a finite value above zero (`SET lock_timeout`,
  `SET LOCAL lock_timeout` or `set_config('lock_timeout', ...)`) before its
  first statement that changes the schema, or the connection that runs the
  migrations sets it for the session: in its connection options
  (`options=-c lock_timeout=...`), or as a default of the migration role or its
  database (`ALTER ROLE ... SET lock_timeout`, `ALTER DATABASE ... SET
  lock_timeout`) that the repository sets. A value of 0 disables the timeout
  and fails. A migration for which none of these holds fails.
- **References:** PostgreSQL 19 documentation, Client Connection Defaults:
  `lock_timeout` (read 2026-11-24); PostgreSQL 19 documentation, `ALTER ROLE`
  (read 2026-11-24); Squawk 2.74.0, rule `require-lock-timeout` (read
  2026-11-24)

Each migration MUST set `lock_timeout` to a finite value before its first
statement that changes the schema, in the file or on the connection that runs
it, so that it fails instead of blocking other queries.

**Rationale:** A statement that changes a table waits for a lock, and every
query that arrives after it waits behind it. With no `lock_timeout` the
migration waits as long as the holder of the lock lasts, and the whole table
stops. With one, the migration fails, and the automatic upgrade of
USS-DAT-001 can be run again when the table is quiet.

### USS-PG-004: Connect with verify-full TLS

- **Level:** MUST
- **Applies when:** uses-network
- **Category:** Safety
- **Refines:** USS-SEC-022, USS-SEC-023
- **Verify:** Take each place where the repository configures a connection to
  a PostgreSQL server: a connection string or URI, a keyword and value
  string, an `sslmode` parameter of a driver, the `PGSSLMODE` environment
  variable, a service file. Each one sets `sslmode=verify-full`. A driver that
  does not read libpq parameters has its own setting that makes it check the
  certificate chain and the host name, and that setting is on. A connection
  through a Unix-domain socket is not over a network and is not judged. An
  unset `sslmode` fails, because libpq's default is `prefer`, and so does any
  other mode.
- **References:** PostgreSQL 19 documentation, libpq: Database Connection
  Control Functions, `sslmode`, `sslrootcert` and `ssl_min_protocol_version`
  (read 2026-11-24); PostgreSQL 19 documentation, libpq: SSL Support (read
  2026-11-24)

Each connection to a PostgreSQL server over a network MUST use
`sslmode=verify-full`. A driver that does not read libpq parameters uses its
own setting that checks the certificate chain and the host name. A connection
through a Unix-domain socket is not over a network.

**Rationale:** `verify-full` is the one mode that both encrypts the connection
and proves that the server is the host that was asked for: `require` accepts
any certificate, and `verify-ca` accepts any certificate from a trusted
authority, whatever its host name. A client that connects with a weaker mode
can be redirected to a server of an attacker. libpq's own minimum protocol
version is TLS 1.2, which meets USS-SEC-022.

### USS-PG-005: Connect with a least-privilege role

- **Level:** MUST
- **Applies when:** uses-secrets
- **Category:** Safety
- **Refines:** USS-SEC-007
- **Verify:** Find the role that the application's connection configuration
  names, and the role that the migrations use. They are two different roles.
  In the migrations and the role-setup scripts of the repository, the
  application role owns no object (no table, view, sequence, function, schema
  or type: each `CREATE` runs as the migration role, or is followed by
  `ALTER ... OWNER TO` that names another role). The application role has no
  `SUPERUSER`,
  `CREATEDB`, `CREATEROLE` or `BYPASSRLS` attribute, and the privileges that
  `GRANT` statements give it (to the role or to a group role that it belongs
  to, or to `PUBLIC`) are the ones that the code uses: a table privilege that
  no query of the code needs fails. Where the repository sets up no roles, and
  the roles live on a host that the auditor cannot see, record `unverified`.
- **References:** PostgreSQL 19 documentation, Privileges (read 2026-11-24);
  PostgreSQL 19 documentation, `CREATE ROLE` (read 2026-11-24)

The application MUST connect with a role that owns no objects and has only the
privileges that it uses, and the migrations MUST run with a separate role.

**Rationale:** The owner of an object holds every privilege on it and can drop
it, and that right cannot be revoked. An application that owns its tables
gives every flaw in its queries the power to drop them. A separate role for
the migrations keeps that power away from the running application.

### USS-PG-006: Set a statement timeout

- **Level:** MUST
- **Applies when:** uses-network
- **Category:** Reliability
- **Refines:** USS-RES-001
- **Verify:** Take each place where the application opens a connection to a
  PostgreSQL server. The sessions of that connection have `statement_timeout`
  set to a finite value above zero: in the connection options
  (`options=-c statement_timeout=...`), by a `SET statement_timeout` that runs
  when each connection opens (such as a pool's initialisation statement), or as
  a default of the application's role or its database that the repository
  sets (`ALTER ROLE ... SET statement_timeout`, `ALTER DATABASE ... SET
  statement_timeout`). A value of 0 disables the timeout and fails. Where the
  settings live on a host that the auditor cannot see, record `unverified`.
- **References:** PostgreSQL 19 documentation, Client Connection Defaults:
  `statement_timeout` (read 2026-11-24); PostgreSQL 19 documentation, libpq:
  Database Connection Control Functions, `options` (read 2026-11-24);
  PostgreSQL 19 documentation, `ALTER ROLE` (read 2026-11-24)

Each connection that the application opens MUST set `statement_timeout` to a
finite value.

**Rationale:** A client timeout stops the client from waiting, but the server
goes on running the statement and holds its locks and its memory. The server's
own `statement_timeout` ends the statement. It is the whole-call timeout of
USS-RES-001 for the database.

### USS-PG-007: No unsupported PostgreSQL versions

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SUP-006
- **Verify:** Take the versioning policy of the PostgreSQL project at the audit
  date. A major version is unsupported when the policy marks it unsupported or
  its final release date has passed. Collect each PostgreSQL major version that
  the project runs on or tests against: the image tags of the Compose files,
  manifests and Dockerfiles, the versions in the CI matrix and its service
  containers and the versions in the version files. None of them is
  unsupported. A reference that names
  no major version (such as `latest`) is not judged here. Where the policy
  cannot be read, record `unverified`.
- **References:** PostgreSQL Global Development Group, Versioning Policy (read
  2026-11-24)

The project MUST NOT run on, or test against, a PostgreSQL major version that
the PostgreSQL project no longer supports.

**Rationale:** The PostgreSQL project supports a major version for five years
and then releases a last minor version, so a flaw found afterwards is not
fixed in it. A project that tests against an unsupported version also
invites its users to run on one.

### USS-PG-008: Test each supported major version

- **Level:** SHOULD
- **Applies when:** uses-relational-database
- **Category:** Reliability
- **Refines:** USS-TST-004
- **Verify:** The PostgreSQL major versions that the project supports are those
  that its documentation names (a range names each major version in it); a
  project that names none fails. CI runs the integration tests against a real
  PostgreSQL server of each of those major versions, in a matrix, a service
  container or a container that the tests start, with one run for each major
  version (any minor release of it). A supported major version without a run
  fails.
- **References:** PostgreSQL Global Development Group, Versioning Policy (read
  2026-11-24)

CI SHOULD run the integration tests against each PostgreSQL major version that
the project supports.

**Rationale:** Major versions of PostgreSQL change the planner, the defaults
and the catalogue, so a query that works on one can behave differently on
another. The version that a user runs is the one that the project did not
test, unless it tests all that it names.
