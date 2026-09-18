# `datafusion_common::config::MapKeyDedupPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.MapKeyDedupPolicy.json).

<a id="op-691dbe81197856466e4de2e2"></a>
## MapKeyDedupPolicy

`enum` · `datafusion_common::config::MapKeyDedupPolicy` · datafusion-common 55.1.0

```rust
enum MapKeyDedupPolicy
```

Source: `src/config.rs:755`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Policy for handling duplicate keys in Spark-compatible map-construction
functions (`map_from_arrays`, `map_from_entries`, `str_to_map`). Mirrors
Spark's [`spark.sql.mapKeyDedupPolicy`](https://github.com/apache/spark/blob/cf3a34e19dfcf70e2d679217ff1ba21302212472/sql/catalyst/src/main/scala/org/apache/spark/sql/internal/SQLConf.scala#L4961).

<a id="op-5d8a77f7c51cd45695e1ea8e"></a>
## Err

`assoc_type` · `datafusion_common::config::MapKeyDedupPolicy::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MapKeyDedupPolicy", "path": "MapKeyDedupPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [763, 1], "end": [775, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/config.rs:764`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-264f21e0347cff231246dd5e"></a>
## Exception

`variant` · `datafusion_common::config::MapKeyDedupPolicy::Exception` · datafusion-common 55.1.0

```rust
Exception
```

Source: `src/config.rs:758`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Raise `[DUPLICATED_MAP_KEY]` at runtime on any duplicate key.

<a id="op-7093bdb6edfd7618fbdd8935"></a>
## LastWin

`variant` · `datafusion_common::config::MapKeyDedupPolicy::LastWin` · datafusion-common 55.1.0

```rust
LastWin
```

Source: `src/config.rs:760`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Keep the last occurrence of each duplicate key.

<a id="op-d1cd7fcb1864cbd517e1d3e4"></a>
## clone

`function` · `datafusion_common::config::MapKeyDedupPolicy::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> MapKeyDedupPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MapKeyDedupPolicy", "path": "MapKeyDedupPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 26], "end": [754, 31], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:754`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38a0e53d4471e91e2fdf3930"></a>
## default

`function` · `datafusion_common::config::MapKeyDedupPolicy::default` · datafusion-common 55.1.0

```rust
fn default() -> MapKeyDedupPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MapKeyDedupPolicy", "path": "MapKeyDedupPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 17], "end": [754, 24], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:754`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d2cff57e22450316ecc63db"></a>
## eq

`function` · `datafusion_common::config::MapKeyDedupPolicy::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &MapKeyDedupPolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MapKeyDedupPolicy", "path": "MapKeyDedupPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 39], "end": [754, 48], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:754`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8b0c4fc6e2ae86c8e755200"></a>
## fmt

`function` · `datafusion_common::config::MapKeyDedupPolicy::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MapKeyDedupPolicy", "path": "MapKeyDedupPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [788, 1], "end": [796, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/config.rs:789`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0d6b1fd017a652fd01cfa1e"></a>
## fmt

`function` · `datafusion_common::config::MapKeyDedupPolicy::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MapKeyDedupPolicy", "path": "MapKeyDedupPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [754, 10], "end": [754, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:754`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3af8a21ae697d116b2469cfc"></a>
## from_str

`function` · `datafusion_common::config::MapKeyDedupPolicy::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MapKeyDedupPolicy", "path": "MapKeyDedupPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [763, 1], "end": [775, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/config.rs:766`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ed3918d0919ec27a922ba27"></a>
## set

`function` · `datafusion_common::config::MapKeyDedupPolicy::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MapKeyDedupPolicy", "path": "MapKeyDedupPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 1], "end": [786, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:782`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6f1f6ea3c9540447b91d043c"></a>
## visit

`function` · `datafusion_common::config::MapKeyDedupPolicy::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::MapKeyDedupPolicy", "path": "MapKeyDedupPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [777, 1], "end": [786, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:778`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
