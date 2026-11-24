# Observability

- **Prefix:** OBS
- **Kind:** area

This area covers what the software lets its operators see and control while it
runs: logs, health, metrics, traces and the way it stops. Its rules count
toward Reliability. The Privacy (PRV) area holds keeping sensitive data out of
logs and telemetry (USS-PRV-004), and the Deployment hardening (HRD) area
holds the probes in Kubernetes manifests; the health rule here is about the
service's own endpoints.

### USS-OBS-001: Write structured logs

- **Level:** MUST
- **Applies when:** is-network-service or operates-service
- **Category:** Reliability
- **Verify:** Run the software, or read its logging set-up and its tests, and
  take a sample of log lines (at least 20, from start-up, a request and an
  error). Each line is a record that a parser reads without a custom pattern
  (such as JSON, or key-value pairs with fixed keys), and has a timestamp, a
  level and a message. A line of free text without those three fields fails.
  For the project's own deployment, the sample comes from its production log
  output, when the auditor has access.
- **References:** OpenTelemetry Specification 1.63.0, Logs Data Model
  (Timestamp, SeverityText, Body)

Each log record MUST be structured (such as JSON), with at least a timestamp, a
level and a message.

**Rationale:** Operators search and count logs with tools, and a tool reads
fields better than prose. A timestamp and a level are the least that every
query needs.

### USS-OBS-002: Let operators set the log level

- **Level:** MUST
- **Applies when:** is-network-service or has-cli
- **Category:** Reliability
- **Verify:** The documentation names the setting (a configuration key, an
  environment variable or a command-line option) that sets the log level.
  Start the software twice, with two different levels (such as the most
  detailed and the quietest), without a rebuild: the output differs as the
  levels say, and the setting is read in the code at start-up.
- **References:** none

The log level MUST be changeable through configuration or an option, without a
rebuild.

**Rationale:** An operator who looks into a fault needs more detail for an
hour, and quiet logs the rest of the time. A rebuild for that is not possible
in production.

### USS-OBS-003: Provide health and readiness checks

- **Level:** MUST
- **Applies when:** is-network-service
- **Category:** Reliability
- **Verify:** The documentation names two endpoints (or two commands) of the
  service itself: one for liveness and one for readiness. Start a test
  instance and query both. Before start-up has finished, readiness reports not
  ready. Stop a dependency that the service needs to serve its main requests
  (such as its database): readiness reports not ready, and liveness still
  reports alive. Probes in deployment manifests are not read here.
- **References:** none

The service MUST report separately whether it is alive and whether it is ready
to serve, such as through two HTTP endpoints.

**Rationale:** An orchestrator or a load balancer restarts a service that is
not alive, and stops sending work to one that is not ready. One check for both
turns a slow database into a restart storm.

### USS-OBS-004: Expose metrics

- **Level:** SHOULD
- **Applies when:** is-network-service
- **Category:** Reliability
- **Verify:** The service has an endpoint, or an exporter, for metrics in a
  standard format (such as OpenTelemetry, or the Prometheus or OpenMetrics
  exposition format). Query it on a test instance after some requests, one of
  them failing: it gives a count or rate of requests, a count or rate of
  errors, a latency measure (such as a histogram) and a measure of saturation
  (such as queue length, pool use, memory or CPU use). A set that lacks one of
  the four fails.
- **References:** Google SRE Book (2016), chapter 6, the four golden signals;
  Prometheus, Exposition formats (read 2026-11-24), the text format and
  OpenMetrics

The service SHOULD expose metrics for its request rate, errors, latency and
saturation in a standard format (such as OpenTelemetry or the Prometheus
exposition format).

**Rationale:** These four numbers tell an operator whether users are served and
how close the service is to its limit. A standard format lets the operator
use the monitoring tools that they already run.

### USS-OBS-005: Emit traces

- **Level:** SHOULD
- **Applies when:** is-network-service
- **Category:** Reliability
- **Verify:** The code or the dependencies show an OpenTelemetry tracing set-up
  that creates a span for each incoming request and exports spans (to a
  collector, or to a console exporter for a test). For each outgoing call to
  another service, the code or the instrumentation library sends a
  `traceparent` header. On a test instance, call the service with a
  `traceparent` header and capture an outgoing request: it carries the same
  trace ID.
- **References:** OpenTelemetry Specification 1.63.0, Trace API; W3C Trace
  Context, Level 1 Recommendation (2021-11-23), section 3.2

The service SHOULD emit OpenTelemetry traces and pass W3C Trace Context on to
the services that it calls.

**Rationale:** In a system of several services, a slow request is a puzzle
unless one identifier follows it through each call. Trace context is that
identifier.

### USS-OBS-006: Shut down gracefully

- **Level:** MUST
- **Applies when:** is-network-service
- **Category:** Reliability
- **Verify:** The code handles the termination signal (such as SIGTERM) or the
  runtime does, and the documentation gives the time limit for shutdown. On a
  test instance, start a slow request and send the signal: new connections or
  requests are refused (or readiness reports not ready), the slow request ends
  with its normal result, or its work is handed off (such as by returning it to
  a queue), and the process exits within the time limit, with a status of 0 when
  it finished cleanly. A process that is killed at once, or never exits, fails.
- **References:** none

On a termination signal, the service MUST stop taking new work, finish or hand
off its work in progress within a time limit, and then exit.

**Rationale:** A service is stopped at each deployment and each scaling step. An
abrupt stop drops the requests in flight, and a stop that waits forever
blocks the deployment.
