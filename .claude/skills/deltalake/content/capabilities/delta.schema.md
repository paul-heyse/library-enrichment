# Preserve schema meaning across Arrow, Delta and schema evolution

Distinguish representation conversion, value casting, schema normalization and table evolution. A successful Arrow conversion does not by itself authorize a Delta write or preserve every Arrow datatype exactly.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| Existing compatible schema | Appending values without changing table metadata | Check actual nullability, nested fields and protocol-dependent types |
| SchemaMode::Merge | Additive source fields should evolve the table | Existing rows need compatible missing-field/null behavior; mapped tables have extra restrictions |
| SchemaMode::Overwrite | A full replacement also changes schema | Requires overwrite mode and operation-specific validation |
| Arrow/kernel conversion traits | You need a schema at the integration boundary | Conversion errors and normalization are separate from row-value casting |

## Contract

**cast policy.** WriteBuilder::with_cast_safety(true) converted invalid string-to-integer values to null in the probe; false returned an error. Merge/update spell this policy with_safe_cast.
Claim `delta.schema.1`; runtime_observation; evidence: runtime.

**evolution.** A wider batch failed without schema mode, then SchemaMode::Merge added its field and supplied null for an old row.
Claim `delta.schema.2`; runtime_observation; evidence: runtime.

**fidelity.** Check nested child nullability, decimal precision/scale, signedness, timestamp units/timezone and mapping metadata explicitly. Normalized schema identity is not byte-identical Arrow schema preservation.
Claim `delta.schema.3`; source_observation; evidence: upstream, source.

**fidelity.** The nested decimal field retained precision/scale and nullability through normalized kernel/Arrow conversion. In the tested profile a nanosecond UTC schema normalized to microseconds, and UInt64 schema conversion round-tripped as Int64; successful conversion is not signedness or timestamp-unit preservation.
Claim `delta.schema.4`; runtime_observation; evidence: runtime.

## Implementation

- Inspect both schema and values after conversion; do not infer losslessness from trait names.
- Choose cast failure behavior according to whether null substitution is acceptable to the application.
- Keep the nanosecond Cargo feature and table protocol feature separate; compare the consumer profile with the documentation capture.

## Effects

- cast values
- possibly update table schema/protocol

## Errors

- Unsupported Arrow types, incompatible field changes, invalid values and missing protocol features fail at different boundaries.

## Limits and unknowns

- The local controls cover additive nullable fields, string/integer casts, a nested decimal schema, unsigned schema conversion and timestamp normalization. Every value-range, timezone and nested evolution combination is not certified.

## Exact contracts

- [`deltalake_core::operations::write::SchemaMode`](../operations/deltalake_core.operations.write.SchemaMode.md#op-d1bf43766000f07558d3ef98) — `enum SchemaMode`
- [`deltalake_core::operations::write::WriteBuilder::with_cast_safety`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-8c216c90e09068c23b8f3afa) — `fn with_cast_safety(self, safe: bool) -> Self`
- [`deltalake_core::operations::merge::MergeBuilder::with_safe_cast`](../operations/deltalake_core.operations.merge.MergeBuilder.md#op-1e68915a42890152686e34b6) — `fn with_safe_cast(self, safe_cast: bool) -> Self`
- [`deltalake_core::kernel::schema::cast::normalize_for_delta`](../operations/deltalake_core.kernel.schema.cast.normalize_for_delta.md#op-6601c2591b6129782f1a8b98) — `fn normalize_for_delta(schema: &arrow_schema::SchemaRef) -> arrow_schema::SchemaRef`
- [`buoyant_kernel::engine::arrow_conversion::TryIntoKernel::try_into_kernel`](../operations/buoyant_kernel.engine.arrow_conversion.TryIntoKernel.md#op-4e6d33212c1e0b1131f6fa1a) — `fn try_into_kernel(self) -> Result<KernelType, ArrowError>`
- [`buoyant_kernel::engine::arrow_conversion::TryIntoArrow::try_into_arrow`](../operations/buoyant_kernel.engine.arrow_conversion.TryIntoArrow.md#op-25b4a288393e65df0404d747) — `fn try_into_arrow(self) -> Result<ArrowType, ArrowError>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/operations/write/mod.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: schema_merge_and_cast_error_policy, nested_schema_and_timestamp_normalization_have_explicit_loss_boundaries
