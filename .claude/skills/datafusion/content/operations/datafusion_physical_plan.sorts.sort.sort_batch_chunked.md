# `datafusion_physical_plan::sorts::sort::sort_batch_chunked`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.sorts.sort.sort_batch_chunked.json).

<a id="op-952008ba36e7698f7b36358f"></a>
## sort_batch_chunked

`function` · `datafusion_physical_plan::sorts::sort::sort_batch_chunked` · datafusion-physical-plan 55.1.0

```rust
fn sort_batch_chunked(batch: &arrow::array::RecordBatch, expressions: &datafusion_physical_expr::LexOrdering, batch_size: usize) -> datafusion_common::Result<Vec<arrow::array::RecordBatch>>
```

Source: `src/sorts/sort.rs:919`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Sort a batch and return the result as multiple batches of size `batch_size`.
This is useful when you want to avoid creating one large sorted batch in memory,
and instead want to process the sorted data in smaller chunks.
