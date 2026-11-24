# Operations

- **Prefix:** OPS
- **Kind:** area
- **Applies when:** operates-service

This area covers how the project runs its own production deployment: the
objectives that the service is held to, the alerts and runbooks that keep it
there, and what the team does and says when it fails. Its rules count toward
Reliability. The Observability (OBS) area holds the metrics that the symptom
alerts read (USS-OBS-004) and the Backup (BAK) area holds the alert on failed
backups (USS-BAK-010). The Documentation (DOC) area holds the style of
instructions, which the runbooks follow (USS-DOC-007). The incident response
here is operational: restoring the service. Handling a reported vulnerability
and its advisory belong to the Security (SEC) area (USS-SEC-029 to
USS-SEC-031).

### USS-OPS-001: Set service-level objectives

- **Level:** MUST
- **Applies when:** always
- **Category:** Reliability
- **Verify:** List each user-facing service from the deployment configuration
  and the documentation (a service that people or other programs outside the
  team call or open). For each, the documentation gives at least one
  service-level objective: the measure (such as the share of requests that
  succeed, or the share served within a time), a target value, and the time
  window. It also gives an error-budget policy for the service, which names at
  least one action that is taken when the budget is spent (such as pausing
  feature releases, or giving reliability work priority) and who takes it. A
  user-facing service with no objective, or with no policy, fails.
- **References:** Google, Site Reliability Engineering (2016), chapter 4;
  Google, The Site Reliability Workbook (2018), chapter 2, "Error Budget
  Policy"

Each user-facing service MUST have documented service-level objectives, with an
error-budget policy that says what happens when the budget is spent.

**Rationale:** Without a target, "reliable enough" is an argument that the
loudest voice wins. An objective sets how much failure is acceptable, and the
policy turns a spent budget into a decision that was made in advance.

### USS-OPS-002: Alert on symptoms

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Reliability
- **Verify:** List the paging alerts in the monitoring configuration: the
  rules that wake or interrupt a person (routed to a pager, a phone call or an
  on-call rotation). Read the condition of each. It fires on a symptom that
  users feel: the burn rate of an error budget, the error rate, the latency,
  or the availability of a user-facing service, measured at the point where
  users reach the service. A paging alert whose condition is only a cause
  (such as processor use, memory use, disk use or a restart count) fails. An
  alert that goes to a ticket queue or a chat channel is not a paging alert.
- **References:** Google, Site Reliability Engineering (2016), chapter 6,
  "Symptoms Versus Causes"; Google, The Site Reliability Workbook (2018),
  chapter 5

Paging alerts SHOULD fire on symptoms that users feel (such as the
error-budget burn rate, the error rate or latency), not on causes alone.

**Rationale:** A cause does not always hurt users, and a harm to users does not
always have a cause that was foreseen. A symptom alert fires for both. Cause
alerts remain useful as tickets and as aids to diagnosis. The service metrics
(USS-OBS-004) hold the inputs.

### USS-OPS-003: Link a runbook from every alert

- **Level:** MUST
- **Applies when:** always
- **Category:** Reliability
- **Verify:** List every alert rule in the monitoring configuration, paging or
  not. Each has a link to a runbook (such as a runbook field, a label or an
  annotation of the rule), and the link opens a document in a place that the
  on-call people can reach. The document says how to diagnose the alert (what
  to look at and what each finding means) and how to resolve it (the steps, or
  who to call when the steps end). An alert with no link, a broken link, or a
  document that only repeats the alert's name fails.
- **References:** Google, Site Reliability Engineering (2016), chapter 1,
  "Emergency Response" (the playbook)

Every alert MUST link to a runbook that says how to diagnose and resolve it.

**Rationale:** The person who is paged at night may never have seen this fault.
A recorded procedure shortens the repair, as the SRE book reports, and it
leaves the knowledge in the project and not in one person.

### USS-OPS-004: Document incident response

