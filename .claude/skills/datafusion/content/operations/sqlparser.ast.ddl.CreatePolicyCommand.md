# `sqlparser::ast::ddl::CreatePolicyCommand`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.CreatePolicyCommand.json).

<a id="op-8bd140f5669722dd8a16c31d"></a>
## CreatePolicyCommand

`enum` · `sqlparser::ast::ddl::CreatePolicyCommand` · sqlparser 0.62.0

```rust
enum CreatePolicyCommand
```

Source: `src/ast/ddl.rs:5653`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Command that a policy can apply to (FOR clause).
```sql
FOR [ALL | SELECT | INSERT | UPDATE | DELETE]
```
[PostgreSQL](https://www.postgresql.org/docs/current/sql-createpolicy.html)

<a id="op-33fbee978c55a189bad76fe0"></a>
## All

`variant` · `sqlparser::ast::ddl::CreatePolicyCommand::All` · sqlparser 0.62.0

```rust
All
```

Source: `src/ast/ddl.rs:5655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Applies to all commands.

<a id="op-8f89fc288e461a2362eadb56"></a>
## Delete

`variant` · `sqlparser::ast::ddl::CreatePolicyCommand::Delete` · sqlparser 0.62.0

```rust
Delete
```

Source: `src/ast/ddl.rs:5663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Applies to DELETE.

<a id="op-ae5049c8a068bc17e9c95bb0"></a>
## Insert

`variant` · `sqlparser::ast::ddl::CreatePolicyCommand::Insert` · sqlparser 0.62.0

```rust
Insert
```

Source: `src/ast/ddl.rs:5659`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Applies to INSERT.

<a id="op-007a5911693dd66335a4916e"></a>
## Select

`variant` · `sqlparser::ast::ddl::CreatePolicyCommand::Select` · sqlparser 0.62.0

```rust
Select
```

Source: `src/ast/ddl.rs:5657`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Applies to SELECT.

<a id="op-60949471878a57d3ade188af"></a>
## Update

`variant` · `sqlparser::ast::ddl::CreatePolicyCommand::Update` · sqlparser 0.62.0

```rust
Update
```

Source: `src/ast/ddl.rs:5661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Applies to UPDATE.

<a id="op-2160d42ebdee9d403c05af36"></a>
## clone

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> CreatePolicyCommand
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5650, 17], "end": [5650, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:5650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fa9e092d2be837346113909"></a>
## cmp

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &CreatePolicyCommand) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5650, 57], "end": [5650, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:5650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3ce6b73a48c8fa7c64bddc73"></a>
## deserialize

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [5651, 49], "end": [5651, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:5651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f44f410844a11df1616bc650"></a>
## eq

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &CreatePolicyCommand) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5650, 30], "end": [5650, 39], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:5650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fa03e2e1bd5448455e10f22"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5650, 10], "end": [5650, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:5650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-839f7f3023f7bb836e18503c"></a>
## fmt

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5666, 1], "end": [5676, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:5667`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-136895d38a59a73e0a99ce3f"></a>
## hash

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5650, 62], "end": [5650, 66], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:5650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-321f9f2884df0785d205386f"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &CreatePolicyCommand) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5650, 41], "end": [5650, 51], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:5650`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdaac1a92d8612f9c96473ba"></a>
## serialize

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5651, 38], "end": [5651, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:5651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12420a31e51c6219d9dad20f"></a>
## visit

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5652, 40], "end": [5652, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:5652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-edf1b7a72d9819d9cc6ee145"></a>
## visit

`function` · `sqlparser::ast::ddl::CreatePolicyCommand::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::CreatePolicyCommand", "path": "CreatePolicyCommand"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [5652, 47], "end": [5652, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:5652`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
