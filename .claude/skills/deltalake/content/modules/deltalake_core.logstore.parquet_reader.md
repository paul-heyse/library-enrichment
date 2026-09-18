# `deltalake_core::logstore::parquet_reader`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.logstore.parquet_reader.json).

<a id="op-c456f6fecadcb67b501e5720"></a>
## parquet_reader

`module` · `deltalake_core::logstore::parquet_reader` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod parquet_reader
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/logstore/parquet_reader.rs#L1).

Source: `crates/core/src/logstore/parquet_reader.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Parquet reader for object storage
Parquet Object Reader

A custom implementation of [`AsyncFileReader`] that reads Parquet files
from an [`ObjectStore`]. This is a simplified version of the deprecated
`ParquetObjectReader` struct from the parquet crate.

See: <https://docs.rs/parquet/latest/parquet/arrow/async_reader/trait.AsyncFileReader.html>

Unresolved upstream links (retained, not inferred): ``ObjectStore``.
