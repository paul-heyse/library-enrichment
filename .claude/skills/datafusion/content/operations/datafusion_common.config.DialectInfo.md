# `datafusion_common::config::DialectInfo`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.config.DialectInfo.json).

<a id="op-b85310619a7e55681db3b03e"></a>
## DialectInfo

`struct` · `datafusion_common::config::DialectInfo` · datafusion-common 55.1.0

```rust
struct DialectInfo
```

Source: `src/config.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Metadata for a SQL dialect supported by DataFusion configuration.

<a id="op-383eabf67457d75b420cebf5"></a>
## aliases

`struct_field` · `datafusion_common::config::DialectInfo::aliases` · datafusion-common 55.1.0

```rust
aliases: &'static [&'static str]
```

Source: `src/config.rs:334`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5d126dcb6d5fab8381bc5a3"></a>
## canonical_name

`struct_field` · `datafusion_common::config::DialectInfo::canonical_name` · datafusion-common 55.1.0

```rust
canonical_name: &'static str
```

Source: `src/config.rs:332`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b44dbed52c50f050e7f15efc"></a>
## clone

`function` · `datafusion_common::config::DialectInfo::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> DialectInfo
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::DialectInfo", "path": "DialectInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 17], "end": [328, 22], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/config.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de7ae17afd3f23159d158f81"></a>
## dialect

`struct_field` · `datafusion_common::config::DialectInfo::dialect` · datafusion-common 55.1.0

```rust
dialect: Dialect
```

Source: `src/config.rs:331`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bf798c1982a62c8236a20bc"></a>
## display_name

`struct_field` · `datafusion_common::config::DialectInfo::display_name` · datafusion-common 55.1.0

```rust
display_name: &'static str
```

Source: `src/config.rs:333`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b122e3418d4fa179caafc79d"></a>
## eq

`function` · `datafusion_common::config::DialectInfo::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &DialectInfo) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::DialectInfo", "path": "DialectInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 30], "end": [328, 39], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/config.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-282a7ad89c2ab7938ca96a9d"></a>
## fmt

`function` · `datafusion_common::config::DialectInfo::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::config::DialectInfo", "path": "DialectInfo"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [328, 10], "end": [328, 15], "filename": "src/config.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/config.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
