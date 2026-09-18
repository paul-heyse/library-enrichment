# `sqlparser::ast::GrantObjects`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.GrantObjects.json).

<a id="op-bb4bcb4e8102b95173f90f9f"></a>
## GrantObjects

`enum` · `sqlparser::ast::GrantObjects` · sqlparser 0.62.0

```rust
enum GrantObjects
```

Source: `src/ast/mod.rs:7510`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Objects on which privileges are granted in a GRANT statement.

<a id="op-abca06050255e05362c91599"></a>
## AllExternalTablesInSchema

`variant` · `sqlparser::ast::GrantObjects::AllExternalTablesInSchema` · sqlparser 0.62.0

```rust
AllExternalTablesInSchema
```

Source: `src/ast/mod.rs:7532`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `ALL EXTERNAL TABLES IN SCHEMA <schema_name> [, ...]`

<a id="op-89d1ea121c36b524a979a435"></a>
## AllFunctionsInSchema

`variant` · `sqlparser::ast::GrantObjects::AllFunctionsInSchema` · sqlparser 0.62.0

```rust
AllFunctionsInSchema
```

Source: `src/ast/mod.rs:7537`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `ALL FUNCTIONS IN SCHEMA <schema_name> [, ...]`

<a id="op-bf443fb0b622e5896b335e2b"></a>
## AllMaterializedViewsInSchema

`variant` · `sqlparser::ast::GrantObjects::AllMaterializedViewsInSchema` · sqlparser 0.62.0

```rust
AllMaterializedViewsInSchema
```

Source: `src/ast/mod.rs:7527`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `ALL MATERIALIZED VIEWS IN SCHEMA <schema_name> [, ...]`

<a id="op-9f36e201acad2a1294270148"></a>
## AllSequencesInSchema

`variant` · `sqlparser::ast::GrantObjects::AllSequencesInSchema` · sqlparser 0.62.0

```rust
AllSequencesInSchema
```

Source: `src/ast/mod.rs:7512`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `ALL SEQUENCES IN SCHEMA <schema_name> [, ...]`

<a id="op-11b04c1aaa680d688cf2a754"></a>
## AllTablesInSchema

`variant` · `sqlparser::ast::GrantObjects::AllTablesInSchema` · sqlparser 0.62.0

```rust
AllTablesInSchema
```

Source: `src/ast/mod.rs:7517`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `ALL TABLES IN SCHEMA <schema_name> [, ...]`

<a id="op-ffba2d4e2c54285dcd8915ec"></a>
## AllViewsInSchema

`variant` · `sqlparser::ast::GrantObjects::AllViewsInSchema` · sqlparser 0.62.0

```rust
AllViewsInSchema
```

Source: `src/ast/mod.rs:7522`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `ALL VIEWS IN SCHEMA <schema_name> [, ...]`

<a id="op-1874b91502933d7f8eb6b856"></a>
## ComputePools

`variant` · `sqlparser::ast::GrantObjects::ComputePools` · sqlparser 0.62.0

```rust
ComputePools
```

Source: `src/ast/mod.rs:7590`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on compute pools

<a id="op-82c8323a7357028360fa5d19"></a>
## Connections

`variant` · `sqlparser::ast::GrantObjects::Connections` · sqlparser 0.62.0

```rust
Connections
```

Source: `src/ast/mod.rs:7592`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on connections

<a id="op-bda5bb4cf985bce074ae11f6"></a>
## Databases

`variant` · `sqlparser::ast::GrantObjects::Databases` · sqlparser 0.62.0

```rust
Databases
```

Source: `src/ast/mod.rs:7572`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on specific databases

<a id="op-956a665b9ec735fadcbc0d25"></a>
## ExternalVolumes

`variant` · `sqlparser::ast::GrantObjects::ExternalVolumes` · sqlparser 0.62.0

```rust
ExternalVolumes
```

Source: `src/ast/mod.rs:7598`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on external volumes

