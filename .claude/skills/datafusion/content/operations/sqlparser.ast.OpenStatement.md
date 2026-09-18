# `sqlparser::ast::OpenStatement`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.OpenStatement.json).

<a id="op-a98d50783fb99de9f615c541"></a>
## OpenStatement

`struct` · `sqlparser::ast::OpenStatement` · sqlparser 0.62.0

```rust
struct OpenStatement
```

Source: `src/ast/mod.rs:11373`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents an `OPEN` statement.

<a id="op-3da9237782cababe5dc683d5"></a>
## clone

`function` · `sqlparser::ast::OpenStatement::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> OpenStatement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11370, 17], "end": [11370, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06e29ec708bd1110f18257a2"></a>
## cmp

`function` · `sqlparser::ast::OpenStatement::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &OpenStatement) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11370, 51], "end": [11370, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58f7d52cde6ec1b0f5030a9d"></a>
## cursor_name

`struct_field` · `sqlparser::ast::OpenStatement::cursor_name` · sqlparser 0.62.0

```rust
cursor_name: Ident
```

Source: `src/ast/mod.rs:11375`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Cursor name

<a id="op-12c6e40e08891daf8665beae"></a>
## deserialize

`function` · `sqlparser::ast::OpenStatement::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11371, 49], "end": [11371, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c33c039ffc8c326279bbe1d2"></a>
## eq

`function` · `sqlparser::ast::OpenStatement::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &OpenStatement) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11370, 24], "end": [11370, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bb9a5cd4a0936d7c122e470"></a>
## fmt

`function` · `sqlparser::ast::OpenStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11378, 1], "end": [11382, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11379`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52dd4536c074eea4523587f4"></a>
## fmt

`function` · `sqlparser::ast::OpenStatement::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11370, 10], "end": [11370, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a981de974cbf9c0022d5acd"></a>
## hash

`function` · `sqlparser::ast::OpenStatement::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11370, 56], "end": [11370, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95b4b00933a912f0b1db67fd"></a>
## partial_cmp

`function` · `sqlparser::ast::OpenStatement::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &OpenStatement) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11370, 35], "end": [11370, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11370`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9bf055007774df305079ffd"></a>
## serialize

`function` · `sqlparser::ast::OpenStatement::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11371, 38], "end": [11371, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11371`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66a4c0b5f896d47742f1243f"></a>
## span

`function` · `sqlparser::ast::OpenStatement::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "super::OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2432, 1], "end": [2437, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22665de95403e26f7c99a8b8"></a>
## visit

`function` · `sqlparser::ast::OpenStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11372, 47], "end": [11372, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11372`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35b0c568c21bbcb02de48fc6"></a>
## visit

`function` · `sqlparser::ast::OpenStatement::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::OpenStatement", "path": "OpenStatement"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11372, 40], "end": [11372, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11372`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
