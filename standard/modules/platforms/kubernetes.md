# Kubernetes and Helm

- **Prefix:** K8S
- **Kind:** module
- **Applies when:** platform.kubernetes or platform.helm

This module covers projects that ship Kubernetes manifests or Helm charts. The
Deployment hardening area holds the hardening fields of a workload
(USS-HRD-004 to USS-HRD-008); this module adds the checks that run on them, the
rules for Helm charts, and the Kubernetes form of two of them, the
service-account token and the network policy. Kubernetes and Helm are the
module's subject, so the module names their fields and commands directly. It
uses one vendor profile, `standard/vendors/stackrox/kube-linter.md`, and names
one tool without a profile, kubeconform, because it has no rule set. The rules
were checked against Kubernetes 1.37, Helm 4.3.2 (Helm 4 is the current major
version; its documentation pages are at 4.3.1, and some of them say that they
are not yet updated for Helm 4, so the behaviour that the rules rely on was
also run on Helm 4.3.2), kube-linter 0.8.3 and kubeconform 0.8.0, the current
releases on 2026-11-24. The core rules that this module does not refine apply
as written.

## Manifests

### USS-K8S-001: Lint rendered manifests for hardening

- **Level:** MUST
- **Applies when:** deploys-to-kubernetes
- **Category:** Safety
- **Refines:** USS-HRD-004, USS-HRD-005, USS-HRD-006, USS-HRD-008
- **Verify:** Read the CI configuration. For each chart that the project ships,
  a step renders it with its default values (`helm template`), and each manifest
  file that the project ships is read as it is, or rendered by the project's own
  tool (such as Kustomize) when it needs rendering. A policy linter then checks
  every rendered chart and manifest, with each of its checks that tests a
  requirement of USS-HRD-004, USS-HRD-005, USS-HRD-006 or USS-HRD-008 turned on
  and none of them excluded. The linter's documentation names its checks: the
  fields of the Restricted Pod Security Standard, the resource requests and
  limits, the probes and the service-account token mount. The job exits with a
  failure when the linter reports a violation, as the linter's documentation
  states for its exit status. A chart or manifest file that the linter does not
  read, or a linter that the job runs with failures tolerated, fails.
- **References:** Kubernetes documentation, Pod Security Standards (read
  2026-11-24), Restricted profile; kube-linter 0.8.3, documentation, Using
  KubeLinter and KubeLinter checks (read 2026-11-24)

CI MUST render every chart and manifest that the project ships and check the
result with a policy linter that covers the hardening rules, and a violation
MUST fail the build.

**Rationale:** A missing hardening field is easy to write and hard to see in a
review of a template. A linter that reads the rendered result finds it before
the manifest reaches a cluster, and a check that does not fail the build is
soon ignored. The linter finds field-level gaps only: each requirement of the
hardening rules is still judged as those rules say.

### USS-K8S-002: Lint with kube-linter

