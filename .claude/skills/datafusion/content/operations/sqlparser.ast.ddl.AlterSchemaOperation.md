# `sqlparser::ast::ddl::AlterSchemaOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterSchemaOperation.json).

<a id="op-ce2c0acd0a21815e50747847"></a>
## AlterSchemaOperation

`enum` · `sqlparser::ast::ddl::AlterSchemaOperation` · sqlparser 0.62.0

```rust
enum AlterSchemaOperation
```

Source: `src/ast/ddl.rs:3798`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `ALTER SCHEMA` (`Statement::AlterSchema`) operation.

See [BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#alter_schema_collate_statement)
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-alterschema.html)

<a id="op-b2303953688cc2c03ea66c3a"></a>
## AddReplica

`variant` · `sqlparser::ast::ddl::AlterSchemaOperation::AddReplica` · sqlparser 0.62.0

```rust
AddReplica
```

Source: `src/ast/ddl.rs:3805`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Add a replica to the schema.

<a id="op-19e007052cb8ee6dc3f75dc1"></a>
## DropReplica

`variant` · `sqlparser::ast::ddl::AlterSchemaOperation::DropReplica` · sqlparser 0.62.0

```rust
DropReplica
```

Source: `src/ast/ddl.rs:3812`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Drop a replica from the schema.

<a id="op-5811bdcada9490db3e079d1e"></a>
## OwnerTo

`variant` · `sqlparser::ast::ddl::AlterSchemaOperation::OwnerTo` · sqlparser 0.62.0

```rust
OwnerTo
```

Source: `src/ast/ddl.rs:3827`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Change the owner of the schema.

<a id="op-9edf93c2a58389df285ca5b6"></a>
## Rename

`variant` · `sqlparser::ast::ddl::AlterSchemaOperation::Rename` · sqlparser 0.62.0

```rust
Rename
```

Source: `src/ast/ddl.rs:3822`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Rename the schema.

<a id="op-da7ff5865665cbbbcedca882"></a>
## SetDefaultCollate

`variant` · `sqlparser::ast::ddl::AlterSchemaOperation::SetDefaultCollate` · sqlparser 0.62.0

```rust
SetDefaultCollate
```

Source: `src/ast/ddl.rs:3800`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set the default collation for the schema.

<a id="op-088b190a4d52fc816c4246df"></a>
## SetOptionsParens

`variant` · `sqlparser::ast::ddl::AlterSchemaOperation::SetOptionsParens` · sqlparser 0.62.0

```rust
SetOptionsParens
```

Source: `src/ast/ddl.rs:3817`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Set options for the schema.

<a id="op-fe7c0d3cd0d279d5401a98b5"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterSchemaOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3795, 17], "end": [3795, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:3795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-725863b628ed544966b50d00"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterSchemaOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3795, 51], "end": [3795, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:3795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c54c3705f0071b810d6cdf7a"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3796, 49], "end": [3796, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:3796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5742d71b2beab357106390fb"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterSchemaOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3795, 24], "end": [3795, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:3795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bc40e061038866b0f647941"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3833, 1], "end": [3854, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:3834`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8df0fceb4a9e0fab18e8162"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3795, 10], "end": [3795, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:3795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-985c17a0766dfd4278bb2cc9"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3795, 56], "end": [3795, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:3795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-308104be5672111407127041"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterSchemaOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3795, 35], "end": [3795, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:3795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-053f6a12e7c97a40de17a1da"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3796, 38], "end": [3796, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:3796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96711582d8c96c4d272fe34b"></a>
## span

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "crate::ast::AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2439, 1], "end": [2461, 2], "filename": "src/ast/spans.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/spans.rs:2440`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62481757d2c682f7198e83ea"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3797, 47], "end": [3797, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:3797`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad039785c3ae4b1265a1ed76"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterSchemaOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterSchemaOperation", "path": "AlterSchemaOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3797, 40], "end": [3797, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:3797`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
