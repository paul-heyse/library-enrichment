# `sqlparser::ast::CatalogSyncNamespaceMode`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CatalogSyncNamespaceMode.json).

<a id="op-55878ed9b62876a3e666e745"></a>
## CatalogSyncNamespaceMode

`enum` · `sqlparser::ast::CatalogSyncNamespaceMode` · sqlparser 0.62.0

```rust
enum CatalogSyncNamespaceMode
```

Source: `src/ast/mod.rs:11254`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake CatalogSyncNamespaceMode
```sql
[ CATALOG_SYNC_NAMESPACE_MODE = { NEST | FLATTEN } ]
```

<https://docs.snowflake.com/en/sql-reference/sql/create-database>

<a id="op-4db36cba90d505ad1a3b5ab7"></a>
## Flatten

`variant` · `sqlparser::ast::CatalogSyncNamespaceMode::Flatten` · sqlparser 0.62.0

```rust
Flatten
```

Source: `src/ast/mod.rs:11258`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flatten namespaces when syncing catalog.

<a id="op-fc1f6e1202ed4449ec4234c7"></a>
## Nest

`variant` · `sqlparser::ast::CatalogSyncNamespaceMode::Nest` · sqlparser 0.62.0

```rust
Nest
```

Source: `src/ast/mod.rs:11256`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Nest namespaces when syncing catalog.

<a id="op-e02fabff1430eedcf4abd9d1"></a>
## clone

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CatalogSyncNamespaceMode
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11251, 23], "end": [11251, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e5c7db70331db05f673be28"></a>
## cmp

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CatalogSyncNamespaceMode) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11251, 57], "end": [11251, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36dd51bc1ece9483b8e12734"></a>
## deserialize

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11252, 49], "end": [11252, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11252`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4f9a827fa8094f7a3e6eafca"></a>
## eq

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CatalogSyncNamespaceMode) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11251, 30], "end": [11251, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46861a2c380eff828bac9e25"></a>
## fmt

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11251, 10], "end": [11251, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7275936ad14909e9360e8c38"></a>
## fmt

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11261, 1], "end": [11268, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed7148243630d3d4b6b8e6e7"></a>
## hash

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11251, 62], "end": [11251, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b19dd4269adb4f0fdc61590"></a>
## partial_cmp

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CatalogSyncNamespaceMode) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11251, 41], "end": [11251, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11251`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a037c2021c484d6d43bc3a7"></a>
## serialize

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11252, 38], "end": [11252, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11252`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d009afd5466e8e1e3b4bbbf"></a>
## visit

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11253, 40], "end": [11253, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89eb3d07f821b75287cddad3"></a>
## visit

`function` · `sqlparser::ast::CatalogSyncNamespaceMode::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CatalogSyncNamespaceMode", "path": "CatalogSyncNamespaceMode"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11253, 47], "end": [11253, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11253`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
