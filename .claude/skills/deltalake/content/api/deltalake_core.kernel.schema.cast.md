# `deltalake_core::kernel::schema::cast`

Crate `deltalake-core` · 3 public items · structured records in [`model/deltalake_core.kernel.schema.cast.json`](../model/deltalake_core.kernel.schema.cast.json)

## cast_record_batch

`function` · `deltalake_core::kernel::schema::cast::cast_record_batch`

Also reachable as `deltalake::cast::cast_record_batch`, `deltalake::cast_record_batch`, `deltalake::kernel::cast::cast_record_batch`, `deltalake::kernel::cast_record_batch`, `deltalake::kernel::schema::cast::cast_record_batch`, `deltalake::kernel::schema::cast_record_batch`, `deltalake::schema::cast_record_batch`, `deltalake_core::cast::cast_record_batch` (+5 more)

```rust
fn cast_record_batch(batch: &arrow_array::RecordBatch, target_schema: arrow_schema::SchemaRef, safe: bool, add_missing: bool) -> DeltaResult<arrow_array::RecordBatch>
```

Cast recordbatch to a new target_schema, by casting each column array

---

## normalize_for_delta

`function` · `deltalake_core::kernel::schema::cast::normalize_for_delta`

Also reachable as `deltalake::cast::normalize_for_delta`, `deltalake::kernel::cast::normalize_for_delta`, `deltalake::kernel::normalize_for_delta`, `deltalake::kernel::schema::cast::normalize_for_delta`, `deltalake::kernel::schema::normalize_for_delta`, `deltalake::normalize_for_delta`, `deltalake::schema::normalize_for_delta`, `deltalake_core::cast::normalize_for_delta` (+5 more)

```rust
fn normalize_for_delta(schema: &arrow_schema::SchemaRef) -> arrow_schema::SchemaRef
```

Normalize an Arrow schema so it can be safely written to a Delta table.

Delta does not support all Arrow types verbatim (for example nanosecond timestamps);
this rewrites such fields to the closest Delta-compatible representation, returning the
original schema untouched when no changes are required.

---

## set_cast_nanos_timestamps_to_micros

`function` · `deltalake_core::kernel::schema::cast::set_cast_nanos_timestamps_to_micros`

Also reachable as `deltalake::cast::set_cast_nanos_timestamps_to_micros`, `deltalake::kernel::cast::set_cast_nanos_timestamps_to_micros`, `deltalake::kernel::schema::cast::set_cast_nanos_timestamps_to_micros`, `deltalake::kernel::schema::set_cast_nanos_timestamps_to_micros`, `deltalake::kernel::set_cast_nanos_timestamps_to_micros`, `deltalake::schema::set_cast_nanos_timestamps_to_micros`, `deltalake::set_cast_nanos_timestamps_to_micros`, `deltalake_core::cast::set_cast_nanos_timestamps_to_micros` (+5 more)

```rust
fn set_cast_nanos_timestamps_to_micros(cast: bool)
```

Set whether casting nanosecond timestamps to microsecond timestamps happens.

---
