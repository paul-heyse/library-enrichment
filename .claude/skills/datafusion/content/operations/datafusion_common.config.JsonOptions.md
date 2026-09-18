# `datafusion_common::config::JsonOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.JsonOptions.json).

<a id="op-ebeac9b20521507e4656a21b"></a>
## JsonOptions

`struct` · `datafusion_common::config::JsonOptions` · datafusion-common 55.1.0

```rust
struct JsonOptions
```

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options controlling JSON format

<a id="op-77bf407d3820fc4f93144992"></a>
## clone

`function` · `datafusion_common::config::JsonOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> JsonOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3817, 1], "end": [3847, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2817e15e11e4dee74de36dc"></a>
## compression

`struct_field` · `datafusion_common::config::JsonOptions::compression` · datafusion-common 55.1.0

```rust
compression: parsers::CompressionTypeVariant
```

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-279594ab56d983608c7ad92b"></a>
## compression_level

`struct_field` · `datafusion_common::config::JsonOptions::compression_level` · datafusion-common 55.1.0

```rust
compression_level: Option<u32>
```

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compression level for the output file. The valid range depends on the
compression algorithm:
- ZSTD: 1 to 22 (default: 3)
- GZIP: 0 to 9 (default: 6)
- BZIP2: 0 to 9 (default: 6)
- XZ: 0 to 9 (default: 6)
If not specified, the default level for the compression algorithm is used.

<a id="op-757fda07919355e864b43be6"></a>
## default

`function` · `datafusion_common::config::JsonOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3817, 1], "end": [3847, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ac8815e8f9d13143cfecbb3"></a>
## eq

`function` · `datafusion_common::config::JsonOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &JsonOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3817, 1], "end": [3847, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99ea168b7bc897ed9e84b80b"></a>
## fmt

`function` · `datafusion_common::config::JsonOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3817, 1], "end": [3847, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbb657d5cb9aabaf1d842853"></a>
## newline_delimited

`struct_field` · `datafusion_common::config::JsonOptions::newline_delimited` · datafusion-common 55.1.0

```rust
newline_delimited: bool
```

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

The JSON format to use when reading files.

When `true` (default), expects newline-delimited JSON (NDJSON):
```text
{"key1": 1, "key2": "val"}
{"key1": 2, "key2": "vals"}
```

When `false`, expects JSON array format:
```text
[
  {"key1": 1, "key2": "val"},
  {"key1": 2, "key2": "vals"}
]
```

<a id="op-8ae2a0c4dd1fb7ac0c1201e3"></a>
## reset

`function` · `datafusion_common::config::JsonOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3817, 1], "end": [3847, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a223f6336817f058687aade"></a>
## schema_infer_max_rec

`struct_field` · `datafusion_common::config::JsonOptions::schema_infer_max_rec` · datafusion-common 55.1.0

```rust
schema_infer_max_rec: Option<usize>
```

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-355a3762ff66b5b5a5ab5556"></a>
## set

`function` · `datafusion_common::config::JsonOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3817, 1], "end": [3847, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9fb6bf639559e73ccb7acc0f"></a>
## visit

`function` · `datafusion_common::config::JsonOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::JsonOptions", "path": "JsonOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3817, 1], "end": [3847, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:3817`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
