# Distinguish batch validity, relational schema and metadata propagation

Arrow Schema/Field describe physical arrays and metadata; DFSchema adds relational qualifiers. Batch construction validates structural contracts, not application meaning. Test metadata through the exact operator path.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| RecordBatch constructors | Assemble arrays with explicit shape/nullability | Zero-column rows need explicit row count; domain validation is separate. |
| DFSchema | Resolve qualified relational columns | Do not discard qualifiers before resolution when names collide. |
| DefaultPhysicalExprAdapter | Adapt physical expressions to table/file schema differences | Inspect missing-column, casting and nested-field policy; not arbitrary domain schema migration. |

## Contract

**validation.** RecordBatch::try_new checks field/column count, datatypes and row lengths, including top-level non-nullable fields. It does not validate domain units, keys or every semantic invariant.
Claim `arrow.schema.validation`; upstream_contract_interpretation; evidence: upstream.

**metadata.** Extension annotations reside in field metadata. Direct filter preserves the tested field/schema metadata; that does not prove UNION, projection, cast or custom UDF paths preserve it.
Claim `arrow.schema.metadata`; upstream_contract_interpretation; evidence: upstream.

**shape.** Schema alignment must account for field order, type, nullability and nested structure, not only names. DFSchema qualifiers and physical Schema are different representations.
Claim `arrow.schema.shape`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Specify which metadata is contractual versus advisory.
- Carry or reconstruct fields explicitly when kernels return only arrays.
- Probe each consequential logical-to-physical operator boundary, including schema plus values.

## Limits and unknowns

- Nested metadata propagation and every adapter conversion remain contract-backed discovery, not an exhaustive runtime matrix.

## Exact contracts

- [`arrow_array::record_batch::RecordBatch::try_new`](../operations/arrow_array.record_batch.RecordBatch.md#op-dd87dabf7102df1ff3d8ea95) — `fn try_new(schema: SchemaRef, columns: Vec<ArrayRef>) -> Result<Self, ArrowError>`
- [`arrow_array::record_batch::RecordBatch::try_new_with_options`](../operations/arrow_array.record_batch.RecordBatch.md#op-b3154cbd0c6729748077fc9a) — `fn try_new_with_options(schema: SchemaRef, columns: Vec<ArrayRef>, options: &RecordBatchOptions) -> Result<Self, ArrowError>`
- [`arrow_schema::field::Field`](../operations/arrow_schema.field.Field.md#op-66eb7ef45bcc129b0a0189cf) — `struct Field`
- [`datafusion_common::dfschema::DFSchema`](../operations/datafusion_common.dfschema.DFSchema.md#op-8af5adf56b372aa63b81eb98) — `struct DFSchema`
- [`datafusion_physical_expr_adapter::schema_rewriter::DefaultPhysicalExprAdapter`](../operations/datafusion_physical_expr_adapter.schema_rewriter.DefaultPhysicalExprAdapter.md#op-0786dd5cec45ace7db749372) — `struct DefaultPhysicalExprAdapter`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: batch_construction_checks_top_level_nullability, filter_null_is_not_selected_and_metadata_is_retained
