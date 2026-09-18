# `deltalake_core::kernel::schema::cast::cast_record_batch`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.kernel.schema.cast.cast_record_batch.json).

<a id="op-ca4cbd9ec92358c4ce60b9f4"></a>
## cast_record_batch

`function` · `deltalake_core::kernel::schema::cast::cast_record_batch` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn cast_record_batch(batch: &arrow_array::RecordBatch, target_schema: arrow_schema::SchemaRef, safe: bool, add_missing: bool) -> DeltaResult<arrow_array::RecordBatch>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/kernel/schema/cast/mod.rs#L212).

Source: `crates/core/src/kernel/schema/cast/mod.rs:212`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Cast recordbatch to a new target_schema, by casting each column array
