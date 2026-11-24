# Security

- **Prefix:** SEC
- **Kind:** area

This area covers how software is designed, built, tested and supported so that
it resists attack: secrets, input and output handling, authentication and
authorization, web applications and APIs, transport and cryptography, security
testing, and vulnerability handling. Its rules count toward Safety by default.
Supply chain and dependency rules are in their own areas.

The OWASP ASVS rules (USS-SEC-019 to USS-SEC-021) each require a whole ASVS
assessment. A gap that another rule of this area also covers, such as a query
built by concatenation, therefore fails both that rule and the ASVS rule.

## Design and secrets

### USS-SEC-001: Keep a threat model

- **Level:** MUST
- **Applies when:** uses-network or handles-sensitive-data or stores-user-data
- **Category:** Safety
- **Verify:** A threat model in the repository or the project's documentation
  names the assets, the trust boundaries, the threats and the control for each
  threat, and matches the current architecture: no component it names is
  missing from the code, and no major component of the code is missing from
  it. Length and format are free: a short document passes when it names
  those elements, for example by answering the four questions of the Threat
  Modeling Manifesto for each trust boundary.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.1.1; OWASP Threat Modeling
  Cheat Sheet (read 2026-11-23); Threat Modeling Manifesto (read 2026-11-24)

A written threat model MUST cover the current architecture: the assets, the
trust boundaries, the threats and the controls for each.

**Rationale:** Controls chosen without a model of the threats tend to protect
what is easy to protect. A model that tracks the architecture shows where the
risk is, and a stale model hides it.

### USS-SEC-002: Secure by default

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** In the default configuration, every security feature is on. Look
  at default values in code, configuration files and installers, and at code
  that changes a security setting without a user or operator asking for it.
- **References:** CISA, Shifting the Balance of Cybersecurity Risk: Principles
  and Approaches for Secure by Design Software (revised 2023-10-25), Secure by
  Default

Every security feature of the software MUST be on in the default
configuration. The default configuration is the one that applies until a user
or operator sets something on purpose, so the software never turns a security
feature off by itself. Documenting the settings that turn a feature off is a
Documentation (DOC) rule (USS-DOC-004).

**Rationale:** Users seldom change defaults, so the default decides the
security of most installations.

### USS-SEC-003: Run without elevated privileges

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Verify:** Run instructions, service definitions, container files and
  installers do not require administrator or root rights for normal operation.
  Each operation that does need them is documented with its reason.
- **References:** CWE-250

The software MUST NOT need administrator or root rights to run, except for
documented operations that need them.

**Rationale:** A flaw in a process with elevated rights gives an attacker those
rights.

### USS-SEC-004: Keep secrets out of the repository

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Verify:** A secret scan of the working tree and the full history finds no
  credential, key or token. The only exceptions are clearly fake example and
  test values, and credentials that the scanner's configuration suppresses with
  a reason that records their revocation.
- **References:** CWE-798

The project MUST NOT commit any secret to the repository or its history.

**Rationale:** Anyone who can read the repository, now or through a fork or
clone, can read every secret in its history. Removing a secret from the latest
commit does not remove it from the history. A revoked credential grants no
access, so it is no longer a secret.

### USS-SEC-005: Scan for secrets in CI

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The CI configuration runs a secret scanner on each pull request
  and each push to the default branch, and a finding fails that CI check. Each
  suppression has a recorded reason.
- **References:** OWASP Secrets Management Cheat Sheet (read 2026-11-23),
  8 Detection; OpenSSF Concise Guide for Developing More Secure Software
  (2023-06-14), item 9

CI MUST run a secret scanner on every change, and a finding MUST fail that CI
check until the secret is removed or the finding is suppressed with a reason.
Every change means each pull request and each push to the default branch.
Suppressions follow the check-integrity rules USS-QLT-006 to USS-QLT-014.

**Rationale:** A failing check blocks the merge (USS-SUP-008), so a secret
caught by the scan never reaches the history, and it does not have to be
revoked.

### USS-SEC-006: Load secrets at run time

