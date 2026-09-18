# Compact or Z-order while preserving logical data

Use Compact to combine eligible files and ZOrder for selected-column clustering. Measure output file/scan behavior separately from logical-row preservation.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| Compact | Small eligible files should be combined | A repeated run may be a no-op |
| ZOrder(columns) | Clustering by those columns may improve later data skipping | Requires workload evidence; it is not a global result-order guarantee |
| Leave layout unchanged | Rewrite cost exceeds demonstrated benefit | Track file and scan metrics before introducing periodic maintenance |

## Contract

**rows.** Compaction removed multiple small files and preserved all rows in the probe; repeating it did not advance the version. Z-order preserved the same logical rows.
Claim `delta.optimize.1`; runtime_observation; evidence: runtime.

**features.** The deletion-vector fixture preserved the eight live logical rows across optimize.
Claim `delta.optimize.2`; runtime_observation; evidence: runtime.

**resources.** Target size is an optimization input. Concurrency, buffering, memory pool and spill configuration jointly affect resources; no exact RSS or throughput bound follows from the API signature.
Claim `delta.optimize.3`; source_observation; evidence: upstream, source.

## Implementation

- Compare file counts, bytes, operation metrics and query read metrics before/after representative workloads.
- Keep Z-order columns aligned with actual filtering patterns and type support.

## Effects

- rewrite data files
- publish layout changes or no-op

## Errors

- Feature/protocol, data read, expression, resource and commit failures remain possible.

## Limits and unknowns

- No workload-level speedup, exact file-size bound or resource envelope is claimed.

## Exact contracts

- [`deltalake_core::table::DeltaTable::optimize`](../operations/deltalake_core.table.DeltaTable.md#op-30b3d3701758d0059efa4146) — `fn optimize<'a>(self) -> OptimizeBuilder<'a>`
- [`deltalake_core::operations::optimize::OptimizeBuilder::with_type`](../operations/deltalake_core.operations.optimize.OptimizeBuilder.md#op-7197eaf7da542a4421fa2def) — `fn with_type(self, optimize_type: OptimizeType) -> Self`
- [`deltalake_core::operations::optimize::OptimizeBuilder::with_target_size`](../operations/deltalake_core.operations.optimize.OptimizeBuilder.md#op-17215fd88c454cd2ccdba9cf) — `fn with_target_size(self, target: NonZeroU64) -> Self`
- [`deltalake_core::operations::optimize::OptimizeBuilder::Output`](../operations/deltalake_core.operations.optimize.OptimizeBuilder.md#op-bff0e9f1314e94985cdd4cdc) — `type Output = Result<(DeltaTable, Metrics), DeltaTableError>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/operations/optimize.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: optimize_compact_and_zorder_preserve_rows, deletion_vectors_filter_rows_and_optimize_keeps_logical_rows
