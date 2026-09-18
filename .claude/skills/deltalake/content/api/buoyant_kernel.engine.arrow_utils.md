# `buoyant_kernel::engine::arrow_utils`

Crate `buoyant_kernel` · 14 public items · structured records in [`model/buoyant_kernel.engine.arrow_utils.json`](../model/buoyant_kernel.engine.arrow_utils.json)

## ReorderIndexTransform

`enum` · `buoyant_kernel::engine::arrow_utils::ReorderIndexTransform`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.ReorderIndexTransform.md)

Also reachable as `delta_kernel::engine::arrow_utils::ReorderIndexTransform`

```rust
enum ReorderIndexTransform
```

**Variants**: `Cast`, `Nested`, `Identity`, `Missing`, `RowIndex`, `FilePath`

**Derives**: Debug, PartialEq, StructuralPartialEq

---

## build_json_reorder_indices

`function` · `buoyant_kernel::engine::arrow_utils::build_json_reorder_indices`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.build_json_reorder_indices.md)

Also reachable as `delta_kernel::engine::arrow_utils::build_json_reorder_indices`

```rust
fn build_json_reorder_indices(schema: &schema::StructType) -> DeltaResult<Vec<ReorderIndex>>
```

Builds the [`ReorderIndex`] vec for post-processing JSON read batches.

The JSON reader is given a schema with metadata columns stripped (see [`json_arrow_schema`]).
Its output therefore has non-metadata columns at contiguous indices 0..N in schema order.
This function maps those source indices -- and any metadata column specs -- into a
`Vec<ReorderIndex>` that `reorder_struct_array` can use to produce the final batch with
every column at its correct position.

Build the index vec once per schema (e.g. once per file); apply it to every batch produced
by the reader via `reorder_struct_array`.

# Companion function
- Use [`json_arrow_schema`] to strip metadata columns before passing the schema to the JSON
  reader.

---

## fix_nested_null_masks

`function` · `buoyant_kernel::engine::arrow_utils::fix_nested_null_masks`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.fix_nested_null_masks.md)

Also reachable as `buoyant_kernel::engine::arrow_data::fix_nested_null_masks`, `delta_kernel::engine::arrow_utils::fix_nested_null_masks`

```rust
fn fix_nested_null_masks(batch: arrow::array::StructArray) -> arrow::array::StructArray
```

Use this function to recursively compute properly unioned null masks for all nested
columns of a record batch, making it safe to project out and consume nested columns.

Arrow does not guarantee that the null masks associated with nested columns are accurate --
instead, the reader must consult the union of logical null masks the column and all
ancestors. The parquet reader stopped doing this automatically as of arrow-53.3, for example.

---

## fixup_json_read

`function` · `buoyant_kernel::engine::arrow_utils::fixup_json_read`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.fixup_json_read.md)

Also reachable as `delta_kernel::engine::arrow_utils::fixup_json_read`

```rust
fn fixup_json_read(batch: arrow::array::RecordBatch, reorder_indices: &[ReorderIndex], file_location: &str) -> DeltaResult<engine::arrow_data::ArrowEngineData>
```

Applies post-processing to data read from a JSON file. Inserts synthesized metadata columns
(e.g. [`MetadataColumnSpec::FilePath`]) at the positions specified by `reorder_indices`.

`reorder_indices` should be built once per schema via [`build_json_reorder_indices`] and
reused for every batch from the same file.

---

## fixup_parquet_read

`function` · `buoyant_kernel::engine::arrow_utils::fixup_parquet_read`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.fixup_parquet_read.md)

Also reachable as `delta_kernel::engine::arrow_utils::fixup_parquet_read`

```rust
fn fixup_parquet_read(batch: arrow::array::RecordBatch, requested_ordering: &[ReorderIndex], row_indexes: Option<&mut std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i64>>>>, file_location: Option<&str>, target_schema: Option<&schema::SchemaRef>) -> DeltaResult<engine::arrow_data::ArrowEngineData>
```

