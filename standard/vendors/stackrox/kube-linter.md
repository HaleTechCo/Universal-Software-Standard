# kube-linter

- **Kind:** profile
- **Product:** kube-linter
- **Vendor:** StackRox
- **Maintainer:** community
- **Versions:** >=0.8.3, <0.9.0

kube-linter reads Kubernetes YAML files, Helm charts and Kustomize directories
and reports objects that break its built-in checks, such as a container that
runs as root, has no resource limits or has no probe. The Kubernetes module
recommends it in USS-K8S-002. Put the baseline below in `.kube-linter.yaml` (or
`.kube-linter.yml`, which kube-linter reads too) in the repository root. The
checks and settings that the mappings name are those of kube-linter 0.8.3, from
its documentation and source (read 2026-11-24). The baseline turns on every
built-in check, including the ones that are off by default. `kube-linter lint`
returns the error "found N lint errors" when it reports anything, and `main`
turns that into exit status 1, so a CI job fails on a finding. It lints a chart
directory by rendering the chart with its `values.yaml` through the Helm 3
library that it embeds; a project that renders its charts with Helm 4 can lint
the rendered output instead. The checks report on Deployments, StatefulSets,
DaemonSets, ReplicaSets, Pods, Jobs, CronJobs and similar workloads (its
`DeploymentLike` kind), on their pod specs, init containers and ephemeral
containers included unless a mapping says otherwise.

The Restricted Pod Security Standard that USS-HRD-004 requires is not mapped as
one rule: kube-linter tests a part of it. It checks the host namespaces
(`host-network`, `host-pid`, `host-ipc`), privileged containers
(`privileged-container`), the `/proc` mount type (`unsafe-proc-mount`), unsafe
sysctls (`unsafe-sysctls`), some host paths (`sensitive-host-mounts`,
`writable-host-mount`), and, in part, privilege escalation
(`privilege-escalation-container`) and the non-root user (`run-as-non-root`).
It does not check the seccomp profile, the dropped capabilities (only NET_RAW,
by `drop-net-raw-capability`), the volume types, AppArmor, SELinux, host ports
or the Windows HostProcess setting. It has no built-in check for the
service-account token mount of USS-HRD-008 and USS-K8S-007, or for the memory
request that USS-HRD-005 requires. The rows below name each check that tests a
requirement, so USS-K8S-001 can be judged from them, and the rest of
USS-HRD-004 is judged as that rule says (a server-side dry run against a
namespace that enforces the Restricted profile).

A suppression is the annotation `ignore-check.kube-linter.io/<check>` on the
object, and its documentation encourages a reason as the value. The
documentation describes no setting that requires the reason or that reports an
annotation that suppresses nothing, so the auditor reads each annotation and
lists it as a suppression with the check's name as its rule. The annotation
`kube-linter.io/ignore-all` suppresses every check on one object. The checks of
the workload kinds above also report Jobs, CronJobs and Pods that do not restart
(which some mappings below note). The baseline excludes the two built-in checks
whose meaning depends on an organisation's own labels or annotations:
`required-label-owner` (requires a label with the key `owner`) and
`required-annotation-email` (requires an annotation with the key `email` and an
email address as its value). A project has no such label or annotation unless
its organisation defines them, so these two cannot set a floor for every
project; they are the only built-in checks of kube-linter 0.8.3 that read an
organisation's own label or annotation keys. A project that needs another
exception uses the annotation with its reason, not an exclusion.

Community-maintained: written by USS contributors until the vendor takes it
over.

## Baseline configuration

```yaml
# .kube-linter.yaml, in the repository root.
# Run with: kube-linter lint <rendered manifests, or a chart directory>
checks:
  # Every built-in check on, the ones that are off by default included.
  addAllBuiltIn: true
  exclude:
    # These two checks require label and annotation keys that only an
    # organisation defines, so they cannot be a floor for every project.
    - required-label-owner
    - required-annotation-email
  # No other exclusion and no ignorePaths: no other check is turned off and no
  # path is skipped. A single exception is an annotation on the object, with a
  # reason.
```

## Mappings

