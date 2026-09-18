# `sqlparser::ast::ddl::AlterTableType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.AlterTableType.json).

<a id="op-fc4ba20e3f164929c7cd1a2c"></a>
## AlterTableType

`enum` · `sqlparser::ast::ddl::AlterTableType` · sqlparser 0.62.0

```rust
enum AlterTableType
```

Source: `src/ast/ddl.rs:4648`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Table type for ALTER TABLE statements.
Used to distinguish between regular tables, Iceberg tables, and Dynamic tables.

<a id="op-4dca7d1850ade1bbd1cb96bb"></a>
## Dynamic

`variant` · `sqlparser::ast::ddl::AlterTableType::Dynamic` · sqlparser 0.62.0

```rust
Dynamic
```

Source: `src/ast/ddl.rs:4654`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Dynamic table type
<https://docs.snowflake.com/en/sql-reference/sql/alter-dynamic-table>

<a id="op-ce3c906b7a0bac5a367bc868"></a>
## External

`variant` · `sqlparser::ast::ddl::AlterTableType::External` · sqlparser 0.62.0

```rust
External
```

Source: `src/ast/ddl.rs:4657`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

External table type
<https://docs.snowflake.com/en/sql-reference/sql/alter-external-table>

<a id="op-4d22f5bdcf364246fbc12bf1"></a>
## Iceberg

`variant` · `sqlparser::ast::ddl::AlterTableType::Iceberg` · sqlparser 0.62.0

```rust
Iceberg
```

Source: `src/ast/ddl.rs:4651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Iceberg table type
<https://docs.snowflake.com/en/sql-reference/sql/alter-iceberg-table>

<a id="op-49cb452aa81e5b13e5583cc7"></a>
## clone

`function` · `sqlparser::ast::ddl::AlterTableType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterTableType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableType", "path": "AlterTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4645, 17], "end": [4645, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:4645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71e582b6ddf657f13e458427"></a>
## cmp

`function` · `sqlparser::ast::ddl::AlterTableType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterTableType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableType", "path": "AlterTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4645, 51], "end": [4645, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:4645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4104103bae3baf0ec61ee0a"></a>
## deserialize

`function` · `sqlparser::ast::ddl::AlterTableType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableType", "path": "AlterTableType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [4646, 49], "end": [4646, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:4646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5b6e50011603ac757fe170ac"></a>
## eq

`function` · `sqlparser::ast::ddl::AlterTableType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterTableType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableType", "path": "AlterTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4645, 24], "end": [4645, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:4645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ea4759e63a398e4d99363cc"></a>
## fmt

`function` · `sqlparser::ast::ddl::AlterTableType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableType", "path": "AlterTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4645, 10], "end": [4645, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:4645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a220edf584c4054a1e70235"></a>
## hash

`function` · `sqlparser::ast::ddl::AlterTableType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableType", "path": "AlterTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4645, 56], "end": [4645, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:4645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f90b44db1133b5e6d6c18ec3"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::AlterTableType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterTableType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableType", "path": "AlterTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4645, 35], "end": [4645, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:4645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-087f1d744e6251533a6261f4"></a>
## serialize

`function` · `sqlparser::ast::ddl::AlterTableType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableType", "path": "AlterTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4646, 38], "end": [4646, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:4646`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09c9e9076b4894a2663335d7"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTableType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableType", "path": "AlterTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4647, 47], "end": [4647, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:4647`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47687fc6dfa106932aea8351"></a>
## visit

`function` · `sqlparser::ast::ddl::AlterTableType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::AlterTableType", "path": "AlterTableType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [4647, 40], "end": [4647, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:4647`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
