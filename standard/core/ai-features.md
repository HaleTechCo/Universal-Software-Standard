# AI features

- **Prefix:** AI
- **Kind:** area
- **Applies when:** uses-ai-models or has-ai-tool-use

This area covers software that calls or runs AI models, and software that lets
a model take actions or offers tools to a model. Its rules count toward Safety,
except the evaluation rule, which counts toward Reliability. The Security (SEC)
area holds the general defences against untrusted input (USS-SEC-009,
USS-SEC-010, USS-SEC-013 and USS-SEC-014), and the first rule here says that
model output is such input. The Security area also holds the scoping of each
credential (USS-SEC-007) and standard authentication (USS-SEC-015). The Privacy
(PRV) area holds the personal-data inventory (USS-PRV-001). The Usability (UX)
area holds the confirmation of destructive actions that a user starts
(USS-UX-002); the rule on confirmation here is for actions that a model starts.

### USS-AI-001: Treat model output as untrusted input

- **Level:** MUST
- **Applies when:** uses-ai-models
- **Category:** Safety
- **Verify:** List each place where the software passes model output on: into
  a database query, a shell command or process call, a file path, a page or
  document that a browser renders, a URL that it fetches, another service's
  request, or the arguments of a tool call. For each place, read the code: the
  output is checked or encoded in the way that the matching general rule
  requires (USS-SEC-009 for input, USS-SEC-010 for shells, USS-SEC-013 for
  queries, USS-SEC-014 for output to a page) before the software uses it, and
  the code path has no step that passes the raw output on. Then, in a test
  instance, make the model (or a stub in its place) return a string built to
  break each of those: a quote in a query value, a shell metacharacter, a
  script tag, a path with `..`. None of them takes effect. A place that uses
  raw model output fails.
- **References:** OWASP Top 10 for LLM Applications 2025, LLM05

Model output MUST be validated or encoded before the software uses it, such as
in a query, a command, a page or a tool call, as it would be for any untrusted
input.

**Rationale:** A model repeats and transforms what it reads, so an attacker who
shapes its input shapes its output. Output that goes straight into an
interpreter is then an injection path, however trusted the model is. The general
rules for untrusted input already say how to check and encode; this rule only
says that model output is in their scope. It therefore overlaps USS-SEC-009,
USS-SEC-010, USS-SEC-013 and USS-SEC-014 where model output reaches them, and
the overlap is accepted, so that the scope is stated once for model output.

### USS-AI-002: Document prompt-injection defences

- **Level:** MUST
- **Applies when:** uses-ai-models
- **Category:** Safety
- **Verify:** The design documentation lists each path by which content that
  the project does not control reaches a model. Find the paths in the code:
  user messages, uploaded files, retrieved documents or web pages, email,
  results of tools (including those of other servers), and data from other
  systems. Each path that the code has is in the list. For each path, the
  document names the controls that limit what an injected instruction in that
  content can do (such as the permissions of the tools in reach, confirmation
  of actions, separation of the content from the instructions, filtering of
  the output, or a design in which the model has no tools and its output is
  only shown), and says what the control limits. A path that the code has and
  the document leaves out, or a path with no control named, fails.
- **References:** OWASP Top 10 for LLM Applications 2025, LLM01; NIST AI
  600-1 (July 2024), 2.9 Information Security

For each path by which content that the project does not control reaches a
model, the design MUST document the controls that limit what an injected
instruction can do.

**Rationale:** A model cannot tell the instructions of its owner from
instructions hidden in the text it reads, and no known filter removes the
risk. What a project can do is limit the harm, and a limit that nobody wrote
down is not checked when the software changes.

### USS-AI-003: Give models only the tools they need

- **Level:** MUST
- **Applies when:** has-ai-tool-use
- **Category:** Safety
- **Verify:** List each tool that a model can use (the functions, commands,
  APIs and MCP tools that the software offers or calls for the model). For
  each tool, list the permissions that its credential or process holds (such
  as OAuth scopes, database grants, file paths, network reach, or operating
  system rights) and the operations that its code performs. A permission that
  none of the tool's operations uses fails. Then, for a tool that acts for a
  user, read how the call is authorized: it runs with the user's own
  credential, or the code checks the user's rights before it acts. In a test
  instance, ask for an operation that the test user cannot do directly, through
  the model: the tool refuses it. A tool that holds a permission that it does
  not use, or can do what its user cannot, fails.
- **References:** OWASP Top 10 for LLM Applications 2025, LLM06; MCP
  Specification 2026-07-28, Security Best Practices, Scope Minimization

Each tool that a model can use MUST grant only the permissions that its task
needs, and never more than those of the user it acts for.