Applies post-processing to data read from parquet files. This includes `reorder_struct_array` to
ensure schema compatibility, as well as `fix_nested_null_masks` to ensure that leaf columns have
accurate null masks that row visitors rely on for correctness.
`row_indexes` are passed through to `reorder_struct_array`.
`file_location` is used to populate file metadata columns if requested.

If `target_schema` is provided, rewrites the batch's schema wrappers to match the kernel
schema via `apply_schema_to_struct`. Specifically, at every nesting level (struct child, list
element, map key/value):

- field names are taken from the kernel schema (producer names are kept only for list element
  and map key/value positions, where the kernel `ArrayType`/`MapType` is unnamed);
- field nullability is taken from the kernel schema;
- field metadata is replaced wholesale with kernel-derived metadata (translating
  `parquet.field.id` to `PARQUET:field_id` and propagating kernel-only annotations such as
  `delta.typeChanges`);
- if both the source and kernel fields carry a `PARQUET:field_id` and they disagree, the call
  errors (defense against malformed inputs);
- top-level `RecordBatch::schema().metadata()` is not preserved (the rebuilt schema is created
  via `ArrowSchema::new`).

**Type validation.** `apply_schema_to_struct` runs `ensure_data_types(.., Full)` at every
primitive leaf. This is safe because `reorder_struct_array` above has already resolved every
`DataTypeCompat::NeedsCast` into an actual `arrow::compute::cast`, so post-reorder leaf types
are `Identical` to the kernel target.

**Cost.** O(F) per batch where F is the total number of fields (including nested). Row data
(Arrow buffers, offsets, null buffers) is shared via `Arc` and never copied.

---

## generate_mask

`function` · `buoyant_kernel::engine::arrow_utils::generate_mask`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.generate_mask.md)

Also reachable as `delta_kernel::engine::arrow_utils::generate_mask`

```rust
fn generate_mask(_requested_schema: &schema::SchemaRef, _parquet_schema: &arrow::datatypes::SchemaRef, parquet_physical_schema: &parquet::schema::types::SchemaDescriptor, indices: &[usize]) -> Option<parquet::arrow::ProjectionMask>
```

Create a mask that will only select the specified indices from the parquet. `indices` can be
computed from a [`Schema`] using [`get_requested_indices`]

---

## get_requested_indices

`function` · `buoyant_kernel::engine::arrow_utils::get_requested_indices`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.get_requested_indices.md)

Also reachable as `delta_kernel::engine::arrow_utils::get_requested_indices`

```rust
fn get_requested_indices(requested_schema: &schema::SchemaRef, parquet_schema: &arrow::datatypes::SchemaRef) -> DeltaResult<(Vec<usize>, Vec<ReorderIndex>)>
```

Get the indices in `parquet_schema` of the specified columns in `requested_schema`. This returns
a tuple of (mask_indices: Vec<parquet_schema_index>, reorder_indices:
Vec<requested_index>). `mask_indices` is used for generating the mask for reading from the
parquet file, and simply contains an entry for each index we wish to select from the parquet
file set to the index of the requested column in the parquet. `reorder_indices` is used for
re-ordering. See the documentation for [`ReorderIndex`] to understand what each element in the
returned array means.

---

## json_arrow_schema

`function` · `buoyant_kernel::engine::arrow_utils::json_arrow_schema`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.json_arrow_schema.md)

Also reachable as `delta_kernel::engine::arrow_utils::json_arrow_schema`

```rust
fn json_arrow_schema(schema: &schema::StructType) -> DeltaResult<arrow::datatypes::Schema>
```

Builds an Arrow [`ArrowSchema`] from `schema` containing only the "real" JSON columns,
omitting any fields annotated with [`MetadataColumnSpec`].

Pass the returned schema to Arrow's JSON reader; then call [`build_json_reorder_indices`]
once on the same schema and apply `reorder_struct_array` to each resulting batch to
insert the synthesized metadata columns at their correct positions.

