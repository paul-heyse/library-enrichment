# `parquet::file::statistics::TypedStatistics`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.file.statistics.TypedStatistics.json).

<a id="op-78bad238f28740bab835db0b"></a>
## TypedStatistics

`type_alias` · `parquet::file::statistics::TypedStatistics` · parquet 59.3.0

```rust
type TypedStatistics<T> = ValueStatistics<<T as DataType>::T>
```

Source: `src/file/statistics.rs:502`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Typed implementation for [`Statistics`](../operations/parquet.file.statistics.Statistics.md#op-ba51f82bfe4dce01512b0440).
