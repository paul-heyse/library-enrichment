# `parquet::errors::Result`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.errors.Result.json).

<a id="op-6f903a0912ada812d3eef24c"></a>
## Result

`type_alias` · `parquet::errors::Result` · parquet 59.3.0

```rust
type Result<T, E = ParquetError> = result::Result<T, E>
```

Source: `src/errors.rs:151`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A specialized `Result` for Parquet errors.
