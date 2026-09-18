# `datafusion_physical_plan::test::build_table_i32_two_cols`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.build_table_i32_two_cols.json).

<a id="op-760aee72425df21f7347f6f7"></a>
## build_table_i32_two_cols

`function` · `datafusion_physical_plan::test::build_table_i32_two_cols` · datafusion-physical-plan 55.1.0

```rust
fn build_table_i32_two_cols(a: (&str, &Vec<i32>), b: (&str, &Vec<i32>)) -> arrow::array::RecordBatch
```

Source: `src/test.rs:428`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns record batch with 2 columns of i32 in memory
