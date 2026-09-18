# `sqlparser::ast::JsonReturningClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.JsonReturningClause.json).

<a id="op-04febe2469461f60ae48a9d5"></a>
## JsonReturningClause

`struct` · `sqlparser::ast::JsonReturningClause` · sqlparser 0.62.0

```rust
struct JsonReturningClause
```

Source: `src/ast/mod.rs:10967`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL JSON function RETURNING clause

Example:
```sql
JSON_OBJECT('a': 1 RETURNING jsonb)
```

<a id="op-2c86f2a6ec8ba635036d4827"></a>
## clone

`function` · `sqlparser::ast::JsonReturningClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> JsonReturningClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10964, 17], "end": [10964, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:10964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-181a4d36ae3bcb73be74b8da"></a>
## cmp

`function` · `sqlparser::ast::JsonReturningClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &JsonReturningClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10964, 51], "end": [10964, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:10964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-376e3c0af2d835ec29293055"></a>
## data_type

`struct_field` · `sqlparser::ast::JsonReturningClause::data_type` · sqlparser 0.62.0

```rust
data_type: DataType
```

Source: `src/ast/mod.rs:10969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The data type to return from the JSON function (e.g. JSON/JSONB).

<a id="op-ce97ca257501357d81cafa61"></a>
## deserialize

`function` · `sqlparser::ast::JsonReturningClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [10965, 49], "end": [10965, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:10965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79abdccdb32e5a8ba9b4db54"></a>
## eq

`function` · `sqlparser::ast::JsonReturningClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &JsonReturningClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10964, 24], "end": [10964, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:10964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ee7b55f719dc76b9967f533"></a>
## fmt

`function` · `sqlparser::ast::JsonReturningClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10972, 1], "end": [10976, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:10973`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8ce42458eb4ef423a0e7053"></a>
## fmt

`function` · `sqlparser::ast::JsonReturningClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10964, 10], "end": [10964, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:10964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0fba7d1571d013f7da74df1"></a>
## hash

`function` · `sqlparser::ast::JsonReturningClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10964, 56], "end": [10964, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:10964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c1f062d166c346b1cfe5f1e"></a>
## partial_cmp

`function` · `sqlparser::ast::JsonReturningClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &JsonReturningClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10964, 35], "end": [10964, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:10964`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a34f919925d9d935b840c6b"></a>
## serialize

`function` · `sqlparser::ast::JsonReturningClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10965, 38], "end": [10965, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:10965`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34dce8714ef31d85eed7114c"></a>
## visit

`function` · `sqlparser::ast::JsonReturningClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10966, 47], "end": [10966, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:10966`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1203bdf9eb47bd8ae08510d"></a>
## visit

`function` · `sqlparser::ast::JsonReturningClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::JsonReturningClause", "path": "JsonReturningClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [10966, 40], "end": [10966, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:10966`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
