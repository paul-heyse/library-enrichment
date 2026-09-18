# `datafusion_datasource::write::BatchSerializer`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.write.BatchSerializer.json).

<a id="op-bef7b2a88810e075ba82d65a"></a>
## BatchSerializer

`trait` · `datafusion_datasource::write::BatchSerializer` · datafusion-datasource 55.1.0

```rust
trait BatchSerializer: Sync + Send
```

Source: `src/write/mod.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

A trait that defines the methods required for a RecordBatch serializer.

<a id="op-574ef4c7d60d36b30efae770"></a>
## serialize

`function` · `datafusion_datasource::write::BatchSerializer::serialize` · datafusion-datasource 55.1.0

```rust
fn serialize(&self, batch: RecordBatch, initial: bool) -> Result<Bytes>
```

Source: `src/write/mod.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Asynchronously serializes a `RecordBatch` and returns the serialized bytes.
Parameter `initial` signals whether the given batch is the first batch.
This distinction is important for certain serializers (like CSV).