- **Level:** MUST
- **Applies when:** uses-secrets
- **Category:** Safety
- **Verify:** Built artifacts and container images contain no secret values.
  The software reads each secret from the environment, a secret store or a
  protected file that is not part of the build.
- **References:** CWE-798; OWASP Secrets Management Cheat Sheet (read
  2026-11-23)

Secrets MUST come from the environment, a secret store or a protected file
outside the build, and never from built artifacts or container images.
USS-SEC-004 covers secrets in the repository.

**Rationale:** A secret built into an artifact travels with every copy of the
artifact and cannot be rotated without a new build.

### USS-SEC-007: Scope each credential

- **Level:** MUST
- **Applies when:** uses-secrets
- **Category:** Safety
- **Verify:** Each permission that the repository grants to a credential the
  software uses (API keys, tokens, service accounts, database users), in
  configuration, infrastructure code or deployment manifests, matches what the
  software calls. No such credential has administrator or wildcard rights
  without a documented need. Where the repository grants no permissions,
  nothing fails.
- **References:** NIST SP 800-53 Rev. 5 (Release 5.2.0) AC-6 (Least
  Privilege)

Each permission that the repository grants to a credential the software uses
MUST be one that the software needs. The repository grants a permission when
its configuration, infrastructure code or deployment manifests (such as RBAC
roles or Helm charts) set it. USS-SEC-008 covers documenting the permissions
of each credential.

**Rationale:** When a credential leaks, its permissions set the damage.

### USS-SEC-008: Document the permissions of each credential

