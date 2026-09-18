# `sqlparser::ast::Action`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.Action.json).

<a id="op-3fab8fc8bb30219e337e32d4"></a>
## Action

`enum` · `sqlparser::ast::Action` · sqlparser 0.62.0

```rust
enum Action
```

Source: `src/ast/mod.rs:6972`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

A privilege on a database object (table, sequence, etc.).

<a id="op-eecf2d180e215a001c3563c1"></a>
## AddSearchOptimization

`variant` · `sqlparser::ast::Action::AddSearchOptimization` · sqlparser 0.62.0

```rust
AddSearchOptimization
```

Source: `src/ast/mod.rs:6974`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Add a search optimization.

<a id="op-3cae3bf67d1b02e06c71a666"></a>
## Apply

`variant` · `sqlparser::ast::Action::Apply` · sqlparser 0.62.0

```rust
Apply
```

Source: `src/ast/mod.rs:6976`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply an `APPLY` operation with a specific type.

<a id="op-8821add3a207449627d15c8e"></a>
## ApplyBudget

`variant` · `sqlparser::ast::Action::ApplyBudget` · sqlparser 0.62.0

```rust
ApplyBudget
```

Source: `src/ast/mod.rs:6981`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Apply a budget operation.

<a id="op-afcddb96c17f7cdb2652fb0a"></a>
## AttachListing

`variant` · `sqlparser::ast::Action::AttachListing` · sqlparser 0.62.0

```rust
AttachListing
```

Source: `src/ast/mod.rs:6983`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Attach a listing.

<a id="op-97c34b9020d2784433be89d6"></a>
## AttachPolicy

`variant` · `sqlparser::ast::Action::AttachPolicy` · sqlparser 0.62.0

```rust
AttachPolicy
```

Source: `src/ast/mod.rs:6985`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Attach a policy.

<a id="op-15048a6f958f3031802a1dd8"></a>
## Audit

`variant` · `sqlparser::ast::Action::Audit` · sqlparser 0.62.0

```rust
Audit
```

Source: `src/ast/mod.rs:6987`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Audit operation.

<a id="op-edfecf6fe29631c94505b7a7"></a>
## BindServiceEndpoint

`variant` · `sqlparser::ast::Action::BindServiceEndpoint` · sqlparser 0.62.0

```rust
BindServiceEndpoint
```

Source: `src/ast/mod.rs:6989`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Bind a service endpoint.

<a id="op-8417d23b7467a5860f282b5a"></a>
## Connect

`variant` · `sqlparser::ast::Action::Connect` · sqlparser 0.62.0

```rust
Connect
```

Source: `src/ast/mod.rs:6991`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Connect permission.

<a id="op-4ef2bffbd784f07f68c174be"></a>
## Create

`variant` · `sqlparser::ast::Action::Create` · sqlparser 0.62.0

```rust
Create
```

Source: `src/ast/mod.rs:6993`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Create action, optionally specifying an object type.

<a id="op-aaa644d3a440621ec9bc3607"></a>
## DatabaseRole

`variant` · `sqlparser::ast::Action::DatabaseRole` · sqlparser 0.62.0

```rust
DatabaseRole
```

Source: `src/ast/mod.rs:6998`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Actions related to database roles.

<a id="op-d240b834457c26c592484913"></a>
## Delete

`variant` · `sqlparser::ast::Action::Delete` · sqlparser 0.62.0

```rust
Delete
```

Source: `src/ast/mod.rs:7003`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Delete permission.

<a id="op-b184fd3f485e0a740a4381e6"></a>
## Drop

`variant` · `sqlparser::ast::Action::Drop` · sqlparser 0.62.0

```rust
Drop
```

Source: `src/ast/mod.rs:7005`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Drop permission.

<a id="op-ac98d5715225537d541c79a8"></a>
## EvolveSchema

`variant` · `sqlparser::ast::Action::EvolveSchema` · sqlparser 0.62.0

```rust
EvolveSchema
```

Source: `src/ast/mod.rs:7007`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Evolve schema permission.

<a id="op-4120c51c006a1a85cad8f803"></a>
## Exec

`variant` · `sqlparser::ast::Action::Exec` · sqlparser 0.62.0

```rust
Exec
```

Source: `src/ast/mod.rs:7009`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Exec action (execute) with optional object type.

<a id="op-c276f29ecc8926fd45026ab8"></a>
## Execute

`variant` · `sqlparser::ast::Action::Execute` · sqlparser 0.62.0

