# `sqlparser::ast::ddl::DropOperatorClass`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DropOperatorClass.json).

<a id="op-e8426bb81c6870c967c95660"></a>
## DropOperatorClass

`struct` · `sqlparser::ast::ddl::DropOperatorClass` · sqlparser 0.62.0

```rust
struct DropOperatorClass
```

Source: `src/ast/ddl.rs:5068`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`DROP OPERATOR CLASS` statement
See <https://www.postgresql.org/docs/current/sql-dropopclass.html>

<a id="op-066015880693b1ff2ea999d9"></a>
## clone

`function` · `sqlparser::ast::ddl::DropOperatorClass::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropOperatorClass
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5065, 17], "end": [5065, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-94143aa5a4da45dbcf17375d"></a>
## cmp

`function` · `sqlparser::ast::ddl::DropOperatorClass::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropOperatorClass) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5065, 51], "end": [5065, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83ee004d0879da64307dadb0"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DropOperatorClass::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5066, 49], "end": [5066, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5066`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c84b0ea26669ec9c7054df7f"></a>
## drop_behavior

`struct_field` · `sqlparser::ast::ddl::DropOperatorClass::drop_behavior` · sqlparser 0.62.0

```rust
drop_behavior: Option<DropBehavior>
```

Source: `src/ast/ddl.rs:5076`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CASCADE or RESTRICT`

<a id="op-ea7fa6ff51003dc16ca8d593"></a>
## eq

`function` · `sqlparser::ast::ddl::DropOperatorClass::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropOperatorClass) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5065, 24], "end": [5065, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8abd853d253e27b9fc6106b9"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropOperatorClass::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5065, 10], "end": [5065, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7fe70e3e84eedd38b63925a"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropOperatorClass::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5079, 1], "end": [5092, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5080`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ba4fe9e3b89eca4ccb80dce"></a>
## hash

`function` · `sqlparser::ast::ddl::DropOperatorClass::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5065, 56], "end": [5065, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bf299653fc5c29ad1f3c3a48"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::DropOperatorClass::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:5070`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`IF EXISTS` clause

<a id="op-4518a03a5c01bf0f370d04da"></a>
## names

`struct_field` · `sqlparser::ast::ddl::DropOperatorClass::names` · sqlparser 0.62.0

```rust
names: Vec<ast::ObjectName>
```

Source: `src/ast/ddl.rs:5072`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One or more operator classes to drop

<a id="op-c06272b744c03f8b1c91bf9a"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DropOperatorClass::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropOperatorClass) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5065, 35], "end": [5065, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5065`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0eceb2c704962618aed336f8"></a>
## serialize

`function` · `sqlparser::ast::ddl::DropOperatorClass::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5066, 38], "end": [5066, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5066`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-482ffd691f543aee7dca7111"></a>
## span

`function` · `sqlparser::ast::ddl::DropOperatorClass::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5094, 1], "end": [5098, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:5095`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a506628c6f818ebfcbdd305"></a>
## using

`struct_field` · `sqlparser::ast::ddl::DropOperatorClass::using` · sqlparser 0.62.0

```rust
using: ast::Ident
```

Source: `src/ast/ddl.rs:5074`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Index method (btree, hash, gist, gin, etc.)

<a id="op-685d36fbc5a65ed7af11b3d9"></a>
## visit

`function` · `sqlparser::ast::ddl::DropOperatorClass::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5067, 40], "end": [5067, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5067`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-91db083df294a2444b75bb75"></a>
## visit

`function` · `sqlparser::ast::ddl::DropOperatorClass::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropOperatorClass", "path": "DropOperatorClass"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5067, 47], "end": [5067, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5067`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
