# `datafusion_common::config::MaxRowGroupBytes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.MaxRowGroupBytes.json).

<a id="op-f570c00f129e43b78ae7d525"></a>
## MaxRowGroupBytes

`struct` · `datafusion_common::config::MaxRowGroupBytes` · datafusion-common 55.1.0

```rust
struct MaxRowGroupBytes
```

Source: `src/config.rs:1135`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Target maximum size of a Parquet row group in bytes.

Wraps a `usize` so the "must be greater than zero" constraint (arrow-rs
panics on a zero byte limit) is validated when the config is set, rather
than when the writer properties are built.

<a id="op-999c3b73f877e89b4aae2983"></a>
## Err

`assoc_type` · `datafusion_common::config::MaxRowGroupBytes::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MaxRowGroupBytes", "path": "MaxRowGroupBytes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1154, 1], "end": [1165, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/config.rs:1155`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f53b9fc613cba8701f291e6e"></a>
## clone

`function` · `datafusion_common::config::MaxRowGroupBytes::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> MaxRowGroupBytes
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MaxRowGroupBytes", "path": "MaxRowGroupBytes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1134, 10], "end": [1134, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:1134`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ae23342706ccb016af0e10b"></a>
## eq

`function` · `datafusion_common::config::MaxRowGroupBytes::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &MaxRowGroupBytes) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MaxRowGroupBytes", "path": "MaxRowGroupBytes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1134, 30], "end": [1134, 39], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:1134`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09b3920824109345c97011fd"></a>
## fmt

`function` · `datafusion_common::config::MaxRowGroupBytes::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MaxRowGroupBytes", "path": "MaxRowGroupBytes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1134, 23], "end": [1134, 28], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:1134`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e6a16b50d38226d919cca53"></a>
## fmt

`function` · `datafusion_common::config::MaxRowGroupBytes::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MaxRowGroupBytes", "path": "MaxRowGroupBytes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1167, 1], "end": [1171, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/config.rs:1168`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9831b3085266a109c01756d9"></a>
## from_str

`function` · `datafusion_common::config::MaxRowGroupBytes::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MaxRowGroupBytes", "path": "MaxRowGroupBytes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1154, 1], "end": [1165, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/config.rs:1157`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c6a50b954e118985cc0bd432"></a>
## get

`function` · `datafusion_common::config::MaxRowGroupBytes::get` · datafusion-common 55.1.0

```rust
fn get(&self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MaxRowGroupBytes", "path": "MaxRowGroupBytes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 1], "end": [1152, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:1149`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the configured byte limit.

<a id="op-e093e820ff826bb0eb98abbd"></a>
## try_new

`function` · `datafusion_common::config::MaxRowGroupBytes::try_new` · datafusion-common 55.1.0

```rust
fn try_new(value: usize) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MaxRowGroupBytes", "path": "MaxRowGroupBytes"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1137, 1], "end": [1152, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:1139`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Creates a `MaxRowGroupBytes`, rejecting zero.
