# `deltalake_core::writer::utils::record_batch_from_message`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.writer.utils.record_batch_from_message.json).

<a id="op-1df5bad8a87c2ea9c9527196"></a>
## record_batch_from_message

`function` · `deltalake_core::writer::utils::record_batch_from_message` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn record_batch_from_message(arrow_schema: std::sync::Arc<arrow_schema::Schema>, json: &[serde_json::Value]) -> errors::DeltaResult<arrow_array::RecordBatch>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/writer/utils.rs#L62).

Source: `crates/core/src/writer/utils.rs:62`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Convert a vector of json values to a RecordBatch
