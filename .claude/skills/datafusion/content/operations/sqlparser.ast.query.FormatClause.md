# `sqlparser::ast::query::FormatClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.FormatClause.json).

<a id="op-07bb440dc552e68db134c7b0"></a>
## FormatClause

`enum` · `sqlparser::ast::query::FormatClause` · sqlparser 0.62.0

```rust
enum FormatClause
```

Source: `src/ast/query.rs:3790`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FORMAT` identifier or `FORMAT NULL` clause, specific to ClickHouse.

[ClickHouse]: <https://clickhouse.com/docs/en/sql-reference/statements/select/format>

<a id="op-c1359f3e6bfcd206ee09963f"></a>
## Identifier

`variant` · `sqlparser::ast::query::FormatClause::Identifier` · sqlparser 0.62.0

```rust
Identifier
```

Source: `src/ast/query.rs:3792`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The format identifier.

<a id="op-ef6d02d9d484b7b28743fa79"></a>
## Null

`variant` · `sqlparser::ast::query::FormatClause::Null` · sqlparser 0.62.0

```rust
Null
```

Source: `src/ast/query.rs:3794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`FORMAT NULL` clause.

<a id="op-960ee180bed05f8564e41137"></a>
## clone

`function` · `sqlparser::ast::query::FormatClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FormatClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3787, 17], "end": [3787, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a9a58b1069f7ee96f69b014"></a>
## cmp

`function` · `sqlparser::ast::query::FormatClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FormatClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3787, 51], "end": [3787, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3aad6f2863730f484672843c"></a>
## deserialize

`function` · `sqlparser::ast::query::FormatClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3788, 49], "end": [3788, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3788`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7534ec0bb1ce5f1ebd43fa5e"></a>
## eq

`function` · `sqlparser::ast::query::FormatClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FormatClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3787, 24], "end": [3787, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1efec19d66a4f19c23e75d08"></a>
## fmt

`function` · `sqlparser::ast::query::FormatClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3787, 10], "end": [3787, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50469925b6affa8c7d923e91"></a>
## fmt

`function` · `sqlparser::ast::query::FormatClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3797, 1], "end": [3804, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3798`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-93e443ba7ee0b9358d35542b"></a>
## hash

`function` · `sqlparser::ast::query::FormatClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3787, 56], "end": [3787, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e86b6397e6c7cc22ce5e42b"></a>
## partial_cmp

`function` · `sqlparser::ast::query::FormatClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FormatClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3787, 35], "end": [3787, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3787`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fce18e3be4dfd27a0ee21f77"></a>
## serialize

`function` · `sqlparser::ast::query::FormatClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3788, 38], "end": [3788, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3788`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1ac69ff4497b03205030d08c"></a>
## visit

`function` · `sqlparser::ast::query::FormatClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3789, 40], "end": [3789, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db212ee273cf9de7eceb9476"></a>
## visit

`function` · `sqlparser::ast::query::FormatClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::FormatClause", "path": "FormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3789, 47], "end": [3789, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3789`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
