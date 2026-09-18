# `deltalake_core::errors::DeltaTableError::Parquet`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.errors.DeltaTableError.Parquet.json).

<a id="op-5e67decdb5ad99b22c09e1c6"></a>
## source

`struct_field` · `deltalake_core::errors::DeltaTableError::Parquet::source` · deltalake-core 1.0.0+58f07cd6

Access: **internal_field**. Canonical source location is not automatically a valid import path.

```rust
source: parquet::errors::ParquetError
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/errors.rs#L49).

Source: `crates/core/src/errors.rs:49`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parquet error details returned when reading the checkpoint failed.
