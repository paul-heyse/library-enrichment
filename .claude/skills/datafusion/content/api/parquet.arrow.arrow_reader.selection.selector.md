# `parquet::arrow::arrow_reader::selection::selector`

Crate `parquet` · 1 public items · structured records in [`model/parquet.arrow.arrow_reader.selection.selector.json`](../model/parquet.arrow.arrow_reader.selection.selector.json)

## RowSelector

`struct` · `parquet::arrow::arrow_reader::selection::selector::RowSelector`

Also reachable as `parquet::arrow::arrow_reader::RowSelector`

```rust
struct RowSelector
```

**Fields**: `row_count`, `skip`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn select(row_count: usize) -> Self
fn skip(row_count: usize) -> Self
```

[Full member, field, variant and typed contracts](../operations/parquet.arrow.arrow_reader.selection.selector.RowSelector.md).


[`RowSelection`] is a collection of [`RowSelector`] used to skip rows when
scanning a parquet file

[`RowSelection`]: crate::arrow::arrow_reader::RowSelection

---
