# `sqlparser::ast::dcl::CreateRole`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.CreateRole.json).

<a id="op-f5bc14e74c9f2df58741ced7"></a>
## CreateRole

`struct` · `sqlparser::ast::dcl::CreateRole` · sqlparser 0.62.0

```rust
struct CreateRole
```

Source: `src/ast/dcl.rs:311`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

CREATE ROLE statement
See [PostgreSQL](https://www.postgresql.org/docs/current/sql-createrole.html)

<a id="op-fcea753fc5c9369cd24c08b9"></a>
## admin

`struct_field` · `sqlparser::ast::dcl::CreateRole::admin` · sqlparser 0.62.0

```rust
admin: Vec<super::Ident>
```

Source: `src/ast/dcl.rs:346`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Admin users listed in `ADMIN` clause.

<a id="op-5879b44ba8fdc4d8dcd1184e"></a>
## authorization_owner

`struct_field` · `sqlparser::ast::dcl::CreateRole::authorization_owner` · sqlparser 0.62.0

```rust
authorization_owner: Option<ast::ObjectName>
```

Source: `src/ast/dcl.rs:349`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional authorization owner.

<a id="op-76e93cf1351ac10b805ecdcd"></a>
## bypassrls

`struct_field` · `sqlparser::ast::dcl::CreateRole::bypassrls` · sqlparser 0.62.0

```rust
bypassrls: Option<bool>
```

Source: `src/ast/dcl.rs:322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `BYPASSRLS` was specified.

<a id="op-ab889d03ff690827208d321b"></a>
## clone

`function` · `sqlparser::ast::dcl::CreateRole::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateRole
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 17], "end": [308, 22], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dcl.rs:308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d5da4bd2136e8a9de799fdf4"></a>
## cmp

`function` · `sqlparser::ast::dcl::CreateRole::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateRole) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 51], "end": [308, 54], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dcl.rs:308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1098ecc606c121c55d9f5114"></a>
## connection_limit

`struct_field` · `sqlparser::ast::dcl::CreateRole::connection_limit` · sqlparser 0.62.0

```rust
connection_limit: Option<super::Expr>
```

Source: `src/ast/dcl.rs:334`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional connection limit expression.

<a id="op-016584b6754a34c301e6990d"></a>
## create_db

`struct_field` · `sqlparser::ast::dcl::CreateRole::create_db` · sqlparser 0.62.0

```rust
create_db: Option<bool>
```

Source: `src/ast/dcl.rs:328`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `CREATEDB` was specified.

<a id="op-1679d594da778eedc1ff93e8"></a>
## create_role

`struct_field` · `sqlparser::ast::dcl::CreateRole::create_role` · sqlparser 0.62.0

```rust
create_role: Option<bool>
```

Source: `src/ast/dcl.rs:330`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `CREATEROLE` was specified.

<a id="op-6275deb1a552945db990cfa6"></a>
## deserialize

`function` · `sqlparser::ast::dcl::CreateRole::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 49], "end": [309, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dcl.rs:309`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c886e57ac01a6eebbbf6b158"></a>
## eq

`function` · `sqlparser::ast::dcl::CreateRole::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateRole) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 24], "end": [308, 33], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dcl.rs:308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a869263935e756e15094062c"></a>
## fmt

`function` · `sqlparser::ast::dcl::CreateRole::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [352, 1], "end": [426, 2], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dcl.rs:353`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfb6830ff481a82e6bb07add"></a>
## fmt

