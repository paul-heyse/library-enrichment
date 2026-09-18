# `deltalake_core::operations::write::configs::WriterStatsConfig`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.write.configs.WriterStatsConfig.json).

<a id="op-058f774f8874f5ca562e4e91"></a>
## WriterStatsConfig

`struct` · `deltalake_core::operations::write::configs::WriterStatsConfig` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
struct WriterStatsConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/configs.rs#L10).

Source: `crates/core/src/operations/write/configs.rs:10`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Configuration for the writer on how to collect stats

<a id="op-baa719f534a7075a77b5d1c9"></a>
## clone

`function` · `deltalake_core::operations::write::configs::WriterStatsConfig::clone` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn clone(&self) -> WriterStatsConfig
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/configs.rs#L9).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::configs::WriterStatsConfig", "path": "WriterStatsConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9, 10], "end": [9, 15], "filename": "crates/core/src/operations/write/configs.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `crates/core/src/operations/write/configs.rs:9`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f5def5d3c54d05647b9263c"></a>
## from_config

`function` · `deltalake_core::operations::write::configs::WriterStatsConfig::from_config` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn from_config(config: &TableConfiguration) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/configs.rs#L30).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::configs::WriterStatsConfig", "path": "WriterStatsConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [17, 1], "end": [44, 2], "filename": "crates/core/src/operations/write/configs.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/configs.rs:30`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Derive writer statistics configuration from a table's [`TableConfiguration`](../operations/buoyant_kernel.table_configuration.TableConfiguration.md#op-4c1bb6e90ea642d0fb312bae).

<a id="op-a70e8554e9fc0e4268d855c2"></a>
## new

`function` · `deltalake_core::operations::write::configs::WriterStatsConfig::new` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
fn new(num_indexed_cols: DataSkippingNumIndexedCols, stats_columns: Option<Vec<String>>) -> Self
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/configs.rs#L19).

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "deltalake_core::operations::write::configs::WriterStatsConfig", "path": "WriterStatsConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [17, 1], "end": [44, 2], "filename": "crates/core/src/operations/write/configs.rs"}, "trait": null, "trait_path": null}`

Source: `crates/core/src/operations/write/configs.rs:19`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Create new writer stats config

<a id="op-f8341e688dc94bcccb768df5"></a>
## num_indexed_cols

`struct_field` · `deltalake_core::operations::write::configs::WriterStatsConfig::num_indexed_cols` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
num_indexed_cols: delta_kernel::table_properties::DataSkippingNumIndexedCols
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/configs.rs#L12).

Source: `crates/core/src/operations/write/configs.rs:12`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Number of columns to collect stats for, idx based

<a id="op-4f529d381512a21dedd8dbb4"></a>
## stats_columns

`struct_field` · `deltalake_core::operations::write::configs::WriterStatsConfig::stats_columns` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
stats_columns: Option<Vec<String>>
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/write/configs.rs#L14).

Source: `crates/core/src/operations/write/configs.rs:14`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Optional list of columns which to collect stats for, takes precedende over num_index_cols
