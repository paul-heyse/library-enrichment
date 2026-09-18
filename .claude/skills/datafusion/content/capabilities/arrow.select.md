# Choose a row or column selection representation

Select by the representation you have: mask, indices, contiguous range, or column positions. Nullable indices, non-nullable fields and zero-column batches change the valid composition.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| filter_record_batch | Keep true mask positions in source order | False and null mask positions are excluded. |
| take / take_arrays / take_record_batch | Gather in requested order, including repeated indices | Array take and multi-array take_arrays expose checked bounds; the batch helper does not expose options. |
| RecordBatch::slice / project | Contiguous rows / selected columns | Slice may retain shared source allocations; projection does not select rows. |
| DataFrame filter/select | Selection belongs within an optimizable relation | Avoid hiding pushdown opportunities by selecting only after collection. |

## Contract

**shape.** filter_record_batch takes a batch and BooleanArray and returns Result<RecordBatch>. Array take takes values, integer indices and optional TakeOptions and returns Result<ArrayRef>. Mask length must fit the source selection domain.
Claim `arrow.select.shape`; upstream_contract_interpretation; evidence: upstream.

**null-order.** Filter keeps source order and excludes false/null mask entries. Take preserves index order and multiplicity; null indices introduce null values.
Claim `arrow.select.null-order`; upstream_contract_interpretation; evidence: upstream.

**bounds.** TakeOptions.check_bounds=true requests checked errors. With unchecked indices, out-of-bounds access can panic despite Result. take_record_batch calls array take with None in this pin.
Claim `arrow.select.bounds`; upstream_contract_interpretation; evidence: upstream, source.

**schema.** Gathering null indices may violate a non-nullable output field. RecordBatch construction checks top-level nullability. Choose compatible output fields; an array result alone does not carry schema metadata.
Claim `arrow.select.schema`; upstream_contract_interpretation; evidence: upstream.

**empty-shape.** A zero-column batch can have rows via RecordBatchOptions.row_count. filter_record_batch retains the selected count; take_record_batch in 59.3.0 reconstructs with try_new and errors. For zero columns, validate non-null indices against input row count and build the output with explicit indices.len() row count.
Claim `arrow.select.empty-shape`; upstream_contract_interpretation; evidence: upstream, source.

**ownership.** Slices and some selection paths share buffers. Logical output size does not bound retained allocation size.
Claim `arrow.select.ownership`; upstream_contract_interpretation; evidence: upstream.

## Implementation

- Choose mask/index/range/column semantics before the API.
- Check indices and output nullability, preserving field/schema metadata deliberately.
- For zero columns, use explicit row-count construction and retain the same index-domain validation.
- Use take_arrays for a shared gather across columns before constructing a batch with the desired output fields/options.

## Limits and unknowns

- Tests cover Int32/UInt32 and tested metadata; nested/view layouts and every datatype are not qualified.

## Exact contracts

- [`arrow_select::filter::filter_record_batch`](../operations/arrow_select.filter.filter_record_batch.md#op-50a068de42a854681b747251) — `fn filter_record_batch(record_batch: &RecordBatch, predicate: &BooleanArray) -> Result<RecordBatch, ArrowError>`
- [`arrow_select::take::take`](../operations/arrow_select.take.take.md#op-4197d454d308f4ceadb20600) — `fn take(values: &dyn Array, indices: &dyn Array, options: Option<TakeOptions>) -> Result<ArrayRef, arrow_schema::ArrowError>`
- [`arrow_select::take::take_record_batch`](../operations/arrow_select.take.take_record_batch.md#op-929c00ffe22d86d208c23db7) — `fn take_record_batch(record_batch: &RecordBatch, indices: &dyn Array) -> Result<RecordBatch, arrow_schema::ArrowError>`
- [`arrow_array::record_batch::RecordBatch::slice`](../operations/arrow_array.record_batch.RecordBatch.md#op-a79af188f3899a383689384f) — `fn slice(&self, offset: usize, length: usize) -> RecordBatch`
- [`arrow_array::record_batch::RecordBatch::project`](../operations/arrow_array.record_batch.RecordBatch.md#op-124dc8bac9a1c739ad939083) — `fn project(&self, indices: &[usize]) -> Result<RecordBatch, ArrowError>`
- [`arrow_array::record_batch::RecordBatch::try_new_with_options`](../operations/arrow_array.record_batch.RecordBatch.md#op-b3154cbd0c6729748077fc9a) — `fn try_new_with_options(schema: SchemaRef, columns: Vec<ArrayRef>, options: &RecordBatchOptions) -> Result<Self, ArrowError>`
- [`arrow_select::take::take_arrays`](../operations/arrow_select.take.take_arrays.md#op-b8024b1f5c1204e51b646dba) — `fn take_arrays(arrays: &[ArrayRef], indices: &dyn Array, options: Option<TakeOptions>) -> Result<Vec<ArrayRef>, arrow_schema::ArrowError>`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [probe](../../skill_improvement/evidence/implementation/probe-results.json): Only the named assertions in the recorded Cargo profile.
  Tests: filter_null_is_not_selected_and_metadata_is_retained, take_preserves_requested_order_duplicates_and_null_indices, empty_schema_exposes_different_batch_selection_contracts, batch_construction_checks_top_level_nullability
- [source](../../skill_improvement/evidence/sources/arrow-select-59.3.0/src/take.rs): Selected exact published source; source-manifest.json retains provenance.
  Tests: 
