# `sqlparser::ast::ActionCreateObjectType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ActionCreateObjectType.json).

<a id="op-c74cb288b98077550bef3bd9"></a>
## ActionCreateObjectType

`enum` · `sqlparser::ast::ActionCreateObjectType` · sqlparser 0.62.0

```rust
enum ActionCreateObjectType
```

Source: `src/ast/mod.rs:7184`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/sql/grant-privilege>
under `globalPrivileges` in the `CREATE` privilege.

<a id="op-90249a6692a9fe65408bd37e"></a>
## Account

`variant` · `sqlparser::ast::ActionCreateObjectType::Account` · sqlparser 0.62.0

```rust
Account
```

Source: `src/ast/mod.rs:7186`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An account-level object.

<a id="op-6cd23b8410d606bf8c2f55f7"></a>
## Application

`variant` · `sqlparser::ast::ActionCreateObjectType::Application` · sqlparser 0.62.0

```rust
Application
```

Source: `src/ast/mod.rs:7188`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An application object.

<a id="op-e314c800bc833bb739fe6ba6"></a>
## ApplicationPackage

`variant` · `sqlparser::ast::ActionCreateObjectType::ApplicationPackage` · sqlparser 0.62.0

```rust
ApplicationPackage
```

Source: `src/ast/mod.rs:7190`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An application package object.

<a id="op-2dd21d0c9c3f527a3aa2ce53"></a>
## ComputePool

`variant` · `sqlparser::ast::ActionCreateObjectType::ComputePool` · sqlparser 0.62.0

```rust
ComputePool
```

Source: `src/ast/mod.rs:7192`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A compute pool object.

<a id="op-738a59aac88d24f26981813a"></a>
## DataExchangeListing

`variant` · `sqlparser::ast::ActionCreateObjectType::DataExchangeListing` · sqlparser 0.62.0

```rust
DataExchangeListing
```

Source: `src/ast/mod.rs:7194`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A data exchange listing.

<a id="op-d73e16590abddb686458c5a8"></a>
## Database

`variant` · `sqlparser::ast::ActionCreateObjectType::Database` · sqlparser 0.62.0

```rust
Database
```

Source: `src/ast/mod.rs:7196`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A database object.

<a id="op-b5e2343ee8fdb7e3feaceff1"></a>
## ExternalVolume

`variant` · `sqlparser::ast::ActionCreateObjectType::ExternalVolume` · sqlparser 0.62.0

```rust
ExternalVolume
```

Source: `src/ast/mod.rs:7198`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An external volume object.

<a id="op-f9d5d79dc64e879466e571b1"></a>
## FailoverGroup

`variant` · `sqlparser::ast::ActionCreateObjectType::FailoverGroup` · sqlparser 0.62.0

```rust
FailoverGroup
```

Source: `src/ast/mod.rs:7200`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A failover group object.

<a id="op-046cd43805019bcb0ee65b7a"></a>
## Integration

`variant` · `sqlparser::ast::ActionCreateObjectType::Integration` · sqlparser 0.62.0

```rust
Integration
```

Source: `src/ast/mod.rs:7202`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An integration object.

<a id="op-d1adbe325eebb748338534ee"></a>
## NetworkPolicy

`variant` · `sqlparser::ast::ActionCreateObjectType::NetworkPolicy` · sqlparser 0.62.0

```rust
NetworkPolicy
```

Source: `src/ast/mod.rs:7204`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A network policy object.

<a id="op-acaba0bc316d5a3dcf8dc625"></a>
## OrganiationListing

`variant` · `sqlparser::ast::ActionCreateObjectType::OrganiationListing` · sqlparser 0.62.0

```rust
OrganiationListing
```

Source: `src/ast/mod.rs:7206`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

An organization listing.

<a id="op-418d1e9da4d9766bd0ba7a2d"></a>
## ReplicationGroup

`variant` · `sqlparser::ast::ActionCreateObjectType::ReplicationGroup` · sqlparser 0.62.0

```rust
ReplicationGroup
```

Source: `src/ast/mod.rs:7208`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A replication group object.

<a id="op-29badcb42f577ebbfd4620df"></a>
## Role

`variant` · `sqlparser::ast::ActionCreateObjectType::Role` · sqlparser 0.62.0

```rust
Role
```

Source: `src/ast/mod.rs:7210`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A role object.

<a id="op-7526f63f10643ea46869e393"></a>
## Schema

`variant` · `sqlparser::ast::ActionCreateObjectType::Schema` · sqlparser 0.62.0

```rust
Schema
```

Source: `src/ast/mod.rs:7212`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A schema object.

<a id="op-0684f17e6bc97916d9193e11"></a>
## Share

`variant` · `sqlparser::ast::ActionCreateObjectType::Share` · sqlparser 0.62.0

```rust
Share
```

Source: `src/ast/mod.rs:7214`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A share object.

<a id="op-efcb3faced5fe3dfb30367be"></a>
## User

`variant` · `sqlparser::ast::ActionCreateObjectType::User` · sqlparser 0.62.0

```rust
User
```

Source: `src/ast/mod.rs:7216`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A user object.

<a id="op-8e4e8c92e1475d5977a8fc40"></a>
## Warehouse

`variant` · `sqlparser::ast::ActionCreateObjectType::Warehouse` · sqlparser 0.62.0

```rust
Warehouse
```

Source: `src/ast/mod.rs:7218`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A warehouse object.

<a id="op-452f1531d8e396f3a15413e5"></a>
## clone

`function` · `sqlparser::ast::ActionCreateObjectType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ActionCreateObjectType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7179, 17], "end": [7179, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dca177cea99972fbc709c6a6"></a>
## cmp

`function` · `sqlparser::ast::ActionCreateObjectType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ActionCreateObjectType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7179, 51], "end": [7179, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e251dcbff2df7b5117ca533f"></a>
## deserialize

`function` · `sqlparser::ast::ActionCreateObjectType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7180, 49], "end": [7180, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33366a347ff2215c5097401a"></a>
## eq

`function` · `sqlparser::ast::ActionCreateObjectType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ActionCreateObjectType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7179, 24], "end": [7179, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7484b246d5f9d160558874d3"></a>
## fmt

`function` · `sqlparser::ast::ActionCreateObjectType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7221, 1], "end": [7243, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7222`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95237d0b9603f092ef119665"></a>
## fmt

`function` · `sqlparser::ast::ActionCreateObjectType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7179, 10], "end": [7179, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4947ff1fd21f7f9f3584c1f"></a>
## hash

`function` · `sqlparser::ast::ActionCreateObjectType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7179, 56], "end": [7179, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0cfd23e4c51a01e55533dbb9"></a>
## partial_cmp

`function` · `sqlparser::ast::ActionCreateObjectType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ActionCreateObjectType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7179, 35], "end": [7179, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7179`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22670ae5711a37481c35fda8"></a>
## serialize

`function` · `sqlparser::ast::ActionCreateObjectType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7180, 38], "end": [7180, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7180`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a38305fe47867e247d90943"></a>
## visit

`function` · `sqlparser::ast::ActionCreateObjectType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7181, 40], "end": [7181, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d68df2b55b5e08629789746c"></a>
## visit

`function` · `sqlparser::ast::ActionCreateObjectType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionCreateObjectType", "path": "ActionCreateObjectType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7181, 47], "end": [7181, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7181`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
