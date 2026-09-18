# `sqlparser::ast::dcl::RoleOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.RoleOption.json).

<a id="op-6e3935dd0141eadc19caf541"></a>
## RoleOption

`enum` · `sqlparser::ast::dcl::RoleOption` · sqlparser 0.62.0

```rust
enum RoleOption
```

Source: `src/ast/dcl.rs:44`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An option in `ROLE` statement.

<https://www.postgresql.org/docs/current/sql-createrole.html>

<a id="op-c194088c9147f2592e5820cd"></a>
## BypassRLS

`variant` · `sqlparser::ast::dcl::RoleOption::BypassRLS` · sqlparser 0.62.0

```rust
BypassRLS
```

Source: `src/ast/dcl.rs:46`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Enable or disable BYPASSRLS.

<a id="op-8d423d72ff110d77f21aa2ed"></a>
## ConnectionLimit

`variant` · `sqlparser::ast::dcl::RoleOption::ConnectionLimit` · sqlparser 0.62.0

```rust
ConnectionLimit
```

Source: `src/ast/dcl.rs:48`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Connection limit expression.

<a id="op-188df75e8e9ae367c50e6a81"></a>
## CreateDB

`variant` · `sqlparser::ast::dcl::RoleOption::CreateDB` · sqlparser 0.62.0

```rust
CreateDB
```

Source: `src/ast/dcl.rs:50`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATEDB flag.

<a id="op-fa4d5aaa5457a70838aee390"></a>
## CreateRole

`variant` · `sqlparser::ast::dcl::RoleOption::CreateRole` · sqlparser 0.62.0

```rust
CreateRole
```

Source: `src/ast/dcl.rs:52`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATEROLE flag.

<a id="op-d4b08bcf47c54b255e0c86a8"></a>
## Inherit

`variant` · `sqlparser::ast::dcl::RoleOption::Inherit` · sqlparser 0.62.0

```rust
Inherit
```

Source: `src/ast/dcl.rs:54`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

INHERIT flag.

<a id="op-0e919245885ed55b7a190f8c"></a>
## Login

`variant` · `sqlparser::ast::dcl::RoleOption::Login` · sqlparser 0.62.0

```rust
Login
```

Source: `src/ast/dcl.rs:56`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

LOGIN flag.

<a id="op-0fb4bfe72becad1b044f1f6e"></a>
## Password

`variant` · `sqlparser::ast::dcl::RoleOption::Password` · sqlparser 0.62.0

```rust
Password
```

Source: `src/ast/dcl.rs:58`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Password value or NULL password.

<a id="op-4e0469fa42446ca3503a21cf"></a>
## Replication

`variant` · `sqlparser::ast::dcl::RoleOption::Replication` · sqlparser 0.62.0

```rust
Replication
```

Source: `src/ast/dcl.rs:60`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Replication privilege flag.

<a id="op-704cae1db289cf4d7e5fbf65"></a>
## SuperUser

`variant` · `sqlparser::ast::dcl::RoleOption::SuperUser` · sqlparser 0.62.0

```rust
SuperUser
```

Source: `src/ast/dcl.rs:62`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

SUPERUSER flag.

<a id="op-0745da7d527196c6e3dc2c58"></a>
## ValidUntil

`variant` · `sqlparser::ast::dcl::RoleOption::ValidUntil` · sqlparser 0.62.0

```rust
ValidUntil
```

Source: `src/ast/dcl.rs:64`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

`VALID UNTIL` expression.

<a id="op-73de34861506e2e65c81e3b4"></a>
## clone

`function` · `sqlparser::ast::dcl::RoleOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> RoleOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 17], "end": [41, 22], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dcl.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-feac370e9a3cd5f0cb3e0664"></a>
## cmp

`function` · `sqlparser::ast::dcl::RoleOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &RoleOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 51], "end": [41, 54], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dcl.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8e0b37e0f63bb9c441dd853"></a>
## deserialize

`function` · `sqlparser::ast::dcl::RoleOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 49], "end": [42, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dcl.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70e07c87bb7f9d1a17784632"></a>
## eq

`function` · `sqlparser::ast::dcl::RoleOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &RoleOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 24], "end": [41, 33], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dcl.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fe395f56d2d266e186513df"></a>
## fmt

`function` · `sqlparser::ast::dcl::RoleOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [111, 2], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dcl.rs:68`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef5408cb02457f591900bba3"></a>
## fmt

`function` · `sqlparser::ast::dcl::RoleOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 10], "end": [41, 15], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dcl.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62e6baf15ef6fcaa7b3d44bb"></a>
## hash

`function` · `sqlparser::ast::dcl::RoleOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 56], "end": [41, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dcl.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5eef7ae4bfebd377acfd2f92"></a>
## partial_cmp

`function` · `sqlparser::ast::dcl::RoleOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &RoleOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [41, 35], "end": [41, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dcl.rs:41`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-991a9e3498627aaede03a10d"></a>
## serialize

`function` · `sqlparser::ast::dcl::RoleOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 38], "end": [42, 47], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dcl.rs:42`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19ae769aa7cc7fbbd1c21f75"></a>
## visit

`function` · `sqlparser::ast::dcl::RoleOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 47], "end": [43, 55], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dcl.rs:43`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf7d955a250d9e22b69d4eaa"></a>
## visit

`function` · `sqlparser::ast::dcl::RoleOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::RoleOption", "path": "RoleOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [43, 40], "end": [43, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dcl.rs:43`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
