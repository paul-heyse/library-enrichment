# `datafusion_physical_plan::test::mem_exec`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.test.mem_exec.json).

<a id="op-8e76de3ea4de90df5a29ef23"></a>
## mem_exec

`function` · `datafusion_physical_plan::test::mem_exec` · datafusion-physical-plan 55.1.0

```rust
fn mem_exec(partitions: usize) -> TestMemoryExec
```

Source: `src/test.rs:495`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Returns a `DataSourceExec` that scans `partitions` of 100 batches each
