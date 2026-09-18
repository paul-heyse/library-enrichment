# `sqlparser::ast::FlushType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.FlushType.json).

<a id="op-a02086ab634fde3f01f31867"></a>
## FlushType

`enum` · `sqlparser::ast::FlushType` · sqlparser 0.62.0

```rust
enum FlushType
```

Source: `src/ast/mod.rs:9733`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Types of flush operations supported by `FLUSH`.

<a id="op-c4a5eb8c7b8eec02eaa77b37"></a>
## BinaryLogs

`variant` · `sqlparser::ast::FlushType::BinaryLogs` · sqlparser 0.62.0

```rust
BinaryLogs
```

Source: `src/ast/mod.rs:9735`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush binary logs.

<a id="op-976c146202e1ae056b4b8f11"></a>
## EngineLogs

`variant` · `sqlparser::ast::FlushType::EngineLogs` · sqlparser 0.62.0

```rust
EngineLogs
```

Source: `src/ast/mod.rs:9737`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush engine logs.

<a id="op-5c0ff75a1c9b3f7117217b10"></a>
## ErrorLogs

`variant` · `sqlparser::ast::FlushType::ErrorLogs` · sqlparser 0.62.0

```rust
ErrorLogs
```

Source: `src/ast/mod.rs:9739`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush error logs.

<a id="op-ce2435d2dc3bc21d00c65eb2"></a>
## GeneralLogs

`variant` · `sqlparser::ast::FlushType::GeneralLogs` · sqlparser 0.62.0

```rust
GeneralLogs
```

Source: `src/ast/mod.rs:9741`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush general logs.

<a id="op-2c2bbbb26015eafe5bf2aaa8"></a>
## Hosts

`variant` · `sqlparser::ast::FlushType::Hosts` · sqlparser 0.62.0

```rust
Hosts
```

Source: `src/ast/mod.rs:9743`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush hosts information.

<a id="op-9755ddf68d0d7917acc2198e"></a>
## Logs

`variant` · `sqlparser::ast::FlushType::Logs` · sqlparser 0.62.0

```rust
Logs
```

Source: `src/ast/mod.rs:9745`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush logs.

<a id="op-fc4aa2064c27fc1a07ff4e29"></a>
## OptimizerCosts

`variant` · `sqlparser::ast::FlushType::OptimizerCosts` · sqlparser 0.62.0

```rust
OptimizerCosts
```

Source: `src/ast/mod.rs:9749`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush optimizer costs.

<a id="op-d147b79d632edbf2e88075dc"></a>
## Privileges

`variant` · `sqlparser::ast::FlushType::Privileges` · sqlparser 0.62.0

```rust
Privileges
```

Source: `src/ast/mod.rs:9747`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush privileges.

<a id="op-bb130d34e85c07ae3dec3927"></a>
## RelayLogs

`variant` · `sqlparser::ast::FlushType::RelayLogs` · sqlparser 0.62.0

```rust
RelayLogs
```

Source: `src/ast/mod.rs:9751`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush relay logs.

<a id="op-03292292ffedc1c42adba8af"></a>
## SlowLogs

`variant` · `sqlparser::ast::FlushType::SlowLogs` · sqlparser 0.62.0

```rust
SlowLogs
```

Source: `src/ast/mod.rs:9753`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush slow logs.

<a id="op-9e8e209d2aea68db45a2210c"></a>
## Status

`variant` · `sqlparser::ast::FlushType::Status` · sqlparser 0.62.0

```rust
Status
```

Source: `src/ast/mod.rs:9755`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush status.

<a id="op-27090a7ac33faf6b7df93764"></a>
## Tables

`variant` · `sqlparser::ast::FlushType::Tables` · sqlparser 0.62.0

```rust
Tables
```

Source: `src/ast/mod.rs:9759`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush table data.

<a id="op-f342b4119bea195c8b10bc7a"></a>
## UserResources

`variant` · `sqlparser::ast::FlushType::UserResources` · sqlparser 0.62.0

```rust
UserResources
```

Source: `src/ast/mod.rs:9757`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Flush user resources.

<a id="op-ed00e090350dc32ffa84c580"></a>
## clone

`function` · `sqlparser::ast::FlushType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> FlushType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9730, 23], "end": [9730, 28], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:9730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a63d090958fcf48c2a502ce"></a>
## cmp

`function` · `sqlparser::ast::FlushType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &FlushType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9730, 57], "end": [9730, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:9730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c4c9f2ffb936be30698a97a7"></a>
## deserialize

`function` · `sqlparser::ast::FlushType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [9731, 49], "end": [9731, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:9731`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c78b1349a366b9f3d60be1bd"></a>
## eq

`function` · `sqlparser::ast::FlushType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &FlushType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9730, 30], "end": [9730, 39], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:9730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-456d1e4daaac466c0e3e695a"></a>
## fmt

`function` · `sqlparser::ast::FlushType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9730, 10], "end": [9730, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:9730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bc37b5da3792607c1a2b2f61"></a>
## fmt

`function` · `sqlparser::ast::FlushType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9762, 1], "end": [9780, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:9763`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09163fbb7dd96de357a72281"></a>
## hash

`function` · `sqlparser::ast::FlushType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9730, 62], "end": [9730, 66], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:9730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-450914a2487c403913274d44"></a>
## partial_cmp

`function` · `sqlparser::ast::FlushType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &FlushType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9730, 41], "end": [9730, 51], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:9730`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-621a3dca29778429bf91c5a0"></a>
## serialize

`function` · `sqlparser::ast::FlushType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9731, 38], "end": [9731, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:9731`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-30adb57d69eecedb5747ad43"></a>
## visit

`function` · `sqlparser::ast::FlushType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9732, 40], "end": [9732, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:9732`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90809a30992277f07fccbfa8"></a>
## visit

`function` · `sqlparser::ast::FlushType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::FlushType", "path": "FlushType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [9732, 47], "end": [9732, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:9732`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