---

## make_arrow_error

`function` · `buoyant_kernel::engine::arrow_utils::make_arrow_error`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.make_arrow_error.md)

Also reachable as `delta_kernel::engine::arrow_utils::make_arrow_error`

```rust
fn make_arrow_error(s: impl Into<String>) -> Error
```

Create an [`Error::Arrow`] with a backtrace from the given message.

---

## ordering_needs_row_indexes

`function` · `buoyant_kernel::engine::arrow_utils::ordering_needs_row_indexes`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.ordering_needs_row_indexes.md)

Also reachable as `delta_kernel::engine::arrow_utils::ordering_needs_row_indexes`

```rust
fn ordering_needs_row_indexes(requested_ordering: &[ReorderIndex]) -> bool
```

Check if an ordering requires row index computation.

The function only checks if a RowIndex transform is present at the top-level, since metadata
columns are not allowed to be nested.

---

## parse_json

`function` · `buoyant_kernel::engine::arrow_utils::parse_json`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.parse_json.md)

Also reachable as `buoyant_kernel::engine::parse_json`, `delta_kernel::engine::arrow_utils::parse_json`, `deltalake::kernel::engine::parse_json`, `deltalake_core::kernel::engine::parse_json`

```rust
fn parse_json(json_strings: Box<dyn EngineData>, schema: schema::SchemaRef) -> DeltaResult<Box<dyn EngineData>>
```

Parse a column of JSON strings into a typed `RecordBatch` matching `schema`. N input
rows produce N output rows.

Arrow lacks the functionality to json-parse a string column into a struct column, so we
implement it here.

Failure-prone primitive leaves (`Timestamp`, `TimestampNtz`, `Date`, `Decimal`) produce
per-cell NULL when the typed decoder rejects a value (extended-year timestamps,
decimals that overflow the declared precision, etc.). Other leaf type mismatches still
surface as batch-level errors.

---

## to_json_bytes

`function` · `buoyant_kernel::engine::arrow_utils::to_json_bytes`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.to_json_bytes.md)

Also reachable as `buoyant_kernel::engine::to_json_bytes`, `delta_kernel::engine::arrow_utils::to_json_bytes`, `deltalake::kernel::engine::to_json_bytes`, `deltalake_core::kernel::engine::to_json_bytes`

```rust
fn to_json_bytes(data: impl Iterator<Item = DeltaResult<engine_data::FilteredEngineData>> + Send) -> DeltaResult<Vec<u8>>
```

serialize an arrow RecordBatch to a JSON string by appending to a buffer.

---

## ReorderIndex

`struct` · `buoyant_kernel::engine::arrow_utils::ReorderIndex`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.ReorderIndex.md)

Also reachable as `delta_kernel::engine::arrow_utils::ReorderIndex`

```rust
struct ReorderIndex
```

**Fields**: `index`

**Derives**: Debug, PartialEq, StructuralPartialEq

Reordering is specified as a tree. Each level is a vec of `ReorderIndex`s. Each element's
position represents a column that will be in the read parquet data at that level and
position. The `index` of the element is the position that the column should appear in the final
output. The `transform` indicates what, if any, transforms are needed. See the docs for
[`ReorderIndexTransform`] for the meaning.

---

## RowIndexBuilder

`struct` · `buoyant_kernel::engine::arrow_utils::RowIndexBuilder`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.arrow_utils.RowIndexBuilder.md)

Also reachable as `delta_kernel::engine::arrow_utils::RowIndexBuilder`

```rust
struct RowIndexBuilder
```

**Methods** (3)

```rust
fn build(self) -> DeltaResult<std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i64>>>>
fn new(row_groups: &[RowGroupMetaData]) -> Self
fn select_row_groups(&mut self, ordinals: &[usize])
```

Prepares to enumerate row indexes of rows in a parquet file, accounting for row group skipping.

---