```rust
Execute
```

Source: `src/ast/mod.rs:7014`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Execute action with optional object type.

<a id="op-969796e737dfed263d97dafd"></a>
## Failover

`variant` · `sqlparser::ast::Action::Failover` · sqlparser 0.62.0

```rust
Failover
```

Source: `src/ast/mod.rs:7019`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Failover operation.

<a id="op-966552960c69181a22915d79"></a>
## ImportShare

`variant` · `sqlparser::ast::Action::ImportShare` · sqlparser 0.62.0

```rust
ImportShare
```

Source: `src/ast/mod.rs:7023`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Import a share.

<a id="op-44d2177b6a1651ff8990b375"></a>
## ImportedPrivileges

`variant` · `sqlparser::ast::Action::ImportedPrivileges` · sqlparser 0.62.0

```rust
ImportedPrivileges
```

Source: `src/ast/mod.rs:7021`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Use imported privileges.

<a id="op-84eb83b1e8cffbd0c248372c"></a>
## Insert

`variant` · `sqlparser::ast::Action::Insert` · sqlparser 0.62.0

```rust
Insert
```

Source: `src/ast/mod.rs:7025`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Insert rows with optional column list.

<a id="op-80def6c889da0107d1b707d9"></a>
## Manage

`variant` · `sqlparser::ast::Action::Manage` · sqlparser 0.62.0

```rust
Manage
```

Source: `src/ast/mod.rs:7030`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Manage operation with a specific manage type.

<a id="op-f52fdc7c6a33aae922d5296a"></a>
## ManageReleases

`variant` · `sqlparser::ast::Action::ManageReleases` · sqlparser 0.62.0

```rust
ManageReleases
```

Source: `src/ast/mod.rs:7035`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Manage releases.

<a id="op-063b7b43e0251a3dcfd1b69b"></a>
## ManageVersions

`variant` · `sqlparser::ast::Action::ManageVersions` · sqlparser 0.62.0

```rust
ManageVersions
```

Source: `src/ast/mod.rs:7037`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Manage versions.

<a id="op-a26a97591299cb653468745d"></a>
## Modify

`variant` · `sqlparser::ast::Action::Modify` · sqlparser 0.62.0

```rust
Modify
```

Source: `src/ast/mod.rs:7039`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Modify operation with an optional modify type.

<a id="op-3a5703aab3489ab7f076711f"></a>
## Monitor

`variant` · `sqlparser::ast::Action::Monitor` · sqlparser 0.62.0

```rust
Monitor
```

Source: `src/ast/mod.rs:7044`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Monitor operation with an optional monitor type.

<a id="op-3bc0bc9c700260394bbf7ed8"></a>
## Operate

`variant` · `sqlparser::ast::Action::Operate` · sqlparser 0.62.0

```rust
Operate
```

Source: `src/ast/mod.rs:7049`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Operate permission.

<a id="op-a5b3b758fbeb574cc9edc2f4"></a>
## OverrideShareRestrictions

`variant` · `sqlparser::ast::Action::OverrideShareRestrictions` · sqlparser 0.62.0

```rust
OverrideShareRestrictions
```

Source: `src/ast/mod.rs:7051`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Override share restrictions.

<a id="op-5733d89786cfffc7ef3a23c6"></a>
## Ownership

`variant` · `sqlparser::ast::Action::Ownership` · sqlparser 0.62.0

```rust
Ownership
```

Source: `src/ast/mod.rs:7053`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Ownership permission.

<a id="op-bddb70e23ce0b67c38c790e7"></a>
## PurchaseDataExchangeListing

`variant` · `sqlparser::ast::Action::PurchaseDataExchangeListing` · sqlparser 0.62.0

```rust
PurchaseDataExchangeListing
```

Source: `src/ast/mod.rs:7055`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Purchase a data exchange listing.

<a id="op-5a34f9ef33deaec26c43e222"></a>
## Read

`variant` · `sqlparser::ast::Action::Read` · sqlparser 0.62.0

```rust
Read
```

Source: `src/ast/mod.rs:7058`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Read access.

<a id="op-1a5f622b0afe1a0a8c7c8f83"></a>
## ReadSession

`variant` · `sqlparser::ast::Action::ReadSession` · sqlparser 0.62.0

```rust
ReadSession
```

Source: `src/ast/mod.rs:7060`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Read session-level access.

<a id="op-9ce9669695e0a0ce7e093456"></a>
## References

