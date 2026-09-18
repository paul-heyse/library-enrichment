# `sqlparser::ast::KillType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.KillType.json).

<a id="op-26987590669bb8ee4737537c"></a>
## KillType

`enum` · `sqlparser::ast::KillType` · sqlparser 0.62.0

```rust
enum KillType
```

Source: `src/ast/mod.rs:8514`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Types supported by `KILL` statements.

<a id="op-6856583921b67d9770d8f396"></a>
## Connection

`variant` · `sqlparser::ast::KillType::Connection` · sqlparser 0.62.0

```rust
Connection
```

Source: `src/ast/mod.rs:8516`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Kill a connection.

<a id="op-432e9b8ac8b83d983d752662"></a>
## Mutation

`variant` · `sqlparser::ast::KillType::Mutation` · sqlparser 0.62.0

```rust
Mutation
```

Source: `src/ast/mod.rs:8520`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Kill a mutation (ClickHouse).

<a id="op-b439cb8b3ffabbd47e09aab4"></a>
## Query

`variant` · `sqlparser::ast::KillType::Query` · sqlparser 0.62.0

```rust
Query
```

Source: `src/ast/mod.rs:8518`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Kill a running query.

<a id="op-b33ec0e3b14e10ad5d3c0585"></a>
## clone

`function` · `sqlparser::ast::KillType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> KillType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8510, 23], "end": [8510, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:8510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3dd893591bab1dfbc96eb623"></a>
## cmp

`function` · `sqlparser::ast::KillType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &KillType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8510, 57], "end": [8510, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:8510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0dd0c838ac99dc58153226b9"></a>
## deserialize

`function` · `sqlparser::ast::KillType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [8511, 49], "end": [8511, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:8511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f10356ce97b182bfa45ff784"></a>
## eq

`function` · `sqlparser::ast::KillType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &KillType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8510, 30], "end": [8510, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:8510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5802d66a5b9506f2e06ae382"></a>
## fmt

`function` · `sqlparser::ast::KillType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8523, 1], "end": [8533, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:8524`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-748510970923a4899fd954fa"></a>
## fmt

`function` · `sqlparser::ast::KillType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8510, 10], "end": [8510, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:8510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c2ed0cda093bb42f96d3beb"></a>
## hash

`function` · `sqlparser::ast::KillType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8510, 62], "end": [8510, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:8510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42b4dfffd9350720541bd08b"></a>
## partial_cmp

`function` · `sqlparser::ast::KillType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &KillType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8510, 41], "end": [8510, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:8510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d1bad29107860a890061c726"></a>
## serialize

`function` · `sqlparser::ast::KillType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8511, 38], "end": [8511, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:8511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-741f2936f34deb4e87d7a67e"></a>
## visit

`function` · `sqlparser::ast::KillType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8512, 47], "end": [8512, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:8512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9615157bbc4a98cdd50f1e6"></a>
## visit

`function` · `sqlparser::ast::KillType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::KillType", "path": "KillType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [8512, 40], "end": [8512, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:8512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
