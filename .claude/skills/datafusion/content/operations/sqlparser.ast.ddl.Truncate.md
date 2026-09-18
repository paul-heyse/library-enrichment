# `sqlparser::ast::ddl::Truncate`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.Truncate.json).

<a id="op-6f3214cf7574370b07091acc"></a>
## Truncate

`struct` · `sqlparser::ast::ddl::Truncate` · sqlparser 0.62.0

```rust
struct Truncate
```

Source: `src/ast/ddl.rs:4184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A `TRUNCATE` statement.

```sql
TRUNCATE TABLE [IF EXISTS] table_names [PARTITION (partitions)] [RESTART IDENTITY | CONTINUE IDENTITY] [CASCADE | RESTRICT] [ON CLUSTER cluster_name]
```

<a id="op-63f9b05afba0fea451b3858e"></a>
## cascade

`struct_field` · `sqlparser::ast::ddl::Truncate::cascade` · sqlparser 0.62.0

```rust
cascade: Option<super::CascadeOption>
```

Source: `src/ast/ddl.rs:4196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Postgres-specific option: [ CASCADE | RESTRICT ]

<a id="op-221a32528ab87088497722bf"></a>
## clone

`function` · `sqlparser::ast::ddl::Truncate::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Truncate
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4181, 17], "end": [4181, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b7acef3d10ae288185471a8"></a>
## cmp

`function` · `sqlparser::ast::ddl::Truncate::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Truncate) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4181, 51], "end": [4181, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e61e6823041ef215cc488a1"></a>
## deserialize

`function` · `sqlparser::ast::ddl::Truncate::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4182, 49], "end": [4182, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1dfcfcbfbcd0e36f32d5b738"></a>
## eq

`function` · `sqlparser::ast::ddl::Truncate::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Truncate) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4181, 24], "end": [4181, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb3e34c130e4196929cde1db"></a>
## fmt

`function` · `sqlparser::ast::ddl::Truncate::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4202, 1], "end": [4236, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:4203`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee4295890f51a151c4dfb480"></a>
## fmt

`function` · `sqlparser::ast::ddl::Truncate::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4181, 10], "end": [4181, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a527b2cc81d7777f98c6ddb"></a>
## hash

`function` · `sqlparser::ast::ddl::Truncate::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4181, 56], "end": [4181, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d5e09a80717f8d55b78471c"></a>
## identity

`struct_field` · `sqlparser::ast::ddl::Truncate::identity` · sqlparser 0.62.0

```rust
identity: Option<super::TruncateIdentityOption>
```

Source: `src/ast/ddl.rs:4194`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Postgres-specific option: [ RESTART IDENTITY | CONTINUE IDENTITY ]

<a id="op-1176da5ee17e51c910b3f0de"></a>
## if_exists

`struct_field` · `sqlparser::ast::ddl::Truncate::if_exists` · sqlparser 0.62.0

```rust
if_exists: bool
```

Source: `src/ast/ddl.rs:4192`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake/Redshift-specific option: [ IF EXISTS ]

<a id="op-ff2515f1a74b62a77d6fe7ce"></a>
## on_cluster

`struct_field` · `sqlparser::ast::ddl::Truncate::on_cluster` · sqlparser 0.62.0

```rust
on_cluster: Option<ast::Ident>
```

Source: `src/ast/ddl.rs:4199`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

ClickHouse-specific option: [ ON CLUSTER cluster_name ]
[ClickHouse](https://clickhouse.com/docs/en/sql-reference/statements/truncate/)

<a id="op-d3099fe565684f7b58234c5e"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::Truncate::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Truncate) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4181, 35], "end": [4181, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e757960f006035d8d49d8219"></a>
## partitions

`struct_field` · `sqlparser::ast::ddl::Truncate::partitions` · sqlparser 0.62.0

```rust
partitions: Option<Vec<ast::Expr>>
```

Source: `src/ast/ddl.rs:4188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional partition specification

<a id="op-d4df3114cbd9369aed62af2a"></a>
## serialize

`function` · `sqlparser::ast::ddl::Truncate::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4182, 38], "end": [4182, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4182`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee8abd430dbb40b2d8767288"></a>
## span

`function` · `sqlparser::ast::ddl::Truncate::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4238, 1], "end": [4248, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/ddl.rs:4239`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-936285a8851c6ad05b1edaad"></a>
## table

`struct_field` · `sqlparser::ast::ddl::Truncate::table` · sqlparser 0.62.0

```rust
table: bool
```

Source: `src/ast/ddl.rs:4190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

TABLE - optional keyword

<a id="op-c33bf43c5ad2a12487f9f903"></a>
## table_names

`struct_field` · `sqlparser::ast::ddl::Truncate::table_names` · sqlparser 0.62.0

```rust
table_names: Vec<super::TruncateTableTarget>
```

Source: `src/ast/ddl.rs:4186`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table names to truncate

<a id="op-4f839ba3229b4df7c99343c5"></a>
## visit

`function` · `sqlparser::ast::ddl::Truncate::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4183, 40], "end": [4183, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-827c7bedb33cf0babee842c6"></a>
## visit

`function` · `sqlparser::ast::ddl::Truncate::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::Truncate", "path": "Truncate"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4183, 47], "end": [4183, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4183`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
