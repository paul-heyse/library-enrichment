# `buoyant_kernel::engine::parquet_row_group_skipping`

Crate `buoyant_kernel` · 1 public items · structured records in [`model/buoyant_kernel.engine.parquet_row_group_skipping.json`](../model/buoyant_kernel.engine.parquet_row_group_skipping.json)

## ParquetRowGroupSkipping

`trait` · `buoyant_kernel::engine::parquet_row_group_skipping::ParquetRowGroupSkipping`
[Full member contracts, output types and access classification](../operations/buoyant_kernel.engine.parquet_row_group_skipping.ParquetRowGroupSkipping.md)

Also reachable as `delta_kernel::engine::parquet_row_group_skipping::ParquetRowGroupSkipping`

```rust
trait ParquetRowGroupSkipping
```

**Implementors** (1)

- `parquet::arrow::arrow_reader::ArrowReaderBuilder`

**Methods** (2)

```rust
fn with_checkpoint_row_group_filter(self, predicate: &Predicate, partition_columns: &HashSet<String>, row_indexes: Option<&mut RowIndexBuilder>) -> Self
fn with_row_group_filter(self, predicate: &Predicate, row_indexes: Option<&mut RowIndexBuilder>) -> Self
```

An extension trait for [`ArrowReaderBuilder`] that injects row group skipping capability.

---
