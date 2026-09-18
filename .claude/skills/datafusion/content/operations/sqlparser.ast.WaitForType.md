# `sqlparser::ast::WaitForType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.WaitForType.json).

<a id="op-0657fcf9c83d490afa393b35"></a>
## WaitForType

`enum` · `sqlparser::ast::WaitForType` · sqlparser 0.62.0

```rust
enum WaitForType
```

Source: `src/ast/mod.rs:11304`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The type of `WAITFOR` statement (MSSQL).

See: <https://learn.microsoft.com/en-us/sql/t-sql/language-elements/waitfor-transact-sql>

<a id="op-628acb4b7cffcfc4748d33eb"></a>
## Delay

`variant` · `sqlparser::ast::WaitForType::Delay` · sqlparser 0.62.0

```rust
Delay
```

Source: `src/ast/mod.rs:11306`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WAITFOR DELAY 'time_to_pass'`

<a id="op-1d96d9ce024b0c2bdd872171"></a>
## Time

`variant` · `sqlparser::ast::WaitForType::Time` · sqlparser 0.62.0

```rust
Time
```

Source: `src/ast/mod.rs:11308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`WAITFOR TIME 'time_to_execute'`

<a id="op-7accb5b62240c98ab4523731"></a>
## clone

`function` · `sqlparser::ast::WaitForType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> WaitForType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11301, 17], "end": [11301, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea372615d25fd8decf0d215e"></a>
## cmp

`function` · `sqlparser::ast::WaitForType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &WaitForType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11301, 51], "end": [11301, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b420c490e3f6fb658ebee6bb"></a>
## deserialize

`function` · `sqlparser::ast::WaitForType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11302, 49], "end": [11302, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd6ec008e8cc53cc86da4b61"></a>
## eq

`function` · `sqlparser::ast::WaitForType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &WaitForType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11301, 24], "end": [11301, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a8c6acf5e1ea914cdb36c6d"></a>
## fmt

`function` · `sqlparser::ast::WaitForType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11311, 1], "end": [11318, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11312`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3bee0c0ebdb7cd8688e3b92"></a>
## fmt

`function` · `sqlparser::ast::WaitForType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11301, 10], "end": [11301, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2b3c362e69b08135e56c864"></a>
## hash

`function` · `sqlparser::ast::WaitForType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11301, 56], "end": [11301, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5862ba7198af7dfd9fa67d46"></a>
## partial_cmp

`function` · `sqlparser::ast::WaitForType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &WaitForType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11301, 35], "end": [11301, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11301`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5ddd78a103ed143a235d22b"></a>
## serialize

`function` · `sqlparser::ast::WaitForType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11302, 38], "end": [11302, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11302`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ad1e264f804155d84b54672"></a>
## visit

`function` · `sqlparser::ast::WaitForType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11303, 40], "end": [11303, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2659b822eee169df9ffb558"></a>
## visit

`function` · `sqlparser::ast::WaitForType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::WaitForType", "path": "WaitForType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11303, 47], "end": [11303, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11303`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
