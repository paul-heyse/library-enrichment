# `datafusion_common::config::ParquetCdcOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.ParquetCdcOptions.json).

<a id="op-c42aa9448921537e61629473"></a>
## ParquetCdcOptions

`struct` · `datafusion_common::config::ParquetCdcOptions` · datafusion-common 55.1.0

```rust
struct ParquetCdcOptions
```

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options for content-defined chunking (CDC) when writing parquet files.
Mirrors `parquet::file::properties::CdcOptions`.

Carried as a [`ParquetCdcOptions`](../operations/datafusion_common.config.ParquetCdcOptions.md#op-c42aa9448921537e61629473) in [`ParquetOptions::content_defined_chunking`](../operations/datafusion_common.config.ParquetOptions.md#op-3bdc6d36d8738c88e88a5095)
with an explicit `enabled` flag, so it can be toggled with dotted config
keys (`content_defined_chunking.enabled = true|false`) and the result is
independent of the order in which the keys are set.

<a id="op-02c85b13c6951ae0cbd8d603"></a>
## clone

`function` · `datafusion_common::config::ParquetCdcOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> ParquetCdcOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetCdcOptions", "path": "ParquetCdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1082, 1], "end": [1108, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbb56001fad5365a066f947a"></a>
## default

`function` · `datafusion_common::config::ParquetCdcOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetCdcOptions", "path": "ParquetCdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1082, 1], "end": [1108, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3e45be13ebffe864c5769ce"></a>
## disabled

`function` · `datafusion_common::config::ParquetCdcOptions::disabled` · datafusion-common 55.1.0

```rust
fn disabled() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetCdcOptions", "path": "ParquetCdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1110, 1], "end": [1127, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:1124`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns disabled CDC options (equivalent to [`ParquetCdcOptions::default`](../operations/datafusion_common.config.ParquetCdcOptions.md#op-fbb56001fad5365a066f947a)).

<a id="op-0db7f875342420b26c365ba3"></a>
## enabled

`struct_field` · `datafusion_common::config::ParquetCdcOptions::enabled` · datafusion-common 55.1.0

```rust
enabled: bool
```

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

(writing) EXPERIMENTAL: Enable content-defined chunking (CDC) when writing
parquet files. When enabled, parallel writing is automatically disabled
since the chunker state must persist across row groups.

<a id="op-dfd775a1f1a4e7a03455e26d"></a>
## enabled

`function` · `datafusion_common::config::ParquetCdcOptions::enabled` · datafusion-common 55.1.0

```rust
fn enabled() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetCdcOptions", "path": "ParquetCdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1110, 1], "end": [1127, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:1116`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns enabled CDC options with the default chunking parameters.

Shorthand for `ParquetCdcOptions { enabled: true, ..Default::default() }`;
combine with struct-update syntax to override parameters, e.g.
`ParquetCdcOptions { min_chunk_size: 4096, ..ParquetCdcOptions::enabled() }`.

<a id="op-d6bff0a332723cdebd7aee71"></a>
## eq

`function` · `datafusion_common::config::ParquetCdcOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &ParquetCdcOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetCdcOptions", "path": "ParquetCdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1082, 1], "end": [1108, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-275cc001deafb4dda4f0370b"></a>
## fmt

`function` · `datafusion_common::config::ParquetCdcOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetCdcOptions", "path": "ParquetCdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1082, 1], "end": [1108, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71eddf0e2bc95c026558bf2b"></a>
## from

`function` · `datafusion_common::config::ParquetCdcOptions::from` · datafusion-common 55.1.0

```rust
fn from(value: Option<&parquet::file::properties::CdcOptions>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetCdcOptions", "path": "crate::config::ParquetCdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [203, 2], "filename": "src/file_options/parquet_writer.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"resolved_path": {"args": null, "id": "parquet::file::properties::CdcOptions", "path": "CdcOptions"}}}}}], "constraints": []}}, "id": "core::option::Option", "path": "Option"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/file_options/parquet_writer.rs:192`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2da965d0f47b157ba27c96b6"></a>
## max_chunk_size

`struct_field` · `datafusion_common::config::ParquetCdcOptions::max_chunk_size` · datafusion-common 55.1.0

```rust
max_chunk_size: usize
```

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Maximum chunk size in bytes. A split is forced when the accumulated
size exceeds this value. Default is 1 MiB.

<a id="op-1f4ffc8753e100c9775e715f"></a>
## min_chunk_size

`struct_field` · `datafusion_common::config::ParquetCdcOptions::min_chunk_size` · datafusion-common 55.1.0

```rust
min_chunk_size: usize
```

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Minimum chunk size in bytes. The rolling hash will not trigger a split
until this many bytes have been accumulated. Default is 256 KiB.

<a id="op-19c7897318d51298fab80e74"></a>
## norm_level

`struct_field` · `datafusion_common::config::ParquetCdcOptions::norm_level` · datafusion-common 55.1.0

```rust
norm_level: i32
```

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Normalization level. Increasing this improves deduplication ratio
but increases fragmentation. Recommended range is [-3, 3], default is 0.

<a id="op-4e7d88e0c1b05aa9fb7aeb87"></a>
## reset

`function` · `datafusion_common::config::ParquetCdcOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetCdcOptions", "path": "ParquetCdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1082, 1], "end": [1108, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a2e4737c13696a9029e9df7"></a>
## set

`function` · `datafusion_common::config::ParquetCdcOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetCdcOptions", "path": "ParquetCdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1082, 1], "end": [1108, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ab1f46cb20030b79e3a51e24"></a>
## visit

`function` · `datafusion_common::config::ParquetCdcOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::ParquetCdcOptions", "path": "ParquetCdcOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1082, 1], "end": [1108, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1082`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
