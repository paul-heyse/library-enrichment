# Use kernel snapshots, scans and engine interfaces at the correct layer

Use high-level DeltaTable operations for normal table work. Use the kernel and engine interfaces when implementing a scan/engine boundary and when prepared to honor its selection, schema and protocol contracts.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| DeltaTable/provider | The task is querying or mutating a Delta table | Already integrates protocol, storage and DataFusion semantics |
| Kernel Engine/EngineData | You are implementing engine evaluation/I/O or consuming kernel scan primitives | Own the engine contract and apply selection/deletion semantics correctly |
| buoyant_kernel_engine | The supplied engine implementation fits the integration | Keep its fork/feature/version identity aligned with delta-rs |

## Contract

**identity.** The package names are buoyant_kernel and buoyant_kernel_engine, used through delta_kernel/delta_kernel_default_engine aliases in delta-rs. The fork commit is part of API identity, not interchangeable with an arbitrary published delta_kernel.
Claim `delta.kernel.1`; source_observation; evidence: upstream, source.

**representation.** EngineData and FilteredEngineData carry engine-owned batches and selection semantics. A selection vector must be applied according to its contract before treating all physical rows as logical results.
Claim `delta.kernel.2`; source_observation; evidence: upstream, source.

**scope.** The capture includes internal-api-gated surfaces. Visibility under that profile does not promise stability or inclusion in a consumer with fewer features.
Claim `delta.kernel.3`; source_observation; evidence: upstream, source.

## Implementation

- Start from the kernel trait's full member contracts and choose the narrowest engine seam needed.
- Bind Arrow/kernel conversions and selection behavior to the actual package identities; use the DataFusion boundary notes when crossing engine representations.

## Effects

- engine evaluation and I/O
- selection/filter application

## Errors

- Kernel error and engine adapter error paths do not imply an already completed high-level table transaction.

## Limits and unknowns

- Custom engine correctness and every internal-api surface are unreviewed beyond the selected contracts; no complete custom engine implementation is certified.

## Exact contracts

- [`buoyant_kernel::Engine`](../operations/buoyant_kernel.Engine.md#op-144f8dad57c79b7743fd1386) — `trait Engine: AsAny`
- [`buoyant_kernel::engine_data::EngineData::apply_selection_vector`](../operations/buoyant_kernel.engine_data.EngineData.md#op-1e67ac80541d2398db9da79f) — `fn apply_selection_vector(Box<self>, selection_vector: Vec<bool>) -> DeltaResult<Box<dyn EngineData>>`
- [`buoyant_kernel::engine_data::FilteredEngineData::apply_selection_vector`](../operations/buoyant_kernel.engine_data.FilteredEngineData.md#op-c5e31214bf7fcdf905d25151) — `fn apply_selection_vector(self) -> DeltaResult<Box<dyn EngineData>>`
- [`buoyant_kernel::engine::arrow_conversion::TryIntoArrow::try_into_arrow`](../operations/buoyant_kernel.engine.arrow_conversion.TryIntoArrow.md#op-25b4a288393e65df0404d747) — `fn try_into_arrow(self) -> Result<ArrowType, ArrowError>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/kernel/mod.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
