# `datafusion::test_util::parquet::ParquetScanOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.parquet.ParquetScanOptions.json).

<a id="op-2b6ed677af9c43be32f66e79"></a>
## ParquetScanOptions

`struct` · `datafusion::test_util::parquet::ParquetScanOptions` · datafusion 55.1.0

```rust
struct ParquetScanOptions
```

Source: `src/test_util/parquet.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Options for how to create the parquet scan

<a id="op-4f251aed41c6aac3cb5f40e6"></a>
## clone

`function` · `datafusion::test_util::parquet::ParquetScanOptions::clone` · datafusion 55.1.0

```rust
fn clone(&self) -> ParquetScanOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::parquet::ParquetScanOptions", "path": "ParquetScanOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 17], "end": [56, 22], "filename": "src/test_util/parquet.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/test_util/parquet.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-18456342c1e95fd2eae2abd3"></a>
## config

`function` · `datafusion::test_util::parquet::ParquetScanOptions::config` · datafusion 55.1.0

```rust
fn config(&self) -> SessionConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::parquet::ParquetScanOptions", "path": "ParquetScanOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [76, 2], "filename": "src/test_util/parquet.rs"}, "trait": null, "trait_path": null}`

Source: `src/test_util/parquet.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Returns a [`SessionConfig`](../operations/datafusion_execution.config.SessionConfig.md#op-5db676088685c6e8b0496c08) with the given options

<a id="op-88c5ea41ca0f9469fd08bd25"></a>
## enable_page_index

`struct_field` · `datafusion::test_util::parquet::ParquetScanOptions::enable_page_index` · datafusion 55.1.0

```rust
enable_page_index: bool
```

Source: `src/test_util/parquet.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

enable page index

<a id="op-bc50addb39c9d7c1ba462df1"></a>
## fmt

`function` · `datafusion::test_util::parquet::ParquetScanOptions::fmt` · datafusion 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion::test_util::parquet::ParquetScanOptions", "path": "ParquetScanOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 10], "end": [56, 15], "filename": "src/test_util/parquet.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test_util/parquet.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6218c2ab37124b37474a3f9"></a>
## pushdown_filters

`struct_field` · `datafusion::test_util::parquet::ParquetScanOptions::pushdown_filters` · datafusion 55.1.0

```rust
pushdown_filters: bool
```

Source: `src/test_util/parquet.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Enable pushdown filters

<a id="op-dbdad46f8333727e18b90ade"></a>
## reorder_filters

`struct_field` · `datafusion::test_util::parquet::ParquetScanOptions::reorder_filters` · datafusion 55.1.0

```rust
reorder_filters: bool
```

Source: `src/test_util/parquet.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

enable reordering filters
