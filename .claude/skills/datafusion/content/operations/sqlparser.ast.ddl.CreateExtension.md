# `sqlparser::ast::ddl::CreateExtension`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreateExtension.json).

<a id="op-9b931feae42604a66ed6dc8a"></a>
## CreateExtension

`struct` · `sqlparser::ast::ddl::CreateExtension` · sqlparser 0.62.0

```rust
struct CreateExtension
```

Source: `src/ast/ddl.rs:4419`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE EXTENSION statement
Note: this is a PostgreSQL-specific statement

<a id="op-5376bc7ab311653bc69e02af"></a>
## cascade

`struct_field` · `sqlparser::ast::ddl::CreateExtension::cascade` · sqlparser 0.62.0

```rust
cascade: bool
```

Source: `src/ast/ddl.rs:4425`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `CASCADE` was specified for the CREATE EXTENSION.

<a id="op-7b88a3fcf96f4a50efcf7a77"></a>
## clone

`function` · `sqlparser::ast::ddl::CreateExtension::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateExtension
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4416, 17], "end": [4416, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff2c2674f6e791be286f0160"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreateExtension::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateExtension) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4416, 51], "end": [4416, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bcf5f3ed91d64030ef2ab968"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreateExtension::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4417, 49], "end": [4417, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4417`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6dbc8f3b985af0533925b257"></a>
## eq

`function` · `sqlparser::ast::ddl::CreateExtension::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateExtension) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4416, 24], "end": [4416, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7efa215d980ea4263b4d547d"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateExtension::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4416, 10], "end": [4416, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c211267a0553d3d661ae44e"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreateExtension::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4432, 1], "end": [4460, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4433`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a576aac5e0d885b66b32e34"></a>
## hash

`function` · `sqlparser::ast::ddl::CreateExtension::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4416, 56], "end": [4416, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9160cb7214139500d71eda7b"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::ddl::CreateExtension::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/ddl.rs:4423`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF NOT EXISTS` was specified for the CREATE EXTENSION.

<a id="op-c28cf8ffa3b4e5b31cb9efaa"></a>
## name

`struct_field` · `sqlparser::ast::ddl::CreateExtension::name` · sqlparser 0.62.0

```rust
name: ast::Ident
```

Source: `src/ast/ddl.rs:4421`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Extension name

<a id="op-12e76aaf97f88864cf57add7"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreateExtension::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateExtension) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4416, 35], "end": [4416, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4416`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-17ca6431697e7f7c60b69d59"></a>
## schema

`struct_field` · `sqlparser::ast::ddl::CreateExtension::schema` · sqlparser 0.62.0

```rust
schema: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:4427`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional schema name for the extension.

<a id="op-61c04a3e9f957af8632ede69"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreateExtension::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4417, 38], "end": [4417, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4417`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7c20d6ed515747f690d15bb"></a>
## span

`function` · `sqlparser::ast::ddl::CreateExtension::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4462, 1], "end": [4466, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:4463`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b285e630e656d01dacc59af"></a>
## version

`struct_field` · `sqlparser::ast::ddl::CreateExtension::version` · sqlparser 0.62.0

```rust
version: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:4429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional version for the extension.

<a id="op-48485e6ab29bb76d20d3ced8"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateExtension::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4418, 40], "end": [4418, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a7bd2e1d4ef22e908730dc6"></a>
## visit

`function` · `sqlparser::ast::ddl::CreateExtension::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreateExtension", "path": "CreateExtension"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4418, 47], "end": [4418, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4418`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
