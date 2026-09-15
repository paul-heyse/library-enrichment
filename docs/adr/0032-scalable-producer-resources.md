---
id: ADR-0032
title: Configure producer capacity and qualify observed resource limits
status: accepted
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-07, DM-27, DM-35, DM-39, DM-56]
design: [§8.2, §9.1, §9.2, §10]
review: docs/design_review/reviews/design_review_native-architecture-completion_2026-09-14.md
evidence: Implemented
supersedes: []
superseded-by: null
revisit: Measurements require a different concurrency or resource policy, or another execution platform lacks these observed kernel controls.
verification: workstation_capacity_is_preserved_and_invalid_requests_are_rejected; missing_or_changed_resource_observations_cannot_qualify; execution_configured_contract_matches_kernel_limits_and_output_bound
---

# ADR-0032: Configure producer capacity and qualify observed resource limits

## Context

The operator clarified on 2026-09-14 that prompt-serving latency for coding agents has priority
over minimizing resource consumption. Initial intermittent personal deployment uses a 16-core,
32-thread workstation with 192 GiB installed memory. A hardcoded one-CPU producer quota and an
8 GiB memory clamp frustrate that objective, independently of Arrow query settings. Existing
qualification binds configuration but lacks retained typed observations of its kernel limits.

## Scope

Refine resources and qualification within the existing Linux rootless execution boundary.
Extend the private executor data ceiling for larger bounded workspaces. Execution permissions,
public MCP tools, library-fact schemas and retention remain unchanged.

## Drivers

Use available parallelism, preserve explicit containment, reject unsupported settings clearly,
and distinguish configured capacity from actual enforcement and measured performance.

## Options

- Hardcoded small quotas: rejected; unrelated to the hardware or latency objective.
- Remove every resource limit: rejected; unnecessary for performance and inconsistent with
  containment of malformed or runaway work.
- Explicit scalable capacities and kernel qualification: selected; one Rust contract drives
  container creation, provenance, qualification and configuration validation.

## Decision

`Execution::resources()` validates CPU bandwidth, memory, scratch and PID settings before
container creation. CPU capacity is an integer number of logical CPUs, not affinity or a
scheduling reservation. Supported ranges are 1–4096 CPUs, 128 MiB–1 TiB memory, 16 MiB–64 GiB
scratch, and 16–1,048,576 PIDs, including threads. Scratch remains limited by configured memory,
as in ADR-0021. Invalid range values reject; CPU and memory are never silently clamped.

Generated personal deployment selects 32 CPUs, 32 GiB memory, 16 GiB scratch, 2,048 PIDs,
and a 128 GiB aggregate capsule reservation budget. These are capacities, not eager allocations.
The shared Arrow runtime independently has 32 GiB managed query memory, 64 GiB spill, a 2 GiB
metadata cache, 16 partitions and 16 concurrent queries. Eight expensive jobs and eight warm
LSP sessions are available. This starting profile is not a measured optimum or a whole-service
memory ceiling. Concurrent process-tree RSS remains a separate measurement obligation.

Podman receives the validated CPU, memory and PID values. Equal `--memory` and `--memory-swap`
prohibit additional swap, strengthening ADR-0021's resource policy while preserving its workspace
and publication boundary. Private capsule protocol v2 supports up to 64 GiB admitted input/output
bytes; v1 is unsupported. No migration or dual reader is introduced. Changed helper, controller
or resource identities require requalification.

Each image qualification runs fixed Rust-owned argv through the production executor, reading
`cpu.max`, `memory.max`, `memory.swap.max`, `pids.max` and tmpfs capacity. No caller supplies
shell text or host paths. Rust parses exactly those observations into typed fields, requires
successful process termination and whole-container cleanup, and compares requested and observed
values before acceptance. A missing swap file is a failure, never inferred zero. The receipt
retains typed fields and actual bounded process output; readiness revalidates their agreement
and exact image/configuration identity.

### Consequences

Large crates and language servers can use the machine's parallel capacity. Old qualification
receipts and helper protocols confer no readiness. Simultaneous heavy jobs can increase memory
and scheduling pressure; W7 measures mixed workload latency and concurrent peak use. A cgroup
quota is maximum bandwidth: ancestor limits and contention may reduce achieved throughput.

### Compensating controls

Small hostile-resource tests continue to exercise enforcement without workstation-sized
allocations. Separate tests observe full configured limits. Missing, malformed or inconsistent
resource observations reject. Reservation budgets, output limits, deadlines, cleanup supervision
and publication checks remain mandatory.

## Evidence

Verified 2026-09-14 with Context7 discovery, the required upstream-verifier and exact sources.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| CPU quota uses a 100,000-microsecond period | [Podman 4.9.3 conversion](https://github.com/containers/podman/blob/v4.9.3/pkg/util/utils.go#L1125) | 2026-09-14 | `100000` |
| Equal memory-plus-swap disables swap in the installed runtime | [cgroups 0.0.6](https://github.com/opencontainers/cgroups/blob/v0.0.6/fs2/memory.go#L49) | 2026-09-14 | “memory and memorySwap set to the same value -- disable swap” |
| Filesystem capacity fields are available in image coreutils 9.7 | [stat implementation](https://github.com/coreutils/coreutils/blob/v9.7/src/stat.c#L908) | 2026-09-14 | `f_blocks`; `f_frsize` |
| Kernel resource controls have distinct semantics | [cgroup v2](https://docs.kernel.org/admin-guide/cgroup-v2.html) | 2026-09-14 | `cpu.max`; `memory.max`; `memory.swap.max`; `pids.max` |

Read-only image-layer inspection found coreutils 9.7-3 and dash 0.5.12-12 in both recorded
producer images. Actual selected image/tool/resource validation remains required at execution.

## Verification

`workstation_capacity_is_preserved_and_invalid_requests_are_rejected` checks explicit capacity
and protocol compatibility. `missing_or_changed_resource_observations_cannot_qualify` rejects
missing, inconsistent and unlimited observations or a changed CPU request. These are contract
tests, not actual containment acceptance.

`execution_configured_contract_matches_kernel_limits_and_output_bound` observes selected kernel
limits and tests output enforcement. `execution_boundary` and `execution_cleanup` exercise real
hostile memory/process/scratch, cancellation, descendants and cleanup. `just execution-qualify
--apply` writes its receipt only after image probes and that tier pass. Exact logs and failed
attempts belong in the Plan 11 execution ledger.

## Boundaries preserved

§B1–§B13 remain intact. Rust owns configuration, protocol, qualification, evidence and queries;
Python setup only orchestrates the Rust description. Studied-library execution remains in
qualified capsules without repository/home mounts. No public shell, alternate storage engine,
embedded Context7, age-based expiry or automatic project edit is added.

## More information

[Plan 11](../plans/11-arrow-datafusion-completion-and-legacy-removal.md),
[compatibility matrix](../architecture/compatibility-matrix.md),
[workstation configuration](../../config/service.workstation.toml).

## Status history

- 2026-09-14 — proposed; implementation and focused contracts exist; scoped review and full
  configured containment and mixed workload performance qualification remain required.

- 2026-09-14 — accepted after scoped native architecture review; remaining actual producer,
  client and final performance qualification stays in Plan 12.
