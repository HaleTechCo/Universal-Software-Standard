# Deployment hardening

- **Prefix:** HRD
- **Kind:** area
- **Applies when:** ships-container-image or deploys-to-kubernetes

This area covers how the images, Compose files and Kubernetes manifests that the
project publishes are configured for safe operation. Its rules count toward
Safety by default; the probes rule counts toward Reliability. Running without
elevated privileges is USS-SEC-003, and the benchmarks that this area requires
cover it for containers. The Observability (OBS) area holds the health and
readiness endpoints of the service itself (USS-OBS-003); this area holds the
probes that call them in the manifests. The Distribution and install (DST) area
holds how the project's own images are named in what it publishes (USS-DST-016).

## Container images

### USS-HRD-001: Build images to the CIS Docker Benchmark

- **Level:** MUST
- **Applies when:** ships-container-image
- **Category:** Safety
- **Verify:** List each image that the project publishes, with the build file
  of each. For each, check the Level 1 recommendations of the section
  "Container Images and Build File" of the CIS Docker Benchmark against the
  build file and the built image, by hand or with a tool that implements the
  benchmark (such as docker-bench-security). Each Level 1 recommendation that
  applies is met. An image that fails one fails the rule.
- **References:** CIS Docker Benchmark v1.9.0, section 4, Level 1

Each published image MUST meet the Level 1 recommendations of the CIS Docker
Benchmark for container images and build files.

**Rationale:** The benchmark collects the settings that keep an image small,
current and unprivileged, such as a user other than root. It is maintained by a
community of practitioners, so the project does not restate it.

### USS-HRD-002: Run containers to the CIS Docker Benchmark

- **Level:** MUST
- **Applies when:** ships-container-image
- **Category:** Safety
- **Verify:** List each Compose file and each `docker run` instruction that the
  project publishes (in the repository, the documentation or a release). For
  each, check the Level 1 recommendations of the section "Container Runtime"
  of the CIS Docker Benchmark against its options, such as capabilities,
  privileged mode, mounts, ports and resource limits, by hand or with a tool
  that implements the benchmark. Each Level 1 recommendation that applies to
  the options in it is met. A file or an instruction that fails one fails the
  rule.
- **References:** CIS Docker Benchmark v1.9.0, section 5, Level 1

Each Compose file and `docker run` instruction that the project publishes MUST
meet the Level 1 recommendations of the CIS Docker Benchmark for the container
runtime.

**Rationale:** An image that is built well can still be started with every
privilege. Users copy the project's published command, so it has to be a safe
one.

### USS-HRD-003: Use a read-only root filesystem

- **Level:** SHOULD
- **Applies when:** always
- **Category:** Safety
- **Verify:** Render each chart with its default values and take the
  container definitions that the project publishes: each service of each
  Compose file, each `docker run` instruction and each container of each
  Kubernetes workload. Each sets the root filesystem read-only: `read_only:
  true` in Compose, `--read-only` in the instruction, and
  `securityContext.readOnlyRootFilesystem: true` in Kubernetes. Then run the
  software that way for its documented tasks: it works, and each path that it
  writes to (found from the errors when a write fails) is a declared volume,
  tmpfs or mount in that file, instruction or manifest. A container with a
  writable root filesystem, or one that writes to a path that is not declared,
  fails.
- **References:** Kubernetes documentation, Configure a Security Context for
  a Pod or Container (read 2026-11-24); Docker documentation, `docker
  container run`, `--read-only` (read 2026-11-24); Compose Specification,
  services, `read_only` (read 2026-11-24)

Containers SHOULD run with a read-only root filesystem, and each path that the
software writes to SHOULD be a declared volume.

**Rationale:** An attacker who runs code in the container cannot change the
software or plant tools on a filesystem that is read-only. Naming each place
that the software writes also tells operators what to back up. This rule can
overlap the whole-benchmark conformance of USS-HRD-002 where the benchmark
includes the same check, as the specific Security rules overlap the
ASVS-level rules (USS-SEC-019 to USS-SEC-021).

## Kubernetes

### USS-HRD-004: Meet the Restricted Pod Security Standard

