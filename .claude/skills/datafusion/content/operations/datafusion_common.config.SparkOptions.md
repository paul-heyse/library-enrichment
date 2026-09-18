# `datafusion_common::config::SparkOptions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.SparkOptions.json).

<a id="op-115cca59bf45d6ccadf93eaf"></a>
## SparkOptions

`struct` · `datafusion_common::config::SparkOptions` · datafusion-common 55.1.0

```rust
struct SparkOptions
```

Source: `src/config.rs:1881`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Options controlling DataFusion's Spark-compatibility layer (functions
under `datafusion/spark`). Keys here mirror their `spark.sql.*`
equivalents in Apache Spark.

<a id="op-7d39bda08af7d5b2d6e109fa"></a>
## clone

`function` · `datafusion_common::config::SparkOptions::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> SparkOptions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SparkOptions", "path": "SparkOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1881, 1], "end": [1897, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:1881`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9f2a9cbd77515579a59a9dd5"></a>
## default

`function` · `datafusion_common::config::SparkOptions::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SparkOptions", "path": "SparkOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1881, 1], "end": [1897, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:1881`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-086be0185a22da7ee554a13c"></a>
## eq

`function` · `datafusion_common::config::SparkOptions::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &SparkOptions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SparkOptions", "path": "SparkOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1881, 1], "end": [1897, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:1881`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36146440c17cd0666c4e8251"></a>
## fmt

`function` · `datafusion_common::config::SparkOptions::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SparkOptions", "path": "SparkOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1881, 1], "end": [1897, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:1881`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70d02cd942c5fc943cc96a38"></a>
## map_key_dedup_policy

`struct_field` · `datafusion_common::config::SparkOptions::map_key_dedup_policy` · datafusion-common 55.1.0

```rust
map_key_dedup_policy: MapKeyDedupPolicy
```

Source: `src/config.rs:1881`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Policy for handling duplicate keys in Spark-compatible map-construction
functions (`map_from_arrays`, `map_from_entries`, `str_to_map`).

Mirrors Spark's
[`spark.sql.mapKeyDedupPolicy`](https://github.com/apache/spark/blob/cf3a34e19dfcf70e2d679217ff1ba21302212472/sql/catalyst/src/main/scala/org/apache/spark/sql/internal/SQLConf.scala#L4961):
- `EXCEPTION` (default): raise `[DUPLICATED_MAP_KEY]` at runtime on any duplicate key.
- `LAST_WIN`: keep the last occurrence of each duplicate key.

Values are case-insensitive.

<a id="op-2ec6e4f378124d6489409ee4"></a>
## reset

`function` · `datafusion_common::config::SparkOptions::reset` · datafusion-common 55.1.0

```rust
fn reset(&mut self, key: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SparkOptions", "path": "SparkOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1881, 1], "end": [1897, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1881`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd32eedfe7c8b7a0eb55460e"></a>
## set

`function` · `datafusion_common::config::SparkOptions::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, key: &str, value: &str) -> error::Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SparkOptions", "path": "SparkOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1881, 1], "end": [1897, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1881`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-933277f2edace41fe8820afa"></a>
## visit

`function` · `datafusion_common::config::SparkOptions::visit` · datafusion-common 55.1.0

```rust
fn visit<V: config::Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::SparkOptions", "path": "SparkOptions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1881, 1], "end": [1897, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:1881`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
