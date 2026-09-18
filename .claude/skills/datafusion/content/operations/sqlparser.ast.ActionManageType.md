# `sqlparser::ast::ActionManageType`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ActionManageType.json).

<a id="op-15f143b706ce54bf9511e640"></a>
## ActionManageType

`enum` · `sqlparser::ast::ActionManageType` · sqlparser 0.62.0

```rust
enum ActionManageType
```

Source: `src/ast/mod.rs:7325`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

See <https://docs.snowflake.com/en/sql-reference/sql/grant-privilege>
under `globalPrivileges` in the `MANAGE` privilege.

<a id="op-6edb35e6783b6f758830054f"></a>
## AccountSupportCases

`variant` · `sqlparser::ast::ActionManageType::AccountSupportCases` · sqlparser 0.62.0

```rust
AccountSupportCases
```

Source: `src/ast/mod.rs:7327`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Account support cases management.

<a id="op-a33f41aad8fdc7cb30adae7f"></a>
## EventSharing

`variant` · `sqlparser::ast::ActionManageType::EventSharing` · sqlparser 0.62.0

```rust
EventSharing
```

Source: `src/ast/mod.rs:7329`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Event sharing management.

<a id="op-5f817ec7db7d5aab4d594064"></a>
## Grants

`variant` · `sqlparser::ast::ActionManageType::Grants` · sqlparser 0.62.0

```rust
Grants
```

Source: `src/ast/mod.rs:7331`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Grants management.

<a id="op-1da43192880e2f4ca1239e57"></a>
## ListingAutoFulfillment

`variant` · `sqlparser::ast::ActionManageType::ListingAutoFulfillment` · sqlparser 0.62.0

```rust
ListingAutoFulfillment
```

Source: `src/ast/mod.rs:7333`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Listing auto-fulfillment management.

<a id="op-1ff003f0586db207c613ea04"></a>
## OrganizationSupportCases

`variant` · `sqlparser::ast::ActionManageType::OrganizationSupportCases` · sqlparser 0.62.0

```rust
OrganizationSupportCases
```

Source: `src/ast/mod.rs:7335`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Organization support cases management.

<a id="op-12c166c18786b13eb456d7bd"></a>
## UserSupportCases

`variant` · `sqlparser::ast::ActionManageType::UserSupportCases` · sqlparser 0.62.0

```rust
UserSupportCases
```

Source: `src/ast/mod.rs:7337`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

User support cases management.

<a id="op-d83bbdf20f4a925a31df1b34"></a>
## Warehouses

`variant` · `sqlparser::ast::ActionManageType::Warehouses` · sqlparser 0.62.0

```rust
Warehouses
```

Source: `src/ast/mod.rs:7339`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Warehouses management.

<a id="op-e8c00cc265d0c28a74efeb79"></a>
## clone

`function` · `sqlparser::ast::ActionManageType::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> ActionManageType
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7320, 17], "end": [7320, 22], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/mod.rs:7320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-46676dd5745ce2122186c53b"></a>
## cmp

`function` · `sqlparser::ast::ActionManageType::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &ActionManageType) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7320, 51], "end": [7320, 54], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/mod.rs:7320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-34eba9d6dd3ec1f925989551"></a>
## deserialize

`function` · `sqlparser::ast::ActionManageType::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [7321, 49], "end": [7321, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/mod.rs:7321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fbc9e9b7f7e53bc7d4dfbd22"></a>
## eq

`function` · `sqlparser::ast::ActionManageType::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &ActionManageType) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7320, 24], "end": [7320, 33], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/mod.rs:7320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c02691ad9412ed8c92379f1"></a>
## fmt

`function` · `sqlparser::ast::ActionManageType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7320, 10], "end": [7320, 15], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/mod.rs:7320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98efd35bf231d1d96690eecc"></a>
## fmt

`function` · `sqlparser::ast::ActionManageType::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7342, 1], "end": [7354, 2], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/mod.rs:7343`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47b635c7d8c57121994b1d0c"></a>
## hash

`function` · `sqlparser::ast::ActionManageType::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7320, 56], "end": [7320, 60], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/mod.rs:7320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee5e34e8a7d81b61b9f13b52"></a>
## partial_cmp

`function` · `sqlparser::ast::ActionManageType::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &ActionManageType) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7320, 35], "end": [7320, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/mod.rs:7320`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-080bbd527f466b939bcc773e"></a>
## serialize

`function` · `sqlparser::ast::ActionManageType::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7321, 38], "end": [7321, 47], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/mod.rs:7321`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77ea59d34093c3f61438b572"></a>
## visit

`function` · `sqlparser::ast::ActionManageType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7322, 47], "end": [7322, 55], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/mod.rs:7322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3a567241aa6ee40b21e2ca3"></a>
## visit

`function` · `sqlparser::ast::ActionManageType::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ActionManageType", "path": "ActionManageType"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [7322, 40], "end": [7322, 45], "filename": "src/ast/mod.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/mod.rs:7322`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