- **Level:** MUST
- **Applies when:** uses-secrets
- **Category:** Safety
- **Verify:** The documentation names, for each credential the software needs,
  the permissions or scopes that it needs (for example "a token with
  `contents: read` on the repository"), and the named set is no larger than
  what the software calls. A credential with no such entry is a failure.
- **References:** NIST SP 800-53 Rev. 5 (Release 5.2.0) AC-6 (Least
  Privilege)

The documentation MUST name, for each credential the software needs, the
smallest set of permissions or scopes that the software works with.

**Rationale:** Operators create most of the credentials that software uses,
and they can grant only the permissions they know to be needed. Without this
list, the easy choice is a credential with every permission.

## Input and output

### USS-SEC-009: Validate untrusted input

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Each entry point that takes input from outside the trust boundary
  (requests, files, messages, command-line arguments, environment) validates
  it against an allow-list of the expected type, length, range and format
  before use.
- **References:** CWE-20; OWASP Input Validation Cheat Sheet (read
  2026-11-23)

Input from outside the trust boundary MUST be validated against an allow-list
of expected form before use.

**Rationale:** Input that is checked against what is expected is harder to turn
into an attack than input that is only checked against what is known to be bad.

### USS-SEC-010: Keep untrusted input out of shells

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Verify:** In the software's own code, no call to a shell or command
  interpreter includes untrusted input in its command string. A constant shell
  command that carries no untrusted input passes.
- **References:** CWE-78

The software's own code MUST NOT pass untrusted input to a shell or command
interpreter. It can keep such input out, for example, by starting programs
with an argument array and no shell. USS-SUP-012 covers CI scripts.

**Rationale:** A shell reads its input as code, so any input it receives can
become a command.

### USS-SEC-011: Do not deserialize untrusted data into arbitrary types

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Verify:** Code that reads untrusted data uses a data-only format, or a
  deserializer restricted to an allow-list of types. No use of a format or
  library that can create arbitrary types or run code (for example native
  object serialization or unrestricted YAML loaders) touches untrusted data.
- **References:** CWE-502; OWASP Deserialization Cheat Sheet (read
  2026-11-23)

Untrusted data MUST NOT be deserialized with a format or library that can
create arbitrary types or run code.

**Rationale:** Such deserializers let the data choose what code runs, which
gives the sender control of the process.

### USS-SEC-012: Fail closed

- **Level:** MUST
- **Applies when:** uses-network or handles-authentication or uses-cryptography
- **Category:** Safety
- **Verify:** Each security check (authentication, authorization, signature and
  certificate validation, policy lookups) denies access on an error, a timeout
  or a missing value. Look at error and exception paths, and at defaults for
  values that cannot be read.
- **References:** CWE-636

When a security check cannot complete, access MUST be denied. This covers an
error, a timeout and a missing value.

**Rationale:** A check that allows access when it fails can be defeated by
making it fail.

### USS-SEC-013: Use parameterized queries

- **Level:** MUST
- **Applies when:** uses-database
- **Category:** Safety
- **Verify:** No query is built by concatenating external input; all queries
  use bound parameters, or a query builder that binds them.
- **References:** OWASP ASVS 5.0 (injection prevention); CWE-89

Every database query that includes externally supplied values MUST pass those
values as bound parameters.

**Rationale:** The database reads a bound parameter only as a value, never as
SQL, so external input cannot change what a query does.

### USS-SEC-014: Encode output for its context

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** Where data is written into HTML, a URL, a log, a terminal or
  another interpreter or format, other than a database query or a shell
  command, the code encodes or escapes it for that context, through a library
  or the framework's automatic escaping.
- **References:** CWE-116; CWE-117

Data written into another interpreter or format (HTML, URLs, logs, terminals)
MUST be encoded or escaped for that context. Database queries and shell
commands are outside this rule: USS-SEC-013 and USS-SEC-010 cover them.

**Rationale:** Each interpreter has its own special characters. Data that is
safe in one context can change the meaning of the text around it in another.

## Authentication and authorization

### USS-SEC-015: Use standard authentication

- **Level:** MUST
- **Applies when:** handles-authentication
- **Category:** Safety
- **Verify:** Authentication code uses an established method (such as OIDC,
  SAML 2.0, WebAuthn, mutual TLS, or password verification as NIST SP
  800-63B-4 describes it) through a maintained library, framework or identity
  provider. No method of the project's own for verifying identity, such as a
  home-made token format or password hashing scheme, is present. A library or
  identity provider passes by implementing an established method.
- **References:** CWE-287; NIST SP 800-63B-4; OWASP ASVS 5.0 (V6.2 Password
  Security)

Authentication MUST use an established method, such as OIDC, SAML 2.0,
WebAuthn, mutual TLS or password verification as NIST SP 800-63B-4 describes
it, through a maintained library, framework or identity provider, and never a
method of the project's own. A library or identity provider that implements
an established method meets this rule by implementing it.

**Rationale:** Established methods and their libraries have been reviewed and
attacked for years; a new method has not.

### USS-SEC-016: No default credentials

- **Level:** MUST NOT
- **Applies when:** handles-authentication
- **Category:** Safety
- **Verify:** Source, configuration, images and installers contain no default
  or shared password, key or token. Unique credentials pass whether the
  operator sets them on first run or each installation generates its own.
- **References:** CISA, Shifting the Balance of Cybersecurity Risk (revised
  2023-10-25), Secure by Default Tactics (eliminate default passwords); CISA
  Secure by Design Pledge (read 2026-11-23), default passwords goal; CWE-1392;
  CWE-798

The software MUST NOT ship with a default or shared password, key or token.
Instead, for example, the operator sets unique credentials on first run, or
each installation generates its own.

**Rationale:** Default credentials are public once one copy is examined, so
every installation that keeps them is open to attack.

### USS-SEC-017: Offer multi-factor authentication

- **Level:** MUST
- **Applies when:** handles-authentication and has-ui
- **Category:** Safety
- **Verify:** Each interactive sign-in path supports multi-factor
  authentication, either in the software itself or through the identity
  provider it delegates to. Documentation or configuration shows how an account
  enables it.
- **References:** CISA, Shifting the Balance of Cybersecurity Risk (revised
  2023-10-25), Secure by Default Tactics (multifactor authentication); CISA
  Secure by Design Pledge (read 2026-11-23), multi-factor authentication goal;
  NIST SP 800-63B-4

Every account that signs in interactively MUST be able to use multi-factor
authentication, directly or through the identity provider.

**Rationale:** A second factor limits the harm of a stolen or guessed password.

### USS-SEC-018: Check authorization on every access

- **Level:** MUST
- **Applies when:** handles-authentication
- **Category:** Safety
- **Verify:** Every route, handler and operation that reaches a protected
  resource performs a permission check, in server-side or other
  caller-independent code. The default for anything without an explicit rule is
  denial. A test or review of an unprivileged caller shows access refused.
- **References:** CWE-862; CWE-863; OWASP Authorization Cheat Sheet (read
  2026-11-23); NIST SP 800-53 Rev. 5 (Release 5.2.0) AC-3 (Access
  Enforcement)

Access to a protected resource or operation MUST be granted only by an
explicit check of the caller's permissions, in a component the caller does
not control. Anything that no check allows is therefore denied.

**Rationale:** A check the caller can skip or alter is not a check, and an
operation that nobody thought to protect is open by default.

## Web applications and APIs

### USS-SEC-019: Meet OWASP ASVS Level 1

- **Level:** MUST
- **Applies when:** has-web-ui or exposes-web-api
- **Category:** Safety
- **Verify:** An assessment against OWASP ASVS 5.0 Level 1 (with evidence for
  each requirement, or a reason it does not apply to this software) shows every
  requirement met.
- **References:** OWASP ASVS 5.0

The software MUST meet every OWASP ASVS Level 1 requirement.

**Rationale:** ASVS gives a tested list of what web applications and APIs need;
Level 1 is the baseline for any such software.

### USS-SEC-020: Meet OWASP ASVS Level 2

- **Level:** MUST
- **Applies when:** (has-web-ui or exposes-web-api) and
  (handles-authentication or handles-sensitive-data)
- **Category:** Safety
- **Verify:** An assessment against OWASP ASVS 5.0 Level 2 (with evidence for
  each requirement, or a reason it does not apply to this software) shows every
  requirement met.
- **References:** OWASP ASVS 5.0

The software MUST meet every OWASP ASVS Level 2 requirement.

**Rationale:** ASVS 5.0 says that most applications should strive to reach
Level 2. This standard makes Level 2 a requirement where the software signs
users in or handles sensitive data.

### USS-SEC-021: Meet OWASP ASVS Level 3

- **Level:** SHOULD
- **Applies when:** (has-web-ui or exposes-web-api) and high-assurance
- **Category:** Safety
- **Verify:** An assessment against OWASP ASVS 5.0 Level 3 (with evidence for
  each requirement, or a reason it does not apply to this software) shows every
  requirement met.
- **References:** OWASP ASVS 5.0

The software SHOULD meet every OWASP ASVS Level 3 requirement.

**Rationale:** Level 3 suits software where a failure is very costly. The
effort is high, so teams with good reasons can skip it with a recorded
deviation.

## Transport and cryptography

### USS-SEC-022: Encrypt network traffic

- **Level:** MUST
- **Applies when:** uses-network
- **Category:** Safety
- **Verify:** Code and configuration for every connection to another host
  that carries the software's data, credentials or code, including connections
  inside a private network, use TLS 1.2 or later, or another protocol with
  equal protection such as SSH. A TLS configuration scan, or a review of client
  and server settings, shows no older protocol version enabled. Connections
  that carry none of these are outside this check, for example DNS lookups,
  NTP time requests, OCSP and CRL requests (whose responses are signed) and a
  plain HTTP listener that only redirects to HTTPS.
- **References:** NIST SP 800-52 Rev. 2; RFC 9325 (BCP 195); RFC 8446

Every connection to another host that carries the software's data,
credentials or code MUST use TLS 1.2 or later, or another protocol with equal
protection such as SSH. Connections inside a private network are included.

**Rationale:** Unencrypted traffic can be read and changed by anyone on the
path.

### USS-SEC-023: Keep certificate validation on

- **Level:** MUST NOT
- **Applies when:** uses-network
- **Category:** Safety
- **Verify:** The code and configuration, and each test setup that reaches a
  real host, contain no setting that disables certificate or host-name
  validation (such as a verifier that accepts everything or a blanket trust
  setting). The one exception is a user opt-out, such as an `--insecure` flag,
  that is off by default, applies only to the run that sets it and prints a
  warning when used, in software that also lets the user trust one specific
  certificate or certificate authority. A test setup that reaches a real host
  does not use the opt-out. Trusting one self-signed certificate does not turn
  validation off. A test that uses only local fakes is outside this check.
- **References:** CWE-295

Certificate and host-name validation MUST NOT be turned off, except through a
user opt-out that is off by default, applies only to the run that sets it and
warns when used, in software that also lets the user trust one specific
certificate or certificate authority. Tests that reach real hosts get no
exception.

**Rationale:** Without validation, encryption protects against nobody, because
a man in the middle can present any certificate. Code written for a test tends
to reach production. Users whose servers have certificates from a private
certificate authority need a way to connect. Trusting that one authority keeps
the rest of the validation, and an opt-out that lasts one run and prints a
warning is hard to leave on by accident.

### USS-SEC-024: Use vetted cryptography

- **Level:** MUST NOT
- **Applies when:** uses-cryptography and not implements-cryptography
- **Category:** Safety
- **Verify:** The code contains no implementation of a cryptographic primitive
  or protocol (ciphers, hashes, signature schemes, key exchange, random number
  generators). All cryptography calls a maintained, widely reviewed library.
- **References:** CWE-327

The project MUST NOT implement its own cryptographic primitives or protocols,
or use any that do not come from a maintained, widely reviewed library. A
project whose main purpose is to provide cryptography to other software
(`implements-cryptography`) is outside this rule; USS-SEC-025 covers it.

**Rationale:** Cryptography fails through small implementation errors that only
long public review finds.

### USS-SEC-025: Test cryptography against published test vectors

- **Level:** SHOULD
- **Applies when:** implements-cryptography
- **Category:** Safety
- **Verify:** CI runs, for each algorithm and protocol that the project
  implements and for which published test vectors exist, those vectors (such
  as Project Wycheproof's, or those linked from the NIST CAVP algorithm
  pages), and a mismatch fails the CI check.
- **References:** Project Wycheproof (read 2026-11-24); NIST Cryptographic
  Algorithm Validation Program (CAVP) (read 2026-11-24)

A project that implements cryptography SHOULD test each implementation in CI
against the published test vectors for it.

**Rationale:** Test vectors catch the errors that break cryptography without
breaking ordinary tests: output that looks random but is wrong, and inputs
crafted to exploit known attacks, such as invalid curve points.

### USS-SEC-026: No disallowed algorithms

- **Level:** MUST NOT
- **Applies when:** uses-cryptography
- **Category:** Safety
- **Verify:** Code and configuration are checked against NIST SP 800-131A
  Rev. 2 and OWASP ASVS 5.0 Appendix C: no algorithm or key size that either
  disallows for the use appears. For example, a search finds no SHA-1 for
  digital signature generation, no RSA key shorter than 2048 bits for digital
  signature generation, and no MD5, RC4, DES, 3DES or Blowfish for a security
  purpose.
- **References:** NIST SP 800-131A Rev. 2; OWASP ASVS 5.0, Appendix C
  (Cryptography Standards); CWE-327; CWE-328

The project MUST NOT use, for a security purpose, any algorithm or key size
that NIST SP 800-131A disallows or that OWASP ASVS 5.0 Appendix C marks
Disallowed.

**Rationale:** A disallowed algorithm no longer gives the protection it was
chosen for.

### USS-SEC-027: Use FIPS 140-3 validated modules

- **Level:** MUST
- **Applies when:** requires-fips
- **Category:** Safety
- **Verify:** Each cryptographic module the software uses is listed in the NIST
  CMVP validated modules list under FIPS 140-3, is run in its approved mode
  (shown by configuration or build settings), and its certificate is current.
- **References:** FIPS 140-3; NIST CMVP (read 2026-11-23)

All cryptography MUST run in modules validated under FIPS 140-3, in their
approved mode.

**Rationale:** Where a regulation or contract requires FIPS, only validated
modules in approved mode meet it.

## Security testing

### USS-SEC-028: Run static security analysis in CI

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The CI configuration runs a static application security testing
  (SAST) tool on each pull request and each push to the default branch, and a
  finding fails that CI check. Each suppression has a recorded reason.
- **References:** NIST SP 800-218 (SSDF 1.1) PW.7.2

CI MUST run a static application security testing (SAST) tool on every change,
and a finding MUST fail that CI check until it is fixed or suppressed with a
reason. Every change means each pull request and each push to the default
branch. Suppressions follow the check-integrity rules USS-QLT-006 to
USS-QLT-014.

**Rationale:** Static analysis finds many classes of flaw before they ship, and
a finding that does not fail the check is easy to ignore.

## Vulnerability handling

### USS-SEC-029: Publish a vulnerability disclosure policy

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Verify:** The repository holds a security policy (such as `SECURITY.md`)
  that says how to report a vulnerability privately, and what happens after a
  report.
- **References:** ISO/IEC 29147:2018; CISA et al., Establishing a Coordinated
  Vulnerability Disclosure Program to Work With Security Researchers
  (2026-07-15); CISA Secure by Design Pledge (read 2026-11-23), vulnerability
  disclosure policy goal

The repository MUST have a security policy (such as `SECURITY.md`) that says how
to report a vulnerability privately and what happens next.

**Rationale:** Researchers who find a flaw and see no way to report it privately
often disclose it in public.

### USS-SEC-030: State response times

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Safety
- **Verify:** The security policy states a target time to acknowledge a report,
  a target time to assess it and a target time to release a fix.
- **References:** ISO/IEC 30111:2019

The security policy SHOULD state target times to acknowledge a report, assess
it and release a fix.

**Rationale:** Reporters and users can plan around stated times, and the
project can be held to them.

### USS-SEC-031: Publish security advisories

- **Level:** MUST
- **Applies when:** distributed-as-package or ships-container-image
- **Category:** Safety
- **Verify:** For each fixed vulnerability in a released version, a published
  advisory (such as a repository security advisory, a database entry or a
  release note) names the affected and fixed versions and a CVE or other
  public ID.
- **References:** OSV schema 1.9.2; CSAF 2.0; CVE Program, CVE JSON Record
  Format 5.2.0; CISA Secure by Design Pledge (read 2026-11-23),
  vulnerability reporting transparency goal; Regulation (EU) 2024/2847 (Cyber
  Resilience Act), Annex I Part II(4)

Each fixed vulnerability in a released version MUST get a published advisory
that names the affected and fixed versions and a CVE or other public ID.

**Rationale:** Users and their tools learn that an update matters only if the
fix is announced where they look.

### USS-SEC-032: Publish VEX statements

- **Level:** SHOULD
- **Applies when:** distributed-as-package or ships-container-image
- **Category:** Safety
- **Verify:** For each release, any known vulnerability in a dependency that the
  release is not affected by has a VEX statement (in a format such as OpenVEX,
  CSAF VEX or CycloneDX VEX) published with that release, giving the status
  and a justification.
- **References:** CISA Minimum Requirements for Vulnerability Exploitability
  eXchange (VEX) (2023-04-21); OpenVEX 0.2.0, CSAF 2.0 (4.5 Profile 5: VEX) or
  CycloneDX 1.7 VEX

When a known vulnerability in a dependency does not affect a release, the
project SHOULD publish a VEX statement for it with that release.

**Rationale:** Without a statement, each user of the release has to investigate
the same scanner alerts again.

### USS-SEC-033: Serve security.txt

- **Level:** SHOULD
- **Applies when:** operates-service and (has-web-ui or exposes-web-api)
- **Category:** Safety
- **Verify:** Each web service that the project operates returns a valid
  security.txt at `/.well-known/security.txt` over HTTPS, with the fields that
  RFC 9116 requires.
- **References:** RFC 9116

Each web service that the project operates SHOULD serve
`/.well-known/security.txt`.

**Rationale:** The file lets researchers find the reporting channel for a
running service without searching.
