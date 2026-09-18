# `datafusion_physical_plan::test::scan_partitioned`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.scan_partitioned.json).

<a id="op-1f3b4d6cbaf5a93f4f793701"></a>
## scan_partitioned

`function` · `datafusion_physical_plan::test::scan_partitioned` · datafusion-physical-plan 55.1.0

```rust
fn scan_partitioned(partitions: usize) -> std::sync::Arc<dyn ExecutionPlan>
```

Source: `src/test.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a `DataSourceExec` that scans `partitions` of 100 batches each
