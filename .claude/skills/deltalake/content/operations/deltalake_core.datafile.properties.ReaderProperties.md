# `deltalake_core::datafile::properties::ReaderProperties`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.datafile.properties.ReaderProperties.json).

<a id="op-5e5723840bb14ba7b2b674be"></a>
## ReaderProperties

`struct` · `deltalake_core::datafile::properties::ReaderProperties` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct ReaderProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/properties.rs#L11).

Source: `crates/core/src/datafile/properties.rs:11`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Engine-agnostic parquet read configuration for a Delta scan.

<a id="op-321f7c00fd4bc49dfe69fd94"></a>
## clone

`function` · `deltalake_core::datafile::properties::ReaderProperties::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> ReaderProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/properties.rs#L10).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::properties::ReaderProperties", "path": "ReaderProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10, 10], "end": [10, 15], "filename": "crates/core/src/datafile/properties.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/datafile/properties.rs:10`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-159b646dcb950204f8be7dc1"></a>
## default

`function` · `deltalake_core::datafile::properties::ReaderProperties::default` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn default() -> ReaderProperties
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/properties.rs#L10).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::properties::ReaderProperties", "path": "ReaderProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10, 24], "end": [10, 31], "filename": "crates/core/src/datafile/properties.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `crates/core/src/datafile/properties.rs:10`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-704b25dbbc497f2e27735d03"></a>
## fmt

`function` · `deltalake_core::datafile::properties::ReaderProperties::fmt` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/properties.rs#L10).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::properties::ReaderProperties", "path": "ReaderProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10, 17], "end": [10, 22], "filename": "crates/core/src/datafile/properties.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `crates/core/src/datafile/properties.rs:10`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f636585da772e4b0bc846919"></a>
## to_table_parquet_options

`function` · `deltalake_core::datafile::properties::ReaderProperties::to_table_parquet_options` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn to_table_parquet_options(&self, session: &dyn datafusion::catalog::Session) -> datafusion::config::TableParquetOptions
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/datafile/properties.rs#L17).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::datafile::properties::ReaderProperties", "path": "ReaderProperties"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [14, 1], "end": [26, 2], "filename": "crates/core/src/datafile/properties.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/datafile/properties.rs:17`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Build DataFusion's `TableParquetOptions` for a `ParquetSource`, inheriting
the session's parquet execution settings.
