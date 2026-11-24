# Performance and efficiency

- **Prefix:** PRF
- **Kind:** area

This area covers how fast and how lean the software is, and how much it costs
the people who use it and the planet that it runs on. Its rules count toward
Efficiency. The Resilience (RES) area holds the bounds on queues and pools
(USS-RES-004), and the Observability (OBS) area holds the service metrics that
the budgets and the load test read (USS-OBS-004).

### USS-PRF-001: Set performance budgets

- **Level:** SHOULD
- **Applies when:** has-ui or has-cli or is-network-service or is-library
- **Category:** Efficiency
- **Verify:** Open the project's written performance budgets (a document, or a
  configuration file that a tool reads). For each kind of software that the
  project is, at least one measure has a limit given as a number with its unit,
  and says how it is measured: for a UI, the time to load or to respond to an
  input, or the bundle size; for a command-line tool, the start-up time; for a
  network service, the latency at a stated percentile; for a library, the
  size, the time of its main operations, or the memory use. A kind with no
  budgeted measure, or a budget with no number, fails.
- **References:** web.dev, Performance budgets 101 (read 2026-11-24)

Written budgets SHOULD set limits for the measures that matter to the
software's users, such as startup time, latency, memory use, and binary or
bundle size.

**Rationale:** Software gets slower by small steps that nobody decides on. A
number written down gives each change something to be measured against, and
users feel the sum of the steps.

### USS-PRF-002: Fail CI on performance regressions

- **Level:** SHOULD
- **Applies when:** has-ui or has-cli or is-network-service or is-library
- **Category:** Efficiency
- **Verify:** Take the measures that the project has budgeted. The CI
  configuration has a job, for each of them, that runs a benchmark or a
  measurement on the change (or on a schedule, for a benchmark that takes
  too long for each change). The job states a threshold (an absolute limit or
  a percentage against a baseline), and the job fails the build when the
  result is beyond it; the failure blocks the merge or the release as a
  failed required check does. A job that only records the numbers, or a
  measure with no job, fails.
- **References:** web.dev, Incorporate performance budgets into your build
  process (read 2026-11-24)

Benchmarks for the budgeted measures SHOULD run in CI, and a regression beyond
a stated threshold SHOULD fail the build.

**Rationale:** A budget that is checked by hand is checked rarely. A failing
build is the one signal that every contributor sees before the slowdown ships.

### USS-PRF-003: Meet the Core Web Vitals

- **Level:** SHOULD
- **Applies when:** has-web-ui
- **Category:** Efficiency
- **Verify:** List the pages that the project's own deployment or demo
  serves (from the router or the sitemap; the 20 most visited when there are
  more). For each, take field data at the 75th percentile of page loads (such
  as the Chrome UX Report, or the project's own real-user monitoring):
  Largest Contentful Paint is 2.5 seconds or less, Interaction to Next Paint
  is 200 milliseconds or less, and Cumulative Layout Shift is 0.1 or less. A
  page above one threshold fails. Without field data (a project that has no
  deployment with traffic), the auditor records `unverified`.
- **References:** web.dev, Web Vitals (read 2026-11-24)

Web pages SHOULD meet the "good" thresholds of the Core Web Vitals at the 75th
percentile.

**Rationale:** The three measures stand for what users notice: how soon the
content shows, how soon the page answers an input, and whether it stays still
while it loads. The 75th percentile keeps the slower quarter of visits in view.

### USS-PRF-004: Load-test the service

- **Level:** SHOULD
- **Applies when:** operates-service and is-network-service
- **Category:** Efficiency
- **Verify:** The repository or the documentation holds the load test (a
  script and its settings) and states the expected peak load (such as requests
  per second, or concurrent users) and the margin that is added to it (such as
  50 percent). The release process runs the test before each release. In the
  last 12 months, or since the last audit when that is shorter, each release
  has a result record that shows a run at a load of at least the peak plus the
  margin. A release with no record, or a run below that load, fails.
- **References:** Google, Site Reliability Engineering (2016), chapter 17,
  "Stress test"

Before each release, a load test SHOULD run the service at its expected peak
load plus a stated margin.

**Rationale:** Capacity limits found by a load test cost an afternoon. Found by
users at the day's peak, they cost an outage. The margin covers the peak that
nobody forecast.

### USS-PRF-005: Do not busy-wait

- **Level:** MUST NOT
- **Applies when:** has-ui or has-cli or is-network-service
- **Category:** Efficiency
- **Verify:** Run the software idle, with nothing to do, for 60 seconds
  after start-up, and measure the process's processor time: it stays below 1
  percent of one core. Also search the code for loops whose body checks a
  condition and repeats, with no call that blocks (no wait on an event, a
  lock, a socket or a timer): each is a finding. A loop that waits on an event
  or a timer between checks passes. A process that uses a core at rest, or such
  a loop, fails.
- **References:** Apple, Energy Efficiency Guide for iOS Apps, Minimize Timer
  Use (read 2026-11-24)

The software MUST NOT wait in a loop that uses CPU while it has nothing to do;
when idle, it waits on events or timers.

**Rationale:** A spinning loop wastes energy, heats a laptop, drains a battery
and takes processor time from other programs, for no work. Waiting on an event
costs nothing until the event comes.

### USS-PRF-006: Measure carbon intensity

- **Level:** SHOULD
- **Applies when:** operates-service
- **Category:** Efficiency
- **Verify:** Open the project's calculation of the Software Carbon Intensity
  of its production service: it states the software boundary and the
  functional unit, and gives the energy, the carbon intensity of the grid, the
  embodied emissions and the resulting score, as the SCI specification sets
  them. The project keeps the score of each release in the last 12 months, or
  since the last audit when that is shorter (such as a file in the
  repository, or a dashboard that shows the history). A release with no
  score fails.
- **References:** Green Software Foundation, Software Carbon Intensity (SCI)
  Specification, version 1.1.0 (read 2026-11-24)

The project SHOULD calculate the Software Carbon Intensity of the service that
it operates, and track it from release to release.

**Rationale:** What is not measured is not reduced. The score divides the
emissions by a unit of work, so a service that grows can still see whether it
got more efficient. The specification is also published as ISO/IEC 21031:2024.
