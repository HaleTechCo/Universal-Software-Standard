# Docker Compose

- **Prefix:** COMPOSE
- **Kind:** module
- **Applies when:** platform.docker-compose

This module covers projects that ship or document Docker Compose files: how a
file names its images, how each service is hardened, which ports it publishes
and where its secrets come from. Compose is the module's subject, so the module
names Compose attributes directly. It uses no vendor profile and names no tool
without one. The attributes were checked against the Compose Specification,
which carries no version number (read 2026-11-24), and against the Docker
Engine documentation and source of the same date. The hardening rules
(USS-COMPOSE-002 to USS-COMPOSE-005) judge the Compose files that the
documentation tells people to run. A Compose file only for local development is
judged by USS-COMPOSE-001, USS-COMPOSE-006 and USS-COMPOSE-007. Each rule is
judged on the service definitions that remain after Compose merges the files
that the command names (`-f` files, `include`, `extends` and the default
override file). Dockerfiles and `docker run` instructions are outside this
module: the CIS Docker Benchmark rules of the Deployment hardening area cover
them (USS-HRD-001 and USS-HRD-002). The core rules that this module does not
refine apply as written.

## Images

### USS-COMPOSE-001: Pin each third-party image by digest

- **Level:** MUST
- **Applies when:** uses-container-images
- **Category:** Safety
- **Refines:** USS-SUP-004
- **Verify:** In each Compose file in the repository, each service that has no
  `build` section and whose image is not one that the project's own build
  publishes has an `image` value that names a digest: `@sha256:` followed by 64
  hexadecimal digits, with or without a tag before the `@` (`redis@sha256:...`
  and `redis:7@sha256:...` pass). An image that the project builds itself (the
  service has a `build` section, or the repository's own build publishes the
  image) is outside this rule. A value with a tag only, or with neither a tag
  nor a digest, fails. An `image` value written with a variable passes only when
  the default in the variable names a digest (`${IMAGE:-redis@sha256:...}`); a
  variable with no default fails.
- **References:** Compose Specification, services, `image` (read 2026-11-24)

Each third-party `image:` in a Compose file MUST name a digest (`@sha256:...`).
A third-party image is one that the project does not build.

**Rationale:** A tag names whatever image its owner last pushed to it, so the
same Compose file can start different software on two days. A digest names one
image, so the file starts the image that was reviewed.

## Hardening

### USS-COMPOSE-002: Harden each service

- **Level:** MUST
- **Applies when:** ships-container-image
- **Category:** Safety
- **Refines:** USS-HRD-002
- **Verify:** Take each service of each Compose file that the documentation
  tells people to run. Each of these holds, and a service for which one does
  not hold fails.
  (1) Non-root: `user` is set and is neither `root` nor a UID of 0 (a name, a
  UID, or either followed by `:` and a group passes); or `user` is not set and
  the user that the image sets (`USER` in its Dockerfile, shown by `docker
  image inspect` as `Config.User`) is set and is neither `root` nor 0.
  (2) Capabilities dropped: `cap_drop` lists `ALL`.
  (3) Capabilities added with a reason: each entry of `cap_add` has a comment
  on its own line or on the line above it that says why the service needs that
  capability. A service with no `cap_add` passes.
  (4) `security_opt` has the entry `no-new-privileges`, `no-new-privileges=true`
  or `no-new-privileges:true`.
  (5) `privileged` is not `true`.
  (6) `network_mode`, `pid` and `ipc` are not `host`.
  (7) Limits: the service has a memory limit (`mem_limit` or
  `deploy.resources.limits.memory`) and a PID limit (`pids_limit` or
  `deploy.resources.limits.pids`), and neither limit is `-1`.
- **References:** CIS Docker Benchmark v1.9.0, section 5, Level 1; Compose
  Specification, services, `user`, `cap_add`, `cap_drop`, `security_opt`,
  `privileged`, `network_mode`, `pid`, `ipc`, `mem_limit`, `pids_limit` and
  `deploy.resources` (read 2026-11-24); Docker Engine, `parseSecurityOpt` in
  `daemon/daemon_unix.go` (read 2026-11-24); CWE-250

Each service in the Compose files that the documentation tells people to run
MUST be hardened in all of these ways: it runs as a non-root user; it drops all
capabilities and adds back only the capabilities it needs, each with a comment
that gives the reason; it sets `no-new-privileges`; it is not `privileged`; it
uses no host network, PID or IPC namespace; and it has memory and PID limits.

**Rationale:** A process that runs as root with every capability, or that
shares the host's namespaces, can do far more to the host after a flaw than a
process that has been given only what it needs. `no-new-privileges` stops a
program in the container from gaining rights through a set-user-ID file, and
the two limits keep one runaway service from taking the host's memory or
process table. A reason next to each added capability lets a reviewer judge
whether it is still needed.

### USS-COMPOSE-003: Mount the root filesystem read-only

- **Level:** SHOULD
- **Applies when:** ships-container-image
- **Category:** Safety
- **Refines:** USS-HRD-003
- **Verify:** Take each service of each Compose file that the documentation
  tells people to run. Each sets `read_only: true`. Then start the service as
  the documentation says and use it for its documented tasks: it works, and each
  path that it writes to (found from the errors when a write fails) is a `tmpfs`
  entry or a volume of that service (a named volume or a bind mount). A service
  with a writable root filesystem, or one that writes to a path that is not
  declared, fails.
