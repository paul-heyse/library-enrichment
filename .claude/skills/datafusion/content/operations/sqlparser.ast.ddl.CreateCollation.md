# `sqlparser::ast::ddl::CreateCollation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateCollation.json).

<a id="op-6c91190111643c6951f71be2"></a>
## CreateCollation

`struct` · `sqlparser::ast::ddl::CreateCollation` · sqlparser 0.62.0

```rust
struct CreateCollation
```

Source: `src/ast/ddl.rs:4512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE COLLATION statement.
Note: this is a PostgreSQL-specific statement.

<a id="op-f97a5dc36f747c3e1f921ac0"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateCollation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateCollation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4509, 17], "end": [4509, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f14eff57d8b5d53f70ee1e78"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateCollation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateCollation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4509, 51], "end": [4509, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23594f875d8bdb7881b247d4"></a>
## definition

`struct_field` · `sqlparser::ast::ddl::CreateCollation::definition` · sqlparser 0.62.0

```rust
definition: CreateCollationDefinition
```

Source: `src/ast/ddl.rs:4518`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Source definition for the collation.

<a id="op-6ea960f5f779c2affa17f706"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateCollation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4510, 49], "end": [4510, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b535ffd636fa74f39fe18100"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateCollation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateCollation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4509, 24], "end": [4509, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8fa47da45ebd77e8698e699"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateCollation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4509, 10], "end": [4509, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e81822c1de8e84880a99013d"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateCollation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4540, 1], "end": [4561, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4541`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c0e2c95a3e7008ed36b0ff53"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateCollation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4509, 56], "end": [4509, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58e4583f8328b3c6f4f3dfcc"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::CreateCollation::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:4514`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF NOT EXISTS` was specified.

<a id="op-5cbdd6f28e052c3087527ee1"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateCollation::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:4516`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Name of the collation being created.

<a id="op-de2c7abf421c1cc918589bfd"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateCollation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateCollation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4509, 35], "end": [4509, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e3405065ea2ded4c40fac57"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateCollation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4510, 38], "end": [4510, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b661836a9f91797cd08bd16"></a>
## span

`function` · `sqlparser::ast::ddl::CreateCollation::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4563, 1], "end": [4567, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:4564`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-595a7a39bea61c6c5eb87eb1"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateCollation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4511, 40], "end": [4511, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95d2b2a8800f9a9d43293113"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateCollation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateCollation", "path": "CreateCollation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4511, 47], "end": [4511, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4511`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
