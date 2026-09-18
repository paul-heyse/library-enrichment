# `datafusion_common::config::Dialect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.Dialect.json).

<a id="op-353498e2a51f4ee4cc3136be"></a>
## Dialect

`enum` · `datafusion_common::config::Dialect` · datafusion-common 55.1.0

```rust
enum Dialect
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

This is the SQL dialect used by DataFusion's parser.
This mirrors [sqlparser::dialect::Dialect](https://docs.rs/sqlparser/latest/sqlparser/dialect/trait.Dialect.html)
trait in order to offer an easier API and avoid adding the `sqlparser` dependency

<a id="op-fb47219d9d1f7fb3101071d7"></a>
## Ansi

`variant` · `datafusion_common::config::Dialect::Ansi` · datafusion-common 55.1.0

```rust
Ansi
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a0700c97586efe7ccca17434"></a>
## BigQuery

`variant` · `datafusion_common::config::Dialect::BigQuery` · datafusion-common 55.1.0

```rust
BigQuery
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-806752c82dc62bcad1c12e72"></a>
## ClickHouse

`variant` · `datafusion_common::config::Dialect::ClickHouse` · datafusion-common 55.1.0

```rust
ClickHouse
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1d791371fc7925d87a9afa1"></a>
## Databricks

`variant` · `datafusion_common::config::Dialect::Databricks` · datafusion-common 55.1.0

```rust
Databricks
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1608a77e0d7a0b0f88521b4"></a>
## DuckDB

`variant` · `datafusion_common::config::Dialect::DuckDB` · datafusion-common 55.1.0

```rust
DuckDB
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81f71d1c452a7f1381bca4a6"></a>
## Err

`assoc_type` · `datafusion_common::config::Dialect::Err` · datafusion-common 55.1.0

```rust
Err
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [516, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/config.rs:497`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25190753f9f3404bc137d95c"></a>
## Generic

`variant` · `datafusion_common::config::Dialect::Generic` · datafusion-common 55.1.0

```rust
Generic
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72e3051be31fe877343edaa8"></a>
## Hive

`variant` · `datafusion_common::config::Dialect::Hive` · datafusion-common 55.1.0

```rust
Hive
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50ce6c6358de6d4ab7fc1013"></a>
## MsSQL

`variant` · `datafusion_common::config::Dialect::MsSQL` · datafusion-common 55.1.0

```rust
MsSQL
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8668cebc237574fc925f8777"></a>
## MySQL

`variant` · `datafusion_common::config::Dialect::MySQL` · datafusion-common 55.1.0

```rust
MySQL
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47cba6ff8545bcaf248fe56a"></a>
## PostgreSQL

`variant` · `datafusion_common::config::Dialect::PostgreSQL` · datafusion-common 55.1.0

```rust
PostgreSQL
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0acd5d86f44b36178fbc41ca"></a>
## Redshift

`variant` · `datafusion_common::config::Dialect::Redshift` · datafusion-common 55.1.0

```rust
Redshift
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97d82f8424e521c2ec701b7f"></a>
## SQLite

`variant` · `datafusion_common::config::Dialect::SQLite` · datafusion-common 55.1.0

```rust
SQLite
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cde41a81981e2202143c3de"></a>
## Snowflake

`variant` · `datafusion_common::config::Dialect::Snowflake` · datafusion-common 55.1.0

```rust
Snowflake
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd5e62cc2c1d8e176f73bf21"></a>
## Spark

`variant` · `datafusion_common::config::Dialect::Spark` · datafusion-common 55.1.0

```rust
Spark
```

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb6f6e7dae9183cbec989cdb"></a>
## as_ref

`function` · `datafusion_common::config::Dialect::as_ref` · datafusion-common 55.1.0

```rust
fn as_ref(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [490, 1], "end": [494, 2], "filename": "src/config.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"primitive": "str"}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/config.rs:491`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a339ad8e54ce60c9d25761b"></a>
## available

`function` · `datafusion_common::config::Dialect::available` · datafusion-common 55.1.0

```rust
fn available() -> &'static str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [488, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:478`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return all supported dialect names, for use in error messages.

<a id="op-6fa2ba5794f8d1bbd674b419"></a>
## clone

`function` · `datafusion_common::config::Dialect::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> Dialect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [469, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d36da7ec87efbfcf39303914"></a>
## default

`function` · `datafusion_common::config::Dialect::default` · datafusion-common 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [469, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d65f5f6267aac1e2075f0dc"></a>
## eq

`function` · `datafusion_common::config::Dialect::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &Dialect) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [469, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-272dc7927f678f431963cc64"></a>
## fmt

`function` · `datafusion_common::config::Dialect::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [534, 1], "end": [539, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/config.rs:535`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7ba493c08e7b1ffb6f14f8b"></a>
## fmt

`function` · `datafusion_common::config::Dialect::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [397, 1], "end": [469, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43b30ebac6332eae007eac2d"></a>
## from_str

`function` · `datafusion_common::config::Dialect::from_str` · datafusion-common 55.1.0

```rust
fn from_str(s: &str) -> Result<Self, Self::Err>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [496, 1], "end": [516, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::str::traits::FromStr", "path": "FromStr"}, "trait_path": "core::str::traits::FromStr"}`

Source: `src/config.rs:499`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b89eb483c1c4d926cdd3abbb"></a>
## metadata

`function` · `datafusion_common::config::Dialect::metadata` · datafusion-common 55.1.0

```rust
fn metadata() -> &'static [DialectInfo]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [488, 2], "filename": "src/config.rs"}, "trait": null, "trait_path": null}`

Source: `src/config.rs:473`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Return metadata for all supported dialects.

<a id="op-399f4df99cd29a45a10d80b8"></a>
## set

`function` · `datafusion_common::config::Dialect::set` · datafusion-common 55.1.0

```rust
fn set(&mut self, _: &str, value: &str) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 1], "end": [532, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:528`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e4707a50d659610410dd39d"></a>
## visit

`function` · `datafusion_common::config::Dialect::visit` · datafusion-common 55.1.0

```rust
fn visit<V: Visit>(&self, v: &mut V, key: &str, description: &'static str)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::Dialect", "path": "Dialect"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [518, 1], "end": [532, 2], "filename": "src/config.rs"}, "trait": {"args": null, "id": "datafusion_common::config::ConfigField", "path": "ConfigField"}, "trait_path": "datafusion_common::config::ConfigField"}`

Source: `src/config.rs:519`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