<a id="op-eca3ba868efb358d284c3a06"></a>
## FailoverGroup

`variant` · `sqlparser::ast::GrantObjects::FailoverGroup` · sqlparser 0.62.0

```rust
FailoverGroup
```

Source: `src/ast/mod.rs:7594`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on failover groups

<a id="op-8e101ea6976d6f0ba8daf4ec"></a>
## Function

`variant` · `sqlparser::ast::GrantObjects::Function` · sqlparser 0.62.0

```rust
Function
```

Source: `src/ast/mod.rs:7616`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on a function. In dialects that
support overloading, the argument types must be specified.

For example:
`GRANT USAGE ON FUNCTION foo(varchar) TO ROLE role1`

<a id="op-f58ba546f38e514305ab1d19"></a>
## FutureExternalTablesInSchema

`variant` · `sqlparser::ast::GrantObjects::FutureExternalTablesInSchema` · sqlparser 0.62.0

```rust
FutureExternalTablesInSchema
```

Source: `src/ast/mod.rs:7557`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `FUTURE EXTERNAL TABLES IN SCHEMA <schema_name> [, ...]`

<a id="op-6423812258b251ad3bfabe37"></a>
## FutureMaterializedViewsInSchema

`variant` · `sqlparser::ast::GrantObjects::FutureMaterializedViewsInSchema` · sqlparser 0.62.0

```rust
FutureMaterializedViewsInSchema
```

Source: `src/ast/mod.rs:7562`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `FUTURE MATERIALIZED VIEWS IN SCHEMA <schema_name> [, ...]`

<a id="op-57201273be486447cb9aa10f"></a>
## FutureSchemasInDatabase

`variant` · `sqlparser::ast::GrantObjects::FutureSchemasInDatabase` · sqlparser 0.62.0

```rust
FutureSchemasInDatabase
```

Source: `src/ast/mod.rs:7542`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `FUTURE SCHEMAS IN DATABASE <database_name> [, ...]`

<a id="op-d9a1e3e869f2beb6953b1fb9"></a>
## FutureSequencesInSchema

`variant` · `sqlparser::ast::GrantObjects::FutureSequencesInSchema` · sqlparser 0.62.0

```rust
FutureSequencesInSchema
```

Source: `src/ast/mod.rs:7567`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `FUTURE SEQUENCES IN SCHEMA <schema_name> [, ...]`

<a id="op-f67b5f39f487aa88f27acae8"></a>
## FutureTablesInSchema

`variant` · `sqlparser::ast::GrantObjects::FutureTablesInSchema` · sqlparser 0.62.0

```rust
FutureTablesInSchema
```

Source: `src/ast/mod.rs:7547`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `FUTURE TABLES IN SCHEMA <schema_name> [, ...]`

<a id="op-7954b5c9dd966a4aa330719c"></a>
## FutureViewsInSchema

`variant` · `sqlparser::ast::GrantObjects::FutureViewsInSchema` · sqlparser 0.62.0

```rust
FutureViewsInSchema
```

Source: `src/ast/mod.rs:7552`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on `FUTURE VIEWS IN SCHEMA <schema_name> [, ...]`

<a id="op-5eacea591bd3e123893a0d56"></a>
## Integrations

`variant` · `sqlparser::ast::GrantObjects::Integrations` · sqlparser 0.62.0

```rust
Integrations
```

Source: `src/ast/mod.rs:7584`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on specific integrations

<a id="op-dcac63e603b3670a647f4036"></a>
## Procedure

`variant` · `sqlparser::ast::GrantObjects::Procedure` · sqlparser 0.62.0

```rust
Procedure
```

Source: `src/ast/mod.rs:7604`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on a procedure. In dialects that
support overloading, the argument types must be specified.

For example:
`GRANT USAGE ON PROCEDURE foo(varchar) TO ROLE role1`

<a id="op-2637f9d63f9cca5a088614b0"></a>
## ReplicationGroup

`variant` · `sqlparser::ast::GrantObjects::ReplicationGroup` · sqlparser 0.62.0

