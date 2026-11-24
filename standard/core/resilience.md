# Resilience

- **Prefix:** RES
- **Kind:** area

This area covers how the software behaves when what it depends on is slow,
broken or too busy: timeouts, retries, load limits and writes that survive a
crash. Its rules count toward Reliability. The API (API) area holds making the
server side of a write safe to retry (USS-API-006), and the Security (SEC)
area holds failing closed (USS-SEC-012); the atomic writes here are about
stored data and not about security.

### USS-RES-001: Time out every network call

- **Level:** MUST
- **Applies when:** uses-network
- **Category:** Reliability
- **Verify:** Search the code for each outgoing network call (HTTP, RPC,
  database, cache and queue clients, DNS and socket connects). Each has a
  finite timeout, set on the call or on the client that makes it, for the
  connection and for the whole call. Check the default of the library: where
  it is no timeout, or an infinite one, and the code does not set one, the call
  fails.
- **References:** Amazon Builders' Library, Timeouts, retries, and backoff with
  jitter (2019-12-05); CWE-400

Every outgoing network call MUST have a finite timeout.

**Rationale:** A call without a timeout waits as long as the other side stays
silent, and each waiting call holds a thread, a connection and memory. A slow
dependency then takes the caller down with it.

### USS-RES-002: Retry with backoff and jitter

- **Level:** MUST
- **Applies when:** uses-network
- **Category:** Reliability
- **Verify:** Search the code and the configuration for each retry (a loop
  around a call, a retry library, a client option that retries). For each one:
  the call is safe to repeat, which is a read, a call that is idempotent by its
  definition, or a write that carries an idempotency key (USS-API-006 is the
  server side); the delay grows exponentially between attempts; the delay has
  random jitter; and the number of attempts has a limit. A retry that fails any
  of these four fails. The code that never retries has nothing to check.
- **References:** Amazon Builders' Library, Timeouts, retries, and backoff with
  jitter (2019-12-05); Google SRE Book (2016), chapter 22, Addressing Cascading
  Failures

A failed call MUST be retried only when it is safe to repeat, and then with
exponential backoff, jitter and a limit on attempts.

**Rationale:** Retries add load to a service that is already struggling, and
clients that retry together arrive together. Backoff, jitter and a limit keep
retries from turning a short fault into a long outage, and a repeat of a call
that is not safe can do its work twice.

### USS-RES-003: Degrade gracefully

- **Level:** SHOULD
- **Applies when:** is-network-service
- **Category:** Reliability
- **Verify:** List the dependencies from the code and the configuration. A
  dependency counts as not essential when some documented feature or endpoint
  works without it (such as a cache, a recommendation service or a mail
  sender). For each one, stop it in a test instance: the endpoints that do not
  need it still answer successfully, and the service reports what is
  unavailable (in a response field or status, in the readiness detail, or in a
  log line and a metric). A service that fails every request because of a
  non-essential dependency fails.
- **References:** Google SRE Book (2016), chapter 22, Addressing Cascading
  Failures

When a dependency that is not essential fails, the service SHOULD keep serving
what does not need it and report what is unavailable.

**Rationale:** Users prefer a page without recommendations to no page. A
service that says what it cannot do lets users and operators act on it.

### USS-RES-004: Bound every queue, buffer and pool

- **Level:** MUST
- **Applies when:** is-network-service
- **Category:** Reliability
- **Verify:** List each queue, buffer, cache, connection pool and each
  endpoint that accepts a request body, in the code and the configuration.
  Each has a maximum size (a capacity, a count of entries or bytes, a pool
  size, a body size limit), set explicitly or by a documented default that is
  not unbounded. Test a request body above the limit on a test instance, and
  read how the code behaves when a queue or pool is full: the work is refused
  (an error status, a rejected task), and does not wait without end or grow
  memory.
- **References:** Google SRE Book (2016), chapter 22, Addressing Cascading
  Failures; CWE-400; CWE-770

Each queue, buffer, cache, connection pool and accepted request body MUST have
a maximum size, and work over the limit MUST be refused.

**Rationale:** An unbounded structure grows with the load until the process
runs out of memory, and the work that it holds is late by then. A refusal at a
known limit fails one request and keeps the others.

### USS-RES-005: Recover when a dependency returns

- **Level:** MUST
- **Applies when:** is-network-service
- **Category:** Reliability
- **Verify:** For each dependency that the service reaches over a network
  (such as a database, a cache or another service), on a test instance: start
  the service, stop the dependency, make a request that needs it (it fails),
  start the dependency again, and make the same request. It succeeds without a
  restart of the service. A service that needs a restart, or a pool that keeps
  broken connections, fails.
- **References:** none

After a dependency fails and comes back, the service MUST use it again without
a restart.

**Rationale:** A dependency is down for a short time more often than it is
gone. A service that stays broken after it returns turns each short fault into
a person's task.

### USS-RES-006: Write related changes atomically

- **Level:** MUST
- **Applies when:** stores-persistent-data
- **Category:** Reliability
- **Verify:** Find each operation in the code that writes more than one row,
  record or file that the data model ties together (such as a foreign key, a
  counter or a copy of one value), and each write that replaces the content of
  an existing file. Each one is in a single transaction, or writes a new file
  and renames it over the old one. A write that truncates a file and writes it
  again in place, or two writes in separate transactions that must agree, fails.
- **References:** none

Changes to stored data that belong together MUST be written atomically (such as
in a transaction, or by writing a new file and renaming it), so a crash never
leaves data half written.

**Rationale:** A crash, a full disk or a lost connection can stop a process at
any line. If related changes are all or nothing, the data is valid after each
such stop, and no repair is needed.
