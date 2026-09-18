# `buoyant_kernel::engine::arrow_utils::fixup_parquet_read`

Full upstream contracts; raw type trees and source locators in [structured records](buoyant_kernel.engine.arrow_utils.fixup_parquet_read.json).

<a id="op-1997b9099ec7e3dfa7cdabcb"></a>
## fixup_parquet_read

`function` · `buoyant_kernel::engine::arrow_utils::fixup_parquet_read` · buoyant_kernel 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fixup_parquet_read(batch: arrow::array::RecordBatch, requested_ordering: &[ReorderIndex], row_indexes: Option<&mut std::iter::Flatten<std::vec::IntoIter<std::ops::Range<i64>>>>, file_location: Option<&str>, target_schema: Option<&schema::SchemaRef>) -> DeltaResult<engine::arrow_data::ArrowEngineData>
```

[Exact source](https://github.com/buoyant-data/delta-kernel-rs/blob/8ba063f8f84fec222000f66d40d70911d7c79675/kernel/src/engine/arrow_utils/mod.rs#L206).

Source: `/home/paul/.cargo/git/checkouts/delta-kernel-rs-ed98d9651ec5fb51/8ba063f/kernel/src/engine/arrow_utils/mod.rs:206`. [Exact documentation build](https://github.com/buoyant-data/delta-kernel-rs/tree/8ba063f8f84fec222000f66d40d70911d7c79675).

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
