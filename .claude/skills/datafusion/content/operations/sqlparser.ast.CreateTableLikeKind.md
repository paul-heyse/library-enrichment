# `sqlparser::ast::CreateTableLikeKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateTableLikeKind.json).

<a id="op-21ab9a58e90cc77c46ad7360"></a>
## CreateTableLikeKind

`enum` · `sqlparser::ast::CreateTableLikeKind` · sqlparser 0.62.0

```rust
enum CreateTableLikeKind
```

Source: `src/ast/mod.rs:11797`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Specifies how to create a new table based on an existing table's schema.
'''sql
CREATE TABLE new LIKE old ...
'''

<a id="op-b8dda8b1950712001872f48f"></a>
## Parenthesized

`variant` · `sqlparser::ast::CreateTableLikeKind::Parenthesized` · sqlparser 0.62.0

```rust
Parenthesized
```

Source: `src/ast/mod.rs:11802`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

'''sql
CREATE TABLE new (LIKE old ...)
'''
[Redshift](https://docs.aws.amazon.com/redshift/latest/dg/r_CREATE_TABLE_NEW.html)

<a id="op-0591d519365b0a045c787a31"></a>
## Plain

`variant` · `sqlparser::ast::CreateTableLikeKind::Plain` · sqlparser 0.62.0

```rust
Plain
```

Source: `src/ast/mod.rs:11808`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

'''sql
CREATE TABLE new LIKE old ...
'''
[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/create-table#label-create-table-like)
[BigQuery](https://cloud.google.com/bigquery/docs/reference/standard-sql/data-definition-language#create_table_like)

<a id="op-22f6c04b396b37889c64cfb6"></a>
## clone

`function` · `sqlparser::ast::CreateTableLikeKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateTableLikeKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeKind", "path": "CreateTableLikeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11794, 17], "end": [11794, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a216eaf9c18173cf2a5c97d9"></a>
## cmp

`function` · `sqlparser::ast::CreateTableLikeKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateTableLikeKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeKind", "path": "CreateTableLikeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11794, 51], "end": [11794, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01befd55a82122859964ec0a"></a>
## deserialize

`function` · `sqlparser::ast::CreateTableLikeKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeKind", "path": "CreateTableLikeKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11795, 49], "end": [11795, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57842add989e75c7a1aa34a1"></a>
## eq

`function` · `sqlparser::ast::CreateTableLikeKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateTableLikeKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeKind", "path": "CreateTableLikeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11794, 24], "end": [11794, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95a4d4a40aae9ef2403ede38"></a>
## fmt

`function` · `sqlparser::ast::CreateTableLikeKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeKind", "path": "CreateTableLikeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11794, 10], "end": [11794, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3b51f093145e4bd3bfdd9e5a"></a>
## hash

`function` · `sqlparser::ast::CreateTableLikeKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeKind", "path": "CreateTableLikeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11794, 56], "end": [11794, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c18f972f36a2b89c8da35083"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateTableLikeKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateTableLikeKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeKind", "path": "CreateTableLikeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11794, 35], "end": [11794, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11794`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b07b27e06e19c9929a31d2b0"></a>
## serialize

`function` · `sqlparser::ast::CreateTableLikeKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeKind", "path": "CreateTableLikeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11795, 38], "end": [11795, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11795`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b6f96eeac48d270c6a0f35b"></a>
## visit

`function` · `sqlparser::ast::CreateTableLikeKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeKind", "path": "CreateTableLikeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11796, 40], "end": [11796, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5c7460157aa35891c7e79ca"></a>
## visit

`function` · `sqlparser::ast::CreateTableLikeKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateTableLikeKind", "path": "CreateTableLikeKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11796, 47], "end": [11796, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11796`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
