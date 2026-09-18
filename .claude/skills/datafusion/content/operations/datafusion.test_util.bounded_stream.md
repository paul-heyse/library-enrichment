# `datafusion::test_util::bounded_stream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.bounded_stream.json).

<a id="op-8a368fe4575feeb5419ba061"></a>
## bounded_stream

`function` · `datafusion::test_util::bounded_stream` · datafusion 55.1.0

```rust
fn bounded_stream(record_batch: arrow::record_batch::RecordBatch, limit: usize) -> execution::SendableRecordBatchStream
```

Source: `src/test_util/mod.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Creates a bounded stream that emits the same record batch a specified number of times.
This is useful for testing purposes.
