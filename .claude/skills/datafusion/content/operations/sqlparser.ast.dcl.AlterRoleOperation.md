# `sqlparser::ast::dcl::AlterRoleOperation`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.dcl.AlterRoleOperation.json).

<a id="op-3cab83d008e6120b50701fee"></a>
## AlterRoleOperation

`enum` · `sqlparser::ast::dcl::AlterRoleOperation` · sqlparser 0.62.0

```rust
enum AlterRoleOperation
```

Source: `src/ast/dcl.rs:145`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An `ALTER ROLE` (`Statement::AlterRole`) operation

<a id="op-3481e6ad2ba4375455d4df75"></a>
## AddMember

`variant` · `sqlparser::ast::dcl::AlterRoleOperation::AddMember` · sqlparser 0.62.0

```rust
AddMember
```

Source: `src/ast/dcl.rs:153`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MS SQL Server
<https://learn.microsoft.com/en-us/sql/t-sql/statements/alter-role-transact-sql>

<a id="op-98996e227a1401006e994285"></a>
## DropMember

`variant` · `sqlparser::ast::dcl::AlterRoleOperation::DropMember` · sqlparser 0.62.0

```rust
DropMember
```

Source: `src/ast/dcl.rs:160`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

MS SQL Server

<https://learn.microsoft.com/en-us/sql/t-sql/statements/alter-role-transact-sql>

<a id="op-687faeec72484e64166d2569"></a>
## RenameRole

`variant` · `sqlparser::ast::dcl::AlterRoleOperation::RenameRole` · sqlparser 0.62.0

```rust
RenameRole
```

Source: `src/ast/dcl.rs:147`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Generic

<a id="op-f1cbe3e9181b2853da706d06"></a>
## Reset

`variant` · `sqlparser::ast::dcl::AlterRoleOperation::Reset` · sqlparser 0.62.0

```rust
Reset
```

Source: `src/ast/dcl.rs:186`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL
<https://www.postgresql.org/docs/current/sql-alterrole.html>

`RESET configuration_parameter` | `RESET ALL`

<a id="op-bd0667a025e48cf4a1364b5c"></a>
## Set

`variant` · `sqlparser::ast::dcl::AlterRoleOperation::Set` · sqlparser 0.62.0

```rust
Set
```

Source: `src/ast/dcl.rs:174`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL
<https://www.postgresql.org/docs/current/sql-alterrole.html>

`SET configuration_parameter { TO | = } { value | DEFAULT }`

<a id="op-86a21b789906b20f26309ed1"></a>
## WithOptions

`variant` · `sqlparser::ast::dcl::AlterRoleOperation::WithOptions` · sqlparser 0.62.0

```rust
WithOptions
```

Source: `src/ast/dcl.rs:166`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

PostgreSQL
<https://www.postgresql.org/docs/current/sql-alterrole.html>

<a id="op-af84146001ba4e71a644d2a2"></a>
## clone

`function` · `sqlparser::ast::dcl::AlterRoleOperation::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> AlterRoleOperation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 17], "end": [142, 22], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/dcl.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-654452ce5f953b853a8f282d"></a>
## cmp

`function` · `sqlparser::ast::dcl::AlterRoleOperation::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &AlterRoleOperation) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 51], "end": [142, 54], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/dcl.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f145f80ee7501d884892c5f"></a>
## deserialize

`function` · `sqlparser::ast::dcl::AlterRoleOperation::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 49], "end": [143, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/dcl.rs:143`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d3e7c53fcd5545c69471444"></a>
## eq

`function` · `sqlparser::ast::dcl::AlterRoleOperation::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &AlterRoleOperation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 24], "end": [142, 33], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/dcl.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1813098232d63f5c35fee733"></a>
## fmt

`function` · `sqlparser::ast::dcl::AlterRoleOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 10], "end": [142, 15], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/dcl.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f02f76b7d3bd072827110ba5"></a>
## fmt

`function` · `sqlparser::ast::dcl::AlterRoleOperation::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [194, 1], "end": [239, 2], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/dcl.rs:195`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bdf303b61167fdcc71ca9790"></a>
## hash

`function` · `sqlparser::ast::dcl::AlterRoleOperation::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 56], "end": [142, 60], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/dcl.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4206b1a39d4fb5ad231b93e7"></a>
## partial_cmp

`function` · `sqlparser::ast::dcl::AlterRoleOperation::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &AlterRoleOperation) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [142, 35], "end": [142, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/dcl.rs:142`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad3debf690532221b9e5babf"></a>
## serialize

`function` · `sqlparser::ast::dcl::AlterRoleOperation::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [143, 38], "end": [143, 47], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/dcl.rs:143`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64df6eafa33b01d2b7f8478d"></a>
## visit

`function` · `sqlparser::ast::dcl::AlterRoleOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 47], "end": [144, 55], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/dcl.rs:144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-726c77dfc1e7f7e3c2769ec6"></a>
## visit

`function` · `sqlparser::ast::dcl::AlterRoleOperation::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::dcl::AlterRoleOperation", "path": "AlterRoleOperation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [144, 40], "end": [144, 45], "filename": "src/ast/dcl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/dcl.rs:144`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
