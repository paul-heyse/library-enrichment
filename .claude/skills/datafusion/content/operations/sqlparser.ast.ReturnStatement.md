# `sqlparser::ast::ReturnStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ReturnStatement.json).

<a id="op-7da02e9c4e773dfbb56635e9"></a>
## ReturnStatement

`struct` · `sqlparser::ast::ReturnStatement` · sqlparser 0.62.0

```rust
struct ReturnStatement
```

Source: `src/ast/mod.rs:11346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents a `Return` statement.

[MsSql triggers](https://learn.microsoft.com/en-us/sql/t-sql/statements/create-trigger-transact-sql)
[MsSql functions](https://learn.microsoft.com/en-us/sql/t-sql/statements/create-function-transact-sql)

<a id="op-700ec0af1f3eb3b8a78be244"></a>
## clone

`function` · `sqlparser::ast::ReturnStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ReturnStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11343, 17], "end": [11343, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af7078ad0b9264e54b32b185"></a>
## cmp

`function` · `sqlparser::ast::ReturnStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ReturnStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11343, 51], "end": [11343, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc89e05f7756abf77cb43ab5"></a>
## deserialize

`function` · `sqlparser::ast::ReturnStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11344, 49], "end": [11344, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11344`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e862628f698c0144ab05defe"></a>
## eq

`function` · `sqlparser::ast::ReturnStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ReturnStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11343, 24], "end": [11343, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fc01646b5646e9a52b327de"></a>
## fmt

`function` · `sqlparser::ast::ReturnStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11343, 10], "end": [11343, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3716eaa82504f0a0ac04976"></a>
## fmt

`function` · `sqlparser::ast::ReturnStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11351, 1], "end": [11358, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11352`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a9df9cae1e4d11e279e5a493"></a>
## hash

`function` · `sqlparser::ast::ReturnStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11343, 56], "end": [11343, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea1e3d89eef6ed4605ee33d0"></a>
## partial_cmp

`function` · `sqlparser::ast::ReturnStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ReturnStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11343, 35], "end": [11343, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-80b96be11369924ef340728c"></a>
## serialize

`function` · `sqlparser::ast::ReturnStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11344, 38], "end": [11344, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11344`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6748c19fcbdca61231f1a6d2"></a>
## value

`struct_field` · `sqlparser::ast::ReturnStatement::value` · sqlparser 0.62.0

```rust
value: Option<ReturnStatementValue>
```

Source: `src/ast/mod.rs:11348`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional return value expression.

<a id="op-28ed974bad5a14db92598e25"></a>
## visit

`function` · `sqlparser::ast::ReturnStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11345, 47], "end": [11345, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11345`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a81f60e82da7351b1bce9d0e"></a>
## visit

`function` · `sqlparser::ast::ReturnStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ReturnStatement", "path": "ReturnStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11345, 40], "end": [11345, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11345`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
