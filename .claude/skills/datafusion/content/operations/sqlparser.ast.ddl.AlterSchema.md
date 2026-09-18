# `sqlparser::ast::ddl::AlterSchema`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterSchema.json).

<a id="op-fdc24a9d39a31757d195930f"></a>
## AlterSchema

`struct` · `sqlparser::ast::ddl::AlterSchema` · sqlparser 0.62.0

```rust
struct AlterSchema
```

Source: `src/ast/ddl.rs:3883`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `ALTER SCHEMA` (`Statement::AlterSchema`) statement.

<a id="op-7665d822b8782a0b34d1f8e2"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterSchema::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterSchema
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3879, 17], "end": [3879, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3879`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70a04a32894f695702bbc16e"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterSchema::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterSchema) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3879, 51], "end": [3879, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3879`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8eddd46ac254c7e69bc7e39e"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterSchema::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3880, 49], "end": [3880, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3023c5b26bc4f240107c1576"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterSchema::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterSchema) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3879, 24], "end": [3879, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3879`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fbb295e2fe4aef871c9f862"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterSchema::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3892, 1], "end": [3905, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3893`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aea7f120a12a126d05041ef0"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterSchema::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3879, 10], "end": [3879, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3879`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20d681a1348ae536fffa658e"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterSchema::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3879, 56], "end": [3879, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3879`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e16fa2ba7f87e2b6f450630"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::AlterSchema::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:3887`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF EXISTS` was specified.

<a id="op-bc42077a64c6401a152c9a86"></a>
## name

`struct_field` · `sqlparser::ast::ddl::AlterSchema::name` · sqlparser 0.62.0

```rust
name: ast::ObjectName
```

Source: `src/ast/ddl.rs:3885`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The schema name to alter.

<a id="op-d66d6af937c66372b3638c07"></a>
## operations

`struct_field` · `sqlparser::ast::ddl::AlterSchema::operations` · sqlparser 0.62.0

```rust
operations: Vec<AlterSchemaOperation>
```

Source: `src/ast/ddl.rs:3889`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The list of operations to perform on the schema.

<a id="op-203680b84a61c286cf03560d"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterSchema::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterSchema) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3879, 35], "end": [3879, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3879`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-27d0003ab3f3da79667df510"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterSchema::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3880, 38], "end": [3880, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3880`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b378ee1a991be8acbb40e852"></a>
## span

`function` · `sqlparser::ast::ddl::AlterSchema::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "crate::ast::ddl::AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2463, 1], "end": [2469, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2464`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09c5f17799386dbb8a521bae"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterSchema::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3881, 47], "end": [3881, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3881`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6de57b47d875e9696be37b2"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterSchema::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchema", "path": "AlterSchema"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3881, 40], "end": [3881, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3881`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
