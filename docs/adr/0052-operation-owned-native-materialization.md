---
id: ADR-0052
title: Own declared cache fills through native physical execution
status: proposed
date: 2026-09-17
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-16, DM-42]
design: [§17]
review: docs/design_review/reviews/design_review_datafusion-cache-factory_2026-09-17.md
evidence: Interface-checked
supersedes: []
superseded-by: null
revisit: Expansion beyond the four declared operation materialization families or adoption of a memory tier.
verification: operation_index unit tests and Plan 18 CF01-CF10 qualification.
---

# ADR-0052: Own declared cache fills through native physical execution

## Context

The procedural operation index computed a row-count side channel and materialized before its
consumers were planned. DataFusion 55.1 provides CacheFactory and native spill primitives, but
its default cache and example do not supply our operation ownership, reservation or single-fill
contract. The cache review's acceptance gates remain open; this record stays proposed until
application conformance supports an Accept or Accept-scoped review.

## Scope

SearchIndex, ComparisonKeys, OverviewChildren and OverviewNamespaces within one admitted operation.
Immutable Delta provider descriptors and RuntimeEnv file caches retain their distinct lifetimes.
This changes no durable authority, result-retention rule or MCP functionality.

## Drivers

- One explicit relational intermediate serves every count, page and summary consumer.
- Planning and plain EXPLAIN have no execution effects.
- Shared native tasks, memory reservations and retention owners outlive individual waiters.

## Options

- Keep the procedural spill index: duplicates lifecycle policy and bypasses native cache planning.
- Default DataFrame.cache: eager MemTable retention does not meet managed-memory/property contracts.
- Upstream CacheFactory example: executes in planning and duplicates concurrent fills.
- Declared CacheFactory plus owned physical extension: selected; requires a small shared initializer.

## Decision

The QueryRuntime template installs one CacheFactory and composes its extension planner with the
existing Arrow contract, retention, command and Delta planners. Only explicitly bound declared
families are admitted. The immutable binding retains its exact logical input, full output schema,
function objects, effective session configuration, source/build identity and operation capability.
An operation capability is the actual admitted owner, not its correlation string.

The first polled reader starts one native owned fill. All physical plans prepared from that logical
binding share its terminal result. Input narrowing stays outside the shared base. Unknown/effectful,
volatile and unprotected mutable inputs are refused. Input replacement must preserve the admitted
meaning; a changed input needs a new binding. Native aggregation computes counts over the same base.

Use DataFusion SpillManager, DiskManager, SpillFile and SpillMetrics for incremental IPC and replay.
Reserve writer/reader workspace explicitly; the native reader's maximum-batch argument is a hint.
A fill never reacquires its caller's query permit. Operation termination cancels shared work;
dropping one reader does not. Running blocking work retains its resources until physical exit.

### Consequences

The initial backing always spills. It favors one bounded implementation over an unmeasured memory
tier. Coalesced replay advertises one partition and no unproven global ordering. Bindings are neither
serialized nor reused across operations. Full application qualification remains Plan 18 work.

### Compensating controls

Pinned API evidence, exact owner/schema checks, logical pushdown barriers, native reservations,
physical task tracking and isolated concurrency/cancellation/property units. CP12 performs whole
journey qualification only after the pivot and required legacy deletions.

## Evidence

Read from exact local 55.1 source on 2026-09-17; an independent upstream verifier also compiled the
incremental SpillManager API against the cached workspace rlib without executing a service journey.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Factory returns a logical plan | [CacheFactory](https://github.com/apache/datafusion/blob/55.1.0/datafusion/core/src/execution/session_state.rs#L2359) | 2026-09-17 | `fn create(&self, plan: LogicalPlan, session_state: &SessionState) -> Result<LogicalPlan>;` |
| Native incremental spill is available | [SpillManager](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/spill/spill_manager.rs#L86) | 2026-09-17 | `pub fn create_in_progress_file` |
| The read-size argument does not enforce capacity | [Spill reader contract](https://github.com/apache/datafusion/blob/55.1.0/datafusion/physical-plan/src/spill/spill_manager.rs#L171) | 2026-09-17 | "this value is used only as a validation hint" |
| Default logical extension blocks limits | [Logical extension](https://github.com/apache/datafusion/blob/55.1.0/datafusion/expr/src/logical_plan/extension.rs#L323) | 2026-09-17 | `false // Disallow limit push-down by default` |

## Verification

`cargo test -p enrichment-store --lib operation_index::tests` exercises actual DataFusion on small
native fixtures. Receipt: `.dev-state/plan18/execution/operation-cache-units.log` when executed.
CF01-CF10 and final-source Q/SC qualification remain separately recorded in Plan 18; a compile or
unit result does not close those complete application obligations.

## Boundaries preserved

§B1–§B13: Rust retains core policy, state and effects; Python stays thin; native DataFusion and
Delta own transformations and durable transactions. Temporary operation reuse conveys no execution
grant and cannot change publication or retained-evidence authority.

## More information

[Plan 18](../plans/18-schema-governed-runtime-and-cache-completion.md),
[cache review](../design_review/reviews/design_review_datafusion-cache-factory_2026-09-17.md).

## Status history

- 2026-09-17 — proposed; exact interfaces checked, application conformance pending.