- **Level:** SHOULD
- **Applies when:** deploys-to-kubernetes
- **Category:** Safety
- **Refines:** USS-HRD-004, USS-HRD-005, USS-HRD-006, USS-HRD-008
- **Verify:** CI runs `kube-linter lint` on the project's rendered manifests,
  or on a chart directory (which kube-linter renders itself, with the values
  of the chart's `values.yaml`), with a configuration that includes the
  baseline of the profile `standard/vendors/stackrox/kube-linter.md`:
  `checks.addAllBuiltIn` is
  `true` (in `.kube-linter.yaml` or `.kube-linter.yml`, or in the file that
  `--config` names, or as `--add-all-built-in`), and the configuration sets
  no `checks.ignorePaths` (or `--ignore-paths`) and excludes no check other
  than `required-label-owner` and `required-annotation-email` in
  `checks.exclude` (or `--exclude`). The job fails when `kube-linter lint`
  exits with status 1, which it does when it reports a lint error. A project
  that uses another linter fails this rule and records a Standard Deviation; it
  meets USS-K8S-001 through that linter's own profile.
- **References:** kube-linter 0.8.3, documentation, Configuring KubeLinter and
  KubeLinter checks (read 2026-11-24); kube-linter 0.8.3, source of the `lint`
  command and `main` (read 2026-11-24)

The project SHOULD lint rendered manifests with kube-linter, configured as its
vendor profile says.

**Rationale:** kube-linter reads Kubernetes objects directly and has built-in
checks for most of the hardening fields, so a project gets the checks without
writing policies of its own. The profile fixes the configuration that makes
"no violations" mean the same in every project.

### USS-K8S-003: Validate manifests against the API schemas

- **Level:** MUST
- **Applies when:** deploys-to-kubernetes
- **Category:** Reliability
- **Verify:** The versions that the project supports are those that its
  documentation names (a range names each minor version in it); a project that
  names none fails. CI validates the rendered charts and the manifest files of
  USS-K8S-001 against the API schemas of each of those versions, with one run
  for each version (for kubeconform, `-kubernetes-version` with that version).
  Each resource of a kind that the Kubernetes API defines is validated. A
  resource of a kind that the Kubernetes API does not define (a custom
  resource) has no schema in a Kubernetes release, so it can be skipped or
  validated against a schema that the project supplies. The job fails when the
  validator reports an invalid resource (kubeconform exits with status 1).
- **References:** kubeconform 0.8.0, Readme, usage and exit status (read
  2026-11-24)

CI MUST validate the rendered manifests against the API schemas of each
Kubernetes version that the project supports (such as with kubeconform), and an
invalid manifest MUST fail the build.

**Rationale:** A manifest with a misspelled field or a removed API version is
well-formed YAML, and a cluster can accept it and ignore the mistake or reject
it at deploy time. Validating against the schema of each supported version finds
the mistake in CI, and finds the API that a newer version has removed.

## Helm

### USS-K8S-004: Lint charts strictly

- **Level:** MUST
- **Applies when:** platform.helm and deploys-to-kubernetes
- **Category:** Maintainability
- **Refines:** USS-QLT-001
- **Verify:** CI runs `helm lint --strict` with the path of each chart that the
  project ships (Helm 4 requires the path), with or without further flags that
  set values. The job fails when the command exits with a non-zero status: Helm
  exits with status 1 when it reports an error, and with `--strict` a warning
  counts as a failure. A chart for which no such command runs, or a command
  whose failure the job tolerates, fails.
- **References:** Helm 4.3.1 documentation, `helm lint` (read 2026-11-24); Helm
  4.3.2, run of `helm lint` and `helm lint --strict` on a chart with a warning
  (2026-11-24)

CI MUST run `helm lint --strict` on each chart, and a finding MUST fail the
build.

**Rationale:** Without `--strict`, `helm lint` reports a convention that the
chart breaks as a warning and exits with status 0, so the warning goes unread.
The strict flag makes it a failure, which is the zero-violations target of the
core rule.

### USS-K8S-005: Ship a values schema

- **Level:** SHOULD
- **Applies when:** platform.helm and deploys-to-kubernetes
- **Category:** Maintainability
- **Verify:** Each chart that the project ships has a `values.schema.json` in
  its directory. Each object in the schema that lists its properties sets
  `additionalProperties` to `false`, and each property declares its type. An
  object whose keys the user chooses (labels, annotations or a node selector)
  has no fixed properties, so it gives `additionalProperties` a schema for its
  values instead. Then run `helm lint` on the chart, once with a key that the
  chart does not define (such as `--set unknownKey=1`) and once with a defined
  value of the wrong type (`--set` of a number key to a word): each run reports
  an error.
- **References:** Helm 4.3.1 documentation, Charts, Schema Files (read
  2026-11-24; the page is not yet updated for Helm 4); Helm 4.3.2, run of `helm
  lint` on a chart with `additionalProperties` false (2026-11-24)

Each chart SHOULD ship a `values.schema.json` that rejects unknown and
mistyped values.

**Rationale:** A misspelled key in a values file is valid YAML, so Helm ignores
it and the chart runs with the default that the user meant to change. The schema
turns the mistake into an error at `helm lint`, `helm template` and
`helm install`.

### USS-K8S-006: Lock chart dependencies

- **Level:** MUST
- **Applies when:** platform.helm and deploys-to-kubernetes and has-dependencies
- **Category:** Safety
- **Refines:** USS-DEP-001, USS-DEP-002
- **Verify:** In each chart that the project ships, each entry of
  `dependencies` in `Chart.yaml` has a `version` that admits at most one major
  version: an exact version (`1.2.3`), or a range that stops before the next
  major version (`~1.2.0`, `^1.2.0`, `1.2.x`, `1.x`, `>=1.2.0 <2.0.0`). A value
  that is empty, `*`, or a lower bound with no upper bound (`>=1.2.0`) fails.
  Then each such chart has a `Chart.lock` in version control (not excluded by a
  `.gitignore` file) that lists each dependency of `Chart.yaml` with the version
  that was resolved. The integrity hash of each downloaded chart is not judged
  here (USS-DEP-002 judges it).
- **References:** Helm 4.3.1 documentation, `helm dependency` and `helm
  dependency build` (read 2026-11-24); Helm 4.3.2, source of the lock digest
  in `internal/resolver/resolver.go` and `pkg/downloader/manager.go` (read
  2026-11-24); Masterminds/semver 3.5.0, Readme, range comparisons (read
  2026-11-24)

The dependencies of a chart MUST have bounded version ranges, and `Chart.lock`
MUST be committed.

**Rationale:** A range with no upper bound takes a new major version that nobody
has tested. `helm dependency build` restores the charts from `Chart.lock`, so
the committed file gives every build the versions that were resolved, and
`helm dependency update` is the deliberate step that changes them. `Chart.lock`
alone does not record the hash of each chart: its `digest` is a hash of the
dependency list, which `helm dependency build` compares with `Chart.yaml`, and
it verifies a downloaded chart only with signatures (`--verify`). The integrity
requirement of USS-DEP-002 is met another way, such as by committing the
packaged dependency charts under `charts/`, and USS-DEP-002 judges that part.

## Hardening fields

### USS-K8S-007: Do not mount unneeded service-account tokens

- **Level:** MUST NOT
- **Applies when:** deploys-to-kubernetes
- **Category:** Safety
- **Refines:** USS-HRD-008
- **Verify:** Render each chart with its default values and take the manifests
  that the project publishes. For each workload, search the code and the
  documentation for calls to the Kubernetes API (a Kubernetes client library,
  or a call to the API server). A workload that makes none has an effective
  `automountServiceAccountToken` of `false`: the value on its pod spec when that
  is set (it takes precedence), and otherwise the value on its ServiceAccount.
  A workload that has no value on either, or a value of `true` on its pod spec,
  mounts a token and fails when it calls no API. A workload that calls the API
  is not judged here.
- **References:** Kubernetes documentation, Configure Service Accounts for Pods
  (read 2026-11-24)

A Pod that does not call the Kubernetes API MUST NOT mount a service-account
token (`automountServiceAccountToken: false`).

**Rationale:** Kubernetes mounts a token into every pod by default, so leaving
the field out mounts one. The value on the pod spec wins over the value on the
ServiceAccount, so a pod-level `true` undoes a ServiceAccount that turns the
mount off.

### USS-K8S-008: Ship a default-deny network policy

- **Level:** SHOULD
- **Applies when:** deploys-to-kubernetes
- **Category:** Safety
- **Refines:** USS-HRD-007
- **Verify:** Render each chart with its default values and take the manifests
  that the project publishes. They include a NetworkPolicy that denies traffic
  by default: its `podSelector` is empty or matches all the pods of the
  software, its `policyTypes` list `Ingress` and `Egress` (in one policy, or
  one policy for each type), and it has no `ingress` or `egress` rule. They also
  include NetworkPolicy resources that allow the connections that the
  documentation or the manifests show each workload needs, each selecting that
  workload's pods. A workload that resolves names has an allow rule for the
  cluster's DNS, because a default-deny egress policy blocks DNS. No allow rule
  is open to every source or destination (an empty `from` or `to`, or the CIDR
  `0.0.0.0/0`) unless the documentation names a connection that needs it.
- **References:** Kubernetes documentation, Network Policies (read 2026-11-24)

The chart or manifests SHOULD include a NetworkPolicy that denies traffic by
default, and policies that allow only what each workload needs.

**Rationale:** In a namespace with no policy, every pod can reach every other
pod. A policy that selects all pods and allows nothing is the base that the
allow policies build on, and it also covers a workload that is added later and
has no policy of its own. A NetworkPolicy has an effect only where the cluster's
network plugin enforces it.
