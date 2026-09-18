# `sqlparser::ast::ddl::ColumnPolicy`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.ColumnPolicy.json).

<a id="op-1db412d11c83cfb48e9ffdcb"></a>
## ColumnPolicy

`enum` · `sqlparser::ast::ddl::ColumnPolicy` · sqlparser 0.62.0

```rust
enum ColumnPolicy
```

Source: `src/ast/ddl.rs:1816`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Column policy that identify a security policy of access to a column.
Syntax
```sql
[ WITH ] MASKING POLICY <policy_name> [ USING ( <col_name> , <cond_col1> , ... ) ]
[ WITH ] PROJECTION POLICY <policy_name>
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-28cf719fd0af7c044d862780"></a>
## MaskingPolicy

`variant` · `sqlparser::ast::ddl::ColumnPolicy::MaskingPolicy` · sqlparser 0.62.0

```rust
MaskingPolicy
```

Source: `src/ast/ddl.rs:1818`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`MASKING POLICY (<property>)`

<a id="op-703a87fff26f949237b5e545"></a>
## ProjectionPolicy

`variant` · `sqlparser::ast::ddl::ColumnPolicy::ProjectionPolicy` · sqlparser 0.62.0

```rust
ProjectionPolicy
```

Source: `src/ast/ddl.rs:1820`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`PROJECTION POLICY (<property>)`

<a id="op-700ee65b2dd4aa6b791bd626"></a>
## clone

`function` · `sqlparser::ast::ddl::ColumnPolicy::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ColumnPolicy
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1813, 17], "end": [1813, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5f55c70e0301d65be8ea382"></a>
## cmp

`function` · `sqlparser::ast::ddl::ColumnPolicy::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ColumnPolicy) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1813, 51], "end": [1813, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e6704c7ea2c1e4f614fb9f0"></a>
## deserialize

`function` · `sqlparser::ast::ddl::ColumnPolicy::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1814, 49], "end": [1814, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c3b1df3d4c47b851ad444db"></a>
## eq

`function` · `sqlparser::ast::ddl::ColumnPolicy::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ColumnPolicy) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1813, 24], "end": [1813, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-127abd621d5b28b06877ed3e"></a>
## fmt

`function` · `sqlparser::ast::ddl::ColumnPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1823, 1], "end": [1838, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1824`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abf53c75761846f06913b922"></a>
## fmt

`function` · `sqlparser::ast::ddl::ColumnPolicy::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1813, 10], "end": [1813, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf8ae4f2ac8cff46b88bc1ae"></a>
## hash

`function` · `sqlparser::ast::ddl::ColumnPolicy::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1813, 56], "end": [1813, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72f8fdd6eaba4fc29b7fe795"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::ColumnPolicy::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ColumnPolicy) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1813, 35], "end": [1813, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1813`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c51b838d22d79bf8d9f4bfd4"></a>
## serialize

`function` · `sqlparser::ast::ddl::ColumnPolicy::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1814, 38], "end": [1814, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ae8eddcfb9eae774aa0335b"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1815, 40], "end": [1815, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1815`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c74f2c3aa069eeaa26c67a7"></a>
## visit

`function` · `sqlparser::ast::ddl::ColumnPolicy::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::ColumnPolicy", "path": "ColumnPolicy"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1815, 47], "end": [1815, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1815`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