**Rationale:** A model's choices can be steered by the content it reads, so a
tool is as powerful in an attacker's hands as it is in the model's. A tool
with the rights of an administrator turns a prompt injection into an
administrator's action. The general scoping of credentials (USS-SEC-007) still
applies to each credential; this rule adds the ceiling of the user's rights.

### USS-AI-004: Confirm destructive actions with a person

- **Level:** MUST
- **Applies when:** has-ai-tool-use
- **Category:** Safety
- **Verify:** List the actions that a model can start, and mark each one that
  deletes data, spends money, sends a message (to a person or to another system)
  or cannot be undone. For each marked action, read the code: it does not run
  until a person has confirmed it, and the confirmation names the action and its
  target. In a test instance, make the model start each marked action: it waits
  for a person, and does not run if the person declines or does not answer. An
  MCP server meets this in one of two ways. Either it relies on the confirmation
  that the MCP client asks of the person before a tool call (the documentation
  says so, and says that clients confirm tool calls, as the specification
  expects), or its tool sends an elicitation request that the person has to
  accept before the action runs (MCP Specification 2026-07-28, Elicitation). For
  the first route, the documentation also names the confirming clients that the
  server supports (or says that it serves only clients that confirm). Run the
  check with a confirming client where the person declines, or with the
  elicitation declined: the action does not run. An action that runs with no
  person's approval fails, and so does a server that sends no elicitation of its
  own and whose documentation names no confirming client and does not say that
  it serves only clients that confirm.
- **References:** OWASP Top 10 for LLM Applications 2025, LLM06; MCP
  Specification 2026-07-28, Server, Tools (User Interaction Model) and Client,
  Elicitation

An action that a model starts, and that deletes data, spends money, sends a
message or cannot be undone, MUST run only after a person confirms it.

**Rationale:** The person is the one check that an injected or mistaken model
cannot talk past. The MCP specification asks that a person be in the loop with
the ability to deny a tool call, and it leaves the confirmation to the client;
a server that cannot know its client then has to rely on it in the open, or
ask for itself.

### USS-AI-005: Pin model versions

- **Level:** MUST
- **Applies when:** uses-ai-models
- **Category:** Safety
- **Verify:** Find every model identifier in the code and the configuration
  (a model name in an API call, a setting, a deployment manifest, or a model
  file that the software loads). Each names an exact version: a dated or
  numbered snapshot that the provider's documentation lists as fixed, or, for
  a model that the project runs itself, a file or image pinned by a revision or
  a checksum. An identifier that is an alias (such as a name ending in
  `latest`, or a family name that the provider documentation says is moved to
  newer versions) fails.
- **References:** none

The software MUST name exact model versions, not aliases that the provider can
move, so that behaviour changes only with a deliberate update.

**Rationale:** The same prompt can give different answers on a new model
version. A moving alias changes the software's behaviour on the provider's
schedule, with no change in the project's history to explain it, and it breaks
the evaluation that the next rule asks for.

### USS-AI-006: Evaluate models like tests

- **Level:** MUST
- **Applies when:** uses-ai-models
- **Category:** Reliability
- **Verify:** The repository holds an evaluation suite, with test cases and a
  pass threshold for each metric (a score, a rate or a count). The CI
  configuration runs it for each change that touches a model identifier, a
  prompt, or a model setting (such as temperature or a token limit), and a
  result below a threshold fails the job. Take the changes of those kinds merged
  since the last audit, or the last 12 months when there is none: the CI run of
  each one ran the suite before it merged. In a test branch, worsen a prompt
  until a case fails: the job fails. That a failed job blocks the merge is
  judged by USS-SUP-008, not here. A change of those kinds that merged with no
  suite run, a suite with no threshold, or a regression that does not fail the
  job, fails.
- **References:** NIST AI 100-1 (AI RMF 1.0, January 2023), 5.3 Measure

An evaluation suite with pass thresholds MUST run in CI before each change of
model, prompt or model settings merges, and a regression MUST fail the check.

**Rationale:** A model's output cannot be checked by reading the code, so the
change of a prompt is a change of behaviour that only a run can show. A suite
that runs for each such change catches a regression while it is one commit
large.

### USS-AI-007: Limit cost, rate and tokens

