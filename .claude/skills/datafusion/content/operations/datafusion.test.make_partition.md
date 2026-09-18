# `datafusion::test::make_partition`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test.make_partition.json).

<a id="op-9c4cde558e32c6c9e2ceb4ac"></a>
## make_partition

`function` · `datafusion::test::make_partition` · datafusion 55.1.0

```rust
fn make_partition(sz: i32) -> arrow::record_batch::RecordBatch
```

Source: `src/test/mod.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Return a RecordBatch with a single Int32 array with values (0..sz)
