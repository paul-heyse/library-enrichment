# Read change data with explicit bounds, images and residual filtering

CDF uses an explicit build path or DeltaCdfTableProvider rather than IntoFuture. Plan for inclusive version ranges, change images, retention and operation-specific feature restrictions.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| DeltaCdfTableProvider | CDF should be queried as a DataFusion relation | Its scan applies residual row filtering; register it in a compatible context |
| CdfLoadBuilder::build | You need a physical plan directly | Execute the returned plan with the appropriate TaskContext and preserve required residual filtering |
| Snapshot comparison / application log | Required historical change data is unavailable | Define a different recovery contract; do not invent missing CDF history |

## Contract

**bounds.** Version start/end are inclusive. At this pin an ending version beyond head is clamped even without allow_out_of_range; an invalid beyond-head start is rejected unless the opt-in empty-range policy applies.
Claim `delta.cdf.1`; runtime_observation; evidence: runtime.

**images.** An update produced update_preimage and update_postimage rows with commit version metadata. Filtering id > 5 through the provider retained only the matching postimage.
Claim `delta.cdf.2`; runtime_observation; evidence: runtime.

**features.** The column-mapping fixture reads normally through Delta scan but CDF build rejects non-None column mapping.
Claim `delta.cdf.3`; runtime_observation; evidence: runtime.

**resume.** Checkpoint the processed commit interval only after downstream success. Explicitly distinguish a clamped end from the originally requested future version; do not advance a checkpoint over unobserved commits.
Claim `delta.cdf.4`; source_observation; evidence: upstream, source.

**retention.** Enablement, available log/data and requested interval jointly determine CDF availability. A table's current configuration alone does not prove historical changes were recorded.
Claim `delta.cdf.5`; source_observation; evidence: upstream, source.

**historical enablement.** After CDF was enabled on a previously written table, a read starting at the earlier disabled version failed, while an interval beginning at enablement returned the later insert.
Claim `delta.cdf.6`; runtime_observation; evidence: runtime.

**Arrow metadata schema.** The CDF base schema defines nullable _change_type as Utf8, _commit_version as UInt64 and _commit_timestamp as Timestamp(Millisecond, None). The direct query probe returns UInt64 commit versions. Inspect the actual result schema before downcasting, especially after further DataFusion expressions.
Claim `delta.cdf.7`; source_observation; evidence: cdf_schema, runtime.

## Implementation

- Pass a starting version or timestamp; use a bounded interval when reproducibility matters.
- Preserve _change_type, _commit_version and _commit_timestamp in downstream semantics.
- Partition pruning is a candidate reduction, not a substitute for a row-level residual predicate.

## Effects

- read log interval and data/change files
- produce insert/delete/preimage/postimage rows

## Errors

- Missing start, disabled/unrecorded CDF, invalid range, mapping restrictions, missing retained files and planning errors are distinct failure modes.

## Limits and unknowns

- An ORDER BY id query over the tiny update fixture encountered DataFusion SinglePartition/UnknownPartitioning(0) planning failure; the unordered query followed by caller sorting succeeded. Timestamp boundary and all enablement-transition combinations are not exhaustively tested.

## Exact contracts

- [`deltalake_core::table::DeltaTable::scan_cdf`](../operations/deltalake_core.table.DeltaTable.md#op-27518198c8dfbfd4c30b341f) — `fn scan_cdf(self) -> CdfLoadBuilder`
- [`deltalake_core::operations::load_cdf::CdfLoadBuilder::build`](../operations/deltalake_core.operations.load_cdf.CdfLoadBuilder.md#op-c42fdd7acb1de4224cf941c2) — `async fn build(&self, session: &dyn Session, filters: Option<&Arc<dyn PhysicalExpr>>) -> DeltaResult<Arc<dyn ExecutionPlan>>`
- [`deltalake_core::operations::load_cdf::CdfLoadBuilder::build_with_metrics`](../operations/deltalake_core.operations.load_cdf.CdfLoadBuilder.md#op-52940ee116097799a600f742) — `async fn build_with_metrics(&self, session: &dyn Session, filters: Option<&Arc<dyn PhysicalExpr>>, metrics: Option<ExecutionPlanMetricsSet>) -> DeltaResult<Arc<dyn ExecutionPlan>>`
- [`deltalake_core::operations::load_cdf::CdfLoadBuilder::with_allow_out_of_range`](../operations/deltalake_core.operations.load_cdf.CdfLoadBuilder.md#op-ba2fd6c37af303a8f4036c61) — `fn with_allow_out_of_range(self) -> Self`
- [`deltalake_core::delta_datafusion::cdf::scan::DeltaCdfTableProvider::try_new`](../operations/deltalake_core.delta_datafusion.cdf.scan.DeltaCdfTableProvider.md#op-76590db94c2c43f9d137eeea) — `fn try_new(cdf_builder: CdfLoadBuilder) -> DeltaResult<Self>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/operations/load_cdf.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: cdf_bounds_images_and_residual_filter, column_mapping_reads_logical_names_but_cdf_rejects_mapping, competing_marker_writers_conflict_and_cdf_enablement_is_historical
- [cdf_schema](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/delta_datafusion/cdf/mod.rs): Exact CDF metadata field definitions
  Tests: 
