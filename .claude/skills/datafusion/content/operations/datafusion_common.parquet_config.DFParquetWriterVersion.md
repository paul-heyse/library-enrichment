# `datafusion_common::parquet_config::DFParquetWriterVersion`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.parquet_config.DFParquetWriterVersion.json).

<a id="op-c7131a8d896f1d9fd8885667"></a>
## DFParquetWriterVersion

`enum` · `datafusion_common::parquet_config::DFParquetWriterVersion` · datafusion-common 55.1.0

```rust
enum DFParquetWriterVersion
```

Source: `src/parquet_config.rs:30`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Parquet writer version options for controlling the Parquet file format version

This enum validates parquet writer version values at configuration time,
ensuring only valid versions ("1.0" or "2.0") can be set via `SET` commands
or proto deserialization.

<a id="op-155b878b2283616fce50ccff"></a>
## Err

`assoc_type` · `datafusion_common::parquet_config::DFParquetWriterVersion::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parquet_config::DFParquetWriterVersion", "path": "DFParquetWriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [51, 2], "filename": "src/parquet_config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/parquet_config.rs:40`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dbef187ed4e54fc94f8a286"></a>
## V1_0

`variant` · `datafusion_common::parquet_config::DFParquetWriterVersion::V1_0` · datafusion-common 55.1.0

```rust
V1_0
```

Source: `src/parquet_config.rs:33`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Parquet format version 1.0

<a id="op-060bd041d2a67b172b5536ab"></a>
## V2_0

`variant` · `datafusion_common::parquet_config::DFParquetWriterVersion::V2_0` · datafusion-common 55.1.0

```rust
V2_0
```

Source: `src/parquet_config.rs:35`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Parquet format version 2.0

<a id="op-548d70c586791a1707e7b441"></a>
## clone

`function` · `datafusion_common::parquet_config::DFParquetWriterVersion::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DFParquetWriterVersion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parquet_config::DFParquetWriterVersion", "path": "DFParquetWriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 17], "end": [29, 22], "filename": "src/parquet_config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/parquet_config.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f262615d5cd59c5eebc87a67"></a>
## default

`function` · `datafusion_common::parquet_config::DFParquetWriterVersion::default` · datafusion-common 55.1.0

```rust
fn default() -> DFParquetWriterVersion
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parquet_config::DFParquetWriterVersion", "path": "DFParquetWriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 45], "end": [29, 52], "filename": "src/parquet_config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/parquet_config.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e76cd794bd8a03db5294f340"></a>
## eq

`function` · `datafusion_common::parquet_config::DFParquetWriterVersion::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &DFParquetWriterVersion) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parquet_config::DFParquetWriterVersion", "path": "DFParquetWriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 30], "end": [29, 39], "filename": "src/parquet_config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/parquet_config.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a7cc14537f40f0928f480d4"></a>
## fmt

`function` · `datafusion_common::parquet_config::DFParquetWriterVersion::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parquet_config::DFParquetWriterVersion", "path": "DFParquetWriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [29, 10], "end": [29, 15], "filename": "src/parquet_config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/parquet_config.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faf020de2b4ae8e6eeb5b752"></a>
## fmt

`function` · `datafusion_common::parquet_config::DFParquetWriterVersion::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parquet_config::DFParquetWriterVersion", "path": "DFParquetWriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [53, 1], "end": [61, 2], "filename": "src/parquet_config.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/parquet_config.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7949b5ccdea08f1bccbba5f4"></a>
## from

`function` · `datafusion_common::parquet_config::DFParquetWriterVersion::from` · datafusion-common 55.1.0

```rust
fn from(version: parquet::file::properties::WriterVersion) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parquet_config::DFParquetWriterVersion", "path": "DFParquetWriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [108, 2], "filename": "src/parquet_config.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "parquet::file::properties::WriterVersion", "path": "WriterVersion"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/parquet_config.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7629bfe445aa7c3ae2d75162"></a>
## from_str

`function` · `datafusion_common::parquet_config::DFParquetWriterVersion::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parquet_config::DFParquetWriterVersion", "path": "DFParquetWriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [39, 1], "end": [51, 2], "filename": "src/parquet_config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/parquet_config.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f60c4642f1100da40339ae17"></a>
## set

`function` · `datafusion_common::parquet_config::DFParquetWriterVersion::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parquet_config::DFParquetWriterVersion", "path": "DFParquetWriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [72, 2], "filename": "src/parquet_config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/parquet_config.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce98645f966682a424deba54"></a>
## visit

`function` · `datafusion_common::parquet_config::DFParquetWriterVersion::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::parquet_config::DFParquetWriterVersion", "path": "DFParquetWriterVersion"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [72, 2], "filename": "src/parquet_config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/parquet_config.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
