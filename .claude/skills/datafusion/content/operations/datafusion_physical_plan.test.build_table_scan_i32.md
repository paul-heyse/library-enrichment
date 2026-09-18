# `datafusion_physical_plan::test::build_table_scan_i32`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.build_table_scan_i32.json).

<a id="op-144810b2dada31501558316d"></a>
## build_table_scan_i32

`function` · `datafusion_physical_plan::test::build_table_scan_i32` · datafusion-physical-plan 55.1.0

```rust
fn build_table_scan_i32(a: (&str, &Vec<i32>), b: (&str, &Vec<i32>), c: (&str, &Vec<i32>)) -> std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/test.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns memory table scan wrapped around record batch with 3 columns of i32
