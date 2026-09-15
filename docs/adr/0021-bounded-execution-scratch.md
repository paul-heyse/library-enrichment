---
id: ADR-0021
title: Keep execution writes in bounded scratch and validate output handoff
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-11, DM-15, DM-27, DM-35, DM-46]
design: [§8.2, §9.1, §9.2, §10]
review: docs/design_review/reviews/design_review_bounded-execution-scratch_2026-09-14.md
evidence: Proposed
supersedes: []
superseded-by: null
revisit: Another execution platform or a producer requires writable persistent state or output outside this protocol.
verification: execution_boundary; execution_cleanup; retained_capsule_integrity; executor_handoff
---

# ADR-0021: Keep execution writes in bounded scratch and validate output handoff

## Context

Plan 09 W2 and completion review findings F3/F4 require a hard execution-write bound.
The current writable host capsule mount and admission-time directory-size check do not
bound bytes written by target code. A failed or interrupted producer can also leave mutable
capsule content that later preparation trusts without an integrity check.

## Scope

Strengthen the existing isolated execution contract with read-only admitted inputs,
bounded memory-backed workspaces, retained-capsule integrity and explicit output handoff.
The execution profiles, supported ecosystems, dependency pins and frozen MCP envelope
remain unchanged. ADR-0017's Rust-owned dependency admission remains in force.

## Drivers

Bound target writes during execution, preserve validated reusable inputs, and require
confirmed cleanup before admitting outputs. Avoid introducing a second host policy owner.

## Options

- Keep writable host mounts with preflight size checks: rejected; growth remains unbounded.
- Read tmpfs outputs after stopping or pausing: rejected for the installed Podman copy path.
- Add host loopback filesystems/quotas: deferred; adds privileged/platform setup.
- Read-only inputs, explicit tmpfs, and a bounded Rust output protocol: proposed selection.

## Decision

### Containment and input preparation

One Rust-owned broker description defines private paths, sanitized environment, containment
arguments and effective bounds for setup, qualification and service execution. Input trees,
operation manifests and the first-party executor binary are mounted read-only and without
recursive propagation. The helper executable's digest is part of qualification and producer
identity. A repository under study and the operator's home/toolchains are never mounted.

Containers use `--read-only`, `--read-only-tmpfs=false`, `--image-volume=ignore`, and explicit
`--tmpfs=/capsule:rw,exec,nosuid,nodev,size=<bound>m,mode=1777,notmpcopyup`. Writable work
subdirectories are private to UID 65532. The effective scratch bound defaults to 512 MiB
and cannot exceed the configured memory bound; temporary/cache paths stay inside it.
Execution of compiled Rust and native extension code requires the explicit exec permission.
Memory-backed does not claim unswappable memory. Exhaustion is a recorded failed process,
never a fallback to writable host storage.

The small first-party Rust executor copies admitted inputs into scratch, executes a concrete
Rust-derived argv, and reports bounded observations. LSP mode replaces the helper with the
server after initialization of scratch, retaining ordinary framed stdin/stdout. It introduces
no shell-command MCP tool or generic workflow surface.

### Output handoff

Preparation and local rustdoc specify exact output files or bounded directory roots in a
Rust-owned operation record. Ordinary runtime and compiler probes retain process evidence;
they do not copy arbitrary modified workspaces back to reusable input trees.

For commands with retained outputs, the executor streams a versioned, length-delimited
protocol while the container is still running. Before inventory or file reads it must establish
that target execution and all its descendants have stopped: the executor is the only remaining
live container process, after bounded termination/reaping. Failure to establish quiescence
rejects handoff; individually valid digests cannot certify a concurrently changing file set.
Target stdout/stderr are separate bounded
fields, not protocol delimiters. File entries carry relative path, size, kind and content
digest. The daemon enforces its own aggregate byte, file-count, path and frame limits while
reading into an unmounted private quarantine. Unknown/duplicate paths, missing required
outputs, traversal, symlinks, hard links, devices, malformed frames, overflow and incomplete
streams reject the handoff. No arbitrary TAR extraction or stopped-container tmpfs copy is
used. The executor remains alive after a complete handoff until the daemon removes its
container. Only after confirmed whole-container cleanup may validated quarantine content
be promoted to an admitted input tree or retained artifact. Cleanup failure preserves the
obligation and quarantines admission; it does not publish an execution success.

The host's immutable request binds operation, context, admitted input inventory, image,
helper/protocol identity and limits. Container-supplied IDs never replace these bindings.
Target exit/status/output are executor-reported observations within that execution scope;
host deadlines, stream byte counts, digest validation and container cleanup are separate
host observations. The host requires exactly one complete terminal frame matching its
operation plus the declared file inventory. EOF, missing/trailing frames and a dead helper
are failures, not an inferred target exit code. The host deadline covers startup, execution,
quiescence and handoff; a reported success cannot override host timeout or cleanup failure.