- **References:** Compose Specification, services, `read_only`, `tmpfs` and
  `volumes` (read 2026-11-24); Docker documentation, `docker container run`,
  `--read-only` (read 2026-11-24); CIS Docker Benchmark v1.9.0, section 5,
  Level 1

Each service in the Compose files that the documentation tells people to run
SHOULD set `read_only: true`, with a `tmpfs` or a volume for each path that it
writes.

**Rationale:** An attacker who runs code in the container cannot change the
software or plant tools on a filesystem that is read-only. Listing each place
that the service writes also tells operators what to back up.

### USS-COMPOSE-004: Define a health check

- **Level:** MUST
- **Applies when:** ships-container-image and is-network-service
- **Category:** Reliability
- **Refines:** USS-OBS-003
- **Verify:** Take each service of each Compose file that the documentation
  tells people to run. A long-running service is one that keeps running to
  serve requests or do continuous work; a service that runs a task and exits (a
  migration or a set-up step, which the documentation or the file shows by
  `restart: "no"` or by a `depends_on` entry with `condition:
  service_completed_successfully` on it) is not. Each long-running service has
  a `healthcheck` attribute with a `test` that is not `NONE`, and no `disable:
  true`. A long-running service without one fails.
- **References:** Compose Specification, services, `healthcheck` and
  `depends_on` (read 2026-11-24)

Each long-running service in the Compose files that the documentation tells
people to run MUST have a `healthcheck`.

**Rationale:** Compose reports the health of a service from its check, and
another service can wait for it with `depends_on` and `condition:
service_healthy`. A service with no check is reported as running even when it
hangs. The endpoints or commands that the check calls are USS-OBS-003.

### USS-COMPOSE-005: Do not mount the Docker socket

- **Level:** MUST NOT
- **Applies when:** ships-container-image
- **Category:** Safety
- **Refines:** USS-HRD-002
- **Verify:** Take each service of each Compose file that the documentation
  tells people to run. In its `volumes`, in the short syntax (`source:target`)
  and in the long syntax (`type: bind` with a `source`), no source is a path
  whose last part is `docker.sock` (the daemon's default socket is
  `/var/run/docker.sock`), and none is a directory that holds the socket
  (`/var/run`, or `/run` when the socket is there). The service does not set
  `use_api_socket: true`. A service that does either fails.
- **References:** CIS Docker Benchmark v1.9.0, section 5, Level 1; Compose
  Specification, services, `volumes` and `use_api_socket` (read 2026-11-24);
  Docker documentation, `dockerd`, daemon socket (read 2026-11-24); CWE-250

A service in the Compose files that the documentation tells people to run MUST
NOT mount the Docker daemon's socket, with a volume (of the socket or of a
directory that holds it) or with `use_api_socket`.

**Rationale:** Whoever can write to the daemon's socket can start a container
that mounts the host's filesystem, so a service with the socket has the rights
of root on the host. The daemon's own documentation says that access to the
default socket needs root or membership of the `docker` group.

## Secrets and ports

### USS-COMPOSE-006: Keep secrets out of Compose files

- **Level:** MUST NOT
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SEC-004
- **Verify:** Read each Compose file in the repository, and each file that it
  names in `env_file` or in a `file` source of a top-level `secrets` entry when
  the repository holds that file. None holds a secret value, judged as
  USS-SEC-004 judges a secret. Look at `environment`, `env_file` files,
  `build.args`, `command`, `entrypoint`, `labels` and the `secrets` entries. A
  value taken from the environment of the host (`${DB_PASSWORD}`, or a `secrets`
  entry with `environment` or `external`), from a `secrets` entry whose `file`
  the repository does not hold, or from an environment file that the repository
  does not hold passes.
- **References:** Compose Specification, services, `env_file` and `secrets`,
  and the top-level `secrets` element (read 2026-11-24); CWE-798

A Compose file in the repository MUST NOT hold a secret value. Secrets come
from Compose `secrets` or from an environment file that the repository does not
hold.

**Rationale:** A Compose file is committed and copied with the repository, so a
password in it reaches everyone who can read the repository and stays in its
history. The Compose `secrets` element hands a secret to the service as a file
from a source outside the file.

### USS-COMPOSE-007: Bind each published port to a stated address

- **Level:** MUST
- **Applies when:** always
- **Category:** Safety
- **Refines:** USS-SEC-002
- **Verify:** In each Compose file in the repository, each entry of `ports` of
  each service names a host address. In the short syntax
  (`[HOST:]CONTAINER[/PROTOCOL]`), `HOST` has an address before the port:
  `127.0.0.1:8001:8001`, `[::1]:6001:6001` and `0.0.0.0:8080:80` pass, and
  `"3000"` and `"8000:8000"` fail. In the long syntax, `host_ip` is set. A value
  written with a variable passes only when the default in the variable is an
  address (`${BIND_ADDR:-127.0.0.1}:8080:80`); a variable with no default
  fails. An entry under `expose` is not a published port and is not judged.
- **References:** Compose Specification, services, `ports` (read 2026-11-24);
  Docker documentation, Port publishing and mapping (read 2026-11-24); Docker
  documentation, Packet filtering and firewalls (read 2026-11-24)

Each published port MUST name the host address that it binds to, such as
`127.0.0.1:5432:5432`. An address of `0.0.0.0` states the choice too.

**Rationale:** Docker publishes a port on all host addresses when the entry
names none, and a published port is reachable from outside the host, past the
rules of a host firewall such as ufw. The file then opens a service to the
network without anyone having chosen it. Naming the address makes each exposed
port a decision that a reviewer can see.
