# APIs

- **Prefix:** API
- **Kind:** area
- **Applies when:** exposes-network-api

This area covers the network APIs that the software offers to other programs:
the contract, the version, the way old versions end, errors, collections and
retries. Its rules count toward Compatibility by default, because an API is a
promise to its clients. The Security (SEC) area holds input validation and
authorization of API calls, the Testing (TST) area holds the tests against the
contract, and the Data (DAT) area holds the changes to stored data.

### USS-API-001: Publish a machine-readable contract

- **Level:** MUST
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** For each network API that the software exposes, the repository
  holds a contract file in a machine-readable format (such as an OpenAPI or
  AsyncAPI document, a protobuf definition or a GraphQL schema), and the file
  parses with the tool of its format. Compare it with the implementation: each
  route, operation or message in the code appears in the contract, and each
  one in the contract exists in the code. A contract that the build generates
  from the code passes. An API with no contract file, or one that differs from
  the code in an operation, fails.
- **References:** OpenAPI Specification 3.2.1; AsyncAPI Specification 3.0.0;
  GraphQL Specification (October 2021)

Each network API MUST have a machine-readable contract in the repository, such
as OpenAPI, AsyncAPI, a protobuf definition or a GraphQL schema, that matches
the implementation.

**Rationale:** A contract that a tool can read lets clients generate code,
lets tests check the API and lets a review see what a change does to the
promise. A contract that has drifted from the code misleads each of them.

### USS-API-002: Version the API

- **Level:** MUST
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** The contract (USS-API-001) or the documentation states the
  API's version, and the version is part of the contract, the address or a
  request header. For each change to the contract in the last 12 months, or
  since the last audit, compare the old and the new contract: each change that
  removes or renames an operation, a field or a value, makes an optional field
  mandatory or changes a type or the meaning of a field is a breaking change,
  and it ships under a new major API version. A breaking change within the
  same major version fails.
- **References:** OpenAPI Specification 3.2.1, the info object (its version
  field)

The API MUST have a version, and each breaking change to it MUST be released
under a new major API version.

**Rationale:** A client cannot read the server's mind. The major version tells
it when an upgrade may break it, so a change inside one major version never
needs the client to change.

### USS-API-003: Keep old versions through a deprecation period

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** For each major API version that a newer one replaced in the last
  12 months, or since the last audit, the documentation gives a deprecation
  period and an end date, and the old version still answers requests until
  that date. Its responses carry the `Deprecation` header and the `Sunset`
  header with the end date (or, for a protocol with no headers, an equivalent
  field of the response that gives the end date). An old version that stopped
  before its date, or whose responses give no deprecation signal or no end
  date, fails. When no version was replaced in that time,
  there is nothing to check, and the rule passes.
- **References:** RFC 9745; RFC 8594; Martin Fowler, ParallelChange (expand and
  contract; read 2026-11-24), as an example of the practice

After a new major API version is released, the previous one SHOULD keep
working for a documented period, and its responses SHOULD signal the
deprecation and the end date.

**Rationale:** Clients change on their own schedule. A stated period and a
signal in each response give them time to move and a way to find out that
they must.

### USS-API-004: Use a standard error format

- **Level:** MUST
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** The documentation or the contract names the error format. For an
  HTTP API it is RFC 9457 problem details (the `application/problem+json`
  media type), and a request that causes an error, made against the running
  API or a test instance, returns it. For another protocol it is that
  protocol's own error model (such as the gRPC status codes or the GraphQL
  `errors` entry), used for each failure. An API whose errors come in two or
  more private formats, or in a format that the documentation does not name,
  fails.
- **References:** RFC 9457; gRPC, Status codes and their use in gRPC (read
  2026-11-24); GraphQL Specification (October 2021), section 7.1.2 (Errors)

Errors MUST use one documented format: RFC 9457 problem details for HTTP APIs,
or the protocol's own error model, such as gRPC status or GraphQL errors.

**Rationale:** A client that has to learn a new error shape for each
operation cannot handle errors in one place. A standard format also lets
shared libraries and gateways read the error.

### USS-API-005: Paginate unbounded collections

- **Level:** MUST
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** List each operation that returns a collection (a list, a search
  result or a stream of records). For each one whose size grows with the
  stored data or with usage, the contract shows a page size or limit
  parameter, a way to get the next page (such as a cursor, a page token or an
  offset) and a maximum for the page size, and the code enforces the maximum.
  Request more than the maximum from a test instance: the response is no
  larger than the maximum, or it is an error. An operation that returns each
  record in one response fails.
- **References:** OWASP API Security Top 10 2023, API4:2023 Unrestricted
  Resource Consumption

Each operation that returns a collection that can grow without limit MUST
return it in pages, with a maximum page size.

**Rationale:** A response that grows with the data grows until it exhausts
memory, time or bandwidth on the server or the client, and an attacker can
ask for it on purpose.

### USS-API-006: Make writes safe to retry

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Compatibility
- **Verify:** List each operation that creates or changes state. Each one is
  idempotent by its definition (such as an HTTP PUT or DELETE that sets a
  resource to a given state), or it accepts an idempotency key (such as an
  `Idempotency-Key` request header) that the contract documents. Send the same
  request twice with the same key to a test instance: the state changes once,
  and the second response matches the first. An operation that creates a
  second record on a repeated request, with no key to prevent it, fails.
- **References:** RFC 9110, section 9.2.2 (Idempotent Methods); IETF
  Internet-Draft draft-ietf-httpapi-idempotency-key-header-09 (The
  Idempotency-Key HTTP Header Field, 2026-11-21; a draft, not an RFC)

Each operation that creates or changes state SHOULD be idempotent, or accept
an idempotency key, so that a client can retry it safely.

**Rationale:** A network call can fail after the server has acted on it, and
the client cannot tell. If a retry cannot repeat the change, a client can
retry without fear of a double charge or a second record.