```rust
ReplicationGroup
```

Source: `src/ast/mod.rs:7596`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on replication group

<a id="op-182efe563cd0eec384c04bb2"></a>
## ResourceMonitors

`variant` · `sqlparser::ast::GrantObjects::ResourceMonitors` · sqlparser 0.62.0

```rust
ResourceMonitors
```

Source: `src/ast/mod.rs:7586`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on resource monitors

<a id="op-beda592d6180cda7a0cb5cd7"></a>
## Schemas

`variant` · `sqlparser::ast::GrantObjects::Schemas` · sqlparser 0.62.0

```rust
Schemas
```

Source: `src/ast/mod.rs:7574`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on specific schemas

<a id="op-0257f998441151cd243a789a"></a>
## Sequences

`variant` · `sqlparser::ast::GrantObjects::Sequences` · sqlparser 0.62.0

```rust
Sequences
```

Source: `src/ast/mod.rs:7576`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on specific sequences

<a id="op-658168fa1d15d01e30e65e15"></a>
## Tables

`variant` · `sqlparser::ast::GrantObjects::Tables` · sqlparser 0.62.0

```rust
Tables
```

Source: `src/ast/mod.rs:7578`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on specific tables

<a id="op-91d8ef148ea1a56c2927cb88"></a>
## Users

`variant` · `sqlparser::ast::GrantObjects::Users` · sqlparser 0.62.0

```rust
Users
```

Source: `src/ast/mod.rs:7588`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on users

<a id="op-a180906e81570b81cf60f3f0"></a>
## Views

`variant` · `sqlparser::ast::GrantObjects::Views` · sqlparser 0.62.0

```rust
Views
```

Source: `src/ast/mod.rs:7580`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on specific views

<a id="op-71baa97591cadefb4a38d8fd"></a>
## Warehouses

`variant` · `sqlparser::ast::GrantObjects::Warehouses` · sqlparser 0.62.0

```rust
Warehouses
```

Source: `src/ast/mod.rs:7582`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grant privileges on specific warehouses

<a id="op-adfd0ca72c5d125b1df883a0"></a>
## clone

`function` · `sqlparser::ast::GrantObjects::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> GrantObjects
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7507, 17], "end": [7507, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d331a4be707b8fb33b149439"></a>
## cmp

`function` · `sqlparser::ast::GrantObjects::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &GrantObjects) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7507, 51], "end": [7507, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-116478d6438322d551241178"></a>
## deserialize

`function` · `sqlparser::ast::GrantObjects::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7508, 49], "end": [7508, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fcb6401952ce4dd22488acb2"></a>
## eq

`function` · `sqlparser::ast::GrantObjects::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &GrantObjects) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7507, 24], "end": [7507, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c2375bfdb58d7f31159da62"></a>
## fmt

`function` · `sqlparser::ast::GrantObjects::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7507, 10], "end": [7507, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d59071d41ea5e4487ca9ce2c"></a>
## fmt

`function` · `sqlparser::ast::GrantObjects::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7624, 1], "end": [7769, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7625`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73428d036c76ad69b9c53bf0"></a>
## hash

`function` · `sqlparser::ast::GrantObjects::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7507, 56], "end": [7507, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78f66bc445bd31db61a7e767"></a>
## partial_cmp

`function` · `sqlparser::ast::GrantObjects::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &GrantObjects) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7507, 35], "end": [7507, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7507`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e4335357108ea0bd86426f63"></a>
## serialize

`function` · `sqlparser::ast::GrantObjects::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7508, 38], "end": [7508, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7508`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ad66f80d60fc665543d558d"></a>
## visit

`function` · `sqlparser::ast::GrantObjects::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7509, 40], "end": [7509, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b9eaf685f801a214a381f5d"></a>
## visit

`function` · `sqlparser::ast::GrantObjects::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::GrantObjects", "path": "GrantObjects"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7509, 47], "end": [7509, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7509`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
