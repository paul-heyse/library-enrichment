# `sqlparser::ast::ddl::IdentityPropertyKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.IdentityPropertyKind.json).

<a id="op-efe5d0b39af2e75e51da6487"></a>
## IdentityPropertyKind

`enum` · `sqlparser::ast::ddl::IdentityPropertyKind` · sqlparser 0.62.0

```rust
enum IdentityPropertyKind
```

Source: `src/ast/ddl.rs:1667`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Identity is a column option for defining an identity or autoincrement column in a `CREATE TABLE` statement.
Syntax
```sql
{ IDENTITY | AUTOINCREMENT } [ (seed , increment) | START num INCREMENT num ] [ ORDER | NOORDER ]
```
[MS SQL Server]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-table-transact-sql-identity-property
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-da845dc407c92e9a45e04d59"></a>
## Autoincrement

`variant` · `sqlparser::ast::ddl::IdentityPropertyKind::Autoincrement` · sqlparser 0.62.0

```rust
Autoincrement
```

Source: `src/ast/ddl.rs:1675`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An identity property declared via the `AUTOINCREMENT` key word
Example:
```sql
 AUTOINCREMENT(100, 1) NOORDER
 AUTOINCREMENT START 100 INCREMENT 1 ORDER
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-533502e9181d125fb882abfc"></a>
## Identity

`variant` · `sqlparser::ast::ddl::IdentityPropertyKind::Identity` · sqlparser 0.62.0

```rust
Identity
```

Source: `src/ast/ddl.rs:1688`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An identity property declared via the `IDENTITY` key word
Example, [MS SQL Server] or [Snowflake]:
```sql
 IDENTITY(100, 1)
```
[Snowflake]
```sql
 IDENTITY(100, 1) ORDER
 IDENTITY START 100 INCREMENT 1 NOORDER
```
[MS SQL Server]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-table-transact-sql-identity-property
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-82be81e88aceb21e3d79aae3"></a>
## clone

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IdentityPropertyKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1664, 17], "end": [1664, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f6d8ecb111a685130a90679"></a>
## cmp

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IdentityPropertyKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1664, 51], "end": [1664, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2013aefcb21349bf41f04d10"></a>
## deserialize

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1665, 49], "end": [1665, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1665`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e22630b6b55cd7d26c66003"></a>
## eq

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IdentityPropertyKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1664, 24], "end": [1664, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74494169aefa90e434b20f98"></a>
## fmt

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1664, 10], "end": [1664, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-92d5c19165b7770735a7f959"></a>
## fmt

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1691, 1], "end": [1706, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1692`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b9d6d3724d3a89add6b85f5"></a>
## hash

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1664, 56], "end": [1664, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5ba43ca71a3ee3c8153fd282"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IdentityPropertyKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1664, 35], "end": [1664, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1664`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-55c9e6dd02c43aca618914e0"></a>
## serialize

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1665, 38], "end": [1665, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1665`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79e9204802d43a7b232c96d0"></a>
## visit

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1666, 47], "end": [1666, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1666`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7f12579dbc32129fd025a54e"></a>
## visit

`function` · `sqlparser::ast::ddl::IdentityPropertyKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyKind", "path": "IdentityPropertyKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1666, 40], "end": [1666, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1666`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