### Retained input validity and reservations

A retained preparation key binds the release/source digest, declared environment, image,
preparation version, helper/protocol identity, normalized operation and containment semantics,
and meaning-changing policy/configuration. Requalifying a changed helper therefore cannot
reuse an old preparation under the same key. Its host-only manifest records
the exact file inventory, modes, sizes, digests, lock digest and resolved environment. Reuse
requires agreement with that manifest. Missing, changed or extra inputs trigger a new
preparation; an active or incompletely cleaned workspace cannot be removed or overwritten.
A per-key lock lives outside the mounted tree and is held by the continuous execution lease
through preparation, warm use and cleanup. Warm hits reuse their lease and prepared inputs.

The daemon owns aggregate reservations for retained input bytes plus every in-flight
quarantine and new generation, including replacement overlap. Reservation and inventory are
serialized, and the reservation is acquired before any bounded host write. Crash recovery
reconstructs occupied and reserved bytes conservatively from disk before admitting new work.
Failure/cancellation releases space only after quiescence and actual deletion; dropping an
in-memory token alone does not release occupied storage. Promotion publishes one manifest-bound
generation after cleanup; an old valid generation remains intact until atomic replacement.
Full capacity refuses new preparation with the specific
manual cleanup action. Storage pressure and warm-session eviction never delete retained
library evidence. Validated facts remain reusable without age expiry under ADR-0020.

### Qualification

A receipt binds execution root, exact images, broker/runtime identity, helper digest and the
normalized containment description. A changed binding invalidates the receipt. Applying
requalification first invalidates the previous admission and records the attempt; failure
cannot leave stale readiness. Setup's shell exports quote values and reject receipts for
other roots. The binding and parsing logic are owned by Rust, not duplicated in setup Python.

### Consequences

Preparation explicitly names outputs and tests observe live scratch through the private
broker instead of relying on host-writable heartbeat files. The service carries a small
executor binary and protocol reader. Existing images must qualify that executable before
readiness can be claimed; no image pin is changed by this decision.

### Compensating controls

Real probes must prove the scratch filesystem type and byte ceiling, input immutability,
output handoff before cleanup, rejection of malformed/oversized outputs, retained-input
corruption rejection, concurrent reservations and cancellation/creation recovery. Fixture
protocol tests are useful but cannot replace real containment acceptance.

## Evidence

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Automatic writable tmpfs mounts must be disabled explicitly | [Podman 4.9.3 run](https://docs.podman.io/en/v4.9.3/markdown/podman-run.1.html#read-only-tmpfs) | 2026-09-14 | “The default is true.” |
| tmpfs size is an explicit mount property | [Podman 4.9.3 mount options](https://docs.podman.io/en/v4.9.3/markdown/podman-run.1.html#mount-type-type-type-specific-option) | 2026-09-14 | “tmpfs-size: Size of the tmpfs/ramfs mount in bytes.” |
| Copy enters the live mount namespace only for running containers | [Podman v4.9.3 implementation](https://github.com/containers/podman/blob/v4.9.3/libpod/container_copy_linux.go) | 2026-09-14 | `if c.state.State != define.ContainerStateRunning { return f() }` |

Context7 discovery and the upstream-verifier reviewed exact Podman 4.9.3 sources. Its
mount-option parser rejects uid/gid tmpfs options; runtime tests must verify the proposed
mode/UID setup. The output protocol is our proposed contract, not an upstream guarantee.

## Verification

Plan 09 W2: `execution_boundary`, `execution_cleanup`, retained-capsule integrity fixtures,
executor protocol corruption/overflow fixtures, actual preparation/probe/LSP reuse and
qualification invalidation tests. Record source-bound logs under `.dev-state/` and run
`just state-leak-check`. This proposed record does not claim those new scratch oracles passed.

## Boundaries preserved

§B1–§B13 remain. Rust owns policy, identities, dependency admission, storage and publication;
Python remains the adapter/static worker. Existing ty/rust-analyzer and hosted-first rustdoc
choices remain. Context7 stays a separate caller connection. Service state stays outside
repositories under study. No new database, execution profile or client tool is introduced.

## More information

[Plan 09](../plans/09-phase-4-6-completion.md), completion review F3/F4 and
[execution lease review](../design_review/reviews/design_review_execution-leases_2026-09-14.md).

## Status history

- 2026-09-14 — proposed; scratch and handoff implementation and acceptance remain open.
- 2026-09-14 — accepted after the scoped design review at Proposed/Interface-checked strength; implementation and real acceptance remain open.