`function` · `sqlparser::ast::dcl::CreateRole::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 10], "end": [308, 15], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dcl.rs:308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28cf3f880f83284a8524b488"></a>
## hash

`function` · `sqlparser::ast::dcl::CreateRole::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 56], "end": [308, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dcl.rs:308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c717935bd61544daaaa751fb"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::dcl::CreateRole::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/dcl.rs:315`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `IF NOT EXISTS` was specified.

<a id="op-b47f8e78cabcb3c1eec51f0a"></a>
## in_group

`struct_field` · `sqlparser::ast::dcl::CreateRole::in_group` · sqlparser 0.62.0

```rust
in_group: Vec<super::Ident>
```

Source: `src/ast/dcl.rs:340`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Members of `IN GROUP` clause.

<a id="op-5dd0cda5158628d922675f97"></a>
## in_role

`struct_field` · `sqlparser::ast::dcl::CreateRole::in_role` · sqlparser 0.62.0

```rust
in_role: Vec<super::Ident>
```

Source: `src/ast/dcl.rs:338`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Members of `IN ROLE` clause.

<a id="op-47c97eda7327730b926d2615"></a>
## inherit

`struct_field` · `sqlparser::ast::dcl::CreateRole::inherit` · sqlparser 0.62.0

```rust
inherit: Option<bool>
```

Source: `src/ast/dcl.rs:320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `INHERIT` was specified.

<a id="op-08a331bd2869750edaecf3e8"></a>
## login

`struct_field` · `sqlparser::ast::dcl::CreateRole::login` · sqlparser 0.62.0

```rust
login: Option<bool>
```

Source: `src/ast/dcl.rs:318`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `LOGIN` was specified.

<a id="op-5e3a953b07157fc4227a10b8"></a>
## names

`struct_field` · `sqlparser::ast::dcl::CreateRole::names` · sqlparser 0.62.0

```rust
names: Vec<ast::ObjectName>
```

Source: `src/ast/dcl.rs:313`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Role names to create.

<a id="op-324baab65c50f0c95d15e37b"></a>
## partial_cmp

`function` · `sqlparser::ast::dcl::CreateRole::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateRole) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [308, 35], "end": [308, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dcl.rs:308`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e9bfb0c6ec08ac2fb3e235a"></a>
## password

`struct_field` · `sqlparser::ast::dcl::CreateRole::password` · sqlparser 0.62.0

```rust
password: Option<super::Password>
```

Source: `src/ast/dcl.rs:324`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional password for the role.

<a id="op-68812b45d021056ab167d3ed"></a>
## replication

`struct_field` · `sqlparser::ast::dcl::CreateRole::replication` · sqlparser 0.62.0

```rust
replication: Option<bool>
```

Source: `src/ast/dcl.rs:332`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `REPLICATION` privilege was specified.

<a id="op-478d759b9ff6b8347e607876"></a>
## role

`struct_field` · `sqlparser::ast::dcl::CreateRole::role` · sqlparser 0.62.0

```rust
role: Vec<super::Ident>
```

Source: `src/ast/dcl.rs:342`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Roles listed in `ROLE` clause.

<a id="op-d2f40b23fc215ca535f03450"></a>
## serialize

`function` · `sqlparser::ast::dcl::CreateRole::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 38], "end": [309, 47], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dcl.rs:309`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6865ddcbcaeaf4041b11f79f"></a>
## span

`function` · `sqlparser::ast::dcl::CreateRole::span` · sqlparser 0.62.0

```rust
fn span(&self) -> Span
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [428, 1], "end": [432, 2], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::spans::Spanned", "path": "Spanned"}, "trait_path": "sqlparser::ast::spans::Spanned"}`

Source: `src/ast/dcl.rs:429`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fa1fb63ab03fe2ac29afd8b"></a>
## superuser

`struct_field` · `sqlparser::ast::dcl::CreateRole::superuser` · sqlparser 0.62.0

```rust
superuser: Option<bool>
```

Source: `src/ast/dcl.rs:326`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether `SUPERUSER` was specified.

<a id="op-772d98422355ddb1408ddd1e"></a>
## user

`struct_field` · `sqlparser::ast::dcl::CreateRole::user` · sqlparser 0.62.0

```rust
user: Vec<super::Ident>
```

Source: `src/ast/dcl.rs:344`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Users listed in `USER` clause.

<a id="op-d7566464df85d003a408ebdd"></a>
## valid_until

`struct_field` · `sqlparser::ast::dcl::CreateRole::valid_until` · sqlparser 0.62.0

```rust
valid_until: Option<super::Expr>
```

Source: `src/ast/dcl.rs:336`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional account validity expression.

<a id="op-338e9cc6fc4e51a2bc134af6"></a>
## visit

`function` · `sqlparser::ast::dcl::CreateRole::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [310, 40], "end": [310, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dcl.rs:310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52dda8371c8711a9455de79a"></a>
## visit

`function` · `sqlparser::ast::dcl::CreateRole::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::CreateRole", "path": "CreateRole"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [310, 47], "end": [310, 55], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dcl.rs:310`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
