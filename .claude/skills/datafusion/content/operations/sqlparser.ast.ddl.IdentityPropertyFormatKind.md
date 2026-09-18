# `sqlparser::ast::ddl::IdentityPropertyFormatKind`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.IdentityPropertyFormatKind.json).

<a id="op-8e5699b01df35c603f2d6ab1"></a>
## IdentityPropertyFormatKind

`enum` · `sqlparser::ast::ddl::IdentityPropertyFormatKind` · sqlparser 0.62.0

```rust
enum IdentityPropertyFormatKind
```

Source: `src/ast/ddl.rs:1736`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A format of parameters of identity column.

It is [Snowflake] specific.
Syntax
```sql
(seed , increment) | START num INCREMENT num
```
[MS SQL Server] uses one way of representing these parameters.
Syntax
```sql
(seed , increment)
```
[MS SQL Server]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-table-transact-sql-identity-property
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-78faedea82eb40ef164eeb80"></a>
## FunctionCall

`variant` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::FunctionCall` · sqlparser 0.62.0

```rust
FunctionCall
```

Source: `src/ast/ddl.rs:1744`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A parameters of identity column declared like parameters of function call
Example:
```sql
 (100, 1)
```
[MS SQL Server]: https://learn.microsoft.com/en-us/sql/t-sql/statements/create-table-transact-sql-identity-property
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-472c9b04d4554307d9ca9763"></a>
## StartAndIncrement

`variant` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::StartAndIncrement` · sqlparser 0.62.0

```rust
StartAndIncrement
```

Source: `src/ast/ddl.rs:1751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A parameters of identity column declared with keywords `START` and `INCREMENT`
Example:
```sql
 START 100 INCREMENT 1
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/sql/create-table

<a id="op-be752639319c98662ea7da85"></a>
## clone

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> IdentityPropertyFormatKind
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1733, 17], "end": [1733, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:1733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e0c8229ac6ca13d69bfc45b8"></a>
## cmp

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &IdentityPropertyFormatKind) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1733, 51], "end": [1733, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:1733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-422d43437342174e98209c45"></a>
## deserialize

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [1734, 49], "end": [1734, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:1734`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4848475c741ec371cc85b33e"></a>
## eq

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &IdentityPropertyFormatKind) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1733, 24], "end": [1733, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:1733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-839245ff2e8658a5eb938227"></a>
## fmt

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1733, 10], "end": [1733, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:1733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1a86654eeff3c12e7de8274"></a>
## fmt

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1754, 1], "end": [1769, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:1755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2f92b80866a4ec53cf1c991"></a>
## hash

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1733, 56], "end": [1733, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:1733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfb52c7f10ea8ddc74773212"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &IdentityPropertyFormatKind) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1733, 35], "end": [1733, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:1733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa0bb1636b067b37864bcb1d"></a>
## serialize

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1734, 38], "end": [1734, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:1734`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f0462ab615eba388cec1cc3"></a>
## visit

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1735, 40], "end": [1735, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:1735`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-abcc84d41d062b7f017c31e6"></a>
## visit

`function` · `sqlparser::ast::ddl::IdentityPropertyFormatKind::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::IdentityPropertyFormatKind", "path": "IdentityPropertyFormatKind"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1735, 47], "end": [1735, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:1735`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
