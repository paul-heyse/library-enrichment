# `sqlparser::ast::Set`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Set.json).

<a id="op-9d9e31f997a3b9ffa297ad38"></a>
## Set

`enum` · `sqlparser::ast::Set` · sqlparser 0.62.0

```rust
enum Set
```

Source: `src/ast/mod.rs:3245`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Variants for the `SET` family of statements.

<a id="op-ac4496be2edc25e20d68cc1e"></a>
## MultipleAssignments

`variant` · `sqlparser::ast::Set::MultipleAssignments` · sqlparser 0.62.0

```rust
MultipleAssignments
```

Source: `src/ast/mod.rs:3271`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MySQL-style
SET a = 1, b = 2, ..;
`SET a = 1, b = 2` (MySQL-style comma-separated assignments).

<a id="op-4cdaf326e923db5ca13ac58e"></a>
## ParenthesizedAssignments

`variant` · `sqlparser::ast::Set::ParenthesizedAssignments` · sqlparser 0.62.0

```rust
ParenthesizedAssignments
```

Source: `src/ast/mod.rs:3262`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Snowflake-style
SET (a, b, ..) = (1, 2, ..);
`SET (a, b) = (1, 2)` (tuple assignment syntax).

<a id="op-3cdd2ec9882b85f4d81111b5"></a>
## SetNames

`variant` · `sqlparser::ast::Set::SetNames` · sqlparser 0.62.0

```rust
SetNames
```

Source: `src/ast/mod.rs:3322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SET NAMES 'charset_name' [COLLATE 'collation_name']
```

<a id="op-b99338effd7d5a0ddae50659"></a>
## SetNamesDefault

`variant` · `sqlparser::ast::Set::SetNamesDefault` · sqlparser 0.62.0

```rust
SetNamesDefault
```

Source: `src/ast/mod.rs:3333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SET NAMES DEFAULT
```

Note: this is a MySQL-specific statement.

<a id="op-3338462a25c01003f5eff33a"></a>
## SetRole

`variant` · `sqlparser::ast::Set::SetRole` · sqlparser 0.62.0

```rust
SetRole
```

Source: `src/ast/mod.rs:3298`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SET [ SESSION | LOCAL ] ROLE role_name
```

Sets session state. Examples: [ANSI][1], [Postgresql][2], [MySQL][3], and [Oracle][4]

[1]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#set-role-statement
[2]: https://www.postgresql.org/docs/14/sql-set-role.html
[3]: https://dev.mysql.com/doc/refman/8.0/en/set-role.html
[4]: https://docs.oracle.com/cd/B19306_01/server.102/b14200/statements_10004.htm

<a id="op-c6484fc5cbff8326f9149732"></a>
## SetSessionAuthorization

`variant` · `sqlparser::ast::Set::SetSessionAuthorization` · sqlparser 0.62.0

```rust
SetSessionAuthorization
```

Source: `src/ast/mod.rs:3283`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Session authorization for Postgres/Redshift

```sql
SET SESSION AUTHORIZATION { user_name | DEFAULT }
```

See <https://www.postgresql.org/docs/current/sql-set-session-authorization.html>
See <https://docs.aws.amazon.com/redshift/latest/dg/r_SET_SESSION_AUTHORIZATION.html>

<a id="op-dd521153db36756814a4266d"></a>
## SetSessionParam

`variant` · `sqlparser::ast::Set::SetSessionParam` · sqlparser 0.62.0

```rust
SetSessionParam
```

Source: `src/ast/mod.rs:3287`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MS-SQL session

See <https://learn.microsoft.com/en-us/sql/t-sql/statements/set-statements-transact-sql>

<a id="op-d57d9be5cec73166d1f3269f"></a>
## SetTimeZone

`variant` · `sqlparser::ast::Set::SetTimeZone` · sqlparser 0.62.0

```rust
SetTimeZone
```

Source: `src/ast/mod.rs:3313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SET TIME ZONE <value>
```

Note: this is a PostgreSQL-specific statements
`SET TIME ZONE <value>` is an alias for `SET timezone TO <value>` in PostgreSQL
However, we allow it for all dialects.
`SET TIME ZONE` statement. `local` indicates the `LOCAL` keyword.
`SET TIME ZONE <value>` statement.

<a id="op-4297ffd6b71ff4394055853e"></a>
## SetTransaction

`variant` · `sqlparser::ast::Set::SetTransaction` · sqlparser 0.62.0

```rust
SetTransaction
```

Source: `src/ast/mod.rs:3337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

```sql
SET TRANSACTION ...
```

<a id="op-b7bc6543020300589dbaed68"></a>
## SingleAssignment

`variant` · `sqlparser::ast::Set::SingleAssignment` · sqlparser 0.62.0

```rust
SingleAssignment
```

Source: `src/ast/mod.rs:3249`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SQL Standard-style
SET a = 1;
`SET var = value` (standard SQL-style assignment).

<a id="op-593c03b39de4086fb560bb1e"></a>
## clone

`function` · `sqlparser::ast::Set::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Set
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3241, 17], "end": [3241, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:3241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d304273a3e1699c80185d5e"></a>
## cmp

`function` · `sqlparser::ast::Set::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Set) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3241, 51], "end": [3241, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:3241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f21d2959622c7814e94aeb60"></a>
## deserialize

`function` · `sqlparser::ast::Set::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [3242, 49], "end": [3242, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:3242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec9e8cc3f52de25704b3c2c5"></a>
## eq

`function` · `sqlparser::ast::Set::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Set) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3241, 24], "end": [3241, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:3241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-544da29ab84416959a4d1492"></a>
## fmt

`function` · `sqlparser::ast::Set::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3347, 1], "end": [3432, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:3348`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a54cd85e84fa158e64c80f87"></a>
## fmt

`function` · `sqlparser::ast::Set::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3241, 10], "end": [3241, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:3241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44329b1921b556d317f9e35d"></a>
## hash

`function` · `sqlparser::ast::Set::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3241, 56], "end": [3241, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:3241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14e4b4d1cfdd1dde376bef90"></a>
## partial_cmp

`function` · `sqlparser::ast::Set::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Set) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3241, 35], "end": [3241, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:3241`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-caca2ab7f3bb3bef632b4f22"></a>
## serialize

`function` · `sqlparser::ast::Set::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3242, 38], "end": [3242, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:3242`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6acc3db4db102e33e8f1f839"></a>
## visit

`function` · `sqlparser::ast::Set::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3243, 40], "end": [3243, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:3243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c20a61e18ca26360d58c27a2"></a>
## visit

`function` · `sqlparser::ast::Set::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Set", "path": "Set"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3243, 47], "end": [3243, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:3243`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
