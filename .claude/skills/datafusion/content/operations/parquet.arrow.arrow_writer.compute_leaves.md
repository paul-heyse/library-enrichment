# `parquet::arrow::arrow_writer::compute_leaves`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.arrow_writer.compute_leaves.json).

<a id="op-10046d3986fa82e2b72240d8"></a>
## compute_leaves

`function` · `parquet::arrow::arrow_writer::compute_leaves` · parquet 59.3.0

```rust
fn compute_leaves(field: &arrow_schema::Field, array: &arrow_array::ArrayRef) -> errors::Result<Vec<ArrowLeafColumn>>
```

Source: `src/arrow/arrow_writer/mod.rs:935`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Computes the [`ArrowLeafColumn`](../operations/parquet.arrow.arrow_writer.ArrowLeafColumn.md#op-3606dfdb8cca5efdff1e83c8) for a potentially nested [`ArrayRef`](../operations/arrow_array.array.ArrayRef.md#op-657cc3ff3d24afcacda590e1)

This function can be used along with [`get_column_writers`](../operations/parquet.arrow.arrow_writer.get_column_writers.md#op-a6e43915d94077331c016090) to encode
individual columns in parallel. See example on [`ArrowColumnWriter`](../operations/parquet.arrow.arrow_writer.ArrowColumnWriter.md#op-b2f79c40222ecdd56db8aa79)
