# `sqlparser::ast::DeclareType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.DeclareType.json).

<a id="op-c5ea942c2efa9b894e4a0f1b"></a>
## DeclareType

`enum` · `sqlparser::ast::DeclareType` · sqlparser 0.62.0

```rust
enum DeclareType
```

Source: `src/ast/mod.rs:3024`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Represents the type of a `DECLARE` statement.

<a id="op-a4f5b3badb5d805099cfe1b7"></a>
## Cursor

`variant` · `sqlparser::ast::DeclareType::Cursor` · sqlparser 0.62.0

```rust
Cursor
```

Source: `src/ast/mod.rs:3030`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Cursor variable type. e.g. [Snowflake] [PostgreSQL] [MsSql]

[Snowflake]: https://docs.snowflake.com/en/developer-guide/snowflake-scripting/cursors#declaring-a-cursor
[PostgreSQL]: https://www.postgresql.org/docs/current/plpgsql-cursors.html
[MsSql]: https://learn.microsoft.com/en-us/sql/t-sql/language-elements/declare-cursor-transact-sql

<a id="op-3018458cc90496f90207fc44"></a>
## Exception

`variant` · `sqlparser::ast::DeclareType::Exception` · sqlparser 0.62.0

```rust
Exception
```

Source: `src/ast/mod.rs:3048`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Exception declaration syntax. [Snowflake]

Syntax:
```text
<exception_name> EXCEPTION [ ( <exception_number> , '<exception_message>' ) ] ;
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/snowflake-scripting/declare#exception-declaration-syntax

<a id="op-cc612c66a2422eaffc01afd1"></a>
## ResultSet

`variant` · `sqlparser::ast::DeclareType::ResultSet` · sqlparser 0.62.0

```rust
ResultSet
```

Source: `src/ast/mod.rs:3039`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Result set variable type. [Snowflake]

Syntax:
```text
<resultset_name> RESULTSET [ { DEFAULT | := } ( <query> ) ] ;
```
[Snowflake]: https://docs.snowflake.com/en/sql-reference/snowflake-scripting/declare#resultset-declaration-syntax

<a id="op-66d6de685bdd440b35116772"></a>
## clone

`function` · `sqlparser::ast::DeclareType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> DeclareType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3021, 17], "end": [3021, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:3021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5712309528046090f92c0353"></a>
## cmp

`function` · `sqlparser::ast::DeclareType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &DeclareType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3021, 51], "end": [3021, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:3021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-668c64c87806493b744cdc03"></a>
## deserialize

`function` · `sqlparser::ast::DeclareType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3022, 49], "end": [3022, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:3022`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fa8692603547707c959e362"></a>
## eq

`function` · `sqlparser::ast::DeclareType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &DeclareType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3021, 24], "end": [3021, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:3021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-38ad3b52e70c9a3add9c3298"></a>
## fmt

`function` · `sqlparser::ast::DeclareType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3021, 10], "end": [3021, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:3021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b933552ce6e61380006c1102"></a>
## fmt

`function` · `sqlparser::ast::DeclareType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3051, 1], "end": [3065, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:3052`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c2573b276debbc4efb9464d9"></a>
## hash

`function` · `sqlparser::ast::DeclareType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3021, 56], "end": [3021, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:3021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c7b524fd2f172520dac9840"></a>
## partial_cmp

`function` · `sqlparser::ast::DeclareType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &DeclareType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3021, 35], "end": [3021, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:3021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e55745e928bdbd0a9a705735"></a>
## serialize

`function` · `sqlparser::ast::DeclareType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3022, 38], "end": [3022, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:3022`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc93899754c35ad3395f8bbb"></a>
## visit

`function` · `sqlparser::ast::DeclareType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3023, 47], "end": [3023, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:3023`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcc01890cfa2653129afecb5"></a>
## visit

`function` · `sqlparser::ast::DeclareType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::DeclareType", "path": "DeclareType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3023, 40], "end": [3023, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:3023`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