`variant` · `sqlparser::ast::Action::References` · sqlparser 0.62.0

```rust
References
```

Source: `src/ast/mod.rs:7062`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

References with optional column list.

<a id="op-03cf60cc50ce9a0e3430e50f"></a>
## Replicate

`variant` · `sqlparser::ast::Action::Replicate` · sqlparser 0.62.0

```rust
Replicate
```

Source: `src/ast/mod.rs:7067`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Replication permission.

<a id="op-93abbc064f6b1227b8f19238"></a>
## ResolveAll

`variant` · `sqlparser::ast::Action::ResolveAll` · sqlparser 0.62.0

```rust
ResolveAll
```

Source: `src/ast/mod.rs:7069`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Resolve all references.

<a id="op-1ecb9a2dc5bec1dcd105bb74"></a>
## Role

`variant` · `sqlparser::ast::Action::Role` · sqlparser 0.62.0

```rust
Role
```

Source: `src/ast/mod.rs:7071`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Role-related permission with target role name.

<a id="op-1c0bc5fd860d4d14f42f6756"></a>
## Select

`variant` · `sqlparser::ast::Action::Select` · sqlparser 0.62.0

```rust
Select
```

Source: `src/ast/mod.rs:7076`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Select permission with optional column list.

<a id="op-c529119d51e0df124d02cd1f"></a>
## Temporary

`variant` · `sqlparser::ast::Action::Temporary` · sqlparser 0.62.0

```rust
Temporary
```

Source: `src/ast/mod.rs:7081`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Temporary object permission.

<a id="op-5e4e1daca485d2f357423c55"></a>
## Trigger

`variant` · `sqlparser::ast::Action::Trigger` · sqlparser 0.62.0

```rust
Trigger
```

Source: `src/ast/mod.rs:7083`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Trigger-related permission.

<a id="op-8be097694cb58dc7e49e2b21"></a>
## Truncate

`variant` · `sqlparser::ast::Action::Truncate` · sqlparser 0.62.0

```rust
Truncate
```

Source: `src/ast/mod.rs:7085`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Truncate permission.

<a id="op-34001f07087efc7921c6b7c3"></a>
## Update

`variant` · `sqlparser::ast::Action::Update` · sqlparser 0.62.0

```rust
Update
```

Source: `src/ast/mod.rs:7087`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Update permission with optional affected columns.

<a id="op-8a969b1a879c76470f501e77"></a>
## Usage

`variant` · `sqlparser::ast::Action::Usage` · sqlparser 0.62.0

```rust
Usage
```

Source: `src/ast/mod.rs:7092`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Usage permission.

<a id="op-e854cf72bc230dc9b962e37a"></a>
## clone

`function` · `sqlparser::ast::Action::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> Action
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6969, 17], "end": [6969, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:6969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aa2a203788d857a4ec46810b"></a>
## cmp

`function` · `sqlparser::ast::Action::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &Action) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6969, 51], "end": [6969, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:6969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9439f6d9e0e36176aed8a443"></a>
## deserialize

`function` · `sqlparser::ast::Action::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [6970, 49], "end": [6970, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:6970`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf1ba30d311959b21bab1fb1"></a>
## eq

`function` · `sqlparser::ast::Action::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &Action) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6969, 24], "end": [6969, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:6969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12d1ed97cbdbd3050f81a0d1"></a>
## fmt

`function` · `sqlparser::ast::Action::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6969, 10], "end": [6969, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:6969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44aecd0cd3a16234e61b22b7"></a>
## fmt

`function` · `sqlparser::ast::Action::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7095, 1], "end": [7177, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7096`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf373fc9541825b0a77f2e28"></a>
## hash

`function` · `sqlparser::ast::Action::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6969, 56], "end": [6969, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:6969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f46fda1f9f4f19557a83cd4c"></a>
## partial_cmp

`function` · `sqlparser::ast::Action::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &Action) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6969, 35], "end": [6969, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:6969`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ac714474fae853b9fae8b90c"></a>
## serialize

`function` · `sqlparser::ast::Action::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6970, 38], "end": [6970, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:6970`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d41b9a070a5070b99ffda43f"></a>
## visit

`function` · `sqlparser::ast::Action::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6971, 47], "end": [6971, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:6971`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f516ab63e29a8c9a2df2f898"></a>
## visit

`function` · `sqlparser::ast::Action::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::Action", "path": "Action"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [6971, 40], "end": [6971, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:6971`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
