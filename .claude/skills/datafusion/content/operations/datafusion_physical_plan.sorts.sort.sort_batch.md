# `datafusion_physical_plan::sorts::sort::sort_batch`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.sort.sort_batch.json).

<a id="op-3f639b3baafb525c01ea853f"></a>
## sort_batch

`function` · `datafusion_physical_plan::sorts::sort::sort_batch` · datafusion-physical-plan 55.1.0

```rust
fn sort_batch(batch: &arrow::array::RecordBatch, expressions: &datafusion_physical_expr::LexOrdering, fetch: Option<usize>) -> datafusion_common::Result<arrow::array::RecordBatch>
```

Source: `src/sorts/sort.rs:895`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
