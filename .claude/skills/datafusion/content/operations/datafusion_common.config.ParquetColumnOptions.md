# `datafusion_common::config::ParquetColumnOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ParquetColumnOptions.json).

<a id="op-7759752e749ff6eaf3d95e85"></a>
## ParquetColumnOptions

`struct` · `datafusion_common::config::ParquetColumnOptions` · datafusion-common 55.1.0

```rust
struct ParquetColumnOptions
```

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options controlling parquet format for individual columns.

See [`ParquetOptions`](../operations/datafusion_common.config.ParquetOptions.md#op-434fa79e897be23d649f954c) for more details

<a id="op-0bde1962f7281111e6094bbc"></a>
## bloom_filter_enabled

`struct_field` · `datafusion_common::config::ParquetColumnOptions::bloom_filter_enabled` · datafusion-common 55.1.0

```rust
bloom_filter_enabled: Option<bool>
```

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets if bloom filter is enabled for the column path.

<a id="op-ed1e2f6dcf4d3f1346fb484b"></a>
## bloom_filter_fpp

`struct_field` · `datafusion_common::config::ParquetColumnOptions::bloom_filter_fpp` · datafusion-common 55.1.0

```rust
bloom_filter_fpp: Option<f64>
```

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets bloom filter false positive probability for the column path. If NULL, uses
default parquet options

<a id="op-3c4262fbba8bb03024756bee"></a>
## bloom_filter_ndv

`struct_field` · `datafusion_common::config::ParquetColumnOptions::bloom_filter_ndv` · datafusion-common 55.1.0

```rust
bloom_filter_ndv: Option<u64>
```

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets bloom filter number of distinct values. If NULL, uses
default parquet options

<a id="op-2b0f3d31b39961dd963b5adb"></a>
## clone

`function` · `datafusion_common::config::ParquetColumnOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ParquetColumnOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetColumnOptions", "path": "ParquetColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3168, 1], "end": [3209, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4611214a51b763f25d5562a"></a>
## compression

`struct_field` · `datafusion_common::config::ParquetColumnOptions::compression` · datafusion-common 55.1.0

```rust
compression: Option<String>
```

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets default parquet compression codec for the column path.
Valid values are: uncompressed, snappy, gzip(level),
brotli(level), lz4, zstd(level), and lz4_raw.
These values are not case-sensitive. If NULL, uses
default parquet options

<a id="op-193bbd022218f27baf3f6235"></a>
## default

`function` · `datafusion_common::config::ParquetColumnOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetColumnOptions", "path": "ParquetColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3168, 1], "end": [3209, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b05b91fa76deed5801bcdc0b"></a>
## dictionary_enabled

`struct_field` · `datafusion_common::config::ParquetColumnOptions::dictionary_enabled` · datafusion-common 55.1.0

```rust
dictionary_enabled: Option<bool>
```

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets if dictionary encoding is enabled for the column path. If NULL, uses
default parquet options

<a id="op-52458909db6b22ce598d56e4"></a>
## encoding

`struct_field` · `datafusion_common::config::ParquetColumnOptions::encoding` · datafusion-common 55.1.0

```rust
encoding: Option<String>
```

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets encoding for the column path.
Valid values are: plain, plain_dictionary, rle,
bit_packed, delta_binary_packed, delta_length_byte_array,
delta_byte_array, rle_dictionary, and byte_stream_split.
These values are not case-sensitive. If NULL, uses
default parquet options

<a id="op-2a009e8e962e89b6b2f79cbe"></a>
## eq

`function` · `datafusion_common::config::ParquetColumnOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ParquetColumnOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetColumnOptions", "path": "ParquetColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3168, 1], "end": [3209, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-75501e40076db16edb8e3273"></a>
## fmt

`function` · `datafusion_common::config::ParquetColumnOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetColumnOptions", "path": "ParquetColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3168, 1], "end": [3209, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8404aaa53394910dd492d9e3"></a>
## set

`function` · `datafusion_common::config::ParquetColumnOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetColumnOptions", "path": "ParquetColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3168, 1], "end": [3209, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5d0e14e2930f0f321f030dfe"></a>
## statistics_enabled

`struct_field` · `datafusion_common::config::ParquetColumnOptions::statistics_enabled` · datafusion-common 55.1.0

```rust
statistics_enabled: Option<String>
```

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Sets if statistics are enabled for the column
Valid values are: "none", "chunk", and "page"
These values are not case sensitive. If NULL, uses
default parquet options

<a id="op-e7e3ad08aa0f0b06677f6adc"></a>
## visit

`function` · `datafusion_common::config::ParquetColumnOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetColumnOptions", "path": "ParquetColumnOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3168, 1], "end": [3209, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
