# `sqlparser::ast::query::InputFormatClause`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.query.InputFormatClause.json).

<a id="op-aa9a69c8a5e51defcfc76f8c"></a>
## InputFormatClause

`struct` · `sqlparser::ast::query::InputFormatClause` · sqlparser 0.62.0

```rust
struct InputFormatClause
```

Source: `src/ast/query.rs:3812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

FORMAT identifier in input context, specific to ClickHouse.

[ClickHouse]: <https://clickhouse.com/docs/en/interfaces/formats>

<a id="op-80ec3446cba634a88c499ca0"></a>
## clone

`function` · `sqlparser::ast::query::InputFormatClause::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> InputFormatClause
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3809, 17], "end": [3809, 22], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/query.rs:3809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3667423254bb5c6f272276f6"></a>
## cmp

`function` · `sqlparser::ast::query::InputFormatClause::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &InputFormatClause) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3809, 51], "end": [3809, 54], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/query.rs:3809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-572998ca27f50f89af133daa"></a>
## deserialize

`function` · `sqlparser::ast::query::InputFormatClause::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3810, 49], "end": [3810, 60], "filename": "src/ast/query.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/query.rs:3810`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44ddbcee5e365d667f35f138"></a>
## eq

`function` · `sqlparser::ast::query::InputFormatClause::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &InputFormatClause) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3809, 24], "end": [3809, 33], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/query.rs:3809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b0479bd2c3f8d6bc7d61724"></a>
## fmt

`function` · `sqlparser::ast::query::InputFormatClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3809, 10], "end": [3809, 15], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/query.rs:3809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-beaed9c2e852affd0869ab51"></a>
## fmt

`function` · `sqlparser::ast::query::InputFormatClause::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3819, 1], "end": [3829, 2], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/query.rs:3820`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-506a08b6cd683b6fae6e49da"></a>
## hash

`function` · `sqlparser::ast::query::InputFormatClause::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3809, 56], "end": [3809, 60], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/query.rs:3809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df3db83cc75adabb5b5f685f"></a>
## ident

`struct_field` · `sqlparser::ast::query::InputFormatClause::ident` · sqlparser 0.62.0

```rust
ident: Ident
```

Source: `src/ast/query.rs:3814`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The format identifier.

<a id="op-9dab740a1470552e36700271"></a>
## partial_cmp

`function` · `sqlparser::ast::query::InputFormatClause::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &InputFormatClause) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3809, 35], "end": [3809, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/query.rs:3809`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ee065f2514189b8ad9e60ad"></a>
## serialize

`function` · `sqlparser::ast::query::InputFormatClause::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3810, 38], "end": [3810, 47], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/query.rs:3810`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1de36e673f1483795ed4d6cf"></a>
## values

`struct_field` · `sqlparser::ast::query::InputFormatClause::values` · sqlparser 0.62.0

```rust
values: Vec<Expr>
```

Source: `src/ast/query.rs:3816`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional format parameters.

<a id="op-67096b83e4c1394fe7ac5539"></a>
## visit

`function` · `sqlparser::ast::query::InputFormatClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3811, 47], "end": [3811, 55], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/query.rs:3811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7895e27707dbde32481d5f36"></a>
## visit

`function` · `sqlparser::ast::query::InputFormatClause::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::query::InputFormatClause", "path": "InputFormatClause"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3811, 40], "end": [3811, 45], "filename": "src/ast/query.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/query.rs:3811`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