- **Level:** MUST
- **Applies when:** deploys-to-kubernetes
- **Category:** Safety
- **Verify:** Render each chart with its default values (such as `helm
  template`) and take the manifests that the project publishes. Apply them,
  with a server-side dry run, to a namespace that enforces the Restricted
  profile (the label `pod-security.kubernetes.io/enforce: restricted`): every
  workload (pods and the pod templates of controllers) is accepted, with no
  violation and no warning from the profile. A workload that the profile
  rejects fails.
- **References:** Kubernetes documentation, Pod Security Standards (read
  2026-11-24), Restricted profile

Every workload in the project's Kubernetes manifests and charts MUST pass the
Restricted profile of the Kubernetes Pod Security Standards.

**Rationale:** The Restricted profile is the cluster's own statement of what a
hardened pod is. A workload that passes it runs in clusters that enforce it,
which a growing number do.

### USS-HRD-005: Set resource requests and limits

- **Level:** MUST
- **Applies when:** deploys-to-kubernetes
- **Category:** Safety
- **Verify:** Render each chart with its default values and take the manifests
  that the project publishes. Each container of each workload, init
  containers included, has `resources.requests.cpu`,
  `resources.requests.memory` and `resources.limits.memory`. A container
  without one of the three fails.
- **References:** Kubernetes documentation, Resource Management for Pods and
  Containers (read 2026-11-24)

Every container in the manifests and charts MUST have CPU and memory requests
and a memory limit.

**Rationale:** Requests let the scheduler place the pod, and a memory limit
stops one container from taking the memory of the node and the pods that share
it.

### USS-HRD-006: Define liveness and readiness probes

- **Level:** MUST
- **Applies when:** deploys-to-kubernetes
- **Category:** Reliability
- **Verify:** Render each chart with its default values and take the manifests
  that the project publishes. A long-running container is one in the pod
  template of a Deployment, StatefulSet, DaemonSet or ReplicaSet, or in a
  bare Pod that has no `restartPolicy: Never` (init containers and the
  containers of Jobs and CronJobs are not). Each has a `livenessProbe` and a
  `readinessProbe`. A long-running container without both fails.
- **References:** Kubernetes documentation, Configure Liveness, Readiness and
  Startup Probes (read 2026-11-24)

Every long-running container in the manifests and charts MUST have a liveness
probe and a readiness probe.

**Rationale:** The liveness probe lets Kubernetes restart a container that
hangs, and the readiness probe keeps traffic from a container that cannot
serve it. The endpoints that the probes call are USS-OBS-003.

### USS-HRD-007: Deny network traffic that is not needed

- **Level:** SHOULD
- **Applies when:** deploys-to-kubernetes
- **Category:** Safety
- **Verify:** Render each chart with its default values and take the manifests
  that the project publishes. They include NetworkPolicy resources: a policy
  that denies ingress and egress for the pods of the software by default (an
  empty or matching `podSelector` with both policy types and no allow rule),
  and allow policies for the connections that the documentation or the
  manifests show the software needs. No allow rule is open to every source or
  destination (an empty `from` or `to`, or a CIDR of `0.0.0.0/0`) unless the
  documentation names a connection that needs it. A workload without a policy
  that selects it fails.
- **References:** Kubernetes documentation, Network Policies (read 2026-11-24)

The manifests and charts SHOULD include network policies that allow only the
traffic that the software needs.

**Rationale:** By default, every pod in a cluster can reach every other. A
policy limits what an attacker can reach from a pod that is compromised.

### USS-HRD-008: Limit service-account access

- **Level:** MUST
- **Applies when:** deploys-to-kubernetes
- **Category:** Safety
- **Verify:** Render each chart with its default values and take the manifests
  that the project publishes. For each workload, search the code and the
  documentation for calls to the Kubernetes API (a Kubernetes client library, or
  a call to the API server). A workload that makes none sets
  `automountServiceAccountToken: false`, on the pod or on its service account. A
  workload that makes calls can mount the token; the permissions of the roles
  that its service account is bound to are judged by USS-SEC-007, not here. A
  workload that mounts a token without calling the API fails.
- **References:** Kubernetes documentation, Role Based Access Control Good
  Practices (read 2026-11-24)

A workload MUST mount a service-account token only when it calls the Kubernetes
API. The permissions of the role behind the token are scoped by USS-SEC-007.

**Rationale:** A token in a pod is a credential that an attacker who takes the
pod can use against the cluster. Not mounting it takes that credential away from
each workload that has no use for it, and the scoped role of USS-SEC-007 limits
what the others cost.
