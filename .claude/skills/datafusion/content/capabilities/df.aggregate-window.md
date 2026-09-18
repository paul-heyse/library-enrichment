# Reuse aggregate/window functions and characterize state before extensions

Choose built-in aggregates/windows first. For extensions, partial state, merge, grouped layout and window retraction are distinct contracts; an accumulator is not merely a final reduction function.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| built-in aggregate/window | Existing semantics and frames fit | Check empty/null/order/distinct behavior and registration. |
| Accumulator | Custom stateful aggregate | Consumes batches; state output must compose correctly with merge_batch and declared state fields. |
| GroupsAccumulator / adapter | Many groups need shared state layout | Specialization can reduce overhead but must preserve groups/null/emit semantics; benchmark rather than assuming. |
| PartitionEvaluator / retract-capable aggregate | Window-specific or moving-frame evaluation | Capability hooks determine available evaluation paths; returning unsupported/default may be correct. |

## Contract

**state.** Accumulator update_batch receives arrays, state returns partial ScalarValues, merge_batch consumes state arrays, and evaluate produces a final ScalarValue. Declared state fields must agree with both serialization and merge.
Claim `df.aggregate-window.state`; upstream_contract_interpretation; evidence: upstream.

**frames.** Window partitioning, ORDER BY, peer groups and frame bounds determine rows contributing to output. Default ordered RANGE and explicit ROWS can differ on duplicate ordering keys.
Claim `df.aggregate-window.frames`; upstream_contract_interpretation; evidence: upstream.

**retraction.** Sliding windows may require retract_batch or specialized partition evaluation. Advertise support only when removal/order/empty-state semantics are correct.
Claim `df.aggregate-window.retraction`; upstream_contract_interpretation; evidence: upstream.

**memory.** Accumulator size reporting and groups state influence resource accounting. A groups implementation is not an automatic throughput or memory improvement.
Claim `df.aggregate-window.memory`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Look up the built-in function and its signature/null/frame behavior.
- For custom aggregates test partitioned partial->merge against single-pass output.
- For moving windows test duplicate keys, empty frames and retractions; inspect state schema and memory reporting.

## Limits and unknowns

- Runtime coverage compares ordered RANGE/ROWS counts. Custom merge/retract implementations and groups performance require additional probes.

## Exact contracts

- [`datafusion_expr::udaf::AggregateUDFImpl`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-9f175a3c2e0fb1ef9e53572c) — `trait AggregateUDFImpl: Debug + DynEq + DynHash + Send + Sync + Any`
- [`datafusion_expr_common::accumulator::Accumulator`](../operations/datafusion_expr_common.accumulator.Accumulator.md#op-2911d7ffb2098886b7dd6ba8) — `trait Accumulator: Send + Sync + Debug + std::any::Any`
- [`datafusion_expr_common::groups_accumulator::GroupsAccumulator`](../operations/datafusion_expr_common.groups_accumulator.GroupsAccumulator.md#op-9c9c43a7d8bf357eb3adcb6e) — `trait GroupsAccumulator: Send + std::any::Any`
- [`datafusion_expr::partition_evaluator::PartitionEvaluator`](../operations/datafusion_expr.partition_evaluator.PartitionEvaluator.md#op-28021de2fba820ec392e9ef2) — `trait PartitionEvaluator: Debug + Send + std::any::Any`
- [`datafusion_functions_aggregate_common::aggregate::groups_accumulator::GroupsAccumulatorAdapter`](../operations/datafusion_functions_aggregate_common.aggregate.groups_accumulator.GroupsAccumulatorAdapter.md#op-d2e10609875cb34a913017d4) — `struct GroupsAccumulatorAdapter`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: joins_sets_and_window_frames_have_distinct_cardinality