- **Level:** MUST
- **Applies when:** always
- **Category:** Reliability
- **Verify:** Open the incident-response document. It names the roles of an
  incident by title (such as incident commander, operations lead and
  communications lead) and says what each does. It defines at least two
  severity levels, each with a criterion that decides the level from the
  effect on users or data. It says how to escalate: who is called next and
  when, as a trigger such as a time without progress or a severity level. It
  says how to communicate: who tells whom, and through which channel, inside
  the team and to users. A document that lacks one of the four fails.
- **References:** Google, Site Reliability Engineering (2016), chapter 14;
  Google, The Site Reliability Workbook (2018), chapter 9; NIST SP 800-61
  Rev. 3 (April 2025), section 2.2 and RS.MA-04

A documented incident-response process MUST name the roles and the severity
levels, and say how to escalate and how to communicate.

**Rationale:** In an incident, nobody has time to decide who decides. Roles
and levels agreed in advance spare that time, and they stop two people from
changing the system at once.

### USS-OPS-005: Hold blameless postmortems

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Reliability
- **Verify:** List the major incidents of the last 12 months, or since the last
  audit when that is shorter. A major incident is one that the project's
  incident-response document ranks at its highest severity or its highest
  but one; where there is no such document, one in which users lost the
  service or lost data. Each has a postmortem that gives the timeline, the
  impact, the causes and the follow-up actions. The causes are about
  systems and processes: the text blames no person or team by name. Each
  follow-up action is an entry in the project's tracker, with an owner and a
  status that the tracker updates until the action is closed. A major incident
  with no postmortem, an action that is not in the tracker, or a postmortem
  that blames a person fails.
- **References:** Google, Site Reliability Engineering (2016), chapter 15

Each major incident SHOULD get a blameless postmortem, and its follow-up
actions SHOULD be tracked to completion.

**Rationale:** People who fear blame hide what happened, and the fault stays.
A postmortem is worth the follow-up that it gets: an action that nobody tracks
is a lesson that is learned again at the next incident.

### USS-OPS-006: Roll out changes progressively

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Reliability
- **Verify:** Read the deployment pipeline or the rollout configuration for
  production. A change reaches a part of production first (such as a canary
  instance, a percentage of traffic or a first ring of users), is evaluated
  there, and only then reaches the rest. The configuration has a health or
  SLO check (such as an error-rate or latency comparison with the part that
  has the old version) that, when it fails, starts a rollback with no manual
  step. In the deployment history of the last 12 months or since the last
  audit, open at least three deployments (all, if fewer): each shows the
  stages. A pipeline that sends each change to all of production at once, or
  one that a person must roll back, fails.
- **References:** Google, The Site Reliability Workbook (2018), chapter 16,
  "Canarying Releases"

Production changes SHOULD roll out in stages (such as canary releases), and
SHOULD roll back automatically when health or SLO signals fail.

**Rationale:** Most outages follow a change. A change that reaches a few users
first costs a few users when it is wrong, and an automatic rollback ends the
harm at once without waiting for a person to see it.

### USS-OPS-007: Tell users about incidents

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Reliability
- **Verify:** The documentation gives the address of a status page or
  channel (such as a status site, a feed or a pinned channel) that users can
  read. For each incident of the
  last 12 months, or since the last audit when that is shorter, in which
  users felt the failure (the service was down, slow or wrong for them), the
  page or channel has an entry that names what was affected and gives the
  time of its latest update. Where there was no such incident, the page or
  channel exists and its entries have both fields. An incident that users felt
  and that has no entry fails.
- **References:** NIST SP 800-61 Rev. 3 (April 2025), RC.CO-04; Google, The
  Site Reliability Workbook (2018), chapter 9

During an incident that users feel, a status page or channel SHOULD tell them
what is affected and show when it was last updated.

**Rationale:** Silence reads as neglect, and users who do not know what is
broken flood the support desk. An update time tells a reader whether the page
is still true.