| Rule | Category | Tool rules or settings | Coverage | How to verify |
|---|---|---|---|---|
| USS-HRD-003 | Safety | `no-read-only-root-fs` (reports a container whose `securityContext.readOnlyRootFilesystem` is unset or not true, init and ephemeral containers included) | partial: the check reads Kubernetes workloads only, so Compose services and `docker run` instructions are judged apart (USS-COMPOSE-003), and it does not show that each path the software writes is a declared volume | Run `kube-linter lint` on the rendered manifests and expect no `no-read-only-root-fs` finding; run the software read-only and check the paths that it writes. |
| USS-HRD-005 | Safety | `unset-cpu-requirements` (reports a container whose CPU request is unset or 0), `unset-memory-requirements` (reports a container whose memory limit is unset or 0) | partial: no built-in check tests the memory request, which the rule also requires | Run `kube-linter lint` and expect no finding of these checks; read `resources.requests.memory` of each container. |
| USS-HRD-007 | Safety | `non-isolated-pod` (reports a workload that no NetworkPolicy in its namespace selects) | partial: the check passes when any policy selects the pods, so it does not show that a default-deny policy exists, or judge the allow rules | Run `kube-linter lint` and expect no `non-isolated-pod` finding; read the policies for the default deny and for allow rules that are open to every source or destination. |
| USS-K8S-001 | Safety | `host-network`, `host-pid`, `host-ipc`, `privileged-container`, `privilege-escalation-container`, `run-as-non-root`, `unsafe-proc-mount`, `unsafe-sysctls`, `sensitive-host-mounts`, `writable-host-mount` (the Restricted fields that the checks above cover), `unset-cpu-requirements`, `unset-memory-requirements` (resources), `no-liveness-probe`, `no-readiness-probe` (probes); `checks.addAllBuiltIn: true` | partial: CI must render every chart and manifest, run `kube-linter lint` on the result and fail on exit status 1, and the checks that kube-linter lacks (listed above) are judged by the hardening rules themselves | Read the CI job for the render and `kube-linter lint` steps; run `kube-linter lint` on the rendered manifests and expect exit status 0. |
| USS-SEC-003 | Safety | `run-as-non-root` (reports a container with no effective non-root user: `runAsUser` above 0, or `runAsNonRoot` true without `runAsUser` 0), `privileged-container`, `privilege-escalation-container` (reports `allowPrivilegeEscalation` true, `privileged` true or an added `SYS_ADMIN`) | partial: the checks read Kubernetes workloads only, `privilege-escalation-container` reports an explicit true and not an unset field, and other run instructions and installers need their own check | Run `kube-linter lint` and expect no finding of these checks; read the container files, service definitions and installers. |
| USS-SEC-004 | Safety | `env-var-secret` (reports an environment variable whose name contains "secret" and that has a literal value) | partial: the check sees only that name pattern in Kubernetes workloads, so a secret scan of the working tree and the history is still needed | Run `kube-linter lint` and expect no `env-var-secret` finding; run a secret scan of the working tree and the history. |
| USS-SEC-007 | Safety | `wildcard-in-rules` (reports a Role or ClusterRole rule that uses a wildcard), `cluster-admin-role-binding` (reports a binding to cluster-admin), `access-to-secrets` (reports a binding that grants get, list or watch on Secrets) | partial: the checks find wildcard, cluster-admin and Secret-read rights only, so the auditor judges whether each other permission, and each Secret-read grant that the software needs, is justified | Run `kube-linter lint` and expect no finding of these checks; read each Role, ClusterRole and binding against what the software calls. |
| USS-HRD-006 | Reliability | `no-liveness-probe`, `no-readiness-probe` (report a non-init container with no `livenessProbe` or no `readinessProbe`) | partial: the checks also report the containers of Jobs, CronJobs and Pods that never restart, which the rule does not cover, so the auditor leaves those findings out | Run `kube-linter lint` and expect no finding of these checks on a long-running container. |
| USS-K8S-003 | Reliability | `schema-validation` (validates each object against the Kubernetes API schemas with kubeconform, in strict mode and skipping kinds that have no schema) | partial: the built-in check validates against one Kubernetes version and the rule needs each version that the project supports, so CI runs kubeconform for each version | Run `kube-linter lint` and expect no `schema-validation` finding; read the CI job for a run for each supported version. |

## Check-integrity guards

| Guard | Setting | Rule |
|---|---|---|
| Every built-in check is on, except the two that depend on an organisation's labels | `checks.addAllBuiltIn: true`, and a `checks.exclude` that lists only `required-label-owner` and `required-annotation-email` (an exclusion always wins over the checks that are on) | USS-QLT-006, USS-QLT-007 |
| No path is skipped | no `checks.ignorePaths` | USS-QLT-006 |
| A suppression names one check and gives the reason | the annotation `ignore-check.kube-linter.io/<check>` with the reason as its value (the documentation encourages the reason and describes no setting that requires it) | USS-QLT-009 |
| No blanket suppression | the annotation `kube-linter.io/ignore-all` is not used | USS-QLT-010 |
