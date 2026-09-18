# `datafusion_physical_plan::test::make_partition`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.make_partition.json).

<a id="op-c67f4a946a13793fc38bcc7b"></a>
## make_partition

`function` · `datafusion_physical_plan::test::make_partition` · datafusion-physical-plan 55.1.0

```rust
fn make_partition(sz: i32) -> arrow::array::RecordBatch
```

Source: `src/test.rs:459`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Return a RecordBatch with a single Int32 array with values (0..sz) in a field named "i"
