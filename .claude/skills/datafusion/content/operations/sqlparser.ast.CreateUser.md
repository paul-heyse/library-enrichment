# `sqlparser::ast::CreateUser`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.CreateUser.json).

<a id="op-1cd527d1a147cefc78515334"></a>
## CreateUser

`struct` · `sqlparser::ast::CreateUser` · sqlparser 0.62.0

```rust
struct CreateUser
```

Source: `src/ast/mod.rs:11472`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Creates a user

Syntax:
```sql
CREATE [OR REPLACE] USER [IF NOT EXISTS] <name> [OPTIONS]
```

[Snowflake](https://docs.snowflake.com/en/sql-reference/sql/create-user)

<a id="op-3fd69f41f6dd35f6474e6c5c"></a>
## clone

`function` · `sqlparser::ast::CreateUser::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreateUser
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11469, 17], "end": [11469, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:11469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a627d70a46a30d890dbc8553"></a>
## cmp

`function` · `sqlparser::ast::CreateUser::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreateUser) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11469, 51], "end": [11469, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:11469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b478ea4c9a089d1735c5e1fb"></a>
## deserialize

`function` · `sqlparser::ast::CreateUser::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [11470, 49], "end": [11470, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:11470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a73b3cb1a1928edd0d124ed2"></a>
## eq

`function` · `sqlparser::ast::CreateUser::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreateUser) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11469, 24], "end": [11469, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:11469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5effb3eed27dd1069919c48d"></a>
## fmt

`function` · `sqlparser::ast::CreateUser::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11487, 1], "end": [11509, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:11488`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c3e2f9fd28c64821045031a"></a>
## fmt

`function` · `sqlparser::ast::CreateUser::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11469, 10], "end": [11469, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:11469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa5ab62e09cf76739b6ecea3"></a>
## hash

`function` · `sqlparser::ast::CreateUser::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11469, 56], "end": [11469, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:11469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-74ca55937bc34076c35a4ebd"></a>
## if_not_exists

`struct_field` · `sqlparser::ast::CreateUser::if_not_exists` · sqlparser 0.62.0

```rust
if_not_exists: bool
```

Source: `src/ast/mod.rs:11476`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Only create the user if it does not already exist.

<a id="op-7bc59f668e6cc44283f5f551"></a>
## name

`struct_field` · `sqlparser::ast::CreateUser::name` · sqlparser 0.62.0

```rust
name: Ident
```

Source: `src/ast/mod.rs:11478`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

The name of the user to create.

<a id="op-1ac6bb8ac1febb3ce27e4467"></a>
## options

`struct_field` · `sqlparser::ast::CreateUser::options` · sqlparser 0.62.0

```rust
options: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/mod.rs:11480`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Key/value options for user creation.

<a id="op-178b47f95004e5a226048628"></a>
## or_replace

`struct_field` · `sqlparser::ast::CreateUser::or_replace` · sqlparser 0.62.0

```rust
or_replace: bool
```

Source: `src/ast/mod.rs:11474`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Replace existing user if present.

<a id="op-971628bd27d002d00d768e75"></a>
## partial_cmp

`function` · `sqlparser::ast::CreateUser::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreateUser) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11469, 35], "end": [11469, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:11469`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6198c89fc61a3b035a1642f7"></a>
## serialize

`function` · `sqlparser::ast::CreateUser::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11470, 38], "end": [11470, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:11470`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4c8d2f215dcad13f725491b"></a>
## tags

`struct_field` · `sqlparser::ast::CreateUser::tags` · sqlparser 0.62.0

```rust
tags: ast::helpers::key_value_options::KeyValueOptions
```

Source: `src/ast/mod.rs:11484`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Tags for the user.

<a id="op-642c4ebfd15c5df9540000ac"></a>
## visit

`function` · `sqlparser::ast::CreateUser::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11471, 40], "end": [11471, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:11471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1f9ae112c4a1ee1e4b0d364"></a>
## visit

`function` · `sqlparser::ast::CreateUser::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::CreateUser", "path": "CreateUser"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [11471, 47], "end": [11471, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:11471`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-520e3fd211a079647be8397c"></a>
## with_tags

`struct_field` · `sqlparser::ast::CreateUser::with_tags` · sqlparser 0.62.0

```rust
with_tags: bool
```

Source: `src/ast/mod.rs:11482`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether tags are specified using `WITH TAG`.
