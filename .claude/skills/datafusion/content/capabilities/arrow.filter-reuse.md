# Reuse selection preparation when the workload warrants it

FilterBuilder separates mask preparation from applying it. Consider reusable preparation for many arrays sharing one mask; direct filter remains a simple one-shot choice.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| filter | One array or occasional selection | Minimal orchestration; do not assume reusing a builder is faster for this workload. |
| FilterBuilder / FilterPredicate | Repeated application of the same mask | Preparation and optimize have costs; measure mask density, sizes and reuse count. |

## Contract

**shape.** Build from &BooleanArray, optionally optimize, then apply the predicate to arrays/batches with the corresponding row domain.
Claim `arrow.filter-reuse.shape`; upstream_contract_interpretation; evidence: upstream.

**semantics.** Reuse changes preparation, not the expected selected values. False and null mask entries are excluded; selected order remains source order.
Claim `arrow.filter-reuse.semantics`; upstream_contract_interpretation; evidence: upstream.

**performance.** The optimize documentation describes a tradeoff for repeated use; it is not a universal performance guarantee. Benchmark total setup plus application and keep a direct-filter control.
Claim `arrow.filter-reuse.performance`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Reuse one built predicate only for arrays with the intended shared row domain.
- Measure one-shot and repeated cases separately; compare exact outputs before timing.

## Limits and unknowns

- No production throughput claim is established by the equality probe; target-dependent timing is separate.

## Exact contracts

- [`arrow_select::filter::FilterBuilder`](../operations/arrow_select.filter.FilterBuilder.md#op-cffeb561fc559f04206cfdc3) — `struct FilterBuilder`
- [`arrow_select::filter::FilterBuilder::optimize`](../operations/arrow_select.filter.FilterBuilder.md#op-58a30c42976fa26d4bc93e1e) — `fn optimize(self) -> Self`
- [`arrow_select::filter::FilterPredicate::filter`](../operations/arrow_select.filter.FilterPredicate.md#op-5fe81cc083d413b3f0be1467) — `fn filter(&self, values: &dyn Array) -> Result<ArrayRef, ArrowError>`
- [`arrow_select::filter::filter`](../operations/arrow_select.filter.filter.md#op-81498df4434a7e4fa4ec4f9e) — `fn filter(values: &dyn Array, predicate: &BooleanArray) -> Result<ArrayRef, ArrowError>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: filter_null_is_not_selected_and_metadata_is_retained
