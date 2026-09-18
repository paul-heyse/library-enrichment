# `sqlparser::ast::ddl::DropExtension`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.DropExtension.json).

<a id="op-be121ae57cc68acdc92cd510"></a>
## DropExtension

`struct` · `sqlparser::ast::ddl::DropExtension` · sqlparser 0.62.0

```rust
struct DropExtension
```

Source: `src/ast/ddl.rs:4478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

DROP EXTENSION statement
Note: this is a PostgreSQL-specific statement

# References

PostgreSQL Documentation:
<https://www.postgresql.org/docs/current/sql-dropextension.html>

<a id="op-2dd5e890d9d22bab3ed091b2"></a>
## cascade_or_restrict

`struct_field` · `sqlparser::ast::ddl::DropExtension::cascade_or_restrict` · sqlparser 0.62.0

```rust
cascade_or_restrict: Option<ReferentialAction>
```

Source: `src/ast/ddl.rs:4484`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`CASCADE` or `RESTRICT` behaviour for the drop.

<a id="op-18cf212fdd8b7fb524c9dc71"></a>
## clone

`function` · `sqlparser::ast::ddl::DropExtension::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DropExtension
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4475, 17], "end": [4475, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cecd0d1ebc814ca375801b5"></a>
## cmp

`function` · `sqlparser::ast::ddl::DropExtension::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DropExtension) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4475, 51], "end": [4475, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc010e2dccad4233c3969f34"></a>
## deserialize

`function` · `sqlparser::ast::ddl::DropExtension::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4476, 49], "end": [4476, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7127110fb50c8ecca8059639"></a>
## eq

`function` · `sqlparser::ast::ddl::DropExtension::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DropExtension) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4475, 24], "end": [4475, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9416d1318b7a602896a9a03b"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropExtension::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4487, 1], "end": [4499, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4488`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba25f42730374866a4d8a6f8"></a>
## fmt

`function` · `sqlparser::ast::ddl::DropExtension::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4475, 10], "end": [4475, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-856044b9ff7b3ddf812f9903"></a>
## hash

`function` · `sqlparser::ast::ddl::DropExtension::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4475, 56], "end": [4475, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc4a338b24680427126441f6"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::DropExtension::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:4482`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF EXISTS` was specified for the DROP EXTENSION.

<a id="op-3364cffc4b563ffc1709f3f4"></a>
## names

`struct_field` · `sqlparser::ast::ddl::DropExtension::names` · sqlparser 0.62.0

```rust
names: Vec<ast::Ident>
```

Source: `src/ast/ddl.rs:4480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

One or more extension names to drop

<a id="op-edbc761b9d0138d52b5e5bdd"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::DropExtension::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DropExtension) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4475, 35], "end": [4475, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4475`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad71f731b6bbc7c6bda6e221"></a>
## serialize

`function` · `sqlparser::ast::ddl::DropExtension::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4476, 38], "end": [4476, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e4f2e3253909749e533b398"></a>
## span

`function` · `sqlparser::ast::ddl::DropExtension::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4501, 1], "end": [4505, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:4502`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17d79d6ffaeec6a75c3ac018"></a>
## visit

`function` · `sqlparser::ast::ddl::DropExtension::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4477, 40], "end": [4477, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4477`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db31cd084b9bd228cff4bbbd"></a>
## visit

`function` · `sqlparser::ast::ddl::DropExtension::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::DropExtension", "path": "DropExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4477, 47], "end": [4477, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4477`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