- **Level:** MUST
- **Applies when:** uses-ai-models
- **Category:** Safety
- **Verify:** Find in the code or the configuration each of three limits: a cap
  on the tokens of each request (input and output), a cap on the requests per
  user in a period, and a cap on the total spend in a period (in the software or
  in the provider's account settings). A model that does not work in tokens
  (such as an image classifier) needs no token cap, and a model whose calls cost
  nothing (such as one that runs on the project's or the user's own hardware)
  needs no spend cap; the cap on requests applies to every model. Each limit
  that applies has a value. In a test instance, send a request over the token
  cap, then requests over the rate cap: each is refused or cut short. Find the
  alert for a reached limit (such as a notification or a metric with an alert
  rule), and trigger it in the test instance. A limit that applies and is
  missing, or a limit with no alert, fails. Account settings that the auditor
  cannot open are `unverified`.
- **References:** OWASP Top 10 for LLM Applications 2025, LLM10

The software MUST limit the tokens per request and the total spend, where the
model has tokens or a cost per call, and the requests per user, and MUST alert
when a limit is reached.

**Rationale:** Model calls cost money for each token, and an attacker, a loop
or a popular day can use a month's budget in an hour. The limits keep a
failure small, and the alert tells a person before the budget is gone. The
bounds on queues and pools that the Resilience area asks for (USS-RES-004) are
a separate protection of the software's own memory.

### USS-AI-008: No sensitive data to models without a basis

- **Level:** MUST NOT
- **Applies when:** uses-ai-models and handles-sensitive-data
- **Category:** Safety
- **Verify:** Find each flow of data to a model (prompts, retrieved content,
  attached files, and tool results that go back to the model), and mark the
  flows that can carry sensitive data (personal, financial, health or
  credential data, as the answer to the handles-sensitive-data fact defines
  it). For each marked flow, the design documentation records the purpose of
  sending the data and the basis for it. Where the data is personal data, the
  purpose and the basis are in the personal-data inventory (USS-PRV-001), in
  the entry for that data. The documentation also names the model provider's
  terms for the service (such as a data processing agreement, or the terms of
  an API plan) and what they say about the use of the data, and the terms
  allow the purpose. A flow with no recorded purpose and basis, or one that the
  terms do not allow, fails. Terms and account settings that the auditor cannot
  open are `unverified`.
- **References:** OWASP Top 10 for LLM Applications 2025, LLM02; NIST AI 600-1
  (July 2024), 2.4 Data Privacy

Sensitive data MUST NOT be sent to a model unless the design records the
purpose and the basis for it, and the model provider's terms allow it. Personal
data is recorded in the personal-data inventory (USS-PRV-001).

**Rationale:** Data that goes to a model can leave the project's control: the
provider can keep it, learn from it, or reveal it to another user. A purpose
and a basis, written before the data moves, are what let the project answer
for it later.

### USS-AI-009: Tell people when they deal with AI

- **Level:** MUST
- **Applies when:** uses-ai-models and has-ui
- **Category:** Safety
- **Verify:** List the screens, dialogs and messages through which a person
  interacts with an AI system (a chat, an assistant, an automated reply), and
  those that show content that a model generated (text, images, audio, video,
  or a summary). In a test instance, open each: it states in the interface that
  the person is dealing with an AI system, and the generated content carries a
  visible mark (such as a label or an icon with a text alternative). A
  screen that does either with no statement or mark fails.
- **References:** Regulation (EU) 2024/1689 (AI Act), Article 50(1) and 50(4)

The interface MUST tell people when they are interacting with an AI system, and
MUST mark content that AI generated.

**Rationale:** People judge a message differently when they know that a machine
wrote it. Article 50 of the AI Act requires the notice for systems that
interact with people and for deep fakes; this rule is wider than the law, and
a project that gives the notice and the mark everywhere does not have to work
out where the duty starts.

### USS-AI-010: Authenticate MCP clients

- **Level:** MUST
- **Applies when:** exposes-mcp-server and exposes-web-api
- **Category:** Safety
- **Verify:** Take each remote MCP server that the software provides (one that
  clients reach over HTTP). Read the MCP authorization specification's
  requirements for a server and check these in the code and in a test
  instance: the server publishes OAuth 2.0 Protected Resource Metadata
  (RFC 9728) that names its authorization server; a request with no token gets
  a 401 response with a `WWW-Authenticate` header; the server validates each
  token and accepts only a token that was issued for this server as the
  audience; it takes the token from the `Authorization` header and not from the
  query string; and it passes no client token on to other services. A request
  with a token for another audience is refused. A remote server that serves a
  request with no valid token fails.
- **References:** MCP Specification 2026-07-28, Authorization and Security
  Best Practices (Token Passthrough); RFC 9728

A remote MCP server MUST authenticate its clients as the MCP authorization
specification describes.

**Rationale:** A server that offers tools to any caller offers its actions to
any attacker who finds the address. The specification makes authorization
optional for MCP in general; this standard requires it for a server that is
reachable over HTTP, and takes the method from the specification, so that
clients and servers from different makers work together. Standard
authentication (USS-SEC-015) holds the general rule.
