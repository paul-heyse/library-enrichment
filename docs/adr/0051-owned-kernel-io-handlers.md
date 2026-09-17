---
id: ADR-0051
title: Bind all Delta kernel handlers to owned execution lanes
status: accepted
date: 2026-09-16
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-16, DM-42]
design: [§17]
review: not-required: Executor and native handler corrections within the accepted unified runtime boundary.
evidence: Tested
supersedes: []
superseded-by: null
revisit: An exact upstream release admits an owned kernel executor through every LogStore and DataFusionEngine path with task context propagation.
verification: kernel_filesystem_progresses_with_saturated_native_pool, kernel_tasks_preserve_context_on_one_owned_worker, maintenance_vacuum_keeps_current_and_historical_deletion_vectors and Plan 17 Q10.
---

# ADR-0051: Bind all Delta kernel handlers to owned execution lanes

## Context

The pinned kernel's synchronous handlers wait for asynchronous filesystem work. Sharing the
caller's bounded blocking pool can exhaust that pool: a snapshot thread waits for local I/O
queued behind itself. Increasing the limit postpones saturation without removing the dependency.
Overriding LogStore::engine alone also leaves the provider's DataFusionEngine on ambient handles.

## Scope

Correct execution ownership within §17's single DataFusion resource and policy runtime.
Keep one RuntimeEnv, memory/spill/cache configuration, qualified object-store registry, native
planner and publication authority. The two Tokio lanes are scheduling mechanisms for this runtime.
No second query engine, compatibility reader or alternative publication path is introduced.

## Drivers

Maintain progress when native blocking callers saturate their lane. Preserve native Delta
providers, checkpoint/DV handlers, task ownership, correlation and effect authority. Use the
workstation's resources for parallel execution rather than serial custom query operators.

## Options

- Increase a shared blocking pool: still permits the same dependency cycle at saturation.
- Use default background engines: loses explicit ownership and consistent configuration.
- Copy kernel parsing/I/O into application code: duplicates the upstream engine.
- Inject native handlers using owned compute and I/O lanes: selected.

## Decision

Own separate compute and kernel-I/O Tokio lanes inside QueryRuntime. Run synchronous Delta
snapshot/replay callbacks on compute and their awaited kernel filesystem futures on I/O.
Both lanes use the configured worker/blocking count and stack size; counts are per lane.
Every task retains the executor owner and captures the current operation/effect context.

Bind DefaultEngineBuilder's public TaskExecutor, batch-size and buffer-size hooks. The admitted
local LogStore uses the native get_latest_version helper with the wrapper as receiver, since
forwarding to its inner store would recreate the default engine. Other backend implementations
require separate qualification before adopting this delegation.

Add the narrow KernelIoEngine SessionConfig extension at the existing vendored DataFusionEngine
constructor. Reuse the same qualified root filesystem backend and owned native handlers through
TaskContext for checkpoint and deletion-vector work. Keep native Arrow expression evaluation.
Use DataFusion's installed JoinSetTracer in Delta's existing blocking helper, preserving the
upstream span/dispatcher and application task-local context. No engine captures an operation's
authority at construction; reused handlers capture context separately at each task boundary.

### Consequences

The runtime owns an additional I/O lane and its configured blocking capacity. The operator has
explicitly prioritized performance on the 16-core/32-thread, 192-GiB development workstation.
The shared Arrow memory pool remains 32 GiB; neither worker stacks nor kernel allocations are
implicitly covered by that Arrow limit. Physical-exit drain and full external accounting remain
Plan 17 requirements. A detached kernel task's channel lifetime alone is not graceful-shutdown
qualification.

### Compensating controls

A bounded subprocess oracle runs actual native create/append/checkpoint/reopen/scan with one
worker and one blocking thread. It removes JSON logs before exact checkpoint reopen and invokes
DataFusionEngine storage from a saturated compute blocking pool. External DV readback and
historical maintenance use the same minimum blocking configuration. Upstream pins and vendor
hashes identify the exact handler patch; R-49 governs replacing it.

## Evidence

Sources read locally from the exact pinned checkouts on 2026-09-16; the URLs identify those bytes.
An independent upstream-verifier review identified the default-engine escape in provider scans.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Shared blocking saturation is an upstream-documented hazard | [Kernel executor, 8ba063f8](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/default-engine/src/executor.rs#L262) | 2026-09-16 | "If concurrent `block_on` calls exceed Tokio's `max_blocking_threads`, this can deadlock" |
| Provider engine constructors inherit ambient execution | [Delta engine, 58f07cd6](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/engine/mod.rs#L29) | 2026-09-16 | `Self::new(session.task_ctx(), Handle::current()).into()` |
| Native local version discovery takes its engine from the receiver | [Delta log store, 58f07cd6](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/mod.rs#L789) | 2026-09-16 | `let storage = log_store.engine(None).storage_handler();` |

## Verification

Execution receipts under `.dev-state/plan17/execution/`:

- `kernel-single-lane-stack.log`: GDB identifies the forwarded inner latest-version engine
  waiting on its shared blocking pool; the initial bounded subprocess fails.
- `kernel-session-engine-oracle.log`: two tests passed, 0.70 seconds, including the real bounded
  subprocess and context propagation. This receipt predates structured telemetry integration.
- `kernel-session-dv-oracle.log`: real current/historical external-DV maintenance/readback passed,
  1.18 seconds, at one compute worker and one blocking thread. It also predates telemetry changes.

These are focused executions, not complete Q10/Q13 or installed-product acceptance.

## Boundaries preserved

§B1–§B13 remain unchanged: Rust owns policy/effects/publication; Python is the thin MCP boundary;
DataFusion and native Delta own relational computation and transactions; studied repositories are
not state or execution destinations. Resource scheduling does not grant effect authority.

## More information

[Plan 17](../plans/17-schema-governed-unified-runtime-hard-pivot.md),
[compatibility matrix](../architecture/compatibility-matrix.md),
[deferred register](register.md#deferred-decisions).
